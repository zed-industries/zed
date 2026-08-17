use tungstenite::client::{uri_mode, IntoClientRequest};
use tungstenite::handshake::client::Request;
use tungstenite::stream::Mode;
use tungstenite::Error;

use super::TokioAdapter;

use crate::{client_async_with_config, domain, Response, WebSocketConfig, WebSocketStream};

pub type AutoStream<S> = TokioAdapter<S>;

type Connector = ();

async fn wrap_stream<S>(
    socket: S,
    _domain: String,
    _connector: Option<()>,
    mode: Mode,
) -> Result<AutoStream<S>, Error>
where
    S: 'static + tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    match mode {
        Mode::Plain => Ok(TokioAdapter::new(socket)),
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
