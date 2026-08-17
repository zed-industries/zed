//! WebSocket connection builder.

use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{future::BoxFuture, FutureExt};
use tokio_rustls::TlsConnector;
use url::Url;

use super::{Options, WebSocket};
use crate::{stream::MaybeTlsStream, Result};
use tokio::net::TcpStream;

/// Type alias for HTTP requests used in WebSocket connection handling.
///
/// This alias represents HTTP requests with an empty body, used primarily for
/// WebSocket protocol negotiation during the initial handshake process. It encapsulates
/// the HTTP headers and metadata necessary for establishing WebSocket connections
/// according to RFC 6455, while maintaining a minimal memory footprint by using
/// an empty body type.
///
/// Used in conjunction with WebSocket upgrade mechanics to parse and validate
/// incoming connection requests before transitioning to the WebSocket protocol.
pub type HttpRequest = hyper::http::request::Request<()>;

/// Type alias for HTTP request builders used in WebSocket client connection setup.
///
/// This alias represents the builder pattern used to construct HTTP requests during
/// WebSocket handshake initialization. It encapsulates the ability to set headers,
/// method, URI, and other request properties required for proper WebSocket protocol
/// negotiation according to RFC 6455.
///
/// Used primarily in the client-side connection process to prepare the initial
/// HTTP upgrade request with the appropriate WebSocket-specific headers.
pub type HttpRequestBuilder = hyper::http::request::Builder;

/// Builder for establishing WebSocket connections with customizable options.
///
/// The `WebSocketBuilder` uses a builder pattern to configure a WebSocket connection
/// before establishing it. This allows for flexible configuration of TLS settings,
/// connection options, and HTTP request customization.
///
/// # Example
/// ```no_run
/// use yawc::{WebSocket, Options};
/// use tokio_rustls::TlsConnector;
///
/// async fn connect_example() -> yawc::Result<()> {
///     let ws = WebSocket::connect("wss://example.com/socket".parse()?)
///         .with_options(Options::default().with_utf8())
///         .with_connector(create_tls_connector())
///         .await?;
///
///     // Use the WebSocket
///     Ok(())
/// }
///
/// fn create_tls_connector() -> TlsConnector {
///     // Create a custom TLS connector
///     todo!()
/// }
/// ```
pub struct WebSocketBuilder {
    pub(super) opts: Option<WsBuilderOpts>,
    pub(super) future: Option<BoxFuture<'static, Result<WebSocket<MaybeTlsStream<TcpStream>>>>>,
}

/// Internal options structure for WebSocketBuilder.
///
/// Holds all the configuration options needed to establish a WebSocket connection,
/// including the target URL, TLS connector, connection options, and HTTP request builder.
pub(super) struct WsBuilderOpts {
    pub(super) url: Url,
    pub(super) tcp_address: Option<SocketAddr>,
    pub(super) connector: Option<TlsConnector>,
    pub(super) establish_options: Option<Options>,
    pub(super) http_builder: Option<HttpRequestBuilder>,
}

impl WebSocketBuilder {
    /// Creates a new WebSocketBuilder with the specified URL.
    ///
    /// Initializes a builder with default settings that can be customized
    /// before establishing the connection.
    ///
    /// # Parameters
    /// - `url`: The WebSocket URL to connect to (ws:// or wss://)
    pub(super) fn new(url: Url) -> Self {
        Self {
            opts: Some(WsBuilderOpts {
                url,
                tcp_address: None,
                connector: None,
                establish_options: None,
                http_builder: None,
            }),
            future: None,
        }
    }

    /// Sets a custom TLS connector for secure WebSocket connections.
    ///
    /// This allows for customized TLS settings when connecting to wss:// URLs,
    /// such as custom certificate validation, client certificates, or specific
    /// cipher suites.
    ///
    /// # Parameters
    /// - `connector`: The TLS connector to use for secure connections
    ///
    /// # Returns
    /// The builder for method chaining
    pub fn with_connector(mut self, connector: TlsConnector) -> Self {
        let Some(opts) = &mut self.opts else {
            unreachable!()
        };
        opts.connector = Some(connector);
        self
    }

    /// Sets a custom TCP address for the WebSocket connection.
    ///
    /// This allows connecting to a specific IP address or alternate hostname
    /// rather than resolving the hostname from the URL. This is useful for
    /// testing, connecting through proxies, or when DNS resolution should
    /// be handled differently.
    ///
    /// # Parameters
    /// - `address`: The socket address to connect to
    ///
    /// # Returns
    /// The builder for method chaining
    pub fn with_tcp_address(mut self, address: SocketAddr) -> Self {
        let Some(opts) = &mut self.opts else {
            unreachable!()
        };
        opts.tcp_address = Some(address);
        self
    }

    /// Sets WebSocket connection options.
    ///
    /// Configures settings like compression, maximum payload size, and UTF-8 validation
    /// for the WebSocket connection.
    ///
    /// # Parameters
    /// - `options`: Configuration options for the WebSocket connection
    ///
    /// # Returns
    /// The builder for method chaining
    pub fn with_options(mut self, options: Options) -> Self {
        let Some(opts) = &mut self.opts else {
            unreachable!()
        };
        opts.establish_options = Some(options);
        self
    }

    /// Sets a custom HTTP request builder for the WebSocket handshake.
    ///
    /// Allows customization of the initial HTTP upgrade request, enabling addition
    /// of headers, cookies, or other request properties needed for the connection.
    ///
    /// # Parameters
    /// - `builder`: A custom HTTP request builder for the handshake
    ///
    /// # Returns
    /// The builder for method chaining
    ///
    /// # Example
    /// ```no_run
    /// use yawc::WebSocket;
    ///
    /// async fn connect() -> yawc::Result<()> {
    ///     let ws = WebSocket::connect("wss://example.com/socket".parse()?)
    ///         .with_request(
    ///             yawc::HttpRequestBuilder::new()
    ///                 .header("Host", "custom-host.example.com")
    ///         )
    ///         .await?;
    ///
    ///     // Use WebSocket...
    ///     Ok(())
    /// }
    /// ```
    pub fn with_request(mut self, builder: HttpRequestBuilder) -> Self {
        let Some(opts) = &mut self.opts else {
            unreachable!()
        };
        opts.http_builder = Some(builder);
        self
    }
}

impl Future for WebSocketBuilder {
    type Output = Result<WebSocket<MaybeTlsStream<TcpStream>>>;

    /// Polls the future to establish the WebSocket connection.
    ///
    /// When first called, initializes the connection process with the configured
    /// options. Subsequent calls poll the underlying connection future until
    /// the connection is established or fails.
    ///
    /// # Returns
    /// - `Poll::Ready(Ok(WebSocket))` when connection is successfully established
    /// - `Poll::Ready(Err(_))` when connection fails
    /// - `Poll::Pending` when connection is still in progress
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if let Some(opts) = this.opts.take() {
            let future = WebSocket::connect_priv(
                opts.url,
                opts.tcp_address,
                opts.connector,
                opts.establish_options.unwrap_or_default(),
                opts.http_builder.unwrap_or_else(HttpRequest::builder),
            );
            this.future = Some(Box::pin(future));
        }

        let Some(pinned) = &mut this.future else {
            unreachable!()
        };
        pinned.poll_unpin(cx)
    }
}
