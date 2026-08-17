use std::fmt;
use std::fs::File;
use std::future::Future;
use std::io::{self, BufReader, Cursor, Read};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use futures_util::ready;
use hyper::server::accept::Accept;
use hyper::server::conn::{AddrIncoming, AddrStream};
use tokio_rustls::rustls::server::WebPkiClientVerifier;
use tokio_rustls::rustls::{Error as TlsError, RootCertStore, ServerConfig};

use crate::transport::Transport;

/// Represents errors that can occur building the TlsConfig
#[derive(Debug)]
pub(crate) enum TlsConfigError {
    Io(io::Error),
    /// An Error parsing the Certificate
    CertParseError,
    /// Identity PEM is invalid
    InvalidIdentityPem,
    /// Identity PEM is missing a private key such as RSA, ECC or PKCS8
    MissingPrivateKey,
    /// Unknown private key format
    UnknownPrivateKeyFormat,
    /// An error from an empty key
    EmptyKey,
    /// An error from an invalid key
    InvalidKey(TlsError),
}

impl fmt::Display for TlsConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TlsConfigError::Io(err) => err.fmt(f),
            TlsConfigError::CertParseError => write!(f, "certificate parse error"),
            TlsConfigError::UnknownPrivateKeyFormat => write!(f, "unknown private key format"),
            TlsConfigError::MissingPrivateKey => write!(
                f,
                "Identity PEM is missing a private key such as RSA, ECC or PKCS8"
            ),
            TlsConfigError::InvalidIdentityPem => write!(f, "identity PEM is invalid"),
            TlsConfigError::EmptyKey => write!(f, "key contains no private key"),
            TlsConfigError::InvalidKey(err) => write!(f, "key contains an invalid key, {}", err),
        }
    }
}

impl std::error::Error for TlsConfigError {}

/// Tls client authentication configuration.
pub(crate) enum TlsClientAuth {
    /// No client auth.
    Off,
    /// Allow any anonymous or authenticated client.
    Optional(Box<dyn Read + Send + Sync>),
    /// Allow any authenticated client.
    Required(Box<dyn Read + Send + Sync>),
}

/// Builder to set the configuration for the Tls server.
pub(crate) struct TlsConfigBuilder {
    cert: Box<dyn Read + Send + Sync>,
    key: Box<dyn Read + Send + Sync>,
    client_auth: TlsClientAuth,
    ocsp_resp: Vec<u8>,
}

impl fmt::Debug for TlsConfigBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TlsConfigBuilder").finish()
    }
}

impl TlsConfigBuilder {
    /// Create a new TlsConfigBuilder
    pub(crate) fn new() -> TlsConfigBuilder {
        TlsConfigBuilder {
            key: Box::new(io::empty()),
            cert: Box::new(io::empty()),
            client_auth: TlsClientAuth::Off,
            ocsp_resp: Vec::new(),
        }
    }

    /// sets the Tls key via File Path, returns `TlsConfigError::IoError` if the file cannot be open
    pub(crate) fn key_path(mut self, path: impl AsRef<Path>) -> Self {
        self.key = Box::new(LazyFile {
            path: path.as_ref().into(),
            file: None,
        });
        self
    }

    /// sets the Tls key via bytes slice
    pub(crate) fn key(mut self, key: &[u8]) -> Self {
        self.key = Box::new(Cursor::new(Vec::from(key)));
        self
    }

    /// Specify the file path for the TLS certificate to use.
    pub(crate) fn cert_path(mut self, path: impl AsRef<Path>) -> Self {
        self.cert = Box::new(LazyFile {
            path: path.as_ref().into(),
            file: None,
        });
        self
    }

    /// sets the Tls certificate via bytes slice
    pub(crate) fn cert(mut self, cert: &[u8]) -> Self {
        self.cert = Box::new(Cursor::new(Vec::from(cert)));
        self
    }

    /// Sets the trust anchor for optional Tls client authentication via file path.
    ///
    /// Anonymous and authenticated clients will be accepted. If no trust anchor is provided by any
    /// of the `client_auth_` methods, then client authentication is disabled by default.
    pub(crate) fn client_auth_optional_path(mut self, path: impl AsRef<Path>) -> Self {
        let file = Box::new(LazyFile {
            path: path.as_ref().into(),
            file: None,
        });
        self.client_auth = TlsClientAuth::Optional(file);
        self
    }

