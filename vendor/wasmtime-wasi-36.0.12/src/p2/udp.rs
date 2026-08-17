use crate::runtime::with_ambient_tokio_runtime;
use crate::sockets::util::udp_socket;
use crate::sockets::{SocketAddrCheck, SocketAddressFamily};
use async_trait::async_trait;
use cap_net_ext::AddressFamily;
use io_lifetimes::raw::{FromRawSocketlike, IntoRawSocketlike};
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use wasmtime_wasi_io::poll::Pollable;

/// The state of a UDP socket.
///
/// This represents the various states a socket can be in during the
/// activities of binding, and connecting.
pub(crate) enum UdpState {
    /// The initial state for a newly-created socket.
    Default,

    /// Binding started via `start_bind`.
    BindStarted,

    /// Binding finished via `finish_bind`. The socket has an address but
    /// is not yet listening for connections.
    Bound,

    /// The socket is "connected" to a peer address.
    Connected,
}

/// A host UDP socket, plus associated bookkeeping.
///
/// The inner state is wrapped in an Arc because the same underlying socket is
/// used for implementing the stream types.
pub struct UdpSocket {
    /// The part of a `UdpSocket` which is reference-counted so that we
    /// can pass it to async tasks.
    pub(crate) inner: Arc<tokio::net::UdpSocket>,

    /// The current state in the bind/connect progression.
    pub(crate) udp_state: UdpState,

    /// Socket address family.
    pub(crate) family: SocketAddressFamily,

    /// The check of allowed addresses
    pub(crate) socket_addr_check: Option<SocketAddrCheck>,
}

#[async_trait]
impl Pollable for UdpSocket {
    async fn ready(&mut self) {
        // None of the socket-level operations block natively
    }
}

impl UdpSocket {
    /// Create a new socket in the given family.
    pub fn new(family: AddressFamily) -> io::Result<Self> {
        // Create a new host socket and set it to non-blocking, which is needed
        // by our async implementation.
        let fd = udp_socket(family)?;

        let socket_address_family = match family {
            AddressFamily::Ipv4 => SocketAddressFamily::Ipv4,
            AddressFamily::Ipv6 => {
                rustix::net::sockopt::set_ipv6_v6only(&fd, true)?;
                SocketAddressFamily::Ipv6
            }
        };

        let socket = Self::setup_tokio_udp_socket(fd.into())?;

        Ok(UdpSocket {
            inner: Arc::new(socket),
            udp_state: UdpState::Default,
            family: socket_address_family,
            socket_addr_check: None,
        })
    }

    fn setup_tokio_udp_socket(fd: rustix::fd::OwnedFd) -> io::Result<tokio::net::UdpSocket> {
        let std_socket =
            unsafe { std::net::UdpSocket::from_raw_socketlike(fd.into_raw_socketlike()) };
        with_ambient_tokio_runtime(|| tokio::net::UdpSocket::try_from(std_socket))
    }

    pub fn udp_socket(&self) -> &tokio::net::UdpSocket {
        &self.inner
    }
}

pub struct IncomingDatagramStream {
    pub(crate) inner: Arc<tokio::net::UdpSocket>,

    /// If this has a value, the stream is "connected".
    pub(crate) remote_address: Option<SocketAddr>,
}

pub struct OutgoingDatagramStream {
    pub(crate) inner: Arc<tokio::net::UdpSocket>,

    /// If this has a value, the stream is "connected".
    pub(crate) remote_address: Option<SocketAddr>,

    /// Socket address family.
    pub(crate) family: SocketAddressFamily,

    pub(crate) send_state: SendState,

    /// The check of allowed addresses
    pub(crate) socket_addr_check: Option<SocketAddrCheck>,
}

pub(crate) enum SendState {
    /// Waiting for the API consumer to call `check-send`.
    Idle,

    /// Ready to send up to x datagrams.
    Permitted(usize),

    /// Waiting for the OS.
    Waiting,
}
