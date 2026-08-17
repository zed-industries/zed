//! Lower-level client connection API.
//!
//! The types in this module are to provide a lower-level API based around a
//! single connection. Connecting to a host, pooling connections, and the like
//! are not handled at this level. This module provides the building blocks to
//! customize those things externally.
//!
//! If don't have need to manage connections yourself, consider using the
//! higher-level [Client](super) API.
//!
//! ## Example
//! A simple example that uses the `SendRequest` struct to talk HTTP over a Tokio TCP stream
//! ```no_run
//! # #[cfg(all(feature = "client", feature = "http1", feature = "runtime"))]
//! # mod rt {
//! use tower::ServiceExt;
//! use http::{Request, StatusCode};
//! use hyper::{client::conn, Body};
//! use tokio::net::TcpStream;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let target_stream = TcpStream::connect("example.com:80").await?;
//!
//!     let (mut request_sender, connection) = conn::handshake(target_stream).await?;
//!
//!     // spawn a task to poll the connection and drive the HTTP state
//!     tokio::spawn(async move {
//!         if let Err(e) = connection.await {
//!             eprintln!("Error in connection: {}", e);
//!         }
//!     });
//!
//!     let request = Request::builder()
//!         // We need to manually add the host header because SendRequest does not
//!         .header("Host", "example.com")
//!         .method("GET")
//!         .body(Body::from(""))?;
//!     let response = request_sender.send_request(request).await?;
//!     assert!(response.status() == StatusCode::OK);
//!
//!     // To send via the same connection again, it may not work as it may not be ready,
//!     // so we have to wait until the request_sender becomes ready.
//!     request_sender.ready().await?;
//!     let request = Request::builder()
//!         .header("Host", "example.com")
//!         .method("GET")
//!         .body(Body::from(""))?;
//!     let response = request_sender.send_request(request).await?;
//!     assert!(response.status() == StatusCode::OK);
//!     Ok(())
//! }
//!
//! # }
//! ```

#[cfg(all(feature = "backports", feature = "http1"))]
pub mod http1;
#[cfg(all(feature = "backports", feature = "http2"))]
pub mod http2;

#[cfg(not(all(feature = "http1", feature = "http2")))]
use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
#[cfg(not(all(feature = "http1", feature = "http2")))]
use std::marker::PhantomData;
use std::marker::Unpin;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
#[cfg(all(feature = "runtime", feature = "http2"))]
use std::time::Duration;

use bytes::Bytes;
use futures_util::future::{self, Either, FutureExt as _};
use httparse::ParserConfig;
use pin_project_lite::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service::Service;
use tracing::{debug, trace};

use super::dispatch;
use crate::body::HttpBody;
use crate::common::exec::{BoxSendFuture, Exec};
use crate::proto;
use crate::rt::Executor;
#[cfg(feature = "http1")]
use crate::upgrade::Upgraded;
use crate::{Body, Request, Response};

#[cfg(feature = "http1")]
type Http1Dispatcher<T, B> =
    proto::dispatch::Dispatcher<proto::dispatch::Client<B>, B, T, proto::h1::ClientTransaction>;

#[cfg(not(feature = "http1"))]
type Http1Dispatcher<T, B> = (Infallible, PhantomData<(T, Pin<Box<B>>)>);

#[cfg(feature = "http2")]
type Http2ClientTask<B> = proto::h2::ClientTask<B>;

#[cfg(not(feature = "http2"))]
type Http2ClientTask<B> = (Infallible, PhantomData<Pin<Box<B>>>);

pin_project! {
    #[project = ProtoClientProj]
    enum ProtoClient<T, B>
    where
        B: HttpBody,
    {
        H1 {
            #[pin]
            h1: Http1Dispatcher<T, B>,
        },
        H2 {
            #[pin]
            h2: Http2ClientTask<B>,
        },
    }
}

