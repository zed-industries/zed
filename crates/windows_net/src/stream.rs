use std::{os::windows::io::AsRawSocket, pin::Pin, sync::Arc, task::Poll};

use async_io::{ReadableOwned, WritableOwned};
use smol::{future::FutureExt, io::AsyncRead, ready, Async};

use crate::socket::RawSocket;

pub struct UnixStream {
    inner: Arc<Async<RawUnixStream>>,
    readable: Option<ReadableOwned<RawUnixStream>>,
    writable: Option<WritableOwned<RawUnixStream>>,
}

pub struct RawUnixStream(RawSocket);

impl UnixStream {
    pub fn new(raw: Arc<Async<RawUnixStream>>) -> Self {
        Self {
            inner: raw,
            readable: None,
            writable: None,
        }
    }

    pub async fn write_all(&mut self, mut buf: &[u8]) -> std::io::Result<()> {
        self.inner.write_with(|r| r.write_all(buf)).await
    }
}

impl AsyncRead for UnixStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        loop {
            // Attempt the non-blocking operation.
            match self.inner.get_ref().read(buf) {
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {}
                res => {
                    self.readable = None;
                    return Poll::Ready(res);
                }
            }

            // Initialize the future to wait for readiness.
            if self.readable.is_none() {
                self.readable = Some(self.inner.clone().readable_owned());
            }

            // Poll the future for readiness.
            if let Some(f) = &mut self.readable {
                let res = ready!(Pin::new(f).poll(cx));
                self.readable = None;
                res?;
            }
        }
    }
}

impl RawUnixStream {
    pub fn new(raw: RawSocket) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> &RawSocket {
        &self.0
    }

    fn read(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(Error::WRITE_ALL_EOF);
                }
                Ok(n) => buf = &buf[n..],
                Err(ref e) if e.is_interrupted() => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl AsRawSocket for RawUnixStream {
    fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
        self.0.as_raw().0 as _
    }
}
