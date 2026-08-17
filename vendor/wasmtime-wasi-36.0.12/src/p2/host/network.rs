use crate::p2::bindings::sockets::network::{
    self, ErrorCode, IpAddress, IpAddressFamily, IpSocketAddress, Ipv4SocketAddress,
    Ipv6SocketAddress,
};
use crate::p2::{SocketError, WasiCtxView};
use crate::sockets::util::{from_ipv4_addr, from_ipv6_addr, to_ipv4_addr, to_ipv6_addr};
use anyhow::Error;
use rustix::io::Errno;
use std::io;
use wasmtime::component::Resource;

impl network::Host for WasiCtxView<'_> {
    fn convert_error_code(&mut self, error: SocketError) -> anyhow::Result<ErrorCode> {
        error.downcast()
    }

    fn network_error_code(&mut self, err: Resource<Error>) -> anyhow::Result<Option<ErrorCode>> {
        let err = self.table.get(&err)?;

        if let Some(err) = err.downcast_ref::<std::io::Error>() {
            return Ok(Some(ErrorCode::from(err)));
        }

        Ok(None)
    }
}

impl crate::p2::bindings::sockets::network::HostNetwork for WasiCtxView<'_> {
    fn drop(&mut self, this: Resource<network::Network>) -> Result<(), anyhow::Error> {
        self.table.delete(this)?;

        Ok(())
    }
}

impl From<io::Error> for ErrorCode {
    fn from(value: io::Error) -> Self {
        (&value).into()
    }
}

impl From<&io::Error> for ErrorCode {
    fn from(value: &io::Error) -> Self {
        // Attempt the more detailed native error code first:
        if let Some(errno) = Errno::from_io_error(value) {
            return errno.into();
        }

        match value.kind() {
            std::io::ErrorKind::AddrInUse => ErrorCode::AddressInUse,
            std::io::ErrorKind::AddrNotAvailable => ErrorCode::AddressNotBindable,
            std::io::ErrorKind::ConnectionAborted => ErrorCode::ConnectionAborted,
            std::io::ErrorKind::ConnectionRefused => ErrorCode::ConnectionRefused,
            std::io::ErrorKind::ConnectionReset => ErrorCode::ConnectionReset,
            std::io::ErrorKind::Interrupted => ErrorCode::WouldBlock,
            std::io::ErrorKind::InvalidInput => ErrorCode::InvalidArgument,
            std::io::ErrorKind::NotConnected => ErrorCode::InvalidState,
            std::io::ErrorKind::OutOfMemory => ErrorCode::OutOfMemory,
            std::io::ErrorKind::PermissionDenied => ErrorCode::AccessDenied,
            std::io::ErrorKind::TimedOut => ErrorCode::Timeout,
            std::io::ErrorKind::Unsupported => ErrorCode::NotSupported,
            std::io::ErrorKind::WouldBlock => ErrorCode::WouldBlock,

            _ => {
                tracing::debug!("unknown I/O error: {value}");
                ErrorCode::Unknown
            }
        }
    }
}

impl From<Errno> for ErrorCode {
    fn from(value: Errno) -> Self {
        (&value).into()
    }
}

impl From<&Errno> for ErrorCode {
    fn from(value: &Errno) -> Self {
        match *value {
            Errno::WOULDBLOCK => ErrorCode::WouldBlock,
            #[allow(
                unreachable_patterns,
                reason = "EWOULDBLOCK and EAGAIN can have the same value"
            )]
            Errno::AGAIN => ErrorCode::WouldBlock,
            Errno::INTR => ErrorCode::WouldBlock,
            #[cfg(not(windows))]
            Errno::PERM => ErrorCode::AccessDenied,
            Errno::ACCESS => ErrorCode::AccessDenied,
            Errno::ADDRINUSE => ErrorCode::AddressInUse,
            Errno::ADDRNOTAVAIL => ErrorCode::AddressNotBindable,
            Errno::ALREADY => ErrorCode::ConcurrencyConflict,
            Errno::TIMEDOUT => ErrorCode::Timeout,
            Errno::CONNREFUSED => ErrorCode::ConnectionRefused,
            Errno::CONNRESET => ErrorCode::ConnectionReset,
            Errno::CONNABORTED => ErrorCode::ConnectionAborted,
            Errno::INVAL => ErrorCode::InvalidArgument,
            Errno::HOSTUNREACH => ErrorCode::RemoteUnreachable,
            Errno::HOSTDOWN => ErrorCode::RemoteUnreachable,
            Errno::NETDOWN => ErrorCode::RemoteUnreachable,
            Errno::NETUNREACH => ErrorCode::RemoteUnreachable,
            #[cfg(target_os = "linux")]
            Errno::NONET => ErrorCode::RemoteUnreachable,
            Errno::ISCONN => ErrorCode::InvalidState,
            Errno::NOTCONN => ErrorCode::InvalidState,
            Errno::DESTADDRREQ => ErrorCode::InvalidState,
            #[cfg(not(windows))]
            Errno::NFILE => ErrorCode::NewSocketLimit,
            Errno::MFILE => ErrorCode::NewSocketLimit,
            Errno::MSGSIZE => ErrorCode::DatagramTooLarge,
            #[cfg(not(windows))]
            Errno::NOMEM => ErrorCode::OutOfMemory,
            Errno::NOBUFS => ErrorCode::OutOfMemory,
            Errno::OPNOTSUPP => ErrorCode::NotSupported,
            Errno::NOPROTOOPT => ErrorCode::NotSupported,
            Errno::PFNOSUPPORT => ErrorCode::NotSupported,
            Errno::PROTONOSUPPORT => ErrorCode::NotSupported,
            Errno::PROTOTYPE => ErrorCode::NotSupported,
            Errno::SOCKTNOSUPPORT => ErrorCode::NotSupported,
            Errno::AFNOSUPPORT => ErrorCode::NotSupported,

