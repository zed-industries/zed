//! Extension traits for `TcpListener`, `UdpSocket`, and `Pool`.
//!
//! cap-std's [`TcpListener`], following the Rust standard library
//! `TcpListener`, combines the `socket`, `bind`, `listen`, and `connect`
//! operations of the POSIX socket API into a single `bind` or `connect`
//! operation. In some use cases, it's desirable to perform the steps
//! separately.
//!
//! This API adds extension traits to cap-std's `TcpListener`, `UdpSocket`,
//! and `Pool` which support the following sequence for accepting incoming
//! connections:
//!
//!  - [`TcpListenerExt::new`] performs a `socket` and returns a new
//!    `TcpListener` that is not yet bound.
//!  - [`Pool::bind_existing_tcp_listener`] performs a `bind`, checking that
//!    the address is in the `Pool`.
//!  - [`TcpListenerExt::listen`] performs a `listen`.
//!  - Then, the regular [`TcpListener::accept`] may be used to accept new
//!    connections. Alternatively, [`TcpListener::accept_with`] may be used.
//!
//! and the following sequence for initiating outgoing connections:
//!
//!  - [`TcpListenerExt::new`] performs a `socket` and returns a new
//!    `TcpListener` that is not yet connected.
//!  - [`Pool::connect_into_tcp_stream`] performs a `connect`, checking that
//!    the address is in the `Pool`.
//!
//! [`TcpListenerExt::new`] and [`TcpListener::accept_with`] additionally
//! have [`Blocking`] arguments for requesting non-blocking operation.
//!
//! Similar API adaptations are available for UDP sockets as well.

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.svg"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/bytecodealliance/cap-std/main/media/cap-std.ico"
)]

use cap_primitives::net::no_socket_addrs;
use cap_std::net::{IpAddr, Pool, SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use rustix::fd::OwnedFd;
use std::io;

/// Address families supported by [`TcpListenerExt::new`] and
/// [`UdpSocketExt::new`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AddressFamily {
    /// IPv4
    Ipv4,

    /// IPv6
    Ipv6,
}

impl AddressFamily {
    /// Return the `AddressFamily` of an IP address.
    pub fn of_ip_addr(ip_addr: IpAddr) -> Self {
        match ip_addr {
            IpAddr::V4(_) => AddressFamily::Ipv4,
            IpAddr::V6(_) => AddressFamily::Ipv6,
        }
    }

    /// Return the `AddressFamily` of a socket address.
    pub fn of_socket_addr(socket_addr: SocketAddr) -> Self {
        match socket_addr {
            SocketAddr::V4(_) => AddressFamily::Ipv4,
            SocketAddr::V6(_) => AddressFamily::Ipv6,
        }
    }
}

impl From<AddressFamily> for rustix::net::AddressFamily {
    fn from(address_family: AddressFamily) -> Self {
        match address_family {
            AddressFamily::Ipv4 => rustix::net::AddressFamily::INET,
            AddressFamily::Ipv6 => rustix::net::AddressFamily::INET6,
        }
    }
}

/// Select blocking or non-blocking mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Blocking {
    /// Non-blocking
    No,

    /// Blocking
    Yes,
}

/// A trait for extending `TcpListener` types.
pub trait TcpListenerExt: private::Sealed + Sized {
    /// Creates a new TCP socket with the given address family.
    ///
    /// The created socket is not bound or connected to any address and may be
    /// used for either listening or connecting. Use
    /// [`PoolExt::bind_existing_tcp_listener`] to bind it in preparation for
    /// listening, or [`PoolExt::connect_into_tcp_stream`] to initiate a
    /// connection.
    ///
    /// This is similar to [`Pool::bind_tcp_listener`] in that it creates a TCP
    /// socket, however it does not perform the `bind` or `listen` steps. And,
    /// it has a `blocking` argument to select blocking or non-blocking mode
    /// for the created socket.
    ///
    /// And it's similar to [`Pool::connect_tcp_stream`] in that it creates a
    /// TCP socket, however it does not perform the `connect` step. And, it has
    /// a `blocking` argument to select blocking or non-blocking mode for the
    /// created socket.
    fn new(address_family: AddressFamily, blocking: Blocking) -> io::Result<Self>;

