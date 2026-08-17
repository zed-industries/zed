//! Some manually implemented API functions
use futures::io::AsyncWrite;
use rmpv::Value;

use crate::{
  error::CallError, neovim::Neovim, rpc::model::IntoVal, Buffer, Tabpage,
  Window,
};

impl<W> Neovim<W>
where
  W: AsyncWrite + Send + Unpin + 'static,
{
  pub async fn list_bufs(&self) -> Result<Vec<Buffer<W>>, Box<CallError>> {
    match self.call("nvim_list_bufs", call_args![]).await?? {
      Value::Array(arr) => Ok(
        arr
          .into_iter()
          .map(|v| Buffer::new(v, self.clone()))
          .collect(),
      ),
      val => Err(CallError::WrongValueType(val))?,
    }
  }

  pub async fn get_current_buf(&self) -> Result<Buffer<W>, Box<CallError>> {
    Ok(
      self
        .call("nvim_get_current_buf", call_args![])
        .await?
        .map(|val| Buffer::new(val, self.clone()))?,
    )
  }

  pub async fn list_wins(&self) -> Result<Vec<Window<W>>, Box<CallError>> {
    match self.call("nvim_list_wins", call_args![]).await?? {
      Value::Array(arr) => Ok(
        arr
          .into_iter()
          .map(|v| Window::new(v, self.clone()))
          .collect(),
      ),
      val => Err(CallError::WrongValueType(val))?,
    }
  }

  pub async fn get_current_win(&self) -> Result<Window<W>, Box<CallError>> {
    Ok(
      self
        .call("nvim_get_current_win", call_args![])
        .await?
        .map(|val| Window::new(val, self.clone()))?,
    )
  }

  pub async fn create_buf(
    &self,
    listed: bool,
    scratch: bool,
  ) -> Result<Buffer<W>, Box<CallError>> {
    Ok(
      self
        .call("nvim_create_buf", call_args![listed, scratch])
        .await?
        .map(|val| Buffer::new(val, self.clone()))?,
    )
  }

  pub async fn open_win(
    &self,
    buffer: &Buffer<W>,
    enter: bool,
    config: Vec<(Value, Value)>,
  ) -> Result<Window<W>, Box<CallError>> {
    Ok(
      self
        .call("nvim_open_win", call_args![buffer, enter, config])
        .await?
        .map(|val| Window::new(val, self.clone()))?,
    )
  }

  pub async fn list_tabpages(&self) -> Result<Vec<Tabpage<W>>, Box<CallError>> {
    match self.call("nvim_list_tabpages", call_args![]).await?? {
      Value::Array(arr) => Ok(
        arr
          .into_iter()
          .map(|v| Tabpage::new(v, self.clone()))
          .collect(),
      ),
      val => Err(CallError::WrongValueType(val))?,
    }
  }

  pub async fn get_current_tabpage(
    &self,
  ) -> Result<Tabpage<W>, Box<CallError>> {
    Ok(
      self
        .call("nvim_get_current_tabpage", call_args![])
        .await?
        .map(|val| Tabpage::new(val, self.clone()))?,
    )
  }
}
