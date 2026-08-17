//! Lower-level Server connection API.
//!
//! The types in this module are to provide a lower-level API based around a
//! single connection. Accepting a connection and binding it with a service
//! are not handled at this level. This module provides the building blocks to
//! customize those things externally.
//!
//! If you don't have need to manage connections yourself, consider using the
//! higher-level [Server](super) API.
//!
//! ## Example
//! A simple example that uses the `Http` struct to talk HTTP over a Tokio TCP stream
//! ```no_run
//! # #[cfg(all(feature = "http1", feature = "runtime"))]
//! # mod rt {
//! use http::{Request, Response, StatusCode};
//! use hyper::{server::conn::Http, service::service_fn, Body};
//! use std::{net::SocketAddr, convert::Infallible};
//! use tokio::net::TcpListener;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();
//!
//!     let mut tcp_listener = TcpListener::bind(addr).await?;
//!     loop {
//!         let (tcp_stream, _) = tcp_listener.accept().await?;
//!         tokio::task::spawn(async move {
//!             if let Err(http_err) = Http::new()
//!                     .http1_only(true)
//!                     .http1_keep_alive(true)
//!                     .serve_connection(tcp_stream, service_fn(hello))
//!                     .await {
//!                 eprintln!("Error while serving HTTP connection: {}", http_err);
//!             }
//!         });
//!     }
//! }
//!
//! async fn hello(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
//!    Ok(Response::new(Body::from("Hello World!")))
//! }
//! # }
//! ```

#[cfg(all(
    any(feature = "http1", feature = "http2"),
    not(all(feature = "http1", feature = "http2"))
))]
use std::marker::PhantomData;
#[cfg(all(any(feature = "http1", feature = "http2"), feature = "runtime"))]
use std::time::Duration;

#[cfg(feature = "http2")]
use crate::common::io::Rewind;
#[cfg(all(feature = "http1", feature = "http2"))]
use crate::error::{Kind, Parse};
#[cfg(feature = "http1")]
use crate::upgrade::Upgraded;

#[cfg(all(feature = "backports", feature = "http1"))]
pub mod http1;
#[cfg(all(feature = "backports", feature = "http2"))]
pub mod http2;

cfg_feature! {
    #![any(feature = "http1", feature = "http2")]

    use std::error::Error as StdError;
    use std::fmt;
    use std::task::{Context, Poll};
    use std::pin::Pin;
    use std::future::Future;
    use std::marker::Unpin;
    #[cfg(not(all(feature = "http1", feature = "http2")))]
    use std::convert::Infallible;

    use bytes::Bytes;
    use pin_project_lite::pin_project;
    use tokio::io::{AsyncRead, AsyncWrite};
    use tracing::trace;

    pub use super::server::Connecting;
    use crate::body::{Body, HttpBody};
    use crate::common::exec::{ConnStreamExec, Exec};
    use crate::proto;
    use crate::service::HttpService;

    pub(super) use self::upgrades::UpgradeableConnection;
}

#[cfg(feature = "tcp")]
pub use super::tcp::{AddrIncoming, AddrStream};

/// A lower-level configuration of the HTTP protocol.
///
/// This structure is used to configure options for an HTTP server connection.
///
/// If you don't have need to manage connections yourself, consider using the
/// higher-level [Server](super) API.
#[derive(Clone, Debug)]
#[cfg(any(feature = "http1", feature = "http2"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
#[cfg_attr(
    feature = "deprecated",
    deprecated(
        note = "This struct will be replaced with `server::conn::http1::Builder` and `server::conn::http2::Builder` in 1.0, enable the \"backports\" feature to use them now."
    )
)]
pub struct Http<E = Exec> {
    pub(crate) exec: E,
    h1_half_close: bool,
    h1_keep_alive: bool,
    h1_title_case_headers: bool,
    h1_preserve_header_case: bool,
    #[cfg(all(feature = "http1", feature = "runtime"))]
    h1_header_read_timeout: Option<Duration>,
    h1_writev: Option<bool>,
    #[cfg(feature = "http2")]
    h2_builder: proto::h2::server::Config,
    mode: ConnectionMode,
    max_buf_size: Option<usize>,
    pipeline_flush: bool,
}