/// Returns a handshake future over some IO.
///
/// This is a shortcut for `Builder::new().handshake(io)`.
/// See [`client::conn`](crate::client::conn) for more.
#[cfg_attr(
    feature = "deprecated",
    deprecated(
        note = "This function will be replaced with `client::conn::http1::handshake` and `client::conn::http2::handshake` in 1.0, enable the \"backports\" feature to use them now."
    )
)]
#[cfg_attr(feature = "deprecated", allow(deprecated))]
pub async fn handshake<T>(
    io: T,
) -> crate::Result<(SendRequest<crate::Body>, Connection<T, crate::Body>)>
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    #[allow(deprecated)]
    Builder::new().handshake(io).await
}

/// The sender side of an established connection.
#[cfg_attr(
    feature = "deprecated",
    deprecated(
        note = "This type will be replaced with `client::conn::http1::SendRequest` and `client::conn::http2::SendRequest` in 1.0, enable the \"backports\" feature to use them now."
    )
)]
pub struct SendRequest<B> {
    dispatch: dispatch::Sender<Request<B>, Response<Body>>,
}

/// A future that processes all HTTP state for the IO object.
///
/// In most cases, this should just be spawned into an executor, so that it
/// can process incoming and outgoing messages, notice hangups, and the like.
#[must_use = "futures do nothing unless polled"]
#[cfg_attr(
    feature = "deprecated",
    deprecated(
        note = "This type will be replaced with `client::conn::http1::Connection` and `client::conn::http2::Connection` in 1.0, enable the \"backports\" feature to use them now."
    )
)]
pub struct Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Send + 'static,
    B: HttpBody + 'static,
{
    inner: Option<ProtoClient<T, B>>,
}

/// A builder to configure an HTTP connection.
///
/// After setting options, the builder is used to create a handshake future.
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "deprecated",
    deprecated(
        note = "This type will be replaced with `client::conn::http1::Builder` and `client::conn::http2::Builder` in 1.0, enable the \"backports\" feature to use them now."
    )
)]
pub struct Builder {
    pub(super) exec: Exec,
    h09_responses: bool,
    h1_parser_config: ParserConfig,
    h1_writev: Option<bool>,
    h1_title_case_headers: bool,
    h1_preserve_header_case: bool,
    #[cfg(feature = "ffi")]
    h1_preserve_header_order: bool,
    h1_read_buf_exact_size: Option<usize>,
    h1_max_buf_size: Option<usize>,
    #[cfg(feature = "ffi")]
    h1_headers_raw: bool,
    #[cfg(feature = "http2")]
    h2_builder: proto::h2::client::Config,
    version: Proto,
}

#[derive(Clone, Debug)]
enum Proto {
    #[cfg(feature = "http1")]
    Http1,
    #[cfg(feature = "http2")]
    Http2,
}

/// A future returned by `SendRequest::send_request`.
///
/// Yields a `Response` if successful.
#[must_use = "futures do nothing unless polled"]
pub struct ResponseFuture {
    inner: ResponseFutureState,
}

enum ResponseFutureState {
    Waiting(dispatch::Promise<Response<Body>>),
    // Option is to be able to `take()` it in `poll`
    Error(Option<crate::Error>),
}

/// Deconstructed parts of a `Connection`.
///
/// This allows taking apart a `Connection` at a later time, in order to
/// reclaim the IO object, and additional related pieces.
#[derive(Debug)]
pub struct Parts<T> {
    /// The original IO object used in the handshake.
    pub io: T,
    /// A buffer of bytes that have been read but not processed as HTTP.
    ///
    /// For instance, if the `Connection` is used for an HTTP upgrade request,
    /// it is possible the server sent back the first bytes of the new protocol
    /// along with the response upgrade.
    ///
    /// You will want to check for any existing bytes if you plan to continue
    /// communicating on the IO object.
    pub read_buf: Bytes,
    _inner: (),
}

// ========== internal client api

// A `SendRequest` that can be cloned to send HTTP2 requests.
// private for now, probably not a great idea of a type...
#[must_use = "futures do nothing unless polled"]
#[cfg(feature = "http2")]
pub(super) struct Http2SendRequest<B> {
    dispatch: dispatch::UnboundedSender<Request<B>, Response<Body>>,
}

// ===== impl SendRequest

#[cfg_attr(feature = "deprecated", allow(deprecated))]
impl<B> SendRequest<B> {
    /// Polls to determine whether this sender can be used yet for a request.
    ///
    /// If the associated connection is closed, this returns an Error.
    pub fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<()>> {
        self.dispatch.poll_ready(cx)
    }

