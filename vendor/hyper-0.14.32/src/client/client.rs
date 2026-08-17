use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::marker::Unpin;
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_channel::oneshot;
use futures_util::future::{self, Either, FutureExt as _, TryFutureExt as _};
use http::header::{HeaderValue, HOST};
use http::uri::{Port, Scheme};
use http::{Method, Request, Response, Uri, Version};
use tracing::{debug, trace, warn};

use crate::body::{Body, HttpBody};
use crate::client::connect::CaptureConnectionExtension;
use crate::common::{exec::BoxSendFuture, lazy as hyper_lazy, sync_wrapper::SyncWrapper, Lazy};
#[cfg(feature = "http2")]
use crate::ext::Protocol;
use crate::rt::Executor;

use super::conn;
use super::connect::{self, sealed::Connect, Alpn, Connected, Connection};
use super::pool::{
    self, CheckoutIsClosedError, Key as PoolKey, Pool, Poolable, Pooled, Reservation,
};
#[cfg(feature = "tcp")]
use super::HttpConnector;

/// A Client to make outgoing HTTP requests.
///
/// `Client` is cheap to clone and cloning is the recommended way to share a `Client`. The
/// underlying connection pool will be reused.
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
pub struct Client<C, B = Body> {
    config: Config,
    #[cfg_attr(feature = "deprecated", allow(deprecated))]
    conn_builder: conn::Builder,
    connector: C,
    pool: Pool<PoolClient<B>>,
}

#[derive(Clone, Copy, Debug)]
struct Config {
    retry_canceled_requests: bool,
    set_host: bool,
    ver: Ver,
}

/// A `Future` that will resolve to an HTTP Response.
///
/// This is returned by `Client::request` (and `Client::get`).
#[must_use = "futures do nothing unless polled"]
pub struct ResponseFuture {
    inner: SyncWrapper<Pin<Box<dyn Future<Output = crate::Result<Response<Body>>> + Send>>>,
}

// ===== impl Client =====

#[cfg(feature = "tcp")]
impl Client<HttpConnector, Body> {
    /// Create a new Client with the default [config](Builder).
    ///
    /// # Note
    ///
    /// The default connector does **not** handle TLS. Speaking to `https`
    /// destinations will require [configuring a connector that implements
    /// TLS](https://hyper.rs/guides/client/configuration).
    #[cfg_attr(docsrs, doc(cfg(feature = "tcp")))]
    #[inline]
    pub fn new() -> Client<HttpConnector, Body> {
        Builder::default().build_http()
    }
}

#[cfg(feature = "tcp")]
impl Default for Client<HttpConnector, Body> {
    fn default() -> Client<HttpConnector, Body> {
        Client::new()
    }
}

impl Client<(), Body> {
    /// Create a builder to configure a new `Client`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature  = "runtime")]
    /// # fn run () {
    /// use std::time::Duration;
    /// use hyper::Client;
    ///
    /// let client = Client::builder()
    ///     .pool_idle_timeout(Duration::from_secs(30))
    ///     .http2_only(true)
    ///     .build_http();
    /// # let infer: Client<_, hyper::Body> = client;
    /// # drop(infer);
    /// # }
    /// # fn main() {}
    /// ```
    #[inline]
    pub fn builder() -> Builder {
        Builder::default()
    }
}

