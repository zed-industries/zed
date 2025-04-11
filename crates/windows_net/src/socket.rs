use std::{
    io::{Error, ErrorKind, Result},
    path::Path,
};

use windows::Win32::{
    Foundation::{HANDLE, HANDLE_FLAG_INHERIT, HANDLE_FLAGS, SetHandleInformation},
    Networking::WinSock::{
        ADDRESS_FAMILY, AF_UNIX, SEND_RECV_FLAGS, SOCK_STREAM, SOCKADDR, SOCKADDR_UN, SOCKET,
        SOCKET_ERROR, WSA_FLAG_OVERLAPPED, WSASocketW, accept, closesocket, recv, send,
    },
};

use crate::util::map_ret;

pub(crate) struct WindowsSocket(SOCKET);

impl WindowsSocket {
    pub(crate) fn new() -> Result<Self> {
        unsafe {
            let raw = WSASocketW(AF_UNIX as _, SOCK_STREAM.0, 0, None, 0, WSA_FLAG_OVERLAPPED)?;
            SetHandleInformation(
                HANDLE(raw.0 as _),
                HANDLE_FLAG_INHERIT.0,
                HANDLE_FLAGS::default(),
            )?;
            Ok(Self(raw))
        }
    }

    pub(crate) fn as_raw(&self) -> SOCKET {
        self.0
    }

    pub(crate) fn accept(&self, storage: *mut SOCKADDR, len: &mut i32) -> Result<Self> {
        Ok(Self(unsafe { accept(self.0, Some(storage), Some(len)) }?))
    }

    pub(crate) fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        map_ret(unsafe { recv(self.0, buf, SEND_RECV_FLAGS::default()) })
    }

    pub(crate) fn send(&self, buf: &[u8]) -> Result<usize> {
        map_ret(unsafe { send(self.0, buf, SEND_RECV_FLAGS::default()) })
    }
}

impl Drop for WindowsSocket {
    fn drop(&mut self) {
        unsafe { closesocket(self.0) };
    }
}
