//! Connectors used by the `Client`.
//!
//! This module contains:
//!
//! - A default [`HttpConnector`][] that does DNS resolution and establishes
//!   connections over TCP.
//! - Types to build custom connectors.
//!
//! # Connectors
//!
//! A "connector" is a [`Service`][] that takes a [`Uri`][] destination, and
//! its `Response` is some type implementing [`AsyncRead`][], [`AsyncWrite`][],
//! and [`Connection`][].
//!
//! ## Custom Connectors
//!
//! A simple connector that ignores the `Uri` destination and always returns
//! a TCP connection to the same address could be written like this:
//!
//! ```rust,ignore
//! let connector = tower::service_fn(|_dst| async {
//!     tokio::net::TcpStream::connect("127.0.0.1:1337")
//! })
//! ```
//!
//! Or, fully written out:
//!
//! ```
//! # #[cfg(feature = "runtime")]
//! # mod rt {
//! use std::{future::Future, net::SocketAddr, pin::Pin, task::{self, Poll}};
//! use hyper::{service::Service, Uri};
//! use tokio::net::TcpStream;
//!
//! #[derive(Clone)]
//! struct LocalConnector;
//!
//! impl Service<Uri> for LocalConnector {
//!     type Response = TcpStream;
//!     type Error = std::io::Error;
//!     // We can't "name" an `async` generated future.
//!     type Future = Pin<Box<
//!         dyn Future<Output = Result<Self::Response, Self::Error>> + Send
//!     >>;
//!
//!     fn poll_ready(&mut self, _: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
//!         // This connector is always ready, but others might not be.
//!         Poll::Ready(Ok(()))
//!     }
//!
//!     fn call(&mut self, _: Uri) -> Self::Future {
//!         Box::pin(TcpStream::connect(SocketAddr::from(([127, 0, 0, 1], 1337))))
//!     }
//! }
//! # }
//! ```
//!
//! It's worth noting that for `TcpStream`s, the [`HttpConnector`][] is a
//! better starting place to extend from.
//!
//! Using either of the above connector examples, it can be used with the
//! `Client` like this:
//!
//! ```
//! # #[cfg(feature = "runtime")]
//! # fn rt () {
//! # let connector = hyper::client::HttpConnector::new();
//! // let connector = ...
//!
//! let client = hyper::Client::builder()
//!     .build::<_, hyper::Body>(connector);
//! # }
//! ```
//!
//!
//! [`HttpConnector`]: HttpConnector
//! [`Service`]: crate::service::Service
//! [`Uri`]: ::http::Uri
//! [`AsyncRead`]: tokio::io::AsyncRead
//! [`AsyncWrite`]: tokio::io::AsyncWrite
//! [`Connection`]: Connection
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use ::http::Extensions;
use tokio::sync::watch;

cfg_feature! {
    #![feature = "tcp"]

    pub use self::http::{HttpConnector, HttpInfo};

    pub mod dns;
    mod http;
}

cfg_feature! {
    #![any(feature = "http1", feature = "http2")]

    pub use self::sealed::Connect;
}

/// Describes a type returned by a connector.
pub trait Connection {
    /// Return metadata describing the connection.
    fn connected(&self) -> Connected;
}

/// Extra information about the connected transport.
///
/// This can be used to inform recipients about things like if ALPN
/// was used, or if connected to an HTTP proxy.
#[derive(Debug)]
pub struct Connected {
    pub(super) alpn: Alpn,
    pub(super) is_proxied: bool,
    pub(super) extra: Option<Extra>,
    pub(super) poisoned: PoisonPill,
}

#[derive(Clone)]
pub(crate) struct PoisonPill {
    poisoned: Arc<AtomicBool>,
}

impl Debug for PoisonPill {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // print the address of the pill—this makes debugging issues much easier
        write!(
            f,
            "PoisonPill@{:p} {{ poisoned: {} }}",
            self.poisoned,
            self.poisoned.load(Ordering::Relaxed)
        )
    }
}

