use std::io;
use std::net::SocketAddr;
use std::net::{Ipv4Addr, Ipv6Addr};

use async_io::Async;

use crate::net::ToSocketAddrs;
use crate::utils::Context as _;

/// A UDP socket.
///
/// After creating a `UdpSocket` by [`bind`]ing it to a socket address, data can be [sent to] and
/// [received from] any other socket address.
///
/// As stated in the User Datagram Protocol's specification in [IETF RFC 768], UDP is an unordered,
/// unreliable protocol. Refer to [`TcpListener`] and [`TcpStream`] for async TCP primitives.
///
/// This type is an async version of [`std::net::UdpSocket`].
///
/// [`bind`]: #method.bind
/// [received from]: #method.recv_from
/// [sent to]: #method.send_to
/// [`TcpListener`]: struct.TcpListener.html
/// [`TcpStream`]: struct.TcpStream.html
/// [`std::net`]: https://doc.rust-lang.org/std/net/index.html
/// [IETF RFC 768]: https://tools.ietf.org/html/rfc768
/// [`std::net::UdpSocket`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html
///
/// ## Examples
///
/// ```no_run
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::net::UdpSocket;
///
/// let socket = UdpSocket::bind("127.0.0.1:8080").await?;
/// let mut buf = vec![0u8; 1024];
///
/// loop {
///     let (n, peer) = socket.recv_from(&mut buf).await?;
///     socket.send_to(&buf[..n], &peer).await?;
/// }
/// #
/// # }) }
/// ```
#[derive(Debug)]
pub struct UdpSocket {
    watcher: Async<std::net::UdpSocket>,
}

impl UdpSocket {
    /// Creates a UDP socket from the given address.
    ///
    /// Binding with a port number of 0 will request that the OS assigns a port to this socket. The
    /// port allocated can be queried via the [`local_addr`] method.
    ///
    /// [`local_addr`]: #method.local_addr
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::UdpSocket;
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:0").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn bind<A: ToSocketAddrs>(addrs: A) -> io::Result<UdpSocket> {
        let mut last_err = None;
        let addrs = addrs.to_socket_addrs().await?;

        for addr in addrs {
            match Async::<std::net::UdpSocket>::bind(addr) {
                Ok(socket) => {
                    return Ok(UdpSocket { watcher: socket });
                }
                Err(err) => last_err = Some(err),
            }
        }

