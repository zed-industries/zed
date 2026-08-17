//! Traits for working with types that may have up to two I/O objects.

#[cfg(windows)]
use crate::os::windows::{
    AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, RawHandleOrSocket,
};
use std::fs::File;
use std::net::TcpStream;
#[cfg(unix)]
use std::os::unix::net::UnixStream;
#[cfg(not(windows))]
use {
    crate::os::rustix::{AsRawFd, RawFd},
    io_lifetimes::{AsFd, BorrowedFd},
};

/// Like [`AsRawFd`], but for types which may have one or two file descriptors,
/// for reading and writing.
///
/// For types that only have one, both functions return the same value.
#[cfg(not(windows))]
pub trait AsRawReadWriteFd {
    /// Extracts the raw file descriptor for reading.
    ///
    /// Like [`AsRawFd::as_raw_fd`], but returns the reading file descriptor.
    fn as_raw_read_fd(&self) -> RawFd;

    /// Extracts the raw file descriptor for writing.
    ///
    /// Like [`AsRawFd::as_raw_fd`], but returns the writing file descriptor.
    fn as_raw_write_fd(&self) -> RawFd;
}

/// Like [`AsFd`], but for types which may have one or two file descriptors,
/// for reading and writing.
///
/// For types that only have one, both functions return the same value.
#[cfg(not(windows))]
pub trait AsReadWriteFd {
    /// Extracts the file descriptor for reading.
    ///
    /// Like [`AsFd::as_fd`], but returns the reading file descriptor.
    fn as_read_fd(&self) -> BorrowedFd<'_>;

    /// Extracts the file descriptor for writing.
    ///
    /// Like [`AsFd::as_fd`], but returns the writing file descriptor.
    fn as_write_fd(&self) -> BorrowedFd<'_>;
}

/// Like [`AsRawHandleOrSocket`], but for types which may have one or two
/// handles or sockets, for reading and writing.
///
/// For types that only have one, both functions return the same value.
#[cfg(windows)]
pub trait AsRawReadWriteHandleOrSocket {
    /// Extracts the raw handle or socket for reading.
    ///
    /// Like [`AsRawHandleOrSocket::as_raw_handle_or_socket`], but returns the
    /// reading handle.
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket;

    /// Extracts the raw handle or socket for writing.
    ///
    /// Like [`AsRawHandleOrSocket::as_raw_handle_or_socket`], but returns the
    /// writing handle.
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket;
}

/// Like [`AsHandleOrSocket`], but for types which may have one or two
/// handles or sockets, for reading and writing.
///
/// For types that only have one, both functions return the same value.
#[cfg(windows)]
pub trait AsReadWriteHandleOrSocket {
    /// Extracts the handle or socket for reading.
    ///
    /// Like [`AsHandleOrSocket::as_handle_or_socket`], but returns the
    /// reading handle.
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_>;

    /// Extracts the handle or socket for writing.
    ///
    /// Like [`AsHandleOrSocket::as_handle_or_socket`], but returns the
    /// writing handle.
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_>;
}

#[cfg(not(windows))]
impl AsRawReadWriteFd for File {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsReadWriteFd for File {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(windows)]
impl AsRawReadWriteHandleOrSocket for File {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsReadWriteHandleOrSocket for File {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl AsRawReadWriteFd for TcpStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsReadWriteFd for TcpStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(windows)]
impl AsRawReadWriteHandleOrSocket for TcpStream {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsReadWriteHandleOrSocket for TcpStream {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(unix)]
impl AsRawReadWriteFd for UnixStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(unix)]
impl AsReadWriteFd for UnixStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(feature = "async-std", not(windows)))]
impl AsRawReadWriteFd for async_std::fs::File {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "async-std", not(windows)))]
impl AsReadWriteFd for async_std::fs::File {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(feature = "async-std", windows))]
impl AsRawReadWriteHandleOrSocket for async_std::fs::File {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(feature = "async-std", windows))]
impl AsReadWriteHandleOrSocket for async_std::fs::File {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(feature = "async-std", not(windows)))]
impl AsRawReadWriteFd for async_std::net::TcpStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "async-std", not(windows)))]
impl AsReadWriteFd for async_std::net::TcpStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(feature = "async-std", windows))]
impl AsRawReadWriteHandleOrSocket for async_std::net::TcpStream {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(feature = "async-std", windows))]
impl AsReadWriteHandleOrSocket for async_std::net::TcpStream {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(feature = "async-std", unix))]
impl AsRawReadWriteFd for async_std::os::unix::net::UnixStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "async-std", unix))]
impl AsReadWriteFd for async_std::os::unix::net::UnixStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(feature = "tokio", not(windows)))]
impl AsRawReadWriteFd for tokio::fs::File {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "tokio", not(windows)))]
impl AsReadWriteFd for tokio::fs::File {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(feature = "tokio", windows))]
impl AsRawReadWriteHandleOrSocket for tokio::fs::File {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(feature = "tokio", windows))]
impl AsReadWriteHandleOrSocket for tokio::fs::File {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(feature = "tokio", not(windows)))]
impl AsRawReadWriteFd for tokio::net::TcpStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "tokio", not(windows)))]
impl AsReadWriteFd for tokio::net::TcpStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(feature = "tokio", windows))]
impl AsReadWriteHandleOrSocket for tokio::net::TcpStream {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(feature = "tokio", unix))]
impl AsRawReadWriteFd for tokio::net::UnixStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(feature = "tokio", unix))]
impl AsReadWriteFd for tokio::net::UnixStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(not(windows))]
impl<T: AsRawReadWriteFd> AsRawReadWriteFd for Box<T> {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        (**self).as_raw_read_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        (**self).as_raw_write_fd()
    }
}

