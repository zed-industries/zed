use real_tokio_native_tls::TlsConnector as AsyncTlsConnector;
use real_tokio_native_tls::TlsStream;

use tungstenite::client::{uri_mode, IntoClientRequest};
use tungstenite::handshake::client::Request;
use tungstenite::stream::Mode;
use tungstenite::Error;

use crate::stream::Stream as StreamSwitcher;
use crate::{client_async_with_config, domain, Response, WebSocketConfig, WebSocketStream};

use super::TokioAdapter;

/// A stream that might be protected with TLS.
pub type MaybeTlsStream<S> = StreamSwitcher<TokioAdapter<S>, TokioAdapter<TlsStream<S>>>;

pub type AutoStream<S> = MaybeTlsStream<S>;

pub type Connector = AsyncTlsConnector;

async fn wrap_stream<S>(
    socket: S,
    domain: String,
    connector: Option<Connector>,
    mode: Mode,
) -> Result<AutoStream<S>, Error>
where
    S: 'static + tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    match mode {
        Mode::Plain => Ok(StreamSwitcher::Plain(TokioAdapter::new(socket))),
        Mode::Tls => {
            let stream = {
                let connector = if let Some(connector) = connector {
                    connector
                } else {
                    let connector = real_native_tls::TlsConnector::builder()
                        .build()
                        .map_err(|err| Error::Tls(err.into()))?;
                    AsyncTlsConnector::from(connector)
                };
                connector
                    .connect(&domain, socket)
                    .await
                    .map_err(|err| Error::Tls(err.into()))?
            };
            Ok(StreamSwitcher::Tls(TokioAdapter::new(stream)))
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
    S: 'static + tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
    AutoStream<S>: Unpin,
{
    let request: Request = request.into_client_request()?;

    let domain = domain(&request)?;

    // Make sure we check domain and mode first. URL must be valid.
    let mode = uri_mode(request.uri())?;

    let stream = wrap_stream(stream, domain, connector, mode).await?;
    client_async_with_config(request, stream, config).await
}