impl PoisonPill {
    pub(crate) fn healthy() -> Self {
        Self {
            poisoned: Arc::new(AtomicBool::new(false)),
        }
    }
    pub(crate) fn poison(&self) {
        self.poisoned.store(true, Ordering::Relaxed)
    }

    pub(crate) fn poisoned(&self) -> bool {
        self.poisoned.load(Ordering::Relaxed)
    }
}

/// [`CaptureConnection`] allows callers to capture [`Connected`] information
///
/// To capture a connection for a request, use [`capture_connection`].
#[derive(Debug, Clone)]
pub struct CaptureConnection {
    rx: watch::Receiver<Option<Connected>>,
}

/// Capture the connection for a given request
///
/// When making a request with Hyper, the underlying connection must implement the [`Connection`] trait.
/// [`capture_connection`] allows a caller to capture the returned [`Connected`] structure as soon
/// as the connection is established.
///
/// *Note*: If establishing a connection fails, [`CaptureConnection::connection_metadata`] will always return none.
///
/// # Examples
///
/// **Synchronous access**:
/// The [`CaptureConnection::connection_metadata`] method allows callers to check if a connection has been
/// established. This is ideal for situations where you are certain the connection has already
/// been established (e.g. after the response future has already completed).
/// ```rust
/// use hyper::client::connect::{capture_connection, CaptureConnection};
/// let mut request = http::Request::builder()
///   .uri("http://foo.com")
///   .body(())
///   .unwrap();
///
/// let captured_connection = capture_connection(&mut request);
/// // some time later after the request has been sent...
/// let connection_info = captured_connection.connection_metadata();
/// println!("we are connected! {:?}", connection_info.as_ref());
/// ```
///
/// **Asynchronous access**:
/// The [`CaptureConnection::wait_for_connection_metadata`] method returns a future resolves as soon as the
/// connection is available.
///
/// ```rust
/// # #[cfg(feature  = "runtime")]
/// # async fn example() {
/// use hyper::client::connect::{capture_connection, CaptureConnection};
/// let mut request = http::Request::builder()
///   .uri("http://foo.com")
///   .body(hyper::Body::empty())
///   .unwrap();
///
/// let mut captured = capture_connection(&mut request);
/// tokio::task::spawn(async move {
///     let connection_info = captured.wait_for_connection_metadata().await;
///     println!("we are connected! {:?}", connection_info.as_ref());
/// });
///
/// let client = hyper::Client::new();
/// client.request(request).await.expect("request failed");
/// # }
/// ```
pub fn capture_connection<B>(request: &mut crate::http::Request<B>) -> CaptureConnection {
    let (tx, rx) = CaptureConnection::new();
    request.extensions_mut().insert(tx);
    rx
}

/// TxSide for [`CaptureConnection`]
///
/// This is inserted into `Extensions` to allow Hyper to back channel connection info
#[derive(Clone)]
pub(crate) struct CaptureConnectionExtension {
    tx: Arc<watch::Sender<Option<Connected>>>,
}

impl CaptureConnectionExtension {
    pub(crate) fn set(&self, connected: &Connected) {
        self.tx.send_replace(Some(connected.clone()));
    }
}

impl CaptureConnection {
    /// Internal API to create the tx and rx half of [`CaptureConnection`]
    pub(crate) fn new() -> (CaptureConnectionExtension, Self) {
        let (tx, rx) = watch::channel(None);
        (
            CaptureConnectionExtension { tx: Arc::new(tx) },
            CaptureConnection { rx },
        )
    }

