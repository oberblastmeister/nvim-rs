//! This module contains rust structs that represent the neovim api. They all
//! derive `Serialize` and `Deserialize`
//! This is the raw stuff that comes from the msgpack outputted by neovim.
//! The parent module will contain the preprocessing

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use syn::parse_quote;

lazy_static! {
  static ref UNBOUND_ARRAY_RE: Regex =
    Regex::new(r"ArrayOf\(\s*(\w+)\s*\)").unwrap();
}

/// The base api struct
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Api {
  pub version: Version,
  pub functions: Vec<Function>,
  pub ui_options: Vec<String>,
  pub ui_events: Vec<UiEvent>,
  pub types: HashMap<String, ExtType>,
  pub error_types: HashMap<String, ErrorType>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
  pub api_compatible: i64,
  pub api_level: i64,
  pub api_prerelease: bool,
  pub major: i64,
  pub minor: i64,
  pub patch: i64,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
  pub deprecated_since: Option<i64>,
  pub method: bool,
  pub name: String,
  #[serde(skip)]
  pub short_name: String,
  pub parameters: Vec<Param>,
  pub return_type: Type,
  pub since: i64,
  /// not part of msgpack
  #[serde(skip)]
  pub ext_type: bool,
  #[serde(skip)]
  pub ext_type_name: Option<String>,
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtType {
  pub id: i64,
  pub prefix: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorType {
  pub id: i64,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiEvent {
  pub name: String,
  pub parameters: Vec<Param>,
  pub since: i64,
}
