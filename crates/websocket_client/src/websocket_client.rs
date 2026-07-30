//! Client-side WebSocket connections for streaming APIs.
//!
//! Provides a small [`WebSocketClient`]/[`WebSocketConnection`] abstraction so
//! consumers (and their tests) don't depend on a concrete transport, plus
//! [`NativeWebSocketClient`], an async-tungstenite implementation running on
//! smol with rustls TLS. This is independent from the collab RPC connection in
//! the `client` crate, which couples its WebSocket to the protobuf message
//! stream.

use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context as TaskContext, Poll};

use anyhow::{Context as _, Result};
use futures::StreamExt as _;
use futures::future::BoxFuture;
use futures::io::{AsyncRead, AsyncWrite};
use http_client::http::HeaderMap;
use smol::Async;
use url::Url;

/// The server rejected the auth token (HTTP 401).
///
/// The caller should refresh its token and retry, or surface an
/// authentication error if the refreshed token is also rejected.
///
/// Use `error.downcast_ref::<AuthRequired>()` to check for this error.
#[derive(Debug, Clone)]
pub struct AuthRequired;

impl std::fmt::Display for AuthRequired {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "authentication required (401)")
    }
}

impl std::error::Error for AuthRequired {}

/// A factory for creating WebSocket connections.
pub trait WebSocketClient: Send + Sync + 'static {
    /// Connects to the given WebSocket URL and returns a connection.
    ///
    /// Returns an error containing [`AuthRequired`] if the server responds
    /// with HTTP 401. The caller should refresh its credentials and retry.
    fn connect(
        &self,
        url: &str,
        headers: HeaderMap,
    ) -> BoxFuture<'static, Result<Box<dyn WebSocketConnection>>>;
}

/// An enum representing the various forms of a WebSocket message.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum WebSocketMessage {
    Text(String),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close(Option<WebSocketCloseFrame>),
}

/// The code and reason a peer supplied when closing the connection.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WebSocketCloseFrame {
    pub code: u16,
    pub reason: String,
}

/// A WebSocket connection that can send and receive messages.
pub trait WebSocketConnection: Send {
    fn send(&mut self, message: WebSocketMessage) -> BoxFuture<'_, Result<()>>;

    fn receive(&mut self) -> BoxFuture<'_, Option<Result<WebSocketMessage>>>;
}

/// Converts an `http(s)` URL into the corresponding `ws(s)` URL.
///
/// URLs that already use a WebSocket scheme are returned unchanged.
pub fn websocket_url_from_http(mut url: Url) -> Result<Url> {
    let scheme = match url.scheme() {
        "http" => "ws",
        "https" => "wss",
        "ws" | "wss" => return Ok(url),
        other => anyhow::bail!("cannot derive a WebSocket URL from scheme {other}"),
    };
    url.set_scheme(scheme)
        .map_err(|()| anyhow::anyhow!("failed to set WebSocket URL scheme"))?;
    Ok(url)
}

/// A stream that may or may not be wrapped in TLS.
enum MaybeTlsStream {
    Plain(Async<TcpStream>),
    Tls(futures_rustls::client::TlsStream<Async<TcpStream>>),
}

impl AsyncRead for MaybeTlsStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_read(cx, buf),
            Self::Tls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for MaybeTlsStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut TaskContext<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_write(cx, buf),
            Self::Tls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_flush(cx),
            Self::Tls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            Self::Plain(stream) => Pin::new(stream).poll_close(cx),
            Self::Tls(stream) => Pin::new(stream).poll_close(cx),
        }
    }
}

/// A [`WebSocketClient`] backed by async-tungstenite over a smol TCP stream
/// with rustls TLS.
pub struct NativeWebSocketClient {
    /// An explicitly configured proxy URL, taking precedence over the proxy
    /// environment variables.
    proxy: Option<Url>,
}

impl NativeWebSocketClient {
    pub fn new(proxy: Option<Url>) -> Self {
        Self { proxy }
    }
}

