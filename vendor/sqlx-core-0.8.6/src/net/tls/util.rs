use crate::net::Socket;

use std::io::{self, Read, Write};
use std::task::{ready, Context, Poll};

use futures_util::future;

pub struct StdSocket<S> {
    pub socket: S,
    wants_read: bool,
    wants_write: bool,
}

impl<S: Socket> StdSocket<S> {
    pub fn new(socket: S) -> Self {
        Self {
            socket,
            wants_read: false,
            wants_write: false,
        }
    }

    pub fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        if self.wants_write {
            ready!(self.socket.poll_write_ready(cx))?;
            self.wants_write = false;
        }

        if self.wants_read {
            ready!(self.socket.poll_read_ready(cx))?;
            self.wants_read = false;
        }

        Poll::Ready(Ok(()))
    }

    pub async fn ready(&mut self) -> io::Result<()> {
        future::poll_fn(|cx| self.poll_ready(cx)).await
    }
}

impl<S: Socket> Read for StdSocket<S> {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        self.wants_read = true;
        let read = self.socket.try_read(&mut buf)?;
        self.wants_read = false;

        Ok(read)
    }
}

impl<S: Socket> Write for StdSocket<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.wants_write = true;
        let written = self.socket.try_write(buf)?;
        self.wants_write = false;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        // NOTE: TCP sockets and unix sockets are both no-ops for flushes
        Ok(())
    }
}