    /// Enble listening in a `TcpListener`.
    ///
    /// A newly-created [`TcpListener`] created with [`TcpListenerExt::new`]
    /// and bound with [`PoolExt::bind_existing_tcp_listener`] is not yet
    /// listening; this function enables listening. After this, the listener
    /// may accept new connections with [`accept`] or [`accept_with`].
    ///
    /// This is similar to [`Pool::bind_tcp_listener`] in that it performs the
    /// `listen` step, however it does not create the socket itself, or bind
    /// it.
    ///
    /// The `backlog` argument specifies an optional hint to the implementation
    /// about how many connections can be waiting before new connections are
    /// refused or ignored.
    ///
    /// [`accept`]: TcpListener::accept
    /// [`accept_with`]: TcpListenerExt::accept_with
    fn listen(&self, backlog: Option<i32>) -> io::Result<()>;

    /// Similar to [`accept`], but the resulting TCP connection are optionally
    /// set to non-blocking mode.
    ///
    /// The `accept` call itself may still block, if the socket is in blocking
    /// mode.
    ///
    /// [`accept`]: TcpListener::accept
    fn accept_with(&self, blocking: Blocking) -> io::Result<(TcpStream, SocketAddr)>;
}

impl TcpListenerExt for TcpListener {
    fn new(address_family: AddressFamily, blocking: Blocking) -> io::Result<Self> {
        socket(address_family, blocking, rustix::net::SocketType::STREAM).map(Self::from)
    }

    fn listen(&self, backlog: Option<i32>) -> io::Result<()> {
        let backlog = backlog.unwrap_or_else(default_backlog);

        Ok(rustix::net::listen(self, backlog)?)
    }

    fn accept_with(&self, blocking: Blocking) -> io::Result<(TcpStream, SocketAddr)> {
        let (stream, addr) = rustix::net::acceptfrom_with(self, socket_flags(blocking))?;
        set_socket_flags(&stream, blocking)?;

        // We know have a TCP socket, so we know we'll get an IP address.
        let addr = SocketAddr::try_from(addr.unwrap()).unwrap();

        Ok((TcpStream::from(stream), addr))
    }
}

/// A trait for extending `UdpSocket` types.
pub trait UdpSocketExt: private::Sealed + Sized {
    /// Creates a new `UdpSocket` with the given address family.
    ///
    /// The created socket is initially not bound or connected to any address.
    /// Use [`PoolExt::bind_existing_udp_socket`] to bind it, or
    /// [`PoolExt::connect_existing_udp_socket`] to initiate a connection.
    ///
    /// This is similar to [`Pool::bind_udp_socket`] in that it creates a UDP
    /// socket, however it does not perform the `bind`. And, it has a
    /// `blocking` argument to select blocking or non-blocking mode for the
    /// created socket.
    ///
    /// And it's similar to [`Pool::connect_udp_socket`] in that it creates a
    /// UDP socket, however it does not perform the `connect` step. And, it has
    /// a `blocking` argument to select blocking or non-blocking mode for the
    /// created socket.
    fn new(address_family: AddressFamily, blocking: Blocking) -> io::Result<Self>;
}

impl UdpSocketExt for UdpSocket {
    fn new(address_family: AddressFamily, blocking: Blocking) -> io::Result<Self> {
        socket(address_family, blocking, rustix::net::SocketType::DGRAM).map(Self::from)
    }
}

