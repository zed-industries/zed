//! Implements support for Unix domain sockets for Windows. This should probably
//! be a part of an external crate such as `uds`, but currently no Rust crates
//! support them, or if they do, use outdated dependencies such as winapi

#![allow(clippy::mem_forget, unsafe_code)]

use std::{
    io,
    os::windows::{
        io::{AsRawSocket, AsSocket, FromRawSocket, IntoRawSocket, RawSocket},
        prelude::BorrowedSocket,
    },
};

#[allow(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]
mod bindings {
    pub const PF_UNIX: i32 = 1;
    pub const SOCK_STREAM: i32 = 1;
    pub const FIONBIO: i32 = -2147195266;
    pub const INVALID_SOCKET: usize = !0;
    pub const SD_SEND: u32 = 1;
    pub const SOCKET_ERROR: i32 = -1;

    #[repr(C)]
    pub struct WSABUF {
        pub len: u32,
        pub buf: *const u8,
    }

    pub type ADDRESS_FAMILY = u16;

    #[repr(C)]
    pub struct SOCKADDR {
        pub sa_family: ADDRESS_FAMILY,
        pub sa_data: [u8; 14],
    }

    pub type BOOL = i32;
    pub type HANDLE = isize;
    pub type HANDLE_FLAGS = u32;
    pub const HANDLE_FLAG_INHERIT: HANDLE_FLAGS = 1;

    pub type SOCKET = usize;

    pub type SEND_RECV_FLAGS = i32;
    pub const MSG_PEEK: SEND_RECV_FLAGS = 2;

    #[repr(C)]
    pub struct OVERLAPPED_0_0 {
        pub Offset: u32,
        pub OffsetHigh: u32,
    }

    #[repr(C)]
    pub union OVERLAPPED_0 {
        pub Anonymous: std::mem::ManuallyDrop<OVERLAPPED_0_0>,
        pub Pointer: *mut std::ffi::c_void,
    }

    #[repr(C)]
    pub struct OVERLAPPED {
        pub Internal: usize,
        pub InternalHigh: usize,
        pub Anonymous: OVERLAPPED_0,
        pub hEvent: HANDLE,
    }

    pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = Option<
        unsafe extern "system" fn(
            dwError: u32,
            cbTransferred: u32,
            lpOverlapped: *mut OVERLAPPED,
            dwFlags: u32,
        ),
    >;

    pub type WSA_ERROR = i32;
    pub const WSAESHUTDOWN: WSA_ERROR = 10058;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        pub fn SetHandleInformation(hObject: HANDLE, dwMask: u32, dwFlags: HANDLE_FLAGS) -> BOOL;
    }

    #[link(name = "ws2_32")]
    unsafe extern "system" {
        pub fn socket(af: i32, type_: i32, protocol: i32) -> SOCKET;
        pub fn closesocket(s: SOCKET) -> i32;
        pub fn accept(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32) -> SOCKET;
        pub fn recv(s: SOCKET, buf: *const u8, len: i32, flags: SEND_RECV_FLAGS) -> i32;
        pub fn WSARecv(
            s: SOCKET,
            lpBuffers: *const WSABUF,
            dwBufferCount: u32,
            lpNumberOfBytesRecvd: *mut u32,
            lpFlags: *mut u32,
            lpOverlapped: *mut OVERLAPPED,
            lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
        ) -> i32;
        pub fn WSASend(
            s: SOCKET,
            lpBuffers: *const WSABUF,
            dwBufferCount: u32,
            lpNumberOfBytesSent: *mut u32,
            dwFlags: u32,
            lpOverlapped: *mut OVERLAPPED,
            lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
        ) -> i32;
        pub fn ioctlsocket(s: SOCKET, cmd: i32, argp: *mut u32) -> i32;
        pub fn WSAGetLastError() -> WSA_ERROR;
        pub fn shutdown(s: SOCKET, how: i32) -> i32;
        pub fn bind(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
        pub fn listen(s: SOCKET, backlog: i32) -> i32;
        pub fn connect(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: u16,
    pub sun_path: [u8; 108],
}

pub(crate) fn init() {
    static INIT: parking_lot::Once = parking_lot::Once::new();
    INIT.call_once(|| {
        // Let standard library call `WSAStartup` for us, we can't do it
        // ourselves because otherwise using any type in `std::net` would panic
        // when it tries to call `WSAStartup` a second time.
        drop(std::net::UdpSocket::bind("127.0.0.1:0"));
    });
}

#[inline]
fn last_socket_error() -> io::Error {
    // SAFETY: syscall
    io::Error::from_raw_os_error(unsafe { bindings::WSAGetLastError() })
}

pub(crate) struct UnixSocketAddr {
    addr: sockaddr_un,
    len: i32,
}

impl UnixSocketAddr {
    pub(crate) fn from_path(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let path = path.as_ref();
        let path_bytes = path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "path is not utf-8"))?
            .as_bytes();

        let mut sock_addr = sockaddr_un {
            sun_family: bindings::PF_UNIX as _,
            sun_path: [0u8; 108],
        };

        if path_bytes.len() >= sock_addr.sun_path.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "specified path is too long",
            ));
        }

        sock_addr.sun_path[..path_bytes.len()].copy_from_slice(path_bytes);

        Self::from_parts(
            sock_addr,
            // Found some example Windows code that seemed to give no shits
            // about the "actual" size of the address, so if Microsoft doesn't
            // care why should we? https://devblogs.microsoft.com/commandline/windowswsl-interop-with-af_unix/
            std::mem::size_of_val(&sock_addr) as i32,
        )
    }

    #[inline]
    fn from_parts(addr: sockaddr_un, len: i32) -> io::Result<Self> {
        if addr.sun_family != bindings::PF_UNIX as _ {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "socket address is not a unix domain socket",
            ))
        } else {
            Ok(Self { addr, len })
        }
    }
}

