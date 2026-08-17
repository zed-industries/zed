//! Extensions and types for Unix domain socket networking primitives.
//!
//! This module contains a number of extension traits for Windows-specific
//! functionality.

use std::cmp;
use std::fmt;
use std::io;
use std::mem;
use std::os::windows::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

use winapi::ctypes::c_int;
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, LPDWORD, LPINT, TRUE};
use winapi::shared::ntdef::PVOID;
use winapi::shared::ws2def::{
    LPSOCKADDR, SIO_GET_EXTENSION_FUNCTION_POINTER, SOCKADDR, SOCKADDR_STORAGE, SOL_SOCKET, WSABUF,
};
use winapi::um::minwinbase::{LPOVERLAPPED, OVERLAPPED};
use winapi::um::winsock2::{
    bind, setsockopt, u_long, WSAGetLastError, WSAGetOverlappedResult, WSAIoctl, WSARecv, WSASend,
    SOCKET, SOCKET_ERROR, WSA_IO_PENDING,
};

use super::net::{UnixListener, UnixStream};
use super::{c, from_sockaddr_un, sun_path_offset, SocketAddr};

/// A buffer in which an accepted socket's address will be stored
///
/// This type is used with the `accept_overlapped` method on the
/// `UnixListenerExt` trait to provide space for the overlapped I/O operation to
/// fill in the socket addresses upon completion.
#[repr(C)]
pub struct AcceptAddrsBuf {
    // For AcceptEx we've got the restriction that the addresses passed in that
    // buffer need to be at least 16 bytes more than the maximum address length
    // for the protocol in question, so add some extra here and there
    local: SOCKADDR_STORAGE,
    _pad1: [u8; 16],
    remote: SOCKADDR_STORAGE,
    _pad2: [u8; 16],
}

impl fmt::Debug for AcceptAddrsBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let local = unsafe { &*(&self.local as *const _ as *const c::sockaddr_un) };
        let remote = unsafe { &*(&self.remote as *const _ as *const c::sockaddr_un) };
        f.debug_struct("AcceptAddrsBuf")
            .field("local", local)
            .field("remote", remote)
            .finish()
    }
}

/// The parsed return value of `AcceptAddrsBuf`
pub struct AcceptAddrs<'a> {
    local: LPSOCKADDR,
    local_len: c_int,
    remote: LPSOCKADDR,
    remote_len: c_int,
    _data: &'a AcceptAddrsBuf,
}

impl<'a> fmt::Debug for AcceptAddrs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self._data, f)
    }
}

struct WsaExtension {
    guid: GUID,
    val: AtomicUsize,
}

