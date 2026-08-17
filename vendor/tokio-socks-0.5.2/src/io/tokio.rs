//! AsyncSocket trait implementation for tokio's AsyncRead + AsyncWrite
//! traits.
use std::{
    io::Result as IoResult,
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::ready;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use super::AsyncSocket;

impl<S> AsyncSocket for S
where S: AsyncRead + AsyncWrite
{
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<IoResult<usize>> {
        let mut buf = ReadBuf::new(buf);
        ready!(AsyncRead::poll_read(self, cx, &mut buf))?;
        Poll::Ready(Ok(buf.filled().len()))
    }

    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<IoResult<usize>> {
        AsyncWrite::poll_write(self, cx, buf)
    }
}
