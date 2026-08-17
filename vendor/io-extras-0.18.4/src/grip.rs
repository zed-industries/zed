//! "Grip" is an abstraction over "Fd" and "HandleOrSocket". "Handle"
//! would be the obvious term, but that has a more specific meaning on
//! Windows.

#[cfg(windows)]
use crate::os::windows::{
    AsHandleOrSocket, AsRawHandleOrSocket, AsRawReadWriteHandleOrSocket, AsReadWriteHandleOrSocket,
    BorrowedHandleOrSocket, FromRawHandleOrSocket, IntoRawHandleOrSocket, OwnedHandleOrSocket,
    RawHandleOrSocket,
};
#[cfg(not(windows))]
use {
    crate::os::rustix::{AsRawFd, AsRawReadWriteFd, AsReadWriteFd, FromRawFd, IntoRawFd, RawFd},
    io_lifetimes::{AsFd, BorrowedFd, OwnedFd},
};

/// Portability abstraction over `BorrowedFd` and `BorrowedHandleOrSocket`.
#[cfg(not(windows))]
pub type BorrowedGrip<'a> = BorrowedFd<'a>;
/// Portability abstraction over `OwnedFd` and `OwnedHandleOrSocket`.
#[cfg(not(windows))]
pub type OwnedGrip = OwnedFd;

/// Portability abstraction over `BorrowedFd` and `BorrowedHandleOrSocket`.
#[cfg(windows)]
pub type BorrowedGrip<'a> = BorrowedHandleOrSocket<'a>;
/// Portability abstraction over `OwnedFd` and `OwnedHandleOrSocket`.
#[cfg(windows)]
pub type OwnedGrip = OwnedHandleOrSocket;

/// Portability abstraction over `AsFd` and `AsHandleOrSocket`.
#[cfg(not(windows))]
pub trait AsGrip: AsFd {
    /// Extracts the grip.
    fn as_grip(&self) -> BorrowedGrip<'_>;
}

/// Portability abstraction over `AsFd` and `AsHandleOrSocket`.
#[cfg(windows)]
pub trait AsGrip: AsHandleOrSocket {
    /// Extracts the grip.
    fn as_grip(&self) -> BorrowedGrip<'_>;
}

/// Portability abstraction over `AsReadWriteFd` and
/// `AsReadWriteHandleOrSocket`.
#[cfg(not(windows))]
pub trait AsReadWriteGrip: AsReadWriteFd {
    /// Extracts the grip for reading.
    ///
    /// Like [`AsGrip::as_grip`], but returns the
    /// reading grip.
    fn as_read_grip(&self) -> BorrowedGrip<'_>;

    /// Extracts the grip for writing.
    ///
    /// Like [`AsGrip::as_grip`], but returns the
    /// writing grip.
    fn as_write_grip(&self) -> BorrowedGrip<'_>;
}

/// Portability abstraction over `AsReadWriteFd` and
/// `AsReadWriteHandleOrSocket`.
#[cfg(windows)]
pub trait AsReadWriteGrip: AsReadWriteHandleOrSocket {
    /// Extracts the grip for reading.
    ///
    /// Like [`AsGrip::as_grip`], but returns the
    /// reading grip.
    fn as_read_grip(&self) -> BorrowedGrip<'_>;

    /// Extracts the grip for writing.
    ///
    /// Like [`AsGrip::as_grip`], but returns the
    /// writing grip.
    fn as_write_grip(&self) -> BorrowedGrip<'_>;
}

/// Portability abstraction over `Into<OwnedFd>` and
/// `Into<OwnedHandleOrSocket>`.
#[cfg(not(windows))]
pub trait IntoGrip: Into<OwnedFd> {
    /// Consume `self` and convert into an `OwnedGrip`.
    fn into_grip(self) -> OwnedGrip;
}

/// Portability abstraction over `Into<OwnedFd>` and
/// `Into<OwnedHandleOrSocket>`.
#[cfg(windows)]
pub trait IntoGrip: Into<OwnedHandleOrSocket> {
    /// Consume `self` and convert into an `OwnedGrip`.
    fn into_grip(self) -> OwnedGrip;
}

/// Portability abstraction over `From<OwnedFd>` and
/// `From<OwnedHandleOrSocket>`.
#[cfg(not(windows))]
pub trait FromGrip: From<OwnedFd> {
    /// Consume an `OwnedGrip` and convert into a `Self`.
    fn from_grip(owned_grip: OwnedGrip) -> Self;
}

