use std::fmt;
use std::future::Future;
use std::io;
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs};
use std::pin::Pin;
use std::task::{Context, Poll};

use blocking::unblock;
use futures_lite::future;

/// Converts or resolves addresses to [`SocketAddr`] values.
///
/// This trait currently only appears in function signatures and cannot be used directly.
///
/// However, you can use the [`resolve()`][`super::resolve()`] function to resolve addresses.
pub trait AsyncToSocketAddrs: Sealed {}

pub trait Sealed {
    /// Returned iterator over socket addresses which this type may correspond to.
    type Iter: Iterator<Item = SocketAddr> + Unpin;

    /// Converts this object to an iterator of resolved `SocketAddr`s.
    ///
    /// The returned iterator may not actually yield any values depending on the outcome of any
    /// resolution performed.
    ///
    /// Note that this function may block a backend thread while resolution is performed.
    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter>;
}

pub enum ToSocketAddrsFuture<I> {
    Resolving(future::Boxed<io::Result<I>>),
    Ready(io::Result<I>),
    Done,
}

impl<I> fmt::Debug for ToSocketAddrsFuture<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ToSocketAddrsFuture")
    }
}

impl<I: Iterator<Item = SocketAddr> + Unpin> Future for ToSocketAddrsFuture<I> {
    type Output = io::Result<I>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let state = mem::replace(&mut *self, ToSocketAddrsFuture::Done);

        match state {
            ToSocketAddrsFuture::Resolving(mut task) => {
                let poll = Pin::new(&mut task).poll(cx);
                if poll.is_pending() {
                    *self = ToSocketAddrsFuture::Resolving(task);
                }
                poll
            }
            ToSocketAddrsFuture::Ready(res) => Poll::Ready(res),
            ToSocketAddrsFuture::Done => panic!("polled a completed future"),
        }
    }
}

impl AsyncToSocketAddrs for SocketAddr {}

impl Sealed for SocketAddr {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        ToSocketAddrsFuture::Ready(Ok(Some(*self).into_iter()))
    }
}

impl AsyncToSocketAddrs for SocketAddrV4 {}

impl Sealed for SocketAddrV4 {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        Sealed::to_socket_addrs(&SocketAddr::V4(*self))
    }
}

impl AsyncToSocketAddrs for SocketAddrV6 {}

impl Sealed for SocketAddrV6 {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        Sealed::to_socket_addrs(&SocketAddr::V6(*self))
    }
}

impl AsyncToSocketAddrs for (IpAddr, u16) {}

impl Sealed for (IpAddr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        let (ip, port) = *self;
        match ip {
            IpAddr::V4(a) => Sealed::to_socket_addrs(&(a, port)),
            IpAddr::V6(a) => Sealed::to_socket_addrs(&(a, port)),
        }
    }
}

impl AsyncToSocketAddrs for (Ipv4Addr, u16) {}

impl Sealed for (Ipv4Addr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        let (ip, port) = *self;
        Sealed::to_socket_addrs(&SocketAddrV4::new(ip, port))
    }
}

impl AsyncToSocketAddrs for (Ipv6Addr, u16) {}

impl Sealed for (Ipv6Addr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        let (ip, port) = *self;
        Sealed::to_socket_addrs(&SocketAddrV6::new(ip, port, 0, 0))
    }
}

impl AsyncToSocketAddrs for (&str, u16) {}

impl Sealed for (&str, u16) {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        let (host, port) = *self;

        if let Ok(addr) = host.parse::<Ipv4Addr>() {
            let addr = SocketAddrV4::new(addr, port);
            return ToSocketAddrsFuture::Ready(Ok(vec![SocketAddr::V4(addr)].into_iter()));
        }

        if let Ok(addr) = host.parse::<Ipv6Addr>() {
            let addr = SocketAddrV6::new(addr, port, 0, 0);
            return ToSocketAddrsFuture::Ready(Ok(vec![SocketAddr::V6(addr)].into_iter()));
        }

        let host = host.to_string();
        let future = unblock(move || {
            let addr = (host.as_str(), port);
            ToSocketAddrs::to_socket_addrs(&addr)
        });
        ToSocketAddrsFuture::Resolving(Box::pin(future))
    }
}

impl AsyncToSocketAddrs for (String, u16) {}

impl Sealed for (String, u16) {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        Sealed::to_socket_addrs(&(&*self.0, self.1))
    }
}

impl AsyncToSocketAddrs for str {}

impl Sealed for str {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        if let Ok(addr) = self.parse() {
            return ToSocketAddrsFuture::Ready(Ok(vec![addr].into_iter()));
        }

        let addr = self.to_string();
        let future = unblock(move || std::net::ToSocketAddrs::to_socket_addrs(addr.as_str()));
        ToSocketAddrsFuture::Resolving(Box::pin(future))
    }
}

impl AsyncToSocketAddrs for &[SocketAddr] {}

impl<'a> Sealed for &'a [SocketAddr] {
    type Iter = std::iter::Cloned<std::slice::Iter<'a, SocketAddr>>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        ToSocketAddrsFuture::Ready(Ok(self.iter().cloned()))
    }
}

impl<T: AsyncToSocketAddrs + ?Sized> AsyncToSocketAddrs for &T {}

impl<T: Sealed + ?Sized> Sealed for &T {
    type Iter = T::Iter;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        Sealed::to_socket_addrs(&**self)
    }
}

impl AsyncToSocketAddrs for String {}

impl Sealed for String {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> ToSocketAddrsFuture<Self::Iter> {
        Sealed::to_socket_addrs(&**self)
    }
}