    pub(super) async fn when_ready(self) -> crate::Result<Self> {
        let mut me = Some(self);
        future::poll_fn(move |cx| {
            ready!(me.as_mut().unwrap().poll_ready(cx))?;
            Poll::Ready(Ok(me.take().unwrap()))
        })
        .await
    }

    pub(super) fn is_ready(&self) -> bool {
        self.dispatch.is_ready()
    }

    pub(super) fn is_closed(&self) -> bool {
        self.dispatch.is_closed()
    }

    #[cfg(feature = "http2")]
    pub(super) fn into_http2(self) -> Http2SendRequest<B> {
        Http2SendRequest {
            dispatch: self.dispatch.unbound(),
        }
    }
}

#[cfg_attr(feature = "deprecated", allow(deprecated))]
impl<B> SendRequest<B>
where
    B: HttpBody + 'static,
{
    /// Sends a `Request` on the associated connection.
    ///
    /// Returns a future that if successful, yields the `Response`.
    ///
    /// # Note
    ///
    /// There are some key differences in what automatic things the `Client`
    /// does for you that will not be done here:
    ///
    /// - `Client` requires absolute-form `Uri`s, since the scheme and
    ///   authority are needed to connect. They aren't required here.
    /// - Since the `Client` requires absolute-form `Uri`s, it can add
    ///   the `Host` header based on it. You must add a `Host` header yourself
    ///   before calling this method.
    /// - Since absolute-form `Uri`s are not required, if received, they will
    ///   be serialized as-is.
    ///
    /// # Example
    ///
    /// ```
    /// # use http::header::HOST;
    /// # use hyper::client::conn::SendRequest;
    /// # use hyper::Body;
    /// use hyper::Request;
    ///
    /// # async fn doc(mut tx: SendRequest<Body>) -> hyper::Result<()> {
    /// // build a Request
    /// let req = Request::builder()
    ///     .uri("/foo/bar")
    ///     .header(HOST, "hyper.rs")
    ///     .body(Body::empty())
    ///     .unwrap();
    ///
    /// // send it and await a Response
    /// let res = tx.send_request(req).await?;
    /// // assert the Response
    /// assert!(res.status().is_success());
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    pub fn send_request(&mut self, req: Request<B>) -> ResponseFuture {
        let inner = match self.dispatch.send(req) {
            Ok(rx) => ResponseFutureState::Waiting(rx),
            Err(_req) => {
                debug!("connection was not ready");
                let err = crate::Error::new_canceled().with("connection was not ready");
                ResponseFutureState::Error(Some(err))
            }
        };

        ResponseFuture { inner }
    }

    pub(super) fn send_request_retryable(
        &mut self,
        req: Request<B>,
    ) -> impl Future<Output = Result<Response<Body>, (crate::Error, Option<Request<B>>)>> + Unpin
    where
        B: Send,
    {
        match self.dispatch.try_send(req) {
            Ok(rx) => {
                Either::Left(rx.then(move |res| {
                    match res {
                        Ok(Ok(res)) => future::ok(res),
                        Ok(Err(err)) => future::err(err),
                        // this is definite bug if it happens, but it shouldn't happen!
                        Err(_) => panic!("dispatch dropped without returning error"),
                    }
                }))
            }
            Err(req) => {
                debug!("connection was not ready");
                let err = crate::Error::new_canceled().with("connection was not ready");
                Either::Right(future::err((err, Some(req))))
            }
        }
    }
}

#[cfg_attr(feature = "deprecated", allow(deprecated))]
impl<B> Service<Request<B>> for SendRequest<B>
where
    B: HttpBody + 'static,
{
    type Response = Response<Body>;
    type Error = crate::Error;
    type Future = ResponseFuture;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        self.send_request(req)
    }
}

#[cfg_attr(feature = "deprecated", allow(deprecated))]
impl<B> fmt::Debug for SendRequest<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendRequest").finish()
    }
}

// ===== impl Http2SendRequest

#[cfg(feature = "http2")]
impl<B> Http2SendRequest<B> {
    pub(super) fn is_ready(&self) -> bool {
        self.dispatch.is_ready()
    }

