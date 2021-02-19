//! Preprocessing for the deserialized api

use eyre::Result;
use lazy_static::lazy_static;
use rmp_serde::decode;
use std::{
  collections::{HashMap, HashSet},
  io::Cursor,
  path,
};
use xshell::cmd;

mod de;

pub use de::{ErrorType, Function, Type, UiEvent, Version};

lazy_static! {
  static ref MANUALLY_IMPLEMENTED: HashSet<&'static str> = [
    // "nvim_ui_attach",
    // "nvim_tabpage_list_wins",
    // "nvim_tabpage_get_win",
    // "nvim_win_get_buf",
    // "nvim_win_get_tabpage",
    // "nvim_list_bufs",
    // "nvim_get_current_buf",
    // "nvim_list_wins",
    // "nvim_get_current_win",
    // "nvim_create_buf",
    // "nvim_open_win",
    // "nvim_list_tabpages",
    // "nvim_get_current_tabpage",
  ]
  .iter()
  .copied()
  .collect();
}

#[derive(Debug, PartialEq, Eq)]
pub struct Api {
  pub version: Version,
  pub functions: Vec<Function>,
  pub ui_options: Vec<String>,
  pub ui_events: Vec<UiEvent>,
  pub types: Vec<ExtType>,
  pub error_types: HashMap<String, ErrorType>,
}

impl Api {
  pub fn get(nvim_path: impl AsRef<path::Path>) -> Result<Api> {
    let output = cmd!("{nvim_path} --api-info").output()?;
    let cursor = Cursor::new(output.stdout);
    let api: de::Api = decode::from_read(cursor)?;
    Ok(Api::from_de(api))
  }

  fn from_de(de_api: de::Api) -> Api {
    let de::Api {
      version,
      functions,
      ui_options,
      ui_events,
      types,
      error_types,
    } = de_api;

    let mut types: Vec<_> = types
      .into_iter()
      .map(|(name, et)| ExtType::from_de(et, name))
      .collect();

    let mut regular_functions = Vec::new();

    for function in functions
      .into_iter()
      .filter(|f| !MANUALLY_IMPLEMENTED.contains(&*f.name))
    {
      if let Some(not_added) = add_function(&mut types, function) {
        regular_functions.push(not_added);
      }
    }

    Api {
      version,
      functions: regular_functions,
      ui_options,
      ui_events,
      types,
      error_types,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExtType {
  pub name: String,
  pub id: i64,
  pub prefix: String,
  pub functions: Vec<de::Function>,
}

impl ExtType {
  fn from_de(de_type: de::ExtType, name: String) -> ExtType {
    let de::ExtType { id, prefix } = de_type;

    ExtType {
      name,
      id,
      prefix,
      functions: Vec::new(),
    }
  }
}

fn add_function(
  ext_types: &mut [ExtType],
  mut function: de::Function,
) -> Option<Function> {
  let mut found = 0;

  for (idx, ext_type) in ext_types.iter_mut().enumerate() {
    if function.name.starts_with(&ext_type.prefix) {
      if found != 0 {
        panic!("There were multiple exttypes that matched")
      }
      found = idx;
    }
  }

  if found != 0 {
    let ext_type = &mut ext_types[found];
    function.name = function
      .name
      .strip_prefix(&ext_type.prefix)
      .unwrap()
      .to_string();
    function.ext_type = true;
    ext_type.functions.push(function);
    None
  } else {
    function.name = function
      .name
      .strip_prefix("nvim_")
      .unwrap_or(&function.name)
      .to_string();
    Some(function)
  }
}
