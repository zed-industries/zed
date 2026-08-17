use core::fmt::Debug;
use core::mem;
use core::net::SocketAddr;

use std::sync::Arc;

use cap_net_ext::AddressFamily;
use io_lifetimes::AsSocketlike as _;
use io_lifetimes::views::SocketlikeView;
use rustix::net::sockopt;

use crate::p3::bindings::sockets::types::{Duration, ErrorCode, IpAddressFamily, IpSocketAddress};
use crate::runtime::with_ambient_tokio_runtime;
use crate::sockets::util::{
    get_unicast_hop_limit, is_valid_address_family, is_valid_unicast_address, receive_buffer_size,
    send_buffer_size, set_keep_alive_count, set_keep_alive_idle_time, set_keep_alive_interval,
    set_receive_buffer_size, set_send_buffer_size, set_unicast_hop_limit, tcp_bind,
};
use crate::sockets::{DEFAULT_TCP_BACKLOG, SocketAddressFamily};

/// The state of a TCP socket.
///
/// This represents the various states a socket can be in during the
/// activities of binding, listening, accepting, and connecting.
pub enum TcpState {
    /// The initial state for a newly-created socket.
    Default(tokio::net::TcpSocket),

    /// Binding finished. The socket has an address but is not yet listening for connections.
    Bound(tokio::net::TcpSocket),

    /// The socket is now listening and waiting for an incoming connection.
    Listening(Arc<tokio::net::TcpListener>),

    /// An outgoing connection is started.
    Connecting,

    /// A connection has been established.
    Connected(Arc<tokio::net::TcpStream>),

    /// A connection has been established and `receive` has been called.
    Receiving(Arc<tokio::net::TcpStream>),

    Error(ErrorCode),

    Closed,
}

impl Debug for TcpState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default(_) => f.debug_tuple("Default").finish(),
            Self::Bound(_) => f.debug_tuple("Bound").finish(),
            Self::Listening { .. } => f.debug_tuple("Listening").finish(),
            Self::Connecting => f.debug_tuple("Connecting").finish(),
            Self::Connected { .. } => f.debug_tuple("Connected").finish(),
            Self::Receiving { .. } => f.debug_tuple("Receiving").finish(),
            Self::Error(..) => f.debug_tuple("Error").finish(),
            Self::Closed => write!(f, "Closed"),
        }
    }
}

/// A host TCP socket, plus associated bookkeeping.
pub struct TcpSocket {
    /// The current state in the bind/listen/accept/connect progression.
    pub tcp_state: TcpState,

    /// The desired listen queue size.
    pub listen_backlog_size: u32,

    pub family: SocketAddressFamily,

    pub options: NonInheritedOptions,
}

impl TcpSocket {
    /// Create a new socket in the given family.
    pub fn new(family: AddressFamily) -> std::io::Result<Self> {
        with_ambient_tokio_runtime(|| {
            let (socket, family) = match family {
                AddressFamily::Ipv4 => {
                    let socket = tokio::net::TcpSocket::new_v4()?;
                    (socket, SocketAddressFamily::Ipv4)
                }
                AddressFamily::Ipv6 => {
                    let socket = tokio::net::TcpSocket::new_v6()?;
                    sockopt::set_ipv6_v6only(&socket, true)?;
                    (socket, SocketAddressFamily::Ipv6)
                }
            };

            Ok(Self::from_state(TcpState::Default(socket), family))
        })
    }

    /// Create a `TcpSocket` from an existing socket.
    pub fn from_state(state: TcpState, family: SocketAddressFamily) -> Self {
        Self {
            tcp_state: state,
            listen_backlog_size: DEFAULT_TCP_BACKLOG,
            family,
            options: Default::default(),
        }
    }