impl<C, B> Client<C, B>
where
    C: Connect + Clone + Send + Sync + 'static,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    /// Send a `GET` request to the supplied `Uri`.
    ///
    /// # Note
    ///
    /// This requires that the `HttpBody` type have a `Default` implementation.
    /// It *should* return an "empty" version of itself, such that
    /// `HttpBody::is_end_stream` is `true`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature  = "runtime")]
    /// # fn run () {
    /// use hyper::{Client, Uri};
    ///
    /// let client = Client::new();
    ///
    /// let future = client.get(Uri::from_static("http://httpbin.org/ip"));
    /// # }
    /// # fn main() {}
    /// ```
    pub fn get(&self, uri: Uri) -> ResponseFuture
    where
        B: Default,
    {
        let body = B::default();
        if !body.is_end_stream() {
            warn!("default HttpBody used for get() does not return true for is_end_stream");
        }

        let mut req = Request::new(body);
        *req.uri_mut() = uri;
        self.request(req)
    }

    /// Send a constructed `Request` using this `Client`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(feature  = "runtime")]
    /// # fn run () {
    /// use hyper::{Body, Method, Client, Request};
    ///
    /// let client = Client::new();
    ///
    /// let req = Request::builder()
    ///     .method(Method::POST)
    ///     .uri("http://httpbin.org/post")
    ///     .body(Body::from("Hallo!"))
    ///     .expect("request builder");
    ///
    /// let future = client.request(req);
    /// # }
    /// # fn main() {}
    /// ```
    pub fn request(&self, mut req: Request<B>) -> ResponseFuture {
        let is_http_connect = req.method() == Method::CONNECT;
        match req.version() {
            Version::HTTP_11 => (),
            Version::HTTP_10 => {
                if is_http_connect {
                    warn!("CONNECT is not allowed for HTTP/1.0");
                    return ResponseFuture::new(future::err(
                        crate::Error::new_user_unsupported_request_method(),
                    ));
                }
            }
            Version::HTTP_2 => (),
            // completely unsupported HTTP version (like HTTP/0.9)!
            other => return ResponseFuture::error_version(other),
        };

        let pool_key = match extract_domain(req.uri_mut(), is_http_connect) {
            Ok(s) => s,
            Err(err) => {
                return ResponseFuture::new(future::err(err));
            }
        };

        ResponseFuture::new(self.clone().retryably_send_request(req, pool_key))
    }

    async fn retryably_send_request(
        self,
        mut req: Request<B>,
        pool_key: PoolKey,
    ) -> crate::Result<Response<Body>> {
        let uri = req.uri().clone();

        loop {
            req = match self.send_request(req, pool_key.clone()).await {
                Ok(resp) => return Ok(resp),
                Err(ClientError::Normal(err)) => return Err(err),
                Err(ClientError::Canceled {
                    connection_reused,
                    mut req,
                    reason,
                }) => {
                    if !self.config.retry_canceled_requests || !connection_reused {
                        // if client disabled, don't retry
                        // a fresh connection means we definitely can't retry
                        return Err(reason);
                    }

                    trace!(
                        "unstarted request canceled, trying again (reason={:?})",
                        reason
                    );
                    *req.uri_mut() = uri.clone();
                    req
                }
            }
        }
    }

    async fn send_request(
        &self,
        mut req: Request<B>,
        pool_key: PoolKey,
    ) -> Result<Response<Body>, ClientError<B>> {
        let mut pooled = match self.connection_for(pool_key).await {
            Ok(pooled) => pooled,
            Err(ClientConnectError::Normal(err)) => return Err(ClientError::Normal(err)),
            Err(ClientConnectError::H2CheckoutIsClosed(reason)) => {
                return Err(ClientError::Canceled {
                    connection_reused: true,
                    req,
                    reason,
                })
            }
        };
        req.extensions_mut()
            .get_mut::<CaptureConnectionExtension>()
            .map(|conn| conn.set(&pooled.conn_info));
        if pooled.is_http1() {
            if req.version() == Version::HTTP_2 {
                warn!("Connection is HTTP/1, but request requires HTTP/2");
                return Err(ClientError::Normal(
                    crate::Error::new_user_unsupported_version()
                        .with_client_connect_info(pooled.conn_info.clone()),
                ));
            }

            if self.config.set_host {
                let uri = req.uri().clone();
                req.headers_mut().entry(HOST).or_insert_with(|| {
                    let hostname = uri.host().expect("authority implies host");
                    if let Some(port) = get_non_default_port(&uri) {
                        let s = format!("{}:{}", hostname, port);
                        HeaderValue::from_str(&s)
                    } else {
                        HeaderValue::from_str(hostname)
                    }
                    .expect("uri host is valid header value")
                });
            }

            // CONNECT always sends authority-form, so check it first...
            if req.method() == Method::CONNECT {
                authority_form(req.uri_mut());
            } else if pooled.conn_info.is_proxied {
                absolute_form(req.uri_mut());
            } else {
                origin_form(req.uri_mut());
            }
        } else if req.method() == Method::CONNECT {
            #[cfg(not(feature = "http2"))]
            authority_form(req.uri_mut());

            #[cfg(feature = "http2")]
            if req.extensions().get::<Protocol>().is_none() {
                authority_form(req.uri_mut());
            }
        }

        let mut res = match pooled.send_request_retryable(req).await {
            Err((err, orig_req)) => {
                return Err(ClientError::map_with_reused(pooled.is_reused())((
                    err.with_client_connect_info(pooled.conn_info.clone()),
                    orig_req,
                )));
            }
            Ok(res) => res,
        };

        // If the Connector included 'extra' info, add to Response...
        if let Some(extra) = &pooled.conn_info.extra {
            extra.set(res.extensions_mut());
        }

        // As of futures@0.1.21, there is a race condition in the mpsc
        // channel, such that sending when the receiver is closing can
        // result in the message being stuck inside the queue. It won't
        // ever notify until the Sender side is dropped.
        //
        // To counteract this, we must check if our senders 'want' channel
        // has been closed after having tried to send. If so, error out...
        if pooled.is_closed() {
            return Ok(res);
        }

        // If pooled is HTTP/2, we can toss this reference immediately.
        //
        // when pooled is dropped, it will try to insert back into the
        // pool. To delay that, spawn a future that completes once the
        // sender is ready again.
        //
        // This *should* only be once the related `Connection` has polled
        // for a new request to start.
        //
        // It won't be ready if there is a body to stream.
        if pooled.is_http2() || !pooled.is_pool_enabled() || pooled.is_ready() {
            drop(pooled);
        } else if !res.body().is_end_stream() {
            let (delayed_tx, delayed_rx) = oneshot::channel();
            res.body_mut().delayed_eof(delayed_rx);
            let on_idle = future::poll_fn(move |cx| pooled.poll_ready(cx)).map(move |_| {
                // At this point, `pooled` is dropped, and had a chance
                // to insert into the pool (if conn was idle)
                drop(delayed_tx);
            });

            #[cfg_attr(feature = "deprecated", allow(deprecated))]
            self.conn_builder.exec.execute(on_idle);
        } else {
            // There's no body to delay, but the connection isn't
            // ready yet. Only re-insert when it's ready
            let on_idle = future::poll_fn(move |cx| pooled.poll_ready(cx)).map(|_| ());

            #[cfg_attr(feature = "deprecated", allow(deprecated))]
            self.conn_builder.exec.execute(on_idle);
        }

        Ok(res)
    }

    async fn connection_for(
        &self,
        pool_key: PoolKey,
    ) -> Result<Pooled<PoolClient<B>>, ClientConnectError> {
        // This actually races 2 different futures to try to get a ready
        // connection the fastest, and to reduce connection churn.
        //
        // - If the pool has an idle connection waiting, that's used
        //   immediately.
        // - Otherwise, the Connector is asked to start connecting to
        //   the destination Uri.
        // - Meanwhile, the pool Checkout is watching to see if any other
        //   request finishes and tries to insert an idle connection.
        // - If a new connection is started, but the Checkout wins after
        //   (an idle connection became available first), the started
        //   connection future is spawned into the runtime to complete,
        //   and then be inserted into the pool as an idle connection.
        let checkout = self.pool.checkout(pool_key.clone());
        let connect = self.connect_to(pool_key);
        let is_ver_h2 = self.config.ver == Ver::Http2;

        // The order of the `select` is depended on below...

        match future::select(checkout, connect).await {
            // Checkout won, connect future may have been started or not.
            //
            // If it has, let it finish and insert back into the pool,
            // so as to not waste the socket...
            Either::Left((Ok(checked_out), connecting)) => {
                // This depends on the `select` above having the correct
                // order, such that if the checkout future were ready
                // immediately, the connect future will never have been
                // started.
                //
                // If it *wasn't* ready yet, then the connect future will
                // have been started...
                if connecting.started() {
                    let bg = connecting
                        .map_err(|err| {
                            trace!("background connect error: {}", err);
                        })
                        .map(|_pooled| {
                            // dropping here should just place it in
                            // the Pool for us...
                        });
                    // An execute error here isn't important, we're just trying
                    // to prevent a waste of a socket...
                    #[cfg_attr(feature = "deprecated", allow(deprecated))]
                    self.conn_builder.exec.execute(bg);
                }
                Ok(checked_out)
            }
            // Connect won, checkout can just be dropped.
            Either::Right((Ok(connected), _checkout)) => Ok(connected),
            // Either checkout or connect could get canceled:
            //
            // 1. Connect is canceled if this is HTTP/2 and there is
            //    an outstanding HTTP/2 connecting task.
            // 2. Checkout is canceled if the pool cannot deliver an
            //    idle connection reliably.
            //
            // In both cases, we should just wait for the other future.
            Either::Left((Err(err), connecting)) => {
                if err.is_canceled() {
                    connecting.await.map_err(ClientConnectError::Normal)
                } else {
                    Err(ClientConnectError::Normal(err))
                }
            }
            Either::Right((Err(err), checkout)) => {
                if err.is_canceled() {
                    checkout.await.map_err(move |err| {
                        if is_ver_h2
                            && err.is_canceled()
                            && err.find_source::<CheckoutIsClosedError>().is_some()
                        {
                            ClientConnectError::H2CheckoutIsClosed(err)
                        } else {
                            ClientConnectError::Normal(err)
                        }
                    })
                } else {
                    Err(ClientConnectError::Normal(err))
                }
            }
        }
    }

    fn connect_to(
        &self,
        pool_key: PoolKey,
    ) -> impl Lazy<Output = crate::Result<Pooled<PoolClient<B>>>> + Unpin {
        #[cfg_attr(feature = "deprecated", allow(deprecated))]
        let executor = self.conn_builder.exec.clone();
        let pool = self.pool.clone();
        #[cfg(not(feature = "http2"))]
        let conn_builder = self.conn_builder.clone();
        #[cfg(feature = "http2")]
        let mut conn_builder = self.conn_builder.clone();
        let ver = self.config.ver;
        let is_ver_h2 = ver == Ver::Http2;
        let connector = self.connector.clone();
        let dst = domain_as_uri(pool_key.clone());
        hyper_lazy(move || {
            // Try to take a "connecting lock".
            //
            // If the pool_key is for HTTP/2, and there is already a
            // connection being established, then this can't take a
            // second lock. The "connect_to" future is Canceled.
            let connecting = match pool.connecting(&pool_key, ver) {
                Some(lock) => lock,
                None => {
                    let canceled =
                        crate::Error::new_canceled().with("HTTP/2 connection in progress");
                    return Either::Right(future::err(canceled));
                }
            };
            Either::Left(
                connector
                    .connect(connect::sealed::Internal, dst)
                    .map_err(crate::Error::new_connect)
                    .and_then(move |io| {
                        let connected = io.connected();
                        // If ALPN is h2 and we aren't http2_only already,
                        // then we need to convert our pool checkout into
                        // a single HTTP2 one.
                        let connecting = if connected.alpn == Alpn::H2 && !is_ver_h2 {
                            match connecting.alpn_h2(&pool) {
                                Some(lock) => {
                                    trace!("ALPN negotiated h2, updating pool");
                                    lock
                                }
                                None => {
                                    // Another connection has already upgraded,
                                    // the pool checkout should finish up for us.
                                    let canceled = crate::Error::new_canceled()
                                        .with("ALPN upgraded to HTTP/2");
                                    return Either::Right(future::err(canceled));
                                }
                            }
                        } else {
                            connecting
                        };

                        #[cfg_attr(not(feature = "http2"), allow(unused))]
                        let is_h2 = is_ver_h2 || connected.alpn == Alpn::H2;
                        #[cfg(feature = "http2")]
                        {
                            conn_builder.http2_only(is_h2);
                        }

                        Either::Left(Box::pin(async move {
                            let (tx, conn) = conn_builder.handshake(io).await?;

                            trace!("handshake complete, spawning background dispatcher task");
                            executor.execute(
                                conn.map_err(|e| debug!("client connection error: {}", e))
                                    .map(|_| ()),
                            );

                            // Wait for 'conn' to ready up before we
                            // declare this tx as usable
                            let tx = tx.when_ready().await?;

                            let tx = {
                                #[cfg(feature = "http2")]
                                {
                                    if is_h2 {
                                        PoolTx::Http2(tx.into_http2())
                                    } else {
                                        PoolTx::Http1(tx)
                                    }
                                }
                                #[cfg(not(feature = "http2"))]
                                PoolTx::Http1(tx)
                            };

                            Ok(pool.pooled(
                                connecting,
                                PoolClient {
                                    conn_info: connected,
                                    tx,
                                },
                            ))
                        }))
                    }),
            )
        })
    }
}

