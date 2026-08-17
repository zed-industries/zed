use crate::net::{Shutdown, SocketAddr};
#[cfg(not(windows))]
use io_extras::os::rustix::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsSocket, BorrowedSocket, OwnedSocket};
use std::io::{self, IoSlice, IoSliceMut, Read, Write};
use std::time::Duration;
use std::{fmt, net};
#[cfg(windows)]
use {
    io_extras::os::windows::{
        AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, IntoRawHandleOrSocket,
        OwnedHandleOrSocket, RawHandleOrSocket,
    },
    std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket},
};

/// A TCP stream between a local and a remote socket.
///
/// This corresponds to [`std::net::TcpStream`].
///
/// This `TcpStream` has no `connect` method. To create a `TcpStream`, first
/// obtain a [`Pool`] permitting the address, and then call
/// [`Pool::connect_tcp_stream`].
///
/// [`Pool`]: struct.Pool.html
/// [`Pool::connect_tcp_stream`]: struct.Pool.html#method.connect_tcp_stream
pub struct TcpStream {
    std: net::TcpStream,
}

impl TcpStream {
    /// Constructs a new instance of `Self` from the given
    /// `std::net::TcpStream`.
    ///
    /// This grants access the resources the `std::net::TcpStream` instance
    /// already has access to.
    #[inline]
    pub fn from_std(std: net::TcpStream) -> Self {
        Self { std }
    }

    /// Returns the socket address of the remote peer of this TCP connection.
    ///
    /// This corresponds to [`std::net::TcpStream::peer_addr`].
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Returns the local socket address of this listener.
    ///
    /// This corresponds to [`std::net::TcpStream::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This corresponds to [`std::net::TcpStream::shutdown`].
    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.std.shutdown(how)
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::net::TcpStream::try_clone`].
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        let tcp_stream = self.std.try_clone()?;
        Ok(Self::from_std(tcp_stream))
    }

    /// Sets the read timeout to the timeout specified.
    ///
    /// This corresponds to [`std::net::TcpStream::set_read_timeout`].
    #[inline]
    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.std.set_read_timeout(dur)
    }

    /// Sets the write timeout to the timeout specified.
    ///
    /// This corresponds to [`std::net::TcpStream::set_write_timeout`].
    #[inline]
    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.std.set_write_timeout(dur)
    }

    /// Returns the read timeout of this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::read_timeout`].
    #[inline]
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.read_timeout()
    }

    /// Returns the write timeout of this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::write_timeout`].
    #[inline]
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.write_timeout()
    }

    /// Receives data on the socket from the remote address to which it is
    /// connected, without removing that data from the queue.
    ///
    /// This corresponds to [`std::net::TcpStream::peek`].
    #[inline]
    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.peek(buf)
    }

    /// Sets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::set_nodelay`].
    #[inline]
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.std.set_nodelay(nodelay)
    }

    /// Gets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::nodelay`].
    #[inline]
    pub fn nodelay(&self) -> io::Result<bool> {
        self.std.nodelay()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::set_ttl`].
    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.std.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::ttl`].
    #[inline]
    pub fn ttl(&self) -> io::Result<u32> {
        self.std.ttl()
    }

    /// Gets the value of the `SO_ERROR` option on this socket.
    ///
    /// This corresponds to [`std::net::TcpStream::take_error`].
    #[inline]
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.std.take_error()
    }

    /// Moves this TCP stream into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::net::TcpStream::set_nonblocking`].
    #[inline]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.std.set_nonblocking(nonblocking)
    }
}

// Safety: `SocketlikeViewType` is implemented for `std`'s socket types.
unsafe impl io_lifetimes::views::SocketlikeViewType for TcpStream {}

#[cfg(not(windows))]
impl FromRawFd for TcpStream {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(net::TcpStream::from_raw_fd(fd))
    }
}

#[cfg(not(windows))]
impl From<OwnedFd> for TcpStream {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        Self::from_std(net::TcpStream::from(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for TcpStream {
    #[inline]
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_std(net::TcpStream::from_raw_socket(socket))
    }
}

#[cfg(windows)]
impl From<OwnedSocket> for TcpStream {
    #[inline]
    fn from(socket: OwnedSocket) -> Self {
        Self::from_std(net::TcpStream::from(socket))
    }
}

#[cfg(not(windows))]
impl AsRawFd for TcpStream {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for TcpStream {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for TcpStream {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.std.as_raw_socket()
    }
}

#[cfg(windows)]
impl AsSocket for TcpStream {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket<'_> {
        self.std.as_socket()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for TcpStream {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.std.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsHandleOrSocket for TcpStream {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.std.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl IntoRawFd for TcpStream {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

#[cfg(not(windows))]
impl From<TcpStream> for OwnedFd {
    #[inline]
    fn from(stream: TcpStream) -> OwnedFd {
        stream.std.into()
    }
}

#[cfg(windows)]
impl IntoRawSocket for TcpStream {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.std.into_raw_socket()
    }
}

#[cfg(windows)]
impl From<TcpStream> for OwnedSocket {
    #[inline]
    fn from(socket: TcpStream) -> OwnedSocket {
        socket.std.into()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for TcpStream {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.std.into_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl From<TcpStream> for OwnedHandleOrSocket {
    #[inline]
    fn from(stream: TcpStream) -> Self {
        stream.std.into()
    }
}

impl Read for TcpStream {
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

impl Read for &TcpStream {
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

impl Write for TcpStream {
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

impl Write for &TcpStream {
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

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
