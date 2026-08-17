//! `tokio` integration.
use tungstenite::client::IntoClientRequest;
use tungstenite::handshake::client::{Request, Response};
use tungstenite::handshake::server::{Callback, NoCallback};
use tungstenite::protocol::WebSocketConfig;
use tungstenite::Error;

use tokio::net::TcpStream;

use super::{domain, port, WebSocketStream};

use futures_io::{AsyncRead, AsyncWrite};

#[cfg(feature = "tokio-native-tls")]
#[path = "tokio/native_tls.rs"]
mod tls;

#[cfg(all(
    any(
        feature = "tokio-rustls-manual-roots",
        feature = "tokio-rustls-native-certs",
        feature = "tokio-rustls-webpki-roots"
    ),
    not(feature = "tokio-native-tls")
))]
#[path = "tokio/rustls.rs"]
mod tls;

#[cfg(all(
    feature = "tokio-openssl",
    not(any(
        feature = "tokio-native-tls",
        feature = "tokio-rustls-manual-roots",
        feature = "tokio-rustls-native-certs",
        feature = "tokio-rustls-webpki-roots"
    ))
))]
#[path = "tokio/openssl.rs"]
mod tls;

#[cfg(all(
    feature = "async-tls",
    not(any(
        feature = "tokio-native-tls",
        feature = "tokio-rustls-manual-roots",
        feature = "tokio-rustls-native-certs",
        feature = "tokio-rustls-webpki-roots",
        feature = "tokio-openssl"
    ))
))]
#[path = "tokio/async_tls.rs"]
mod tls;

#[cfg(not(any(
    feature = "tokio-native-tls",
    feature = "tokio-rustls-manual-roots",
    feature = "tokio-rustls-native-certs",
    feature = "tokio-rustls-webpki-roots",
    feature = "tokio-openssl",
    feature = "async-tls"
)))]
#[path = "tokio/dummy_tls.rs"]
mod tls;

#[cfg(any(
    feature = "tokio-native-tls",
    feature = "tokio-rustls-manual-roots",
    feature = "tokio-rustls-native-certs",
    feature = "tokio-rustls-webpki-roots",
    feature = "tokio-openssl",
    feature = "async-tls",
))]
pub use self::tls::client_async_tls_with_connector_and_config;
#[cfg(any(
    feature = "tokio-native-tls",
    feature = "tokio-rustls-manual-roots",
    feature = "tokio-rustls-native-certs",
    feature = "tokio-rustls-webpki-roots",
    feature = "tokio-openssl",
    feature = "async-tls"
))]
use self::tls::{AutoStream, Connector};

#[cfg(not(any(
    feature = "tokio-native-tls",
    feature = "tokio-rustls-manual-roots",
    feature = "tokio-rustls-native-certs",
    feature = "tokio-rustls-webpki-roots",
    feature = "tokio-openssl",
    feature = "async-tls"
)))]
pub use self::tls::client_async_tls_with_connector_and_config;
#[cfg(not(any(
    feature = "tokio-native-tls",
    feature = "tokio-rustls-manual-roots",
    feature = "tokio-rustls-native-certs",
    feature = "tokio-rustls-webpki-roots",
    feature = "tokio-openssl",
    feature = "async-tls"
)))]
use self::tls::AutoStream;

/// Creates a WebSocket handshake from a request and a stream.
/// For convenience, the user may call this with a url string, a URL,
/// or a `Request`. Calling with `Request` allows the user to add
/// a WebSocket protocol or other custom headers.
///
/// Internally, this custom creates a handshake representation and returns
/// a future representing the resolution of the WebSocket handshake. The
/// returned future will resolve to either `WebSocketStream<S>` or `Error`
/// depending on whether the handshake is successful.
///
/// This is typically used for clients who have already established, for
/// example, a TCP connection to the remote server.
pub async fn client_async<'a, R, S>(
    request: R,
    stream: S,
) -> Result<(WebSocketStream<TokioAdapter<S>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    client_async_with_config(request, stream, None).await
}