    /// Sets the trust anchor for optional Tls client authentication via bytes slice.
    ///
    /// Anonymous and authenticated clients will be accepted. If no trust anchor is provided by any
    /// of the `client_auth_` methods, then client authentication is disabled by default.
    pub(crate) fn client_auth_optional(mut self, trust_anchor: &[u8]) -> Self {
        let cursor = Box::new(Cursor::new(Vec::from(trust_anchor)));
        self.client_auth = TlsClientAuth::Optional(cursor);
        self
    }

    /// Sets the trust anchor for required Tls client authentication via file path.
    ///
    /// Only authenticated clients will be accepted. If no trust anchor is provided by any of the
    /// `client_auth_` methods, then client authentication is disabled by default.
    pub(crate) fn client_auth_required_path(mut self, path: impl AsRef<Path>) -> Self {
        let file = Box::new(LazyFile {
            path: path.as_ref().into(),
            file: None,
        });
        self.client_auth = TlsClientAuth::Required(file);
        self
    }

    /// Sets the trust anchor for required Tls client authentication via bytes slice.
    ///
    /// Only authenticated clients will be accepted. If no trust anchor is provided by any of the
    /// `client_auth_` methods, then client authentication is disabled by default.
    pub(crate) fn client_auth_required(mut self, trust_anchor: &[u8]) -> Self {
        let cursor = Box::new(Cursor::new(Vec::from(trust_anchor)));
        self.client_auth = TlsClientAuth::Required(cursor);
        self
    }

    /// sets the DER-encoded OCSP response
    pub(crate) fn ocsp_resp(mut self, ocsp_resp: &[u8]) -> Self {
        self.ocsp_resp = Vec::from(ocsp_resp);
        self
    }

    pub(crate) fn build(mut self) -> Result<ServerConfig, TlsConfigError> {
        let mut cert_rdr = BufReader::new(self.cert);
        let cert = rustls_pemfile::certs(&mut cert_rdr)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_e| TlsConfigError::CertParseError)?;

        let mut key_vec = Vec::new();
        self.key
            .read_to_end(&mut key_vec)
            .map_err(TlsConfigError::Io)?;

        if key_vec.is_empty() {
            return Err(TlsConfigError::EmptyKey);
        }

        let mut key_opt = None;
        let mut key_cur = std::io::Cursor::new(key_vec);
        for item in rustls_pemfile::read_all(&mut key_cur)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_e| TlsConfigError::InvalidIdentityPem)?
        {
            match item {
                rustls_pemfile::Item::Pkcs1Key(k) => key_opt = Some(k.into()),
                rustls_pemfile::Item::Pkcs8Key(k) => key_opt = Some(k.into()),
                rustls_pemfile::Item::Sec1Key(k) => key_opt = Some(k.into()),
                _ => return Err(TlsConfigError::UnknownPrivateKeyFormat),
            }
        }
        let key = match key_opt {
            Some(v) => v,
            _ => return Err(TlsConfigError::MissingPrivateKey),
        };

        fn read_trust_anchor(
            trust_anchor: Box<dyn Read + Send + Sync>,
        ) -> Result<RootCertStore, TlsConfigError> {
            let trust_anchors = {
                let mut reader = BufReader::new(trust_anchor);
                rustls_pemfile::certs(&mut reader)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(TlsConfigError::Io)?
            };

            let mut store = RootCertStore::empty();
            let (added, _skipped) = store.add_parsable_certificates(trust_anchors);
            if added == 0 {
                return Err(TlsConfigError::CertParseError);
            }

            Ok(store)
        }

        let config = {
            let builder = ServerConfig::builder();
            let mut config = match self.client_auth {
                TlsClientAuth::Off => builder.with_no_client_auth(),
                TlsClientAuth::Optional(trust_anchor) => {
                    let verifier =
                        WebPkiClientVerifier::builder(read_trust_anchor(trust_anchor)?.into())
                            .allow_unauthenticated()
                            .build()
                            .map_err(|_| TlsConfigError::CertParseError)?;
                    builder.with_client_cert_verifier(verifier)
                }
                TlsClientAuth::Required(trust_anchor) => {
                    let verifier =
                        WebPkiClientVerifier::builder(read_trust_anchor(trust_anchor)?.into())
                            .build()
                            .map_err(|_| TlsConfigError::CertParseError)?;
                    builder.with_client_cert_verifier(verifier)
                }
            }
            .with_single_cert_with_ocsp(cert, key, self.ocsp_resp)
            .map_err(TlsConfigError::InvalidKey)?;
            config.alpn_protocols = vec!["h2".into(), "http/1.1".into()];
            config
        };

        Ok(config)
    }
}

