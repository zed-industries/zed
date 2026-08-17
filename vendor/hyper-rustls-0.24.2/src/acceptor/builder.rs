use std::sync::Arc;

use hyper::server::conn::AddrIncoming;
use rustls::ServerConfig;

use super::TlsAcceptor;
/// Builder for [`TlsAcceptor`]
pub struct AcceptorBuilder<State>(State);

/// State of a builder that needs a TLS client config next
pub struct WantsTlsConfig(());

impl AcceptorBuilder<WantsTlsConfig> {
    /// Creates a new [`AcceptorBuilder`]
    pub fn new() -> Self {
        Self(WantsTlsConfig(()))
    }

    /// Passes a rustls [`ServerConfig`] to configure the TLS connection
    pub fn with_tls_config(self, config: ServerConfig) -> AcceptorBuilder<WantsAlpn> {
        AcceptorBuilder(WantsAlpn(config))
    }

    /// Use rustls [defaults][with_safe_defaults] without [client authentication][with_no_client_auth]
    ///
    /// [with_safe_defaults]: rustls::ConfigBuilder::with_safe_defaults
    /// [with_no_client_auth]: rustls::ConfigBuilder::with_no_client_auth
    pub fn with_single_cert(
        self,
        cert_chain: Vec<rustls::Certificate>,
        key_der: rustls::PrivateKey,
    ) -> Result<AcceptorBuilder<WantsAlpn>, rustls::Error> {
        Ok(AcceptorBuilder(WantsAlpn(
            ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(cert_chain, key_der)?,
        )))
    }
}

impl Default for AcceptorBuilder<WantsTlsConfig> {
    fn default() -> Self {
        Self::new()
    }
}

/// State of a builder that needs a incoming address next
pub struct WantsAlpn(ServerConfig);

impl AcceptorBuilder<WantsAlpn> {
    /// Configure ALPN accept protocols in order
    pub fn with_alpn_protocols(
        mut self,
        alpn_protocols: Vec<Vec<u8>>,
    ) -> AcceptorBuilder<WantsIncoming> {
        self.0 .0.alpn_protocols = alpn_protocols;
        AcceptorBuilder(WantsIncoming(self.0 .0))
    }

    /// Configure ALPN to accept HTTP/2
    pub fn with_http2_alpn(mut self) -> AcceptorBuilder<WantsIncoming> {
        self.0 .0.alpn_protocols = vec![b"h2".to_vec()];
        AcceptorBuilder(WantsIncoming(self.0 .0))
    }

    /// Configure ALPN to accept HTTP/1.0
    pub fn with_http10_alpn(mut self) -> AcceptorBuilder<WantsIncoming> {
        self.0 .0.alpn_protocols = vec![b"http/1.0".to_vec()];
        AcceptorBuilder(WantsIncoming(self.0 .0))
    }

    /// Configure ALPN to accept HTTP/1.1
    pub fn with_http11_alpn(mut self) -> AcceptorBuilder<WantsIncoming> {
        self.0 .0.alpn_protocols = vec![b"http/1.1".to_vec()];
        AcceptorBuilder(WantsIncoming(self.0 .0))
    }

    /// Configure ALPN to accept HTTP/2, HTTP/1.1, HTTP/1.0 in that order.
    pub fn with_all_versions_alpn(mut self) -> AcceptorBuilder<WantsIncoming> {
        self.0 .0.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];
        AcceptorBuilder(WantsIncoming(self.0 .0))
    }
}

/// State of a builder that needs a incoming address next
pub struct WantsIncoming(ServerConfig);

impl AcceptorBuilder<WantsIncoming> {
    /// Passes a [`AddrIncoming`] to configure the TLS connection and
    /// creates the [`TlsAcceptor`]
    pub fn with_incoming(self, incoming: impl Into<AddrIncoming>) -> TlsAcceptor {
        self.with_acceptor(incoming.into())
    }

    /// Passes an acceptor implementing [`Accept`] to configure the TLS connection and
    /// creates the [`TlsAcceptor`]
    ///
    /// [`Accept`]: hyper::server::accept::Accept
    pub fn with_acceptor<A>(self, acceptor: A) -> TlsAcceptor<A> {
        TlsAcceptor {
            config: Arc::new(self.0 .0),
            acceptor,
        }
    }
}