/// The same as `client_async()` but the one can specify a websocket configuration.
/// Please refer to `client_async()` for more details.
pub async fn client_async_with_config<'a, R, S>(
    request: R,
    stream: S,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<TokioAdapter<S>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    crate::client_async_with_config(request, TokioAdapter::new(stream), config).await
}

/// Accepts a new WebSocket connection with the provided stream.
///
/// This function will internally call `server::accept` to create a
/// handshake representation and returns a future representing the
/// resolution of the WebSocket handshake. The returned future will resolve
/// to either `WebSocketStream<S>` or `Error` depending if it's successful
/// or not.
///
/// This is typically used after a socket has been accepted from a
/// `TcpListener`. That socket is then passed to this function to perform
/// the server half of the accepting a client's websocket connection.
pub async fn accept_async<S>(stream: S) -> Result<WebSocketStream<TokioAdapter<S>>, Error>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    accept_hdr_async(stream, NoCallback).await
}

/// The same as `accept_async()` but the one can specify a websocket configuration.
/// Please refer to `accept_async()` for more details.
pub async fn accept_async_with_config<S>(
    stream: S,
    config: Option<WebSocketConfig>,
) -> Result<WebSocketStream<TokioAdapter<S>>, Error>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    accept_hdr_async_with_config(stream, NoCallback, config).await
}

/// Accepts a new WebSocket connection with the provided stream.
///
/// This function does the same as `accept_async()` but accepts an extra callback
/// for header processing. The callback receives headers of the incoming
/// requests and is able to add extra headers to the reply.
pub async fn accept_hdr_async<S, C>(
    stream: S,
    callback: C,
) -> Result<WebSocketStream<TokioAdapter<S>>, Error>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    C: Callback + Unpin,
{
    accept_hdr_async_with_config(stream, callback, None).await
}

/// The same as `accept_hdr_async()` but the one can specify a websocket configuration.
/// Please refer to `accept_hdr_async()` for more details.
pub async fn accept_hdr_async_with_config<S, C>(
    stream: S,
    callback: C,
    config: Option<WebSocketConfig>,
) -> Result<WebSocketStream<TokioAdapter<S>>, Error>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    C: Callback + Unpin,
{
    crate::accept_hdr_async_with_config(TokioAdapter::new(stream), callback, config).await
}

/// Type alias for the stream type of the `client_async()` functions.
pub type ClientStream<S> = AutoStream<S>;

