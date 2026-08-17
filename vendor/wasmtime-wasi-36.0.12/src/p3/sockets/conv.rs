use core::net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6};

use std::net::ToSocketAddrs;

use rustix::io::Errno;
use tracing::debug;

use crate::p3::bindings::sockets::types;
use crate::sockets::util::{from_ipv4_addr, from_ipv6_addr, to_ipv4_addr, to_ipv6_addr};

impl From<IpAddr> for types::IpAddress {
    fn from(addr: IpAddr) -> Self {
        match addr {
            IpAddr::V4(v4) => Self::Ipv4(from_ipv4_addr(v4)),
            IpAddr::V6(v6) => Self::Ipv6(from_ipv6_addr(v6)),
        }
    }
}

impl From<types::IpAddress> for IpAddr {
    fn from(addr: types::IpAddress) -> Self {
        match addr {
            types::IpAddress::Ipv4(v4) => Self::V4(to_ipv4_addr(v4)),
            types::IpAddress::Ipv6(v6) => Self::V6(to_ipv6_addr(v6)),
        }
    }
}

impl From<types::IpSocketAddress> for SocketAddr {
    fn from(addr: types::IpSocketAddress) -> Self {
        match addr {
            types::IpSocketAddress::Ipv4(ipv4) => Self::V4(ipv4.into()),
            types::IpSocketAddress::Ipv6(ipv6) => Self::V6(ipv6.into()),
        }
    }
}

impl From<SocketAddr> for types::IpSocketAddress {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(v4) => Self::Ipv4(v4.into()),
            SocketAddr::V6(v6) => Self::Ipv6(v6.into()),
        }
    }
}

impl From<types::Ipv4SocketAddress> for SocketAddrV4 {
    fn from(addr: types::Ipv4SocketAddress) -> Self {
        Self::new(to_ipv4_addr(addr.address), addr.port)
    }
}

impl From<SocketAddrV4> for types::Ipv4SocketAddress {
    fn from(addr: SocketAddrV4) -> Self {
        Self {
            address: from_ipv4_addr(*addr.ip()),
            port: addr.port(),
        }
    }
}

impl From<types::Ipv6SocketAddress> for SocketAddrV6 {
    fn from(addr: types::Ipv6SocketAddress) -> Self {
        Self::new(
            to_ipv6_addr(addr.address),
            addr.port,
            addr.flow_info,
            addr.scope_id,
        )
    }
}

impl From<SocketAddrV6> for types::Ipv6SocketAddress {
    fn from(addr: SocketAddrV6) -> Self {
        Self {
            address: from_ipv6_addr(*addr.ip()),
            port: addr.port(),
            flow_info: addr.flowinfo(),
            scope_id: addr.scope_id(),
        }
    }
}

impl ToSocketAddrs for types::IpSocketAddress {
    type Iter = <SocketAddr as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        SocketAddr::from(*self).to_socket_addrs()
    }
}

impl ToSocketAddrs for types::Ipv4SocketAddress {
    type Iter = <SocketAddrV4 as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        SocketAddrV4::from(*self).to_socket_addrs()
    }
}

impl ToSocketAddrs for types::Ipv6SocketAddress {
    type Iter = <SocketAddrV6 as ToSocketAddrs>::Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        SocketAddrV6::from(*self).to_socket_addrs()
    }
}

impl From<types::IpAddressFamily> for cap_net_ext::AddressFamily {
    fn from(family: types::IpAddressFamily) -> Self {
        match family {
            types::IpAddressFamily::Ipv4 => Self::Ipv4,
            types::IpAddressFamily::Ipv6 => Self::Ipv6,
        }
    }
}

impl From<cap_net_ext::AddressFamily> for types::IpAddressFamily {
    fn from(family: cap_net_ext::AddressFamily) -> Self {
        match family {
            cap_net_ext::AddressFamily::Ipv4 => Self::Ipv4,
            cap_net_ext::AddressFamily::Ipv6 => Self::Ipv6,
        }
    }
}

impl From<std::io::Error> for types::ErrorCode {
    fn from(value: std::io::Error) -> Self {
        (&value).into()
    }
}

