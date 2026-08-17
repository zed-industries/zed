use std::{
    borrow::Cow,
    io::Result as IoResult,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs},
    pin::Pin,
    task::{Context, Poll},
    vec,
};

use either::Either;
pub use error::Error;
use futures_util::{
    future,
    stream::{self, Once, Stream},
};

pub type Result<T> = std::result::Result<T, Error>;

/// A trait for objects which can be converted or resolved to one or more
/// `SocketAddr` values, which are going to be connected as the the proxy
/// server.
///
/// This trait is similar to `std::net::ToSocketAddrs` but allows asynchronous
/// name resolution.
pub trait ToProxyAddrs {
    type Output: Stream<Item = Result<SocketAddr>> + Unpin;

    fn to_proxy_addrs(&self) -> Self::Output;
}

macro_rules! trivial_impl_to_proxy_addrs {
    ($t: ty) => {
        impl ToProxyAddrs for $t {
            type Output = Once<future::Ready<Result<SocketAddr>>>;

            fn to_proxy_addrs(&self) -> Self::Output {
                stream::once(future::ready(Ok(SocketAddr::from(*self))))
            }
        }
    };
}

trivial_impl_to_proxy_addrs!(SocketAddr);
trivial_impl_to_proxy_addrs!((IpAddr, u16));
trivial_impl_to_proxy_addrs!((Ipv4Addr, u16));
trivial_impl_to_proxy_addrs!((Ipv6Addr, u16));
trivial_impl_to_proxy_addrs!(SocketAddrV4);
trivial_impl_to_proxy_addrs!(SocketAddrV6);

impl<'a> ToProxyAddrs for &'a [SocketAddr] {
    type Output = ProxyAddrsStream;

    fn to_proxy_addrs(&self) -> Self::Output {
        let addrs = self.to_vec();
        ProxyAddrsStream(Some(IoResult::Ok(addrs.into_iter())))
    }
}

impl ToProxyAddrs for str {
    type Output = ProxyAddrsStream;

    fn to_proxy_addrs(&self) -> Self::Output {
        ProxyAddrsStream(Some(self.to_socket_addrs()))
    }
}

impl<'a> ToProxyAddrs for (&'a str, u16) {
    type Output = ProxyAddrsStream;

    fn to_proxy_addrs(&self) -> Self::Output {
        ProxyAddrsStream(Some(self.to_socket_addrs()))
    }
}

impl<'a, T: ToProxyAddrs + ?Sized> ToProxyAddrs for &'a T {
    type Output = T::Output;

    fn to_proxy_addrs(&self) -> Self::Output {
        (**self).to_proxy_addrs()
    }
}

pub struct ProxyAddrsStream(Option<IoResult<vec::IntoIter<SocketAddr>>>);

impl Stream for ProxyAddrsStream {
    type Item = Result<SocketAddr>;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.0.as_mut() {
            Some(Ok(iter)) => Poll::Ready(iter.next().map(Result::Ok)),
            Some(Err(_)) => {
                let err = self.0.take().unwrap().unwrap_err();
                Poll::Ready(Some(Err(err.into())))
            },
            None => unreachable!(),
        }
    }
}

/// A SOCKS connection target.
#[derive(Debug, PartialEq, Eq)]
pub enum TargetAddr<'a> {
    /// Connect to an IP address.
    Ip(SocketAddr),

    /// Connect to a fully-qualified domain name.
    ///
    /// The domain name will be passed along to the proxy server and DNS lookup
    /// will happen there.
    Domain(Cow<'a, str>, u16),
}

impl<'a> TargetAddr<'a> {
    /// Creates owned `TargetAddr` by cloning. It is usually used to eliminate
    /// the lifetime bound.
    pub fn to_owned(&self) -> TargetAddr<'static> {
        match self {
            TargetAddr::Ip(addr) => TargetAddr::Ip(*addr),
            TargetAddr::Domain(domain, port) => TargetAddr::Domain(String::from(domain.clone()).into(), *port),
        }
    }
}

impl<'a> ToSocketAddrs for TargetAddr<'a> {
    type Iter = Either<std::option::IntoIter<SocketAddr>, std::vec::IntoIter<SocketAddr>>;

    fn to_socket_addrs(&self) -> IoResult<Self::Iter> {
        Ok(match self {
            TargetAddr::Ip(addr) => Either::Left(addr.to_socket_addrs()?),
            TargetAddr::Domain(domain, port) => Either::Right((&**domain, *port).to_socket_addrs()?),
        })
    }
}