impl<C, B> tower_service::Service<Request<B>> for Client<C, B>
where
    C: Connect + Clone + Send + Sync + 'static,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Response = Response<Body>;
    type Error = crate::Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        self.request(req)
    }
}

impl<C, B> tower_service::Service<Request<B>> for &'_ Client<C, B>
where
    C: Connect + Clone + Send + Sync + 'static,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Response = Response<Body>;
    type Error = crate::Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        self.request(req)
    }
}

impl<C: Clone, B> Clone for Client<C, B> {
    fn clone(&self) -> Client<C, B> {
        Client {
            config: self.config.clone(),
            conn_builder: self.conn_builder.clone(),
            connector: self.connector.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl<C, B> fmt::Debug for Client<C, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client").finish()
    }
}

// ===== impl ResponseFuture =====

impl ResponseFuture {
    fn new<F>(value: F) -> Self
    where
        F: Future<Output = crate::Result<Response<Body>>> + Send + 'static,
    {
        Self {
            inner: SyncWrapper::new(Box::pin(value)),
        }
    }

    fn error_version(ver: Version) -> Self {
        warn!("Request has unsupported version \"{:?}\"", ver);
        ResponseFuture::new(Box::pin(future::err(
            crate::Error::new_user_unsupported_version(),
        )))
    }
}

impl fmt::Debug for ResponseFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Future<Response>")
    }
}

