//! `HandleOrSocket` variants of io-lifetimes'
//! `BorrowedHandle`/`BorrowedSocket` and `OwnedHandle`/`OwnedSocket`.

use super::{
    AsRawHandleOrSocket, FromRawHandleOrSocket, IntoRawHandleOrSocket, RawEnum, RawHandleOrSocket,
};
use io_lifetimes::{BorrowedHandle, BorrowedSocket, OwnedHandle, OwnedSocket};
use std::fmt;
use std::marker::PhantomData;
use std::mem::forget;
use std::os::windows::io::{
    AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket, RawSocket,
};
use windows_sys::Win32::Networking::WinSock::INVALID_SOCKET;

/// `HandleOrSocket` variant of io-lifetimes'
/// `BorrowedHandle`/`BorrowedSocket`.
#[derive(Copy, Clone)]
pub struct BorrowedHandleOrSocket<'a> {
    raw: RawHandleOrSocket,
    _phantom: PhantomData<&'a OwnedHandleOrSocket>,
}

/// `HandleOrSocket` variant of io-lifetimes'
/// `OwnedHandle`/`OwnedSocket`.
#[allow(missing_copy_implementations)]
pub struct OwnedHandleOrSocket {
    raw: RawHandleOrSocket,
}

impl<'a> BorrowedHandleOrSocket<'a> {
    /// Return a `BorrowedHandleOrSocket` holding the given raw handle or
    /// socket.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `raw` must remain open for the duration of
    /// the returned `BorrowedHandleOrSocket`, and it must not be a null handle
    /// or an invalid socket.
    #[inline]
    pub unsafe fn borrow_raw(raw: RawHandleOrSocket) -> Self {
        match raw.0 {
            RawEnum::Handle(raw_handle) => assert!(!raw_handle.is_null()),
            RawEnum::Socket(raw_socket) => assert_ne!(raw_socket, INVALID_SOCKET as RawSocket),
            RawEnum::Stdio(_) => (),
        }
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    /// Construct a `BorrowedHandleOrSocket` from a `BorrowedHandle`.
    #[inline]
    pub fn from_handle(handle: BorrowedHandle<'a>) -> Self {
        Self {
            raw: RawHandleOrSocket(RawEnum::Handle(handle.as_raw_handle())),
            _phantom: PhantomData,
        }
    }

    /// Construct a `BorrowedHandleOrSocket` from a `BorrowedSocket`.
    #[inline]
    pub fn from_socket(socket: BorrowedSocket<'a>) -> Self {
        Self {
            raw: RawHandleOrSocket(RawEnum::Socket(socket.as_raw_socket())),
            _phantom: PhantomData,
        }
    }

    /// Like [`AsHandle::as_handle`], but returns an `Option` so that
    /// it can return `None` if `self` doesn't contain a `BorrowedHandle`.
    ///
    /// [`AsHandle::as_handle`]: std::os::windows::io::AsHandle::as_handle
    #[inline]
    #[must_use]
    pub fn as_handle(&self) -> Option<BorrowedHandle<'a>> {
        unsafe {
            match self.raw.0 {
                RawEnum::Handle(handle) => Some(BorrowedHandle::borrow_raw(handle)),
                RawEnum::Socket(_) => None,
                RawEnum::Stdio(ref stdio) => {
                    Some(BorrowedHandle::borrow_raw(stdio.as_raw_handle()))
                }
            }
        }
    }

    /// Like [`AsSocket::as_socket`], but returns an `Option` so that
    /// it can return `None` if `self` doesn't contain a `BorrowedSocket`.
    ///
    /// [`AsSocket::as_socket`]: std::os::windows::io::AsSocket::as_socket
    #[inline]
    #[must_use]
    pub fn as_socket(&self) -> Option<BorrowedSocket<'a>> {
        unsafe {
            match self.raw.0 {
                RawEnum::Handle(_) => None,
                RawEnum::Socket(socket) => Some(BorrowedSocket::borrow_raw(socket)),
                RawEnum::Stdio(_) => None,
            }
        }
    }
}

