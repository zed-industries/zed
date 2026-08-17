// Originally sourced from `futures_util::io::buf_writer`, needs to be redefined locally so that
// the `AsyncBufWrite` impl can access its internals, and changed a bit to make it more efficient
// with those methods.

use super::AsyncBufWrite;
use futures_core::ready;
use futures_io::{AsyncSeek, AsyncWrite, SeekFrom};
use pin_project_lite::pin_project;
use std::{
    cmp::min,
    fmt, io,
    pin::Pin,
    task::{Context, Poll},
};

const DEFAULT_BUF_SIZE: usize = 8192;

pin_project! {
    pub struct BufWriter<W> {
        #[pin]
        inner: W,
        buf: Box<[u8]>,
        written: usize,
        buffered: usize,
    }
}

impl<W: AsyncWrite> BufWriter<W> {
    /// Creates a new `BufWriter` with a default buffer capacity. The default is currently 8 KB,
    /// but may change in the future.
    pub fn new(inner: W) -> Self {
        Self::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    /// Creates a new `BufWriter` with the specified buffer capacity.
    pub fn with_capacity(cap: usize, inner: W) -> Self {
        Self {
            inner,
            buf: vec![0; cap].into(),
            written: 0,
            buffered: 0,
        }
    }

    fn partial_flush_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut this = self.project();

        let mut ret = Ok(());
        while *this.written < *this.buffered {
            match this
                .inner
                .as_mut()
                .poll_write(cx, &this.buf[*this.written..*this.buffered])
            {
                Poll::Pending => {
                    break;
                }
                Poll::Ready(Ok(0)) => {
                    ret = Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "failed to write the buffered data",
                    ));
                    break;
                }
                Poll::Ready(Ok(n)) => *this.written += n,
                Poll::Ready(Err(e)) => {
                    ret = Err(e);
                    break;
                }
            }
        }

        if *this.written > 0 {
            this.buf.copy_within(*this.written..*this.buffered, 0);
            *this.buffered -= *this.written;
            *this.written = 0;

            Poll::Ready(ret)
        } else if *this.buffered == 0 {
            Poll::Ready(ret)
        } else {
            ret?;
            Poll::Pending
        }
    }

    fn flush_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut this = self.project();

        let mut ret = Ok(());
        while *this.written < *this.buffered {
            match ready!(this
                .inner
                .as_mut()
                .poll_write(cx, &this.buf[*this.written..*this.buffered]))
            {
                Ok(0) => {
                    ret = Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "failed to write the buffered data",
                    ));
                    break;
                }
                Ok(n) => *this.written += n,
                Err(e) => {
                    ret = Err(e);
                    break;
                }
            }
        }
        this.buf.copy_within(*this.written..*this.buffered, 0);
        *this.buffered -= *this.written;
        *this.written = 0;
        Poll::Ready(ret)
    }
}

impl<W> BufWriter<W> {
    /// Gets a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Gets a mutable reference to the underlying writer.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }

    /// Gets a pinned mutable reference to the underlying writer.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut W> {
        self.project().inner
    }

    /// Consumes this `BufWriter`, returning the underlying writer.
    ///
    /// Note that any leftover data in the internal buffer is lost.
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: AsyncWrite> AsyncWrite for BufWriter<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.as_mut().project();
        if *this.buffered + buf.len() > this.buf.len() {
            ready!(self.as_mut().partial_flush_buf(cx))?;
        }

        let this = self.as_mut().project();
        if buf.len() >= this.buf.len() {
            if *this.buffered == 0 {
                this.inner.poll_write(cx, buf)
            } else {
                // The only way that `partial_flush_buf` would have returned with
                // `this.buffered != 0` is if it were Pending, so our waker was already queued
                Poll::Pending
            }
        } else {
            let len = min(this.buf.len() - *this.buffered, buf.len());
            this.buf[*this.buffered..*this.buffered + len].copy_from_slice(&buf[..len]);
            *this.buffered += len;
            Poll::Ready(Ok(len))
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        ready!(self.as_mut().flush_buf(cx))?;
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        ready!(self.as_mut().flush_buf(cx))?;
        self.project().inner.poll_close(cx)
    }
}

impl<W: AsyncWrite> AsyncBufWrite for BufWriter<W> {
    fn poll_partial_flush_buf(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<&mut [u8]>> {
        ready!(self.as_mut().partial_flush_buf(cx))?;
        let this = self.project();
        Poll::Ready(Ok(&mut this.buf[*this.buffered..]))
    }

    fn produce(self: Pin<&mut Self>, amt: usize) {
        let this = self.project();
        debug_assert!(
            *this.buffered + amt <= this.buf.len(),
            "produce called with amt exceeding buffer capacity"
        );
        *this.buffered += amt;
    }
}

impl<W: fmt::Debug> fmt::Debug for BufWriter<W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufWriter")
            .field("writer", &self.inner)
            .field(
                "buffer",
                &format_args!("{}/{}", self.buffered, self.buf.len()),
            )
            .field("written", &self.written)
            .finish()
    }
}

impl<W: AsyncWrite + AsyncSeek> AsyncSeek for BufWriter<W> {
    /// Seek to the offset, in bytes, in the underlying writer.
    ///
    /// Seeking always writes out the internal buffer before seeking.
    fn poll_seek(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: SeekFrom,
    ) -> Poll<io::Result<u64>> {
        ready!(self.as_mut().flush_buf(cx))?;
        self.project().inner.poll_seek(cx, pos)
    }
}