#[cfg(any(
    feature = "tokio-native-tls",
    feature = "tokio-rustls-native-certs",
    feature = "tokio-rustls-webpki-roots",
    all(feature = "__rustls-tls", not(feature = "tokio-rustls-manual-roots")), // No roots will be available
    all(feature = "async-tls", not(feature = "tokio-openssl"))
))]
/// Creates a WebSocket handshake from a request and a stream,
/// upgrading the stream to TLS if required.
pub async fn client_async_tls<R, S>(
    request: R,
    stream: S,
) -> Result<(WebSocketStream<ClientStream<S>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
    S: 'static + tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    AutoStream<S>: Unpin,
{
    client_async_tls_with_connector_and_config(request, stream, None, None).await
}

#[cfg(any(
    feature = "tokio-native-tls",
    feature = "tokio-rustls-native-certs",
    feature = "tokio-rustls-webpki-roots",
    all(feature = "__rustls-tls", not(feature = "tokio-rustls-manual-roots")), // No roots will be available
    all(feature = "async-tls", not(feature = "tokio-openssl"))
))]
/// Creates a WebSocket handshake from a request and a stream,
/// upgrading the stream to TLS if required and using the given
/// WebSocket configuration.
pub async fn client_async_tls_with_config<R, S>(
    request: R,
    stream: S,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<ClientStream<S>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
    S: 'static + tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    AutoStream<S>: Unpin,
{
    client_async_tls_with_connector_and_config(request, stream, None, config).await
}

#[cfg(any(
    feature = "tokio-native-tls",
    feature = "tokio-rustls-manual-roots",
    feature = "tokio-rustls-native-certs",
    feature = "tokio-rustls-webpki-roots",
    all(feature = "async-tls", not(feature = "tokio-openssl"))
))]
/// Creates a WebSocket handshake from a request and a stream,
/// upgrading the stream to TLS if required and using the given
/// connector.
pub async fn client_async_tls_with_connector<R, S>(
    request: R,
    stream: S,
    connector: Option<Connector>,
) -> Result<(WebSocketStream<ClientStream<S>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
    S: 'static + tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    AutoStream<S>: Unpin,
{
    client_async_tls_with_connector_and_config(request, stream, connector, None).await
}

#[cfg(all(
    feature = "tokio-openssl",
    not(any(
        feature = "tokio-native-tls",
        feature = "tokio-rustls-manual-roots",
        feature = "tokio-rustls-native-certs",
        feature = "tokio-rustls-webpki-roots"
    ))
))]
/// Creates a WebSocket handshake from a request and a stream,
/// upgrading the stream to TLS if required.
pub async fn client_async_tls<R, S>(
    request: R,
    stream: S,
) -> Result<(WebSocketStream<ClientStream<S>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
    S: 'static
        + tokio::io::AsyncRead
        + tokio::io::AsyncWrite
        + Unpin
        + std::fmt::Debug
        + Send
        + Sync,
    AutoStream<S>: Unpin,
{
    client_async_tls_with_connector_and_config(request, stream, None, None).await
}

#[cfg(all(
    feature = "tokio-openssl",
    not(any(
        feature = "tokio-native-tls",
        feature = "tokio-rustls-manual-roots",
        feature = "tokio-rustls-native-certs",
        feature = "tokio-rustls-webpki-roots"
    ))
))]
/// Creates a WebSocket handshake from a request and a stream,
/// upgrading the stream to TLS if required and using the given
/// WebSocket configuration.
pub async fn client_async_tls_with_config<R, S>(
    request: R,
    stream: S,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<ClientStream<S>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
    S: 'static
        + tokio::io::AsyncRead
        + tokio::io::AsyncWrite
        + Unpin
        + std::fmt::Debug
        + Send
        + Sync,
    AutoStream<S>: Unpin,
{
    client_async_tls_with_connector_and_config(request, stream, None, config).await
}

#[cfg(all(
    feature = "tokio-openssl",
    not(any(
        feature = "tokio-native-tls",
        feature = "tokio-rustls-manual-roots",
        feature = "tokio-rustls-native-certs",
        feature = "tokio-rustls-webpki-roots"
    ))
))]
/// Creates a WebSocket handshake from a request and a stream,
/// upgrading the stream to TLS if required and using the given
/// connector.
pub async fn client_async_tls_with_connector<R, S>(
    request: R,
    stream: S,
    connector: Option<Connector>,
) -> Result<(WebSocketStream<ClientStream<S>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
    S: 'static
        + tokio::io::AsyncRead
        + tokio::io::AsyncWrite
        + Unpin
        + std::fmt::Debug
        + Send
        + Sync,
    AutoStream<S>: Unpin,
{
    client_async_tls_with_connector_and_config(request, stream, connector, None).await
}

/// Type alias for the stream type of the `connect_async()` functions.
pub type ConnectStream = ClientStream<TcpStream>;

/// Connect to a given URL.
///
/// Accepts any request that implements [`IntoClientRequest`], which is often just `&str`, but can
/// be a variety of types such as `httparse::Request` or [`tungstenite::http::Request`] for more
/// complex uses.
///
/// ```no_run
/// # use tungstenite::client::IntoClientRequest;
///
/// # async fn test() {
/// use tungstenite::http::{Method, Request};
/// use async_tungstenite::tokio::connect_async;
///
/// let mut request = "wss://api.example.com".into_client_request().unwrap();
/// request.headers_mut().insert("api-key", "42".parse().unwrap());
///
/// let (stream, response) = connect_async(request).await.unwrap();
/// # }
/// ```
pub async fn connect_async<R>(
    request: R,
) -> Result<(WebSocketStream<ConnectStream>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    connect_async_with_config(request, None).await
}

/// Connect to a given URL with a given WebSocket configuration.
pub async fn connect_async_with_config<R>(
    request: R,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<ConnectStream>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    let request: Request = request.into_client_request()?;

    let domain = domain(&request)?;
    let port = port(&request)?;

    let try_socket = TcpStream::connect((domain.as_str(), port)).await;
    let socket = try_socket.map_err(Error::Io)?;
    client_async_tls_with_connector_and_config(request, socket, None, config).await
}

#[cfg(any(
    feature = "async-tls",
    feature = "tokio-native-tls",
    feature = "tokio-rustls-manual-roots",
    feature = "tokio-rustls-native-certs",
    feature = "tokio-rustls-webpki-roots",
    feature = "tokio-openssl"
))]
/// Connect to a given URL using the provided TLS connector.
pub async fn connect_async_with_tls_connector<R>(
    request: R,
    connector: Option<Connector>,
) -> Result<(WebSocketStream<ConnectStream>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    connect_async_with_tls_connector_and_config(request, connector, None).await
}