/// The internal mode of HTTP protocol which indicates the behavior when a parse error occurs.
#[cfg(any(feature = "http1", feature = "http2"))]
#[derive(Clone, Debug, PartialEq)]
enum ConnectionMode {
    /// Always use HTTP/1 and do not upgrade when a parse error occurs.
    #[cfg(feature = "http1")]
    H1Only,
    /// Always use HTTP/2.
    #[cfg(feature = "http2")]
    H2Only,
    /// Use HTTP/1 and try to upgrade to h2 when a parse error occurs.
    #[cfg(all(feature = "http1", feature = "http2"))]
    Fallback,
}

#[cfg(any(feature = "http1", feature = "http2"))]
pin_project! {
    /// A future binding a connection with a Service.
    ///
    /// Polling this future will drive HTTP forward.
    #[must_use = "futures do nothing unless polled"]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
    pub struct Connection<T, S, E = Exec>
    where
        S: HttpService<Body>,
    {
        pub(super) conn: Option<ProtoServer<T, S::ResBody, S, E>>,
        fallback: Fallback<E>,
    }
}

#[cfg(feature = "http1")]
type Http1Dispatcher<T, B, S> =
    proto::h1::Dispatcher<proto::h1::dispatch::Server<S, Body>, B, T, proto::ServerTransaction>;

#[cfg(all(not(feature = "http1"), feature = "http2"))]
type Http1Dispatcher<T, B, S> = (Infallible, PhantomData<(T, Box<Pin<B>>, Box<Pin<S>>)>);

#[cfg(feature = "http2")]
type Http2Server<T, B, S, E> = proto::h2::Server<Rewind<T>, S, B, E>;

#[cfg(all(not(feature = "http2"), feature = "http1"))]
type Http2Server<T, B, S, E> = (
    Infallible,
    PhantomData<(T, Box<Pin<S>>, Box<Pin<B>>, Box<Pin<E>>)>,
);

#[cfg(any(feature = "http1", feature = "http2"))]
pin_project! {
    #[project = ProtoServerProj]
    pub(super) enum ProtoServer<T, B, S, E = Exec>
    where
        S: HttpService<Body>,
        B: HttpBody,
    {
        H1 {
            #[pin]
            h1: Http1Dispatcher<T, B, S>,
        },
        H2 {
            #[pin]
            h2: Http2Server<T, B, S, E>,
        },
    }
}

#[cfg(all(feature = "http1", feature = "http2"))]
#[derive(Clone, Debug)]
enum Fallback<E> {
    ToHttp2(proto::h2::server::Config, E),
    Http1Only,
}

#[cfg(all(
    any(feature = "http1", feature = "http2"),
    not(all(feature = "http1", feature = "http2"))
))]
type Fallback<E> = PhantomData<E>;

#[cfg(all(feature = "http1", feature = "http2"))]
impl<E> Fallback<E> {
    fn to_h2(&self) -> bool {
        match *self {
            Fallback::ToHttp2(..) => true,
            Fallback::Http1Only => false,
        }
    }
}

#[cfg(all(feature = "http1", feature = "http2"))]
impl<E> Unpin for Fallback<E> {}

/// Deconstructed parts of a `Connection`.
///
/// This allows taking apart a `Connection` at a later time, in order to
/// reclaim the IO object, and additional related pieces.
#[derive(Debug)]
#[cfg(any(feature = "http1", feature = "http2"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "http1", feature = "http2"))))]
#[cfg_attr(
    feature = "deprecated",
    deprecated(
        note = "This struct will be replaced with `server::conn::http1::Parts` in 1.0, enable the \"backports\" feature to use them now."
    )
)]
pub struct Parts<T, S> {
    /// The original IO object used in the handshake.
    pub io: T,
    /// A buffer of bytes that have been read but not processed as HTTP.
    ///
    /// If the client sent additional bytes after its last request, and
    /// this connection "ended" with an upgrade, the read buffer will contain
    /// those bytes.
    ///
    /// You will want to check for any existing bytes if you plan to continue
    /// communicating on the IO object.
    pub read_buf: Bytes,
    /// The `Service` used to serve this connection.
    pub service: S,
    _inner: (),
}

