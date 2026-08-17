use {
    crate::{size_hint, Arbitrary, Result, Unstructured},
    std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

impl<'a> Arbitrary<'a> for Ipv4Addr {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Ipv4Addr::from(u32::arbitrary(u)?))
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (4, Some(4))
    }
}

impl<'a> Arbitrary<'a> for Ipv6Addr {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Ipv6Addr::from(u128::arbitrary(u)?))
    }

    #[inline]
    fn size_hint(_depth: usize) -> (usize, Option<usize>) {
        (16, Some(16))
    }
}

impl<'a> Arbitrary<'a> for IpAddr {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        if u.arbitrary()? {
            Ok(IpAddr::V4(u.arbitrary()?))
        } else {
            Ok(IpAddr::V6(u.arbitrary()?))
        }
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::and(
            bool::size_hint(depth),
            size_hint::or(Ipv4Addr::size_hint(depth), Ipv6Addr::size_hint(depth)),
        )
    }
}

impl<'a> Arbitrary<'a> for SocketAddrV4 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(SocketAddrV4::new(u.arbitrary()?, u.arbitrary()?))
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::and(Ipv4Addr::size_hint(depth), u16::size_hint(depth))
    }
}

impl<'a> Arbitrary<'a> for SocketAddrV6 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(SocketAddrV6::new(
            u.arbitrary()?,
            u.arbitrary()?,
            u.arbitrary()?,
            u.arbitrary()?,
        ))
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::and(
            Ipv6Addr::size_hint(depth),
            size_hint::and(
                u16::size_hint(depth),
                size_hint::and(u32::size_hint(depth), u32::size_hint(depth)),
            ),
        )
    }
}

impl<'a> Arbitrary<'a> for SocketAddr {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        if u.arbitrary()? {
            Ok(SocketAddr::V4(u.arbitrary()?))
        } else {
            Ok(SocketAddr::V6(u.arbitrary()?))
        }
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        size_hint::and(
            bool::size_hint(depth),
            size_hint::or(
                SocketAddrV4::size_hint(depth),
                SocketAddrV6::size_hint(depth),
            ),
        )
    }
}
