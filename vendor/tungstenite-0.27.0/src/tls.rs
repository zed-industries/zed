//! Connection helper.
use std::io::{Read, Write};

use crate::{
    client::{client_with_config, uri_mode, IntoClientRequest},
    error::UrlError,
    handshake::client::Response,
    protocol::WebSocketConfig,
    stream::MaybeTlsStream,
    ClientHandshake, Error, HandshakeError, Result, WebSocket,
};

/// A connector that can be used when establishing connections, allowing to control whether
/// `native-tls` or `rustls` is used to create a TLS connection. Or TLS can be disabled with the
/// `Plain` variant.
#[non_exhaustive]
#[allow(missing_debug_implementations)]
pub enum Connector {
    /// Plain (non-TLS) connector.
    Plain,
    /// `native-tls` TLS connector.
    #[cfg(feature = "native-tls")]
    NativeTls(native_tls_crate::TlsConnector),
    /// `rustls` TLS connector.
    #[cfg(feature = "__rustls-tls")]
    Rustls(std::sync::Arc<rustls::ClientConfig>),
}

mod encryption {
    #[cfg(feature = "native-tls")]
    pub mod native_tls {
        use native_tls_crate::{HandshakeError as TlsHandshakeError, TlsConnector};

        use std::io::{Read, Write};

        use crate::{
            error::TlsError,
            stream::{MaybeTlsStream, Mode},
            Error, Result,
        };

        pub fn wrap_stream<S>(
            socket: S,
            domain: &str,
            mode: Mode,
            tls_connector: Option<TlsConnector>,
        ) -> Result<MaybeTlsStream<S>>
        where
            S: Read + Write,
        {
            match mode {
                Mode::Plain => Ok(MaybeTlsStream::Plain(socket)),
                Mode::Tls => {
                    let try_connector = tls_connector.map_or_else(TlsConnector::new, Ok);
                    let connector = try_connector.map_err(TlsError::Native)?;
                    let connected = connector.connect(domain, socket);
                    match connected {
                        Err(e) => match e {
                            TlsHandshakeError::Failure(f) => Err(Error::Tls(f.into())),
                            TlsHandshakeError::WouldBlock(_) => {
                                panic!("Bug: TLS handshake not blocked")
                            }
                        },
                        Ok(s) => Ok(MaybeTlsStream::NativeTls(s)),
                    }
                }
            }
        }
    }

    #[cfg(feature = "__rustls-tls")]
    pub mod rustls {
        use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
        use rustls_pki_types::ServerName;

        use std::{
            io::{Read, Write},
            sync::Arc,
        };

        use crate::{
            error::TlsError,
            stream::{MaybeTlsStream, Mode},
            Result,
        };

