use std::{
    io::{Read, Result, Write},
    os::windows::io::{AsSocket, BorrowedSocket},
    path::Path,
    sync::Arc,
};

use async_io::IoSafe;
use windows::Win32::Networking::WinSock::connect;

use crate::{
    socket::UnixSocket,
    util::{init, map_ret, sockaddr_un},
};

pub struct UnixStream(Arc<UnixSocket>);

unsafe impl IoSafe for UnixStream {}

impl UnixStream {
    pub fn new(socket: UnixSocket) -> Self {
        Self(Arc::new(socket))
    }

    pub fn connect<P: AsRef<Path>>(path: P) -> Result<Self> {
        init();
        unsafe {
            let inner = UnixSocket::new()?;
            let (addr, len) = sockaddr_un(path)?;

            map_ret(connect(
                inner.as_raw(),
                &addr as *const _ as *const _,
                len as i32,
            ))?;
            Ok(Self(Arc::new(inner)))
        }
    }

    pub fn into_split(self) -> (OwnedReadHalf, OwnedWriteHalf) {
        let inner = self.0;
        (OwnedReadHalf(inner.clone()), OwnedWriteHalf(inner))
    }
}

impl Read for UnixStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.recv(buf)
    }
}

impl Write for UnixStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.send(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl AsSocket for UnixStream {
    fn as_socket(&self) -> BorrowedSocket<'_> {
        unsafe { BorrowedSocket::borrow_raw(self.0.as_raw().0 as _) }
    }
}

pub struct OwnedReadHalf(Arc<UnixSocket>);

impl Read for OwnedReadHalf {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.recv(buf)
    }
}

pub struct OwnedWriteHalf(Arc<UnixSocket>);

impl Write for OwnedWriteHalf {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.send(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
