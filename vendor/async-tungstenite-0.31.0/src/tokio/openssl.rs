use openssl::ssl::{ConnectConfiguration, SslConnector, SslMethod};
use real_tokio_openssl::SslStream as TlsStream;

use tungstenite::client::{uri_mode, IntoClientRequest};
use tungstenite::handshake::client::Request;
use tungstenite::stream::Mode;
use tungstenite::Error;

use crate::stream::Stream as StreamSwitcher;
use crate::{client_async_with_config, domain, Response, WebSocketConfig, WebSocketStream};

use super::TokioAdapter;

/// A stream that might be protected with TLS.
pub type MaybeTlsStream<S> =
    StreamSwitcher<TokioAdapter<S>, TokioAdapter<std::pin::Pin<Box<TlsStream<S>>>>>;

pub type AutoStream<S> = MaybeTlsStream<S>;

pub type Connector = ConnectConfiguration;

async fn wrap_stream<S>(
    socket: S,
    domain: String,
    connector: Option<Connector>,
    mode: Mode,
) -> Result<AutoStream<S>, Error>
where
    S: 'static
        + tokio::io::AsyncRead
        + tokio::io::AsyncWrite
        + Unpin
        + std::fmt::Debug
        + Send
        + Sync,
{
    match mode {
        Mode::Plain => Ok(StreamSwitcher::Plain(TokioAdapter::new(socket))),
        Mode::Tls => {
            let stream = {
                let connector = if let Some(connector) = connector {
                    connector
                } else {
                    SslConnector::builder(SslMethod::tls())
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                        .build()
                        .configure()
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                };

                let ssl = connector
                    .into_ssl(&domain)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

                let mut stream = Box::pin(
                    TlsStream::new(ssl, socket)
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?,
                );

                stream
                    .as_mut()
                    .connect()
                    .await
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

                stream
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
    connector: Option<Connector>,
    config: Option<WebSocketConfig>,
) -> Result<(WebSocketStream<AutoStream<S>>, Response), Error>
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
    let request: Request = request.into_client_request()?;

    let domain = domain(&request)?;

    // Make sure we check domain and mode first. URL must be valid.
    let mode = uri_mode(request.uri())?;

    let stream = wrap_stream(stream, domain, connector, mode).await?;
    client_async_with_config(request, stream, config).await
}