impl Future for ResponseFuture {
    type Output = crate::Result<Response<Body>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.get_mut().as_mut().poll(cx)
    }
}

// ===== impl PoolClient =====

// FIXME: allow() required due to `impl Trait` leaking types to this lint
#[allow(missing_debug_implementations)]
struct PoolClient<B> {
    conn_info: Connected,
    tx: PoolTx<B>,
}

enum PoolTx<B> {
    #[cfg_attr(feature = "deprecated", allow(deprecated))]
    Http1(conn::SendRequest<B>),
    #[cfg(feature = "http2")]
    Http2(conn::Http2SendRequest<B>),
}

impl<B> PoolClient<B> {
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<()>> {
        match self.tx {
            PoolTx::Http1(ref mut tx) => tx.poll_ready(cx),
            #[cfg(feature = "http2")]
            PoolTx::Http2(_) => Poll::Ready(Ok(())),
        }
    }

    fn is_http1(&self) -> bool {
        !self.is_http2()
    }

    fn is_http2(&self) -> bool {
        match self.tx {
            PoolTx::Http1(_) => false,
            #[cfg(feature = "http2")]
            PoolTx::Http2(_) => true,
        }
    }

    fn is_ready(&self) -> bool {
        match self.tx {
            PoolTx::Http1(ref tx) => tx.is_ready(),
            #[cfg(feature = "http2")]
            PoolTx::Http2(ref tx) => tx.is_ready(),
        }
    }

    fn is_closed(&self) -> bool {
        match self.tx {
            PoolTx::Http1(ref tx) => tx.is_closed(),
            #[cfg(feature = "http2")]
            PoolTx::Http2(ref tx) => tx.is_closed(),
        }
    }
}

impl<B: HttpBody + 'static> PoolClient<B> {
    fn send_request_retryable(
        &mut self,
        req: Request<B>,
    ) -> impl Future<Output = Result<Response<Body>, (crate::Error, Option<Request<B>>)>>
    where
        B: Send,
    {
        match self.tx {
            #[cfg(not(feature = "http2"))]
            PoolTx::Http1(ref mut tx) => tx.send_request_retryable(req),
            #[cfg(feature = "http2")]
            PoolTx::Http1(ref mut tx) => Either::Left(tx.send_request_retryable(req)),
            #[cfg(feature = "http2")]
            PoolTx::Http2(ref mut tx) => Either::Right(tx.send_request_retryable(req)),
        }
    }
}

impl<B> Poolable for PoolClient<B>
where
    B: Send + 'static,
{
    fn is_open(&self) -> bool {
        if self.conn_info.poisoned.poisoned() {
            trace!(
                "marking {:?} as closed because it was poisoned",
                self.conn_info
            );
            return false;
        }
        match self.tx {
            PoolTx::Http1(ref tx) => tx.is_ready(),
            #[cfg(feature = "http2")]
            PoolTx::Http2(ref tx) => tx.is_ready(),
        }
    }

    fn reserve(self) -> Reservation<Self> {
        match self.tx {
            PoolTx::Http1(tx) => Reservation::Unique(PoolClient {
                conn_info: self.conn_info,
                tx: PoolTx::Http1(tx),
            }),
            #[cfg(feature = "http2")]
            PoolTx::Http2(tx) => {
                let b = PoolClient {
                    conn_info: self.conn_info.clone(),
                    tx: PoolTx::Http2(tx.clone()),
                };
                let a = PoolClient {
                    conn_info: self.conn_info,
                    tx: PoolTx::Http2(tx),
                };
                Reservation::Shared(a, b)
            }
        }
    }

    fn can_share(&self) -> bool {
        self.is_http2()
    }
}

// ===== impl ClientError =====

// FIXME: allow() required due to `impl Trait` leaking types to this lint
#[allow(missing_debug_implementations)]
enum ClientError<B> {
    Normal(crate::Error),
    Canceled {
        connection_reused: bool,
        req: Request<B>,
        reason: crate::Error,
    },
}

impl<B> ClientError<B> {
    fn map_with_reused(conn_reused: bool) -> impl Fn((crate::Error, Option<Request<B>>)) -> Self {
        move |(err, orig_req)| {
            if let Some(req) = orig_req {
                ClientError::Canceled {
                    connection_reused: conn_reused,
                    reason: err,
                    req,
                }
            } else {
                ClientError::Normal(err)
            }
        }
    }
}

enum ClientConnectError {
    Normal(crate::Error),
    H2CheckoutIsClosed(crate::Error),
}

/// A marker to identify what version a pooled connection is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum Ver {
    Auto,
    Http2,
}

fn origin_form(uri: &mut Uri) {
    let path = match uri.path_and_query() {
        Some(path) if path.as_str() != "/" => {
            let mut parts = ::http::uri::Parts::default();
            parts.path_and_query = Some(path.clone());
            Uri::from_parts(parts).expect("path is valid uri")
        }
        _none_or_just_slash => {
            debug_assert!(Uri::default() == "/");
            Uri::default()
        }
    };
    *uri = path
}

