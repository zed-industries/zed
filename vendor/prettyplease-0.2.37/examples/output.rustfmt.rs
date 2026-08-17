use crate::cmp::Ordering;
use crate::fmt::{self, Write as FmtWrite};
use crate::hash;
use crate::io::Write as IoWrite;
use crate::mem::transmute;
use crate::sys::net::netc as c;
use crate::sys_common::{AsInner, FromInner, IntoInner};
#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
#[derive(Copy)]
pub struct Ipv4Addr {
    inner: c::in_addr,
}
#[derive(Copy)]
pub struct Ipv6Addr {
    inner: c::in6_addr,
}
#[derive(Copy, PartialEq, Eq, Clone, Hash, Debug)]
#[non_exhaustive]
pub enum Ipv6MulticastScope {
    InterfaceLocal,
    LinkLocal,
    RealmLocal,
    AdminLocal,
    SiteLocal,
    OrganizationLocal,
    Global,
}
impl IpAddr {
    pub const fn is_unspecified(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_unspecified(),
            IpAddr::V6(ip) => ip.is_unspecified(),
        }
    }
    pub const fn is_loopback(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_loopback(),
            IpAddr::V6(ip) => ip.is_loopback(),
        }
    }
    pub const fn is_global(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_global(),
            IpAddr::V6(ip) => ip.is_global(),
        }
    }
    pub const fn is_multicast(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_multicast(),
            IpAddr::V6(ip) => ip.is_multicast(),
        }
    }
    pub const fn is_documentation(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_documentation(),
            IpAddr::V6(ip) => ip.is_documentation(),
        }
    }
    pub const fn is_benchmarking(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_benchmarking(),
            IpAddr::V6(ip) => ip.is_benchmarking(),
        }
    }
    pub const fn is_ipv4(&self) -> bool {
        matches!(self, IpAddr::V4(_))
    }
    pub const fn is_ipv6(&self) -> bool {
        matches!(self, IpAddr::V6(_))
    }
    pub const fn to_canonical(&self) -> IpAddr {
        match self {
            &v4 @ IpAddr::V4(_) => v4,
            IpAddr::V6(v6) => v6.to_canonical(),
        }
    }
}
impl Ipv4Addr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        Ipv4Addr {
            inner: c::in_addr {
                s_addr: u32::from_ne_bytes([a, b, c, d]),
            },
        }
    }
    pub const LOCALHOST: Self = Ipv4Addr::new(127, 0, 0, 1);
    #[doc(alias = "INADDR_ANY")]
    pub const UNSPECIFIED: Self = Ipv4Addr::new(0, 0, 0, 0);
    pub const BROADCAST: Self = Ipv4Addr::new(255, 255, 255, 255);
    pub const fn octets(&self) -> [u8; 4] {
        self.inner.s_addr.to_ne_bytes()
    }
    pub const fn is_unspecified(&self) -> bool {
        self.inner.s_addr == 0
    }
    pub const fn is_loopback(&self) -> bool {
        self.octets()[0] == 127
    }
    pub const fn is_private(&self) -> bool {
        match self.octets() {
            [10, ..] => true,
            [172, b, ..] if b >= 16 && b <= 31 => true,
            [192, 168, ..] => true,
            _ => false,
        }
    }
    pub const fn is_link_local(&self) -> bool {
        matches!(self.octets(), [169, 254, ..])
    }
    pub const fn is_global(&self) -> bool {
        if u32::from_be_bytes(self.octets()) == 0xc0000009
            || u32::from_be_bytes(self.octets()) == 0xc000000a
        {
            return true;
        }
        !self.is_private()
            && !self.is_loopback()
            && !self.is_link_local()
            && !self.is_broadcast()
            && !self.is_documentation()
            && !self.is_shared()
            && !(self.octets()[0] == 192 && self.octets()[1] == 0 && self.octets()[2] == 0)
            && !self.is_reserved()
            && !self.is_benchmarking()
            && self.octets()[0] != 0
    }
    pub const fn is_shared(&self) -> bool {
        self.octets()[0] == 100 && (self.octets()[1] & 0b1100_0000 == 0b0100_0000)
    }
    pub const fn is_benchmarking(&self) -> bool {
        self.octets()[0] == 198 && (self.octets()[1] & 0xfe) == 18
    }
    pub const fn is_reserved(&self) -> bool {
        self.octets()[0] & 240 == 240 && !self.is_broadcast()
    }
    pub const fn is_multicast(&self) -> bool {
        self.octets()[0] >= 224 && self.octets()[0] <= 239
    }
    pub const fn is_broadcast(&self) -> bool {
        u32::from_be_bytes(self.octets()) == u32::from_be_bytes(Self::BROADCAST.octets())
    }
    pub const fn is_documentation(&self) -> bool {
        matches!(
            self.octets(),
            [192, 0, 2, _] | [198, 51, 100, _] | [203, 0, 113, _]
        )
    }
    pub const fn to_ipv6_compatible(&self) -> Ipv6Addr {
        let [a, b, c, d] = self.octets();
        Ipv6Addr {
            inner: c::in6_addr {
                s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, a, b, c, d],
            },
        }
    }
    pub const fn to_ipv6_mapped(&self) -> Ipv6Addr {
        let [a, b, c, d] = self.octets();
        Ipv6Addr {
            inner: c::in6_addr {
                s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, a, b, c, d],
            },
        }
    }
}
impl fmt::Display for IpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddr::V4(ip) => ip.fmt(fmt),
            IpAddr::V6(ip) => ip.fmt(fmt),
        }
    }
}
impl fmt::Debug for IpAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}
impl From<Ipv4Addr> for IpAddr {
    fn from(ipv4: Ipv4Addr) -> IpAddr {
        IpAddr::V4(ipv4)
    }
}
impl From<Ipv6Addr> for IpAddr {
    fn from(ipv6: Ipv6Addr) -> IpAddr {
        IpAddr::V6(ipv6)
    }
}
impl fmt::Display for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octets = self.octets();
        if fmt.precision().is_none() && fmt.width().is_none() {
            write!(
                fmt,
                "{}.{}.{}.{}",
                octets[0], octets[1], octets[2], octets[3]
            )
        } else {
            const IPV4_BUF_LEN: usize = 15;
            let mut buf = [0u8; IPV4_BUF_LEN];
            let mut buf_slice = &mut buf[..];
            write!(
                buf_slice,
                "{}.{}.{}.{}",
                octets[0], octets[1], octets[2], octets[3]
            )
            .unwrap();
            let len = IPV4_BUF_LEN - buf_slice.len();
            let buf = unsafe { crate::str::from_utf8_unchecked(&buf[..len]) };
            fmt.pad(buf)
        }
    }
}
impl fmt::Debug for Ipv4Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}
impl Clone for Ipv4Addr {
    fn clone(&self) -> Ipv4Addr {
        *self
    }
}
impl PartialEq for Ipv4Addr {
    fn eq(&self, other: &Ipv4Addr) -> bool {
        self.inner.s_addr == other.inner.s_addr
    }
}
impl PartialEq<Ipv4Addr> for IpAddr {
    fn eq(&self, other: &Ipv4Addr) -> bool {
        match self {
            IpAddr::V4(v4) => v4 == other,
            IpAddr::V6(_) => false,
        }
    }
}
impl PartialEq<IpAddr> for Ipv4Addr {
    fn eq(&self, other: &IpAddr) -> bool {
        match other {
            IpAddr::V4(v4) => self == v4,
            IpAddr::V6(_) => false,
        }
    }
}
impl Eq for Ipv4Addr {}
impl hash::Hash for Ipv4Addr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        { self.inner.s_addr }.hash(s)
    }
}
impl PartialOrd for Ipv4Addr {
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialOrd<Ipv4Addr> for IpAddr {
    fn partial_cmp(&self, other: &Ipv4Addr) -> Option<Ordering> {
        match self {
            IpAddr::V4(v4) => v4.partial_cmp(other),
            IpAddr::V6(_) => Some(Ordering::Greater),
        }
    }
}
impl PartialOrd<IpAddr> for Ipv4Addr {
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        match other {
            IpAddr::V4(v4) => self.partial_cmp(v4),
            IpAddr::V6(_) => Some(Ordering::Less),
        }
    }
}
impl Ord for Ipv4Addr {
    fn cmp(&self, other: &Ipv4Addr) -> Ordering {
        u32::from_be(self.inner.s_addr).cmp(&u32::from_be(other.inner.s_addr))
    }
}
impl IntoInner<c::in_addr> for Ipv4Addr {
    fn into_inner(self) -> c::in_addr {
        self.inner
    }
}
impl From<Ipv4Addr> for u32 {
    fn from(ip: Ipv4Addr) -> u32 {
        let ip = ip.octets();
        u32::from_be_bytes(ip)
    }
}
impl From<u32> for Ipv4Addr {
    fn from(ip: u32) -> Ipv4Addr {
        Ipv4Addr::from(ip.to_be_bytes())
    }
}
impl From<[u8; 4]> for Ipv4Addr {
    fn from(octets: [u8; 4]) -> Ipv4Addr {
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
    }
}
impl From<[u8; 4]> for IpAddr {
    fn from(octets: [u8; 4]) -> IpAddr {
        IpAddr::V4(Ipv4Addr::from(octets))
    }
}
impl Ipv6Addr {
    pub const fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Ipv6Addr {
        let addr16 = [
            a.to_be(),
            b.to_be(),
            c.to_be(),
            d.to_be(),
            e.to_be(),
            f.to_be(),
            g.to_be(),
            h.to_be(),
        ];
        Ipv6Addr {
            inner: c::in6_addr {
                s6_addr: unsafe { transmute::<_, [u8; 16]>(addr16) },
            },
        }
    }
    pub const LOCALHOST: Self = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    pub const UNSPECIFIED: Self = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
    pub const fn segments(&self) -> [u16; 8] {
        let [a, b, c, d, e, f, g, h] = unsafe { transmute::<_, [u16; 8]>(self.inner.s6_addr) };
        [
            u16::from_be(a),
            u16::from_be(b),
            u16::from_be(c),
            u16::from_be(d),
            u16::from_be(e),
            u16::from_be(f),
            u16::from_be(g),
            u16::from_be(h),
        ]
    }
    pub const fn is_unspecified(&self) -> bool {
        u128::from_be_bytes(self.octets()) == u128::from_be_bytes(Ipv6Addr::UNSPECIFIED.octets())
    }
    pub const fn is_loopback(&self) -> bool {
        u128::from_be_bytes(self.octets()) == u128::from_be_bytes(Ipv6Addr::LOCALHOST.octets())
    }
    pub const fn is_global(&self) -> bool {
        match self.multicast_scope() {
            Some(Ipv6MulticastScope::Global) => true,
            None => self.is_unicast_global(),
            _ => false,
        }
    }
    pub const fn is_unique_local(&self) -> bool {
        (self.segments()[0] & 0xfe00) == 0xfc00
    }
    pub const fn is_unicast(&self) -> bool {
        !self.is_multicast()
    }
    pub const fn is_unicast_link_local(&self) -> bool {
        (self.segments()[0] & 0xffc0) == 0xfe80
    }
    pub const fn is_documentation(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0xdb8)
    }
    pub const fn is_benchmarking(&self) -> bool {
        (self.segments()[0] == 0x2001) && (self.segments()[1] == 0x2) && (self.segments()[2] == 0)
    }
    pub const fn is_unicast_global(&self) -> bool {
        self.is_unicast()
            && !self.is_loopback()
            && !self.is_unicast_link_local()
            && !self.is_unique_local()
            && !self.is_unspecified()
            && !self.is_documentation()
    }
    pub const fn multicast_scope(&self) -> Option<Ipv6MulticastScope> {
        if self.is_multicast() {
            match self.segments()[0] & 0x000f {
                1 => Some(Ipv6MulticastScope::InterfaceLocal),
                2 => Some(Ipv6MulticastScope::LinkLocal),
                3 => Some(Ipv6MulticastScope::RealmLocal),
                4 => Some(Ipv6MulticastScope::AdminLocal),
                5 => Some(Ipv6MulticastScope::SiteLocal),
                8 => Some(Ipv6MulticastScope::OrganizationLocal),
                14 => Some(Ipv6MulticastScope::Global),
                _ => None,
            }
        } else {
            None
        }
    }
    pub const fn is_multicast(&self) -> bool {
        (self.segments()[0] & 0xff00) == 0xff00
    }
    pub const fn to_ipv4_mapped(&self) -> Option<Ipv4Addr> {
        match self.octets() {
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, a, b, c, d] => {
                Some(Ipv4Addr::new(a, b, c, d))
            }
            _ => None,
        }
    }
    pub const fn to_ipv4(&self) -> Option<Ipv4Addr> {
        if let [0, 0, 0, 0, 0, 0 | 0xffff, ab, cd] = self.segments() {
            let [a, b] = ab.to_be_bytes();
            let [c, d] = cd.to_be_bytes();
            Some(Ipv4Addr::new(a, b, c, d))
        } else {
            None
        }
    }
    pub const fn to_canonical(&self) -> IpAddr {
        if let Some(mapped) = self.to_ipv4_mapped() {
            return IpAddr::V4(mapped);
        }
        IpAddr::V6(*self)
    }
    pub const fn octets(&self) -> [u8; 16] {
        self.inner.s6_addr
    }
}
impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.precision().is_none() && f.width().is_none() {
            let segments = self.segments();
            if self.is_unspecified() {
                f.write_str("::")
            } else if self.is_loopback() {
                f.write_str("::1")
            } else if let Some(ipv4) = self.to_ipv4() {
                match segments[5] {
                    0 => write!(f, "::{}", ipv4),
                    0xffff => write!(f, "::ffff:{}", ipv4),
                    _ => unreachable!(),
                }
            } else { # [derive (Copy , Clone , Default)] struct Span { start : usize , len : usize , } let zeroes = { let mut longest = Span :: default () ; let mut current = Span :: default () ; for (i , & segment) in segments . iter () . enumerate () { if segment == 0 { if current . len == 0 { current . start = i ; } current . len += 1 ; if current . len > longest . len { longest = current ; } } else { current = Span :: default () ; } } longest } ; # [doc = " Write a colon-separated part of the address"] # [inline] fn fmt_subslice (f : & mut fmt :: Formatter < '_ > , chunk : & [u16]) -> fmt :: Result { if let Some ((first , tail)) = chunk . split_first () { write ! (f , "{:x}" , first) ? ; for segment in tail { f . write_char (':') ? ; write ! (f , "{:x}" , segment) ? ; } } Ok (()) } if zeroes . len > 1 { fmt_subslice (f , & segments [.. zeroes . start]) ? ; f . write_str ("::") ? ; fmt_subslice (f , & segments [zeroes . start + zeroes . len ..]) } else { fmt_subslice (f , & segments) } }
        } else {
            const IPV6_BUF_LEN: usize = (4 * 8) + 7;
            let mut buf = [0u8; IPV6_BUF_LEN];
            let mut buf_slice = &mut buf[..];
            write!(buf_slice, "{}", self).unwrap();
            let len = IPV6_BUF_LEN - buf_slice.len();
            let buf = unsafe { crate::str::from_utf8_unchecked(&buf[..len]) };
            f.pad(buf)
        }
    }
}
impl fmt::Debug for Ipv6Addr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}
impl Clone for Ipv6Addr {
    fn clone(&self) -> Ipv6Addr {
        *self
    }
}
impl PartialEq for Ipv6Addr {
    fn eq(&self, other: &Ipv6Addr) -> bool {
        self.inner.s6_addr == other.inner.s6_addr
    }
}
impl PartialEq<IpAddr> for Ipv6Addr {
    fn eq(&self, other: &IpAddr) -> bool {
        match other {
            IpAddr::V4(_) => false,
            IpAddr::V6(v6) => self == v6,
        }
    }
}
impl PartialEq<Ipv6Addr> for IpAddr {
    fn eq(&self, other: &Ipv6Addr) -> bool {
        match self {
            IpAddr::V4(_) => false,
            IpAddr::V6(v6) => v6 == other,
        }
    }
}
impl Eq for Ipv6Addr {}
impl hash::Hash for Ipv6Addr {
    fn hash<H: hash::Hasher>(&self, s: &mut H) {
        self.inner.s6_addr.hash(s)
    }
}
impl PartialOrd for Ipv6Addr {
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialOrd<Ipv6Addr> for IpAddr {
    fn partial_cmp(&self, other: &Ipv6Addr) -> Option<Ordering> {
        match self {
            IpAddr::V4(_) => Some(Ordering::Less),
            IpAddr::V6(v6) => v6.partial_cmp(other),
        }
    }
}
impl PartialOrd<IpAddr> for Ipv6Addr {
    fn partial_cmp(&self, other: &IpAddr) -> Option<Ordering> {
        match other {
            IpAddr::V4(_) => Some(Ordering::Greater),
            IpAddr::V6(v6) => self.partial_cmp(v6),
        }
    }
}
impl Ord for Ipv6Addr {
    fn cmp(&self, other: &Ipv6Addr) -> Ordering {
        self.segments().cmp(&other.segments())
    }
}
impl AsInner<c::in6_addr> for Ipv6Addr {
    fn as_inner(&self) -> &c::in6_addr {
        &self.inner
    }
}
impl FromInner<c::in6_addr> for Ipv6Addr {
    fn from_inner(addr: c::in6_addr) -> Ipv6Addr {
        Ipv6Addr { inner: addr }
    }
}
impl From<Ipv6Addr> for u128 {
    fn from(ip: Ipv6Addr) -> u128 {
        let ip = ip.octets();
        u128::from_be_bytes(ip)
    }
}
impl From<u128> for Ipv6Addr {
    fn from(ip: u128) -> Ipv6Addr {
        Ipv6Addr::from(ip.to_be_bytes())
    }
}
impl From<[u8; 16]> for Ipv6Addr {
    fn from(octets: [u8; 16]) -> Ipv6Addr {
        let inner = c::in6_addr { s6_addr: octets };
        Ipv6Addr::from_inner(inner)
    }
}
impl From<[u16; 8]> for Ipv6Addr {
    fn from(segments: [u16; 8]) -> Ipv6Addr {
        let [a, b, c, d, e, f, g, h] = segments;
        Ipv6Addr::new(a, b, c, d, e, f, g, h)
    }
}
impl From<[u8; 16]> for IpAddr {
    fn from(octets: [u8; 16]) -> IpAddr {
        IpAddr::V6(Ipv6Addr::from(octets))
    }
}
impl From<[u16; 8]> for IpAddr {
    fn from(segments: [u16; 8]) -> IpAddr {
        IpAddr::V6(Ipv6Addr::from(segments))
    }
}