            // FYI, EINPROGRESS should have already been handled by connect.
            _ => {
                tracing::debug!("unknown I/O error: {value}");
                ErrorCode::Unknown
            }
        }
    }
}

impl From<std::net::IpAddr> for IpAddress {
    fn from(addr: std::net::IpAddr) -> Self {
        match addr {
            std::net::IpAddr::V4(v4) => Self::Ipv4(from_ipv4_addr(v4)),
            std::net::IpAddr::V6(v6) => Self::Ipv6(from_ipv6_addr(v6)),
        }
    }
}

impl From<IpSocketAddress> for std::net::SocketAddr {
    fn from(addr: IpSocketAddress) -> Self {
        match addr {
            IpSocketAddress::Ipv4(ipv4) => Self::V4(ipv4.into()),
            IpSocketAddress::Ipv6(ipv6) => Self::V6(ipv6.into()),
        }
    }
}

impl From<std::net::SocketAddr> for IpSocketAddress {
    fn from(addr: std::net::SocketAddr) -> Self {
        match addr {
            std::net::SocketAddr::V4(v4) => Self::Ipv4(v4.into()),
            std::net::SocketAddr::V6(v6) => Self::Ipv6(v6.into()),
        }
    }
}

impl From<Ipv4SocketAddress> for std::net::SocketAddrV4 {
    fn from(addr: Ipv4SocketAddress) -> Self {
        Self::new(to_ipv4_addr(addr.address), addr.port)
    }
}

impl From<std::net::SocketAddrV4> for Ipv4SocketAddress {
    fn from(addr: std::net::SocketAddrV4) -> Self {
        Self {
            address: from_ipv4_addr(*addr.ip()),
            port: addr.port(),
        }
    }
}

impl From<Ipv6SocketAddress> for std::net::SocketAddrV6 {
    fn from(addr: Ipv6SocketAddress) -> Self {
        Self::new(
            to_ipv6_addr(addr.address),
            addr.port,
            addr.flow_info,
            addr.scope_id,
        )
    }
}

impl From<std::net::SocketAddrV6> for Ipv6SocketAddress {
    fn from(addr: std::net::SocketAddrV6) -> Self {
        Self {
            address: from_ipv6_addr(*addr.ip()),
            port: addr.port(),
            flow_info: addr.flowinfo(),
            scope_id: addr.scope_id(),
        }
    }
}

impl std::net::ToSocketAddrs for IpSocketAddress {
    type Iter = <std::net::SocketAddr as std::net::ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        std::net::SocketAddr::from(*self).to_socket_addrs()
    }
}

impl std::net::ToSocketAddrs for Ipv4SocketAddress {
    type Iter = <std::net::SocketAddrV4 as std::net::ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        std::net::SocketAddrV4::from(*self).to_socket_addrs()
    }
}

impl std::net::ToSocketAddrs for Ipv6SocketAddress {
    type Iter = <std::net::SocketAddrV6 as std::net::ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        std::net::SocketAddrV6::from(*self).to_socket_addrs()
    }
}

impl From<IpAddressFamily> for cap_net_ext::AddressFamily {
    fn from(family: IpAddressFamily) -> Self {
        match family {
            IpAddressFamily::Ipv4 => cap_net_ext::AddressFamily::Ipv4,
            IpAddressFamily::Ipv6 => cap_net_ext::AddressFamily::Ipv6,
        }
    }
}

impl From<cap_net_ext::AddressFamily> for IpAddressFamily {
    fn from(family: cap_net_ext::AddressFamily) -> Self {
        match family {
            cap_net_ext::AddressFamily::Ipv4 => IpAddressFamily::Ipv4,
            cap_net_ext::AddressFamily::Ipv6 => IpAddressFamily::Ipv6,
        }
    }
}