/// A trait for objects that can be converted to `TargetAddr`.
pub trait IntoTargetAddr<'a> {
    /// Converts the value of self to a `TargetAddr`.
    fn into_target_addr(self) -> Result<TargetAddr<'a>>;
}

macro_rules! trivial_impl_into_target_addr {
    ($t: ty) => {
        impl<'a> IntoTargetAddr<'a> for $t {
            fn into_target_addr(self) -> Result<TargetAddr<'a>> {
                Ok(TargetAddr::Ip(SocketAddr::from(self)))
            }
        }
    };
}

trivial_impl_into_target_addr!(SocketAddr);
trivial_impl_into_target_addr!((IpAddr, u16));
trivial_impl_into_target_addr!((Ipv4Addr, u16));
trivial_impl_into_target_addr!((Ipv6Addr, u16));
trivial_impl_into_target_addr!(SocketAddrV4);
trivial_impl_into_target_addr!(SocketAddrV6);

impl<'a> IntoTargetAddr<'a> for TargetAddr<'a> {
    fn into_target_addr(self) -> Result<TargetAddr<'a>> {
        Ok(self)
    }
}

impl<'a> IntoTargetAddr<'a> for (&'a str, u16) {
    fn into_target_addr(self) -> Result<TargetAddr<'a>> {
        // Try IP address first
        if let Ok(addr) = self.0.parse::<IpAddr>() {
            return (addr, self.1).into_target_addr();
        }

        // Treat as domain name
        if self.0.len() > 255 {
            return Err(Error::InvalidTargetAddress("overlong domain"));
        }
        // TODO: Should we validate the domain format here?

        Ok(TargetAddr::Domain(self.0.into(), self.1))
    }
}

impl<'a> IntoTargetAddr<'a> for &'a str {
    fn into_target_addr(self) -> Result<TargetAddr<'a>> {
        // Try IP address first
        if let Ok(addr) = self.parse::<SocketAddr>() {
            return addr.into_target_addr();
        }

        let mut parts_iter = self.rsplitn(2, ':');
        let port: u16 = parts_iter
            .next()
            .and_then(|port_str| port_str.parse().ok())
            .ok_or(Error::InvalidTargetAddress("invalid address format"))?;
        let domain = parts_iter
            .next()
            .ok_or(Error::InvalidTargetAddress("invalid address format"))?;
        if domain.len() > 255 {
            return Err(Error::InvalidTargetAddress("overlong domain"));
        }
        Ok(TargetAddr::Domain(domain.into(), port))
    }
}

impl IntoTargetAddr<'static> for String {
    fn into_target_addr(mut self) -> Result<TargetAddr<'static>> {
        // Try IP address first
        if let Ok(addr) = self.parse::<SocketAddr>() {
            return addr.into_target_addr();
        }

        let mut parts_iter = self.rsplitn(2, ':');
        let port: u16 = parts_iter
            .next()
            .and_then(|port_str| port_str.parse().ok())
            .ok_or(Error::InvalidTargetAddress("invalid address format"))?;
        let domain_len = parts_iter
            .next()
            .ok_or(Error::InvalidTargetAddress("invalid address format"))?
            .len();
        if domain_len > 255 {
            return Err(Error::InvalidTargetAddress("overlong domain"));
        }
        self.truncate(domain_len);
        Ok(TargetAddr::Domain(self.into(), port))
    }
}

impl IntoTargetAddr<'static> for (String, u16) {
    fn into_target_addr(self) -> Result<TargetAddr<'static>> {
        let addr = (self.0.as_str(), self.1).into_target_addr()?;
        if let TargetAddr::Ip(addr) = addr {
            Ok(TargetAddr::Ip(addr))
        } else {
            Ok(TargetAddr::Domain(self.0.into(), self.1))
        }
    }
}

impl<'a, T> IntoTargetAddr<'a> for &'a T
where T: IntoTargetAddr<'a> + Copy
{
    fn into_target_addr(self) -> Result<TargetAddr<'a>> {
        (*self).into_target_addr()
    }
}

/// Authentication methods
#[derive(Debug)]
enum Authentication<'a> {
    Password { username: &'a str, password: &'a str },
    None,
}

impl<'a> Authentication<'a> {
    fn id(&self) -> u8 {
        match self {
            Authentication::Password { .. } => 0x02,
            Authentication::None => 0x00,
        }
    }
}