/// Portability abstraction over `From<OwnedFd>` and
/// `From<OwnedHandleOrSocket>`.
#[cfg(windows)]
pub trait FromGrip: From<OwnedHandleOrSocket> {
    /// Consume an `OwnedGrip` and convert into a `Self`.
    fn from_grip(owned_grip: OwnedGrip) -> Self;
}

#[cfg(not(windows))]
impl<T: AsFd> AsGrip for T {
    #[inline]
    fn as_grip(&self) -> BorrowedGrip<'_> {
        self.as_fd()
    }
}

#[cfg(windows)]
impl<T: AsHandleOrSocket> AsGrip for T {
    #[inline]
    fn as_grip(&self) -> BorrowedGrip<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<T: AsReadWriteFd> AsReadWriteGrip for T {
    #[inline]
    fn as_read_grip(&self) -> BorrowedGrip<'_> {
        self.as_read_fd()
    }

    #[inline]
    fn as_write_grip(&self) -> BorrowedGrip<'_> {
        self.as_write_fd()
    }
}

#[cfg(windows)]
impl<T: AsReadWriteHandleOrSocket> AsReadWriteGrip for T {
    #[inline]
    fn as_read_grip(&self) -> BorrowedGrip<'_> {
        self.as_read_handle_or_socket()
    }

    #[inline]
    fn as_write_grip(&self) -> BorrowedGrip<'_> {
        self.as_write_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<T: Into<OwnedFd>> IntoGrip for T {
    #[inline]
    fn into_grip(self) -> OwnedGrip {
        self.into()
    }
}

#[cfg(windows)]
impl<T: Into<OwnedHandleOrSocket>> IntoGrip for T {
    #[inline]
    fn into_grip(self) -> OwnedGrip {
        self.into()
    }
}

#[cfg(not(windows))]
impl<T: From<OwnedFd>> FromGrip for T {
    #[inline]
    fn from_grip(owned_grip: OwnedGrip) -> Self {
        Self::from(owned_grip)
    }
}

#[cfg(windows)]
impl<T: From<OwnedHandleOrSocket>> FromGrip for T {
    #[inline]
    fn from_grip(owned_grip: OwnedGrip) -> Self {
        Self::from(owned_grip)
    }
}

/// Portability abstraction over `RawFd` and `RawHandleOrSocket`.
#[cfg(not(windows))]
pub type RawGrip = RawFd;

/// Portability abstraction over `RawFd` and `RawHandleOrSocket`.
#[cfg(windows)]
pub type RawGrip = RawHandleOrSocket;

/// Portability abstraction over `AsFd` and `AsHandleOrSocket`.
#[cfg(not(windows))]
pub trait AsRawGrip: AsRawFd {
    /// Extracts the raw grip.
    fn as_raw_grip(&self) -> RawGrip;
}

/// Portability abstraction over `AsFd` and `AsHandleOrSocket`.
#[cfg(windows)]
pub trait AsRawGrip: AsRawHandleOrSocket {
    /// Extracts the raw grip.
    fn as_raw_grip(&self) -> RawGrip;
}

/// Portability abstraction over `AsReadWriteFd` and
/// `AsReadWriteHandleOrSocket`.
#[cfg(not(windows))]
pub trait AsRawReadWriteGrip: AsRawReadWriteFd {
    /// Extracts the grip for reading.
    ///
    /// Like [`AsRawGrip::as_raw_grip`], but returns the
    /// raw reading grip.
    fn as_raw_read_grip(&self) -> RawGrip;

    /// Extracts the grip for writing.
    ///
    /// Like [`AsRawGrip::as_raw_grip`], but returns the
    /// raw writing grip.
    fn as_raw_write_grip(&self) -> RawGrip;
}

/// Portability abstraction over `AsReadWriteFd` and
/// `AsReadWriteHandleOrSocket`.
#[cfg(windows)]
pub trait AsRawReadWriteGrip: AsRawReadWriteHandleOrSocket {
    /// Extracts the grip for reading.
    ///
    /// Like [`AsRawGrip::as_raw_grip`], but returns the
    /// raw reading grip.
    fn as_raw_read_grip(&self) -> RawGrip;

    /// Extracts the grip for writing.
    ///
    /// Like [`AsRawGrip::as_raw_grip`], but returns the
    /// raw writing grip.
    fn as_raw_write_grip(&self) -> RawGrip;
}

/// Portability abstraction over `IntoRawFd` and
/// `IntoRawHandleOrSocket`.
#[cfg(not(windows))]
pub trait IntoRawGrip: IntoRawFd {
    /// Consume `self` and convert into an `RawGrip`.
    fn into_raw_grip(self) -> RawGrip;
}