    pub(super) fn is_closed(&self) -> bool {
        self.dispatch.is_closed()
    }
}

#[cfg(feature = "http2")]
impl<B> Http2SendRequest<B>
where
    B: HttpBody + 'static,
{
    pub(super) fn send_request_retryable(
        &mut self,
        req: Request<B>,
    ) -> impl Future<Output = Result<Response<Body>, (crate::Error, Option<Request<B>>)>>
    where
        B: Send,
    {
        match self.dispatch.try_send(req) {
            Ok(rx) => {
                Either::Left(rx.then(move |res| {
                    match res {
                        Ok(Ok(res)) => future::ok(res),
                        Ok(Err(err)) => future::err(err),
                        // this is definite bug if it happens, but it shouldn't happen!
                        Err(_) => panic!("dispatch dropped without returning error"),
                    }
                }))
            }
            Err(req) => {
                debug!("connection was not ready");
                let err = crate::Error::new_canceled().with("connection was not ready");
                Either::Right(future::err((err, Some(req))))
            }
        }
    }
}

#[cfg(feature = "http2")]
impl<B> fmt::Debug for Http2SendRequest<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Http2SendRequest").finish()
    }
}

#[cfg(feature = "http2")]
impl<B> Clone for Http2SendRequest<B> {
    fn clone(&self) -> Self {
        Http2SendRequest {
            dispatch: self.dispatch.clone(),
        }
    }
}

// ===== impl Connection

#[cfg_attr(feature = "deprecated", allow(deprecated))]
impl<T, B> Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    B: HttpBody + Unpin + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    /// Return the inner IO object, and additional information.
    ///
    /// Only works for HTTP/1 connections. HTTP/2 connections will panic.
    pub fn into_parts(self) -> Parts<T> {
        match self.inner.expect("already upgraded") {
            #[cfg(feature = "http1")]
            ProtoClient::H1 { h1 } => {
                let (io, read_buf, _) = h1.into_inner();
                Parts {
                    io,
                    read_buf,
                    _inner: (),
                }
            }
            ProtoClient::H2 { .. } => {
                panic!("http2 cannot into_inner");
            }

            #[cfg(not(feature = "http1"))]
            ProtoClient::H1 { h1 } => match h1.0 {},
        }
    }

    /// Poll the connection for completion, but without calling `shutdown`
    /// on the underlying IO.
    ///
    /// This is useful to allow running a connection while doing an HTTP
    /// upgrade. Once the upgrade is completed, the connection would be "done",
    /// but it is not desired to actually shutdown the IO object. Instead you
    /// would take it back using `into_parts`.
    ///
    /// Use [`poll_fn`](https://docs.rs/futures/0.1.25/futures/future/fn.poll_fn.html)
    /// and [`try_ready!`](https://docs.rs/futures/0.1.25/futures/macro.try_ready.html)
    /// to work with this function; or use the `without_shutdown` wrapper.
    pub fn poll_without_shutdown(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<()>> {
        match *self.inner.as_mut().expect("already upgraded") {
            #[cfg(feature = "http1")]
            ProtoClient::H1 { ref mut h1 } => h1.poll_without_shutdown(cx),
            #[cfg(feature = "http2")]
            ProtoClient::H2 { ref mut h2, .. } => Pin::new(h2).poll(cx).map_ok(|_| ()),

            #[cfg(not(feature = "http1"))]
            ProtoClient::H1 { ref mut h1 } => match h1.0 {},
            #[cfg(not(feature = "http2"))]
            ProtoClient::H2 { ref mut h2, .. } => match h2.0 {},
        }
    }

    /// Prevent shutdown of the underlying IO object at the end of service the request,
    /// instead run `into_parts`. This is a convenience wrapper over `poll_without_shutdown`.
    pub fn without_shutdown(self) -> impl Future<Output = crate::Result<Parts<T>>> {
        let mut conn = Some(self);
        future::poll_fn(move |cx| -> Poll<crate::Result<Parts<T>>> {
            ready!(conn.as_mut().unwrap().poll_without_shutdown(cx))?;
            Poll::Ready(Ok(conn.take().unwrap().into_parts()))
        })
    }

    /// Returns whether the [extended CONNECT protocol][1] is enabled or not.
    ///
    /// This setting is configured by the server peer by sending the
    /// [`SETTINGS_ENABLE_CONNECT_PROTOCOL` parameter][2] in a `SETTINGS` frame.
    /// This method returns the currently acknowledged value received from the
    /// remote.
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc8441#section-4
    /// [2]: https://datatracker.ietf.org/doc/html/rfc8441#section-3
    #[cfg(feature = "http2")]
    pub fn http2_is_extended_connect_protocol_enabled(&self) -> bool {
        match self.inner.as_ref().unwrap() {
            ProtoClient::H1 { .. } => false,
            ProtoClient::H2 { h2 } => h2.is_extended_connect_protocol_enabled(),
        }
    }
}

