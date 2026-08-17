use crate::Win32::Networking::WinSock::{IN_ADDR, IN_ADDR_0};

impl From<std::net::Ipv4Addr> for IN_ADDR {
    fn from(addr: std::net::Ipv4Addr) -> Self {
        // u32::from(addr) is in host byte order
        // S_addr must be big-endian, network byte order
        Self { S_un: IN_ADDR_0 { S_addr: u32::from(addr).to_be() } }
    }
}
impl From<IN_ADDR> for std::net::Ipv4Addr {
    fn from(in_addr: IN_ADDR) -> Self {
        // SAFETY: this is safe because the union variants are just views of the same exact data
        // in_addr.S_un.S_addr is big-endian, network byte order
        // Ipv4Addr::new() expects the parameter in host byte order
        Self::from(u32::from_be(unsafe { in_addr.S_un.S_addr }))
    }
}
