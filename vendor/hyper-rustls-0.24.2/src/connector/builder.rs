use rustls::ClientConfig;

use super::HttpsConnector;
#[cfg(any(feature = "rustls-native-certs", feature = "webpki-roots"))]
use crate::config::ConfigBuilderExt;

#[cfg(feature = "tokio-runtime")]
use hyper::client::HttpConnector;

/// A builder for an [`HttpsConnector`]
///
/// This makes configuration flexible and explicit and ensures connector
/// features match crate features
///
/// # Examples
///
/// ```
/// use hyper_rustls::HttpsConnectorBuilder;
///
/// # #[cfg(all(feature = "webpki-roots", feature = "tokio-runtime", feature = "http1"))]
/// let https = HttpsConnectorBuilder::new()
///     .with_webpki_roots()
///     .https_only()
///     .enable_http1()
///     .build();
/// ```
pub struct ConnectorBuilder<State>(State);

/// State of a builder that needs a TLS client config next
pub struct WantsTlsConfig(());

impl ConnectorBuilder<WantsTlsConfig> {
    /// Creates a new [`ConnectorBuilder`]
    pub fn new() -> Self {
        Self(WantsTlsConfig(()))
    }

    /// Passes a rustls [`ClientConfig`] to configure the TLS connection
    ///
    /// The [`alpn_protocols`](ClientConfig::alpn_protocols) field is
    /// required to be empty (or the function will panic) and will be
    /// rewritten to match the enabled schemes (see
    /// [`enable_http1`](ConnectorBuilder::enable_http1),
    /// [`enable_http2`](ConnectorBuilder::enable_http2)) before the
    /// connector is built.
    pub fn with_tls_config(self, config: ClientConfig) -> ConnectorBuilder<WantsSchemes> {
        assert!(
            config.alpn_protocols.is_empty(),
            "ALPN protocols should not be pre-defined"
        );
        ConnectorBuilder(WantsSchemes { tls_config: config })
    }

    /// Shorthand for using rustls' [safe defaults][with_safe_defaults]
    /// and native roots
    ///
    /// See [`ConfigBuilderExt::with_native_roots`]
    ///
    /// [with_safe_defaults]: rustls::ConfigBuilder::with_safe_defaults
    #[cfg(feature = "rustls-native-certs")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rustls-native-certs")))]
    pub fn with_native_roots(self) -> ConnectorBuilder<WantsSchemes> {
        self.with_tls_config(
            ClientConfig::builder()
                .with_safe_defaults()
                .with_native_roots()
                .with_no_client_auth(),
        )
    }

    /// Shorthand for using rustls' [safe defaults][with_safe_defaults]
    /// and Mozilla roots
    ///
    /// See [`ConfigBuilderExt::with_webpki_roots`]
    ///
    /// [with_safe_defaults]: rustls::ConfigBuilder::with_safe_defaults
    #[cfg(feature = "webpki-roots")]
    #[cfg_attr(docsrs, doc(cfg(feature = "webpki-roots")))]
    pub fn with_webpki_roots(self) -> ConnectorBuilder<WantsSchemes> {
        self.with_tls_config(
            ClientConfig::builder()
                .with_safe_defaults()
                .with_webpki_roots()
                .with_no_client_auth(),
        )
    }
}

impl Default for ConnectorBuilder<WantsTlsConfig> {
    fn default() -> Self {
        Self::new()
    }
}

/// State of a builder that needs schemes (https:// and http://) to be
/// configured next
pub struct WantsSchemes {
    tls_config: ClientConfig,
}

impl ConnectorBuilder<WantsSchemes> {
    /// Enforce the use of HTTPS when connecting
    ///
    /// Only URLs using the HTTPS scheme will be connectable.
    pub fn https_only(self) -> ConnectorBuilder<WantsProtocols1> {
        ConnectorBuilder(WantsProtocols1 {
            tls_config: self.0.tls_config,
            https_only: true,
            override_server_name: None,
        })
    }

    /// Allow both HTTPS and HTTP when connecting
    ///
    /// HTTPS URLs will be handled through rustls,
    /// HTTP URLs will be handled by the lower-level connector.
    pub fn https_or_http(self) -> ConnectorBuilder<WantsProtocols1> {
        ConnectorBuilder(WantsProtocols1 {
            tls_config: self.0.tls_config,
            https_only: false,
            override_server_name: None,
        })
    }
}

