use std::fmt;
use std::pin::Pin;

use crate::io::{self, BufRead, Read};
use crate::task::{Context, Poll};

/// Creates a reader that contains no data.
///
/// # Examples
///
/// ```rust
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::io;
/// use async_std::prelude::*;
///
/// let mut buf = Vec::new();
/// let mut reader = io::empty();
/// reader.read_to_end(&mut buf).await?;
///
/// assert!(buf.is_empty());
/// #
/// # Ok(()) }) }
/// ```
pub fn empty() -> Empty {
    Empty { _private: () }
}

/// A reader that contains no data.
///
/// This reader is created by the [`empty`] function. See its
/// documentation for more.
///
/// [`empty`]: fn.empty.html
pub struct Empty {
    _private: (),
}

impl fmt::Debug for Empty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Empty { .. }")
    }
}

impl Read for Empty {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        _: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }
}

impl BufRead for Empty {
    #[inline]
    fn poll_fill_buf<'a>(
        self: Pin<&'a mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<io::Result<&'a [u8]>> {
        Poll::Ready(Ok(&[]))
    }

    #[inline]
    fn consume(self: Pin<&mut Self>, _: usize) {}
}