/// Portability abstraction over `IntoRawFd` and
/// `IntoRawHandleOrSocket`.
#[cfg(windows)]
pub trait IntoRawGrip: IntoRawHandleOrSocket {
    /// Consume `self` and convert into an `RawGrip`.
    fn into_raw_grip(self) -> RawGrip;
}

/// Portability abstraction over `From<OwnedFd>` and
/// `From<OwnedHandleOrSocket>`.
#[cfg(not(windows))]
pub trait FromRawGrip: FromRawFd {
    /// Consume an `RawGrip` and convert into a `Self`.
    ///
    /// # Safety
    ///
    /// `raw_grip` must be a suitable grip for assuming ownership.
    unsafe fn from_raw_grip(raw_grip: RawGrip) -> Self;
}

/// Portability abstraction over `From<OwnedFd>` and
/// `From<OwnedHandleOrSocket>`.
#[cfg(windows)]
pub trait FromRawGrip: FromRawHandleOrSocket {
    /// Consume an `RawGrip` and convert into a `Self`.
    ///
    /// # Safety
    ///
    /// `raw_grip` must be a suitable grip for assuming ownership.
    unsafe fn from_raw_grip(raw_grip: RawGrip) -> Self;
}

#[cfg(not(windows))]
impl<T: AsRawFd> AsRawGrip for T {
    #[inline]
    fn as_raw_grip(&self) -> RawGrip {
        self.as_raw_fd()
    }
}

#[cfg(windows)]
impl<T: AsRawHandleOrSocket> AsRawGrip for T {
    #[inline]
    fn as_raw_grip(&self) -> RawGrip {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<T: AsRawReadWriteFd> AsRawReadWriteGrip for T {
    #[inline]
    fn as_raw_read_grip(&self) -> RawGrip {
        self.as_raw_read_fd()
    }

    #[inline]
    fn as_raw_write_grip(&self) -> RawGrip {
        self.as_raw_write_fd()
    }
}

#[cfg(windows)]
impl<T: AsRawReadWriteHandleOrSocket> AsRawReadWriteGrip for T {
    #[inline]
    fn as_raw_read_grip(&self) -> RawGrip {
        self.as_raw_read_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_grip(&self) -> RawGrip {
        self.as_raw_write_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<T: IntoRawFd> IntoRawGrip for T {
    #[inline]
    fn into_raw_grip(self) -> RawGrip {
        self.into_raw_fd()
    }
}

#[cfg(windows)]
impl<T: IntoRawHandleOrSocket> IntoRawGrip for T {
    #[inline]
    fn into_raw_grip(self) -> RawGrip {
        self.into_raw_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl<T: FromRawFd> FromRawGrip for T {
    #[inline]
    unsafe fn from_raw_grip(raw_grip: RawGrip) -> Self {
        Self::from_raw_fd(raw_grip)
    }
}

#[cfg(windows)]
impl<T: FromRawHandleOrSocket> FromRawGrip for T {
    #[inline]
    unsafe fn from_raw_grip(raw_grip: RawGrip) -> Self {
        Self::from_raw_handle_or_socket(raw_grip)
    }
}

/// Portability abstraction over `BorrowedFd::from_raw_fd` and
/// `BorrowedHandleOrSocket::from_raw_handle_or_socket`.
///
/// # Safety
///
/// See the safety conditions for [`borrow_raw`].
///
/// [`borrow_raw`]: https://doc.rust-lang.org/stable/std/os/unix/io/struct.BorrowedFd.html#method.borrow_raw
#[cfg(not(windows))]
#[must_use]
#[inline]
pub unsafe fn borrow_raw<'a>(grip: RawGrip) -> BorrowedGrip<'a> {
    BorrowedFd::borrow_raw(grip)
}

/// Portability abstraction over `BorrowedFd::from_raw_fd` and
/// `BorrowedHandleOrSocket::from_raw_handle_or_socket`.
///
/// # Safety
///
/// See the safety conditions for [`BorrowedHandle::borrow_raw`], and
/// [`BorrowedSocket::borrow_raw`].
///
/// [`BorrowedHandle::borrow_raw`]: https://doc.rust-lang.org/stable/std/os/windows/io/struct.BorrowedHandle.html#method.borrow_raw
/// [`BorrowedSocket::borrow_raw`]: https://doc.rust-lang.org/stable/std/os/windows/io/struct.BorrowedSocket.html#method.borrow_raw
#[cfg(windows)]
#[must_use]
#[inline]
pub unsafe fn borrow_raw<'a>(grip: RawGrip) -> BorrowedGrip<'a> {
    BorrowedHandleOrSocket::borrow_raw(grip)
}
