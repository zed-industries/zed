use crate::io::AsyncWrite;

use bytes::Buf;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io::{self, IoSlice};
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

pin_project! {
    /// A future to write some of the buffer to an `AsyncWrite`.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WriteBuf<'a, W, B> {
        writer: &'a mut W,
        buf: &'a mut B,
        #[pin]
        _pin: PhantomPinned,
    }
}

/// Tries to write some bytes from the given `buf` to the writer in an
/// asynchronous manner, returning a future.
pub(crate) fn write_buf<'a, W, B>(writer: &'a mut W, buf: &'a mut B) -> WriteBuf<'a, W, B>
where
    W: AsyncWrite + Unpin,
    B: Buf,
{
    WriteBuf {
        writer,
        buf,
        _pin: PhantomPinned,
    }
}

impl<W, B> Future for WriteBuf<'_, W, B>
where
    W: AsyncWrite + Unpin,
    B: Buf,
{
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<usize>> {
        const MAX_VECTOR_ELEMENTS: usize = 64;

        let me = self.project();

        if !me.buf.has_remaining() {
            return Poll::Ready(Ok(0));
        }

        let n = if me.writer.is_write_vectored() {
            let mut slices = [IoSlice::new(&[]); MAX_VECTOR_ELEMENTS];
            let cnt = me.buf.chunks_vectored(&mut slices);
            ready!(Pin::new(&mut *me.writer).poll_write_vectored(cx, &slices[..cnt]))?
        } else {
            ready!(Pin::new(&mut *me.writer).poll_write(cx, me.buf.chunk()))?
        };

        me.buf.advance(n);
        Poll::Ready(Ok(n))
    }
}