    /// Retrieve the connection metadata, if available
    pub fn connection_metadata(&self) -> impl Deref<Target = Option<Connected>> + '_ {
        self.rx.borrow()
    }

    /// Wait for the connection to be established
    ///
    /// If a connection was established, this will always return `Some(...)`. If the request never
    /// successfully connected (e.g. DNS resolution failure), this method will never return.
    pub async fn wait_for_connection_metadata(
        &mut self,
    ) -> impl Deref<Target = Option<Connected>> + '_ {
        if self.rx.borrow().is_some() {
            return self.rx.borrow();
        }
        let _ = self.rx.changed().await;
        self.rx.borrow()
    }
}

pub(super) struct Extra(Box<dyn ExtraInner>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Alpn {
    H2,
    None,
}

impl Connected {
    /// Create new `Connected` type with empty metadata.
    pub fn new() -> Connected {
        Connected {
            alpn: Alpn::None,
            is_proxied: false,
            extra: None,
            poisoned: PoisonPill::healthy(),
        }
    }

    /// Set whether the connected transport is to an HTTP proxy.
    ///
    /// This setting will affect if HTTP/1 requests written on the transport
    /// will have the request-target in absolute-form or origin-form:
    ///
    /// - When `proxy(false)`:
    ///
    /// ```http
    /// GET /guide HTTP/1.1
    /// ```
    ///
    /// - When `proxy(true)`:
    ///
    /// ```http
    /// GET http://hyper.rs/guide HTTP/1.1
    /// ```
    ///
    /// Default is `false`.
    pub fn proxy(mut self, is_proxied: bool) -> Connected {
        self.is_proxied = is_proxied;
        self
    }

    /// Determines if the connected transport is to an HTTP proxy.
    pub fn is_proxied(&self) -> bool {
        self.is_proxied
    }

    /// Set extra connection information to be set in the extensions of every `Response`.
    pub fn extra<T: Clone + Send + Sync + 'static>(mut self, extra: T) -> Connected {
        if let Some(prev) = self.extra {
            self.extra = Some(Extra(Box::new(ExtraChain(prev.0, extra))));
        } else {
            self.extra = Some(Extra(Box::new(ExtraEnvelope(extra))));
        }
        self
    }

    /// Copies the extra connection information into an `Extensions` map.
    pub fn get_extras(&self, extensions: &mut Extensions) {
        if let Some(extra) = &self.extra {
            extra.set(extensions);
        }
    }

    /// Set that the connected transport negotiated HTTP/2 as its next protocol.
    pub fn negotiated_h2(mut self) -> Connected {
        self.alpn = Alpn::H2;
        self
    }

    /// Determines if the connected transport negotiated HTTP/2 as its next protocol.
    pub fn is_negotiated_h2(&self) -> bool {
        self.alpn == Alpn::H2
    }

    /// Poison this connection
    ///
    /// A poisoned connection will not be reused for subsequent requests by the pool
    pub fn poison(&self) {
        self.poisoned.poison();
        tracing::debug!(
            poison_pill = ?self.poisoned, "connection was poisoned"
        );
    }

    // Don't public expose that `Connected` is `Clone`, unsure if we want to
    // keep that contract...
    pub(super) fn clone(&self) -> Connected {
        Connected {
            alpn: self.alpn.clone(),
            is_proxied: self.is_proxied,
            extra: self.extra.clone(),
            poisoned: self.poisoned.clone(),
        }
    }
}

// ===== impl Extra =====

impl Extra {
    pub(super) fn set(&self, res: &mut Extensions) {
        self.0.set(res);
    }
}

impl Clone for Extra {
    fn clone(&self) -> Extra {
        Extra(self.0.clone_box())
    }
}

impl fmt::Debug for Extra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extra").finish()
    }
}

trait ExtraInner: Send + Sync {
    fn clone_box(&self) -> Box<dyn ExtraInner>;
    fn set(&self, res: &mut Extensions);
}

// This indirection allows the `Connected` to have a type-erased "extra" value,
// while that type still knows its inner extra type. This allows the correct
// TypeId to be used when inserting into `res.extensions_mut()`.
#[derive(Clone)]
struct ExtraEnvelope<T>(T);

