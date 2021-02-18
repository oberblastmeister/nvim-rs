//! This module contains rust structs that represent the neovim api. They all
//! derive `Serialize` and `Deserialize`

use std::{
  borrow::Cow,
  collections::{HashMap, HashSet},
  mem::{self, ManuallyDrop, MaybeUninit},
  panic, process,
};

use getset::Getters;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use syn::parse_quote;

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

  fn get_prefixes(&self) -> Vec<&str> {
    self.types().values().map(|et| &**et.prefix()).collect()
  }

  pub fn non_ext_type_functions(&self) -> impl Iterator<Item = &Function> {
    let prefixes = self.get_prefixes();
    self.functions().iter().filter(move |func| {
      !prefixes
        .iter()
        .any(|prefix| func.name().starts_with(prefix))
    })
  }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct Version {
  api_compatible: i64,
  api_level: i64,
  api_prerelease: bool,
  major: i64,
  minor: i64,
  patch: i64,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct Function {
  deprecated_since: Option<i64>,
  method: bool,
  name: String,
  parameters: Vec<Param>,
  return_type: Type,
  since: i64,
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
      "Array" => parse_quote! { &[Value] },
      "ArrayOf(Integer, 2)" => parse_quote! { (i64, i64) },
      "void" => parse_quote! { () },
      "Integer" => parse_quote! { i64 },
      "Float" => parse_quote! { f64 },
      "Boolean" => parse_quote! { bool },
      "String" => parse_quote! { &str },
      "Object" => parse_quote! { Value },
      "Dictionary" => {
        parse_quote! { &[(Value, Value)] }
      }
      "Window" => parse_quote! { Window<W> },
      "Buffer" => parse_quote! { Buffer<W> },
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
        parse_quote! { &[#inner_rust] }
      }
      _ => return None,
    })
  }

  pub fn to_rust_type_val(&self) -> Option<syn::Type> {
    Some(match &*self.0 {
      "String" => parse_quote! { String },
      "Array" => parse_quote! { std::vec::Vec<Value> },
      "Dictionary" => {
        parse_quote! { std::vec::Vec<(Value, Value)> }
      }
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

// /// modify a mutable reference in place with a function that must take
/// ownership of the value
fn modify<T>(t: &mut T, f: impl FnOnce(T) -> T) {
  let t_ptr = t as *mut T;
  unsafe {
    let new = panic::catch_unwind(panic::AssertUnwindSafe(|| f(t_ptr.read())))
      .unwrap_or_else(|_| process::abort());
    t_ptr.write(new);
  }
}

fn modify_default<T: Default>(t: &mut T, f: impl FnOnce(T) -> T) {
  let t_ptr = t as *mut T;

  unsafe {
    let new = panic::catch_unwind(panic::AssertUnwindSafe(|| f(t_ptr.read())))
      .unwrap_or_else(|e| {
        t_ptr.write(T::default()); // prevent from double freeing
        panic!("Modify closure panicked: {:?}", e);
      });
    t_ptr.write(new);
  }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct UiEvent {
  name: String,
  parameters: Vec<Param>,
  since: i64,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct ExtType {
  id: i64,
  prefix: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Getters)]
#[get = "pub"]
pub struct ErrorType {
  id: i64,
}