// ===== impl Http =====

#[cfg_attr(feature = "deprecated", allow(deprecated))]
#[cfg(any(feature = "http1", feature = "http2"))]
impl Http {
    /// Creates a new instance of the HTTP protocol, ready to spawn a server or
    /// start accepting connections.
    pub fn new() -> Http {
        Http {
            exec: Exec::Default,
            h1_half_close: false,
            h1_keep_alive: true,
            h1_title_case_headers: false,
            h1_preserve_header_case: false,
            #[cfg(all(feature = "http1", feature = "runtime"))]
            h1_header_read_timeout: None,
            h1_writev: None,
            #[cfg(feature = "http2")]
            h2_builder: Default::default(),
            mode: ConnectionMode::default(),
            max_buf_size: None,
            pipeline_flush: false,
        }
    }
}

#[cfg_attr(feature = "deprecated", allow(deprecated))]
#[cfg(any(feature = "http1", feature = "http2"))]
impl<E> Http<E> {
    /// Sets whether HTTP1 is required.
    ///
    /// Default is false
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub fn http1_only(&mut self, val: bool) -> &mut Self {
        if val {
            self.mode = ConnectionMode::H1Only;
        } else {
            #[cfg(feature = "http2")]
            {
                self.mode = ConnectionMode::Fallback;
            }
        }
        self
    }

    /// Set whether HTTP/1 connections should support half-closures.
    ///
    /// Clients can chose to shutdown their write-side while waiting
    /// for the server to respond. Setting this to `true` will
    /// prevent closing the connection immediately if `read`
    /// detects an EOF in the middle of a request.
    ///
    /// Default is `false`.
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub fn http1_half_close(&mut self, val: bool) -> &mut Self {
        self.h1_half_close = val;
        self
    }

    /// Enables or disables HTTP/1 keep-alive.
    ///
    /// Default is true.
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub fn http1_keep_alive(&mut self, val: bool) -> &mut Self {
        self.h1_keep_alive = val;
        self
    }

