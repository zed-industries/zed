//! `BorrowedReadable` and `BorrowedWriteable`.

use crate::grip::{AsRawGrip, BorrowedGrip, FromRawGrip};
#[cfg(windows)]
use crate::os::windows::{AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket};
use crate::raw::{RawReadable, RawWriteable};
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd};
use std::fmt;
use std::io::{self, IoSlice, IoSliceMut, Read, Write};
use std::marker::PhantomData;
#[cfg(all(doc, not(windows)))]
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::io::AsRawFd;
#[cfg(target_os = "wasi")]
use std::os::wasi::io::AsRawFd;

/// An owning I/O handle that implements [`Read`].
///
/// This doesn't implement `Into*` or `From*` traits.
///
/// # Platform-specific behavior
///
/// On Posix-ish platforms, this reads from the handle as if it were a
/// [`File`]. On Windows, this reads from a file-like handle as if it were a
/// [`File`], and from a socket-like handle as if it were a [`TcpStream`].
#[repr(transparent)]
pub struct BorrowedReadable<'a> {
    raw: RawReadable,
    _phantom: PhantomData<&'a ()>,
}

/// An owning I/O handle that implements [`Write`].
///
/// This doesn't implement `Into*` or `From*` traits.
///
/// # Platform-specific behavior
///
/// On Posix-ish platforms, this writes to the handle as if it were a
/// [`File`]. On Windows, this writes to a file-like handle as if it were a
/// [`File`], and to a socket-like handle as if it were a [`TcpStream`].
#[repr(transparent)]
pub struct BorrowedWriteable<'a> {
    raw: RawWriteable,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> BorrowedReadable<'a> {
    /// Create a `BorrowedReadable` that can read from a `BorrowedGrip`.
    #[must_use]
    #[inline]
    pub fn borrow(grip: BorrowedGrip<'a>) -> Self {
        Self {
            raw: unsafe { RawReadable::from_raw_grip(grip.as_raw_grip()) },
            _phantom: PhantomData,
        }
    }
}

impl<'a> BorrowedWriteable<'a> {
    /// Create a `BorrowedReadable` that can write to a `BorrowedGrip`.
    #[must_use]
    #[inline]
    pub fn borrow(grip: BorrowedGrip<'a>) -> Self {
        Self {
            raw: unsafe { RawWriteable::from_raw_grip(grip.as_raw_grip()) },
            _phantom: PhantomData,
        }
    }
}

/// `BorrowedReadable` borrows its handle.
#[cfg(not(windows))]
impl<'a> AsFd for BorrowedReadable<'a> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw(self.raw.as_raw_fd()) }
    }
}

/// `BorrowedWriteable` borrows its handle.
#[cfg(not(windows))]
impl<'a> AsFd for BorrowedWriteable<'a> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw(self.raw.as_raw_fd()) }
    }
}

// Windows implementations.

/// `BorrowedReadable` borrows its handle.
#[cfg(windows)]
impl<'a> AsHandleOrSocket for BorrowedReadable<'a> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'a> {
        unsafe { BorrowedHandleOrSocket::borrow_raw(self.raw.as_raw_handle_or_socket()) }
    }
}

/// `BorrowedWriteable` borrows its handle.
#[cfg(windows)]
impl<'a> AsHandleOrSocket for BorrowedWriteable<'a> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'a> {
        unsafe { BorrowedHandleOrSocket::borrow_raw(self.raw.as_raw_handle_or_socket()) }
    }
}

#[cfg(not(windows))]
impl<'a> Read for BorrowedReadable<'a> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.raw.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.raw.read_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.raw.is_read_vectored()
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.raw.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.raw.read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.raw.read_exact(buf)
    }
}

#[cfg(windows)]
impl<'a> Read for BorrowedReadable<'a> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.raw.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.raw.read_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.raw.is_read_vectored()
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.raw.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.raw.read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.raw.read_exact(buf)
    }
}

#[cfg(not(windows))]
impl<'a> Write for BorrowedWriteable<'a> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.raw.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.raw.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.raw.write_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.raw.is_write_vectored()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.raw.write_all(buf)
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        self.raw.write_all_vectored(bufs)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.raw.write_fmt(fmt)
    }
}

#[cfg(windows)]
impl<'a> Write for BorrowedWriteable<'a> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.raw.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.raw.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.raw.write_vectored(bufs)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.raw.is_write_vectored()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.raw.write_all(buf)
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()> {
        self.raw.write_all_vectored(bufs)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        self.raw.write_fmt(fmt)
    }
}

#[cfg(not(windows))]
impl<'a> fmt::Debug for BorrowedReadable<'a> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw fd number.
        f.debug_struct("BorrowedReadable")
            .field("fd", &self.raw)
            .finish()
    }
}

#[cfg(windows)]
impl<'a> fmt::Debug for BorrowedReadable<'a> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw handle or socket.
        f.debug_struct("BorrowedReadable")
            .field("handle_or_socket", &self.raw)
            .finish()
    }
}

#[cfg(not(windows))]
impl<'a> fmt::Debug for BorrowedWriteable<'a> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw fd number.
        f.debug_struct("BorrowedWriteable")
            .field("fd", &self.raw)
            .finish()
    }
}

#[cfg(windows)]
impl<'a> fmt::Debug for BorrowedWriteable<'a> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Just print the raw handle or socket.
        f.debug_struct("BorrowedWriteable")
            .field("handle_or_socket", &self.raw)
            .finish()
    }
}
