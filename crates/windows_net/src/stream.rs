use std::{
    io::{Read, Write},
    os::windows::io::AsRawSocket,
    path::Path,
    pin::Pin,
    sync::Arc,
    task::Poll,
};

use async_io::{ReadableOwned, WritableOwned};
use smol::{
    future::FutureExt,
    io::{AsyncRead, AsyncWrite},
    ready, Async,
};
use windows::Win32::Networking::WinSock::{connect, getpeername};

use crate::{
    init, map_ret, sockaddr_un,
    socket::{UnixSocketAddr, WindowsSocket},
};

#[derive(Debug)]
pub struct UnixStream {
    inner: Arc<Async<WindowsStream>>,
    readable: Option<ReadableOwned<WindowsStream>>,
    writable: Option<WritableOwned<WindowsStream>>,
}

#[derive(Debug)]
pub struct WindowsStream(WindowsSocket);

impl UnixStream {
    pub fn new(raw: Arc<Async<WindowsStream>>) -> Self {
        Self {
            inner: raw,
            readable: None,
            writable: None,
        }
    }

    pub fn peer_addr(&self) -> std::io::Result<UnixSocketAddr> {
        UnixSocketAddr::new(|addr, len| unsafe {
            getpeername(*self.inner.get_ref().0.as_raw(), addr, len)
        })
    }

    pub fn as_inner(&self) -> &Arc<Async<WindowsStream>> {
        &self.inner
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

impl AsyncWrite for UnixStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        loop {
            // Attempt the non-blocking operation.
            match self.inner.get_ref().write(buf) {
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {}
                res => {
                    self.writable = None;
                    return Poll::Ready(res);
                }
            }

            // Initialize the future to wait for readiness.
            if self.writable.is_none() {
                self.writable = Some(self.inner.clone().writable_owned());
            }

            // Poll the future for readiness.
            if let Some(f) = &mut self.writable {
                let res = ready!(Pin::new(f).poll(cx));
                self.writable = None;
                res?;
            }
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        todo!()
    }

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        todo!()
    }
}

impl WindowsStream {
    pub fn new(raw: WindowsSocket) -> Self {
        Self(raw)
    }

    pub fn connect<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        init();
        unsafe {
            let inner = WindowsSocket::new()?;
            let (addr, len) = sockaddr_un(path.as_ref())?;

            map_ret(connect(
                *inner.as_raw(),
                &addr as *const _ as *const _,
                len as i32,
            ))?;
            Ok(Self(inner))
        }
    }

    pub fn as_raw(&self) -> &WindowsSocket {
        &self.0
    }
}

impl AsRawSocket for WindowsStream {
    fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
        self.0.as_raw().0 as _
    }
}

impl Read for WindowsStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Read::read(&mut &*self, buf)
    }
}

impl<'a> Read for &'a WindowsStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl Write for WindowsStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Write::write(&mut &*self, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Write::flush(&mut &*self)
    }
}

impl<'a> Write for &'a WindowsStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
