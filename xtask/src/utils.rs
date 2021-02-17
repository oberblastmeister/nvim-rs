use std::{env, path::PathBuf};
use std::path::Path;

use eyre::Result;
use xshell::cmd;

pub fn xtask_root() -> PathBuf {
  Path::new(
    &env::var("CARGO_MANIFEST_DIR")
      .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
  )
  .to_path_buf()
}

pub fn project_root() -> PathBuf {
  xtask_root().ancestors().nth(1).unwrap().to_path_buf()
}

pub fn nvim_path() -> PathBuf {
  project_root().join("neovim/build/bin/nvim")
}

pub const PREAMBLE: &str =
  "Generated file, do not edit by hand, see `xtask/src/codegen`";

pub fn reformat(text: &str) -> Result<String> {
  let stdout = cmd!("rustfmt").stdin(text).read()?;
  Ok(format!("//! {}\n\n{}\n", PREAMBLE, stdout))
}