/// State of a builder that needs to have some protocols (HTTP1 or later)
/// enabled next
///
/// No protocol has been enabled at this point.
pub struct WantsProtocols1 {
    tls_config: ClientConfig,
    https_only: bool,
    override_server_name: Option<String>,
}

impl WantsProtocols1 {
    fn wrap_connector<H>(self, conn: H) -> HttpsConnector<H> {
        HttpsConnector {
            force_https: self.https_only,
            http: conn,
            tls_config: std::sync::Arc::new(self.tls_config),
            override_server_name: self.override_server_name,
        }
    }

    #[cfg(feature = "tokio-runtime")]
    fn build(self) -> HttpsConnector<HttpConnector> {
        let mut http = HttpConnector::new();
        // HttpConnector won't enforce scheme, but HttpsConnector will
        http.enforce_http(false);
        self.wrap_connector(http)
    }
}

impl ConnectorBuilder<WantsProtocols1> {
    /// Enable HTTP1
    ///
    /// This needs to be called explicitly, no protocol is enabled by default
    #[cfg(feature = "http1")]
    pub fn enable_http1(self) -> ConnectorBuilder<WantsProtocols2> {
        ConnectorBuilder(WantsProtocols2 { inner: self.0 })
    }

    /// Enable HTTP2
    ///
    /// This needs to be called explicitly, no protocol is enabled by default
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn enable_http2(mut self) -> ConnectorBuilder<WantsProtocols3> {
        self.0.tls_config.alpn_protocols = vec![b"h2".to_vec()];
        ConnectorBuilder(WantsProtocols3 {
            inner: self.0,
            enable_http1: false,
        })
    }

    /// Enable all HTTP versions built into this library (enabled with Cargo features)
    ///
    /// For now, this could enable both HTTP 1 and 2, depending on active features.
    /// In the future, other supported versions will be enabled as well.
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn enable_all_versions(mut self) -> ConnectorBuilder<WantsProtocols3> {
        #[cfg(feature = "http1")]
        let alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        #[cfg(not(feature = "http1"))]
        let alpn_protocols = vec![b"h2".to_vec()];

        self.0.tls_config.alpn_protocols = alpn_protocols;
        ConnectorBuilder(WantsProtocols3 {
            inner: self.0,
            enable_http1: cfg!(feature = "http1"),
        })
    }

    /// Override server name for the TLS stack
    ///
    /// By default, for each connection hyper-rustls will extract host portion
    /// of the destination URL and verify that server certificate contains
    /// this value.
    ///
    /// If this method is called, hyper-rustls will instead verify that server
    /// certificate contains `override_server_name`. Domain name included in
    /// the URL will not affect certificate validation.
    pub fn with_server_name(mut self, override_server_name: String) -> Self {
        self.0.override_server_name = Some(override_server_name);
        self
    }
}

/// State of a builder with HTTP1 enabled, that may have some other
/// protocols (HTTP2 or later) enabled next
///
/// At this point a connector can be built, see
/// [`build`](ConnectorBuilder<WantsProtocols2>::build) and
/// [`wrap_connector`](ConnectorBuilder<WantsProtocols2>::wrap_connector).
pub struct WantsProtocols2 {
    inner: WantsProtocols1,
}

impl ConnectorBuilder<WantsProtocols2> {
    /// Enable HTTP2
    ///
    /// This needs to be called explicitly, no protocol is enabled by default
    #[cfg(feature = "http2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "http2")))]
    pub fn enable_http2(mut self) -> ConnectorBuilder<WantsProtocols3> {
        self.0.inner.tls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        ConnectorBuilder(WantsProtocols3 {
            inner: self.0.inner,
            enable_http1: true,
        })
    }

    /// This builds an [`HttpsConnector`] built on hyper's default [`HttpConnector`]
    #[cfg(feature = "tokio-runtime")]
    pub fn build(self) -> HttpsConnector<HttpConnector> {
        self.0.inner.build()
    }