        Err(last_err.unwrap_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not resolve to any addresses",
            )
        }))
    }

    /// Returns the peer address that this listener is connected to.
    ///
    /// This can be useful, for example, when connect to port 0 to figure out which port was
    /// actually connected.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::UdpSocket;
    ///
    /// let socket1 = UdpSocket::bind("127.0.0.1:0").await?;
    /// let socket2 = UdpSocket::bind("127.0.0.1:0").await?;
    /// socket1.connect(socket2.local_addr()?).await?;
    /// let addr = socket1.peer_addr()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.watcher
            .get_ref()
            .peer_addr()
            .context(|| String::from("could not get peer address"))
    }

    /// Returns the local address that this listener is bound to.
    ///
    /// This can be useful, for example, when binding to port 0 to figure out which port was
    /// actually bound.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::UdpSocket;
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:0").await?;
    /// let addr = socket.local_addr()?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.watcher
            .get_ref()
            .local_addr()
            .context(|| String::from("could not get local address"))
    }

    /// Sends data on the socket to the given address.
    ///
    /// On success, returns the number of bytes written.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::UdpSocket;
    ///
    /// const THE_MERCHANT_OF_VENICE: &[u8] = b"
    ///     If you prick us, do we not bleed?
    ///     If you tickle us, do we not laugh?
    ///     If you poison us, do we not die?
    ///     And if you wrong us, shall we not revenge?
    /// ";
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:0").await?;
    ///
    /// let addr = "127.0.0.1:7878";
    /// let sent = socket.send_to(THE_MERCHANT_OF_VENICE, &addr).await?;
    /// println!("Sent {} bytes to {}", sent, addr);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addrs: A) -> io::Result<usize> {
        let addr = match addrs.to_socket_addrs().await?.next() {
            Some(addr) => addr,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "no addresses to send data to",
                ));
            }
        };

        self.watcher
            .send_to(buf, addr)
            .await
            .context(|| format!("could not send packet to {}", addr))
    }

    /// Receives data from the socket.
    ///
    /// On success, returns the number of bytes read and the origin.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::UdpSocket;
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:0").await?;
    ///
    /// let mut buf = vec![0; 1024];
    /// let (n, peer) = socket.recv_from(&mut buf).await?;
    /// println!("Received {} bytes from {}", n, peer);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.watcher.recv_from(buf).await
    }

    /// Receives data from socket without removing it from the queue.
    ///
    /// On success, returns the number of bytes peeked and the origin.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::UdpSocket;
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:0").await?;
    ///
    /// let mut buf = vec![0; 1024];
    /// let (n, peer) = socket.peek_from(&mut buf).await?;
    /// println!("Peeked {} bytes from {}", n, peer);
    /// #
    /// # Ok (()) }) }
    /// ```
    pub async fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.watcher.peek_from(buf).await
    }

    /// Connects the UDP socket to a remote address.
    ///
    /// When connected, methods [`send`] and [`recv`] will use the specified address for sending
    /// and receiving messages. Additionally, a filter will be applied to [`recv_from`] so that it
    /// only receives messages from that same address.
    ///
    /// [`send`]: #method.send
    /// [`recv`]: #method.recv
    /// [`recv_from`]: #method.recv_from
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::UdpSocket;
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:0").await?;
    /// socket.connect("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn connect<A: ToSocketAddrs>(&self, addrs: A) -> io::Result<()> {
        let mut last_err = None;
        let addrs = addrs
            .to_socket_addrs()
            .await
            .context(|| String::from("could not resolve addresses"))?;

        for addr in addrs {
            // TODO(stjepang): connect on the blocking pool
            match self.watcher.get_ref().connect(addr) {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err),
            }
        }

        Err(last_err.unwrap_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not resolve to any addresses",
            )
        }))
    }

    /// Sends data on the socket to the remote address to which it is connected.
    ///
    /// The [`connect`] method will connect this socket to a remote address.
    /// This method will fail if the socket is not connected.
    ///
    /// [`connect`]: #method.connect
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::UdpSocket;
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:34254").await?;
    /// socket.connect("127.0.0.1:8080").await?;
    /// let bytes = socket.send(b"Hi there!").await?;
    ///
    /// println!("Sent {} bytes", bytes);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.watcher.send(buf).await
    }

    /// Receives data from the socket.
    ///
    /// On success, returns the number of bytes read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::UdpSocket;
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:0").await?;
    /// socket.connect("127.0.0.1:8080").await?;
    ///
    /// let mut buf = vec![0; 1024];
    /// let n = socket.recv(&mut buf).await?;
    /// println!("Received {} bytes", n);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.watcher.recv(buf).await
    }

    /// Receives data from the socket without removing it from the queue.
    ///
    /// On success, returns the number of bytes peeked.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::net::UdpSocket;
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:0").await?;
    /// socket.connect("127.0.0.1:8080").await?;
    ///
    /// let mut buf = vec![0; 1024];
    /// let n = socket.peek(&mut buf).await?;
    /// println!("Peeked {} bytes", n);
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.watcher.peek(buf).await
    }

    /// Gets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// For more information about this option, see [`set_broadcast`].
    ///
    /// [`set_broadcast`]: #method.set_broadcast
    pub fn broadcast(&self) -> io::Result<bool> {
        self.watcher.get_ref().broadcast()
    }

    /// Sets the value of the `SO_BROADCAST` option for this socket.
    ///
    /// When enabled, this socket is allowed to send packets to a broadcast address.
    pub fn set_broadcast(&self, on: bool) -> io::Result<()> {
        self.watcher.get_ref().set_broadcast(on)
    }

    /// Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// For more information about this option, see [`set_multicast_loop_v4`].
    ///
    /// [`set_multicast_loop_v4`]: #method.set_multicast_loop_v4
    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.watcher.get_ref().multicast_loop_v4()
    }

    /// Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
    ///
    /// If enabled, multicast packets will be looped back to the local socket.
    ///
    /// # Note
    ///
    /// This may not have any affect on IPv6 sockets.
    pub fn set_multicast_loop_v4(&self, on: bool) -> io::Result<()> {
        self.watcher.get_ref().set_multicast_loop_v4(on)
    }

    /// Gets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// For more information about this option, see [`set_multicast_ttl_v4`].
    ///
    /// [`set_multicast_ttl_v4`]: #method.set_multicast_ttl_v4
    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.watcher.get_ref().multicast_ttl_v4()
    }

    /// Sets the value of the `IP_MULTICAST_TTL` option for this socket.
    ///
    /// Indicates the time-to-live value of outgoing multicast packets for this socket. The default
    /// value is 1 which means that multicast packets don't leave the local network unless
    /// explicitly requested.
    ///
    /// # Note
    ///
    /// This may not have any affect on IPv6 sockets.
    pub fn set_multicast_ttl_v4(&self, ttl: u32) -> io::Result<()> {
        self.watcher.get_ref().set_multicast_ttl_v4(ttl)
    }

    /// Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// For more information about this option, see [`set_multicast_loop_v6`].
    ///
    /// [`set_multicast_loop_v6`]: #method.set_multicast_loop_v6
    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.watcher.get_ref().multicast_loop_v6()
    }

    /// Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
    ///
    /// Controls whether this socket sees the multicast packets it sends itself.
    ///
    /// # Note
    ///
    /// This may not have any affect on IPv4 sockets.
    pub fn set_multicast_loop_v6(&self, on: bool) -> io::Result<()> {
        self.watcher.get_ref().set_multicast_loop_v6(on)
    }

    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// For more information about this option, see [`set_ttl`].
    ///
    /// [`set_ttl`]: #method.set_ttl
    pub fn ttl(&self) -> io::Result<u32> {
        self.watcher.get_ref().ttl()
    }

    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This value sets the time-to-live field that is used in every packet sent
    /// from this socket.
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.watcher.get_ref().set_ttl(ttl)
    }

    /// Executes an operation of the `IP_ADD_MEMBERSHIP` type.
    ///
    /// This method specifies a new multicast group for this socket to join. The address must be
    /// a valid multicast address, and `interface` is the address of the local interface with which
    /// the system should join the multicast group. If it's equal to `INADDR_ANY` then an
    /// appropriate interface is chosen by the system.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use std::net::Ipv4Addr;
    ///
    /// use async_std::net::UdpSocket;
    ///
    /// let interface = Ipv4Addr::new(0, 0, 0, 0);
    /// let mdns_addr = Ipv4Addr::new(224, 0, 0, 123);
    ///
    /// let socket = UdpSocket::bind("127.0.0.1:0").await?;
    /// socket.join_multicast_v4(mdns_addr, interface)?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn join_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> io::Result<()> {
        self.watcher
            .get_ref()
            .join_multicast_v4(&multiaddr, &interface)
    }

    /// Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
    ///
    /// This method specifies a new multicast group for this socket to join. The address must be
    /// a valid multicast address, and `interface` is the index of the interface to join/leave (or
    /// 0 to indicate any interface).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use std::net::{Ipv6Addr, SocketAddr};
    ///
    /// use async_std::net::UdpSocket;
    ///
    /// let socket_addr = SocketAddr::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(), 0);
    /// let mdns_addr = Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 0x0123);
    /// let socket = UdpSocket::bind(&socket_addr).await?;
    ///
    /// socket.join_multicast_v6(&mdns_addr, 0)?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.watcher
            .get_ref()
            .join_multicast_v6(multiaddr, interface)
    }

    /// Executes an operation of the `IP_DROP_MEMBERSHIP` type.
    ///
    /// For more information about this option, see [`join_multicast_v4`].
    ///
    /// [`join_multicast_v4`]: #method.join_multicast_v4
    pub fn leave_multicast_v4(&self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> io::Result<()> {
        self.watcher
            .get_ref()
            .leave_multicast_v4(&multiaddr, &interface)
    }

    /// Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
    ///
    /// For more information about this option, see [`join_multicast_v6`].
    ///
    /// [`join_multicast_v6`]: #method.join_multicast_v6
    pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
        self.watcher
            .get_ref()
            .leave_multicast_v6(multiaddr, interface)
    }
}

