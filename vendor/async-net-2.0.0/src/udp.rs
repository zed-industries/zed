use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
#[cfg(unix)]
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, AsSocket, BorrowedSocket, OwnedSocket, RawSocket};
use std::sync::Arc;

use async_io::Async;

use crate::addr::AsyncToSocketAddrs;

/// A UDP socket.
///
/// After creating a [`UdpSocket`] by [`bind`][`UdpSocket::bind()`]ing it to a socket address, data
/// can be [sent to] and [received from] any other socket address.
///
/// Cloning a [`UdpSocket`] creates another handle to the same socket. The socket will be closed
/// when all handles to it are dropped.
///
/// Although UDP is a connectionless protocol, this implementation provides an interface to set an
/// address where data should be sent and received from. After setting a remote address with
/// [`connect()`][`UdpSocket::connect()`], data can be sent to and received from that address with
/// [`send()`][`UdpSocket::send()`] and [`recv()`][`UdpSocket::recv()`].
///
/// As stated in the User Datagram Protocol's specification in [IETF RFC 768], UDP is an unordered,
/// unreliable protocol. Refer to [`TcpListener`][`super::TcpListener`] and
/// [`TcpStream`][`super::TcpStream`] for TCP primitives.
///
/// [received from]: UdpSocket::recv_from()
/// [sent to]: UdpSocket::send_to()
/// [IETF RFC 768]: https://tools.ietf.org/html/rfc768
///
/// # Examples
///
/// ```no_run
/// use async_net::UdpSocket;
///
/// # futures_lite::future::block_on(async {
/// let socket = UdpSocket::bind("127.0.0.1:8080").await?;
/// let mut buf = vec![0u8; 20];
///
/// loop {
///     // Receive a single datagram message.
///     // If `buf` is too small to hold the entire message, it will be cut off.
///     let (n, addr) = socket.recv_from(&mut buf).await?;
///
///     // Send the message back to the same address that has sent it.
///     socket.send_to(&buf[..n], &addr).await?;
/// }
/// # std::io::Result::Ok(()) });
/// ```
#[derive(Clone, Debug)]
pub struct UdpSocket {
    inner: Arc<Async<std::net::UdpSocket>>,
}

impl UdpSocket {
    fn new(inner: Arc<Async<std::net::UdpSocket>>) -> UdpSocket {
        UdpSocket { inner }
    }

    /// Creates a new [`UdpSocket`] bound to the given address.
    ///
    /// Binding with a port number of 0 will request that the operating system assigns an available
    /// port to this socket. The assigned port can be queried via the
    /// [`local_addr()`][`UdpSocket::local_addr()`] method.
    ///
    /// If `addr` yields multiple addresses, binding will be attempted with each of the addresses
    /// until one succeeds and returns the socket. If none of the addresses succeed in creating a
    /// socket, the error from the last attempt is returned.
    ///
    /// # Examples
    ///
    /// Create a UDP socket bound to `127.0.0.1:3400`:
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:3400").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    ///
    /// Create a UDP socket bound to `127.0.0.1:3400`. If that address is unavailable, then try
    /// binding to `127.0.0.1:3401`:
    ///
    /// ```no_run
    /// use async_net::{SocketAddr, UdpSocket};
    ///
    /// # futures_lite::future::block_on(async {
    /// let addrs = [
    ///     SocketAddr::from(([127, 0, 0, 1], 3400)),
    ///     SocketAddr::from(([127, 0, 0, 1], 3401)),
    /// ];
    /// let socket = UdpSocket::bind(&addrs[..]).await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn bind<A: AsyncToSocketAddrs>(addr: A) -> io::Result<UdpSocket> {
        let mut last_err = None;

        for addr in addr.to_socket_addrs().await? {
            match Async::<std::net::UdpSocket>::bind(addr) {
                Ok(socket) => return Ok(UdpSocket::new(Arc::new(socket))),
                Err(err) => last_err = Some(err),
            }
        }

