use crate::net::Shutdown;
use crate::os::unix::net::SocketAddr;
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
use std::fmt;
use std::io::{self, IoSlice, IoSliceMut, Read, Write};
use std::os::unix;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::time::Duration;

/// A Unix stream socket.
///
/// This corresponds to [`std::os::unix::net::UnixStream`].
///
/// This `UnixStream` has no `connect` method. To create a `UnixStream`, first
/// obtain a [`Dir`] containing the path, and then call
/// [`Dir::connect_unix_stream`].
///
/// [`std::os::unix::net::UnixStream`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::connect_unix_stream`]: struct.Dir.html#method.connect_unix_stream
pub struct UnixStream {
    std: unix::net::UnixStream,
}

impl UnixStream {
    /// Constructs a new instance of `Self` from the given
    /// `std::os::unix::net::UnixStream`.
    ///
    /// This grants access the resources the `std::os::unix::net::UnixStream`
    /// instance already has access to.
    #[inline]
    pub fn from_std(std: unix::net::UnixStream) -> Self {
        Self { std }
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::pair`].
    ///
    /// TODO: should this require a capability?
    ///
    /// [`std::os::unix::net::UnixStream::pair`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.pair
    #[inline]
    pub fn pair() -> io::Result<(Self, Self)> {
        unix::net::UnixStream::pair().map(|(a, b)| (Self::from_std(a), Self::from_std(b)))
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::try_clone`].
    ///
    /// [`std::os::unix::net::UnixStream::try_clone`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.try_clone
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        let unix_stream = self.std.try_clone()?;
        Ok(Self::from_std(unix_stream))
    }

    /// Returns the socket address of the local half of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::local_addr`].
    ///
    /// [`std::os::unix::net::UnixStream::local_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Returns the socket address of the remote half of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::peer_addr`].
    ///
    /// [`std::os::unix::net::UnixStream::peer_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.peer_addr
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Sets the read timeout for the socket.
    ///
    /// This corresponds to
    /// [`std::os::unix::net::UnixStream::set_read_timeout`].
    ///
    /// [`std::os::unix::net::UnixStream::set_read_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.set_read_timeout
    #[inline]
    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.std.set_read_timeout(timeout)
    }

    /// Sets the write timeout for the socket.
    ///
    /// This corresponds to
    /// [`std::os::unix::net::UnixStream::set_write_timeout`].
    ///
    /// [`std::os::unix::net::UnixStream::set_write_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.set_write_timeout
    #[inline]
    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.std.set_write_timeout(timeout)
    }

    /// Returns the read timeout of this socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::read_timeout`].
    ///
    /// [`std::os::unix::net::UnixStream::read_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.read_timeout
    #[inline]
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.read_timeout()
    }

    /// Returns the write timeout of this socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::write_timeout`].
    ///
    /// [`std::os::unix::net::UnixStream::write_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.write_timeout
    #[inline]
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.write_timeout()
    }

    /// Moves the socket into or out of nonblocking mode.
    ///
    /// This corresponds to
    /// [`std::os::unix::net::UnixStream::set_nonblocking`].
    ///
    /// [`std::os::unix::net::UnixStream::set_nonblocking`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.set_nonblocking
    #[inline]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.std.set_nonblocking(nonblocking)
    }

    /// Returns the value of the `SO_ERROR` option.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::take_error`].
    ///
    /// [`std::os::unix::net::UnixStream::take_error`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.take_error
    #[inline]
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.std.take_error()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixStream::shutdown`].
    ///
    /// [`std::os::unix::net::UnixStream::shutdown`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html#method.shutdown
    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.std.shutdown(how)
    }
}

impl FromRawFd for UnixStream {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(unix::net::UnixStream::from_raw_fd(fd))
    }
}

impl From<OwnedFd> for UnixStream {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        Self::from_std(unix::net::UnixStream::from(fd))
    }
}

impl AsRawFd for UnixStream {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

impl AsFd for UnixStream {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

impl IntoRawFd for UnixStream {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

impl From<UnixStream> for OwnedFd {
    #[inline]
    fn from(stream: UnixStream) -> OwnedFd {
        stream.std.into()
    }
}

impl Read for UnixStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        self.std.read_vectored(bufs)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.std.read_exact(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.std.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.std.read_to_string(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.std.is_read_vectored()
    }
}

impl Read for &UnixStream {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&mut &self.std).read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut]) -> io::Result<usize> {
        (&mut &self.std).read_vectored(bufs)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        (&mut &self.std).read_exact(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        (&mut &self.std).read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        (&mut &self.std).read_to_string(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_read_vectored(&self) -> bool {
        self.std.is_read_vectored()
    }
}

impl Write for UnixStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.std.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.std.flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        self.std.write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.std.write_all(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.std.is_write_vectored()
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice]) -> io::Result<()> {
        self.std.write_all_vectored(bufs)
    }
}

impl Write for &UnixStream {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&mut &self.std).write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        (&mut &self.std).flush()
    }

    #[inline]
    fn write_vectored(&mut self, bufs: &[IoSlice]) -> io::Result<usize> {
        (&mut &self.std).write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        (&mut &self.std).write_all(buf)
    }

    #[cfg(can_vector)]
    #[inline]
    fn is_write_vectored(&self) -> bool {
        self.std.is_write_vectored()
    }

    #[cfg(write_all_vectored)]
    #[inline]
    fn write_all_vectored(&mut self, bufs: &mut [IoSlice]) -> io::Result<()> {
        (&mut &self.std).write_all_vectored(bufs)
    }
}

impl fmt::Debug for UnixStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