impl From<&std::io::Error> for types::ErrorCode {
    fn from(value: &std::io::Error) -> Self {
        // Attempt the more detailed native error code first:
        if let Some(errno) = Errno::from_io_error(value) {
            return errno.into();
        }

        match value.kind() {
            std::io::ErrorKind::AddrInUse => Self::AddressInUse,
            std::io::ErrorKind::AddrNotAvailable => Self::AddressNotBindable,
            std::io::ErrorKind::ConnectionAborted => Self::ConnectionAborted,
            std::io::ErrorKind::ConnectionRefused => Self::ConnectionRefused,
            std::io::ErrorKind::ConnectionReset => Self::ConnectionReset,
            std::io::ErrorKind::InvalidInput => Self::InvalidArgument,
            std::io::ErrorKind::NotConnected => Self::InvalidState,
            std::io::ErrorKind::OutOfMemory => Self::OutOfMemory,
            std::io::ErrorKind::PermissionDenied => Self::AccessDenied,
            std::io::ErrorKind::TimedOut => Self::Timeout,
            std::io::ErrorKind::Unsupported => Self::NotSupported,
            _ => {
                debug!("unknown I/O error: {value}");
                Self::Unknown
            }
        }
    }
}

impl From<Errno> for types::ErrorCode {
    fn from(value: Errno) -> Self {
        (&value).into()
    }
}

impl From<&Errno> for types::ErrorCode {
    fn from(value: &Errno) -> Self {
        match *value {
            #[cfg(not(windows))]
            Errno::PERM => Self::AccessDenied,
            Errno::ACCESS => Self::AccessDenied,
            Errno::ADDRINUSE => Self::AddressInUse,
            Errno::ADDRNOTAVAIL => Self::AddressNotBindable,
            Errno::TIMEDOUT => Self::Timeout,
            Errno::CONNREFUSED => Self::ConnectionRefused,
            Errno::CONNRESET => Self::ConnectionReset,
            Errno::CONNABORTED => Self::ConnectionAborted,
            Errno::INVAL => Self::InvalidArgument,
            Errno::HOSTUNREACH => Self::RemoteUnreachable,
            Errno::HOSTDOWN => Self::RemoteUnreachable,
            Errno::NETDOWN => Self::RemoteUnreachable,
            Errno::NETUNREACH => Self::RemoteUnreachable,
            #[cfg(target_os = "linux")]
            Errno::NONET => Self::RemoteUnreachable,
            Errno::ISCONN => Self::InvalidState,
            Errno::NOTCONN => Self::InvalidState,
            Errno::DESTADDRREQ => Self::InvalidState,
            Errno::MSGSIZE => Self::DatagramTooLarge,
            #[cfg(not(windows))]
            Errno::NOMEM => Self::OutOfMemory,
            Errno::NOBUFS => Self::OutOfMemory,
            Errno::OPNOTSUPP => Self::NotSupported,
            Errno::NOPROTOOPT => Self::NotSupported,
            Errno::PFNOSUPPORT => Self::NotSupported,
            Errno::PROTONOSUPPORT => Self::NotSupported,
            Errno::PROTOTYPE => Self::NotSupported,
            Errno::SOCKTNOSUPPORT => Self::NotSupported,
            Errno::AFNOSUPPORT => Self::NotSupported,

            // FYI, EINPROGRESS should have already been handled by connect.
            _ => {
                debug!("unknown I/O error: {value}");
                Self::Unknown
            }
        }
    }
}

impl From<crate::sockets::util::ErrorCode> for types::ErrorCode {
    fn from(code: crate::sockets::util::ErrorCode) -> Self {
        match code {
            crate::sockets::util::ErrorCode::Unknown => Self::Unknown,
            crate::sockets::util::ErrorCode::AccessDenied => Self::AccessDenied,
            crate::sockets::util::ErrorCode::NotSupported => Self::NotSupported,
            crate::sockets::util::ErrorCode::InvalidArgument => Self::InvalidArgument,
            crate::sockets::util::ErrorCode::OutOfMemory => Self::OutOfMemory,
            crate::sockets::util::ErrorCode::Timeout => Self::Timeout,
            crate::sockets::util::ErrorCode::InvalidState => Self::InvalidState,
            crate::sockets::util::ErrorCode::AddressNotBindable => Self::AddressNotBindable,
            crate::sockets::util::ErrorCode::AddressInUse => Self::AddressInUse,
            crate::sockets::util::ErrorCode::RemoteUnreachable => Self::RemoteUnreachable,
            crate::sockets::util::ErrorCode::ConnectionRefused => Self::ConnectionRefused,
            crate::sockets::util::ErrorCode::ConnectionReset => Self::ConnectionReset,
            crate::sockets::util::ErrorCode::ConnectionAborted => Self::ConnectionAborted,
            crate::sockets::util::ErrorCode::DatagramTooLarge => Self::DatagramTooLarge,
        }
    }
}