#[cfg_attr(feature = "deprecated", allow(deprecated))]
impl<T, B> Future for Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Unpin + Send,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Output = crate::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match ready!(Pin::new(self.inner.as_mut().unwrap()).poll(cx))? {
            proto::Dispatched::Shutdown => Poll::Ready(Ok(())),
            #[cfg(feature = "http1")]
            proto::Dispatched::Upgrade(pending) => match self.inner.take() {
                Some(ProtoClient::H1 { h1 }) => {
                    let (io, buf, _) = h1.into_inner();
                    pending.fulfill(Upgraded::new(io, buf));
                    Poll::Ready(Ok(()))
                }
                _ => {
                    drop(pending);
                    unreachable!("Upgrade expects h1");
                }
            },
        }
    }
}

#[cfg_attr(feature = "deprecated", allow(deprecated))]
impl<T, B> fmt::Debug for Connection<T, B>
where
    T: AsyncRead + AsyncWrite + fmt::Debug + Send + 'static,
    B: HttpBody + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection").finish()
    }
}

// ===== impl Builder

#[cfg_attr(feature = "deprecated", allow(deprecated))]
impl Builder {
    /// Creates a new connection builder.
    #[inline]
    pub fn new() -> Builder {
        Builder {
            exec: Exec::Default,
            h09_responses: false,
            h1_writev: None,
            h1_read_buf_exact_size: None,
            h1_parser_config: Default::default(),
            h1_title_case_headers: false,
            h1_preserve_header_case: false,
            #[cfg(feature = "ffi")]
            h1_preserve_header_order: false,
            h1_max_buf_size: None,
            #[cfg(feature = "ffi")]
            h1_headers_raw: false,
            #[cfg(feature = "http2")]
            h2_builder: Default::default(),
            #[cfg(feature = "http1")]
            version: Proto::Http1,
            #[cfg(not(feature = "http1"))]
            version: Proto::Http2,
        }
    }

    /// Provide an executor to execute background HTTP2 tasks.
    pub fn executor<E>(&mut self, exec: E) -> &mut Builder
    where
        E: Executor<BoxSendFuture> + Send + Sync + 'static,
    {
        self.exec = Exec::Executor(Arc::new(exec));
        self
    }

    /// Set whether HTTP/0.9 responses should be tolerated.
    ///
    /// Default is false.
    pub fn http09_responses(&mut self, enabled: bool) -> &mut Builder {
        self.h09_responses = enabled;
        self
    }

    /// Set whether HTTP/1 connections will accept spaces between header names
    /// and the colon that follow them in responses.
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
    pub fn http1_allow_spaces_after_header_name_in_responses(
        &mut self,
        enabled: bool,
    ) -> &mut Builder {
        self.h1_parser_config
            .allow_spaces_after_header_name_in_responses(enabled);
        self
    }

    /// Set whether HTTP/1 connections will accept obsolete line folding for
    /// header values.
    ///
    /// Newline codepoints (`\r` and `\n`) will be transformed to spaces when
    /// parsing.
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
    pub fn http1_allow_obsolete_multiline_headers_in_responses(
        &mut self,
        enabled: bool,
    ) -> &mut Builder {
        self.h1_parser_config
            .allow_obsolete_multiline_headers_in_responses(enabled);
        self
    }

