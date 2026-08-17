#[cfg(test)]
use crate::ambient_authority;
use crate::net::pool::net::ToSocketAddrs;
use crate::AmbientAuthority;
use ipnet::IpNet;
#[cfg(test)]
use std::str::FromStr;
use std::{io, net};

// TODO: Perhaps we should have our own version of `ToSocketAddrs` which
// returns hostnames rather than parsing them, so we can add unresolved
// hostnames to the pool.
#[derive(Clone)]
enum AddrSet {
    Net(IpNet),
}

impl AddrSet {
    fn contains(&self, addr: net::IpAddr) -> bool {
        match self {
            Self::Net(ip_net) => ip_net.contains(&addr),
        }
    }
}

#[derive(Clone)]
struct IpGrant {
    set: AddrSet,
    ports_start: u16,
    ports_end: Option<u16>,
}

impl IpGrant {
    fn contains(&self, addr: &net::SocketAddr) -> bool {
        if !self.set.contains(addr.ip()) {
            return false;
        }

        let port = addr.port();
        if port < self.ports_start {
            return false;
        }
        if let Some(ports_end) = self.ports_end {
            if port >= ports_end {
                return false;
            }
        }

        true
    }
}

/// A representation of a set of network resources that may be accessed.
///
/// This is presently a very simple concept, though it could grow in
/// sophistication in the future.
///
/// `Pool` implements `Clone`, which creates new independent entities that
/// carry the full authority of the originals. This means that in a borrow
/// of a `Pool`, the scope of the authority is not necessarily limited to
/// the scope of the borrow.
///
/// Similarly, the [`cap_net_ext::PoolExt`] class allows creating "binder"
/// and "connecter" objects which represent capabilities to bind and
/// connect to addresses.
///
/// [`cap_net_ext::PoolExt`]: https://docs.rs/cap-net-ext/latest/cap_net_ext/trait.PoolExt.html
#[derive(Clone, Default)]
pub struct Pool {
    // TODO: when compiling for WASI, use WASI-specific handle instead
    grants: Vec<IpGrant>,
}

impl Pool {
    /// Construct a new empty pool.
    pub fn new() -> Self {
        Self { grants: Vec::new() }
    }

    /// Add addresses to the pool.
    ///
    /// # Ambient Authority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert<A: ToSocketAddrs>(
        &mut self,
        addrs: A,
        ambient_authority: AmbientAuthority,
    ) -> io::Result<()> {
        for addr in addrs.to_socket_addrs()? {
            self.insert_socket_addr(addr, ambient_authority);
        }
        Ok(())
    }

    /// Add a specific [`net::SocketAddr`] to the pool.
    ///
    /// # Ambient Authority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert_socket_addr(
        &mut self,
        addr: net::SocketAddr,
        ambient_authority: AmbientAuthority,
    ) {
        self.insert_ip_net(addr.ip().into(), addr.port(), ambient_authority)
    }

    /// Add a range of network addresses, accepting any port, to the pool.
    ///
    /// # Ambient Authority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert_ip_net_port_any(
        &mut self,
        ip_net: ipnet::IpNet,
        ambient_authority: AmbientAuthority,
    ) {
        self.insert_ip_net_port_range(ip_net, 0, None, ambient_authority)
    }

    /// Add a range of network addresses, accepting a range of ports, to the
    /// pool.
    ///
    /// This grants access to the port range starting at `ports_start` and,
    /// if `ports_end` is provided, ending before `ports_end`.
    ///
    /// # Ambient Authority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert_ip_net_port_range(
        &mut self,
        ip_net: ipnet::IpNet,
        ports_start: u16,
        ports_end: Option<u16>,
        ambient_authority: AmbientAuthority,
    ) {
        let _ = ambient_authority;

        self.grants.push(IpGrant {
            set: AddrSet::Net(ip_net),
            ports_start,
            ports_end,
        })
    }

    /// Add a range of network addresses with a specific port to the pool.
    ///
    /// # Ambient Authority
    ///
    /// This function allows ambient access to any IP address.
    pub fn insert_ip_net(
        &mut self,
        ip_net: ipnet::IpNet,
        port: u16,
        ambient_authority: AmbientAuthority,
    ) {
        self.insert_ip_net_port_range(ip_net, port, port.checked_add(1), ambient_authority)
    }

    /// Check whether the given address is within the pool.
    pub fn check_addr(&self, addr: &net::SocketAddr) -> io::Result<()> {
        if self.grants.iter().any(|grant| grant.contains(addr)) {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "An address was outside the pool",
            ))
        }
    }
}

