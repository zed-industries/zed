use crate::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
#[cfg(not(windows))]
use io_extras::os::rustix::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsSocket, BorrowedSocket, OwnedSocket};
use std::time::Duration;
use std::{fmt, io, net};
#[cfg(windows)]
use {
    io_extras::os::windows::{
        AsHandleOrSocket, AsRawHandleOrSocket, BorrowedHandleOrSocket, IntoRawHandleOrSocket,
        OwnedHandleOrSocket, RawHandleOrSocket,
    },
    std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket},
};

/// A UDP socket.
///
/// This corresponds to [`std::net::UdpSocket`].
///
/// This `UdpSocket` has no `bind`, `connect`, or `send_to` methods. To create
/// a `UdpSocket` bound to an address or to send a message to an address, first
/// obtain a [`Pool`] permitting the address, and then call
/// [`Pool::bind_udp_socket`], or [`Pool::connect_udp_socket`], or
/// [`Pool::send_to_udp_socket_addr`].
///
/// [`Pool`]: struct.Pool.html
/// [`Pool::bind_udp_socket`]: struct.Pool.html#method.bind_udp_socket
/// [`Pool::connect_udp_socket`]: struct.Pool.html#method.connect_udp_socket
/// [`Pool::send_to_udp_socket_addr`]: struct.Pool.html#method.send_to_udp_socket_addr
pub struct UdpSocket {
    pub(crate) std: net::UdpSocket,
}

impl UdpSocket {
    /// Constructs a new instance of `Self` from the given
    /// `std::net::UdpSocket`.
    ///
    /// This grants access the resources the `std::net::UdpSocket` instance
    /// already has access to.
    #[inline]
    pub fn from_std(std: net::UdpSocket) -> Self {
        Self { std }
    }

    /// Receives a single datagram message on the socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::recv_from`].
    #[inline]
    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.std.recv_from(buf)
    }

    /// Receives a single datagram message on the socket, without removing it
    /// from the queue.
    ///
    /// This corresponds to [`std::net::UdpSocket::peek_from`].
    #[inline]
    pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.std.peek_from(buf)
    }

    /// Returns the socket address of the remote peer this socket was connected
    /// to.
    ///
    /// This corresponds to [`std::net::UdpSocket::peer_addr`].
    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.std.peer_addr()
    }

    /// Returns the socket address that this socket was created from.
    ///
    /// This corresponds to [`std::net::UdpSocket::local_addr`].
    #[inline]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.std.local_addr()
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::try_clone`].
    #[inline]
    pub fn try_clone(&self) -> io::Result<Self> {
        let udp_socket = self.std.try_clone()?;
        Ok(Self::from_std(udp_socket))
    }

    /// Sets the read timeout to the timeout specified.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_read_timeout`].
    #[inline]
    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.std.set_read_timeout(dur)
    }

    /// Sets the write timeout to the timeout specified.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_write_timeout`].
    #[inline]
    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.std.set_write_timeout(dur)
    }

    /// Returns the read timeout of this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::read_timeout`].
    #[inline]
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.read_timeout()
    }

    /// Returns the write timeout of this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::write_timeout`].
    #[inline]
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.std.write_timeout()
    }

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_broadcast`].
    #[inline]
    pub fn set_broadcast(&self, broadcast: bool) -> io::Result<()> {
        self.std.set_broadcast(broadcast)
    }

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::broadcast`].
    #[inline]
    pub fn broadcast(&self) -> io::Result<bool> {
        self.std.broadcast()
    }

    /// Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_multicast_loop_v4`].
    #[inline]
    pub fn set_multicast_loop_v4(&self, multicast_loop_v4: bool) -> io::Result<()> {
        self.std.set_multicast_loop_v4(multicast_loop_v4)
    }

    /// Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::multicast_loop_v4`].
    #[inline]
    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.std.multicast_loop_v4()
    }

    /// Sets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_multicast_ttl_v4`].
    #[inline]
    pub fn set_multicast_ttl_v4(&self, multicast_ttl_v4: u32) -> io::Result<()> {
        self.std.set_multicast_ttl_v4(multicast_ttl_v4)
    }

    /// Gets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::multicast_ttl_v4`].
    #[inline]
    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.std.multicast_ttl_v4()
    }

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_multicast_loop_v6`].
    #[inline]
    pub fn set_multicast_loop_v6(&self, multicast_loop_v6: bool) -> io::Result<()> {
        self.std.set_multicast_loop_v6(multicast_loop_v6)
    }

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::multicast_loop_v6`].
    #[inline]
    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.std.multicast_loop_v6()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_ttl`].
    #[inline]
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.std.set_ttl(ttl)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::ttl`].
    #[inline]
    pub fn ttl(&self) -> io::Result<u32> {
        self.std.ttl()
    }

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    ///
    /// This corresponds to [`std::net::UdpSocket::join_multicast_v4`].
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        self.std.join_multicast_v4(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    ///
    /// This corresponds to [`std::net::UdpSocket::join_multicast_v6`].
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.std.join_multicast_v6(multiaddr, interface)
    }

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    ///
    /// This corresponds to [`std::net::UdpSocket::leave_multicast_v4`].
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
        self.std.leave_multicast_v4(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    ///
    /// This corresponds to [`std::net::UdpSocket::leave_multicast_v6`].
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.std.leave_multicast_v6(multiaddr, interface)
    }

    /// Gets the value of the `SO_ERROR` option on this socket.
    ///
    /// This corresponds to [`std::net::UdpSocket::take_error`].
    #[inline]
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.std.take_error()
    }

    /// Sends data on the socket to the remote address to which it is
    /// connected.
    ///
    /// This corresponds to [`std::net::UdpSocket::send`].
    #[inline]
    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.std.send(buf)
    }

    /// Receives a single datagram message on the socket from the remote
    /// address to which it is connected.
    ///
    /// This corresponds to [`std::net::UdpSocket::recv`].
    #[inline]
    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.recv(buf)
    }

    /// Receives single datagram on the socket from the remote address to which
    /// it is connected, without removing the message from input queue.
    ///
    /// This corresponds to [`std::net::UdpSocket::peek`].
    #[inline]
    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.std.peek(buf)
    }

    /// Moves this UDP socket into or out of nonblocking mode.
    ///
    /// This corresponds to [`std::net::UdpSocket::set_nonblocking`].
    #[inline]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.std.set_nonblocking(nonblocking)
    }
}