struct Socket(bindings::SOCKET);

impl Socket {
    pub fn new() -> io::Result<Socket> {
        // SAFETY: syscall
        let socket = unsafe { bindings::socket(bindings::PF_UNIX, bindings::SOCK_STREAM, 0) };

        if socket == bindings::INVALID_SOCKET {
            Err(last_socket_error())
        } else {
            let socket = Self(socket);
            socket.set_no_inherit()?;
            Ok(socket)
        }
    }

    fn accept(&self, storage: *mut bindings::SOCKADDR, len: &mut i32) -> io::Result<Self> {
        // SAFETY: syscall
        let socket = unsafe { bindings::accept(self.0, storage, len) };

        if socket == bindings::INVALID_SOCKET {
            Err(last_socket_error())
        } else {
            let socket = Self(socket);
            socket.set_no_inherit()?;
            Ok(socket)
        }
    }

    #[inline]
    fn set_no_inherit(&self) -> io::Result<()> {
        // SAFETY: syscall
        if unsafe { bindings::SetHandleInformation(self.0 as _, bindings::HANDLE_FLAG_INHERIT, 0) }
            == 0
        {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        let mut nonblocking = nonblocking as u32;
        // SAFETY: syscall
        let r = unsafe { bindings::ioctlsocket(self.0, bindings::FIONBIO, &mut nonblocking) };
        if r == 0 {
            Ok(())
        } else {
            Err(last_socket_error())
        }
    }

    fn recv_with_flags(&self, buf: &mut [u8], flags: i32) -> io::Result<usize> {
        // On unix when a socket is shut down all further reads return 0, so we
        // do the same on windows to map a shut down socket to returning EOF.
        let length = std::cmp::min(buf.len(), i32::MAX as usize) as i32;
        // SAFETY: syscall
        let result = unsafe {
            bindings::recv(
                self.as_raw_socket() as _,
                buf.as_mut_ptr().cast(),
                length,
                flags,
            )
        };

        match result {
            bindings::SOCKET_ERROR => {
                let error = unsafe { bindings::WSAGetLastError() };

                if error == bindings::WSAESHUTDOWN {
                    Ok(0)
                } else {
                    Err(io::Error::from_raw_os_error(error))
                }
            }
            _ => Ok(result as usize),
        }
    }

    fn recv_vectored(&self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        // On unix when a socket is shut down all further reads return 0, so we
        // do the same on windows to map a shut down socket to returning EOF.
        let length = std::cmp::min(bufs.len(), u32::MAX as usize) as u32;
        let mut nread = 0;
        let mut flags = 0;
        // SAFETY: syscall
        let result = unsafe {
            bindings::WSARecv(
                self.as_raw_socket() as _,
                bufs.as_mut_ptr().cast(),
                length,
                &mut nread,
                &mut flags,
                std::ptr::null_mut(),
                None,
            )
        };

        if result == 0 {
            Ok(nread as usize)
        } else {
            // SAFETY: syscall
            let error = unsafe { bindings::WSAGetLastError() };

            if error == bindings::WSAESHUTDOWN {
                Ok(0)
            } else {
                Err(io::Error::from_raw_os_error(error))
            }
        }
    }

    fn send_vectored(&self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        let length = std::cmp::min(bufs.len(), u32::MAX as usize) as u32;
        let mut nwritten = 0;
        // SAFETY: syscall
        let result = unsafe {
            bindings::WSASend(
                self.as_raw_socket() as _,
                bufs.as_ptr().cast::<bindings::WSABUF>() as *mut _,
                length,
                &mut nwritten,
                0,
                std::ptr::null_mut(),
                None,
            )
        };

        if result == 0 {
            Ok(nwritten as usize)
        } else {
            Err(last_socket_error())
        }
    }
}

impl AsRawSocket for Socket {
    fn as_raw_socket(&self) -> RawSocket {
        self.0 as RawSocket
    }
}

impl FromRawSocket for Socket {
    unsafe fn from_raw_socket(sock: RawSocket) -> Self {
        Self(sock as bindings::SOCKET)
    }
}

impl IntoRawSocket for Socket {
    fn into_raw_socket(self) -> RawSocket {
        let ret = self.0 as RawSocket;
        std::mem::forget(self);
        ret
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        // SAFETY: syscalls
        let _ = unsafe {
            // https://docs.microsoft.com/en-us/windows/win32/winsock/graceful-shutdown-linger-options-and-socket-closure-2
            if bindings::shutdown(self.0, bindings::SD_SEND as i32) == 0 {
                // Loop until we've received all data
                let mut chunk = [0u8; 1024];
                while let Ok(sz) = self.recv_with_flags(&mut chunk, 0) {
                    if sz == 0 {
                        break;
                    }
                }
            }

            bindings::closesocket(self.0)
        };
    }
}

/// A Unix domain socket server
pub(crate) struct UnixListener(Socket);

impl UnixListener {
    pub(crate) fn bind(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        init();

        let inner = Socket::new()?;
        let addr = UnixSocketAddr::from_path(path.as_ref())?;

        // SAFETY: syscall
        if unsafe {
            bindings::bind(
                inner.as_raw_socket() as _,
                (&addr.addr as *const sockaddr_un).cast(),
                addr.len,
            )
        } != 0
        {
            return Err(io::Error::last_os_error());
        }

        // SAFETY: syscall
        if unsafe {
            bindings::listen(inner.as_raw_socket() as _, 128 /* backlog */)
        } != 0
        {
            Err(last_socket_error())
        } else {
            Ok(Self(inner))
        }
    }

