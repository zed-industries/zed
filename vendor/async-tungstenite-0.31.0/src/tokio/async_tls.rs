use real_async_tls::client::TlsStream;
use real_async_tls::TlsConnector;

use tungstenite::client::IntoClientRequest;
use tungstenite::Error;

use crate::stream::Stream as StreamSwitcher;
use crate::{Response, WebSocketConfig, WebSocketStream};

use super::TokioAdapter;

pub type MaybeTlsStream<S> = StreamSwitcher<S, TlsStream<S>>;

pub type AutoStream<S> = MaybeTlsStream<TokioAdapter<S>>;

pub type Connector = TlsConnector;

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
    S: 'static + tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    AutoStream<S>: Unpin,
{
    crate::async_tls::client_async_tls_with_connector_and_config(
        request,
        TokioAdapter::new(stream),
        connector,
        config,
    )
    .await
}