    /// Set whether HTTP/1 connections will write header names as title case at
    /// the socket level.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub fn http1_title_case_headers(&mut self, enabled: bool) -> &mut Self {
        self.h1_title_case_headers = enabled;
        self
    }

    /// Set whether to support preserving original header cases.
    ///
    /// Currently, this will record the original cases received, and store them
    /// in a private extension on the `Request`. It will also look for and use
    /// such an extension in any provided `Response`.
    ///
    /// Since the relevant extension is still private, there is no way to
    /// interact with the original cases. The only effect this can have now is
    /// to forward the cases in a proxy-like fashion.
    ///
    /// Note that this setting does not affect HTTP/2.
    ///
    /// Default is false.
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub fn http1_preserve_header_case(&mut self, enabled: bool) -> &mut Self {
        self.h1_preserve_header_case = enabled;
        self
    }

    /// Set a timeout for reading client request headers. If a client does not
    /// transmit the entire header within this time, the connection is closed.
    ///
    /// Default is None.
    #[cfg(all(feature = "http1", feature = "runtime"))]
    #[cfg_attr(docsrs, doc(cfg(all(feature = "http1", feature = "runtime"))))]
    pub fn http1_header_read_timeout(&mut self, read_timeout: Duration) -> &mut Self {
        self.h1_header_read_timeout = Some(read_timeout);
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
    #[inline]
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub fn http1_writev(&mut self, val: bool) -> &mut Self {
        self.h1_writev = Some(val);
        self
    }

    /// Sets whether HTTP2 is required.
    ///
    /// Default is false
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_only(&mut self, val: bool) -> &mut Self {
        if val {
            self.mode = ConnectionMode::H2Only;
        } else {
            #[cfg(feature = "http1")]
            {
                self.mode = ConnectionMode::Fallback;
            }
        }
        self
    }

    /// Configures the maximum number of pending reset streams allowed before a GOAWAY will be sent.
    ///
    /// This will default to the default value set by the [`h2` crate](https://crates.io/crates/h2).
    /// As of v0.3.17, it is 20.
    ///
    /// See <https://github.com/hyperium/hyper/issues/2877> for more information.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_max_pending_accept_reset_streams(
        &mut self,
        max: impl Into<Option<usize>>,
    ) -> &mut Self {
        self.h2_builder.max_pending_accept_reset_streams = max.into();

        self
    }

    /// Configures the maximum number of pending reset streams allowed before a GOAWAY will be sent.
    ///
    /// This will default to the default value set by the [`h2` crate](https://crates.io/crates/h2).
    /// As of v0.3.17, it is 20.
    ///
    /// See <https://github.com/hyperium/hyper/issues/2877> for more information.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_max_local_error_reset_streams(
        &mut self,
        max: impl Into<Option<usize>>,
    ) -> &mut Self {
        self.h2_builder.max_local_error_reset_streams = max.into();

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

    /// Sets the max connection-level flow control for HTTP2.
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

    /// Sets the [`SETTINGS_MAX_CONCURRENT_STREAMS`][spec] option for HTTP2
    /// connections.
    ///
    /// Default is no limit (`std::u32::MAX`). Passing `None` will do nothing.
    ///
    /// [spec]: https://http2.github.io/http2-spec/#SETTINGS_MAX_CONCURRENT_STREAMS
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_max_concurrent_streams(&mut self, max: impl Into<Option<u32>>) -> &mut Self {
        self.h2_builder.max_concurrent_streams = max.into();
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

    /// Set the maximum write buffer size for each HTTP/2 stream.
    ///
    /// Default is currently ~400KB, but may change.
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

    /// Enables the [extended CONNECT protocol].
    ///
    /// [extended CONNECT protocol]: https://datatracker.ietf.org/doc/html/rfc8441#section-4
    #[cfg(feature = "http2")]
    pub fn http2_enable_connect_protocol(&mut self) -> &mut Self {
        self.h2_builder.enable_connect_protocol = true;
        self
    }

    /// Sets the max size of received header frames.
    ///
    /// Default is currently ~16MB, but may change.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn http2_max_header_list_size(&mut self, max: u32) -> &mut Self {
        self.h2_builder.max_header_list_size = max;
        self
    }

    /// Set the maximum buffer size for the connection.
    ///
    /// Default is ~400kb.
    ///
    /// # Panics
    ///
    /// The minimum value allowed is 8192. This method panics if the passed `max` is less than the minimum.
    #[cfg(feature = "http1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http1")))]
    pub fn max_buf_size(&mut self, max: usize) -> &mut Self {
        assert!(
            max >= proto::h1::MINIMUM_MAX_BUFFER_SIZE,
            "the max_buf_size cannot be smaller than the minimum that h1 specifies."
        );
        self.max_buf_size = Some(max);
        self
    }

    /// Aggregates flushes to better support pipelined responses.
    ///
    /// Experimental, may have bugs.
    ///
    /// Default is false.
    pub fn pipeline_flush(&mut self, enabled: bool) -> &mut Self {
        self.pipeline_flush = enabled;
        self
    }

    /// Set the executor used to spawn background tasks.
    ///
    /// Default uses implicit default (like `tokio::spawn`).
    pub fn with_executor<E2>(self, exec: E2) -> Http<E2> {
        Http {
            exec,
            h1_half_close: self.h1_half_close,
            h1_keep_alive: self.h1_keep_alive,
            h1_title_case_headers: self.h1_title_case_headers,
            h1_preserve_header_case: self.h1_preserve_header_case,
            #[cfg(all(feature = "http1", feature = "runtime"))]
            h1_header_read_timeout: self.h1_header_read_timeout,
            h1_writev: self.h1_writev,
            #[cfg(feature = "http2")]
            h2_builder: self.h2_builder,
            mode: self.mode,
            max_buf_size: self.max_buf_size,
            pipeline_flush: self.pipeline_flush,
        }
    }

    /// Bind a connection together with a [`Service`](crate::service::Service).
    ///
    /// This returns a Future that must be polled in order for HTTP to be
    /// driven on the connection.
    ///
    /// # Example
    ///
    /// ```
    /// # use hyper::{Body, Request, Response};
    /// # use hyper::service::Service;
    /// # use hyper::server::conn::Http;
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # async fn run<I, S>(some_io: I, some_service: S)
    /// # where
    /// #     I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    /// #     S: Service<hyper::Request<Body>, Response=hyper::Response<Body>> + Send + 'static,
    /// #     S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    /// #     S::Future: Send,
    /// # {
    /// let http = Http::new();
    /// let conn = http.serve_connection(some_io, some_service);
    ///
    /// if let Err(e) = conn.await {
    ///     eprintln!("server connection error: {}", e);
    /// }
    /// # }
    /// # fn main() {}
    /// ```
    pub fn serve_connection<S, I, Bd>(&self, io: I, service: S) -> Connection<I, S, E>
    where
        S: HttpService<Body, ResBody = Bd>,
        S::Error: Into<Box<dyn StdError + Send + Sync>>,
        Bd: HttpBody + 'static,
        Bd::Error: Into<Box<dyn StdError + Send + Sync>>,
        I: AsyncRead + AsyncWrite + Unpin,
        E: ConnStreamExec<S::Future, Bd>,
    {
        #[cfg(feature = "http1")]
        macro_rules! h1 {
            () => {{
                let mut conn = proto::Conn::new(io);
                if !self.h1_keep_alive {
                    conn.disable_keep_alive();
                }
                if self.h1_half_close {
                    conn.set_allow_half_close();
                }
                if self.h1_title_case_headers {
                    conn.set_title_case_headers();
                }
                if self.h1_preserve_header_case {
                    conn.set_preserve_header_case();
                }
                #[cfg(all(feature = "http1", feature = "runtime"))]
                if let Some(header_read_timeout) = self.h1_header_read_timeout {
                    conn.set_http1_header_read_timeout(header_read_timeout);
                }
                if let Some(writev) = self.h1_writev {
                    if writev {
                        conn.set_write_strategy_queue();
                    } else {
                        conn.set_write_strategy_flatten();
                    }
                }
                conn.set_flush_pipeline(self.pipeline_flush);
                if let Some(max) = self.max_buf_size {
                    conn.set_max_buf_size(max);
                }
                let sd = proto::h1::dispatch::Server::new(service);
                ProtoServer::H1 {
                    h1: proto::h1::Dispatcher::new(sd, conn),
                }
            }};
        }

        let proto = match self.mode {
            #[cfg(feature = "http1")]
            #[cfg(not(feature = "http2"))]
            ConnectionMode::H1Only => h1!(),
            #[cfg(feature = "http2")]
            #[cfg(feature = "http1")]
            ConnectionMode::H1Only | ConnectionMode::Fallback => h1!(),
            #[cfg(feature = "http2")]
            ConnectionMode::H2Only => {
                let rewind_io = Rewind::new(io);
                let h2 =
                    proto::h2::Server::new(rewind_io, service, &self.h2_builder, self.exec.clone());
                ProtoServer::H2 { h2 }
            }
        };

        Connection {
            conn: Some(proto),
            #[cfg(all(feature = "http1", feature = "http2"))]
            fallback: if self.mode == ConnectionMode::Fallback {
                Fallback::ToHttp2(self.h2_builder.clone(), self.exec.clone())
            } else {
                Fallback::Http1Only
            },
            #[cfg(not(all(feature = "http1", feature = "http2")))]
            fallback: PhantomData,
        }
    }
}

