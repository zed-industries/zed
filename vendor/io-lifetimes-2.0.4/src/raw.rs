//! Portability abstractions over `Raw*`.
//!
//! On Unix, "everything is a file descriptor". On Windows, file/pipe/process
//! handles are distinct from socket descriptors. This file provides a minimal
//! layer of portability over this difference.

#[cfg(target_os = "hermit")]
use std::os::hermit::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{
    AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
    RawHandle, RawSocket,
};

/// A raw filelike object.
///
/// This is a portability abstraction over Unix-like [`RawFd`] and
/// Windows' `RawHandle`.
#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
pub type RawFilelike = RawFd;

/// A raw filelike object.
///
/// This is a portability abstraction over Unix-like `RawFd` and
/// Windows' [`RawHandle`].
#[cfg(windows)]
pub type RawFilelike = RawHandle;

/// A raw socketlike object.
///
/// This is a portability abstraction over Unix-like [`RawFd`] and
/// Windows' `RawSocket`.
#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
pub type RawSocketlike = RawFd;

/// A raw socketlike object.
///
/// This is a portability abstraction over Unix-like `RawFd` and
/// Windows' [`RawSocket`].
#[cfg(windows)]
pub type RawSocketlike = RawSocket;

/// A portable trait to obtain the raw value of an underlying filelike object.
///
/// This is a portability abstraction over Unix-like [`AsRawFd`] and Windows'
/// `AsRawHandle`.
#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
pub trait AsRawFilelike: AsRawFd {
    /// Returns the raw value.
    fn as_raw_filelike(&self) -> RawFilelike;
}

#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
impl<T: AsRawFd> AsRawFilelike for T {
    #[inline]
    fn as_raw_filelike(&self) -> RawFilelike {
        self.as_raw_fd()
    }
}

/// This is a portability abstraction over Unix-like `AsRawFd` and Windows'
/// [`AsRawHandle`].
#[cfg(windows)]
pub trait AsRawFilelike: AsRawHandle {
    /// Returns the raw value.
    fn as_raw_filelike(&self) -> RawFilelike;
}

#[cfg(windows)]
impl<T: AsRawHandle> AsRawFilelike for T {
    #[inline]
    fn as_raw_filelike(&self) -> RawFilelike {
        self.as_raw_handle()
    }
}

/// This is a portability abstraction over Unix-like [`AsRawFd`] and Windows'
/// `AsRawSocket`.
#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
pub trait AsRawSocketlike: AsRawFd {
    /// Returns the raw value.
    fn as_raw_socketlike(&self) -> RawSocketlike;
}

#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
impl<T: AsRawFd> AsRawSocketlike for T {
    #[inline]
    fn as_raw_socketlike(&self) -> RawSocketlike {
        self.as_raw_fd()
    }
}

/// This is a portability abstraction over Unix-like `AsRawFd` and Windows'
/// [`AsRawSocket`].
#[cfg(windows)]
pub trait AsRawSocketlike: AsRawSocket {
    /// Returns the raw value.
    fn as_raw_socketlike(&self) -> RawSocketlike;
}

#[cfg(windows)]
impl<T: AsRawSocket> AsRawSocketlike for T {
    #[inline]
    fn as_raw_socketlike(&self) -> RawSocketlike {
        self.as_raw_socket()
    }
}

/// This is a portability abstraction over Unix-like [`IntoRawFd`] and Windows'
/// `IntoRawHandle`.
#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
pub trait IntoRawFilelike: IntoRawFd {
    /// Returns the raw value.
    fn into_raw_filelike(self) -> RawFilelike;
}

#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
impl<T: IntoRawFd> IntoRawFilelike for T {
    #[inline]
    fn into_raw_filelike(self) -> RawFilelike {
        self.into_raw_fd()
    }
}

/// This is a portability abstraction over Unix-like `IntoRawFd` and Windows'
/// [`IntoRawHandle`].
#[cfg(windows)]
pub trait IntoRawFilelike: IntoRawHandle {
    /// Returns the raw value.
    fn into_raw_filelike(self) -> RawFilelike;
}

#[cfg(windows)]
impl<T: IntoRawHandle> IntoRawFilelike for T {
    #[inline]
    fn into_raw_filelike(self) -> RawFilelike {
        self.into_raw_handle()
    }
}