#[cfg(not(windows))]
impl<T: AsReadWriteFd> AsReadWriteFd for Box<T> {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        (**self).as_read_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        (**self).as_write_fd()
    }
}

#[cfg(windows)]
impl<T: AsRawReadWriteHandleOrSocket> AsRawReadWriteHandleOrSocket for Box<T> {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        (**self).as_raw_read_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        (**self).as_raw_write_handle_or_socket()
    }
}

#[cfg(windows)]
impl<T: AsReadWriteHandleOrSocket> AsReadWriteHandleOrSocket for Box<T> {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        (**self).as_read_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        (**self).as_write_handle_or_socket()
    }
}

#[cfg(all(not(windows), feature = "socket2"))]
impl AsRawReadWriteFd for socket2::Socket {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(not(windows), feature = "socket2"))]
impl AsReadWriteFd for socket2::Socket {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(windows, feature = "socket2"))]
impl AsRawReadWriteHandleOrSocket for socket2::Socket {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(windows, feature = "socket2"))]
impl AsReadWriteHandleOrSocket for socket2::Socket {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsRawReadWriteFd for mio::net::TcpListener {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsReadWriteFd for mio::net::TcpListener {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsRawReadWriteHandleOrSocket for mio::net::TcpListener {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
#[cfg(not(io_lifetimes_use_std))] // TODO: Enable once we have mio impls
impl AsReadWriteHandleOrSocket for mio::net::TcpListener {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsRawReadWriteFd for mio::net::TcpStream {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsReadWriteFd for mio::net::TcpStream {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsRawReadWriteHandleOrSocket for mio::net::TcpStream {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
#[cfg(not(io_lifetimes_use_std))] // TODO: Enable once we have mio impls
impl AsReadWriteHandleOrSocket for mio::net::TcpStream {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsRawReadWriteFd for mio::net::UdpSocket {
    #[inline]
    fn as_raw_read_fd(&self) -> RawFd {
        self.as_raw_fd()
    }

    #[inline]
    fn as_raw_write_fd(&self) -> RawFd {
        self.as_raw_fd()
    }
}

#[cfg(all(not(windows), feature = "use_mio_net"))]
impl AsReadWriteFd for mio::net::UdpSocket {
    #[inline]
    fn as_read_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_write_fd(&self) -> BorrowedFd<'_> {
        self.as_fd()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
impl AsRawReadWriteHandleOrSocket for mio::net::UdpSocket {
    #[inline]
    fn as_raw_read_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }

    #[inline]
    fn as_raw_write_handle_or_socket(&self) -> RawHandleOrSocket {
        self.as_raw_handle_or_socket()
    }
}

#[cfg(all(windows, feature = "use_mio_net"))]
#[cfg(not(io_lifetimes_use_std))] // TODO: Enable once we have mio impls
impl AsReadWriteHandleOrSocket for mio::net::UdpSocket {
    #[inline]
    fn as_read_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }

    #[inline]
    fn as_write_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.as_handle_or_socket()
    }
}

/// Adapt an `AsReadWriteGrip` implementation to implement
/// `AsGrip` with the read handle.
#[allow(clippy::exhaustive_structs)]
#[derive(Debug, Copy, Clone)]
pub struct ReadHalf<'a, RW>(&'a RW);

impl<'a, RW> ReadHalf<'a, RW> {
    /// Returns a new instance of `Self`.
    #[inline]
    pub const fn new(rw: &'a RW) -> Self {
        Self(rw)
    }
}

#[cfg(not(windows))]
impl<'a, RW: AsReadWriteFd> AsFd for ReadHalf<'a, RW> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'a> {
        self.0.as_read_fd()
    }
}

#[cfg(windows)]
impl<'a, RW: AsReadWriteHandleOrSocket> AsHandleOrSocket for ReadHalf<'a, RW> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'a> {
        self.0.as_read_handle_or_socket()
    }
}

/// Adapt an `AsReadWriteGrip` implementation to implement
/// `AsGrip` with the write handle.
#[allow(clippy::exhaustive_structs)]
#[derive(Debug, Copy, Clone)]
pub struct WriteHalf<'a, RW>(&'a RW);

impl<'a, RW> WriteHalf<'a, RW> {
    /// Returns a new instance of `Self`.
    #[inline]
    pub const fn new(rw: &'a RW) -> Self {
        Self(rw)
    }
}

#[cfg(not(windows))]
impl<'a, RW: AsReadWriteFd> AsFd for WriteHalf<'a, RW> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'a> {
        self.0.as_write_fd()
    }
}

#[cfg(windows)]
impl<'a, RW: AsReadWriteHandleOrSocket> AsHandleOrSocket for WriteHalf<'a, RW> {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'a> {
        self.0.as_write_handle_or_socket()
    }
}
