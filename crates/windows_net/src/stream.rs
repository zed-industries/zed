use std::{
    io::{Read, Result, Write},
    os::windows::io::{AsSocket, BorrowedSocket},
    path::Path,
};

use async_io::IoSafe;
use windows::Win32::Networking::WinSock::connect;

use crate::{
    socket::UnixSocket,
    util::{init, map_ret, sockaddr_un},
};

pub struct UnixStream(UnixSocket);

unsafe impl IoSafe for UnixStream {}

impl UnixStream {
    pub fn new(socket: UnixSocket) -> Self {
        Self(socket)
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
            Ok(Self(inner))
        }
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