    /// This wraps an arbitrary low-level connector into an [`HttpsConnector`]
    pub fn wrap_connector<H>(self, conn: H) -> HttpsConnector<H> {
        // HTTP1-only, alpn_protocols stays empty
        // HttpConnector doesn't have a way to say http1-only;
        // its connection pool may still support HTTP2
        // though it won't be used
        self.0.inner.wrap_connector(conn)
    }
}

/// State of a builder with HTTP2 (and possibly HTTP1) enabled
///
/// At this point a connector can be built, see
/// [`build`](ConnectorBuilder<WantsProtocols3>::build) and
/// [`wrap_connector`](ConnectorBuilder<WantsProtocols3>::wrap_connector).
#[cfg(feature = "http2")]
pub struct WantsProtocols3 {
    inner: WantsProtocols1,
    // ALPN is built piecemeal without the need to read back this field
    #[allow(dead_code)]
    enable_http1: bool,
}

#[cfg(feature = "http2")]
impl ConnectorBuilder<WantsProtocols3> {
    /// This builds an [`HttpsConnector`] built on hyper's default [`HttpConnector`]
    #[cfg(feature = "tokio-runtime")]
    pub fn build(self) -> HttpsConnector<HttpConnector> {
        self.0.inner.build()
    }

    /// This wraps an arbitrary low-level connector into an [`HttpsConnector`]
    pub fn wrap_connector<H>(self, conn: H) -> HttpsConnector<H> {
        // If HTTP1 is disabled, we can set http2_only
        // on the Client (a higher-level object that uses the connector)
        // client.http2_only(!self.0.enable_http1);
        self.0.inner.wrap_connector(conn)
    }
}

#[cfg(test)]
mod tests {
    // Typical usage
    #[test]
    #[cfg(all(feature = "webpki-roots", feature = "http1"))]
    fn test_builder() {
        let _connector = super::ConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http1()
            .build();
    }

    #[test]
    #[cfg(feature = "http1")]
    #[should_panic(expected = "ALPN protocols should not be pre-defined")]
    fn test_reject_predefined_alpn() {
        let roots = rustls::RootCertStore::empty();
        let mut config_with_alpn = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(roots)
            .with_no_client_auth();
        config_with_alpn.alpn_protocols = vec![b"fancyprotocol".to_vec()];
        let _connector = super::ConnectorBuilder::new()
            .with_tls_config(config_with_alpn)
            .https_only()
            .enable_http1()
            .build();
    }

    #[test]
    #[cfg(all(feature = "http1", feature = "http2"))]
    fn test_alpn() {
        let roots = rustls::RootCertStore::empty();
        let tls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let connector = super::ConnectorBuilder::new()
            .with_tls_config(tls_config.clone())
            .https_only()
            .enable_http1()
            .build();
        assert!(connector
            .tls_config
            .alpn_protocols
            .is_empty());
        let connector = super::ConnectorBuilder::new()
            .with_tls_config(tls_config.clone())
            .https_only()
            .enable_http2()
            .build();
        assert_eq!(&connector.tls_config.alpn_protocols, &[b"h2".to_vec()]);
        let connector = super::ConnectorBuilder::new()
            .with_tls_config(tls_config.clone())
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();
        assert_eq!(
            &connector.tls_config.alpn_protocols,
            &[b"h2".to_vec(), b"http/1.1".to_vec()]
        );
        let connector = super::ConnectorBuilder::new()
            .with_tls_config(tls_config)
            .https_only()
            .enable_all_versions()
            .build();
        assert_eq!(
            &connector.tls_config.alpn_protocols,
            &[b"h2".to_vec(), b"http/1.1".to_vec()]
        );
    }

    #[test]
    #[cfg(all(not(feature = "http1"), feature = "http2"))]
    fn test_alpn_http2() {
        let roots = rustls::RootCertStore::empty();
        let tls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let connector = super::ConnectorBuilder::new()
            .with_tls_config(tls_config.clone())
            .https_only()
            .enable_http2()
            .build();
        assert_eq!(&connector.tls_config.alpn_protocols, &[b"h2".to_vec()]);
        let connector = super::ConnectorBuilder::new()
            .with_tls_config(tls_config)
            .https_only()
            .enable_all_versions()
            .build();
        assert_eq!(&connector.tls_config.alpn_protocols, &[b"h2".to_vec()]);
    }
}