fn absolute_form(uri: &mut Uri) {
    debug_assert!(uri.scheme().is_some(), "absolute_form needs a scheme");
    debug_assert!(
        uri.authority().is_some(),
        "absolute_form needs an authority"
    );
    // If the URI is to HTTPS, and the connector claimed to be a proxy,
    // then it *should* have tunneled, and so we don't want to send
    // absolute-form in that case.
    if uri.scheme() == Some(&Scheme::HTTPS) {
        origin_form(uri);
    }
}

fn authority_form(uri: &mut Uri) {
    if let Some(path) = uri.path_and_query() {
        // `https://hyper.rs` would parse with `/` path, don't
        // annoy people about that...
        if path != "/" {
            warn!("HTTP/1.1 CONNECT request stripping path: {:?}", path);
        }
    }
    *uri = match uri.authority() {
        Some(auth) => {
            let mut parts = ::http::uri::Parts::default();
            parts.authority = Some(auth.clone());
            Uri::from_parts(parts).expect("authority is valid")
        }
        None => {
            unreachable!("authority_form with relative uri");
        }
    };
}

fn extract_domain(uri: &mut Uri, is_http_connect: bool) -> crate::Result<PoolKey> {
    let uri_clone = uri.clone();
    match (uri_clone.scheme(), uri_clone.authority()) {
        (Some(scheme), Some(auth)) => Ok((scheme.clone(), auth.clone())),
        (None, Some(auth)) if is_http_connect => {
            let scheme = match auth.port_u16() {
                Some(443) => {
                    set_scheme(uri, Scheme::HTTPS);
                    Scheme::HTTPS
                }
                _ => {
                    set_scheme(uri, Scheme::HTTP);
                    Scheme::HTTP
                }
            };
            Ok((scheme, auth.clone()))
        }
        _ => {
            debug!("Client requires absolute-form URIs, received: {:?}", uri);
            Err(crate::Error::new_user_absolute_uri_required())
        }
    }
}

fn domain_as_uri((scheme, auth): PoolKey) -> Uri {
    http::uri::Builder::new()
        .scheme(scheme)
        .authority(auth)
        .path_and_query("/")
        .build()
        .expect("domain is valid Uri")
}

fn set_scheme(uri: &mut Uri, scheme: Scheme) {
    debug_assert!(
        uri.scheme().is_none(),
        "set_scheme expects no existing scheme"
    );
    let old = mem::replace(uri, Uri::default());
    let mut parts: ::http::uri::Parts = old.into();
    parts.scheme = Some(scheme);
    parts.path_and_query = Some("/".parse().expect("slash is a valid path"));
    *uri = Uri::from_parts(parts).expect("scheme is valid");
}

fn get_non_default_port(uri: &Uri) -> Option<Port<&str>> {
    match (uri.port().map(|p| p.as_u16()), is_schema_secure(uri)) {
        (Some(443), true) => None,
        (Some(80), false) => None,
        _ => uri.port(),
    }
}

fn is_schema_secure(uri: &Uri) -> bool {
    uri.scheme_str()
        .map(|scheme_str| matches!(scheme_str, "wss" | "https"))
        .unwrap_or_default()
}

/// A builder to configure a new [`Client`](Client).
///
/// # Example
///
/// ```
/// # #[cfg(feature  = "runtime")]
/// # fn run () {
/// use std::time::Duration;
/// use hyper::Client;
///
/// let client = Client::builder()
///     .pool_idle_timeout(Duration::from_secs(30))
///     .http2_only(true)
///     .build_http();
/// # let infer: Client<_, hyper::Body> = client;
/// # drop(infer);
/// # }
/// # fn main() {}
/// ```
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
#[derive(Clone)]
pub struct Builder {
    client_config: Config,
    #[cfg_attr(feature = "deprecated", allow(deprecated))]
    conn_builder: conn::Builder,
    pool_config: pool::Config,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            client_config: Config {
                retry_canceled_requests: true,
                set_host: true,
                ver: Ver::Auto,
            },
            #[cfg_attr(feature = "deprecated", allow(deprecated))]
            conn_builder: conn::Builder::new(),
            pool_config: pool::Config {
                idle_timeout: Some(Duration::from_secs(90)),
                max_idle_per_host: std::usize::MAX,
            },
        }
    }
}

