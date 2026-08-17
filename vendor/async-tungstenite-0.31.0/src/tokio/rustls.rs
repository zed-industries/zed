use real_tokio_rustls::rustls::{ClientConfig, RootCertStore};
use real_tokio_rustls::{client::TlsStream, TlsConnector};
use rustls_pki_types::ServerName;

use tungstenite::client::{uri_mode, IntoClientRequest};
use tungstenite::error::TlsError;
use tungstenite::handshake::client::Request;
use tungstenite::stream::Mode;
use tungstenite::Error;

use std::convert::TryFrom;

use crate::stream::Stream as StreamSwitcher;
use crate::{client_async_with_config, domain, Response, WebSocketConfig, WebSocketStream};

use super::TokioAdapter;

/// A stream that might be protected with TLS.
pub type MaybeTlsStream<S> = StreamSwitcher<TokioAdapter<S>, TokioAdapter<TlsStream<S>>>;

pub type AutoStream<S> = MaybeTlsStream<S>;

pub type Connector = TlsConnector;

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
                    #[cfg(feature = "tokio-rustls-manual-roots")]
                    log::error!("tokio-rustls-manual-roots was selected, but no connector was provided! No certificates can be verified in this state.");

                    #[cfg(feature = "tokio-rustls-manual-roots")]
                    let root_store = RootCertStore::empty();
                    #[cfg(not(feature = "tokio-rustls-manual-roots"))]
                    let mut root_store = RootCertStore::empty();

                    #[cfg(feature = "tokio-rustls-native-certs")]
                    {
                        let mut native_certs = rustls_native_certs::load_native_certs();
                        if let Some(err) = native_certs.errors.drain(..).next() {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, err).into());
                        }
                        let native_certs = native_certs.certs;
                        let total_number = native_certs.len();
                        let (number_added, number_ignored) =
                            root_store.add_parsable_certificates(native_certs);
                        log::debug!("Added {number_added}/{total_number} native root certificates (ignored {number_ignored})");
                    }
                    #[cfg(all(
                        feature = "tokio-rustls-webpki-roots",
                        not(feature = "tokio-rustls-native-certs"),
                        not(feature = "tokio-rustls-manual-roots")
                    ))]
                    {
                        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
                    }
                    TlsConnector::from(std::sync::Arc::new(
                        ClientConfig::builder()
                            .with_root_certificates(root_store)
                            .with_no_client_auth(),
                    ))
                };
                let domain = ServerName::try_from(domain)
                    .map_err(|_| Error::Tls(TlsError::InvalidDnsName))?;
                connector.connect(domain, socket).await?
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