/// A trait for extending `Pool` types.
///
/// These functions have a `ToSocketAddrs` argument, which can return either
/// IPv4 or IPv6 addresses, however they also require the socket to be created
/// with a specific address family up front. Consequently, it's recommended to
/// do address resolution outside of this API and just pass resolved
/// `SocketAddr`s in.
pub trait PoolExt: private::Sealed {
    /// Bind a [`TcpListener`] to the specified address.
    ///
    /// A newly-created `TcpListener` created with [`TcpListenerExt::new`]
    /// has not been bound yet; this function binds it. Before it can accept
    /// connections, it must be marked for listening with
    /// [`TcpListenerExt::listen`].
    ///
    /// This is similar to [`Pool::bind_tcp_listener`] in that it binds a TCP
    /// socket, however it does not create the socket itself, or perform the
    /// `listen` step.
    ///
    /// This function ensures that the address to be bound is permitted by the
    /// pool, and performs the bind. To perform these steps separately, create
    /// a [`TcpBinder`] with [`Self::tcp_binder`] and use
    /// [`TcpBinder::bind_existing_tcp_listener`].
    fn bind_existing_tcp_listener<A: ToSocketAddrs>(
        &self,
        listener: &TcpListener,
        addrs: A,
    ) -> io::Result<()>;

    /// Bind a [`UdpSocket`] to the specified address.
    ///
    /// A newly-created `UdpSocket` created with [`UdpSocketExt::new`] has not
    /// been bound yet; this function binds it.
    ///
    /// This is similar to [`Pool::bind_udp_socket`] in that it binds a UDP
    /// socket, however it does not create the socket itself.
    ///
    /// This function ensures that the address to be bound is permitted by the
    /// pool, and performs the bind. To perform these steps separately, create
    /// a [`UdpBinder`] with [`Self::udp_binder`] and use
    /// [`UdpBinder::bind_existing_udp_socket`].
    fn bind_existing_udp_socket<A: ToSocketAddrs>(
        &self,
        socket: &UdpSocket,
        addrs: A,
    ) -> io::Result<()>;

    /// Initiate a TCP connection, converting a [`TcpListener`] to a
    /// [`TcpStream`].
    ///
    /// This is simlar to [`Pool::connect_tcp_stream`] in that it performs a
    /// TCP connection, but instead of creating a new socket itself it takes a
    /// [`TcpListener`], such as one created with [`TcpListenerExt::new`].
    ///
    /// Despite the name, this function uses the `TcpListener` type as a
    /// generic socket container.
    ///
    /// This function ensures that the address to connect to is permitted by
    /// the pool, and performs the connect. To perform these steps separately,
    /// create a [`TcpConnecter`] with [`Self::tcp_connecter`] and use
    /// [`TcpConnecter::connect_into_tcp_stream`].
    fn connect_into_tcp_stream<A: ToSocketAddrs>(
        &self,
        socket: TcpListener,
        addrs: A,
    ) -> io::Result<TcpStream>;

    /// Initiate a TCP connection on a socket.
    ///
    /// This is simlar to [`Self::connect_into_tcp_stream`], however instead of
    /// converting a `TcpListener` to a `TcpStream`, it leaves fd in the
    /// existing `TcpListener`.
    ///
    /// This function ensures that the address to connect to is permitted by
    /// the pool, and performs the connect. To perform these steps separately,
    /// create a [`TcpConnecter`] with [`Self::tcp_connecter`] and use
    /// [`TcpConnecter::connect_existing_tcp_listener`].
    fn connect_existing_tcp_listener<A: ToSocketAddrs>(
        &self,
        socket: &TcpListener,
        addrs: A,
    ) -> io::Result<()>;

    /// Initiate a UDP connection.
    ///
    /// This is simlar to [`Pool::connect_udp_socket`] in that it performs a
    /// UDP connection, but instead of creating a new socket itself it takes a
    /// [`UdpSocket`], such as one created with [`UdpSocketExt::new`].
    ///
    /// This function ensures that the address to connect to is permitted by
    /// the pool, and performs the connect. To perform these steps separately,
    /// create a [`UdpConnecter`] with [`Self::udp_connecter`] and use
    /// [`UdpConnecter::connect_existing_udp_socket`].
    fn connect_existing_udp_socket<A: ToSocketAddrs>(
        &self,
        socket: &UdpSocket,
        addrs: A,
    ) -> io::Result<()>;