impl<T> ExtraInner for ExtraEnvelope<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn ExtraInner> {
        Box::new(self.clone())
    }

    fn set(&self, res: &mut Extensions) {
        res.insert(self.0.clone());
    }
}

struct ExtraChain<T>(Box<dyn ExtraInner>, T);

impl<T: Clone> Clone for ExtraChain<T> {
    fn clone(&self) -> Self {
        ExtraChain(self.0.clone_box(), self.1.clone())
    }
}

impl<T> ExtraInner for ExtraChain<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn ExtraInner> {
        Box::new(self.clone())
    }

    fn set(&self, res: &mut Extensions) {
        self.0.set(res);
        res.insert(self.1.clone());
    }
}

#[cfg(any(feature = "http1", feature = "http2"))]
pub(super) mod sealed {
    use std::error::Error as StdError;
    use std::future::Future;
    use std::marker::Unpin;

    use ::http::Uri;
    use tokio::io::{AsyncRead, AsyncWrite};

    use super::Connection;

    /// Connect to a destination, returning an IO transport.
    ///
    /// A connector receives a [`Uri`](::http::Uri) and returns a `Future` of the
    /// ready connection.
    ///
    /// # Trait Alias
    ///
    /// This is really just an *alias* for the `tower::Service` trait, with
    /// additional bounds set for convenience *inside* hyper. You don't actually
    /// implement this trait, but `tower::Service<Uri>` instead.
    // The `Sized` bound is to prevent creating `dyn Connect`, since they cannot
    // fit the `Connect` bounds because of the blanket impl for `Service`.
    pub trait Connect: Sealed + Sized {
        #[doc(hidden)]
        type _Svc: ConnectSvc;
        #[doc(hidden)]
        fn connect(self, internal_only: Internal, dst: Uri) -> <Self::_Svc as ConnectSvc>::Future;
    }

    #[allow(unreachable_pub)]
    pub trait ConnectSvc {
        type Connection: AsyncRead + AsyncWrite + Connection + Unpin + Send + 'static;
        type Error: Into<Box<dyn StdError + Send + Sync>>;
        type Future: Future<Output = Result<Self::Connection, Self::Error>> + Unpin + Send + 'static;

        fn connect(self, internal_only: Internal, dst: Uri) -> Self::Future;
    }

    impl<S, T> Connect for S
    where
        S: tower_service::Service<Uri, Response = T> + Send + 'static,
        S::Error: Into<Box<dyn StdError + Send + Sync>>,
        S::Future: Unpin + Send,
        T: AsyncRead + AsyncWrite + Connection + Unpin + Send + 'static,
    {
        type _Svc = S;

        fn connect(self, _: Internal, dst: Uri) -> crate::service::Oneshot<S, Uri> {
            crate::service::oneshot(self, dst)
        }
    }

    impl<S, T> ConnectSvc for S
    where
        S: tower_service::Service<Uri, Response = T> + Send + 'static,
        S::Error: Into<Box<dyn StdError + Send + Sync>>,
        S::Future: Unpin + Send,
        T: AsyncRead + AsyncWrite + Connection + Unpin + Send + 'static,
    {
        type Connection = T;
        type Error = S::Error;
        type Future = crate::service::Oneshot<S, Uri>;

        fn connect(self, _: Internal, dst: Uri) -> Self::Future {
            crate::service::oneshot(self, dst)
        }
    }

    impl<S, T> Sealed for S
    where
        S: tower_service::Service<Uri, Response = T> + Send,
        S::Error: Into<Box<dyn StdError + Send + Sync>>,
        S::Future: Unpin + Send,
        T: AsyncRead + AsyncWrite + Connection + Unpin + Send + 'static,
    {
    }

    pub trait Sealed {}
    #[allow(missing_debug_implementations)]
    pub struct Internal;
}

#[cfg(test)]
mod tests {
    use super::Connected;
    use crate::client::connect::CaptureConnection;