        Err(last_err.unwrap_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not bind to any of the addresses",
            )
        }))
    }

    /// Returns the local address this socket is bound to.
    ///
    /// This can be useful, for example, when binding to port 0 to figure out which port was
    /// actually bound.
    ///
    /// # Examples
    ///
    /// Bind to port 0 and then see which port was assigned by the operating system:
    ///
    /// ```no_run
    /// use async_net::{SocketAddr, UdpSocket};
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:0").await?;
    /// println!("Bound to {}", socket.local_addr()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.get_ref().local_addr()
    }

    /// Returns the remote address this socket is connected to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.connect("192.168.0.1:41203").await?;
    /// println!("Connected to {}", socket.peer_addr()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.inner.get_ref().peer_addr()
    }

    /// Connects the UDP socket to an address.
    ///
    /// When connected, methods [`send()`][`UdpSocket::send()`] and [`recv()`][`UdpSocket::recv()`]
    /// will use the specified address for sending and receiving messages. Additionally, a filter
    /// will be applied to [`recv_from()`][`UdpSocket::recv_from()`] so that it only receives
    /// messages from that same address.
    ///
    /// If `addr` yields multiple addresses, connecting will be attempted with each of the
    /// addresses until the operating system accepts one. If none of the addresses are accepted,
    /// the error from the last attempt is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:3400").await?;
    /// socket.connect("127.0.0.1:8080").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn connect<A: AsyncToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        let mut last_err = None;

        for addr in addr.to_socket_addrs().await? {
            match self.inner.get_ref().connect(addr) {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err),
            }
        }

        Err(last_err.unwrap_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not connect to any of the addresses",
            )
        }))
    }

    /// Receives a single datagram message.
    ///
    /// On success, returns the number of bytes received and the address message came from.
    ///
    /// This method must be called with a valid byte buffer of sufficient size to hold a message.
    /// If the received message is too long to fit into the buffer, it may be truncated.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    ///
    /// let mut buf = vec![0u8; 1024];
    /// let (n, addr) = socket.recv_from(&mut buf).await?;
    /// println!("Received {} bytes from {}", n, addr);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.inner.recv_from(buf).await
    }

    /// Receives a single datagram message without removing it from the queue.
    ///
    /// On success, returns the number of bytes peeked and the address message came from.
    ///
    /// This method must be called with a valid byte buffer of sufficient size to hold a message.
    /// If the received message is too long to fit into the buffer, it may be truncated.
    ///
    /// Successive calls return the same message. This is accomplished by passing `MSG_PEEK` as a
    /// flag to the underlying `recvfrom` system call.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    ///
    /// let mut buf = vec![0u8; 1024];
    /// let (n, addr) = socket.peek_from(&mut buf).await?;
    /// println!("Peeked {} bytes from {}", n, addr);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.inner.get_ref().peek_from(buf)
    }

    /// Sends data to the given address.
    ///
    /// On success, returns the number of bytes sent.
    ///
    /// If `addr` yields multiple addresses, the message will only be sent to the first address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.send_to(b"hello", "127.0.0.1:4242").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn send_to<A: AsyncToSocketAddrs>(&self, buf: &[u8], addr: A) -> io::Result<usize> {
        let addr = match addr.to_socket_addrs().await?.next() {
            Some(addr) => addr,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "no addresses to send data to",
                ))
            }
        };

        self.inner.send_to(buf, addr).await
    }

    /// Receives a single datagram message from the connected address.
    ///
    /// On success, returns the number of bytes received.
    ///
    /// This method must be called with a valid byte buffer of sufficient size to hold a message.
    /// If the received message is too long to fit into the buffer, it may be truncated.
    ///
    /// The [`connect()`][`UdpSocket::connect()`] method connects this socket to an address. This
    /// method will fail if the socket is not connected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.connect("127.0.0.1:8080").await?;
    ///
    /// let mut buf = vec![0u8; 1024];
    /// let n = socket.recv(&mut buf).await?;
    /// println!("Received {} bytes", n);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.recv(buf).await
    }

    /// Receives a single datagram from the connected address without removing it from the queue.
    ///
    /// On success, returns the number of bytes peeked.
    ///
    /// This method must be called with a valid byte buffer of sufficient size to hold a message.
    /// If the received message is too long to fit into the buffer, it may be truncated.
    ///
    /// Successive calls return the same message. This is accomplished by passing `MSG_PEEK` as a
    /// flag to the underlying `recv` system call.
    ///
    /// The [`connect()`][`UdpSocket::connect()`] method connects this socket to an address. This
    /// method will fail if the socket is not connected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.connect("127.0.0.1:8080").await?;
    ///
    /// let mut buf = vec![0u8; 1024];
    /// let n = socket.peek(&mut buf).await?;
    /// println!("Peeked {} bytes", n);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.peek(buf).await
    }

    /// Sends data to the connected address.
    ///
    /// The [`connect()`][`UdpSocket::connect()`] method connects this socket to an address. This
    /// method will fail if the socket is not connected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.connect("127.0.0.1:8080").await?;
    /// socket.send(b"hello").await?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub async fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.inner.send(buf).await
    }

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// If set to `true`, this socket is allowed to send packets to a broadcast address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// println!("SO_BROADCAST is set to {}", socket.broadcast()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn broadcast(&self) -> io::Result<bool> {
        self.inner.get_ref().broadcast()
    }

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// If set to `true`, this socket is allowed to send packets to a broadcast address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.set_broadcast(true)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn set_broadcast(&self, broadcast: bool) -> io::Result<()> {
        self.inner.get_ref().set_broadcast(broadcast)
    }

    /// Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// If set to `true`, multicast packets will be looped back to the local socket.
    ///
    /// Note that this option may not have any affect on IPv6 sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// println!("IP_MULTICAST_LOOP is set to {}", socket.multicast_loop_v4()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.inner.get_ref().multicast_loop_v4()
    }

    /// Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// If set to `true`, multicast packets will be looped back to the local socket.
    ///
    /// Note that this option may not have any affect on IPv6 sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.set_multicast_loop_v4(true)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn set_multicast_loop_v4(&self, multicast_loop_v4: bool) -> io::Result<()> {
        self.inner
            .get_ref()
            .set_multicast_loop_v4(multicast_loop_v4)
    }

    /// Gets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// Indicates the time-to-live value of outgoing multicast packets for this socket. The default
    /// value is 1, which means that multicast packets don't leave the local network unless
    /// explicitly requested.
    ///
    /// Note that this option may not have any effect on IPv6 sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// println!("IP_MULTICAST_TTL is set to {}", socket.multicast_loop_v4()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.inner.get_ref().multicast_ttl_v4()
    }

    /// Sets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// Indicates the time-to-live value of outgoing multicast packets for this socket. The default
    /// value is 1, which means that multicast packets don't leave the local network unless
    /// explicitly requested.
    ///
    /// Note that this option may not have any effect on IPv6 sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.set_multicast_ttl_v4(10)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn set_multicast_ttl_v4(&self, ttl: u32) -> io::Result<()> {
        self.inner.get_ref().set_multicast_ttl_v4(ttl)
    }

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// Controls whether this socket sees the multicast packets it sends itself.
    ///
    /// Note that this option may not have any effect on IPv4 sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// println!("IPV6_MULTICAST_LOOP is set to {}", socket.multicast_loop_v6()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.inner.get_ref().multicast_loop_v6()
    }

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// Controls whether this socket sees the multicast packets it sends itself.
    ///
    /// Note that this option may not have any effect on IPv4 sockets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.set_multicast_loop_v6(true)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn set_multicast_loop_v6(&self, multicast_loop_v6: bool) -> io::Result<()> {
        self.inner
            .get_ref()
            .set_multicast_loop_v6(multicast_loop_v6)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// This option configures the time-to-live field that is used in every packet sent from this
    /// socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// println!("IP_TTL is set to {}", socket.ttl()?);
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn ttl(&self) -> io::Result<u32> {
        self.inner.get_ref().ttl()
    }

    /// Sets the value of the `IP_TTL` option for this socket.
    ///
    /// This option configures the time-to-live field that is used in every packet sent from this
    /// socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use async_net::UdpSocket;
    ///
    /// # futures_lite::future::block_on(async {
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.set_ttl(100)?;
    /// # std::io::Result::Ok(()) });
    /// ```
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.inner.get_ref().set_ttl(ttl)
    }

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    ///
    /// This method specifies a new multicast group for this socket to join. Argument `multiaddr`
    /// must be a valid multicast address, and `interface` is the address of the local interface
    /// with which the system should join the multicast group. If it's equal to `INADDR_ANY` then
    /// an appropriate interface is chosen by the system.
    pub fn join_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> io::Result<()> {
        self.inner
            .get_ref()
            .join_multicast_v4(&multiaddr, &interface)
    }

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    ///
    /// This method leaves a multicast group. Argument `multiaddr` must be a valid multicast
    /// address, and `interface` is the index of the interface to leave.
    pub fn leave_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> io::Result<()> {
        self.inner
            .get_ref()
            .leave_multicast_v4(&multiaddr, &interface)
    }

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    ///
    /// This method specifies a new multicast group for this socket to join. Argument `multiaddr`
    /// must be a valid multicast address, and `interface` is the index of the interface to join
    /// (or 0 to indicate any interface).
    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.inner.get_ref().join_multicast_v6(multiaddr, interface)
    }

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    ///
    /// This method leaves a multicast group. Argument `multiaddr` must be a valid multicast
    /// address, and `interface` is the index of the interface to leave.
    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.inner
            .get_ref()
            .leave_multicast_v6(multiaddr, interface)
    }
}