    /// Create a TCP binder.
    ///
    /// This is an alternative to [`Self::bind_existing_tcp_listener`]. It
    /// checks that all the addresses in `addrs` are permitted for TCP binding
    /// up front, and then records them in a [`TcpBinder`] which can then be
    /// used to make repeated [`TcpBinder::bind_existing_tcp_listener`] calls.
    fn tcp_binder<A: ToSocketAddrs>(&self, addrs: A) -> io::Result<TcpBinder>;

    /// Create a UDP binder.
    ///
    /// This is an alternative to [`Self::bind_existing_udp_socket`]. It checks
    /// that all the addresses in `addrs` are permitted for UDP binding up
    /// front, and then records them in a [`UdpBinder`] which can then be used
    /// to make repeated [`UdpBinder::bind_existing_udp_socket`] calls.
    fn udp_binder<A: ToSocketAddrs>(&self, addrs: A) -> io::Result<UdpBinder>;

    /// Create a TCP connecter.
    ///
    /// This is an alternative to [`Self::connect_into_tcp_stream`] and
    /// [`Self::connect_existing_tcp_listener`]. It checks that all the
    /// addresses in `addrs` are permitted for TCP connecting up front, and
    /// then records them in a [`TcpConnecter`] which can then be used to make
    /// repeated [`TcpConnecter::connect_into_tcp_stream`] and
    /// [`TcpConnecter::connect_existing_tcp_listener`] calls.
    fn tcp_connecter<A: ToSocketAddrs>(&self, addrs: A) -> io::Result<TcpConnecter>;

    /// Create a UDP connecter.
    ///
    /// This is an alternative to [`Self::connect_existing_udp_socket`]. It
    /// checks that all the addresses in `addrs` are permitted for UDP
    /// connecting up front, and then records them in a [`UdpConnecter`] which
    /// can then be used to make repeated
    /// [`UdpConnecter::connect_existing_udp_socket`] calls.
    fn udp_connecter<A: ToSocketAddrs>(&self, addrs: A) -> io::Result<UdpConnecter>;
}

impl PoolExt for Pool {
    fn bind_existing_tcp_listener<A: ToSocketAddrs>(
        &self,
        listener: &TcpListener,
        addrs: A,
    ) -> io::Result<()> {
        let addrs = addrs.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self._pool().check_addr(&addr)?;

            set_reuseaddr(listener)?;

            match rustix::net::bind(listener, &addr) {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err.into()),
            }
        }
        match last_err {
            Some(err) => Err(err),
            None => Err(no_socket_addrs()),
        }
    }

    fn bind_existing_udp_socket<A: ToSocketAddrs>(
        &self,
        socket: &UdpSocket,
        addrs: A,
    ) -> io::Result<()> {
        let addrs = addrs.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self._pool().check_addr(&addr)?;

            match rustix::net::bind(socket, &addr) {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err),
            }
        }
        match last_err {
            Some(err) => Err(err.into()),
            None => Err(no_socket_addrs()),
        }
    }

    fn connect_into_tcp_stream<A: ToSocketAddrs>(
        &self,
        socket: TcpListener,
        addrs: A,
    ) -> io::Result<TcpStream> {
        self.connect_existing_tcp_listener(&socket, addrs)?;
        Ok(TcpStream::from(OwnedFd::from(socket)))
    }

    fn connect_existing_tcp_listener<A: ToSocketAddrs>(
        &self,
        socket: &TcpListener,
        addrs: A,
    ) -> io::Result<()> {
        let addrs = addrs.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self._pool().check_addr(&addr)?;

            match rustix::net::connect(socket, &addr) {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err),
            }
        }
        match last_err {
            Some(err) => Err(err.into()),
            None => Err(no_socket_addrs()),
        }
    }

    fn connect_existing_udp_socket<A: ToSocketAddrs>(
        &self,
        socket: &UdpSocket,
        addrs: A,
    ) -> io::Result<()> {
        let addrs = addrs.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            self._pool().check_addr(&addr)?;

            match rustix::net::connect(socket, &addr) {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err),
            }
        }
        match last_err {
            Some(err) => Err(err.into()),
            None => Err(no_socket_addrs()),
        }
    }

    fn tcp_binder<A: ToSocketAddrs>(&self, addrs: A) -> io::Result<TcpBinder> {
        Ok(TcpBinder(check_addrs(self._pool(), addrs)?))
    }

    fn udp_binder<A: ToSocketAddrs>(&self, addrs: A) -> io::Result<UdpBinder> {
        Ok(UdpBinder(check_addrs(self._pool(), addrs)?))
    }

    fn tcp_connecter<A: ToSocketAddrs>(&self, addrs: A) -> io::Result<TcpConnecter> {
        Ok(TcpConnecter(check_addrs(self._pool(), addrs)?))
    }

    fn udp_connecter<A: ToSocketAddrs>(&self, addrs: A) -> io::Result<UdpConnecter> {
        Ok(UdpConnecter(check_addrs(self._pool(), addrs)?))
    }
}