        pub fn wrap_stream<S>(
            socket: S,
            domain: &str,
            mode: Mode,
            tls_connector: Option<Arc<ClientConfig>>,
        ) -> Result<MaybeTlsStream<S>>
        where
            S: Read + Write,
        {
            match mode {
                Mode::Plain => Ok(MaybeTlsStream::Plain(socket)),
                Mode::Tls => {
                    let config = match tls_connector {
                        Some(config) => config,
                        None => {
                            #[allow(unused_mut)]
                            let mut root_store = RootCertStore::empty();

                            #[cfg(feature = "rustls-tls-native-roots")]
                            {
                                let rustls_native_certs::CertificateResult {
                                    certs, errors, ..
                                } = rustls_native_certs::load_native_certs();

                                if !errors.is_empty() {
                                    log::warn!(
                                        "native root CA certificate loading errors: {errors:?}"
                                    );
                                }

                                // Not finding any native root CA certificates is not fatal if the
                                // "rustls-tls-webpki-roots" feature is enabled.
                                #[cfg(not(feature = "rustls-tls-webpki-roots"))]
                                if certs.is_empty() {
                                    return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("no native root CA certificates found (errors: {errors:?})")).into());
                                }

                                let total_number = certs.len();
                                let (number_added, number_ignored) =
                                    root_store.add_parsable_certificates(certs);
                                log::debug!("Added {number_added}/{total_number} native root certificates (ignored {number_ignored})");
                            }
                            #[cfg(feature = "rustls-tls-webpki-roots")]
                            {
                                root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
                            }

                            Arc::new(
                                ClientConfig::builder()
                                    .with_root_certificates(root_store)
                                    .with_no_client_auth(),
                            )
                        }
                    };
                    let domain = ServerName::try_from(domain)
                        .map_err(|_| TlsError::InvalidDnsName)?
                        .to_owned();
                    let client = ClientConnection::new(config, domain).map_err(TlsError::Rustls)?;
                    let stream = StreamOwned::new(client, socket);

                    Ok(MaybeTlsStream::Rustls(stream))
                }
            }
        }
    }

    pub mod plain {
        use std::io::{Read, Write};

        use crate::{
            error::UrlError,
            stream::{MaybeTlsStream, Mode},
            Error, Result,
        };

        pub fn wrap_stream<S>(socket: S, mode: Mode) -> Result<MaybeTlsStream<S>>
        where
            S: Read + Write,
        {
            match mode {
                Mode::Plain => Ok(MaybeTlsStream::Plain(socket)),
                Mode::Tls => Err(Error::Url(UrlError::TlsFeatureNotEnabled)),
            }
        }
    }
}

type TlsHandshakeError<S> = HandshakeError<ClientHandshake<MaybeTlsStream<S>>>;

/// Creates a WebSocket handshake from a request and a stream,
/// upgrading the stream to TLS if required.
pub fn client_tls<R, S>(
    request: R,
    stream: S,
) -> Result<(WebSocket<MaybeTlsStream<S>>, Response), TlsHandshakeError<S>>
where
    R: IntoClientRequest,
    S: Read + Write,
{
    client_tls_with_config(request, stream, None, None)
}

/// The same as [`client_tls()`] but one can specify a websocket configuration,
/// and an optional connector. If no connector is specified, a default one will
/// be created.
///
/// Please refer to [`client_tls()`] for more details.
pub fn client_tls_with_config<R, S>(
    request: R,
    stream: S,
    config: Option<WebSocketConfig>,
    connector: Option<Connector>,
) -> Result<(WebSocket<MaybeTlsStream<S>>, Response), TlsHandshakeError<S>>
where
    R: IntoClientRequest,
    S: Read + Write,
{
    let request = request.into_client_request()?;

    #[cfg(any(feature = "native-tls", feature = "__rustls-tls"))]
    let domain = match request.uri().host() {
        Some(d) => Ok(d.to_string()),
        None => Err(Error::Url(UrlError::NoHostName)),
    }?;

    let mode = uri_mode(request.uri())?;

    let stream = match connector {
        Some(conn) => match conn {
            #[cfg(feature = "native-tls")]
            Connector::NativeTls(conn) => {
                self::encryption::native_tls::wrap_stream(stream, &domain, mode, Some(conn))
            }
            #[cfg(feature = "__rustls-tls")]
            Connector::Rustls(conn) => {
                self::encryption::rustls::wrap_stream(stream, &domain, mode, Some(conn))
            }
            Connector::Plain => self::encryption::plain::wrap_stream(stream, mode),
        },
        None => {
            #[cfg(feature = "native-tls")]
            {
                self::encryption::native_tls::wrap_stream(stream, &domain, mode, None)
            }
            #[cfg(all(feature = "__rustls-tls", not(feature = "native-tls")))]
            {
                self::encryption::rustls::wrap_stream(stream, &domain, mode, None)
            }
            #[cfg(not(any(feature = "native-tls", feature = "__rustls-tls")))]
            {
                self::encryption::plain::wrap_stream(stream, mode)
            }
        }
    }?;

    client_with_config(request, stream, config)
}
