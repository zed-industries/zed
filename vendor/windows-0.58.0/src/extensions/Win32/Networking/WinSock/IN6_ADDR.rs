use crate::Win32::Networking::WinSock::{IN6_ADDR, IN6_ADDR_0};

impl From<std::net::Ipv6Addr> for IN6_ADDR {
    fn from(addr: std::net::Ipv6Addr) -> Self {
        Self { u: IN6_ADDR_0 { Byte: addr.octets() } }
    }
}
impl From<IN6_ADDR> for std::net::Ipv6Addr {
    fn from(in6_addr: IN6_ADDR) -> Self {
        // SAFETY: this is safe because the union variants are just views of the same exact data
        Self::from(unsafe { in6_addr.u.Byte })
    }
}
