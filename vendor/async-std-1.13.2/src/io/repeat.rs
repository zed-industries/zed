use std::fmt;
use std::pin::Pin;

use crate::io::{self, Read};
use crate::task::{Context, Poll};

/// Creates an instance of a reader that infinitely repeats one byte.
///
/// All reads from this reader will succeed by filling the specified buffer with the given byte.
///
/// ## Examples
///
/// ```rust
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::io;
/// use async_std::prelude::*;
///
/// let mut buffer = [0; 3];
/// io::repeat(0b101).read_exact(&mut buffer).await?;
///
/// assert_eq!(buffer, [0b101, 0b101, 0b101]);
/// #
/// # Ok(()) }) }
/// ```
pub fn repeat(byte: u8) -> Repeat {
    Repeat { byte }
}

/// A reader which yields one byte over and over and over and over and over and...
///
/// This reader is created by the [`repeat`] function. See its
/// documentation for more.
///
/// [`repeat`]: fn.repeat.html
pub struct Repeat {
    byte: u8,
}

impl fmt::Debug for Repeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Empty { .. }")
    }
}

impl Read for Repeat {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        for b in &mut *buf {
            *b = self.byte;
        }
        Poll::Ready(Ok(buf.len()))
    }
}
