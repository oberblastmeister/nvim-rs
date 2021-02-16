mod codegen;
mod utils;
mod api;

use eyre::Result;

fn main() -> Result<()> {
    codegen::run()?;

    Ok(())
}
