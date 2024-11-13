use std::{
    io::{Read, Write},
    net::Shutdown,
    os::windows::io::AsRawSocket,
    path::Path,
};

use windows::Win32::Networking::WinSock::{connect, getpeername};

use crate::{init, map_ret, sockaddr_un, socket::WindowsSocket};

#[derive(Debug)]
pub struct UnixStream(WindowsSocket);

impl UnixStream {
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

    pub fn shutdown(&self, how: Shutdown) -> std::io::Result<()> {
        self.0.shutdown(how)
    }

    pub fn as_raw(&self) -> &WindowsSocket {
        &self.0
    }
}

impl AsRawSocket for UnixStream {
    fn as_raw_socket(&self) -> std::os::windows::io::RawSocket {
        self.0.as_raw().0 as _
    }
}

impl Read for UnixStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Read::read(&mut &*self, buf)
    }
}

impl<'a> Read for &'a UnixStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl Write for UnixStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Write::write(&mut &*self, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Write::flush(&mut &*self)
    }
}

impl<'a> Write for &'a UnixStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