impl From<std::net::UdpSocket> for UdpSocket {
    /// Converts a `std::net::UdpSocket` into its asynchronous equivalent.
    fn from(socket: std::net::UdpSocket) -> UdpSocket {
        UdpSocket {
            watcher: Async::new(socket).expect("UdpSocket is known to be good"),
        }
    }
}

impl std::convert::TryFrom<UdpSocket> for std::net::UdpSocket {
    type Error = io::Error;
    /// Converts a `UdpSocket` into its synchronous equivalent.
    fn try_from(listener: UdpSocket) -> io::Result<std::net::UdpSocket> {
        let inner = listener.watcher.into_inner()?;
        inner.set_nonblocking(false)?;
        Ok(inner)
    }
}

cfg_unix! {
    use crate::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

    impl AsRawFd for UdpSocket {
        fn as_raw_fd(&self) -> RawFd {
            self.watcher.get_ref().as_raw_fd()
        }
    }

    impl FromRawFd for UdpSocket {
        unsafe fn from_raw_fd(fd: RawFd) -> UdpSocket {
            std::net::UdpSocket::from_raw_fd(fd).into()
        }
    }

    impl IntoRawFd for UdpSocket {
        fn into_raw_fd(self) -> RawFd {
            self.watcher.into_inner().unwrap().into_raw_fd()
        }
    }

    cfg_io_safety! {
        use crate::os::unix::io::{AsFd, BorrowedFd, OwnedFd};

        impl AsFd for UdpSocket {
            fn as_fd(&self) -> BorrowedFd<'_> {
                self.watcher.get_ref().as_fd()
            }
        }

        impl From<OwnedFd> for UdpSocket {
            fn from(fd: OwnedFd) -> UdpSocket {
                std::net::UdpSocket::from(fd).into()
            }
        }

        impl From<UdpSocket> for OwnedFd {
            fn from(stream: UdpSocket) -> OwnedFd {
                stream.watcher.into_inner().unwrap().into()
            }
        }
    }
}

