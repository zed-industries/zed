//! Client-side WebSocket connections for streaming APIs.
//!
//! Provides a small [`WebSocketClient`]/[`WebSocketConnection`] abstraction so
//! consumers (and their tests) don't depend on a concrete transport, plus
//! [`NativeWebSocketClient`], an async-tungstenite implementation running on
//! smol with rustls TLS and support for tunneling through HTTP(S) and SOCKS
//! proxies. This is independent from the collab RPC connection in the
//! `client` crate, which couples its WebSocket to the protobuf message
//! stream.

mod proxy;

#[cfg(any(test, feature = "test-support"))]
pub mod test_support;

use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result};
use futures::StreamExt as _;
use futures::future::BoxFuture;
use http_client::http::HeaderMap;
use smol::Async;
use url::Url;

use crate::proxy::AsyncReadWrite;

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

/// Bounds the whole connection setup: DNS resolution, proxy tunneling, TCP
/// connect, TLS handshake, and the WebSocket upgrade. Without it, a peer
/// that accepts the TCP connection but never completes a handshake would
/// stall the connect future indefinitely, and callers that fall back to
/// another transport on connect errors would never get to do so.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(15);

/// The connection setup did not complete within [`CONNECT_TIMEOUT`].
///
/// Use `error.downcast_ref::<ConnectTimeout>()` to check for this error.
#[derive(Debug, Clone)]
pub struct ConnectTimeout;

impl std::fmt::Display for ConnectTimeout {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "timed out while establishing the WebSocket connection"
        )
    }
}

impl std::error::Error for ConnectTimeout {}

/// Creates a future that resolves after the given duration.
///
/// Injected into [`NativeWebSocketClient`] rather than created from a
/// concrete timer so this crate stays executor-agnostic; callers running on
/// GPUI should pass `gpui::BackgroundExecutor::timer`, which is
/// deterministic in tests.
pub type Timer = Arc<dyn Fn(Duration) -> BoxFuture<'static, ()> + Send + Sync>;

/// Resolves to the connect future's result, or to a [`ConnectTimeout`]
/// error if the deadline future resolves first.
async fn connect_with_deadline<T>(
    connect: impl Future<Output = Result<T>>,
    deadline: impl Future<Output = ()>,
) -> Result<T> {
    let timeout = async {
        deadline.await;
        Err(ConnectTimeout.into())
    };
    smol::future::or(connect, timeout).await
}

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
    pub code: WebSocketCloseCode,
    pub reason: String,
}

/// Status code used to indicate why an endpoint is closing the WebSocket connection.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum WebSocketCloseCode {
    Normal,
    Away,
    Protocol,
    Unsupported,
    Status,
    Abnormal,
    Invalid,
    Policy,
    Size,
    Extension,
    Error,
    Restart,
    Again,
    Tls,
    Reserved(u16),
    Iana(u16),
    Library(u16),
    Bad(u16),
}

impl From<u16> for WebSocketCloseCode {
    /// Maps a raw close code to its variant using the same ranges as
    /// Tungstenite, so frames arriving through non-Tungstenite transports
    /// (e.g. test fakes) compare equal to ones from the real client.
    fn from(code: u16) -> Self {
        match code {
            1000 => Self::Normal,
            1001 => Self::Away,
            1002 => Self::Protocol,
            1003 => Self::Unsupported,
            1005 => Self::Status,
            1006 => Self::Abnormal,
            1007 => Self::Invalid,
            1008 => Self::Policy,
            1009 => Self::Size,
            1010 => Self::Extension,
            1011 => Self::Error,
            1012 => Self::Restart,
            1013 => Self::Again,
            1015 => Self::Tls,
            1016..=2999 => Self::Reserved(code),
            3000..=3999 => Self::Iana(code),
            4000..=4999 => Self::Library(code),
            _ => Self::Bad(code),
        }
    }
}

impl From<WebSocketCloseCode> for u16 {
    fn from(code: WebSocketCloseCode) -> Self {
        match code {
            WebSocketCloseCode::Normal => 1000,
            WebSocketCloseCode::Away => 1001,
            WebSocketCloseCode::Protocol => 1002,
            WebSocketCloseCode::Unsupported => 1003,
            WebSocketCloseCode::Status => 1005,
            WebSocketCloseCode::Abnormal => 1006,
            WebSocketCloseCode::Invalid => 1007,
            WebSocketCloseCode::Policy => 1008,
            WebSocketCloseCode::Size => 1009,
            WebSocketCloseCode::Extension => 1010,
            WebSocketCloseCode::Error => 1011,
            WebSocketCloseCode::Restart => 1012,
            WebSocketCloseCode::Again => 1013,
            WebSocketCloseCode::Tls => 1015,
            WebSocketCloseCode::Reserved(code)
            | WebSocketCloseCode::Iana(code)
            | WebSocketCloseCode::Library(code)
            | WebSocketCloseCode::Bad(code) => code,
        }
    }
}

