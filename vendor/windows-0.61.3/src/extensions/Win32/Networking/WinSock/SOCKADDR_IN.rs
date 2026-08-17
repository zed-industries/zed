use crate::Win32::Networking::WinSock::{AF_INET, SOCKADDR_IN};

impl From<std::net::SocketAddrV4> for SOCKADDR_IN {
    fn from(addr: std::net::SocketAddrV4) -> Self {
        // addr.port() is in host byte order
        // sin_port must be big-endian, network byte order
        SOCKADDR_IN { sin_family: AF_INET, sin_port: addr.port().to_be(), sin_addr: (*addr.ip()).into(), ..Default::default() }
    }
}