impl Builder {
    #[doc(hidden)]
    #[deprecated(
        note = "name is confusing, to disable the connection pool, call pool_max_idle_per_host(0)"
    )]
    pub fn keep_alive(&mut self, val: bool) -> &mut Self {
        if !val {
            // disable
            self.pool_max_idle_per_host(0)
        } else if self.pool_config.max_idle_per_host == 0 {
            // enable
            self.pool_max_idle_per_host(std::usize::MAX)
        } else {
            // already enabled
            self
        }
    }

    #[doc(hidden)]
    #[deprecated(note = "renamed to `pool_idle_timeout`")]
    pub fn keep_alive_timeout<D>(&mut self, val: D) -> &mut Self
    where
        D: Into<Option<Duration>>,
    {
        self.pool_idle_timeout(val)
    }

    /// Set an optional timeout for idle sockets being kept-alive.
    ///
    /// Pass `None` to disable timeout.
    ///
    /// Default is 90 seconds.
    pub fn pool_idle_timeout<D>(&mut self, val: D) -> &mut Self
    where
        D: Into<Option<Duration>>,
    {
        self.pool_config.idle_timeout = val.into();
        self
    }

    #[doc(hidden)]
    #[deprecated(note = "renamed to `pool_max_idle_per_host`")]
    pub fn max_idle_per_host(&mut self, max_idle: usize) -> &mut Self {
        self.pool_config.max_idle_per_host = max_idle;
        self
    }

    /// Sets the maximum idle connection per host allowed in the pool.
    ///
    /// Default is `usize::MAX` (no limit).
    pub fn pool_max_idle_per_host(&mut self, max_idle: usize) -> &mut Self {
        self.pool_config.max_idle_per_host = max_idle;
        self
    }

    // HTTP/1 options

    /// Sets the exact size of the read buffer to *always* use.
    ///
    /// Note that setting this option unsets the `http1_max_buf_size` option.
    ///
    /// Default is an adaptive read buffer.
    pub fn http1_read_buf_exact_size(&mut self, sz: usize) -> &mut Self {
        self.conn_builder.http1_read_buf_exact_size(Some(sz));
        self
    }

    /// Set the maximum buffer size for the connection.
    ///
    /// Default is ~400kb.
    ///
    /// Note that setting this option unsets the `http1_read_exact_buf_size` option.
    ///
    /// # Panics
    ///
    /// The minimum value allowed is 8192. This method panics if the passed `max` is less than the minimum.
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub fn http1_max_buf_size(&mut self, max: usize) -> &mut Self {
        self.conn_builder.http1_max_buf_size(max);
        self
    }

    /// Set whether HTTP/1 connections will accept spaces between header names
    /// and the colon that follow them in responses.
    ///
    /// Newline codepoints (`\r` and `\n`) will be transformed to spaces when
    /// parsing.
    ///
    /// You probably don't need this, here is what [RFC 7230 Section 3.2.4.] has
    /// to say about it:
    ///
    /// > No whitespace is allowed between the header field-name and colon. In
    /// > the past, differences in the handling of such whitespace have led to
    /// > security vulnerabilities in request routing and response handling. A
    /// > server MUST reject any received request message that contains
    /// > whitespace between a header field-name and colon with a response code
    /// > of 400 (Bad Request). A proxy MUST remove any such whitespace from a
    /// > response message before forwarding the message downstream.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    ///
    /// [RFC 7230 Section 3.2.4.]: https://tools.ietf.org/html/rfc7230#section-3.2.4
    pub fn http1_allow_spaces_after_header_name_in_responses(&mut self, val: bool) -> &mut Self {
        self.conn_builder
            .http1_allow_spaces_after_header_name_in_responses(val);
        self
    }

    /// Set whether HTTP/1 connections will accept obsolete line folding for
    /// header values.
    ///
    /// You probably don't need this, here is what [RFC 7230 Section 3.2.4.] has
    /// to say about it:
    ///
    /// > A server that receives an obs-fold in a request message that is not
    /// > within a message/http container MUST either reject the message by
    /// > sending a 400 (Bad Request), preferably with a representation
    /// > explaining that obsolete line folding is unacceptable, or replace
    /// > each received obs-fold with one or more SP octets prior to
    /// > interpreting the field value or forwarding the message downstream.
    ///
    /// > A proxy or gateway that receives an obs-fold in a response message
    /// > that is not within a message/http container MUST either discard the
    /// > message and replace it with a 502 (Bad Gateway) response, preferably
    /// > with a representation explaining that unacceptable line folding was
    /// > received, or replace each received obs-fold with one or more SP
    /// > octets prior to interpreting the field value or forwarding the
    /// > message downstream.
    ///
    /// > A user agent that receives an obs-fold in a response message that is
    /// > not within a message/http container MUST replace each received
    /// > obs-fold with one or more SP octets prior to interpreting the field
    /// > value.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    ///
    /// [RFC 7230 Section 3.2.4.]: https://tools.ietf.org/html/rfc7230#section-3.2.4
    pub fn http1_allow_obsolete_multiline_headers_in_responses(&mut self, val: bool) -> &mut Self {
        self.conn_builder
            .http1_allow_obsolete_multiline_headers_in_responses(val);
        self
    }

    /// Sets whether invalid header lines should be silently ignored in HTTP/1 responses.
    ///
    /// This mimicks the behaviour of major browsers. You probably don't want this.
    /// You should only want this if you are implementing a proxy whose main
    /// purpose is to sit in front of browsers whose users access arbitrary content
    /// which may be malformed, and they expect everything that works without
    /// the proxy to keep working with the proxy.
    ///
    /// This option will prevent Hyper's client from returning an error encountered
    /// when parsing a header, except if the error was caused by the character NUL
    /// (ASCII code 0), as Chrome specifically always reject those.
    ///
    /// The ignorable errors are:
    /// * empty header names;
    /// * characters that are not allowed in header names, except for `\0` and `\r`;
    /// * when `allow_spaces_after_header_name_in_responses` is not enabled,
    ///   spaces and tabs between the header name and the colon;
    /// * missing colon between header name and colon;
    /// * characters that are not allowed in header values except for `\0` and `\r`.
    ///
    /// If an ignorable error is encountered, the parser tries to find the next
    /// line in the input to resume parsing the rest of the headers. An error
    /// will be emitted nonetheless if it finds `\0` or a lone `\r` while
    /// looking for the next line.
    pub fn http1_ignore_invalid_headers_in_responses(&mut self, val: bool) -> &mut Builder {
        self.conn_builder
            .http1_ignore_invalid_headers_in_responses(val);
        self
    }

    /// Set whether HTTP/1 connections should try to use vectored writes,
    /// or always flatten into a single buffer.
    ///
    /// Note that setting this to false may mean more copies of body data,
    /// but may also improve performance when an IO transport doesn't
    /// support vectored writes well, such as most TLS implementations.
    ///
    /// Setting this to true will force hyper to use queued strategy
    /// which may eliminate unnecessary cloning on some TLS backends
    ///
    /// Default is `auto`. In this mode hyper will try to guess which
    /// mode to use
    pub fn http1_writev(&mut self, enabled: bool) -> &mut Builder {
        self.conn_builder.http1_writev(enabled);
        self
    }

    /// Set whether HTTP/1 connections will write header names as title case at
    /// the socket level.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    pub fn http1_title_case_headers(&mut self, val: bool) -> &mut Self {
        self.conn_builder.http1_title_case_headers(val);
        self
    }

    /// Set whether to support preserving original header cases.
    ///
    /// Currently, this will record the original cases received, and store them
    /// in a private extension on the `Response`. It will also look for and use
    /// such an extension in any provided `Request`.
    ///
    /// Since the relevant extension is still private, there is no way to
    /// interact with the original cases. The only effect this can have now is
    /// to forward the cases in a proxy-like fashion.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    pub fn http1_preserve_header_case(&mut self, val: bool) -> &mut Self {
        self.conn_builder.http1_preserve_header_case(val);
        self
    }

    /// Set whether HTTP/0.9 responses should be tolerated.
    ///
    /// Default is false.
    pub fn http09_responses(&mut self, val: bool) -> &mut Self {
        self.conn_builder.http09_responses(val);
        self
    }

    /// Set whether the connection **must** use HTTP/2.
    ///
    /// The destination must either allow HTTP2 Prior Knowledge, or the
    /// `Connect` should be configured to do use ALPN to upgrade to `h2`
    /// as part of the connection process. This will not make the `Client`
    /// utilize ALPN by itself.
    ///
    /// Note that setting this to true prevents HTTP/1 from being allowed.
    ///
    /// Default is false.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_only(&mut self, val: bool) -> &mut Self {
        self.client_config.ver = if val { Ver::Http2 } else { Ver::Auto };
        self
    }

    /// Sets the [`SETTINGS_INITIAL_WINDOW_SIZE`][spec] option for HTTP2
    /// stream-level flow control.
    ///
    /// Passing `None` will do nothing.
    ///
    /// If not set, hyper will use a default.
    ///
    /// [spec]: https://http2.github.io/http2-spec/#SETTINGS_INITIAL_WINDOW_SIZE
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_initial_stream_window_size(&mut self, sz: impl Into<Option<u32>>) -> &mut Self {
        self.conn_builder
            .http2_initial_stream_window_size(sz.into());
        self
    }

    /// Sets the max connection-level flow control for HTTP2
    ///
    /// Passing `None` will do nothing.
    ///
    /// If not set, hyper will use a default.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_initial_connection_window_size(
        &mut self,
        sz: impl Into<Option<u32>>,
    ) -> &mut Self {
        self.conn_builder
            .http2_initial_connection_window_size(sz.into());
        self
    }

    /// Sets whether to use an adaptive flow control.
    ///
    /// Enabling this will override the limits set in
    /// `http2_initial_stream_window_size` and
    /// `http2_initial_connection_window_size`.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_adaptive_window(&mut self, enabled: bool) -> &mut Self {
        self.conn_builder.http2_adaptive_window(enabled);
        self
    }

    /// Sets the maximum frame size to use for HTTP2.
    ///
    /// Passing `None` will do nothing.
    ///
    /// If not set, hyper will use a default.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_max_frame_size(&mut self, sz: impl Into<Option<u32>>) -> &mut Self {
        self.conn_builder.http2_max_frame_size(sz);
        self
    }

    /// Sets an interval for HTTP2 Ping frames should be sent to keep a
    /// connection alive.
    ///
    /// Pass `None` to disable HTTP2 keep-alive.
    ///
    /// Default is currently disabled.
    ///
    /// # Cargo Feature
    ///
    /// Requires the `runtime` cargo feature to be enabled.
    #[cfg(feature = "runtime")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_keep_alive_interval(
        &mut self,
        interval: impl Into<Option<Duration>>,
    ) -> &mut Self {
        self.conn_builder.http2_keep_alive_interval(interval);
        self
    }

    /// Sets a timeout for receiving an acknowledgement of the keep-alive ping.
    ///
    /// If the ping is not acknowledged within the timeout, the connection will
    /// be closed. Does nothing if `http2_keep_alive_interval` is disabled.
    ///
    /// Default is 20 seconds.
    ///
    /// # Cargo Feature
    ///
    /// Requires the `runtime` cargo feature to be enabled.
    #[cfg(feature = "runtime")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_keep_alive_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.conn_builder.http2_keep_alive_timeout(timeout);
        self
    }

    /// Sets whether HTTP2 keep-alive should apply while the connection is idle.
    ///
    /// If disabled, keep-alive pings are only sent while there are open
    /// request/responses streams. If enabled, pings are also sent when no
    /// streams are active. Does nothing if `http2_keep_alive_interval` is
    /// disabled.
    ///
    /// Default is `false`.
    ///
    /// # Cargo Feature
    ///
    /// Requires the `runtime` cargo feature to be enabled.
    #[cfg(feature = "runtime")]
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_keep_alive_while_idle(&mut self, enabled: bool) -> &mut Self {
        self.conn_builder.http2_keep_alive_while_idle(enabled);
        self
    }

    /// Sets the maximum number of HTTP2 concurrent locally reset streams.
    ///
    /// See the documentation of [`h2::client::Builder::max_concurrent_reset_streams`] for more
    /// details.
    ///
    /// The default value is determined by the `h2` crate.
    ///
    /// [`h2::client::Builder::max_concurrent_reset_streams`]: https://docs.rs/h2/client/struct.Builder.html#method.max_concurrent_reset_streams
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_max_concurrent_reset_streams(&mut self, max: usize) -> &mut Self {
        self.conn_builder.http2_max_concurrent_reset_streams(max);
        self
    }

    /// Set the maximum write buffer size for each HTTP/2 stream.
    ///
    /// Default is currently 1MB, but may change.
    ///
    /// # Panics
    ///
    /// The value must be no larger than `u32::MAX`.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_max_send_buf_size(&mut self, max: usize) -> &mut Self {
        self.conn_builder.http2_max_send_buf_size(max);
        self
    }

    /// Set whether to retry requests that get disrupted before ever starting
    /// to write.
    ///
    /// This means a request that is queued, and gets given an idle, reused
    /// connection, and then encounters an error immediately as the idle
    /// connection was found to be unusable.
    ///
    /// When this is set to `false`, the related `ResponseFuture` would instead
    /// resolve to an `Error::Cancel`.
    ///
    /// Default is `true`.
    #[inline]
    pub fn retry_canceled_requests(&mut self, val: bool) -> &mut Self {
        self.client_config.retry_canceled_requests = val;
        self
    }

    /// Set whether to automatically add the `Host` header to requests.
    ///
    /// If true, and a request does not include a `Host` header, one will be
    /// added automatically, derived from the authority of the `Uri`.
    ///
    /// Default is `true`.
    #[inline]
    pub fn set_host(&mut self, val: bool) -> &mut Self {
        self.client_config.set_host = val;
        self
    }

    /// Provide an executor to execute background `Connection` tasks.
    pub fn executor<E>(&mut self, exec: E) -> &mut Self
    where
        E: Executor<BoxSendFuture> + Send + Sync + 'static,
    {
        self.conn_builder.executor(exec);
        self
    }

    /// Builder a client with this configuration and the default `HttpConnector`.
    #[cfg(feature = "tcp")]
    pub fn build_http<B>(&self) -> Client<HttpConnector, B>
    where
        B: HttpBody + Send,
        B::Data: Send,
    {
        let mut connector = HttpConnector::new();
        if self.pool_config.is_enabled() {
            connector.set_keepalive(self.pool_config.idle_timeout);
        }
        self.build(connector)
    }

    /// Combine the configuration of this builder with a connector to create a `Client`.
    pub fn build<C, B>(&self, connector: C) -> Client<C, B>
    where
        C: Connect + Clone,
        B: HttpBody + Send,
        B::Data: Send,
    {
        #[cfg_attr(feature = "deprecated", allow(deprecated))]
        Client {
            config: self.client_config,
            conn_builder: self.conn_builder.clone(),
            connector,
            pool: Pool::new(self.pool_config, &self.conn_builder.exec),
        }
    }
}

