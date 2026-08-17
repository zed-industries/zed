//! Buffers, windows, tabpages of neovim
mod buffer;
mod tabpage;
mod window;

pub use buffer::Buffer;
pub use tabpage::Tabpage;
pub use window::Window;

/// A macro to implement trait for the [`exttypes`](crate::exttypes)
#[macro_export]
macro_rules! impl_exttype_traits {
  ($ext:ident) => {
    impl<W> PartialEq for $ext<W>
    where
      W: AsyncWrite + Send + Unpin + 'static,
    {
      fn eq(&self, other: &Self) -> bool {
        self.code_data == other.code_data && self.neovim == other.neovim
      }
    }
    impl<W> Eq for $ext<W> where W: AsyncWrite + Send + Unpin + 'static {}

    impl<W> Clone for $ext<W>
    where
      W: AsyncWrite + Send + Unpin + 'static,
    {
      fn clone(&self) -> Self {
        Self {
          code_data: self.code_data.clone(),
          neovim: self.neovim.clone(),
        }
      }
    }

    impl<W> IntoVal<Value> for &$ext<W>
    where
      W: AsyncWrite + Send + Unpin + 'static,
    {
      fn into_val(self) -> Value {
        self.code_data.clone()
      }
    }
  };
}
