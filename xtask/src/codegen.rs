use crate::api::{Api, Function};
use crate::utils;

use std::process::Command;
use std::{io::Cursor, path::Path};

use eyre::{eyre, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rmp_serde::decode;
use xshell::{cmd, read_file, write_file};

pub fn gen_from_api(api: Api) -> Result<String> {
  let ext_type_impls = api
    .types()
    .keys()
    .map(|k| {
      let api_funcs: Vec<_> = api.ext_type_functions(k).unwrap().collect();
      gen_impl(k, api_funcs)
    })
    .collect::<Vec<_>>();

  let tokens = quote! {
    use crate::{Value, Neovim, error::CallError, rpc::unpack::TryUnpack};
    use futures::io::AsyncWrite;

    #(#ext_type_impls)*
  };

  Ok(utils::reformat(&tokens.to_string())?)
}

pub fn gen_impl(ext_type_name: &str, functions: Vec<&Function>) -> TokenStream {
  let ext_type_ident = format_ident!("{}", ext_type_name);

  let ext_type_methods = functions.iter().map(|f| gen_function(f));

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

fn gen_function(f: &Function) -> TokenStream {
  let name_str = f.name();
  let name = format_ident!("{}", name_str);

  let param_names: Vec<_> = f
    .parameters()
    .iter()
    .skip(1)
    .map(|param| format_ident!("{}", param.name()))
    .collect();

  let param_types = f
    .parameters()
    .iter()
    .skip(1)
    .filter_map(|param| {
      let tipe_str = param.tipe();
      tipe_str.to_rust_type()
      // tipe_str.to_rust_type().ok_or_else(|| {
      //   eyre!("Failed to find rust type for type_str {}", tipe_str.inner())
      // })
    })
    .collect::<Vec<_>>();
  // .collect::<Result<Vec<_>>>()
  // .unwrap();

  let return_type = f
    .return_type()
    .to_rust_type()
    .expect("Return type was not converted to rust type");

  quote! {
    pub async fn #name(&self, #(#param_names: #param_types),*) -> Result<#return_type, Box<CallError>> {
      self.neovim.call(
        #name_str,
        [
          self.code_data.clone(),
          #(Value::from(#param_names)),*
        ]
      )
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))
    }
  }
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