impl From<Async<std::net::UdpSocket>> for UdpSocket {
    fn from(socket: Async<std::net::UdpSocket>) -> UdpSocket {
        UdpSocket::new(Arc::new(socket))
    }
}

impl TryFrom<std::net::UdpSocket> for UdpSocket {
    type Error = io::Error;

    fn try_from(socket: std::net::UdpSocket) -> io::Result<UdpSocket> {
        Ok(UdpSocket::new(Arc::new(Async::new(socket)?)))
    }
}

impl From<UdpSocket> for Arc<Async<std::net::UdpSocket>> {
    fn from(val: UdpSocket) -> Self {
        val.inner
    }
}

#[cfg(unix)]
impl AsRawFd for UdpSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

#[cfg(unix)]
impl AsFd for UdpSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.inner.get_ref().as_fd()
    }
}

#[cfg(unix)]
impl TryFrom<OwnedFd> for UdpSocket {
    type Error = io::Error;

    fn try_from(value: OwnedFd) -> Result<Self, Self::Error> {
        Self::try_from(std::net::UdpSocket::from(value))
    }
}

#[cfg(windows)]
impl AsRawSocket for UdpSocket {
    fn as_raw_socket(&self) -> RawSocket {
        self.inner.as_raw_socket()
    }
}

#[cfg(windows)]
impl AsSocket for UdpSocket {
    fn as_socket(&self) -> BorrowedSocket<'_> {
        self.inner.get_ref().as_socket()
    }
}

#[cfg(windows)]
impl TryFrom<OwnedSocket> for UdpSocket {
    type Error = io::Error;

    fn try_from(value: OwnedSocket) -> Result<Self, Self::Error> {
        Self::try_from(std::net::UdpSocket::from(value))
    }
}