mod error;
pub mod io;
pub mod tcp;

#[cfg(test)]
mod tests {
    use futures_executor::block_on;
    use futures_util::StreamExt;

    use super::*;

    fn to_proxy_addrs<T: ToProxyAddrs>(t: T) -> Result<Vec<SocketAddr>> {
        Ok(block_on(t.to_proxy_addrs().map(Result::unwrap).collect()))
    }

    #[test]
    fn converts_socket_addr_to_proxy_addrs() -> Result<()> {
        let addr = SocketAddr::from(([1, 1, 1, 1], 443));
        let res = to_proxy_addrs(addr)?;
        assert_eq!(&res[..], &[addr]);
        Ok(())
    }

    #[test]
    fn converts_socket_addr_ref_to_proxy_addrs() -> Result<()> {
        let addr = SocketAddr::from(([1, 1, 1, 1], 443));
        let res = to_proxy_addrs(addr)?;
        assert_eq!(&res[..], &[addr]);
        Ok(())
    }

    #[test]
    fn converts_socket_addrs_to_proxy_addrs() -> Result<()> {
        let addrs = [
            SocketAddr::from(([1, 1, 1, 1], 443)),
            SocketAddr::from(([8, 8, 8, 8], 53)),
        ];
        let res = to_proxy_addrs(&addrs[..])?;
        assert_eq!(&res[..], &addrs);
        Ok(())
    }

    fn into_target_addr<'a, T>(t: T) -> Result<TargetAddr<'a>>
    where T: IntoTargetAddr<'a> {
        t.into_target_addr()
    }

    #[test]
    fn converts_socket_addr_to_target_addr() -> Result<()> {
        let addr = SocketAddr::from(([1, 1, 1, 1], 443));
        let res = into_target_addr(addr)?;
        assert_eq!(TargetAddr::Ip(addr), res);
        Ok(())
    }

    #[test]
    fn converts_socket_addr_ref_to_target_addr() -> Result<()> {
        let addr = SocketAddr::from(([1, 1, 1, 1], 443));
        let res = into_target_addr(addr)?;
        assert_eq!(TargetAddr::Ip(addr), res);
        Ok(())
    }

    #[test]
    fn converts_socket_addr_str_to_target_addr() -> Result<()> {
        let addr = SocketAddr::from(([1, 1, 1, 1], 443));
        let ip_str = format!("{}", addr);
        let res = into_target_addr(ip_str.as_str())?;
        assert_eq!(TargetAddr::Ip(addr), res);
        Ok(())
    }

    #[test]
    fn converts_ip_str_and_port_target_addr() -> Result<()> {
        let addr = SocketAddr::from(([1, 1, 1, 1], 443));
        let ip_str = format!("{}", addr.ip());
        let res = into_target_addr((ip_str.as_str(), addr.port()))?;
        assert_eq!(TargetAddr::Ip(addr), res);
        Ok(())
    }

    #[test]
    fn converts_domain_to_target_addr() -> Result<()> {
        let domain = "www.example.com:80";
        let res = into_target_addr(domain)?;
        assert_eq!(TargetAddr::Domain(Cow::Borrowed("www.example.com"), 80), res);

        let res = into_target_addr(domain.to_owned())?;
        assert_eq!(TargetAddr::Domain(Cow::Owned("www.example.com".to_owned()), 80), res);
        Ok(())
    }

    #[test]
    fn converts_domain_and_port_to_target_addr() -> Result<()> {
        let domain = "www.example.com";
        let res = into_target_addr((domain, 80))?;
        assert_eq!(TargetAddr::Domain(Cow::Borrowed("www.example.com"), 80), res);
        Ok(())
    }

    #[test]
    fn overlong_domain_to_target_addr_should_fail() {
        let domain = format!("www.{:a<1$}.com:80", 'a', 300);
        assert!(into_target_addr(domain.as_str()).is_err());
        let domain = format!("www.{:a<1$}.com", 'a', 300);
        assert!(into_target_addr((domain.as_str(), 80)).is_err());
    }

    #[test]
    fn addr_with_invalid_port_to_target_addr_should_fail() {
        let addr = "[ffff::1]:65536";
        assert!(into_target_addr(addr).is_err());
        let addr = "www.example.com:65536";
        assert!(into_target_addr(addr).is_err());
    }
}