/// This is a portability abstraction over Unix-like [`IntoRawFd`] and Windows'
/// `IntoRawSocket`.
#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
pub trait IntoRawSocketlike: IntoRawFd {
    /// Returns the raw value.
    fn into_raw_socketlike(self) -> RawSocketlike;
}

#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
impl<T: IntoRawFd> IntoRawSocketlike for T {
    #[inline]
    fn into_raw_socketlike(self) -> RawSocketlike {
        self.into_raw_fd()
    }
}

/// This is a portability abstraction over Unix-like `IntoRawFd` and Windows'
/// [`IntoRawSocket`].
#[cfg(windows)]
pub trait IntoRawSocketlike: IntoRawSocket {
    /// Returns the raw value.
    fn into_raw_socketlike(self) -> RawSocketlike;
}

#[cfg(windows)]
impl<T: IntoRawSocket> IntoRawSocketlike for T {
    #[inline]
    fn into_raw_socketlike(self) -> RawSocketlike {
        self.into_raw_socket()
    }
}

/// This is a portability abstraction over Unix-like [`FromRawFd`] and Windows'
/// `FromRawHandle`.
#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
pub trait FromRawFilelike: FromRawFd {
    /// Constructs `Self` from the raw value.
    ///
    /// # Safety
    ///
    /// This is `unsafe` for the same reason as [`from_raw_fd`] and
    /// [`from_raw_handle`].
    ///
    /// [`from_raw_fd`]: https://doc.rust-lang.org/stable/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd
    /// [`from_raw_handle`]: https://doc.rust-lang.org/stable/std/os/windows/io/trait.FromRawHandle.html#tymethod.from_raw_handle
    unsafe fn from_raw_filelike(raw: RawFilelike) -> Self;
}

#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
impl<T: FromRawFd> FromRawFilelike for T {
    #[inline]
    unsafe fn from_raw_filelike(raw: RawFilelike) -> Self {
        Self::from_raw_fd(raw)
    }
}

/// This is a portability abstraction over Unix-like `FromRawFd` and Windows'
/// [`FromRawHandle`].
#[cfg(windows)]
pub trait FromRawFilelike: FromRawHandle {
    /// Constructs `Self` from the raw value.
    unsafe fn from_raw_filelike(raw: RawFilelike) -> Self;
}

#[cfg(windows)]
impl<T: FromRawHandle> FromRawFilelike for T {
    #[inline]
    unsafe fn from_raw_filelike(raw: RawFilelike) -> Self {
        Self::from_raw_handle(raw)
    }
}

/// This is a portability abstraction over Unix-like [`FromRawFd`] and Windows'
/// `FromRawSocket`.
#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
pub trait FromRawSocketlike: FromRawFd {
    /// Constructs `Self` from the raw value.
    ///
    /// # Safety
    ///
    /// This is `unsafe` for the same reason as [`from_raw_fd`] and
    /// [`from_raw_socket`].
    ///
    /// [`from_raw_fd`]: https://doc.rust-lang.org/stable/std/os/fd/trait.FromRawFd.html#tymethod.from_raw_fd
    /// [`from_raw_socket`]: https://doc.rust-lang.org/stable/std/os/windows/io/trait.FromRawSocket.html#tymethod.from_raw_socket
    unsafe fn from_raw_socketlike(raw: RawSocketlike) -> Self;
}

#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
impl<T: FromRawFd> FromRawSocketlike for T {
    #[inline]
    unsafe fn from_raw_socketlike(raw: RawSocketlike) -> Self {
        Self::from_raw_fd(raw)
    }
}

/// This is a portability abstraction over Unix-like `FromRawFd` and Windows'
/// [`FromRawSocket`].
#[cfg(windows)]
pub trait FromRawSocketlike: FromRawSocket {
    /// Constructs `Self` from the raw value.
    unsafe fn from_raw_socketlike(raw: RawSocketlike) -> Self;
}

#[cfg(windows)]
impl<T: FromRawSocket> FromRawSocketlike for T {
    #[inline]
    unsafe fn from_raw_socketlike(raw: RawSocketlike) -> Self {
        Self::from_raw_socket(raw)
    }
}
