#[cfg_attr(not(feature = "all-implementations"), allow(unused))]
use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

pub struct TrackClosed<W> {
    inner: W,
    closed: bool,
}

impl<W> TrackClosed<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            closed: false,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}

#[cfg(feature = "futures-io")]
impl<W: futures::io::AsyncWrite + Unpin> futures::io::AsyncWrite for TrackClosed<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize>> {
        assert!(!self.closed);
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        assert!(!self.closed);
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        assert!(!self.closed);
        match Pin::new(&mut self.inner).poll_close(cx) {
            Poll::Ready(Ok(())) => {
                self.closed = true;
                Poll::Ready(Ok(()))
            }
            other => other,
        }
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<Result<usize>> {
        assert!(!self.closed);
        Pin::new(&mut self.inner).poll_write_vectored(cx, bufs)
    }
}

#[cfg(feature = "tokio")]
impl<W: tokio::io::AsyncWrite + Unpin> tokio::io::AsyncWrite for TrackClosed<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize>> {
        assert!(!self.closed);
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        assert!(!self.closed);
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        assert!(!self.closed);
        match Pin::new(&mut self.inner).poll_shutdown(cx) {
            Poll::Ready(Ok(())) => {
                self.closed = true;
                Poll::Ready(Ok(()))
            }
            other => other,
        }
    }
}
