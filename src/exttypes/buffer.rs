use futures::io::AsyncWrite;
use rmpv::Value;

use crate::{rpc::model::IntoVal, Neovim};
/// A struct representing a neovim buffer. It is specific to a
/// [`Neovim`](crate::neovim::Neovim) instance, and calling a method on it will
/// always use this instance.
#[derive(Clone)]
pub struct Buffer<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) code_data: u32,
  pub(crate) neovim: Neovim<W>,
}
