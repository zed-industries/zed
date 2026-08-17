use crate::Win32::Networking::WinSock::SOCKADDR_INET;

impl From<std::net::SocketAddrV4> for SOCKADDR_INET {
    fn from(addr: std::net::SocketAddrV4) -> Self {
        SOCKADDR_INET { Ipv4: addr.into() }
    }
}
impl From<std::net::SocketAddrV6> for SOCKADDR_INET {
    fn from(addr: std::net::SocketAddrV6) -> Self {
        SOCKADDR_INET { Ipv6: addr.into() }
    }
}
impl From<std::net::SocketAddr> for SOCKADDR_INET {
    fn from(addr: std::net::SocketAddr) -> Self {
        match addr {
            std::net::SocketAddr::V4(socket_addr_v4) => socket_addr_v4.into(),
            std::net::SocketAddr::V6(socket_addr_v6) => socket_addr_v6.into(),
        }
    }
}
