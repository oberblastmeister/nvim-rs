mod codegen;
mod utils;
mod api;
mod syn_helpers;

use eyre::Result;

fn main() -> Result<()> {
    codegen::run()?;

    Ok(())
}
