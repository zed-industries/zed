use crate::net::Shutdown;
use crate::os::unix::net::SocketAddr;
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
use std::os::unix;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::time::Duration;
use std::{fmt, io};

/// A Unix datagram socket.
///
/// This corresponds to [`std::os::unix::net::UnixDatagram`].
///
/// This `UnixDatagram` has no `bind`, `connect`, or `send_to` methods. To
/// create a `UnixDatagram`, first obtain a [`Dir`] containing the path, and
/// then call [`Dir::bind_unix_datagram`], [`Dir::connect_unix_datagram`], or
/// [`Dir::send_to_unix_datagram_addr`].
///
/// [`std::os::unix::net::UnixDatagram`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html
/// [`Dir`]: struct.Dir.html
/// [`Dir::connect_unix_datagram`]: struct.Dir.html#method.connect_unix_datagram
/// [`Dir::bind_unix_datagram`]: struct.Dir.html#method.bind_unix_datagram
/// [`Dir::send_to_unix_datagram_addr`]: struct.Dir.html#method.send_to_unix_datagram_addr
pub struct UnixDatagram {
    std: unix::net::UnixDatagram,
}

impl UnixDatagram {
    /// Constructs a new instance of `Self` from the given
    /// `std::os::unix::net::UnixDatagram`.
    ///
    /// This grants access the resources the `std::os::unix::net::UnixDatagram`
    /// instance already has access to.
    #[inline]
    pub fn from_std(std: unix::net::UnixDatagram) -> Self {
        Self { std }
    }

    /// Creates a Unix Datagram socket which is not bound to any address.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::unbound`].
    ///
    /// TODO: should this require a capability?
    ///
    /// [`std::os::unix::net::UnixDatagram::unbound`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.unbound
    #[inline]
    pub fn unbound() -> io::Result<Self> {
        let unix_datagram = unix::net::UnixDatagram::unbound()?;
        Ok(Self::from_std(unix_datagram))
    }

    /// Creates an unnamed pair of connected sockets.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::pair`].
    ///
    /// TODO: should this require a capability?
    ///
    /// [`std::os::unix::net::UnixDatagram::pair`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.pair
    #[inline]
    pub fn pair() -> io::Result<(Self, Self)> {
        unix::net::UnixDatagram::pair().map(|(a, b)| (Self::from_std(a), Self::from_std(b)))
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::try_clone`].
    ///
    /// [`std::os::unix::net::UnixDatagram::try_clone`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.try_clone
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        let unix_datagram = self.std.try_clone()?;
        Ok(Self::from_std(unix_datagram))
    }

    /// Returns the address of this socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::local_addr`].
    ///
    /// [`std::os::unix::net::UnixDatagram::local_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.local_addr
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Returns the address of this socket's peer.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::peer_addr`].
    ///
    /// [`std::os::unix::net::UnixDatagram::peer_addr`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.peer_addr
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Receives data from the socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::recv_from`].
    ///
    /// [`std::os::unix::net::UnixDatagram::recv_from`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.recv_from
    #[inline]
    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.std.recv_from(buf)
    }

    /// Receives data from the socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::recv`].
    ///
    /// [`std::os::unix::net::UnixDatagram::recv`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.recv
    #[inline]
    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.recv(buf)
    }

    /// Sends data on the socket to the socket's peer.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::send`].
    ///
    /// [`std::os::unix::net::UnixDatagram::send`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.send
    #[inline]
    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.std.send(buf)
    }

    /// Sets the read timeout for the socket.
    ///
    /// This corresponds to
    /// [`std::os::unix::net::UnixDatagram::set_read_timeout`].
    ///
    /// [`std::os::unix::net::UnixDatagram::set_read_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.set_read_timeout
    #[inline]
    pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.std.set_read_timeout(timeout)
    }

    /// Sets the write timeout for the socket.
    ///
    /// This corresponds to
    /// [`std::os::unix::net::UnixDatagram::set_write_timeout`].
    ///
    /// [`std::os::unix::net::UnixDatagram::set_write_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.set_write_timeout
    #[inline]
    pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
        self.std.set_write_timeout(timeout)
    }

    /// Returns the read timeout of this socket.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::read_timeout`].
    ///
    /// [`std::os::unix::net::UnixDatagram::read_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.read_timeout
    #[inline]
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.read_timeout()
    }

    /// Returns the write timeout of this socket.
    ///
    /// This corresponds to
    /// [`std::os::unix::net::UnixDatagram::write_timeout`].
    ///
    /// [`std::os::unix::net::UnixDatagram::write_timeout`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.write_timeout
    #[inline]
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.write_timeout()
    }

    /// Moves the socket into or out of nonblocking mode.
    ///
    /// This corresponds to
    /// [`std::os::unix::net::UnixDatagram::set_nonblocking`].
    ///
    /// [`std::os::unix::net::UnixDatagram::set_nonblocking`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.set_nonblocking
    #[inline]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.std.set_nonblocking(nonblocking)
    }

    /// Returns the value of the `SO_ERROR` option.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::take_error`].
    ///
    /// [`std::os::unix::net::UnixDatagram::take_error`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.take_error
    #[inline]
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.std.take_error()
    }

    /// Shut down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`std::os::unix::net::UnixDatagram::shutdown`].
    ///
    /// [`std::os::unix::net::UnixDatagram::shutdown`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixDatagram.html#method.shutdown
    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.std.shutdown(how)
    }
}

impl FromRawFd for UnixDatagram {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(unix::net::UnixDatagram::from_raw_fd(fd))
    }
}

impl From<OwnedFd> for UnixDatagram {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        Self::from_std(unix::net::UnixDatagram::from(fd))
    }
}

impl AsRawFd for UnixDatagram {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

impl AsFd for UnixDatagram {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

impl IntoRawFd for UnixDatagram {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

impl From<UnixDatagram> for OwnedFd {
    #[inline]
    fn from(datagram: UnixDatagram) -> OwnedFd {
        datagram.std.into()
    }
}

impl fmt::Debug for UnixDatagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
