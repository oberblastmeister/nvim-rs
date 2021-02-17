use crate::api::{give_lifetime, Api, Function};
use crate::utils;

use std::{collections::HashSet, process::Command};
use std::{io::Cursor, path::Path};

use eyre::{eyre, Result};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use rmp_serde::decode;
use xshell::{cmd, read_file, write_file};

pub fn gen_from_api(api: Api) -> Result<String> {
  let manually_implemented = [
    "nvim_ui_attach",
    "nvim_tabpage_list_wins",
    "nvim_tabpage_get_win",
    "nvim_win_get_buf",
    "nvim_win_get_tabpage",
    "nvim_list_bufs",
    "nvim_get_current_buf",
    "nvim_list_wins",
    "nvim_get_current_win",
    "nvim_create_buf",
    "nvim_open_win",
    "nvim_list_tabpages",
    "nvim_get_current_tabpage",
  ]
  .iter()
  .copied()
  .collect::<HashSet<&'static str>>();

  let ext_type_impls = api
    .types()
    .keys()
    .map(|k| {
      let api_funcs: Vec<_> = api
        .ext_type_functions(k)
        .unwrap()
        .filter(|f| !manually_implemented.contains(&**f.name()))
        .collect();
      gen_impl(k, api_funcs)
    })
    .collect::<Vec<_>>();

  let tokens = quote! {
    use crate::{Value, Neovim, error::CallError, rpc::unpack::TryUnpack};
    use crate::rpc::model::IntoVal;
    use futures::io::AsyncWrite;
    use serde::Serialize;
    use std::marker::PhantomData;

    #(#ext_type_impls)*
  };

  Ok(utils::reformat(&tokens.to_string())?)
}

pub fn gen_impl(ext_type_name: &str, functions: Vec<&Function>) -> TokenStream {
  let ext_type_ident = format_ident!("{}", ext_type_name);

  let ext_type_methods = functions.iter().filter_map(|f| gen_function(f, true));

  quote! {
    use crate::#ext_type_ident;

    impl<W: AsyncWrite + Send + Unpin + 'static> #ext_type_ident<W> {
      pub fn new(code_data: Value, neovim: Neovim<W>) -> #ext_type_ident<W> {
        #ext_type_ident {
          code_data,
          neovim,
        }
      }

      /// Internal value, that represent type
      pub fn get_value(&self) -> &Value {
        &self.code_data
      }

      #(#ext_type_methods)*
    }
  }
}

fn gen_function(f: &Function, is_ext_type: bool) -> Option<TokenStream> {
  let name_str = f.name();
  let name = format_ident!("{}", name_str);

  let skip_amount = if is_ext_type { 1 } else { 0 };

  let param_names: Vec<_> = f
    .parameters()
    .iter()
    .skip(skip_amount)
    .map(|param| format_ident!("{}", param.name()))
    .collect();

  let param_types = f
    .parameters()
    .iter()
    .skip(skip_amount)
    .map(|param| {
      let tipe_str = param.tipe();
      tipe_str.to_rust_type_ref()
    })
    .collect::<Option<Vec<_>>>()?; // we will skip functions that don't have a corresponding rust type
                                   // like LuaRef

  let arg_param_types = f
    .parameters()
    .iter()
    .skip(skip_amount)
    .filter_map(|param| {
      let tipe_str = param.tipe();
      let mut tipe = tipe_str.to_rust_type_ref()?;
      give_lifetime(&mut tipe, "'a");
      Some(tipe)
    })
    .collect::<Vec<_>>();

  let return_type = f
    .return_type()
    .to_rust_type_val()
    .expect("Return type was not converted to rust type");

  let deprecated_doc = if let Some(since) = f.deprecated_since() {
    let s = format!("Deprecated since {}", since);
    quote! { #[doc = #s] }
  } else {
    quote! {}
  };

  let since_doc = {
    let s = format!("Since {}", f.since());
    quote! { #[doc = #s] }
  };

  let args_struct = quote! {
    #[derive(Debug, Serialize)]
    // pub struct Args<'a, W: AsyncWrite + Send + Unpin + 'static>(PhantomData<fn(&'a ())>, Value, #(#arg_param_types),*);
    pub struct Args<'a>(PhantomData<&'a ()>, Value, #(#arg_param_types),*);
  };

  Some(quote! {
    #deprecated_doc
    #since_doc
    pub async fn #name(&self, #(#param_names: #param_types),*) -> Result<#return_type, Box<CallError>> {
      #args_struct

      self.neovim.call(
        #name_str,
        Args(
          std::marker::PhantomData,
          self.code_data.clone(),
          #(#param_names),*
        )
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
    }
  })
}

pub fn run() -> Result<()> {
  let nvim_path = utils::nvim_path();
  let output = cmd!("{nvim_path} --api-info").output()?;
  let cursor = Cursor::new(output.stdout);
  let api: Api = decode::from_read(cursor)?;

  let res = gen_from_api(api)?;
  update(&utils::project_root().join("src/neovim_api.rs"), &res)?;

  Ok(())
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
fn update(path: &Path, contents: &str) -> Result<()> {
  fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n")
  }

  match read_file(path) {
    Ok(old_contents) if normalize(&old_contents) == normalize(contents) => {
      return Ok(());
    }
    _ => (),
  }

  eprintln!("updating {}", path.display());
  write_file(path, contents)?;
  Ok(())
}
