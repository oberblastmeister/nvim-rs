use crate::{
  api::{Api, ExtType, Function, Type},
  syn_helpers::{is_ext_type, AddLifetime, MaybeQuote},
  utils,
};

use std::{collections::HashSet, io::Cursor, path::Path, process::Command};

use eyre::{eyre, Result};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use rmp_serde::decode;
use syn::{fold::Fold, parse_quote as pq};
use xshell::{cmd, read_file, write_file};

pub(crate) trait Gen {
  fn gen(&self) -> Result<TokenStream>;
}

impl Gen for Api {
  fn gen(&self) -> Result<TokenStream> {
    let manually_implemented = [
      "nvim_ui_attach",
      "ui_attach",
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
    .collect::<HashSet<_>>();

    let ext_type_impls = self
      .types
      .iter()
      .map(ExtType::gen)
      .collect::<Result<Vec<_>>>()?;

    let non_ext_type_functions = self
      .functions
      .iter()
      .filter(|f| f.deprecated_since.is_none())
      .filter(|f| !manually_implemented.contains(&*f.name))
      .map(Function::gen)
      .collect::<Result<Vec<_>>>()?;

    let tokens = quote! {
        use crate::{Value, Neovim, error::CallError, rpc::unpack::TryUnpack};
        use futures::io::AsyncWrite;
        use serde::Serialize;
        use std::marker::PhantomData;

        impl<W: AsyncWrite + Send + Unpin + 'static> Neovim<W> {
            #(#non_ext_type_functions)*
        }

        #(#ext_type_impls)*
    };

    Ok(tokens)
  }
}

impl Gen for ExtType {
  fn gen(&self) -> Result<TokenStream> {
    let ident = format_ident!("{}", self.name);

    let methods = self.functions.iter().map(|f| f.gen().unwrap());

    let res = quote! {
        pub struct #ident<W>
            where
                W: AsyncWrite + Send + Unpin + 'static,
            {
                pub(crate) code_data: i64,
                pub(crate) neovim: Neovim<W>,
            }

        impl<W: AsyncWrite + Send + Unpin + 'static> #ident<W> {
            pub fn new(code_data: i64, neovim: Neovim<W>) -> #ident<W> {
                #ident {
                    code_data,
                    neovim,
                }
            }

            /// Internal value, that represent type
            pub fn get_value(&self) -> i64 {
                self.code_data
            }

            #(#methods)*
        }
    };

    Ok(res)
  }
}

impl Gen for Function {
  fn gen(&self) -> Result<TokenStream> {
    let name_str = &self.name;
    let name = format_ident!("{}", self.name);

    let skip_amount = if self.ext_type { 1 } else { 0 };

    // the name of the params
    let param_names: Vec<_> = self
      .parameters
      .iter()
      .skip(skip_amount)
      .map(|param| format_ident!("r#{}", param.name()))
      .collect();

    // the types of the params
    let param_types = self
      .parameters
      .iter()
      .skip(skip_amount)
      .filter_map(|param| {
        let tipe_str = param.tipe();
        tipe_str.to_rust_type_ref()
      })
      .collect::<Vec<_>>(); // like LuaRef

    let (arg_param_names, arg_param_types): (Vec<_>, Vec<_>) = self
      .parameters
      .iter()
      .skip(skip_amount)
      .filter_map(|param| {
        let tipe_str = param.tipe();
        let mut tipe = tipe_str.to_rust_type_ref()?;

        let name = if is_ext_type(&tipe) {
          tipe = pq! { i64 };
          let name = format_ident!("r#{}", param.name());
          quote! { #name.get_value() }
        } else {
          let name = format_ident!("r#{}", param.name());
          quote! { #name }
        };

        let tipe = Some(AddLifetime::new("'a").fold_type(tipe));

        tipe.map(|tipe| (name, tipe))
      })
      .unzip();

    let return_type = self
      .return_type
      .to_rust_type_val()
      .expect("Return type was not converted to rust type");

    let deprecated_doc = MaybeQuote(self.deprecated_since.map(|n| {
      let s = format!("Deprecated since {}", n);
      quote! { #[doc = #s] }
    }));

    let since_doc = {
      let s = format!("Since {}", self.since);
      quote! { #[doc = #s] }
    };

    let code_data_ty = MaybeQuote(self.ext_type.then(|| quote! { i64, }));

    let code_data_param =
      MaybeQuote(self.ext_type.then(|| quote! { self.get_value(), }));

    let args_struct = quote! {
        #[derive(Debug, Serialize)]
        // pub struct Args<'a, W: AsyncWrite + Send + Unpin + 'static>(PhantomData<fn(&'a ())>, Value, #(#arg_param_types),*);
        pub struct Args<'a>(PhantomData<&'a ()>, #code_data_ty #(#arg_param_types),*);
    };

    let uses_internal_neovim =
      MaybeQuote(self.ext_type.then(|| quote! { .neovim }));

    Ok(quote! {
        #deprecated_doc
        #since_doc
        pub async fn #name(&self, #(#param_names: #param_types),*) -> Result<#return_type, Box<CallError>> {
            #args_struct

            self#uses_internal_neovim.call(
                #name_str,
                Args(
                    std::marker::PhantomData,
                    #code_data_param
                    #(#arg_param_names),*
                )
            )
                .await??
                .try_unpack()
                .map_err(|v| Box::new(CallError::WrongValueType(v)))
        }
    })
  }
}

pub fn gen_api<P: AsRef<Path>>(nvim_path: P) -> Result<String> {
  Ok(utils::reformat(
    &Api::get(nvim_path.as_ref())?.gen()?.to_string(),
  )?)
}

pub fn run() -> Result<()> {
  let res = gen_api(utils::nvim_path())?;
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
