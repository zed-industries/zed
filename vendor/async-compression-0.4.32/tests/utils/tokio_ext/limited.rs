use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug)]
pub struct Limited<Io> {
    io: Io,
    limit: usize,
}

impl<Io> Limited<Io> {
    pub(crate) fn new(io: Io, limit: usize) -> Limited<Io> {
        Limited { io, limit }
    }
}

impl<W: tokio::io::AsyncWrite + Unpin> tokio::io::AsyncWrite for Limited<W> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let limit = self.limit;
        Pin::new(&mut self.io).poll_write(cx, &buf[..std::cmp::min(limit, buf.len())])
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.io).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.io).poll_shutdown(cx)
    }
}