// ===== impl Connection =====

#[cfg(any(feature = "http1", feature = "http2"))]
impl<I, B, S, E> Connection<I, S, E>
where
    S: HttpService<Body, ResBody = B>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    I: AsyncRead + AsyncWrite + Unpin,
    B: HttpBody + 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
    E: ConnStreamExec<S::Future, B>,
{
    /// Start a graceful shutdown process for this connection.
    ///
    /// This `Connection` should continue to be polled until shutdown
    /// can finish.
    ///
    /// # Note
    ///
    /// This should only be called while the `Connection` future is still
    /// pending. If called after `Connection::poll` has resolved, this does
    /// nothing.
    pub fn graceful_shutdown(mut self: Pin<&mut Self>) {
        match self.conn {
            #[cfg(feature = "http1")]
            Some(ProtoServer::H1 { ref mut h1, .. }) => {
                h1.disable_keep_alive();
            }
            #[cfg(feature = "http2")]
            Some(ProtoServer::H2 { ref mut h2 }) => {
                h2.graceful_shutdown();
            }
            None => (),

            #[cfg(not(feature = "http1"))]
            Some(ProtoServer::H1 { ref mut h1, .. }) => match h1.0 {},
            #[cfg(not(feature = "http2"))]
            Some(ProtoServer::H2 { ref mut h2 }) => match h2.0 {},
        }
    }

    /// Return the inner IO object, and additional information.
    ///
    /// If the IO object has been "rewound" the io will not contain those bytes rewound.
    /// This should only be called after `poll_without_shutdown` signals
    /// that the connection is "done". Otherwise, it may not have finished
    /// flushing all necessary HTTP bytes.
    ///
    /// # Panics
    /// This method will panic if this connection is using an h2 protocol.
    #[cfg_attr(feature = "deprecated", allow(deprecated))]
    pub fn into_parts(self) -> Parts<I, S> {
        self.try_into_parts()
            .unwrap_or_else(|| panic!("h2 cannot into_inner"))
    }

    /// Return the inner IO object, and additional information, if available.
    ///
    /// This method will return a `None` if this connection is using an h2 protocol.
    #[cfg_attr(feature = "deprecated", allow(deprecated))]
    pub fn try_into_parts(self) -> Option<Parts<I, S>> {
        match self.conn.unwrap() {
            #[cfg(feature = "http1")]
            ProtoServer::H1 { h1, .. } => {
                let (io, read_buf, dispatch) = h1.into_inner();
                Some(Parts {
                    io,
                    read_buf,
                    service: dispatch.into_service(),
                    _inner: (),
                })
            }
            ProtoServer::H2 { .. } => None,

            #[cfg(not(feature = "http1"))]
            ProtoServer::H1 { h1, .. } => match h1.0 {},
        }
    }

    /// Poll the connection for completion, but without calling `shutdown`
    /// on the underlying IO.
    ///
    /// This is useful to allow running a connection while doing an HTTP
    /// upgrade. Once the upgrade is completed, the connection would be "done",
    /// but it is not desired to actually shutdown the IO object. Instead you
    /// would take it back using `into_parts`.
    pub fn poll_without_shutdown(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<()>> {
        loop {
            match *self.conn.as_mut().unwrap() {
                #[cfg(feature = "http1")]
                ProtoServer::H1 { ref mut h1, .. } => match ready!(h1.poll_without_shutdown(cx)) {
                    Ok(()) => return Poll::Ready(Ok(())),
                    Err(e) => {
                        #[cfg(feature = "http2")]
                        match *e.kind() {
                            Kind::Parse(Parse::VersionH2) if self.fallback.to_h2() => {
                                self.upgrade_h2();
                                continue;
                            }
                            _ => (),
                        }

                        return Poll::Ready(Err(e));
                    }
                },
                #[cfg(feature = "http2")]
                ProtoServer::H2 { ref mut h2 } => return Pin::new(h2).poll(cx).map_ok(|_| ()),

                #[cfg(not(feature = "http1"))]
                ProtoServer::H1 { ref mut h1, .. } => match h1.0 {},
                #[cfg(not(feature = "http2"))]
                ProtoServer::H2 { ref mut h2 } => match h2.0 {},
            };
        }
    }

    /// Prevent shutdown of the underlying IO object at the end of service the request,
    /// instead run `into_parts`. This is a convenience wrapper over `poll_without_shutdown`.
    ///
    /// # Error
    ///
    /// This errors if the underlying connection protocol is not HTTP/1.
    #[cfg_attr(feature = "deprecated", allow(deprecated))]
    pub fn without_shutdown(self) -> impl Future<Output = crate::Result<Parts<I, S>>> {
        let mut conn = Some(self);
        futures_util::future::poll_fn(move |cx| {
            ready!(conn.as_mut().unwrap().poll_without_shutdown(cx))?;
            Poll::Ready(
                conn.take()
                    .unwrap()
                    .try_into_parts()
                    .ok_or_else(crate::Error::new_without_shutdown_not_h1),
            )
        })
    }

    #[cfg(all(feature = "http1", feature = "http2"))]
    fn upgrade_h2(&mut self) {
        trace!("Trying to upgrade connection to h2");
        let conn = self.conn.take();

        let (io, read_buf, dispatch) = match conn.unwrap() {
            ProtoServer::H1 { h1, .. } => h1.into_inner(),
            ProtoServer::H2 { .. } => {
                panic!("h2 cannot into_inner");
            }
        };
        let mut rewind_io = Rewind::new(io);
        rewind_io.rewind(read_buf);
        let (builder, exec) = match self.fallback {
            Fallback::ToHttp2(ref builder, ref exec) => (builder, exec),
            Fallback::Http1Only => unreachable!("upgrade_h2 with Fallback::Http1Only"),
        };
        let h2 = proto::h2::Server::new(rewind_io, dispatch.into_service(), builder, exec.clone());

        debug_assert!(self.conn.is_none());
        self.conn = Some(ProtoServer::H2 { h2 });
    }

    /// Enable this connection to support higher-level HTTP upgrades.
    ///
    /// See [the `upgrade` module](crate::upgrade) for more.
    pub fn with_upgrades(self) -> UpgradeableConnection<I, S, E>
    where
        I: Send,
    {
        UpgradeableConnection { inner: self }
    }
}

#[cfg(any(feature = "http1", feature = "http2"))]
impl<I, B, S, E> Future for Connection<I, S, E>
where
    S: HttpService<Body, ResBody = B>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    I: AsyncRead + AsyncWrite + Unpin,
    B: HttpBody + 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
    E: ConnStreamExec<S::Future, B>,
{
    type Output = crate::Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match ready!(Pin::new(self.conn.as_mut().unwrap()).poll(cx)) {
                Ok(done) => {
                    match done {
                        proto::Dispatched::Shutdown => {}
                        #[cfg(feature = "http1")]
                        proto::Dispatched::Upgrade(pending) => {
                            // With no `Send` bound on `I`, we can't try to do
                            // upgrades here. In case a user was trying to use
                            // `Body::on_upgrade` with this API, send a special
                            // error letting them know about that.
                            pending.manual();
                        }
                    };
                    return Poll::Ready(Ok(()));
                }
                Err(e) => {
                    #[cfg(feature = "http1")]
                    #[cfg(feature = "http2")]
                    match *e.kind() {
                        Kind::Parse(Parse::VersionH2) if self.fallback.to_h2() => {
                            self.upgrade_h2();
                            continue;
                        }
                        _ => (),
                    }

                    return Poll::Ready(Err(e));
                }
            }
        }
    }
}

