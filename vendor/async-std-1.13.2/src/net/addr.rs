use std::future::Future;
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};
use std::pin::Pin;

use crate::io;
use crate::task::{spawn_blocking, Context, JoinHandle, Poll};
use crate::utils::Context as ErrorContext;

cfg_not_docs! {
    macro_rules! ret {
        (impl Future<Output = $out:ty>, $fut:ty) => ($fut);
    }
}

cfg_docs! {
    #[doc(hidden)]
    pub struct ImplFuture<T>(std::marker::PhantomData<T>);

    macro_rules! ret {
        (impl Future<Output = $out:ty>, $fut:ty) => (ImplFuture<$out>);
    }
}

/// Converts or resolves addresses to [`SocketAddr`] values.
///
/// This trait is an async version of [`std::net::ToSocketAddrs`].
///
/// [`std::net::ToSocketAddrs`]: https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html
/// [`SocketAddr`]: enum.SocketAddr.html
///
/// # Examples
///
/// ```
/// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
/// #
/// use async_std::net::ToSocketAddrs;
///
/// let addr = "localhost:8080".to_socket_addrs().await?.next().unwrap();
/// println!("resolved: {:?}", addr);
/// #
/// # Ok(()) }) }
/// ```
pub trait ToSocketAddrs {
    /// Returned iterator over socket addresses which this type may correspond to.
    type Iter: Iterator<Item = SocketAddr>;

    /// Converts this object to an iterator of resolved `SocketAddr`s.
    ///
    /// The returned iterator may not actually yield any values depending on the outcome of any
    /// resolution performed.
    ///
    /// Note that this function may block a backend thread while resolution is performed.
    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    );
}

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub enum ToSocketAddrsFuture<I> {
    Resolving(JoinHandle<io::Result<I>>),
    Ready(io::Result<I>),
    Done,
}

// The field of `Self::Resolving` is `Unpin`, and the field of `Self::Ready` is never pinned.
impl<I> Unpin for ToSocketAddrsFuture<I> {}

/// Wrap `std::io::Error` with additional message
///
/// Keeps the original error kind and stores the original I/O error as `source`.
impl<T> ErrorContext for ToSocketAddrsFuture<T> {
    fn context(self, message: impl Fn() -> String) -> Self {
        match self {
            ToSocketAddrsFuture::Ready(res) => ToSocketAddrsFuture::Ready(res.context(message)),
            x => x,
        }
    }
}

impl<I: Iterator<Item = SocketAddr>> Future for ToSocketAddrsFuture<I> {
    type Output = io::Result<I>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let state = mem::replace(this, ToSocketAddrsFuture::Done);

        match state {
            ToSocketAddrsFuture::Resolving(mut task) => {
                let poll = Pin::new(&mut task).poll(cx);
                if poll.is_pending() {
                    *this = ToSocketAddrsFuture::Resolving(task);
                }
                poll
            }
            ToSocketAddrsFuture::Ready(res) => Poll::Ready(res),
            ToSocketAddrsFuture::Done => panic!("polled a completed future"),
        }
    }
}

impl ToSocketAddrs for SocketAddr {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
        ToSocketAddrsFuture::Ready(Ok(Some(*self).into_iter()))
    }
}

impl ToSocketAddrs for SocketAddrV4 {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
        SocketAddr::V4(*self)
            .to_socket_addrs()
            .context(|| format!("could not resolve address `{}`", self))
    }
}

impl ToSocketAddrs for SocketAddrV6 {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
        SocketAddr::V6(*self)
            .to_socket_addrs()
            .context(|| format!("could not resolve address `{}`", self))
    }
}

impl ToSocketAddrs for (IpAddr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
        let (ip, port) = *self;
        match ip {
            IpAddr::V4(a) => (a, port).to_socket_addrs(),
            IpAddr::V6(a) => (a, port).to_socket_addrs(),
        }
    }
}

impl ToSocketAddrs for (Ipv4Addr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
        let (ip, port) = *self;
        SocketAddrV4::new(ip, port).to_socket_addrs()
    }
}

impl ToSocketAddrs for (Ipv6Addr, u16) {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
        let (ip, port) = *self;
        SocketAddrV6::new(ip, port, 0, 0).to_socket_addrs()
    }
}

impl ToSocketAddrs for (&str, u16) {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
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
        let task = spawn_blocking(move || {
            let addr = (host.as_str(), port);
            std::net::ToSocketAddrs::to_socket_addrs(&addr)
                .context(|| format!("could not resolve address `{:?}`", addr))
        });
        ToSocketAddrsFuture::Resolving(task)
    }
}

impl ToSocketAddrs for str {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
        if let Ok(addr) = self.parse() {
            return ToSocketAddrsFuture::Ready(Ok(vec![addr].into_iter()));
        }

        let addr = self.to_string();
        let task = spawn_blocking(move || {
            std::net::ToSocketAddrs::to_socket_addrs(addr.as_str())
                .context(|| format!("could not resolve address `{:?}`", addr))
        });
        ToSocketAddrsFuture::Resolving(task)
    }
}

impl<'a> ToSocketAddrs for &'a [SocketAddr] {
    type Iter = std::iter::Cloned<std::slice::Iter<'a, SocketAddr>>;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
        ToSocketAddrsFuture::Ready(Ok(self.iter().cloned()))
    }
}

impl<T: ToSocketAddrs + ?Sized> ToSocketAddrs for &T {
    type Iter = T::Iter;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
        (**self).to_socket_addrs()
    }
}

impl ToSocketAddrs for String {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(
        &self,
    ) -> ret!(
        impl Future<Output = Self::Iter>,
        ToSocketAddrsFuture<Self::Iter>
    ) {
        (&**self).to_socket_addrs()
    }
}
