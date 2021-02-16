//! This module contains rust structs that represent the neovim api. They all derive `Serialize`
//! and `Deserialize`

use std::{borrow::Cow, collections::HashMap};

use getset::Getters;
use lazy_static::lazy_static;
use proc_macro2::Span;
use quote::format_ident;
use regex::Regex;
use serde::{Deserialize, Serialize};
use syn::{parse_quote, GenericArgument, TypePath};

lazy_static! {
  static ref UNBOUND_ARRAY_RE: Regex =
    Regex::new(r"ArrayOf\(\s*(\w+)\s*\)").unwrap();
}

/// The base api struct
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct Api {
  version: Version,
  functions: Vec<Function>,
  ui_options: Vec<String>,
  ui_events: Vec<UiEvent>,
  types: HashMap<String, ExtType>,
  error_types: HashMap<String, ErrorType>,
}

impl Api {
  /// Gets the functions corresponding to the name of the ext_type
  pub fn ext_type_functions<'a>(
    &'a self,
    ext_type_name: &str,
  ) -> Option<impl Iterator<Item = &'a Function>> {
    let ext_type = self.types().get(ext_type_name)?;
    let prefix = ext_type.prefix();
    let iter = self
      .functions()
      .iter()
      .filter(move |func| func.name().starts_with(prefix));
    Some(iter)
  }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct Version {
  api_compatible: u32,
  api_level: u32,
  api_prerelease: bool,
  major: u32,
  minor: u32,
  patch: u32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct Function {
  deprecated_since: Option<u32>,
  method: bool,
  name: String,
  parameters: Vec<Param>,
  return_type: Type,
  since: u32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Param(Type, String);

impl Param {
  /// Weird name because `type` is a reserved keyword
  pub fn tipe(&self) -> &Type {
    &self.0
  }
  pub fn name(&self) -> &str {
    &self.1
  }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Type(String);

impl Type {
  pub fn inner(&self) -> &str {
    &self.0
  }

  pub fn to_rust_type_ref(&self) -> Option<syn::Type> {
    Some(match &*self.0 {
      "Array" => parse_quote! { std::vec::Vec<Value> },
      "ArrayOf(Integer, 2)" => parse_quote! { (i64, i64) },
      "void" => parse_quote! { () },
      "Integer" => parse_quote! { i64 },
      "Float" => parse_quote! { f64 },
      "Boolean" => parse_quote! { bool },
      "String" => parse_quote! { &str },
      "Object" => parse_quote! { Value },
      "Dictionary" => {
        parse_quote! { std::vec::Vec<(Value, Value)> }
      }
      "Window" => parse_quote! { Window<W> },
      "Buffer" => parse_quote! { Window<W> },
      "Tabpage" => parse_quote! { Tabpage<W> },
      s if UNBOUND_ARRAY_RE.is_match(s) => {
        let inner = UNBOUND_ARRAY_RE
          .captures(s)
          .expect("No captures")
          .get(1)
          .unwrap()
          .as_str();
        let inner_ty = Type(inner.into());
        let inner_rust = inner_ty.to_rust_type_ref().unwrap();
        // parse_quote! { &[#inner_rust] }
        parse_quote! { std::vec::Vec<#inner_rust> }
      }
      _ => return None,
    })
  }

  pub fn to_rust_type_val(&self) -> Option<syn::Type> {
    Some(match &*self.0 {
      "String" => parse_quote! { String },
      s if UNBOUND_ARRAY_RE.is_match(s) => {
        let inner = UNBOUND_ARRAY_RE
          .captures(s)
          .expect("No captures")
          .get(1)
          .unwrap()
          .as_str();
        let inner_ty = Type(inner.into());
        let inner_rust = inner_ty.to_rust_type_val().unwrap();
        parse_quote! { std::vec::Vec<#inner_rust> }
      }
      _ => return self.to_rust_type_ref(),
    })
  }
}

pub fn give_lifetime(tipe: &mut syn::Type, lifetime_name: &str) -> bool {
  let mut gave = false;

  match tipe {
    syn::Type::Reference(ref mut type_ref) if type_ref.lifetime.is_none() => {
      let lifetime = syn::Lifetime::new(lifetime_name, Span::call_site());
      type_ref.lifetime = Some(lifetime);
      gave = true;
    }
    syn::Type::Path(TypePath { ref mut path, .. }) => {
      let last_seg = path.segments.iter_mut().last().unwrap();
      match last_seg.arguments {
        syn::PathArguments::None => (),
        syn::PathArguments::AngleBracketed(ref mut angle_bracketed) => {
          for generic_arg in angle_bracketed.args.iter_mut() {
            if let GenericArgument::Type(tipe) = generic_arg {
              gave = give_lifetime(tipe, lifetime_name) || gave;
            }
          }
        }
        _ => (),
      }
    }
    _ => (),
  }

  gave
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct UiEvent {
  name: String,
  parameters: Vec<Param>,
  since: u32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct ExtType {
  id: u32,
  prefix: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct ErrorType {
  id: u32,
}