impl WebSocketClient for NativeWebSocketClient {
    fn connect(
        &self,
        url: &str,
        headers: HeaderMap,
    ) -> BoxFuture<'static, Result<Box<dyn WebSocketConnection>>> {
        let url = url.to_string();
        let proxy = self.proxy.clone().or_else(http_client::read_proxy_from_env);
        Box::pin(async move {
            // Tunneling through a proxy is not supported yet, and users
            // configure proxies in contexts where silently bypassing one
            // would leak traffic. Refusing here lets callers fall back to
            // their HTTP transport, which fully honors proxy settings.
            if let Some(proxy) = proxy {
                anyhow::bail!(
                    "WebSocket connections through a proxy are not supported (proxy {})",
                    proxy_for_logging(&proxy)
                );
            }

            let parsed = Url::parse(&url).context("failed to parse WebSocket URL")?;
            let host = parsed
                .host_str()
                .context("missing host in URL")?
                .to_string();
            let port = parsed
                .port_or_known_default()
                .context("missing port in URL")?;

            let address = smol::unblock({
                let host = host.clone();
                move || {
                    (host.as_str(), port)
                        .to_socket_addrs()?
                        .next()
                        .context("failed to resolve address")
                }
            })
            .await?;
            let tcp_stream = Async::<TcpStream>::connect(address).await?;

            let stream = if parsed.scheme() == "wss" {
                let connector =
                    futures_rustls::TlsConnector::from(Arc::new(http_client_tls::tls_config()));
                let server_name = rustls::pki_types::ServerName::try_from(host.clone())
                    .context("invalid DNS name for TLS")?;
                let tls_stream = connector
                    .connect(server_name, tcp_stream)
                    .await
                    .context("TLS handshake failed")?;
                MaybeTlsStream::Tls(tls_stream)
            } else {
                MaybeTlsStream::Plain(tcp_stream)
            };

            let ws_uri: async_tungstenite::tungstenite::http::Uri =
                url.parse().context("failed to parse WebSocket URI")?;
            let mut ws_request = async_tungstenite::tungstenite::ClientRequestBuilder::new(ws_uri);
            for (name, value) in &headers {
                ws_request = ws_request.with_header(
                    name.as_str(),
                    value
                        .to_str()
                        .context("WebSocket header value is not valid text")?
                        .to_string(),
                );
            }
            let (ws_stream, _handshake_response) =
                match async_tungstenite::client_async(ws_request, stream).await {
                    Ok(result) => result,
                    Err(async_tungstenite::tungstenite::Error::Http(response)) => {
                        log::error!(
                            "WebSocket upgrade failed with HTTP {}: {:?}",
                            response.status(),
                            response.body().as_ref().map(|b| String::from_utf8_lossy(b))
                        );
                        if response.status().as_u16() == 401 {
                            return Err(AuthRequired.into());
                        }
                        return Err(anyhow::anyhow!(
                            "HTTP error during WebSocket upgrade: {}",
                            response.status()
                        ));
                    }
                    Err(error) => {
                        return Err(
                            anyhow::Error::from(error).context("failed to upgrade to WebSocket")
                        );
                    }
                };

            log::debug!("WebSocket connected to {url}");

            Ok(Box::new(TungsteniteConnection { stream: ws_stream })
                as Box<dyn WebSocketConnection>)
        })
    }
}

/// The proxy URL with any credentials omitted, safe to include in logs and
/// error messages.
fn proxy_for_logging(proxy: &Url) -> String {
    let host = proxy.host_str().unwrap_or("<invalid>");
    match proxy.port() {
        Some(port) => format!("{}://{}:{}", proxy.scheme(), host, port),
        None => format!("{}://{}", proxy.scheme(), host),
    }
}

struct TungsteniteConnection {
    stream: async_tungstenite::WebSocketStream<MaybeTlsStream>,
}

impl WebSocketConnection for TungsteniteConnection {
    fn send(&mut self, message: WebSocketMessage) -> BoxFuture<'_, Result<()>> {
        use async_tungstenite::tungstenite::Message;

        let message = match message {
            WebSocketMessage::Text(text) => Message::text(text),
            WebSocketMessage::Binary(bytes) => Message::Binary(bytes.into()),
            WebSocketMessage::Ping(bytes) => Message::Ping(bytes.into()),
            WebSocketMessage::Pong(bytes) => Message::Pong(bytes.into()),
            WebSocketMessage::Close(close) => Message::Close(close.map(|frame| {
                async_tungstenite::tungstenite::protocol::CloseFrame {
                    code: frame.code.into(),
                    reason: frame.reason.into(),
                }
            })),
        };
        Box::pin(async move { self.stream.send(message).await.map_err(Into::into) })
    }

    fn receive(&mut self) -> BoxFuture<'_, Option<Result<WebSocketMessage>>> {
        use async_tungstenite::tungstenite::Message;

        Box::pin(async move {
            match self.stream.next().await {
                Some(Ok(message)) => Some(Ok(match message {
                    Message::Text(text) => WebSocketMessage::Text(text.as_str().to_string()),
                    Message::Binary(bytes) => WebSocketMessage::Binary(bytes.to_vec()),
                    Message::Ping(bytes) => WebSocketMessage::Ping(bytes.to_vec()),
                    Message::Pong(bytes) => WebSocketMessage::Pong(bytes.to_vec()),
                    Message::Close(close) => {
                        WebSocketMessage::Close(close.map(|frame| WebSocketCloseFrame {
                            code: frame.code.into(),
                            reason: frame.reason.as_str().to_string(),
                        }))
                    }
                    Message::Frame(_) => {
                        return Some(Err(anyhow::anyhow!(
                            "unexpected raw WebSocket protocol frame"
                        )));
                    }
                })),
                Some(Err(error)) => Some(Err(error.into())),
                None => None,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn websocket_url_from_http_converts_schemes() {
        let url = websocket_url_from_http(Url::parse("https://cloud.zed.dev/completions").unwrap())
            .unwrap();
        assert_eq!(url.as_str(), "wss://cloud.zed.dev/completions");

        let url = websocket_url_from_http(Url::parse("http://localhost:8787/x").unwrap()).unwrap();
        assert_eq!(url.as_str(), "ws://localhost:8787/x");

        let url = websocket_url_from_http(Url::parse("wss://api.openai.com/v1").unwrap()).unwrap();
        assert_eq!(url.as_str(), "wss://api.openai.com/v1");

        assert!(websocket_url_from_http(Url::parse("ftp://example.com").unwrap()).is_err());
    }

    #[test]
    fn proxy_for_logging_omits_credentials() {
        let proxy = Url::parse("http://user:hunter2@proxy.example.com:8080").unwrap();
        assert_eq!(proxy_for_logging(&proxy), "http://proxy.example.com:8080");

        let proxy = Url::parse("socks5://proxy.example.com").unwrap();
        assert_eq!(proxy_for_logging(&proxy), "socks5://proxy.example.com");
    }
}