impl std::fmt::Display for WebSocketCloseCode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", u16::from(*self))
    }
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

/// A [`WebSocketClient`] backed by async-tungstenite over a smol TCP stream
/// with rustls TLS, tunneling through a proxy when one is configured.
pub struct NativeWebSocketClient {
    /// An explicitly configured proxy URL, taking precedence over proxy
    /// environment variables for subsequent connection attempts.
    configured_proxy: parking_lot::Mutex<Option<Url>>,
    timer: Timer,
}

impl NativeWebSocketClient {
    pub fn new(configured_proxy: Option<Url>, timer: Timer) -> Self {
        Self {
            configured_proxy: parking_lot::Mutex::new(configured_proxy),
            timer,
        }
    }

    /// Changes the configured proxy for subsequent connection attempts.
    /// Existing connections are unaffected; reconnect to apply it.
    pub fn set_proxy(&self, proxy: Option<Url>) {
        *self.configured_proxy.lock() = proxy;
    }
}

impl WebSocketClient for NativeWebSocketClient {
    fn connect(
        &self,
        url: &str,
        headers: HeaderMap,
    ) -> BoxFuture<'static, Result<Box<dyn WebSocketConnection>>> {
        let url = url.to_string();
        let configured_proxy = self.configured_proxy.lock().clone();
        let deadline = (self.timer)(CONNECT_TIMEOUT);
        let connect = async move {
            let parsed = Url::parse(&url).context("failed to parse WebSocket URL")?;
            let host = parsed
                .host_str()
                .context("missing host in URL")?
                .to_string();
            let port = parsed
                .port_or_known_default()
                .context("missing port in URL")?;

            let tcp_stream: Box<dyn AsyncReadWrite> =
                match proxy::proxy_for_host(configured_proxy, &host) {
                    Some(proxy_url) => {
                        log::info!(
                            "connecting to WebSocket host {host} via proxy {}",
                            proxy::proxy_for_logging(&proxy_url)
                        );
                        proxy::connect_proxy_stream(&proxy_url, &host, port).await?
                    }
                    None => {
                        let address = proxy::resolve(&host, port)
                            .await?
                            .into_iter()
                            .next()
                            .context("failed to resolve address")?;
                        Box::new(Async::<TcpStream>::connect(address).await?)
                    }
                };

            let stream: Box<dyn AsyncReadWrite> = if parsed.scheme() == "wss" {
                let connector =
                    futures_rustls::TlsConnector::from(Arc::new(http_client_tls::tls_config()));
                let server_name = rustls::pki_types::ServerName::try_from(host.clone())
                    .context("invalid DNS name for TLS")?;
                Box::new(
                    connector
                        .connect(server_name, tcp_stream)
                        .await
                        .context("TLS handshake failed")?,
                )
            } else {
                tcp_stream
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
        };
        Box::pin(connect_with_deadline(connect, deadline))
    }
}

struct TungsteniteConnection {
    stream: async_tungstenite::WebSocketStream<Box<dyn AsyncReadWrite>>,
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
                    code: u16::from(frame.code).into(),
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
                            code: u16::from(frame.code).into(),
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
    fn connect_deadline_turns_a_stalled_connect_into_a_typed_timeout() {
        let result = smol::block_on(connect_with_deadline::<()>(
            std::future::pending(),
            std::future::ready(()),
        ));
        let error = result.unwrap_err();
        assert!(error.downcast_ref::<ConnectTimeout>().is_some());

        let result = smol::block_on(connect_with_deadline(
            std::future::ready(Ok("connected")),
            std::future::pending(),
        ));
        assert_eq!(result.unwrap(), "connected");
    }

    #[test]
    fn close_codes_round_trip_through_raw_values() {
        for code in [1000, 1001, 1006, 1011, 1015, 2000, 3000, 4000, 5000] {
            assert_eq!(u16::from(WebSocketCloseCode::from(code)), code);
        }
        assert_eq!(WebSocketCloseCode::from(1000), WebSocketCloseCode::Normal);
        assert_eq!(
            WebSocketCloseCode::from(4001),
            WebSocketCloseCode::Library(4001)
        );
    }
}
