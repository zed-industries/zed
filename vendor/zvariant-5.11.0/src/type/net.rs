use crate::impl_type_with_repr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

#[cfg(feature = "url")]
impl_type_with_repr! {
    url::Url => &str {
        url_ {
            samples = [url::Url::parse("https://example.com").unwrap()],
            repr(url) = url.as_ref(),
        }
    }
}

impl_type_with_repr! {
    Ipv4Addr => [u8; 4] {
        ipv4_addr {
            samples = [Ipv4Addr::LOCALHOST],
            repr(addr) = addr.octets(),
        }
    }
}

impl_type_with_repr! {
    Ipv6Addr => [u8; 16] {
        ipv6_addr {
            samples = [Ipv6Addr::LOCALHOST],
            repr(addr) = addr.octets(),
        }
    }
}

impl_type_with_repr! {
    IpAddr => (u32, &[u8]) {
        ip_addr {
            samples = [IpAddr::V4(Ipv4Addr::LOCALHOST), IpAddr::V6(Ipv6Addr::LOCALHOST)],
            repr(addr) = match addr {
                IpAddr::V4(v4) => (0, &v4.octets()),
                IpAddr::V6(v6) => (1, &v6.octets()),
            },
        }
    }
}

impl_type_with_repr! {
    SocketAddrV4 => (Ipv4Addr, u16) {
        socket_addr_v4 {
            samples = [SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080)],
            repr(addr) = (*addr.ip(), addr.port()),
        }
    }
}

impl_type_with_repr! {
    SocketAddrV6 => (Ipv6Addr, u16) {
        socket_addr_v6 {
            samples = [SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8080, 0, 0)],
            // https://github.com/serde-rs/serde/blob/9b868ef831c95f50dd4bde51a7eb52e3b9ee265a/serde/src/ser/impls.rs#L966
            repr(addr) = (*addr.ip(), addr.port()),
        }
    }
}

// TODO(bash): Implement DynamicType for SocketAddr