    /// Set whether HTTP/1 connections will silently ignored malformed header lines.
    ///
    /// If this is enabled and and a header line does not start with a valid header
    /// name, or does not include a colon at all, the line will be silently ignored
    /// and no error will be reported.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    pub fn http1_ignore_invalid_headers_in_responses(&mut self, enabled: bool) -> &mut Builder {
        self.h1_parser_config
            .ignore_invalid_headers_in_responses(enabled);
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
        self.h1_writev = Some(enabled);
        self
    }

    /// Set whether HTTP/1 connections will write header names as title case at
    /// the socket level.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    pub fn http1_title_case_headers(&mut self, enabled: bool) -> &mut Builder {
        self.h1_title_case_headers = enabled;
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
    pub fn http1_preserve_header_case(&mut self, enabled: bool) -> &mut Builder {
        self.h1_preserve_header_case = enabled;
        self
    }

    /// Set whether to support preserving original header order.
    ///
    /// Currently, this will record the order in which headers are received, and store this
    /// ordering in a private extension on the `Response`. It will also look for and use
    /// such an extension in any provided `Request`.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    #[cfg(feature = "ffi")]
    pub fn http1_preserve_header_order(&mut self, enabled: bool) -> &mut Builder {
        self.h1_preserve_header_order = enabled;
        self
    }

    /// Sets the exact size of the read buffer to *always* use.
    ///
    /// Note that setting this option unsets the `http1_max_buf_size` option.
    ///
    /// Default is an adaptive read buffer.
    pub fn http1_read_buf_exact_size(&mut self, sz: Option<usize>) -> &mut Builder {
        self.h1_read_buf_exact_size = sz;
        self.h1_max_buf_size = None;
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
        assert!(
            max >= proto::h1::MINIMUM_MAX_BUFFER_SIZE,
            "the max_buf_size cannot be smaller than the minimum that h1 specifies."
        );

        self.h1_max_buf_size = Some(max);
        self.h1_read_buf_exact_size = None;
        self
    }

    #[cfg(feature = "ffi")]
    pub(crate) fn http1_headers_raw(&mut self, enabled: bool) -> &mut Self {
        self.h1_headers_raw = enabled;
        self
    }

    /// Sets whether HTTP2 is required.
    ///
    /// Default is false.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_only(&mut self, enabled: bool) -> &mut Builder {
        if enabled {
            self.version = Proto::Http2
        }
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
        if let Some(sz) = sz.into() {
            self.h2_builder.adaptive_window = false;
            self.h2_builder.initial_stream_window_size = sz;
        }
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
        if let Some(sz) = sz.into() {
            self.h2_builder.adaptive_window = false;
            self.h2_builder.initial_conn_window_size = sz;
        }
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
        use proto::h2::SPEC_WINDOW_SIZE;

        self.h2_builder.adaptive_window = enabled;
        if enabled {
            self.h2_builder.initial_conn_window_size = SPEC_WINDOW_SIZE;
            self.h2_builder.initial_stream_window_size = SPEC_WINDOW_SIZE;
        }
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
        if let Some(sz) = sz.into() {
            self.h2_builder.max_frame_size = sz;
        }
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
        self.h2_builder.keep_alive_interval = interval.into();
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
        self.h2_builder.keep_alive_timeout = timeout;
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
        self.h2_builder.keep_alive_while_idle = enabled;
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
        self.h2_builder.max_concurrent_reset_streams = Some(max);
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
        assert!(max <= std::u32::MAX as usize);
        self.h2_builder.max_send_buffer_size = max;
        self
    }

    /// Constructs a connection with the configured options and IO.
    /// See [`client::conn`](crate::client::conn) for more.
    ///
    /// Note, if [`Connection`] is not `await`-ed, [`SendRequest`] will
    /// do nothing.
    pub fn handshake<T, B>(
        &self,
        io: T,
    ) -> impl Future<Output = crate::Result<(SendRequest<B>, Connection<T, B>)>>
    where
        T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
        B: HttpBody + 'static,
        B::Data: Send,
        B::Error: Into<Box<dyn StdError + Send + Sync>>,
    {
        let opts = self.clone();

        async move {
            trace!("client handshake {:?}", opts.version);

            let (tx, rx) = dispatch::channel();
            let proto = match opts.version {
                #[cfg(feature = "http1")]
                Proto::Http1 => {
                    let mut conn = proto::Conn::new(io);
                    conn.set_h1_parser_config(opts.h1_parser_config);
                    if let Some(writev) = opts.h1_writev {
                        if writev {
                            conn.set_write_strategy_queue();
                        } else {
                            conn.set_write_strategy_flatten();
                        }
                    }
                    if opts.h1_title_case_headers {
                        conn.set_title_case_headers();
                    }
                    if opts.h1_preserve_header_case {
                        conn.set_preserve_header_case();
                    }
                    #[cfg(feature = "ffi")]
                    if opts.h1_preserve_header_order {
                        conn.set_preserve_header_order();
                    }
                    if opts.h09_responses {
                        conn.set_h09_responses();
                    }

                    #[cfg(feature = "ffi")]
                    conn.set_raw_headers(opts.h1_headers_raw);

                    if let Some(sz) = opts.h1_read_buf_exact_size {
                        conn.set_read_buf_exact_size(sz);
                    }
                    if let Some(max) = opts.h1_max_buf_size {
                        conn.set_max_buf_size(max);
                    }
                    let cd = proto::h1::dispatch::Client::new(rx);
                    let dispatch = proto::h1::Dispatcher::new(cd, conn);
                    ProtoClient::H1 { h1: dispatch }
                }
                #[cfg(feature = "http2")]
                Proto::Http2 => {
                    let h2 =
                        proto::h2::client::handshake(io, rx, &opts.h2_builder, opts.exec.clone())
                            .await?;
                    ProtoClient::H2 { h2 }
                }
            };

            Ok((
                SendRequest { dispatch: tx },
                Connection { inner: Some(proto) },
            ))
        }
    }
}

