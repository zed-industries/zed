use std::mem;
use std::pin::Pin;
use std::str;

use pin_project_lite::pin_project;

use super::read_until_internal;
use crate::io::{self, BufRead};
use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    /// A stream of lines in a byte stream.
    ///
    /// This stream is created by the [`lines`] method on types that implement [`BufRead`].
    ///
    /// This type is an async version of [`std::io::Lines`].
    ///
    /// [`lines`]: trait.BufRead.html#method.lines
    /// [`BufRead`]: trait.BufRead.html
    /// [`std::io::Lines`]: https://doc.rust-lang.org/std/io/struct.Lines.html
    #[derive(Debug)]
    pub struct Lines<R> {
        #[pin]
        pub(crate) reader: R,
        pub(crate) buf: String,
        pub(crate) bytes: Vec<u8>,
        pub(crate) read: usize,
    }
}

impl<R: BufRead> Stream for Lines<R> {
    type Item = io::Result<String>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let n = futures_core::ready!(read_line_internal(
            this.reader,
            cx,
            this.buf,
            this.bytes,
            this.read
        ))?;
        if n == 0 && this.buf.is_empty() {
            return Poll::Ready(None);
        }
        if this.buf.ends_with('\n') {
            this.buf.pop();
            if this.buf.ends_with('\r') {
                this.buf.pop();
            }
        }
        Poll::Ready(Some(Ok(mem::replace(this.buf, String::new()))))
    }
}

pub fn read_line_internal<R: BufRead + ?Sized>(
    reader: Pin<&mut R>,
    cx: &mut Context<'_>,
    buf: &mut String,
    bytes: &mut Vec<u8>,
    read: &mut usize,
) -> Poll<io::Result<usize>> {
    let ret = futures_core::ready!(read_until_internal(reader, cx, b'\n', bytes, read));
    if str::from_utf8(&bytes).is_err() {
        Poll::Ready(ret.and_then(|_| {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "stream did not contain valid UTF-8",
            ))
        }))
    } else {
        debug_assert!(buf.is_empty());
        debug_assert_eq!(*read, 0);
        // Safety: `bytes` is a valid UTF-8 because `str::from_utf8` returned `Ok`.
        mem::swap(unsafe { buf.as_mut_vec() }, bytes);
        Poll::Ready(ret)
    }
}
