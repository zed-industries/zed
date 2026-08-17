use std::mem;
use std::pin::Pin;

use pin_project_lite::pin_project;

use super::read_until_internal;
use crate::io::{self, BufRead};
use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    /// A stream over the contents of an instance of [`BufRead`] split on a particular byte.
    ///
    /// This stream is created by the [`split`] method on types that implement [`BufRead`].
    ///
    /// This type is an async version of [`std::io::Split`].
    ///
    /// [`split`]: trait.BufRead.html#method.lines
    /// [`BufRead`]: trait.BufRead.html
    /// [`std::io::Split`]: https://doc.rust-lang.org/std/io/struct.Split.html
    #[derive(Debug)]
    pub struct Split<R> {
        #[pin]
        pub(crate) reader: R,
        pub(crate) buf: Vec<u8>,
        pub(crate) read: usize,
        pub(crate) delim: u8,
    }
}

impl<R: BufRead> Stream for Split<R> {
    type Item = io::Result<Vec<u8>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let n = futures_core::ready!(read_until_internal(
            this.reader,
            cx,
            *this.delim,
            this.buf,
            this.read
        ))?;
        if n == 0 && this.buf.is_empty() {
            return Poll::Ready(None);
        }
        if this.buf[this.buf.len() - 1] == *this.delim {
            this.buf.pop();
        }
        Poll::Ready(Some(Ok(mem::replace(this.buf, vec![]))))
    }
}
