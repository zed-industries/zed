use std::net::Shutdown;

use windows::Win32::{
    Foundation::{SetHandleInformation, HANDLE, HANDLE_FLAGS, HANDLE_FLAG_INHERIT},
    Networking::WinSock::{
        accept, closesocket, recv, send, shutdown, WSAGetLastError, WSASocketW, AF_UNIX, SD_BOTH,
        SD_RECEIVE, SD_SEND, SEND_RECV_FLAGS, SOCKADDR, SOCKADDR_UN, SOCKET, SOCKET_ERROR,
        SOCK_STREAM, WINSOCK_SHUTDOWN_HOW, WSA_FLAG_NO_HANDLE_INHERIT, WSA_FLAG_OVERLAPPED,
    },
};

use crate::{map_ret, sun_path_offset};

#[derive(Debug)]
pub struct WindowsSocket(SOCKET);

#[derive(Debug)]
pub struct UnixSocketAddr {
    addr: SOCKADDR_UN,
    len: i32,
}

impl WindowsSocket {
    pub(crate) fn new() -> std::io::Result<Self> {
        unsafe {
            let raw = WSASocketW(
                AF_UNIX as _,
                SOCK_STREAM.0,
                0,
                None,
                0,
                WSA_FLAG_OVERLAPPED,
                // WSA_FLAG_NO_HANDLE_INHERIT,
            )?;
            SetHandleInformation(
                HANDLE(raw.0 as _),
                HANDLE_FLAG_INHERIT.0,
                HANDLE_FLAGS::default(),
            )?;
            Ok(Self(raw))
        }
    }

    pub(crate) fn from_socket(socket: SOCKET) -> Self {
        Self(socket)
    }

    pub(crate) fn as_raw(&self) -> &SOCKET {
        &self.0
    }

    pub(crate) fn accept(&self, storage: *mut SOCKADDR, len: &mut i32) -> std::io::Result<Self> {
        Self::static_accept(self.0, storage, len)
    }

    pub fn static_accept(
        socket: SOCKET,
        storage: *mut SOCKADDR,
        len: &mut i32,
    ) -> std::io::Result<Self> {
        Ok(Self(unsafe { accept(socket, Some(storage), Some(len)) }?))
    }

    fn recv_with_flags(&self, buf: &mut [u8], flags: SEND_RECV_FLAGS) -> std::io::Result<usize> {
        let ret = unsafe { recv(self.0, buf, flags) };
        if ret == SOCKET_ERROR {
            return Err(std::io::Error::last_os_error());
        }
        Ok(ret as usize)
    }

    pub(crate) fn read(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.recv_with_flags(buf, SEND_RECV_FLAGS::default())
    }

    pub(crate) fn write(&self, buf: &[u8]) -> std::io::Result<usize> {
        map_ret(unsafe { send(self.0, buf, SEND_RECV_FLAGS::default()) })
    }

    pub fn shutdown(&self, how: Shutdown) -> std::io::Result<()> {
        let how = match how {
            Shutdown::Read => SD_RECEIVE,
            Shutdown::Write => SD_SEND,
            Shutdown::Both => SD_BOTH,
        };
        map_ret(unsafe { shutdown(self.0, how) })?;
        Ok(())
    }
}

impl Drop for WindowsSocket {
    fn drop(&mut self) {
        unsafe { closesocket(self.0) };
    }
}

impl UnixSocketAddr {
    pub(crate) fn from_parts(addr: SOCKADDR_UN, mut len: i32) -> std::io::Result<Self> {
        if len == 0 {
            // When there is a datagram from unnamed unix socket
            // linux returns zero bytes of address
            len = sun_path_offset(&addr) as i32; // i.e. zero-length address
        } else if addr.sun_family.0 != AF_UNIX {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "file descriptor did not correspond to a Unix socket",
            ));
        }

        Ok(Self { addr, len })
    }

    pub(crate) fn new<F>(f: F) -> std::io::Result<Self>
    where
        F: FnOnce(*mut SOCKADDR, *mut i32) -> i32,
    {
        let mut addr = SOCKADDR_UN::default();
        let mut len = std::mem::size_of::<SOCKADDR_UN>() as i32;
        map_ret(f(&mut addr as *mut _ as *mut _, &mut len))?;
        Self::from_parts(addr, len)
    }
}