cfg_windows! {
    use crate::os::windows::io::{
        RawSocket, AsRawSocket, IntoRawSocket, FromRawSocket
    };

    impl AsRawSocket for UdpSocket {
        fn as_raw_socket(&self) -> RawSocket {
            self.watcher.get_ref().as_raw_socket()
        }
    }

    impl FromRawSocket for UdpSocket {
        unsafe fn from_raw_socket(handle: RawSocket) -> UdpSocket {
            std::net::UdpSocket::from_raw_socket(handle).into()
        }
    }

    impl IntoRawSocket for UdpSocket {
        fn into_raw_socket(self) -> RawSocket {
            self.watcher.into_inner().unwrap().into_raw_socket()
        }
    }

    cfg_io_safety! {
        use crate::os::windows::io::{AsSocket, BorrowedSocket, OwnedSocket};

        impl AsSocket for UdpSocket {
            fn as_socket(&self) -> BorrowedSocket<'_> {
                self.watcher.get_ref().as_socket()
            }
        }

        impl From<OwnedSocket> for UdpSocket {
            fn from(fd: OwnedSocket) -> UdpSocket {
                std::net::UdpSocket::from(fd).into()
            }
        }

        impl From<UdpSocket> for OwnedSocket {
            fn from(stream: UdpSocket) -> OwnedSocket {
                stream.watcher.into_inner().unwrap().into()
            }
        }
    }
}