/// Check all the addresses in `addrs` and return a new list of them.
fn check_addrs<A: ToSocketAddrs>(
    pool: &cap_primitives::net::Pool,
    addrs: A,
) -> io::Result<smallvec::SmallVec<[SocketAddr; 1]>> {
    let mut checked = smallvec::SmallVec::new();
    for addr in addrs.to_socket_addrs()? {
        pool.check_addr(&addr)?;
        checked.push(addr);
    }
    Ok(checked)
}

/// A utility for binding TCP listeners.
///
/// See [`PoolExt::tcp_binder`] for details.
pub struct TcpBinder(smallvec::SmallVec<[SocketAddr; 1]>);

impl TcpBinder {
    /// Bind a [`TcpListener`].
    ///
    /// A newly-created `TcpListener` created with [`TcpListenerExt::new`]
    /// has not been bound yet; this function binds it. Before it can accept
    /// connections, it must be marked for listening with
    /// [`TcpListenerExt::listen`].
    ///
    /// This is similar to [`Pool::bind_tcp_listener`] in that it binds a TCP
    /// socket, however it does not create the socket itself, or perform the
    /// `listen` step.
    ///
    /// This is similar to [`PoolExt::bind_existing_tcp_listener`] except that
    /// it uses a `TcpBinder` which contains addresses that have already been
    /// checked against a `Pool`.
    pub fn bind_existing_tcp_listener(&self, listener: &TcpListener) -> io::Result<()> {
        let mut last_err = None;
        for addr in &self.0 {
            set_reuseaddr(listener)?;

            match rustix::net::bind(listener, addr) {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err.into()),
            }
        }
        match last_err {
            Some(err) => Err(err),
            None => Err(no_socket_addrs()),
        }
    }
}

/// A utility for binding UDP sockets.
///
/// See [`PoolExt::udp_binder`] for details.
pub struct UdpBinder(smallvec::SmallVec<[SocketAddr; 1]>);

impl UdpBinder {
    /// Bind a [`UdpSocket`] to the specified address.
    ///
    /// A newly-created `UdpSocket` created with [`UdpSocketExt::new`] has not
    /// been bound yet; this function binds it.
    ///
    /// This is similar to [`Pool::bind_udp_socket`] in that it binds a UDP
    /// socket, however it does not create the socket itself.
    ///
    /// This is similar to [`PoolExt::bind_existing_udp_socket`] except that
    /// it uses a `UdpBinder` which contains addresses that have already been
    /// checked against a `Pool`.
    pub fn bind_existing_udp_socket(&self, socket: &UdpSocket) -> io::Result<()> {
        let mut last_err = None;
        for addr in &self.0 {
            match rustix::net::bind(socket, addr) {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err.into()),
            }
        }
        match last_err {
            Some(err) => Err(err),
            None => Err(no_socket_addrs()),
        }
    }
}

