use futures::io::AsyncWrite;
use rmpv::Value;

use crate::{
  error::CallError, exttypes::Window, impl_exttype_traits, rpc::model::IntoVal,
  Neovim,
};

/// A struct representing a neovim tabpage. It is specific to a
/// [`Neovim`](crate::neovim::Neovim) instance, and calling a method on it will
/// always use this instance.
pub struct Tabpage<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub(crate) code_data: Value,
  pub(crate) neovim: Neovim<W>,
}

impl_exttype_traits!(Tabpage);

impl<W> Tabpage<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  /// since: 1
  pub async fn list_wins(&self) -> Result<Vec<Window<W>>, Box<CallError>> {
    match self
      .neovim
      .call("nvim_tabpage_list_wins", call_args![self.code_data.clone()])
      .await??
    {
      Value::Array(arr) => Ok(
        arr
          .into_iter()
          .map(|v| Window::new(v, self.neovim.clone()))
          .collect(),
      ),
      val => Err(CallError::WrongValueType(val))?,
    }
  }
  /// since: 1
  pub async fn get_win(&self) -> Result<Window<W>, Box<CallError>> {
    Ok(
      self
        .neovim
        .call("nvim_tabpage_get_win", call_args![self.code_data.clone()])
        .await?
        .map(|val| Window::new(val, self.neovim.clone()))?,
    )
  }
}