/// An empty array of `SocketAddr`s.
pub const NO_SOCKET_ADDRS: &[net::SocketAddr] = &[];

/// Return an error for reporting that no socket addresses were available.
#[cold]
pub fn no_socket_addrs() -> io::Error {
    std::net::TcpListener::bind(NO_SOCKET_ADDRS).unwrap_err()
}

#[test]
fn test_empty() {
    let p = Pool::new();

    p.check_addr(&net::SocketAddr::from_str("[::1]:0").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:1023").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:1024").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:8080").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:65535").unwrap())
        .unwrap_err();
}

#[test]
fn test_port_any() {
    let mut p = Pool::new();
    p.insert_ip_net_port_any(
        IpNet::new(net::IpAddr::V6(net::Ipv6Addr::LOCALHOST), 48).unwrap(),
        ambient_authority(),
    );

    p.check_addr(&net::SocketAddr::from_str("[::1]:0").unwrap())
        .unwrap();
    p.check_addr(&net::SocketAddr::from_str("[::1]:1023").unwrap())
        .unwrap();
    p.check_addr(&net::SocketAddr::from_str("[::1]:1024").unwrap())
        .unwrap();
    p.check_addr(&net::SocketAddr::from_str("[::1]:8080").unwrap())
        .unwrap();
    p.check_addr(&net::SocketAddr::from_str("[::1]:65535").unwrap())
        .unwrap();
}

#[test]
fn test_port_range() {
    let mut p = Pool::new();
    p.insert_ip_net_port_range(
        IpNet::new(net::IpAddr::V6(net::Ipv6Addr::LOCALHOST), 48).unwrap(),
        1024,
        Some(9000),
        ambient_authority(),
    );

    p.check_addr(&net::SocketAddr::from_str("[::1]:0").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:1023").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:1024").unwrap())
        .unwrap();
    p.check_addr(&net::SocketAddr::from_str("[::1]:8080").unwrap())
        .unwrap();
    p.check_addr(&net::SocketAddr::from_str("[::1]:65535").unwrap())
        .unwrap_err();
}

#[test]
fn test_port_one() {
    let mut p = Pool::new();
    p.insert_ip_net(
        IpNet::new(net::IpAddr::V6(net::Ipv6Addr::LOCALHOST), 48).unwrap(),
        8080,
        ambient_authority(),
    );

    p.check_addr(&net::SocketAddr::from_str("[::1]:0").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:1023").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:1024").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:8080").unwrap())
        .unwrap();
    p.check_addr(&net::SocketAddr::from_str("[::1]:65535").unwrap())
        .unwrap_err();
}

#[test]
fn test_addrs() {
    let mut p = Pool::new();
    match p.insert("example.com:80", ambient_authority()) {
        Ok(()) => (),
        Err(_) => return, // not all test environments have DNS
    }

    p.check_addr(&net::SocketAddr::from_str("[::1]:0").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:1023").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:1024").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:8080").unwrap())
        .unwrap_err();
    p.check_addr(&net::SocketAddr::from_str("[::1]:65535").unwrap())
        .unwrap_err();

    for addr in "example.com:80".to_socket_addrs().unwrap() {
        p.check_addr(&addr).unwrap();
    }
}