    pub(crate) fn accept_unix_addr(&self) -> io::Result<(UnixStream, UnixSocketAddr)> {
        let mut sock_addr = std::mem::MaybeUninit::<sockaddr_un>::uninit();
        let mut len = std::mem::size_of::<sockaddr_un>() as i32;

        let sock = self.0.accept(sock_addr.as_mut_ptr().cast(), &mut len)?;
        // SAFETY: should have been initialized if accept succeeded
        let addr = UnixSocketAddr::from_parts(unsafe { sock_addr.assume_init() }, len)?;

        Ok((UnixStream(sock), addr))
    }

    #[inline]
    pub(crate) fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.0.set_nonblocking(nonblocking)
    }
}

impl AsRawSocket for UnixListener {
    fn as_raw_socket(&self) -> RawSocket {
        self.0.as_raw_socket()
    }
}

impl AsSocket for UnixListener {
    fn as_socket(&self) -> BorrowedSocket<'_> {
        unsafe { BorrowedSocket::borrow_raw(self.as_raw_socket()) }
    }
}

impl FromRawSocket for UnixListener {
    unsafe fn from_raw_socket(sock: RawSocket) -> Self {
        Self(unsafe { Socket::from_raw_socket(sock) })
    }
}

impl IntoRawSocket for UnixListener {
    fn into_raw_socket(self) -> RawSocket {
        let ret = self.0.as_raw_socket();
        std::mem::forget(self);
        ret
    }
}

/// A Unix doman socket stream
pub(crate) struct UnixStream(Socket);

impl UnixStream {
    pub(crate) fn connect(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        init();

        let inner = Socket::new()?;
        let addr = UnixSocketAddr::from_path(path)?;

        // SAFETY: syscall
        if unsafe {
            bindings::connect(
                inner.as_raw_socket() as _,
                (&addr.addr as *const sockaddr_un).cast(),
                addr.len,
            )
        } != 0
        {
            Err(last_socket_error())
        } else {
            Ok(Self(inner))
        }
    }

    #[inline]
    pub(crate) fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.recv_with_flags(buf, bindings::MSG_PEEK)
    }

    #[inline]
    pub(crate) fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv_vectored(&mut [io::IoSliceMut::new(buf)])
    }

    #[inline]
    pub(crate) fn recv_vectored(&self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.recv_vectored(bufs)
    }

    #[inline]
    pub(crate) fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.send_vectored(&[io::IoSlice::new(buf)])
    }

    #[inline]
    pub(crate) fn send_vectored(&self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.0.send_vectored(bufs)
    }
}

impl AsRawSocket for UnixStream {
    fn as_raw_socket(&self) -> RawSocket {
        self.0.as_raw_socket()
    }
}

impl AsSocket for UnixStream {
    fn as_socket(&self) -> BorrowedSocket<'_> {
        unsafe { BorrowedSocket::borrow_raw(self.as_raw_socket()) }
    }
}

impl FromRawSocket for UnixStream {
    unsafe fn from_raw_socket(sock: RawSocket) -> Self {
        Self(unsafe { Socket::from_raw_socket(sock) })
    }
}

impl IntoRawSocket for UnixStream {
    fn into_raw_socket(self) -> RawSocket {
        let ret = self.0.as_raw_socket();
        std::mem::forget(self);
        ret
    }
}
