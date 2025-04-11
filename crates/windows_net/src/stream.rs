use std::{
    io::{Read, Result, Write},
    os::windows::io::{AsSocket, BorrowedSocket},
    path::Path,
};

use async_io::IoSafe;
use windows::Win32::Networking::WinSock::connect;

use crate::{
    socket::WindowsSocket,
    util::{init, map_ret, sockaddr_un},
};

pub(crate) struct WindowsStream(WindowsSocket);

unsafe impl IoSafe for WindowsStream {}

impl WindowsStream {
    pub(crate) fn new(socket: WindowsSocket) -> Self {
        Self(socket)
    }

    pub(crate) fn connect(path: &Path) -> Result<Self> {
        init();
        unsafe {
            let inner = WindowsSocket::new()?;
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

impl Read for WindowsStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.recv(buf)
    }
}

impl Write for WindowsStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.send(buf)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl AsSocket for WindowsStream {
    fn as_socket(&self) -> BorrowedSocket<'_> {
        unsafe { BorrowedSocket::borrow_raw(self.0.as_raw().0 as _) }
    }
}