#[cfg(any(feature = "http1", feature = "http2"))]
impl<I, S> fmt::Debug for Connection<I, S>
where
    S: HttpService<Body>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection").finish()
    }
}

// ===== impl ConnectionMode =====

#[cfg(any(feature = "http1", feature = "http2"))]
impl Default for ConnectionMode {
    #[cfg(all(feature = "http1", feature = "http2"))]
    fn default() -> ConnectionMode {
        ConnectionMode::Fallback
    }

    #[cfg(all(feature = "http1", not(feature = "http2")))]
    fn default() -> ConnectionMode {
        ConnectionMode::H1Only
    }

    #[cfg(all(not(feature = "http1"), feature = "http2"))]
    fn default() -> ConnectionMode {
        ConnectionMode::H2Only
    }
}

// ===== impl ProtoServer =====

#[cfg(any(feature = "http1", feature = "http2"))]
impl<T, B, S, E> Future for ProtoServer<T, B, S, E>
where
    T: AsyncRead + AsyncWrite + Unpin,
    S: HttpService<Body, ResBody = B>,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
    B: HttpBody + 'static,
    B::Error: Into<Box<dyn StdError + Send + Sync>>,
    E: ConnStreamExec<S::Future, B>,
{
    type Output = crate::Result<proto::Dispatched>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            #[cfg(feature = "http1")]
            ProtoServerProj::H1 { h1, .. } => h1.poll(cx),
            #[cfg(feature = "http2")]
            ProtoServerProj::H2 { h2 } => h2.poll(cx),

            #[cfg(not(feature = "http1"))]
            ProtoServerProj::H1 { h1, .. } => match h1.0 {},
            #[cfg(not(feature = "http2"))]
            ProtoServerProj::H2 { h2 } => match h2.0 {},
        }
    }
}