impl OwnedHandleOrSocket {
    /// Construct a new `OwnedHandleOrSocket` from an `OwnedHandle`.
    #[inline]
    pub fn from_handle(handle: OwnedHandle) -> Self {
        Self {
            raw: RawHandleOrSocket(RawEnum::Handle(handle.into_raw_handle())),
        }
    }

    /// Construct a new `OwnedHandleOrSocket` from an `OwnedSocket`.
    #[inline]
    pub fn from_socket(socket: OwnedSocket) -> Self {
        Self {
            raw: RawHandleOrSocket(RawEnum::Socket(socket.into_raw_socket())),
        }
    }

    /// Like [`AsHandle::as_handle`], but returns an `Option` so that
    /// it can return `None` if `self` doesn't contain a `BorrowedHandle`.
    ///
    /// [`AsHandle::as_handle`]: std::os::windows::io::AsHandle::as_handle
    #[inline]
    #[must_use]
    pub fn as_handle(&self) -> Option<BorrowedHandle> {
        unsafe {
            match self.raw.0 {
                RawEnum::Handle(handle) => Some(BorrowedHandle::borrow_raw(handle)),
                RawEnum::Socket(_) => None,
                RawEnum::Stdio(ref stdio) => {
                    Some(BorrowedHandle::borrow_raw(stdio.as_raw_handle()))
                }
            }
        }
    }

    /// Like [`AsSocket::as_socket`], but returns an `Option` so that
    /// it can return `None` if `self` doesn't contain a `BorrowedSocket`.
    ///
    /// [`AsSocket::as_socket`]: std::os::windows::io::AsSocket::as_socket
    #[inline]
    #[must_use]
    pub fn as_socket(&self) -> Option<BorrowedSocket> {
        unsafe {
            match self.raw.0 {
                RawEnum::Handle(_) => None,
                RawEnum::Socket(socket) => Some(BorrowedSocket::borrow_raw(socket)),
                RawEnum::Stdio(_) => None,
            }
        }
    }
}

impl AsRawHandleOrSocket for OwnedHandleOrSocket {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.raw
    }
}

impl AsRawHandleOrSocket for BorrowedHandleOrSocket<'_> {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.raw
    }
}

impl IntoRawHandleOrSocket for OwnedHandleOrSocket {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        let raw = self.raw;
        forget(self);
        raw
    }
}

impl FromRawHandleOrSocket for OwnedHandleOrSocket {
    #[inline]
    unsafe fn from_raw_handle_or_socket(raw: RawHandleOrSocket) -> Self {
        match raw.0 {
            RawEnum::Handle(raw_handle) => assert!(!raw_handle.is_null()),
            RawEnum::Socket(raw_socket) => assert_ne!(raw_socket, INVALID_SOCKET as RawSocket),
            RawEnum::Stdio(_) => (),
        }
        Self { raw }
    }
}

impl Drop for OwnedHandleOrSocket {
    fn drop(&mut self) {
        match self.raw.0 {
            RawEnum::Handle(raw_handle) => unsafe {
                let _ = OwnedHandle::from_raw_handle(raw_handle);
            },
            RawEnum::Socket(raw_socket) => unsafe {
                let _ = OwnedSocket::from_raw_socket(raw_socket);
            },
            RawEnum::Stdio(_) => (),
        }
    }
}

impl fmt::Debug for BorrowedHandleOrSocket<'_> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw handle or socket; don't try to print the path or
        // any information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        f.debug_struct("BorrowedHandleOrSocket")
            .field("raw", &self.raw)
            .finish()
    }
}

impl fmt::Debug for OwnedHandleOrSocket {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw handle or socket; don't try to print the path or
        // any information about it, because this information is otherwise
        // unavailable to safe portable Rust code.
        f.debug_struct("OwnedHandleOrSocket")
            .field("raw", &self.raw)
            .finish()
    }
}