impl fmt::Debug for Builder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Builder")
            .field("client_config", &self.client_config)
            .field("conn_builder", &self.conn_builder)
            .field("pool_config", &self.pool_config)
            .finish()
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn response_future_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ResponseFuture>();
    }

    #[test]
    fn set_relative_uri_with_implicit_path() {
        let mut uri = "http://hyper.rs".parse().unwrap();
        origin_form(&mut uri);
        assert_eq!(uri.to_string(), "/");
    }

    #[test]
    fn test_origin_form() {
        let mut uri = "http://hyper.rs/guides".parse().unwrap();
        origin_form(&mut uri);
        assert_eq!(uri.to_string(), "/guides");

        let mut uri = "http://hyper.rs/guides?foo=bar".parse().unwrap();
        origin_form(&mut uri);
        assert_eq!(uri.to_string(), "/guides?foo=bar");
    }

    #[test]
    fn test_absolute_form() {
        let mut uri = "http://hyper.rs/guides".parse().unwrap();
        absolute_form(&mut uri);
        assert_eq!(uri.to_string(), "http://hyper.rs/guides");

        let mut uri = "https://hyper.rs/guides".parse().unwrap();
        absolute_form(&mut uri);
        assert_eq!(uri.to_string(), "/guides");
    }

    #[test]
    fn test_authority_form() {
        let _ = pretty_env_logger::try_init();

        let mut uri = "http://hyper.rs".parse().unwrap();
        authority_form(&mut uri);
        assert_eq!(uri.to_string(), "hyper.rs");

        let mut uri = "hyper.rs".parse().unwrap();
        authority_form(&mut uri);
        assert_eq!(uri.to_string(), "hyper.rs");
    }

    #[test]
    fn test_extract_domain_connect_no_port() {
        let mut uri = "hyper.rs".parse().unwrap();
        let (scheme, host) = extract_domain(&mut uri, true).expect("extract domain");
        assert_eq!(scheme, *"http");
        assert_eq!(host, "hyper.rs");
    }

    #[test]
    fn test_is_secure() {
        assert_eq!(
            is_schema_secure(&"http://hyper.rs".parse::<Uri>().unwrap()),
            false
        );
        assert_eq!(is_schema_secure(&"hyper.rs".parse::<Uri>().unwrap()), false);
        assert_eq!(
            is_schema_secure(&"wss://hyper.rs".parse::<Uri>().unwrap()),
            true
        );
        assert_eq!(
            is_schema_secure(&"ws://hyper.rs".parse::<Uri>().unwrap()),
            false
        );
    }

    #[test]
    fn test_get_non_default_port() {
        assert!(get_non_default_port(&"http://hyper.rs".parse::<Uri>().unwrap()).is_none());
        assert!(get_non_default_port(&"http://hyper.rs:80".parse::<Uri>().unwrap()).is_none());
        assert!(get_non_default_port(&"https://hyper.rs:443".parse::<Uri>().unwrap()).is_none());
        assert!(get_non_default_port(&"hyper.rs:80".parse::<Uri>().unwrap()).is_none());

        assert_eq!(
            get_non_default_port(&"http://hyper.rs:123".parse::<Uri>().unwrap())
                .unwrap()
                .as_u16(),
            123
        );
        assert_eq!(
            get_non_default_port(&"https://hyper.rs:80".parse::<Uri>().unwrap())
                .unwrap()
                .as_u16(),
            80
        );
        assert_eq!(
            get_non_default_port(&"hyper.rs:123".parse::<Uri>().unwrap())
                .unwrap()
                .as_u16(),
            123
        );
    }
}