#[cfg(any(
    feature = "async-tls",
    feature = "tokio-native-tls",
    feature = "tokio-rustls-manual-roots",
    feature = "tokio-rustls-native-certs",
    feature = "tokio-rustls-webpki-roots",
    feature = "tokio-openssl"
))]
/// Connect to a given URL using the provided TLS connector.
pub async fn connect_async_with_tls_connector_and_config<R>(
    request: R,
    connector: Option<Connector>,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<ConnectStream>, Response), Error>
where
    R: IntoClientRequest + Unpin,
{
    let request: Request = request.into_client_request()?;

    let domain = domain(&request)?;
    let port = port(&request)?;

    let try_socket = TcpStream::connect((domain.as_str(), port)).await;
    let socket = try_socket.map_err(Error::Io)?;
    client_async_tls_with_connector_and_config(request, socket, connector, config).await
}

use std::pin::Pin;
use std::task::{Context, Poll};

pin_project_lite::pin_project! {
    /// Adapter for `tokio::io::AsyncRead` and `tokio::io::AsyncWrite` to provide
    /// the variants from the `futures` crate and the other way around.
    #[derive(Debug, Clone)]
    pub struct TokioAdapter<T> {
        #[pin]
        inner: T,
    }
}

impl<T> TokioAdapter<T> {
    /// Creates a new `TokioAdapter` wrapping the provided value.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Consumes this `TokioAdapter`, returning the underlying value.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Get a reference to the underlying value.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Get a mutable reference to the underlying value.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: tokio::io::AsyncRead> AsyncRead for TokioAdapter<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let mut buf = tokio::io::ReadBuf::new(buf);
        match self.project().inner.poll_read(cx, &mut buf)? {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => Poll::Ready(Ok(buf.filled().len())),
        }
    }
}

impl<T: tokio::io::AsyncWrite> AsyncWrite for TokioAdapter<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        self.project().inner.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        self.project().inner.poll_shutdown(cx)
    }
}

impl<T: AsyncRead> tokio::io::AsyncRead for TokioAdapter<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let slice = buf.initialize_unfilled();
        let n = match self.project().inner.poll_read(cx, slice)? {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(n) => n,
        };
        buf.advance(n);
        Poll::Ready(Ok(()))
    }
}

impl<T: AsyncWrite> tokio::io::AsyncWrite for TokioAdapter<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        self.project().inner.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        self.project().inner.poll_close(cx)
    }
}
