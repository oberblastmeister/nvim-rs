use crate::{
  api::{Api, ExtType, Function},
  syn_helpers::{is_ext_type, ext_type_ident, MaybeQuote},
  utils,
};

use std::path::Path;

use eyre::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use xshell::{read_file, write_file};
use syn::parse_quote as pq;

pub(crate) trait Gen {
  fn gen(&self) -> Result<TokenStream>;
}

impl Gen for Api {
  fn gen(&self) -> Result<TokenStream> {
    let ext_type_impls = self
      .types
      .iter()
      .map(ExtType::gen)
      .collect::<Result<Vec<_>>>()?;

    let non_ext_type_functions = self
      .functions
      .iter()
      .filter(|f| f.deprecated_since.is_none())
      .map(Function::gen)
      .collect::<Result<Vec<_>>>()?;

    let imports = quote! {
      use crate::{Value, Neovim, error::CallError, rpc::unpack::TryUnpack};
      use futures::io::AsyncWrite;
      use serde::Serialize;
      use std::marker::PhantomData;
    };

    let tokens = quote! {
        #imports

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

    let trait_bound = quote! { AsyncWrite + Send + Unpin + 'static };

    let res = quote! {
        pub struct #ident<W>
            where
                W: #trait_bound,
            {
                pub(crate) code_data: i64,
                pub(crate) neovim: Neovim<W>,
            }

        impl<W: #trait_bound> #ident<W> {
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
    let short_name_str = &self.short_name;
    let short_name = format_ident!("{}", short_name_str);

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

    let arg_param_names: Vec<_> = self
      .parameters
      .iter()
      .skip(skip_amount)
      .filter_map(|param| {
        let tipe_str = param.tipe();
        let tipe = tipe_str.to_rust_type_ref()?;
        let name = format_ident!("r#{}", param.name());
        let name = if is_ext_type(&tipe) {
          quote! { #name.get_value() }
        } else {
          quote! { #name }
        };
        Some(name)
      })
      .collect();

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

    let code_data_param =
      MaybeQuote(self.ext_type.then(|| quote! { self.get_value(), }));

    let uses_internal_neovim =
      MaybeQuote(self.ext_type.then(|| quote! { .neovim }));

    let convert = if is_ext_type(&return_type) {
      let ident = ext_type_ident(&return_type);
      quote! {
        .await?
        .map(|val| #ident::new(val.try_unpack().unwrap(), self#uses_internal_neovim.clone()))
        .map_err(|v| Box::new(CallError::WrongValueType(v)))
      }
    } else {
      quote! {
        .await??
        .try_unpack()
        .map_err(|v| Box::new(CallError::WrongValueType(v)))
      }
    };

    let mut neovim_arguments: syn::Expr = pq! {
        (
            #code_data_param
            #(#arg_param_names,)*
        )
    };

    if neovim_arguments == pq! { () } {
        neovim_arguments = pq! {
            {
                let empty: [Value; 0] =  [];
                empty
            }
        }
    }

    Ok(quote! {
        #deprecated_doc
        #since_doc
        pub async fn #short_name(&self, #(#param_names: #param_types),*) -> Result<#return_type, Box<CallError>> {
            self#uses_internal_neovim.call(
                #name_str,
                #neovim_arguments
            )
              #convert
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