struct LazyFile {
    path: PathBuf,
    file: Option<File>,
}

impl LazyFile {
    fn lazy_read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.file.is_none() {
            self.file = Some(File::open(&self.path)?);
        }

        self.file.as_mut().unwrap().read(buf)
    }
}

impl Read for LazyFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.lazy_read(buf).map_err(|err| {
            let kind = err.kind();
            io::Error::new(
                kind,
                format!("error reading file ({:?}): {}", self.path.display(), err),
            )
        })
    }
}

impl Transport for TlsStream {
    fn remote_addr(&self) -> Option<SocketAddr> {
        Some(self.remote_addr)
    }
}

enum State {
    Handshaking(tokio_rustls::Accept<AddrStream>),
    Streaming(tokio_rustls::server::TlsStream<AddrStream>),
}

// tokio_rustls::server::TlsStream doesn't expose constructor methods,
// so we have to TlsAcceptor::accept and handshake to have access to it
// TlsStream implements AsyncRead/AsyncWrite handshaking tokio_rustls::Accept first
pub(crate) struct TlsStream {
    state: State,
    remote_addr: SocketAddr,
}

impl TlsStream {
    fn new(stream: AddrStream, config: Arc<ServerConfig>) -> TlsStream {
        let remote_addr = stream.remote_addr();
        let accept = tokio_rustls::TlsAcceptor::from(config).accept(stream);
        TlsStream {
            state: State::Handshaking(accept),
            remote_addr,
        }
    }
}

impl AsyncRead for TlsStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let pin = self.get_mut();
        match pin.state {
            State::Handshaking(ref mut accept) => match ready!(Pin::new(accept).poll(cx)) {
                Ok(mut stream) => {
                    let result = Pin::new(&mut stream).poll_read(cx, buf);
                    pin.state = State::Streaming(stream);
                    result
                }
                Err(err) => Poll::Ready(Err(err)),
            },
            State::Streaming(ref mut stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for TlsStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let pin = self.get_mut();
        match pin.state {
            State::Handshaking(ref mut accept) => match ready!(Pin::new(accept).poll(cx)) {
                Ok(mut stream) => {
                    let result = Pin::new(&mut stream).poll_write(cx, buf);
                    pin.state = State::Streaming(stream);
                    result
                }
                Err(err) => Poll::Ready(Err(err)),
            },
            State::Streaming(ref mut stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.state {
            State::Handshaking(_) => Poll::Ready(Ok(())),
            State::Streaming(ref mut stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.state {
            State::Handshaking(_) => Poll::Ready(Ok(())),
            State::Streaming(ref mut stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

pub(crate) struct TlsAcceptor {
    config: Arc<ServerConfig>,
    incoming: AddrIncoming,
}

impl TlsAcceptor {
    pub(crate) fn new(config: ServerConfig, incoming: AddrIncoming) -> TlsAcceptor {
        TlsAcceptor {
            config: Arc::new(config),
            incoming,
        }
    }
}

impl Accept for TlsAcceptor {
    type Conn = TlsStream;
    type Error = io::Error;

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        let pin = self.get_mut();
        match ready!(Pin::new(&mut pin.incoming).poll_accept(cx)) {
            Some(Ok(sock)) => Poll::Ready(Some(Ok(TlsStream::new(sock, pin.config.clone())))),
            Some(Err(e)) => Poll::Ready(Some(Err(e))),
            None => Poll::Ready(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_cert_key() {
        TlsConfigBuilder::new()
            .key_path("examples/tls/key.rsa")
            .cert_path("examples/tls/cert.pem")
            .build()
            .unwrap();
    }

    #[test]
    fn bytes_cert_key() {
        let key = include_str!("../examples/tls/key.rsa");
        let cert = include_str!("../examples/tls/cert.pem");

        TlsConfigBuilder::new()
            .key(key.as_bytes())
            .cert(cert.as_bytes())
            .build()
            .unwrap();
    }

    #[test]
    fn file_ecc_cert_key() {
        TlsConfigBuilder::new()
            .key_path("examples/tls/key.ecc")
            .cert_path("examples/tls/cert.ecc.pem")
            .build()
            .unwrap();
    }

    #[test]
    fn bytes_ecc_cert_key() {
        let key = include_str!("../examples/tls/key.ecc");
        let cert = include_str!("../examples/tls/cert.ecc.pem");

        TlsConfigBuilder::new()
            .key(key.as_bytes())
            .cert(cert.as_bytes())
            .build()
            .unwrap();
    }
}
