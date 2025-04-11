use std::{
    io::Result,
    os::windows::io::{AsSocket, BorrowedSocket},
    path::Path,
};

use windows::Win32::Networking::WinSock::{SOCKADDR_UN, SOMAXCONN, bind, listen};

use crate::{
    socket::WindowsSocket,
    stream::WindowsStream,
    util::{init, map_ret, sockaddr_un},
};

pub(crate) struct WindowsListener(WindowsSocket);

impl WindowsListener {
    pub(crate) fn bind(path: &Path) -> Result<Self> {
        init();
        let socket = WindowsSocket::new()?;
        let (addr, len) = sockaddr_un(path)?;
        unsafe {
            map_ret(bind(
                socket.as_raw(),
                &addr as *const _ as *const _,
                len as i32,
            ))?;
            map_ret(listen(socket.as_raw(), SOMAXCONN as _))?;
        }
        Ok(Self(socket))
    }

    pub(crate) fn accept(&self) -> Result<WindowsStream> {
        let mut storage = SOCKADDR_UN::default();
        let mut len = std::mem::size_of_val(&storage) as i32;
        let raw = self.0.accept(&mut storage as *mut _ as *mut _, &mut len)?;
        Ok(WindowsStream::new(raw))
    }
}

impl AsSocket for WindowsListener {
    fn as_socket(&self) -> BorrowedSocket<'_> {
        unsafe { BorrowedSocket::borrow_raw(self.0.as_raw().0 as _) }
    }
}
