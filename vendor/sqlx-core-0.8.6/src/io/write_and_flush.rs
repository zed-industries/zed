use crate::error::Error;
use sqlx_rt::AsyncWrite;
use std::future::Future;
use std::io::{BufRead, Cursor};
use std::pin::Pin;
use std::task::{ready, Context, Poll};

// Atomic operation that writes the full buffer to the stream, flushes the stream, and then
// clears the buffer (even if either of the two previous operations failed).
pub struct WriteAndFlush<'a, S> {
    pub(super) stream: &'a mut S,
    pub(super) buf: Cursor<&'a mut Vec<u8>>,
}

impl<S: AsyncWrite + Unpin> Future for WriteAndFlush<'_, S> {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self {
            ref mut stream,
            ref mut buf,
        } = *self;

        loop {
            let read = buf.fill_buf()?;

            if !read.is_empty() {
                let written = ready!(Pin::new(&mut *stream).poll_write(cx, read)?);
                buf.consume(written);
            } else {
                break;
            }
        }

        Pin::new(stream).poll_flush(cx).map_err(Error::Io)
    }
}

impl<'a, S> Drop for WriteAndFlush<'a, S> {
    fn drop(&mut self) {
        // clear the buffer regardless of whether the flush succeeded or not
        self.buf.get_mut().clear();
    }
}