/// Additional methods for the `UnixStream` type
pub trait UnixStreamExt {
    /// Execute an overlapped read I/O operation on this Unix domain socket
    /// stream.
    ///
    /// This function will issue an overlapped I/O read (via `WSARecv`) on this
    /// socket. The provided buffer will be filled in when the operation
    /// completes and the given `OVERLAPPED` instance is used to track the
    /// overlapped operation.
    ///
    /// If the operation succeeds, `Ok(Some(n))` is returned indicating how
    /// many bytes were read. If the operation returns an error indicating that
    /// the I/O is currently pending, `Ok(None)` is returned. Otherwise, the
    /// error associated with the operation is returned and no overlapped
    /// operation is enqueued.
    ///
    /// The number of bytes read will be returned as part of the completion
    /// notification when the I/O finishes.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the kernel requires that the `buf` and
    /// `overlapped` pointers are valid until the end of the I/O operation. The
    /// kernel also requires that `overlapped` is unique for this I/O operation
    /// and is not in use for any other I/O.
    ///
    /// To safely use this function callers must ensure that these two input
    /// pointers are valid until the I/O operation is completed, typically via
    /// completion ports and waiting to receive the completion notification on
    /// the port.
    unsafe fn read_overlapped(
        &self,
        buf: &mut [u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<Option<usize>>;

    /// Execute an overlapped write I/O operation on this Unix domain socket
    /// stream.
    ///
    /// This function will issue an overlapped I/O write (via `WSASend`) on this
    /// socket. The provided buffer will be written when the operation completes
    /// and the given `OVERLAPPED` instance is used to track the overlapped
    /// operation.
    ///
    /// If the operation succeeds, `Ok(Some(n))` is returned where `n` is the
    /// number of bytes that were written. If the operation returns an error
    /// indicating that the I/O is currently pending, `Ok(None)` is returned.
    /// Otherwise, the error associated with the operation is returned and no
    /// overlapped operation is enqueued.
    ///
    /// The number of bytes written will be returned as part of the completion
    /// notification when the I/O finishes.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the kernel requires that the `buf` and
    /// `overlapped` pointers are valid until the end of the I/O operation. The
    /// kernel also requires that `overlapped` is unique for this I/O operation
    /// and is not in use for any other I/O.
    ///
    /// To safely use this function callers must ensure that these two input
    /// pointers are valid until the I/O operation is completed, typically via
    /// completion ports and waiting to receive the completion notification on
    /// the port.
    unsafe fn write_overlapped(
        &self,
        buf: &[u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<Option<usize>>;

    /// Attempt to consume the internal socket in this builder by executing an
    /// overlapped connect operation.
    ///
    /// This function will issue a connect operation to the address specified on
    /// the underlying socket, flagging it as an overlapped operation which will
    /// complete asynchronously. If successful this function will return the
    /// corresponding Unix domain socket stream.
    ///
    /// The `buf` argument provided is an initial buffer of data that should be
    /// sent after the connection is initiated. It's acceptable to
    /// pass an empty slice here.
    ///
    /// This function will also return whether the connect immediately
    /// succeeded or not. If `Ok(None)` is returned then the I/O operation is
    /// still pending and will complete later. If `Ok(Some(bytes))` is returned
    /// then that many bytes were transferred.
    ///
    /// Note that to succeed this requires that the underlying socket has
    /// previously been bound via a call to `bind` to a local path.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the kernel requires that the
    /// `overlapped` and `buf` pointers to be  valid until the end of the I/O
    /// operation. The kernel also requires that `overlapped` is unique for
    /// this I/O operation and is not in use for any other I/O.
    ///
    /// To safely use this function callers must ensure that this pointer is
    /// valid until the I/O operation is completed, typically via completion
    /// ports and waiting to receive the completion notification on the port.
    unsafe fn connect_overlapped(
        &self,
        addr: &SocketAddr,
        buf: &[u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<Option<usize>>;

    /// Once a `connect_overlapped` has finished, this function needs to be
    /// called to finish the connect operation.
    ///
    /// Currently this just calls `setsockopt` with `SO_UPDATE_CONNECT_CONTEXT`
    /// to ensure that further functions like `getpeername` and `getsockname`
    /// work correctly.
    fn connect_complete(&self) -> io::Result<()>;

    /// Calls the `GetOverlappedResult` function to get the result of an
    /// overlapped operation for this handle.
    ///
    /// This function takes the `OVERLAPPED` argument which must have been used
    /// to initiate an overlapped I/O operation, and returns either the
    /// successful number of bytes transferred during the operation or an error
    /// if one occurred, along with the results of the `lpFlags` parameter of
    /// the relevant operation, if applicable.
    ///
    /// # Safety
    ///
    /// This function is unsafe as `overlapped` must have previously been used
    /// to execute an operation for this handle, and it must also be a valid
    /// pointer to an `OVERLAPPED` instance.
    ///
    /// # Panics
    ///
    /// This function will panic
    unsafe fn result(&self, overlapped: *mut OVERLAPPED) -> io::Result<(usize, u32)>;
}

/// Additional methods for the `UnixListener` type
pub trait UnixListenerExt {
    /// Perform an accept operation on this listener, accepting a connection in
    /// an overlapped fashion.
    ///
    /// This function will issue an I/O request to accept an incoming connection
    /// with the specified overlapped instance. The `socket` provided must be
    /// configured but not bound or connected. If successful this method will
    /// consume the socket to return a Unix stream.
    ///
    /// The `addrs` buffer provided will be filled in with the local and remote
    /// addresses of the connection upon completion.
    ///
    /// If the accept succeeds immediately, `Ok(true)` is returned. If the
    /// connect indicates that the I/O is currently pending, `Ok(false)` is
    /// returned. Otherwise, the error associated with the operation is returned
    /// and no overlapped operation is enqueued.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the kernel requires that the
    /// `addrs` and `overlapped` pointers are valid until the end of the I/O
    /// operation. The kernel also requires that `overlapped` is unique for this
    /// I/O operation and is not in use for any other I/O.
    ///
    /// To safely use this function callers must ensure that the pointers are
    /// valid until the I/O operation is completed, typically via completion
    /// ports and waiting to receive the completion notification on the port.
    unsafe fn accept_overlapped(
        &self,
        socket: &UnixStream,
        addrs: &mut AcceptAddrsBuf,
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<bool>;

    /// Once an `accept_overlapped` has finished, this function needs to be
    /// called to finish the accept operation.
    ///
    /// Currently this just calls `setsockopt` with `SO_UPDATE_ACCEPT_CONTEXT`
    /// to ensure that further functions like `getpeername` and `getsockname`
    /// work correctly.
    fn accept_complete(&self, socket: &UnixStream) -> io::Result<()>;

    /// Calls the `GetOverlappedResult` function to get the result of an
    /// overlapped operation for this handle.
    ///
    /// This function takes the `OVERLAPPED` argument which must have been used
    /// to initiate an overlapped I/O operation, and returns either the
    /// successful number of bytes transferred during the operation or an error
    /// if one occurred, along with the results of the `lpFlags` parameter of
    /// the relevant operation, if applicable.
    ///
    /// # Safety
    ///
    /// This function is unsafe as `overlapped` must have previously been used
    /// to execute an operation for this handle, and it must also be a valid
    /// pointer to an `OVERLAPPED` instance.
    ///
    /// # Panics
    ///
    /// This function will panic
    unsafe fn result(&self, overlapped: *mut OVERLAPPED) -> io::Result<(usize, u32)>;
}

trait NetInt {
    fn from_be(i: Self) -> Self;
    fn to_be(&self) -> Self;
}
macro_rules! doit {
    ($($t:ident)*) => ($(impl NetInt for $t {
        fn from_be(i: Self) -> Self { <$t>::from_be(i) }
        fn to_be(&self) -> Self { <$t>::to_be(*self) }
    })*)
}
doit! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }

fn last_err() -> io::Result<Option<usize>> {
    let err = unsafe { WSAGetLastError() };
    if err == WSA_IO_PENDING {
        Ok(None)
    } else {
        Err(io::Error::from_raw_os_error(err))
    }
}

fn cvt(i: c_int, size: DWORD) -> io::Result<Option<usize>> {
    if i == SOCKET_ERROR {
        last_err()
    } else {
        Ok(Some(size as usize))
    }
}

fn socket_addr_to_ptrs(addr: &SocketAddr) -> (*const SOCKADDR, c_int) {
    (
        &addr.addr as *const _ as *const _,
        mem::size_of::<c::sockaddr_un>() as c_int,
    )
}

unsafe fn ptrs_to_socket_addr(ptr: *const SOCKADDR, len: c_int) -> Option<SocketAddr> {
    if (len as usize) < mem::size_of::<c_int>() {
        return None;
    }
    match (*ptr).sa_family {
        c::AF_UNIX if len as usize >= mem::size_of::<c::sockaddr_un>() => {
            let b = &*(ptr as *const c::sockaddr_un);
            match b.sun_path.iter().position(|c| *c == 0) {
                Some(0) => from_sockaddr_un(*b, len).ok(),
                Some(i) => {
                    let mut l = sun_path_offset(b) + i;
                    match b.sun_path.first() {
                        Some(&0) | None => {}
                        Some(_) => l += 1,
                    }
                    from_sockaddr_un(*b, l as c_int).ok()
                }
                _ => None, // Invalid socket path, no terminating null byte
            }
        }
        _ => None, // Invalid socket family, should be AF_UNIX
    }
}

unsafe fn slice2buf(slice: &[u8]) -> WSABUF {
    WSABUF {
        len: cmp::min(slice.len(), <u_long>::max_value() as usize) as u_long,
        buf: slice.as_ptr() as *mut _,
    }
}

unsafe fn result(socket: SOCKET, overlapped: *mut OVERLAPPED) -> io::Result<(usize, u32)> {
    let mut transferred = 0;
    let mut flags = 0;
    let r = WSAGetOverlappedResult(socket, overlapped, &mut transferred, FALSE, &mut flags);
    if r == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok((transferred as usize, flags))
    }
}

impl UnixStreamExt for UnixStream {
    unsafe fn read_overlapped(
        &self,
        buf: &mut [u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<Option<usize>> {
        let mut buf = slice2buf(buf);
        let mut flags = 0;
        let mut bytes_read: DWORD = 0;
        let r = WSARecv(
            self.as_raw_socket() as SOCKET,
            &mut buf,
            1,
            &mut bytes_read,
            &mut flags,
            overlapped,
            None,
        );
        cvt(r, bytes_read)
    }

    unsafe fn write_overlapped(
        &self,
        buf: &[u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<Option<usize>> {
        let mut buf = slice2buf(buf);
        let mut bytes_written = 0;

        // Note here that we capture the number of bytes written. The
        // documentation on MSDN, however, states:
        //
        // > Use NULL for this parameter if the lpOverlapped parameter is not
        // > NULL to avoid potentially erroneous results. This parameter can be
        // > NULL only if the lpOverlapped parameter is not NULL.
        //
        // If we're not passing a null overlapped pointer here, then why are we
        // then capturing the number of bytes! Well so it turns out that this is
        // clearly faster to learn the bytes here rather than later calling
        // `WSAGetOverlappedResult`, and in practice almost all implementations
        // use this anyway [1].
        //
        // As a result we use this to and report back the result.
        //
        // [1]: https://github.com/carllerche/mio/pull/520#issuecomment-273983823
        let r = WSASend(
            self.as_raw_socket() as SOCKET,
            &mut buf,
            1,
            &mut bytes_written,
            0,
            overlapped,
            None,
        );
        cvt(r, bytes_written)
    }

    unsafe fn connect_overlapped(
        &self,
        addr: &SocketAddr,
        buf: &[u8],
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<Option<usize>> {
        connect_overlapped(self.as_raw_socket() as SOCKET, addr, buf, overlapped)
    }

    fn connect_complete(&self) -> io::Result<()> {
        const SO_UPDATE_CONNECT_CONTEXT: c_int = 0x7010;
        let result = unsafe {
            setsockopt(
                self.as_raw_socket() as SOCKET,
                SOL_SOCKET,
                SO_UPDATE_CONNECT_CONTEXT,
                std::ptr::null(),
                0,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    unsafe fn result(&self, overlapped: *mut OVERLAPPED) -> io::Result<(usize, u32)> {
        result(self.as_raw_socket() as SOCKET, overlapped)
    }
}

unsafe fn connect_overlapped(
    socket: SOCKET,
    addr: &SocketAddr,
    buf: &[u8],
    overlapped: *mut OVERLAPPED,
) -> io::Result<Option<usize>> {
    let anonaddr = c::sockaddr_un {
        sun_family: c::AF_UNIX,
        sun_path: [0; 108],
    };
    let len = mem::size_of::<c::sockaddr_un>() as c_int;
    super::cvt(bind(
        socket as _,
        &anonaddr as *const _ as *const _,
        len as _,
    ))?;

    static CONNECTEX: WsaExtension = WsaExtension {
        guid: GUID {
            Data1: 0x25a207b9,
            Data2: 0xddf3,
            Data3: 0x4660,
            Data4: [0x8e, 0xe9, 0x76, 0xe5, 0x8c, 0x74, 0x06, 0x3e],
        },
        val: AtomicUsize::new(0),
    };
    type ConnectEx = unsafe extern "system" fn(
        SOCKET,
        *const SOCKADDR,
        c_int,
        PVOID,
        DWORD,
        LPDWORD,
        LPOVERLAPPED,
    ) -> BOOL;

    let ptr = CONNECTEX.get(socket)?;
    assert!(ptr != 0);
    let connect_ex = mem::transmute::<_, ConnectEx>(ptr);

    let (addr_buf, addr_len) = socket_addr_to_ptrs(addr);
    let mut bytes_sent: DWORD = 0;
    let r = connect_ex(
        socket,
        addr_buf,
        addr_len,
        buf.as_ptr() as *mut _,
        buf.len() as u32,
        &mut bytes_sent,
        overlapped,
    );
    if r == TRUE {
        Ok(Some(bytes_sent as usize))
    } else {
        last_err()
    }
}

impl UnixListenerExt for UnixListener {
    unsafe fn accept_overlapped(
        &self,
        socket: &UnixStream,
        addrs: &mut AcceptAddrsBuf,
        overlapped: *mut OVERLAPPED,
    ) -> io::Result<bool> {
        static ACCEPTEX: WsaExtension = WsaExtension {
            guid: GUID {
                Data1: 0xb5367df1,
                Data2: 0xcbac,
                Data3: 0x11cf,
                Data4: [0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92],
            },
            val: AtomicUsize::new(0),
        };
        type AcceptEx = unsafe extern "system" fn(
            SOCKET,
            SOCKET,
            PVOID,
            DWORD,
            DWORD,
            DWORD,
            LPDWORD,
            LPOVERLAPPED,
        ) -> BOOL;

        let ptr = ACCEPTEX.get(self.as_raw_socket() as SOCKET)?;
        assert!(ptr != 0);
        let accept_ex = mem::transmute::<_, AcceptEx>(ptr);

        let mut bytes = 0;
        let (a, b, c, d) = (*addrs).args();
        let r = accept_ex(
            self.as_raw_socket() as SOCKET,
            socket.as_raw_socket() as SOCKET,
            a,
            b,
            c,
            d,
            &mut bytes,
            overlapped,
        );
        let succeeded = if r == TRUE {
            true
        } else {
            last_err()?;
            false
        };
        Ok(succeeded)
    }

    fn accept_complete(&self, socket: &UnixStream) -> io::Result<()> {
        const SO_UPDATE_ACCEPT_CONTEXT: c_int = 0x700B;
        let me = self.as_raw_socket();
        let result = unsafe {
            setsockopt(
                socket.as_raw_socket() as SOCKET,
                SOL_SOCKET,
                SO_UPDATE_ACCEPT_CONTEXT,
                &me as *const _ as *const _,
                mem::size_of_val(&me) as c_int,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    unsafe fn result(&self, overlapped: *mut OVERLAPPED) -> io::Result<(usize, u32)> {
        result(self.as_raw_socket() as SOCKET, overlapped)
    }
}

static GETACCEPTEXSOCKADDRS: WsaExtension = WsaExtension {
    guid: GUID {
        Data1: 0xb5367df2,
        Data2: 0xcbac,
        Data3: 0x11cf,
        Data4: [0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92],
    },
    val: AtomicUsize::new(0),
};
type GetAcceptExSockaddrs = unsafe extern "system" fn(
    PVOID,
    DWORD,
    DWORD,
    DWORD,
    *mut LPSOCKADDR,
    LPINT,
    *mut LPSOCKADDR,
    LPINT,
);

impl Default for AcceptAddrsBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl AcceptAddrsBuf {
    /// Creates a new blank buffer ready to be passed to a call to
    /// `accept_overlapped`.
    pub fn new() -> AcceptAddrsBuf {
        unsafe { mem::zeroed() }
    }

    /// Parses the data contained in this address buffer, returning the parsed
    /// result if successful.
    ///
    /// This function can be called after a call to `accept_overlapped` has
    /// succeeded to parse out the data that was written in.
    pub fn parse(&self, socket: &UnixListener) -> io::Result<AcceptAddrs> {
        let mut ret = AcceptAddrs {
            local: std::ptr::null_mut(),
            local_len: 0,
            remote: std::ptr::null_mut(),
            remote_len: 0,
            _data: self,
        };
        let ptr = GETACCEPTEXSOCKADDRS.get(socket.as_raw_socket() as SOCKET)?;
        assert!(ptr != 0);
        unsafe {
            let get_sockaddrs = mem::transmute::<_, GetAcceptExSockaddrs>(ptr);
            let (a, b, c, d) = self.args();
            get_sockaddrs(
                a,
                b,
                c,
                d,
                &mut ret.local,
                &mut ret.local_len,
                &mut ret.remote,
                &mut ret.remote_len,
            );
            Ok(ret)
        }
    }

    fn args(&self) -> (PVOID, DWORD, DWORD, DWORD) {
        let remote_offset = memoffset::offset_of!(AcceptAddrsBuf, remote);
        (
            self as *const _ as *mut _,
            0,
            remote_offset as DWORD,
            (mem::size_of_val(self) - remote_offset) as DWORD,
        )
    }
}

impl<'a> AcceptAddrs<'a> {
    /// Returns the local socket address contained in this buffer.
    #[allow(dead_code)]
    pub fn local(&self) -> Option<SocketAddr> {
        unsafe { ptrs_to_socket_addr(self.local, self.local_len) }
    }

    /// Returns the remote socket address contained in this buffer.
    pub fn remote(&self) -> Option<SocketAddr> {
        unsafe { ptrs_to_socket_addr(self.remote, self.remote_len) }
    }
}

impl WsaExtension {
    fn get(&self, socket: SOCKET) -> io::Result<usize> {
        let prev = self.val.load(Ordering::SeqCst);
        if prev != 0 && !cfg!(debug_assertions) {
            return Ok(prev);
        }
        let mut ret = 0_usize;
        let mut bytes = 0;
        let r = unsafe {
            WSAIoctl(
                socket,
                SIO_GET_EXTENSION_FUNCTION_POINTER,
                &self.guid as *const _ as *mut _,
                mem::size_of_val(&self.guid) as DWORD,
                &mut ret as *mut _ as *mut _,
                mem::size_of_val(&ret) as DWORD,
                &mut bytes,
                std::ptr::null_mut(),
                None,
            )
        };
        cvt(r, 0).map(|_| {
            debug_assert_eq!(bytes as usize, mem::size_of_val(&ret));
            debug_assert!(prev == 0 || prev == ret);
            self.val.store(ret, Ordering::SeqCst);
            ret
        })
    }
}
