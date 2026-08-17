#[cfg(feature = "tls")]
use crate::tls::TlsConfigBuilder;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::future::Future;
use std::net::SocketAddr;
#[cfg(feature = "tls")]
use std::path::Path;

use futures_util::{future, FutureExt, TryFuture, TryStream, TryStreamExt};
use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server as HyperServer;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::Instrument;

use crate::filter::Filter;
use crate::reject::IsReject;
use crate::reply::Reply;
use crate::transport::Transport;

/// Create a `Server` with the provided `Filter`.
pub fn serve<F>(filter: F) -> Server<F>
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply,
    F::Error: IsReject,
{
    Server {
        pipeline: false,
        filter,
    }
}

/// A Warp Server ready to filter requests.
#[derive(Debug)]
pub struct Server<F> {
    pipeline: bool,
    filter: F,
}

/// A Warp Server ready to filter requests over TLS.
///
/// *This type requires the `"tls"` feature.*
#[cfg(feature = "tls")]
pub struct TlsServer<F> {
    server: Server<F>,
    tls: TlsConfigBuilder,
}

// Getting all various generic bounds to make this a re-usable method is
// very complicated, so instead this is just a macro.
macro_rules! into_service {
    ($into:expr) => {{
        let inner = crate::service($into);
        make_service_fn(move |transport| {
            let inner = inner.clone();
            let remote_addr = Transport::remote_addr(transport);
            future::ok::<_, Infallible>(service_fn(move |req| {
                inner.call_with_addr(req, remote_addr)
            }))
        })
    }};
}

macro_rules! addr_incoming {
    ($addr:expr) => {{
        let mut incoming = AddrIncoming::bind($addr)?;
        incoming.set_nodelay(true);
        let addr = incoming.local_addr();
        (addr, incoming)
    }};
}

macro_rules! bind_inner {
    ($this:ident, $addr:expr) => {{
        let service = into_service!($this.filter);
        let (addr, incoming) = addr_incoming!($addr);
        let srv = HyperServer::builder(incoming)
            .http1_pipeline_flush($this.pipeline)
            .serve(service);
        Ok::<_, hyper::Error>((addr, srv))
    }};

    (tls: $this:ident, $addr:expr) => {{
        let service = into_service!($this.server.filter);
        let (addr, incoming) = addr_incoming!($addr);
        let tls = $this.tls.build()?;
        let srv = HyperServer::builder(crate::tls::TlsAcceptor::new(tls, incoming))
            .http1_pipeline_flush($this.server.pipeline)
            .serve(service);
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>((addr, srv))
    }};
}

macro_rules! bind {
    ($this:ident, $addr:expr) => {{
        let addr = $addr.into();
        (|addr| bind_inner!($this, addr))(&addr).unwrap_or_else(|e| {
            panic!("error binding to {}: {}", addr, e);
        })
    }};

    (tls: $this:ident, $addr:expr) => {{
        let addr = $addr.into();
        (|addr| bind_inner!(tls: $this, addr))(&addr).unwrap_or_else(|e| {
            panic!("error binding to {}: {}", addr, e);
        })
    }};
}

macro_rules! try_bind {
    ($this:ident, $addr:expr) => {{
        (|addr| bind_inner!($this, addr))($addr)
    }};

    (tls: $this:ident, $addr:expr) => {{
        (|addr| bind_inner!(tls: $this, addr))($addr)
    }};
}

// ===== impl Server =====

