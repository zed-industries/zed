use crate::net::{Incoming, SocketAddr, TcpStream};
#[cfg(not(windows))]
use io_extras::os::rustix::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsSocket, BorrowedSocket, OwnedSocket};
use std::{fmt, io, net};
#[cfg(windows)]
use {
    io_extras::os::windows::{
        AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, IntoRawHandleOrSocket,
        OwnedHandleOrSocket, RawHandleOrSocket,
    },
    std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket},
};

/// A TCP socket server, listening for connections.
///
/// This corresponds to [`std::net::TcpListener`].
///
/// This `TcpListener` has no `bind` method. To bind it to a socket address,
/// first obtain a [`Pool`] permitting the address, and then call
/// [`Pool::bind_tcp_listener`].
///
/// [`Pool`]: struct.Pool.html
/// [`Pool::bind_tcp_listener`]: struct.Pool.html#method.bind_tcp_listener
pub struct TcpListener {
    std: net::TcpListener,
}

impl TcpListener {
    /// Constructs a new instance of `Self` from the given
    /// `std::net::TcpListener`.
    ///
    /// This grants access the resources the `std::net::TcpListener` instance
    /// already has access to.
    #[inline]
    pub fn from_std(std: net::TcpListener) -> Self {
        Self { std }
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::net::TcpListener::try_clone`].
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        let tcp_listener = self.std.try_clone()?;
        Ok(Self::from_std(tcp_listener))
    }

    /// Accept a new incoming connection from this listener.
    ///
    /// This corresponds to [`std::net::TcpListener::accept`].
    #[inline]
    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.std
            .accept()
            .map(|(tcp_stream, addr)| (TcpStream::from_std(tcp_stream), addr))
    }

    /// Returns an iterator over the connections being received on this
    /// listener.
    ///
    /// This corresponds to [`std::net::TcpListener::incoming`].
    #[inline]
    pub fn incoming(&self) -> Incoming {
        let incoming = self.std.incoming();
        Incoming::from_std(incoming)
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpListener::set_ttl`].
    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.std.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::TcpListener::ttl`].
    #[inline]
    pub fn ttl(&self) -> io::Result<u32> {
        self.std.ttl()
    }

    /// Gets the value of the `SO_ERROR` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpListener::take_error`].
    #[inline]
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.std.take_error()
    }

    /// Moves this TCP stream into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::net::TcpListener::set_nonblocking`].
    #[inline]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.std.set_nonblocking(nonblocking)
    }
}

// Safety: `SocketlikeViewType` is implemented for `std`'s socket types.
unsafe impl io_lifetimes::views::SocketlikeViewType for TcpListener {}

#[cfg(not(windows))]
impl FromRawFd for TcpListener {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(net::TcpListener::from_raw_fd(fd))
    }
}

#[cfg(not(windows))]
impl From<OwnedFd> for TcpListener {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        Self::from_std(net::TcpListener::from(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpListener {
    #[inline]
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_std(net::TcpListener::from_raw_socket(socket))
    }
}

#[cfg(windows)]
impl From<OwnedSocket> for TcpListener {
    #[inline]
    fn from(socket: OwnedSocket) -> Self {
        Self::from_std(net::TcpListener::from(socket))
    }
}

#[cfg(not(windows))]
impl AsRawFd for TcpListener {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for TcpListener {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpListener {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.std.as_raw_socket()
    }
}

#[cfg(windows)]
impl AsSocket for TcpListener {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket<'_> {
        self.std.as_socket()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for TcpListener {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.std.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsHandleOrSocket for TcpListener {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.std.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl IntoRawFd for TcpListener {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

#[cfg(not(windows))]
impl From<TcpListener> for OwnedFd {
    #[inline]
    fn from(listener: TcpListener) -> OwnedFd {
        listener.std.into()
    }
}

#[cfg(windows)]
impl IntoRawSocket for TcpListener {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.std.into_raw_socket()
    }
}

#[cfg(windows)]
impl From<TcpListener> for OwnedSocket {
    #[inline]
    fn from(listener: TcpListener) -> OwnedSocket {
        listener.std.into()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for TcpListener {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.std.into_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl From<TcpListener> for OwnedHandleOrSocket {
    #[inline]
    fn from(listener: TcpListener) -> Self {
        listener.std.into()
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
