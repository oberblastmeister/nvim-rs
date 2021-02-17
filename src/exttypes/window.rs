use futures::io::AsyncWrite;
use rmpv::Value;

use crate::rpc::model::IntoVal;
use crate::rpc::unpack::TryUnpack;
use crate::{error::CallError, Neovim};
use crate::{Buffer, Tabpage};
use serde::Serialize;

/// A struct representing a neovim window. It is specific to a
/// [`Neovim`](crate::neovim::Neovim) instance, and calling a method on it will
/// always use this instance.
#[derive(Clone)]
pub struct Window<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) code_data: i64,
  pub(crate) neovim: Neovim<W>,
}

#[derive(Serialize)]
struct Args(i64);

impl<W> Window<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  /// since: 1
  pub async fn get_buf(&self) -> Result<Buffer<W>, Box<CallError>> {
    Ok(
      self
        .neovim
        .call("nvim_win_get_buf", Args(self.code_data.clone()))
        .await?
        .map(|val| Buffer {
          code_data: val.try_unpack().unwrap(),
          neovim: self.neovim.clone(),
        })?,
    )
  }
  /// since: 1
  pub async fn get_tabpage(&self) -> Result<Tabpage<W>, Box<CallError>> {
    Ok(
      self
        .neovim
        .call("nvim_win_get_tabpage", Args(self.code_data.clone()))
        .await?
        .map(|val| Tabpage {
          code_data: val.try_unpack().unwrap(),
          neovim: self.neovim.clone(),
        })?,
    )
  }
}