impl<F> Server<F>
where
    F: Filter + Clone + Send + Sync + 'static,
    <F::Future as TryFuture>::Ok: Reply,
    <F::Future as TryFuture>::Error: IsReject,
{
    /// Run this `Server` forever on the current thread.
    ///
    /// # Panics
    ///
    /// Panics if we are unable to bind to the provided address.
    pub async fn run(self, addr: impl Into<SocketAddr>) {
        let (addr, fut) = self.bind_ephemeral(addr);
        let span = tracing::info_span!("Server::run", ?addr);
        tracing::info!(parent: &span, "listening on http://{}", addr);

        fut.instrument(span).await;
    }

    /// Run this `Server` forever on the current thread with a specific stream
    /// of incoming connections.
    ///
    /// This can be used for Unix Domain Sockets, or TLS, etc.
    pub async fn run_incoming<I>(self, incoming: I)
    where
        I: TryStream + Send,
        I::Ok: AsyncRead + AsyncWrite + Send + 'static + Unpin,
        I::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        self.run_incoming2(incoming.map_ok(crate::transport::LiftIo).into_stream())
            .instrument(tracing::info_span!("Server::run_incoming"))
            .await;
    }

    async fn run_incoming2<I>(self, incoming: I)
    where
        I: TryStream + Send,
        I::Ok: Transport + Send + 'static + Unpin,
        I::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        let fut = self.serve_incoming2(incoming);

        tracing::info!("listening with custom incoming");

        fut.await;
    }

    /// Bind to a socket address, returning a `Future` that can be
    /// executed on the current runtime.
    ///
    /// # Panics
    ///
    /// Panics if we are unable to bind to the provided address.
    pub fn bind(self, addr: impl Into<SocketAddr> + 'static) -> impl Future<Output = ()> + 'static {
        let (_, fut) = self.bind_ephemeral(addr);
        fut
    }

    /// Bind to a socket address, returning a `Future` that can be
    /// executed on any runtime.
    ///
    /// In case we are unable to bind to the specified address, resolves to an
    /// error and logs the reason.
    pub async fn try_bind(self, addr: impl Into<SocketAddr>) {
        let addr = addr.into();
        let srv = match try_bind!(self, &addr) {
            Ok((_, srv)) => srv,
            Err(err) => {
                tracing::error!("error binding to {}: {}", addr, err);
                return;
            }
        };

        srv.map(|result| {
            if let Err(err) = result {
                tracing::error!("server error: {}", err)
            }
        })
        .await;
    }

    /// Bind to a possibly ephemeral socket address.
    ///
    /// Returns the bound address and a `Future` that can be executed on
    /// the current runtime.
    ///
    /// # Panics
    ///
    /// Panics if we are unable to bind to the provided address.
    pub fn bind_ephemeral(
        self,
        addr: impl Into<SocketAddr>,
    ) -> (SocketAddr, impl Future<Output = ()> + 'static) {
        let (addr, srv) = bind!(self, addr);
        let srv = srv.map(|result| {
            if let Err(err) = result {
                tracing::error!("server error: {}", err)
            }
        });

        (addr, srv)
    }

    /// Tried to bind a possibly ephemeral socket address.
    ///
    /// Returns a `Result` which fails in case we are unable to bind with the
    /// underlying error.
    ///
    /// Returns the bound address and a `Future` that can be executed on
    /// the current runtime.
    pub fn try_bind_ephemeral(
        self,
        addr: impl Into<SocketAddr>,
    ) -> Result<(SocketAddr, impl Future<Output = ()> + 'static), crate::Error> {
        let addr = addr.into();
        let (addr, srv) = try_bind!(self, &addr).map_err(crate::Error::new)?;
        let srv = srv.map(|result| {
            if let Err(err) = result {
                tracing::error!("server error: {}", err)
            }
        });

        Ok((addr, srv))
    }

    /// Create a server with graceful shutdown signal.
    ///
    /// When the signal completes, the server will start the graceful shutdown
    /// process.
    ///
    /// Returns the bound address and a `Future` that can be executed on
    /// the current runtime.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use warp::Filter;
    /// use futures_util::future::TryFutureExt;
    /// use tokio::sync::oneshot;
    ///
    /// # fn main() {
    /// let routes = warp::any()
    ///     .map(|| "Hello, World!");
    ///
    /// let (tx, rx) = oneshot::channel();
    ///
    /// let (addr, server) = warp::serve(routes)
    ///     .bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async {
    ///          rx.await.ok();
    ///     });
    ///
    /// // Spawn the server into a runtime
    /// tokio::task::spawn(server);
    ///
    /// // Later, start the shutdown...
    /// let _ = tx.send(());
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if we are unable to bind to the provided address.
    pub fn bind_with_graceful_shutdown(
        self,
        addr: impl Into<SocketAddr> + 'static,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> (SocketAddr, impl Future<Output = ()> + 'static) {
        let (addr, srv) = bind!(self, addr);
        let fut = srv.with_graceful_shutdown(signal).map(|result| {
            if let Err(err) = result {
                tracing::error!("server error: {}", err)
            }
        });
        (addr, fut)
    }

    /// Create a server with graceful shutdown signal.
    ///
    /// When the signal completes, the server will start the graceful shutdown
    /// process.
    pub fn try_bind_with_graceful_shutdown(
        self,
        addr: impl Into<SocketAddr> + 'static,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(SocketAddr, impl Future<Output = ()> + 'static), crate::Error> {
        let addr = addr.into();
        let (addr, srv) = try_bind!(self, &addr).map_err(crate::Error::new)?;
        let srv = srv.with_graceful_shutdown(signal).map(|result| {
            if let Err(err) = result {
                tracing::error!("server error: {}", err)
            }
        });

        Ok((addr, srv))
    }

    /// Setup this `Server` with a specific stream of incoming connections.
    ///
    /// This can be used for Unix Domain Sockets, or TLS, etc.
    ///
    /// Returns a `Future` that can be executed on the current runtime.
    pub fn serve_incoming<I>(self, incoming: I) -> impl Future<Output = ()>
    where
        I: TryStream + Send,
        I::Ok: AsyncRead + AsyncWrite + Send + 'static + Unpin,
        I::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        let incoming = incoming.map_ok(crate::transport::LiftIo);
        self.serve_incoming2(incoming)
            .instrument(tracing::info_span!("Server::serve_incoming"))
    }

    /// Setup this `Server` with a specific stream of incoming connections and a
    /// signal to initiate graceful shutdown.
    ///
    /// This can be used for Unix Domain Sockets, or TLS, etc.
    ///
    /// When the signal completes, the server will start the graceful shutdown
    /// process.
    ///
    /// Returns a `Future` that can be executed on the current runtime.
    pub fn serve_incoming_with_graceful_shutdown<I>(
        self,
        incoming: I,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> impl Future<Output = ()>
    where
        I: TryStream + Send,
        I::Ok: AsyncRead + AsyncWrite + Send + 'static + Unpin,
        I::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        let incoming = incoming.map_ok(crate::transport::LiftIo);
        let service = into_service!(self.filter);
        let pipeline = self.pipeline;

        async move {
            let srv =
                HyperServer::builder(hyper::server::accept::from_stream(incoming.into_stream()))
                    .http1_pipeline_flush(pipeline)
                    .serve(service)
                    .with_graceful_shutdown(signal)
                    .await;

            if let Err(err) = srv {
                tracing::error!("server error: {}", err);
            }
        }
        .instrument(tracing::info_span!(
            "Server::serve_incoming_with_graceful_shutdown"
        ))
    }

    async fn serve_incoming2<I>(self, incoming: I)
    where
        I: TryStream + Send,
        I::Ok: Transport + Send + 'static + Unpin,
        I::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        let service = into_service!(self.filter);

        let srv = HyperServer::builder(hyper::server::accept::from_stream(incoming.into_stream()))
            .http1_pipeline_flush(self.pipeline)
            .serve(service)
            .await;

        if let Err(err) = srv {
            tracing::error!("server error: {}", err);
        }
    }

    // Generally shouldn't be used, as it can slow down non-pipelined responses.
    //
    // It's only real use is to make silly pipeline benchmarks look better.
    #[doc(hidden)]
    pub fn unstable_pipeline(mut self) -> Self {
        self.pipeline = true;
        self
    }

    /// Configure a server to use TLS.
    ///
    /// *This function requires the `"tls"` feature.*
    #[cfg(feature = "tls")]
    pub fn tls(self) -> TlsServer<F> {
        TlsServer {
            server: self,
            tls: TlsConfigBuilder::new(),
        }
    }
}

