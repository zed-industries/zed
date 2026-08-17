//! `async-std` integration.
use tungstenite::client::IntoClientRequest;
use tungstenite::handshake::client::{Request, Response};
use tungstenite::protocol::WebSocketConfig;
use tungstenite::Error;

use async_std::net::TcpStream;

use super::{domain, port, WebSocketStream};

#[cfg(feature = "async-native-tls")]
use futures_io::{AsyncRead, AsyncWrite};

#[cfg(feature = "async-native-tls")]
pub(crate) mod async_native_tls {
    use async_native_tls::TlsConnector as AsyncTlsConnector;
    use async_native_tls::TlsStream;
    use real_async_native_tls as async_native_tls;

    use tungstenite::client::uri_mode;
    use tungstenite::handshake::client::Request;
    use tungstenite::stream::Mode;
    use tungstenite::Error;

    use futures_io::{AsyncRead, AsyncWrite};

    use crate::stream::Stream as StreamSwitcher;
    use crate::{
        client_async_with_config, domain, IntoClientRequest, Response, WebSocketConfig,
        WebSocketStream,
    };

    /// A stream that might be protected with TLS.
    pub type MaybeTlsStream<S> = StreamSwitcher<S, TlsStream<S>>;

    pub type AutoStream<S> = MaybeTlsStream<S>;

    pub type Connector = AsyncTlsConnector;

    async fn wrap_stream<S>(
        socket: S,
        domain: String,
        connector: Option<Connector>,
        mode: Mode,
    ) -> Result<AutoStream<S>, Error>
    where
        S: 'static + AsyncRead + AsyncWrite + Unpin,
    {
        match mode {
            Mode::Plain => Ok(StreamSwitcher::Plain(socket)),
            Mode::Tls => {
                let stream = {
                    let connector = if let Some(connector) = connector {
                        connector
                    } else {
                        AsyncTlsConnector::new()
                    };
                    connector
                        .connect(&domain, socket)
                        .await
                        .map_err(|err| Error::Tls(err.into()))?
                };
                Ok(StreamSwitcher::Tls(stream))
            }
        }
    }

    /// Creates a WebSocket handshake from a request and a stream,
    /// upgrading the stream to TLS if required and using the given
    /// connector and WebSocket configuration.
    pub async fn client_async_tls_with_connector_and_config<R, S>(
        request: R,
        stream: S,
        connector: Option<AsyncTlsConnector>,
        config: Option<WebSocketConfig>,
    ) -> Result<(WebSocketStream<AutoStream<S>>, Response), Error>
    where
        R: IntoClientRequest + Unpin,
        S: 'static + AsyncRead + AsyncWrite + Unpin,
        AutoStream<S>: Unpin,
    {
        let request: Request = request.into_client_request()?;

        let domain = domain(&request)?;

        // Make sure we check domain and mode first. URL must be valid.
        let mode = uri_mode(request.uri())?;

        let stream = wrap_stream(stream, domain, connector, mode).await?;
        client_async_with_config(request, stream, config).await
    }
}

#[cfg(not(any(feature = "async-tls", feature = "async-native-tls")))]
pub(crate) mod dummy_tls {
    use futures_io::{AsyncRead, AsyncWrite};

    use tungstenite::client::{uri_mode, IntoClientRequest};
    use tungstenite::handshake::client::Request;
    use tungstenite::stream::Mode;
    use tungstenite::Error;

    use crate::{client_async_with_config, domain, Response, WebSocketConfig, WebSocketStream};

    pub type AutoStream<S> = S;
    type Connector = ();

    async fn wrap_stream<S>(
        socket: S,
        _domain: String,
        _connector: Option<()>,
        mode: Mode,
    ) -> Result<AutoStream<S>, Error>
    where
        S: 'static + AsyncRead + AsyncWrite + Unpin,
    {
        match mode {
            Mode::Plain => Ok(socket),
            Mode::Tls => Err(Error::Url(
                tungstenite::error::UrlError::TlsFeatureNotEnabled,
            )),
        }
    }

    /// Creates a WebSocket handshake from a request and a stream,
    /// upgrading the stream to TLS if required and using the given
    /// connector and WebSocket configuration.
    pub async fn client_async_tls_with_connector_and_config<R, S>(
        request: R,
        stream: S,
        connector: Option<Connector>,
        config: Option<WebSocketConfig>,
    ) -> Result<(WebSocketStream<AutoStream<S>>, Response), Error>
    where
        R: IntoClientRequest + Unpin,
        S: 'static + AsyncRead + AsyncWrite + Unpin,
        AutoStream<S>: Unpin,
    {
        let request: Request = request.into_client_request()?;

        let domain = domain(&request)?;

        // Make sure we check domain and mode first. URL must be valid.
        let mode = uri_mode(request.uri())?;

        let stream = wrap_stream(stream, domain, connector, mode).await?;
        client_async_with_config(request, stream, config).await
    }
}

#[cfg(not(any(feature = "async-tls", feature = "async-native-tls")))]
pub use self::dummy_tls::client_async_tls_with_connector_and_config;
#[cfg(not(any(feature = "async-tls", feature = "async-native-tls")))]
use self::dummy_tls::AutoStream;

#[cfg(all(feature = "async-tls", not(feature = "async-native-tls")))]
pub use crate::async_tls::client_async_tls_with_connector_and_config;
#[cfg(all(feature = "async-tls", not(feature = "async-native-tls")))]
use crate::async_tls::AutoStream;
#[cfg(all(feature = "async-tls", not(feature = "async-native-tls")))]
type Connector = real_async_tls::TlsConnector;

#[cfg(feature = "async-native-tls")]
pub use self::async_native_tls::client_async_tls_with_connector_and_config;
#[cfg(feature = "async-native-tls")]
use self::async_native_tls::{AutoStream, Connector};

/// Type alias for the stream type of the `client_async()` functions.
pub type ClientStream<S> = AutoStream<S>;

#[cfg(feature = "async-native-tls")]
/// Creates a WebSocket handshake from a request and a stream,
/// upgrading the stream to TLS if required.
pub async fn client_async_tls<R, S>(
    request: R,
    stream: S,
) -> Result<(WebSocketStream<ClientStream<S>>, Response), Error>
where
    R: IntoClientRequest + Unpin,
    S: 'static + AsyncRead + AsyncWrite + Unpin,
    AutoStream<S>: Unpin,
{
    client_async_tls_with_connector_and_config(request, stream, None, None).await
}

#[cfg(feature = "async-native-tls")]
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
    S: 'static + AsyncRead + AsyncWrite + Unpin,
    AutoStream<S>: Unpin,
{
    client_async_tls_with_connector_and_config(request, stream, None, config).await
}

#[cfg(feature = "async-native-tls")]
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
    S: 'static + AsyncRead + AsyncWrite + Unpin,
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
/// use async_tungstenite::async_std::connect_async;
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

#[cfg(any(feature = "async-tls", feature = "async-native-tls"))]
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

#[cfg(any(feature = "async-tls", feature = "async-native-tls"))]
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
