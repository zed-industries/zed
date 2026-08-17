use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::io::AsyncWrite;
use tokio::net::TcpStream;

use crate::io::ReadBuf;
use crate::net::Socket;

impl Socket for TcpStream {
    fn try_read(&mut self, mut buf: &mut dyn ReadBuf) -> io::Result<usize> {
        // Requires `&mut impl BufMut`
        self.try_read_buf(&mut buf)
    }

    fn try_write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (*self).try_write(buf)
    }

    fn poll_read_ready(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        (*self).poll_read_ready(cx)
    }

    fn poll_write_ready(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        (*self).poll_write_ready(cx)
    }

    fn poll_shutdown(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(self).poll_shutdown(cx)
    }
}

#[cfg(unix)]
impl Socket for tokio::net::UnixStream {
    fn try_read(&mut self, mut buf: &mut dyn ReadBuf) -> io::Result<usize> {
        self.try_read_buf(&mut buf)
    }

    fn try_write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (*self).try_write(buf)
    }

    fn poll_read_ready(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        (*self).poll_read_ready(cx)
    }

    fn poll_write_ready(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        (*self).poll_write_ready(cx)
    }

    fn poll_shutdown(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(self).poll_shutdown(cx)
    }
}
