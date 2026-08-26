use std::io::{Error, ErrorKind, Result};

use windows::Win32::{
    Foundation::{HANDLE, HANDLE_FLAG_INHERIT, HANDLE_FLAGS, SetHandleInformation},
    Networking::WinSock::{
        AF_UNIX, SEND_RECV_FLAGS, SOCK_STREAM, SOCKADDR, SOCKET, WSA_FLAG_OVERLAPPED,
        WSAEWOULDBLOCK, WSASocketW, accept, closesocket, recv, send,
    },
};

use crate::util::map_ret;

pub struct UnixSocket(SOCKET);

impl UnixSocket {
    pub fn new() -> Result<Self> {
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

    pub fn accept(&self, storage: *mut SOCKADDR, len: &mut i32) -> Result<Self> {
        match unsafe { accept(self.0, Some(storage), Some(len)) } {
            Ok(sock) => Ok(Self(sock)),
            Err(err) => {
                let wsa_err = unsafe { windows::Win32::Networking::WinSock::WSAGetLastError().0 };
                if wsa_err == WSAEWOULDBLOCK.0 {
                    Err(Error::new(ErrorKind::WouldBlock, "accept would block"))
                } else {
                    Err(err.into())
                }
            }
        }
    }

    pub(crate) fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        map_ret(unsafe { recv(self.0, buf, SEND_RECV_FLAGS::default()) })
    }

    pub(crate) fn send(&self, buf: &[u8]) -> Result<usize> {
        map_ret(unsafe { send(self.0, buf, SEND_RECV_FLAGS::default()) })
    }
}

impl Drop for UnixSocket {
    fn drop(&mut self) {
        unsafe { closesocket(self.0) };
    }
}