// ===== impl ResponseFuture

impl Future for ResponseFuture {
    type Output = crate::Result<Response<Body>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner {
            ResponseFutureState::Waiting(ref mut rx) => {
                Pin::new(rx).poll(cx).map(|res| match res {
                    Ok(Ok(resp)) => Ok(resp),
                    Ok(Err(err)) => Err(err),
                    // this is definite bug if it happens, but it shouldn't happen!
                    Err(_canceled) => panic!("dispatch dropped without returning error"),
                })
            }
            ResponseFutureState::Error(ref mut err) => {
                Poll::Ready(Err(err.take().expect("polled after ready")))
            }
        }
    }
}

impl fmt::Debug for ResponseFuture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResponseFuture").finish()
    }
}

// ===== impl ProtoClient

impl<T, B> Future for ProtoClient<T, B>
where
    T: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
    type Output = crate::Result<proto::Dispatched>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            #[cfg(feature = "http1")]
            ProtoClientProj::H1 { h1 } => h1.poll(cx),
            #[cfg(feature = "http2")]
            ProtoClientProj::H2 { h2, .. } => h2.poll(cx),

            #[cfg(not(feature = "http1"))]
            ProtoClientProj::H1 { h1 } => match h1.0 {},
            #[cfg(not(feature = "http2"))]
            ProtoClientProj::H2 { h2, .. } => match h2.0 {},
        }
    }
}

// assert trait markers

trait AssertSend: Send {}
trait AssertSendSync: Send + Sync {}

#[cfg_attr(feature = "deprecated", allow(deprecated))]
#[doc(hidden)]
impl<B: Send> AssertSendSync for SendRequest<B> {}

#[cfg_attr(feature = "deprecated", allow(deprecated))]
#[doc(hidden)]
impl<T: Send, B: Send> AssertSend for Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Send + 'static,
    B: HttpBody + 'static,
    B::Data: Send,
{
}

#[cfg_attr(feature = "deprecated", allow(deprecated))]
#[doc(hidden)]
impl<T: Send + Sync, B: Send + Sync> AssertSendSync for Connection<T, B>
where
    T: AsyncRead + AsyncWrite + Send + 'static,
    B: HttpBody + 'static,
    B::Data: Send + Sync + 'static,
{
}

#[cfg_attr(feature = "deprecated", allow(deprecated))]
#[doc(hidden)]
impl AssertSendSync for Builder {}

#[doc(hidden)]
impl AssertSend for ResponseFuture {}