#[cfg(any(feature = "http1", feature = "http2"))]
mod upgrades {
    use super::*;

    // A future binding a connection with a Service with Upgrade support.
    //
    // This type is unnameable outside the crate, and so basically just an
    // `impl Future`, without requiring Rust 1.26.
    #[must_use = "futures do nothing unless polled"]
    #[allow(missing_debug_implementations)]
    pub struct UpgradeableConnection<T, S, E>
    where
        S: HttpService<Body>,
    {
        pub(super) inner: Connection<T, S, E>,
    }

    impl<I, B, S, E> UpgradeableConnection<I, S, E>
    where
        S: HttpService<Body, ResBody = B>,
        S::Error: Into<Box<dyn StdError + Send + Sync>>,
        I: AsyncRead + AsyncWrite + Unpin,
        B: HttpBody + 'static,
        B::Error: Into<Box<dyn StdError + Send + Sync>>,
        E: ConnStreamExec<S::Future, B>,
    {
        /// Start a graceful shutdown process for this connection.
        ///
        /// This `Connection` should continue to be polled until shutdown
        /// can finish.
        pub fn graceful_shutdown(mut self: Pin<&mut Self>) {
            Pin::new(&mut self.inner).graceful_shutdown()
        }
    }

    impl<I, B, S, E> Future for UpgradeableConnection<I, S, E>
    where
        S: HttpService<Body, ResBody = B>,
        S::Error: Into<Box<dyn StdError + Send + Sync>>,
        I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
        B: HttpBody + 'static,
        B::Error: Into<Box<dyn StdError + Send + Sync>>,
        E: ConnStreamExec<S::Future, B>,
    {
        type Output = crate::Result<()>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            loop {
                match ready!(Pin::new(self.inner.conn.as_mut().unwrap()).poll(cx)) {
                    Ok(proto::Dispatched::Shutdown) => return Poll::Ready(Ok(())),
                    #[cfg(feature = "http1")]
                    Ok(proto::Dispatched::Upgrade(pending)) => {
                        match self.inner.conn.take() {
                            Some(ProtoServer::H1 { h1, .. }) => {
                                let (io, buf, _) = h1.into_inner();
                                pending.fulfill(Upgraded::new(io, buf));
                                return Poll::Ready(Ok(()));
                            }
                            _ => {
                                drop(pending);
                                unreachable!("Upgrade expects h1")
                            }
                        };
                    }
                    Err(e) => {
                        #[cfg(feature = "http1")]
                        #[cfg(feature = "http2")]
                        match *e.kind() {
                            Kind::Parse(Parse::VersionH2) if self.inner.fallback.to_h2() => {
                                self.inner.upgrade_h2();
                                continue;
                            }
                            _ => (),
                        }

                        return Poll::Ready(Err(e));
                    }
                }
            }
        }
    }
}
