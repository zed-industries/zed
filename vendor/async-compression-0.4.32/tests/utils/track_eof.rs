#[cfg_attr(not(feature = "all-implementations"), allow(unused))]
use std::{
    io::Result,
    pin::Pin,
    task::{Context, Poll},
};

pub struct TrackEof<R> {
    inner: R,
    eof: bool,
}

impl<R: Unpin> TrackEof<R> {
    pub fn new(inner: R) -> Self {
        Self { inner, eof: false }
    }

    pub fn project(self: Pin<&mut Self>) -> (Pin<&mut R>, &mut bool) {
        let Self { inner, eof } = Pin::into_inner(self);
        (Pin::new(inner), eof)
    }
}

#[cfg(feature = "futures-io")]
impl<R: futures::io::AsyncRead + Unpin> futures::io::AsyncRead for TrackEof<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        let (inner, eof) = self.project();
        assert!(!*eof);
        match inner.poll_read(cx, buf) {
            Poll::Ready(Ok(0)) => {
                if !buf.is_empty() {
                    *eof = true;
                }
                Poll::Ready(Ok(0))
            }
            other => other,
        }
    }
}

#[cfg(feature = "futures-io")]
impl<R: futures::io::AsyncBufRead + Unpin> futures::io::AsyncBufRead for TrackEof<R> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<&[u8]>> {
        let (inner, eof) = self.project();
        assert!(!*eof);
        match inner.poll_fill_buf(cx) {
            Poll::Ready(Ok(buf)) => {
                if buf.is_empty() {
                    *eof = true;
                }
                Poll::Ready(Ok(buf))
            }
            other => other,
        }
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        self.project().0.consume(amt)
    }
}

#[cfg(feature = "tokio")]
impl<R: tokio::io::AsyncRead + Unpin> tokio::io::AsyncRead for TrackEof<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        let (inner, eof) = self.project();
        assert!(!*eof);
        let len = buf.filled().len();
        match inner.poll_read(cx, buf) {
            Poll::Ready(Ok(())) => {
                if buf.filled().len() == len && buf.remaining() > 0 {
                    *eof = true;
                }
                Poll::Ready(Ok(()))
            }
            other => other,
        }
    }
}

#[cfg(feature = "tokio")]
impl<R: tokio::io::AsyncBufRead + Unpin> tokio::io::AsyncBufRead for TrackEof<R> {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<&[u8]>> {
        let (inner, eof) = self.project();
        assert!(!*eof);
        match inner.poll_fill_buf(cx) {
            Poll::Ready(Ok(buf)) => {
                if buf.is_empty() {
                    *eof = true;
                }
                Poll::Ready(Ok(buf))
            }
            other => other,
        }
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        self.project().0.consume(amt)
    }
}