// Safety: `SocketlikeViewType` is implemented for `std`'s socket types.
unsafe impl io_lifetimes::views::SocketlikeViewType for UdpSocket {}

#[cfg(not(windows))]
impl FromRawFd for UdpSocket {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_std(net::UdpSocket::from_raw_fd(fd))
    }
}

#[cfg(not(windows))]
impl From<OwnedFd> for UdpSocket {
    #[inline]
    fn from(fd: OwnedFd) -> Self {
        Self::from_std(net::UdpSocket::from(fd))
    }
}

#[cfg(windows)]
impl FromRawSocket for UdpSocket {
    #[inline]
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        Self::from_std(net::UdpSocket::from_raw_socket(socket))
    }
}

#[cfg(windows)]
impl From<OwnedSocket> for UdpSocket {
    #[inline]
    fn from(socket: OwnedSocket) -> Self {
        Self::from_std(net::UdpSocket::from(socket))
    }
}

#[cfg(not(windows))]
impl AsRawFd for UdpSocket {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.std.as_raw_fd()
    }
}

#[cfg(not(windows))]
impl AsFd for UdpSocket {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.std.as_fd()
    }
}

#[cfg(windows)]
impl AsRawSocket for UdpSocket {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.std.as_raw_socket()
    }
}

#[cfg(windows)]
impl AsSocket for UdpSocket {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket<'_> {
        self.std.as_socket()
    }
}

#[cfg(windows)]
impl AsRawHandleOrSocket for UdpSocket {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        self.std.as_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl AsHandleOrSocket for UdpSocket {
    #[inline]
    fn as_handle_or_socket(&self) -> BorrowedHandleOrSocket<'_> {
        self.std.as_handle_or_socket()
    }
}

#[cfg(not(windows))]
impl IntoRawFd for UdpSocket {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.std.into_raw_fd()
    }
}

#[cfg(not(windows))]
impl From<UdpSocket> for OwnedFd {
    #[inline]
    fn from(socket: UdpSocket) -> OwnedFd {
        socket.std.into()
    }
}

#[cfg(windows)]
impl IntoRawSocket for UdpSocket {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.std.into_raw_socket()
    }
}

#[cfg(windows)]
impl From<UdpSocket> for OwnedSocket {
    #[inline]
    fn from(socket: UdpSocket) -> OwnedSocket {
        socket.std.into()
    }
}

#[cfg(windows)]
impl IntoRawHandleOrSocket for UdpSocket {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        self.std.into_raw_handle_or_socket()
    }
}

#[cfg(windows)]
impl From<UdpSocket> for OwnedHandleOrSocket {
    #[inline]
    fn from(socket: UdpSocket) -> Self {
        socket.std.into()
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.std.fmt(f)
    }
}
