//! Archived versions of network types.

use crate::Archived;

/// An archived [`Ipv4Addr`](std::net::Ipv4Addr).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ArchivedIpv4Addr {
    octets: [Archived<u8>; 4],
}

impl ArchivedIpv4Addr {
    /// Returns the four eight-bit integers that make up this address.
    #[inline]
    pub const fn octets(&self) -> [u8; 4] {
        self.octets
    }
}

/// An archived [`Ipv6Addr`](std::net::Ipv6Addr).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ArchivedIpv6Addr {
    octets: [Archived<u8>; 16],
}

impl ArchivedIpv6Addr {
    /// Returns the eight 16-bit segments that make up this address.
    #[inline]
    pub const fn segments(&self) -> [u16; 8] {
        [
            u16::from_be_bytes([self.octets[0], self.octets[1]]),
            u16::from_be_bytes([self.octets[2], self.octets[3]]),
            u16::from_be_bytes([self.octets[4], self.octets[5]]),
            u16::from_be_bytes([self.octets[6], self.octets[7]]),
            u16::from_be_bytes([self.octets[8], self.octets[9]]),
            u16::from_be_bytes([self.octets[10], self.octets[11]]),
            u16::from_be_bytes([self.octets[12], self.octets[13]]),
            u16::from_be_bytes([self.octets[14], self.octets[15]]),
        ]
    }
}

/// An archived [`IpAddr`](std::net::IpAddr).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ArchivedIpAddr {
    /// An IPv4 address.
    V4(ArchivedIpv4Addr),
    /// An IPv6 address.
    V6(ArchivedIpv6Addr),
}

impl ArchivedIpAddr {
    /// Returns `true` if this address is an [`IPv4` address](std::net::IpAddr::V4), and `false`
    /// otherwise.
    #[inline]
    pub const fn is_ipv4(&self) -> bool {
        matches!(self, ArchivedIpAddr::V4(_))
    }

    /// Returns `true` if this address is an [`IPv6` address](std::net::IpAddr::V6), and `false`
    /// otherwise.
    #[inline]
    pub const fn is_ipv6(&self) -> bool {
        matches!(self, ArchivedIpAddr::V6(_))
    }
}

/// An archived [`SocketAddrV4`](std::net::SocketAddrV4).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedSocketAddrV4 {
    pub(crate) ip: ArchivedIpv4Addr,
    pub(crate) port: Archived<u16>,
}

impl ArchivedSocketAddrV4 {
    /// Returns the IP address associated with this socket address.
    #[inline]
    pub const fn ip(&self) -> &ArchivedIpv4Addr {
        &self.ip
    }

    /// Returns the port number associated with this socket address.
    #[inline]
    pub const fn port(&self) -> u16 {
        from_archived!(self.port)
    }
}

/// An archived [`SocketAddrV6`](std::net::SocketAddrV6).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedSocketAddrV6 {
    pub(crate) ip: ArchivedIpv6Addr,
    pub(crate) port: Archived<u16>,
    pub(crate) flowinfo: Archived<u32>,
    pub(crate) scope_id: Archived<u32>,
}

impl ArchivedSocketAddrV6 {
    /// Returns the flow information associated with this address.
    ///
    /// See [`SocketAddrV6::flowinfo()`](std::net::SocketAddrV6::flowinfo()) for more details.
    #[inline]
    pub const fn flowinfo(&self) -> u32 {
        from_archived!(self.flowinfo)
    }

    /// Returns the IP address associated with this socket address.
    #[inline]
    pub const fn ip(&self) -> &ArchivedIpv6Addr {
        &self.ip
    }

    /// Returns the port number associated with this socket address.
    #[inline]
    pub const fn port(&self) -> u16 {
        from_archived!(self.port)
    }

    /// Returns the scope ID associated with this address.
    ///
    /// See [`SocketAddrV6::scope_id()`](std::net::SocketAddrV6::scope_id()) for more details.
    #[inline]
    pub const fn scope_id(&self) -> u32 {
        from_archived!(self.scope_id)
    }
}

/// An archived [`SocketAddr`](std::net::SocketAddr).
#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ArchivedSocketAddr {
    /// An IPv4 socket address.
    V4(ArchivedSocketAddrV4),
    /// An IPv6 socket address.
    V6(ArchivedSocketAddrV6),
}

impl ArchivedSocketAddr {
    /// Returns the port number associated with this socket address.
    #[inline]
    pub fn port(&self) -> u16 {
        match self {
            ArchivedSocketAddr::V4(addr) => addr.port(),
            ArchivedSocketAddr::V6(addr) => addr.port(),
        }
    }

    /// Returns `true` if the [IP address](std::net::IpAddr) in this `ArchivedSocketAddr` is an
    /// [`IPv4` address](std::net::IpAddr::V4), and `false` otherwise.
    #[inline]
    pub fn is_ipv4(&self) -> bool {
        matches!(self, ArchivedSocketAddr::V4(_))
    }

    /// Returns `true` if the [IP address](std::net::IpAddr) in this `ArchivedSocketAddr` is an
    /// [`IPv6` address](std::net::IpAddr::V6), and `false` otherwise.
    #[inline]
    pub fn is_ipv6(&self) -> bool {
        matches!(self, ArchivedSocketAddr::V6(_))
    }
}