// // ===== impl TlsServer =====

#[cfg(feature = "tls")]
impl<F> TlsServer<F>
where
    F: Filter + Clone + Send + Sync + 'static,
    <F::Future as TryFuture>::Ok: Reply,
    <F::Future as TryFuture>::Error: IsReject,
{
    // TLS config methods

    /// Specify the file path to read the private key.
    ///
    /// *This function requires the `"tls"` feature.*
    pub fn key_path(self, path: impl AsRef<Path>) -> Self {
        self.with_tls(|tls| tls.key_path(path))
    }

    /// Specify the file path to read the certificate.
    ///
    /// *This function requires the `"tls"` feature.*
    pub fn cert_path(self, path: impl AsRef<Path>) -> Self {
        self.with_tls(|tls| tls.cert_path(path))
    }

    /// Specify the file path to read the trust anchor for optional client authentication.
    ///
    /// Anonymous and authenticated clients will be accepted. If no trust anchor is provided by any
    /// of the `client_auth_` methods, then client authentication is disabled by default.
    ///
    /// *This function requires the `"tls"` feature.*
    pub fn client_auth_optional_path(self, path: impl AsRef<Path>) -> Self {
        self.with_tls(|tls| tls.client_auth_optional_path(path))
    }

    /// Specify the file path to read the trust anchor for required client authentication.
    ///
    /// Only authenticated clients will be accepted. If no trust anchor is provided by any of the
    /// `client_auth_` methods, then client authentication is disabled by default.
    ///
    /// *This function requires the `"tls"` feature.*
    pub fn client_auth_required_path(self, path: impl AsRef<Path>) -> Self {
        self.with_tls(|tls| tls.client_auth_required_path(path))
    }

    /// Specify the in-memory contents of the private key.
    ///
    /// *This function requires the `"tls"` feature.*
    pub fn key(self, key: impl AsRef<[u8]>) -> Self {
        self.with_tls(|tls| tls.key(key.as_ref()))
    }

    /// Specify the in-memory contents of the certificate.
    ///
    /// *This function requires the `"tls"` feature.*
    pub fn cert(self, cert: impl AsRef<[u8]>) -> Self {
        self.with_tls(|tls| tls.cert(cert.as_ref()))
    }

    /// Specify the in-memory contents of the trust anchor for optional client authentication.
    ///
    /// Anonymous and authenticated clients will be accepted. If no trust anchor is provided by any
    /// of the `client_auth_` methods, then client authentication is disabled by default.
    ///
    /// *This function requires the `"tls"` feature.*
    pub fn client_auth_optional(self, trust_anchor: impl AsRef<[u8]>) -> Self {
        self.with_tls(|tls| tls.client_auth_optional(trust_anchor.as_ref()))
    }

    /// Specify the in-memory contents of the trust anchor for required client authentication.
    ///
    /// Only authenticated clients will be accepted. If no trust anchor is provided by any of the
    /// `client_auth_` methods, then client authentication is disabled by default.
    ///
    /// *This function requires the `"tls"` feature.*
    pub fn client_auth_required(self, trust_anchor: impl AsRef<[u8]>) -> Self {
        self.with_tls(|tls| tls.client_auth_required(trust_anchor.as_ref()))
    }

    /// Specify the DER-encoded OCSP response.
    ///
    /// *This function requires the `"tls"` feature.*
    pub fn ocsp_resp(self, resp: impl AsRef<[u8]>) -> Self {
        self.with_tls(|tls| tls.ocsp_resp(resp.as_ref()))
    }

    fn with_tls<Func>(self, func: Func) -> Self
    where
        Func: FnOnce(TlsConfigBuilder) -> TlsConfigBuilder,
    {
        let TlsServer { server, tls } = self;
        let tls = func(tls);
        TlsServer { server, tls }
    }

    // Server run methods

    /// Run this `TlsServer` forever on the current thread.
    ///
    /// *This function requires the `"tls"` feature.*
    pub async fn run(self, addr: impl Into<SocketAddr>) {
        let (addr, fut) = self.bind_ephemeral(addr);
        let span = tracing::info_span!("TlsServer::run", %addr);
        tracing::info!(parent: &span, "listening on https://{}", addr);

        fut.instrument(span).await;
    }

    /// Bind to a socket address, returning a `Future` that can be
    /// executed on a runtime.
    ///
    /// *This function requires the `"tls"` feature.*
    ///
    /// # Panics
    ///
    /// Panics if we are unable to bind to the provided address.
    pub async fn bind(self, addr: impl Into<SocketAddr>) {
        let (_, fut) = self.bind_ephemeral(addr);
        fut.await;
    }

    /// Bind to a possibly ephemeral socket address.
    ///
    /// Returns the bound address and a `Future` that can be executed on
    /// the current runtime.
    ///
    /// *This function requires the `"tls"` feature.*
    ///
    /// # Panics
    ///
    /// Panics if we are unable to bind to the provided address.
    pub fn bind_ephemeral(
        self,
        addr: impl Into<SocketAddr>,
    ) -> (SocketAddr, impl Future<Output = ()> + 'static) {
        let (addr, srv) = bind!(tls: self, addr);
        let srv = srv.map(|result| {
            if let Err(err) = result {
                tracing::error!("server error: {}", err)
            }
        });

        (addr, srv)
    }

    /// Create a server with graceful shutdown signal.
    ///
    /// When the signal completes, the server will start the graceful shutdown
    /// process.
    ///
    /// *This function requires the `"tls"` feature.*
    ///
    /// # Panics
    ///
    /// Panics if we are unable to bind to the provided address.
    pub fn bind_with_graceful_shutdown(
        self,
        addr: impl Into<SocketAddr> + 'static,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> (SocketAddr, impl Future<Output = ()> + 'static) {
        let (addr, srv) = bind!(tls: self, addr);

        let fut = srv.with_graceful_shutdown(signal).map(|result| {
            if let Err(err) = result {
                tracing::error!("server error: {}", err)
            }
        });
        (addr, fut)
    }

    /// Create a server with graceful shutdown signal.
    ///
    /// When the signal completes, the server will start the graceful shutdown
    /// process.
    ///
    /// *This function requires the `"tls"` feature.*
    pub fn try_bind_with_graceful_shutdown(
        self,
        addr: impl Into<SocketAddr> + 'static,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(SocketAddr, impl Future<Output = ()> + 'static), crate::Error> {
        let addr = addr.into();
        let (addr, srv) = try_bind!(tls: self, &addr).map_err(crate::Error::new)?;
        let srv = srv.with_graceful_shutdown(signal).map(|result| {
            if let Err(err) = result {
                tracing::error!("server error: {}", err)
            }
        });

        Ok((addr, srv))
    }
}

#[cfg(feature = "tls")]
impl<F> ::std::fmt::Debug for TlsServer<F>
where
    F: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct("TlsServer")
            .field("server", &self.server)
            .finish()
    }
}