/// A utility for making TCP connections.
///
/// See [`PoolExt::tcp_connecter`] for details.
pub struct TcpConnecter(smallvec::SmallVec<[SocketAddr; 1]>);

impl TcpConnecter {
    /// Initiate a TCP connection, converting a [`TcpListener`] to a
    /// [`TcpStream`].
    ///
    /// This is simlar to [`Pool::connect_tcp_stream`] in that it performs a
    /// TCP connection, but instead of creating a new socket itself it takes a
    /// [`TcpListener`], such as one created with [`TcpListenerExt::new`].
    ///
    /// Despite the name, this function uses the `TcpListener` type as a
    /// generic socket container.
    ///
    /// This is similar to [`PoolExt::connect_into_tcp_stream`] except that
    /// it uses a `TcpConnecter` which contains addresses that have already
    /// been checked against a `Pool`.
    pub fn connect_into_tcp_stream(&self, socket: TcpListener) -> io::Result<TcpStream> {
        self.connect_existing_tcp_listener(&socket)?;
        Ok(TcpStream::from(OwnedFd::from(socket)))
    }

    /// Initiate a TCP connection on a socket.
    ///
    /// This is simlar to [`Pool::connect_into_tcp_stream`], however instead of
    /// converting a `TcpListener` to a `TcpStream`, it leaves fd in the
    /// existing `TcpListener`.
    ///
    /// This is similar to [`PoolExt::connect_existing_tcp_listener`] except
    /// that it uses a `TcpConnecter` which contains addresses that have
    /// already been checked against a `Pool`.
    pub fn connect_existing_tcp_listener(&self, socket: &TcpListener) -> io::Result<()> {
        let mut last_err = None;
        for addr in &self.0 {
            match rustix::net::connect(socket, addr) {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err),
            }
        }
        match last_err {
            Some(err) => Err(err.into()),
            None => Err(no_socket_addrs()),
        }
    }
}

/// A utility for making UDP connections.
///
/// See [`PoolExt::udp_connecter`] for details.
pub struct UdpConnecter(smallvec::SmallVec<[SocketAddr; 1]>);

impl UdpConnecter {
    /// Initiate a UDP connection.
    ///
    /// This is simlar to [`Pool::connect_udp_socket`] in that it performs a
    /// UDP connection, but instead of creating a new socket itself it takes a
    /// [`UdpSocket`], such as one created with [`UdpSocketExt::new`].
    ///
    /// This is similar to [`PoolExt::connect_existing_udp_socket`] except that
    /// it uses a `UdpConnecter` which contains addresses that have already
    /// been checked against a `Pool`.
    pub fn connect_existing_udp_socket(&self, socket: &UdpSocket) -> io::Result<()> {
        let mut last_err = None;
        for addr in &self.0 {
            match rustix::net::connect(socket, addr) {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err),
            }
        }
        match last_err {
            Some(err) => Err(err.into()),
            None => Err(no_socket_addrs()),
        }
    }
}

fn socket(
    address_family: AddressFamily,
    blocking: Blocking,
    socket_type: rustix::net::SocketType,
) -> io::Result<OwnedFd> {
    // The Rust standard library has code to call `WSAStartup`, which is needed
    // on Windows before we do any other Winsock2 calls, so just make a useless
    // API call once.
    #[cfg(windows)]
    {
        use std::sync::Once;
        static START: Once = Once::new();
        START.call_once(|| {
            std::net::TcpStream::connect(std::net::SocketAddrV4::new(
                std::net::Ipv4Addr::UNSPECIFIED,
                0,
            ))
            .unwrap_err();
        });
    }

    // Create the socket, using the desired flags if we can.
    let socket = rustix::net::socket_with(
        address_family.into(),
        socket_type,
        socket_flags(blocking),
        None,
    )?;

    // Set the desired flags if we couldn't set them at creation.
    set_socket_flags(&socket, blocking)?;

    Ok(socket)
}