    pub fn as_std_view(&self) -> Result<SocketlikeView<'_, std::net::TcpStream>, ErrorCode> {
        match &self.tcp_state {
            TcpState::Default(socket) | TcpState::Bound(socket) => Ok(socket.as_socketlike_view()),
            TcpState::Connected(stream) | TcpState::Receiving(stream) => {
                Ok(stream.as_socketlike_view())
            }
            TcpState::Listening(listener) => Ok(listener.as_socketlike_view()),
            TcpState::Connecting | TcpState::Closed => Err(ErrorCode::InvalidState),
            TcpState::Error(err) => Err(*err),
        }
    }

    pub fn bind(&mut self, addr: SocketAddr) -> Result<(), ErrorCode> {
        let ip = addr.ip();
        if !is_valid_unicast_address(ip) || !is_valid_address_family(ip, self.family) {
            return Err(ErrorCode::InvalidArgument);
        }
        match mem::replace(&mut self.tcp_state, TcpState::Closed) {
            TcpState::Default(sock) => {
                if let Err(err) = tcp_bind(&sock, addr) {
                    self.tcp_state = TcpState::Default(sock);
                    Err(err.into())
                } else {
                    self.tcp_state = TcpState::Bound(sock);
                    Ok(())
                }
            }
            tcp_state => {
                self.tcp_state = tcp_state;
                Err(ErrorCode::InvalidState)
            }
        }
    }

    pub fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        match &self.tcp_state {
            TcpState::Bound(socket) => {
                let addr = socket.local_addr()?;
                Ok(addr.into())
            }
            TcpState::Connected(stream) | TcpState::Receiving(stream) => {
                let addr = stream.local_addr()?;
                Ok(addr.into())
            }
            TcpState::Listening(listener) => {
                let addr = listener.local_addr()?;
                Ok(addr.into())
            }
            TcpState::Error(err) => Err(*err),
            _ => Err(ErrorCode::InvalidState),
        }
    }

    pub fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
        match &self.tcp_state {
            TcpState::Connected(stream) | TcpState::Receiving(stream) => {
                let addr = stream.peer_addr()?;
                Ok(addr.into())
            }
            TcpState::Error(err) => Err(*err),
            _ => Err(ErrorCode::InvalidState),
        }
    }

    pub fn is_listening(&self) -> bool {
        matches!(self.tcp_state, TcpState::Listening { .. })
    }

    pub fn address_family(&self) -> IpAddressFamily {
        match self.family {
            SocketAddressFamily::Ipv4 => IpAddressFamily::Ipv4,
            SocketAddressFamily::Ipv6 => IpAddressFamily::Ipv6,
        }
    }

    pub fn set_listen_backlog_size(&mut self, value: u64) -> Result<(), ErrorCode> {
        const MIN_BACKLOG: u32 = 1;
        const MAX_BACKLOG: u32 = i32::MAX as u32; // OS'es will most likely limit it down even further.

        if value == 0 {
            return Err(ErrorCode::InvalidArgument);
        }
        // Silently clamp backlog size. This is OK for us to do, because operating systems do this too.
        let value = value
            .try_into()
            .unwrap_or(MAX_BACKLOG)
            .clamp(MIN_BACKLOG, MAX_BACKLOG);
        match &self.tcp_state {
            TcpState::Default(..) | TcpState::Bound(..) => {
                // Socket not listening yet. Stash value for first invocation to `listen`.
                self.listen_backlog_size = value;
                Ok(())
            }
            TcpState::Listening(listener) => {
                // Try to update the backlog by calling `listen` again.
                // Not all platforms support this. We'll only update our own value if the OS supports changing the backlog size after the fact.
                if rustix::net::listen(&listener, value.try_into().unwrap_or(i32::MAX)).is_err() {
                    return Err(ErrorCode::NotSupported);
                }
                self.listen_backlog_size = value;
                Ok(())
            }
            TcpState::Error(err) => Err(*err),
            _ => Err(ErrorCode::InvalidState),
        }
    }

    pub fn keep_alive_enabled(&self) -> Result<bool, ErrorCode> {
        let fd = &*self.as_std_view()?;
        let v = sockopt::socket_keepalive(fd)?;
        Ok(v)
    }

    pub fn set_keep_alive_enabled(&self, value: bool) -> Result<(), ErrorCode> {
        let fd = &*self.as_std_view()?;
        sockopt::set_socket_keepalive(fd, value)?;
        Ok(())
    }

    pub fn keep_alive_idle_time(&self) -> Result<Duration, ErrorCode> {
        let fd = &*self.as_std_view()?;
        let v = sockopt::tcp_keepidle(fd)?;
        Ok(v.as_nanos().try_into().unwrap_or(u64::MAX))
    }

    pub fn set_keep_alive_idle_time(&mut self, value: Duration) -> Result<(), ErrorCode> {
        let value = {
            let fd = self.as_std_view()?;
            set_keep_alive_idle_time(&*fd, value)?
        };
        self.options.set_keep_alive_idle_time(value);
        Ok(())
    }

    pub fn keep_alive_interval(&self) -> Result<Duration, ErrorCode> {
        let fd = &*self.as_std_view()?;
        let v = sockopt::tcp_keepintvl(fd)?;
        Ok(v.as_nanos().try_into().unwrap_or(u64::MAX))
    }

    pub fn set_keep_alive_interval(&self, value: Duration) -> Result<(), ErrorCode> {
        let fd = &*self.as_std_view()?;
        set_keep_alive_interval(fd, core::time::Duration::from_nanos(value))?;
        Ok(())
    }

    pub fn keep_alive_count(&self) -> Result<u32, ErrorCode> {
        let fd = &*self.as_std_view()?;
        let v = sockopt::tcp_keepcnt(fd)?;
        Ok(v)
    }

    pub fn set_keep_alive_count(&self, value: u32) -> Result<(), ErrorCode> {
        let fd = &*self.as_std_view()?;
        set_keep_alive_count(fd, value)?;
        Ok(())
    }

    pub fn hop_limit(&self) -> Result<u8, ErrorCode> {
        let fd = &*self.as_std_view()?;
        let n = get_unicast_hop_limit(fd, self.family)?;
        Ok(n)
    }

    pub fn set_hop_limit(&mut self, value: u8) -> Result<(), ErrorCode> {
        {
            let fd = &*self.as_std_view()?;
            set_unicast_hop_limit(fd, self.family, value)?;
        }
        self.options.set_hop_limit(value);
        Ok(())
    }

    pub fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
        let fd = &*self.as_std_view()?;
        let n = receive_buffer_size(fd)?;
        Ok(n)
    }

    pub fn set_receive_buffer_size(&mut self, value: u64) -> Result<(), ErrorCode> {
        let res = {
            let fd = &*self.as_std_view()?;
            set_receive_buffer_size(fd, value)?
        };
        self.options.set_receive_buffer_size(res);
        Ok(())
    }

    pub fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
        let fd = &*self.as_std_view()?;
        let n = send_buffer_size(fd)?;
        Ok(n)
    }

    pub fn set_send_buffer_size(&mut self, value: u64) -> Result<(), ErrorCode> {
        let res = {
            let fd = &*self.as_std_view()?;
            set_send_buffer_size(fd, value)?
        };
        self.options.set_send_buffer_size(res);
        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
pub use inherits_option::*;
#[cfg(not(target_os = "macos"))]
mod inherits_option {
    use crate::sockets::SocketAddressFamily;
    use tokio::net::TcpStream;

    #[derive(Default, Clone)]
    pub struct NonInheritedOptions;

    impl NonInheritedOptions {
        pub fn set_keep_alive_idle_time(&mut self, _value: u64) {}

        pub fn set_hop_limit(&mut self, _value: u8) {}

        pub fn set_receive_buffer_size(&mut self, _value: usize) {}

        pub fn set_send_buffer_size(&mut self, _value: usize) {}

        pub fn apply(&self, _family: SocketAddressFamily, _stream: &TcpStream) {}
    }
}

#[cfg(target_os = "macos")]
pub use does_not_inherit_options::*;
#[cfg(target_os = "macos")]
mod does_not_inherit_options {
    use crate::sockets::SocketAddressFamily;
    use rustix::net::sockopt;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU8, AtomicU64, AtomicUsize, Ordering::Relaxed};
    use std::time::Duration;
    use tokio::net::TcpStream;

    // The socket options below are not automatically inherited from the listener
    // on all platforms. So we keep track of which options have been explicitly
    // set and manually apply those values to newly accepted clients.
    #[derive(Default, Clone)]
    pub struct NonInheritedOptions(Arc<Inner>);

    #[derive(Default)]
    struct Inner {
        receive_buffer_size: AtomicUsize,
        send_buffer_size: AtomicUsize,
        hop_limit: AtomicU8,
        keep_alive_idle_time: AtomicU64, // nanoseconds
    }

    impl NonInheritedOptions {
        pub fn set_keep_alive_idle_time(&mut self, value: u64) {
            self.0.keep_alive_idle_time.store(value, Relaxed);
        }

        pub fn set_hop_limit(&mut self, value: u8) {
            self.0.hop_limit.store(value, Relaxed);
        }

        pub fn set_receive_buffer_size(&mut self, value: usize) {
            self.0.receive_buffer_size.store(value, Relaxed);
        }

        pub fn set_send_buffer_size(&mut self, value: usize) {
            self.0.send_buffer_size.store(value, Relaxed);
        }

        pub fn apply(&self, family: SocketAddressFamily, stream: &TcpStream) {
            // Manually inherit socket options from listener. We only have to
            // do this on platforms that don't already do this automatically
            // and only if a specific value was explicitly set on the listener.

            let receive_buffer_size = self.0.receive_buffer_size.load(Relaxed);
            if receive_buffer_size > 0 {
                // Ignore potential error.
                _ = sockopt::set_socket_recv_buffer_size(&stream, receive_buffer_size);
            }

            let send_buffer_size = self.0.send_buffer_size.load(Relaxed);
            if send_buffer_size > 0 {
                // Ignore potential error.
                _ = sockopt::set_socket_send_buffer_size(&stream, send_buffer_size);
            }

            // For some reason, IP_TTL is inherited, but IPV6_UNICAST_HOPS isn't.
            if family == SocketAddressFamily::Ipv6 {
                let hop_limit = self.0.hop_limit.load(Relaxed);
                if hop_limit > 0 {
                    // Ignore potential error.
                    _ = sockopt::set_ipv6_unicast_hops(&stream, Some(hop_limit));
                }
            }

            let keep_alive_idle_time = self.0.keep_alive_idle_time.load(Relaxed);
            if keep_alive_idle_time > 0 {
                // Ignore potential error.
                _ = sockopt::set_tcp_keepidle(&stream, Duration::from_nanos(keep_alive_idle_time));
            }
        }
    }
}