    #[derive(Clone, Debug, PartialEq)]
    struct Ex1(usize);

    #[derive(Clone, Debug, PartialEq)]
    struct Ex2(&'static str);

    #[derive(Clone, Debug, PartialEq)]
    struct Ex3(&'static str);

    #[test]
    fn test_connected_extra() {
        let c1 = Connected::new().extra(Ex1(41));

        let mut ex = ::http::Extensions::new();

        assert_eq!(ex.get::<Ex1>(), None);

        c1.extra.as_ref().expect("c1 extra").set(&mut ex);

        assert_eq!(ex.get::<Ex1>(), Some(&Ex1(41)));
    }

    #[test]
    fn test_connected_extra_chain() {
        // If a user composes connectors and at each stage, there's "extra"
        // info to attach, it shouldn't override the previous extras.

        let c1 = Connected::new()
            .extra(Ex1(45))
            .extra(Ex2("zoom"))
            .extra(Ex3("pew pew"));

        let mut ex1 = ::http::Extensions::new();

        assert_eq!(ex1.get::<Ex1>(), None);
        assert_eq!(ex1.get::<Ex2>(), None);
        assert_eq!(ex1.get::<Ex3>(), None);

        c1.extra.as_ref().expect("c1 extra").set(&mut ex1);

        assert_eq!(ex1.get::<Ex1>(), Some(&Ex1(45)));
        assert_eq!(ex1.get::<Ex2>(), Some(&Ex2("zoom")));
        assert_eq!(ex1.get::<Ex3>(), Some(&Ex3("pew pew")));

        // Just like extensions, inserting the same type overrides previous type.
        let c2 = Connected::new()
            .extra(Ex1(33))
            .extra(Ex2("hiccup"))
            .extra(Ex1(99));

        let mut ex2 = ::http::Extensions::new();

        c2.extra.as_ref().expect("c2 extra").set(&mut ex2);

        assert_eq!(ex2.get::<Ex1>(), Some(&Ex1(99)));
        assert_eq!(ex2.get::<Ex2>(), Some(&Ex2("hiccup")));
    }

    #[test]
    fn test_sync_capture_connection() {
        let (tx, rx) = CaptureConnection::new();
        assert!(
            rx.connection_metadata().is_none(),
            "connection has not been set"
        );
        tx.set(&Connected::new().proxy(true));
        assert_eq!(
            rx.connection_metadata()
                .as_ref()
                .expect("connected should be set")
                .is_proxied(),
            true
        );

        // ensure it can be called multiple times
        assert_eq!(
            rx.connection_metadata()
                .as_ref()
                .expect("connected should be set")
                .is_proxied(),
            true
        );
    }

    #[tokio::test]
    async fn async_capture_connection() {
        let (tx, mut rx) = CaptureConnection::new();
        assert!(
            rx.connection_metadata().is_none(),
            "connection has not been set"
        );
        let test_task = tokio::spawn(async move {
            assert_eq!(
                rx.wait_for_connection_metadata()
                    .await
                    .as_ref()
                    .expect("connection should be set")
                    .is_proxied(),
                true
            );
            // can be awaited multiple times
            assert!(
                rx.wait_for_connection_metadata().await.is_some(),
                "should be awaitable multiple times"
            );

            assert_eq!(rx.connection_metadata().is_some(), true);
        });
        // can't be finished, we haven't set the connection yet
        assert_eq!(test_task.is_finished(), false);
        tx.set(&Connected::new().proxy(true));

        assert!(test_task.await.is_ok());
    }

    #[tokio::test]
    async fn capture_connection_sender_side_dropped() {
        let (tx, mut rx) = CaptureConnection::new();
        assert!(
            rx.connection_metadata().is_none(),
            "connection has not been set"
        );
        drop(tx);
        assert!(rx.wait_for_connection_metadata().await.is_none());
    }
}