/// Compute flags to pass to socket calls.
fn socket_flags(blocking: Blocking) -> rustix::net::SocketFlags {
    let _ = blocking;

    #[allow(unused_mut)]
    let mut socket_flags = rustix::net::SocketFlags::empty();

    // On platforms which do support `SOCK_CLOEXEC`, use it.
    #[cfg(not(any(
        windows,
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "visionos",
        target_os = "haiku"
    )))]
    {
        socket_flags |= rustix::net::SocketFlags::CLOEXEC;
    }

    // On platforms which do support `SOCK_NONBLOCK`, use it.
    #[cfg(not(any(
        windows,
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "visionos",
        target_os = "haiku"
    )))]
    match blocking {
        Blocking::Yes => (),
        Blocking::No => socket_flags |= rustix::net::SocketFlags::NONBLOCK,
    }

    socket_flags
}

/// On platforms which don't support `SOCK_CLOEXEC` or `SOCK_NONBLOCK, set them
/// after creating the socket.
fn set_socket_flags(fd: &OwnedFd, blocking: Blocking) -> io::Result<()> {
    let _ = fd;
    let _ = blocking;

    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "visionos",
    ))]
    {
        rustix::io::ioctl_fioclex(fd)?;
    }

    #[cfg(any(
        windows,
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "visionos"
    ))]
    match blocking {
        Blocking::Yes => (),
        Blocking::No => rustix::io::ioctl_fionbio(fd, true)?,
    }

    #[cfg(target_os = "haiku")]
    {
        let mut flags = rustix::fs::fcntl_getfd(fd)?;
        flags |= rustix::fs::OFlags::CLOEXEC;
        match blocking {
            Blocking::Yes => (),
            Blocking::No => flags |= rustix::fs::OFlags::NONBLOCK,
        }
        rustix::fs::fcntl_setfd(fd, flags)?;
    }

    Ok(())
}

/// On platforms where it's desirable, set the `SO_REUSEADDR` option.
fn set_reuseaddr(listener: &TcpListener) -> io::Result<()> {
    let _ = listener;

    // The following logic is from
    // <https://github.com/rust-lang/rust/blob/master/library/std/src/sys_common/net.rs>
    // at revision defa2456246a8272ceace9c1cdccdf2e4c36175e.

    // On platforms with Berkeley-derived sockets, this allows to quickly
    // rebind a socket, without needing to wait for the OS to clean up the
    // previous one.
    //
    // On Windows, this allows rebinding sockets which are actively in use,
    // which allows “socket hijacking”, so we explicitly don't set it here.
    // https://docs.microsoft.com/en-us/windows/win32/winsock/using-so-reuseaddr-and-so-exclusiveaddruse
    #[cfg(not(windows))]
    rustix::net::sockopt::set_socket_reuseaddr(listener, true)?;

    Ok(())
}

/// Determine the platform-specific default backlog value.
fn default_backlog() -> i32 {
    // The following logic is from
    // <https://github.com/rust-lang/rust/blob/master/library/std/src/sys_common/net.rs>
    // at revision defa2456246a8272ceace9c1cdccdf2e4c36175e.

    // The 3DS doesn't support a big connection backlog. Sometimes
    // it allows up to about 37, but other times it doesn't even
    // accept 32. There may be a global limitation causing this.
    #[cfg(target_os = "horizon")]
    let backlog = 20;

    // The default for all other platforms
    #[cfg(not(target_os = "horizon"))]
    let backlog = 128;

    backlog
}

/// Seal the public traits for [future-proofing].
///
/// [future-proofing]: https://rust-lang.github.io/api-guidelines/future-proofing.html
mod private {
    pub trait Sealed {}
    impl Sealed for super::TcpListener {}
    impl Sealed for super::UdpSocket {}
    impl Sealed for super::Pool {}
}
