/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::hyper_legacy::timeout_middleware::HttpTimeoutError;
use aws_smithy_async::future::timeout::TimedOutError;
use aws_smithy_async::rt::sleep::{default_async_sleep, AsyncSleep, SharedAsyncSleep};
use aws_smithy_runtime_api::box_error::BoxError;
use aws_smithy_runtime_api::client::connection::CaptureSmithyConnection;
use aws_smithy_runtime_api::client::connection::ConnectionMetadata;
use aws_smithy_runtime_api::client::connector_metadata::ConnectorMetadata;
use aws_smithy_runtime_api::client::http::{
    HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings, SharedHttpClient,
    SharedHttpConnector,
};
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest, HttpResponse};
use aws_smithy_runtime_api::client::result::ConnectorError;
use aws_smithy_runtime_api::client::runtime_components::{
    RuntimeComponents, RuntimeComponentsBuilder,
};
use aws_smithy_runtime_api::shared::IntoShared;
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::config_bag::ConfigBag;
use aws_smithy_types::error::display::DisplayErrorContext;
use aws_smithy_types::retry::ErrorKind;
use h2_0_3::Reason;
use hyper_0_14::client::connect::{capture_connection, CaptureConnection, Connection, HttpInfo};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::RwLock;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};

#[cfg(feature = "legacy-rustls-ring")]
mod default_connector {
    use aws_smithy_async::rt::sleep::SharedAsyncSleep;
    use aws_smithy_runtime_api::client::http::HttpConnectorSettings;
    use legacy_hyper_rustls as hyper_rustls;
    use legacy_rustls as rustls;
    use std::sync::LazyLock;

    // Creating a `with_native_roots` HTTP client takes 300ms on OS X. Cache this so that we
    // don't need to repeatedly incur that cost.
    pub(crate) static HTTPS_NATIVE_ROOTS: LazyLock<
        hyper_rustls::HttpsConnector<hyper_0_14::client::HttpConnector>,
    > = LazyLock::new(default_tls);

    fn default_tls() -> hyper_rustls::HttpsConnector<hyper_0_14::client::HttpConnector> {
        use legacy_rustls::client::WantsTransparencyPolicyOrClientCert;
        use legacy_rustls::{ClientConfig, ConfigBuilder, WantsVerifier};
        use rustls_native_certs;
        // polyfill with_native_roots from https://docs.rs/hyper-rustls/0.24.2/src/hyper_rustls/config.rs.html#22-70
        // to use the new rustls_native_certs, since rustls_native_certs 0.6 depends on rustls-pemfile which is deprecated
        fn with_native_roots(
            this: ConfigBuilder<ClientConfig, WantsVerifier>,
        ) -> ConfigBuilder<ClientConfig, WantsTransparencyPolicyOrClientCert> {
            let mut roots = rustls::RootCertStore::empty();
            let mut valid_count = 0;
            let mut invalid_count = 0;

            for cert in
                rustls_native_certs::load_native_certs().expect("could not load platform certs")
            {
                let cert = rustls::Certificate(cert.to_vec());
                match roots.add(&cert) {
                    Ok(_) => valid_count += 1,
                    Err(err) => {
                        tracing::trace!("invalid cert der {:?}", cert.0);
                        tracing::debug!("certificate parsing failed: {:?}", err);
                        invalid_count += 1
                    }
                }
            }
            tracing::debug!(
                "with_native_roots processed {} valid and {} invalid certs",
                valid_count,
                invalid_count
            );
            assert!(!roots.is_empty(), "no CA certificates found");

            this.with_root_certificates(roots)
        }
        hyper_rustls::HttpsConnectorBuilder::new()
               .with_tls_config(
                with_native_roots(rustls::ClientConfig::builder()
                    .with_cipher_suites(&[
                        // TLS1.3 suites
                        rustls::cipher_suite::TLS13_AES_256_GCM_SHA384,
                        rustls::cipher_suite::TLS13_AES_128_GCM_SHA256,
                        // TLS1.2 suites
                        rustls::cipher_suite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
                        rustls::cipher_suite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
                        rustls::cipher_suite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
                        rustls::cipher_suite::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
                        rustls::cipher_suite::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
                    ])
                    .with_safe_default_kx_groups()
                    .with_safe_default_protocol_versions()
                    .expect("Error with the TLS configuration. Please file a bug report under https://github.com/smithy-lang/smithy-rs/issues."))
                    .with_no_client_auth()
            )
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build()
    }

    pub(super) fn base(
        settings: &HttpConnectorSettings,
        sleep: Option<SharedAsyncSleep>,
    ) -> super::HyperConnectorBuilder {
        let mut hyper = super::HyperConnector::builder().connector_settings(settings.clone());
        if let Some(sleep) = sleep {
            hyper = hyper.sleep_impl(sleep);
        }
        hyper
    }

    /// Return a default HTTPS connector backed by the `rustls` crate.
    ///
    /// It requires a minimum TLS version of 1.2.
    /// It allows you to connect to both `http` and `https` URLs.
    pub(super) fn https() -> hyper_rustls::HttpsConnector<hyper_0_14::client::HttpConnector> {
        HTTPS_NATIVE_ROOTS.clone()
    }
}

/// Given `HttpConnectorSettings` and an `SharedAsyncSleep`, create a `SharedHttpConnector` from defaults depending on what cargo features are activated.
pub fn default_connector(
    settings: &HttpConnectorSettings,
    sleep: Option<SharedAsyncSleep>,
) -> Option<SharedHttpConnector> {
    #[cfg(feature = "legacy-rustls-ring")]
    {
        tracing::trace!(settings = ?settings, sleep = ?sleep, "creating a new default connector");
        let hyper = default_connector::base(settings, sleep).build_https();
        Some(SharedHttpConnector::new(hyper))
    }
    #[cfg(not(feature = "legacy-rustls-ring"))]
    {
        tracing::trace!(settings = ?settings, sleep = ?sleep, "no default connector available");
        None
    }
}

/// Creates a hyper-backed HTTPS client from defaults depending on what cargo features are activated.
pub fn default_client() -> Option<SharedHttpClient> {
    #[cfg(feature = "legacy-rustls-ring")]
    {
        tracing::trace!("creating a new default hyper 0.14.x client");
        Some(HyperClientBuilder::new().build_https())
    }
    #[cfg(not(feature = "legacy-rustls-ring"))]
    {
        tracing::trace!("no default connector available");
        None
    }
}

/// [`HttpConnector`] that uses [`hyper_0_14`] to make HTTP requests.
///
/// This connector also implements socket connect and read timeouts.
///
/// This shouldn't be used directly in most cases.
/// See the docs on [`HyperClientBuilder`] for examples of how
/// to customize the Hyper client.
#[derive(Debug)]
pub struct HyperConnector {
    adapter: Box<dyn HttpConnector>,
}

impl HyperConnector {
    /// Builder for a Hyper connector.
    pub fn builder() -> HyperConnectorBuilder {
        Default::default()
    }
}

impl HttpConnector for HyperConnector {
    fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
        self.adapter.call(request)
    }
}

/// Builder for [`HyperConnector`].
#[derive(Default, Debug)]
pub struct HyperConnectorBuilder {
    connector_settings: Option<HttpConnectorSettings>,
    sleep_impl: Option<SharedAsyncSleep>,
    client_builder: Option<hyper_0_14::client::Builder>,
}

impl HyperConnectorBuilder {
    /// Create a [`HyperConnector`] from this builder and a given connector.
    pub fn build<C>(self, tcp_connector: C) -> HyperConnector
    where
        C: Clone + Send + Sync + 'static,
        C: hyper_0_14::service::Service<http_02x::Uri>,
        C::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
        C::Future: Unpin + Send + 'static,
        C::Error: Into<BoxError>,
    {
        let client_builder = self.client_builder.unwrap_or_default();
        let sleep_impl = self.sleep_impl.or_else(default_async_sleep);
        let (connect_timeout, read_timeout) = self
            .connector_settings
            .map(|c| (c.connect_timeout(), c.read_timeout()))
            .unwrap_or((None, None));

        let connector = match connect_timeout {
            Some(duration) => timeout_middleware::ConnectTimeout::new(
                tcp_connector,
                sleep_impl
                    .clone()
                    .expect("a sleep impl must be provided in order to have a connect timeout"),
                duration,
            ),
            None => timeout_middleware::ConnectTimeout::no_timeout(tcp_connector),
        };
        let base = client_builder.build(connector);
        let read_timeout = match read_timeout {
            Some(duration) => timeout_middleware::HttpReadTimeout::new(
                base,
                sleep_impl.expect("a sleep impl must be provided in order to have a read timeout"),
                duration,
            ),
            None => timeout_middleware::HttpReadTimeout::no_timeout(base),
        };
        HyperConnector {
            adapter: Box::new(Adapter {
                client: read_timeout,
            }),
        }
    }

    /// Create a [`HyperConnector`] with the default rustls HTTPS implementation.
    #[cfg(feature = "legacy-rustls-ring")]
    pub fn build_https(self) -> HyperConnector {
        self.build(default_connector::https())
    }

    /// Set the async sleep implementation used for timeouts
    ///
    /// Calling this is only necessary for testing or to use something other than
    /// [`default_async_sleep`].
    pub fn sleep_impl(mut self, sleep_impl: impl AsyncSleep + 'static) -> Self {
        self.sleep_impl = Some(sleep_impl.into_shared());
        self
    }

    /// Set the async sleep implementation used for timeouts
    ///
    /// Calling this is only necessary for testing or to use something other than
    /// [`default_async_sleep`].
    pub fn set_sleep_impl(&mut self, sleep_impl: Option<SharedAsyncSleep>) -> &mut Self {
        self.sleep_impl = sleep_impl;
        self
    }

    /// Configure the HTTP settings for the `HyperAdapter`
    pub fn connector_settings(mut self, connector_settings: HttpConnectorSettings) -> Self {
        self.connector_settings = Some(connector_settings);
        self
    }

    /// Configure the HTTP settings for the `HyperAdapter`
    pub fn set_connector_settings(
        &mut self,
        connector_settings: Option<HttpConnectorSettings>,
    ) -> &mut Self {
        self.connector_settings = connector_settings;
        self
    }

    /// Override the Hyper client [`Builder`](hyper_0_14::client::Builder) used to construct this client.
    ///
    /// This enables changing settings like forcing HTTP2 and modifying other default client behavior.
    pub fn hyper_builder(mut self, hyper_builder: hyper_0_14::client::Builder) -> Self {
        self.client_builder = Some(hyper_builder);
        self
    }

    /// Override the Hyper client [`Builder`](hyper_0_14::client::Builder) used to construct this client.
    ///
    /// This enables changing settings like forcing HTTP2 and modifying other default client behavior.
    pub fn set_hyper_builder(
        &mut self,
        hyper_builder: Option<hyper_0_14::client::Builder>,
    ) -> &mut Self {
        self.client_builder = hyper_builder;
        self
    }
}

/// Adapter from a [`hyper_0_14::Client`] to [`HttpConnector`].
///
/// This adapter also enables TCP `CONNECT` and HTTP `READ` timeouts via [`HyperConnector::builder`].
struct Adapter<C> {
    client: timeout_middleware::HttpReadTimeout<
        hyper_0_14::Client<timeout_middleware::ConnectTimeout<C>, SdkBody>,
    >,
}

impl<C> fmt::Debug for Adapter<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Adapter")
            .field("client", &"** hyper client **")
            .finish()
    }
}

/// Extract a smithy connection from a hyper CaptureConnection
fn extract_smithy_connection(capture_conn: &CaptureConnection) -> Option<ConnectionMetadata> {
    let capture_conn = capture_conn.clone();
    if let Some(conn) = capture_conn.clone().connection_metadata().as_ref() {
        let mut extensions = http_02x::Extensions::new();
        conn.get_extras(&mut extensions);
        let http_info = extensions.get::<HttpInfo>();
        let mut builder = ConnectionMetadata::builder()
            .proxied(conn.is_proxied())
            .poison_fn(move || match capture_conn.connection_metadata().as_ref() {
                Some(conn) => conn.poison(),
                None => tracing::trace!("no connection existed to poison"),
            });

        builder
            .set_local_addr(http_info.map(|info| info.local_addr()))
            .set_remote_addr(http_info.map(|info| info.remote_addr()));

        let smithy_connection = builder.build();

        Some(smithy_connection)
    } else {
        None
    }
}

impl<C> HttpConnector for Adapter<C>
where
    C: Clone + Send + Sync + 'static,
    C: hyper_0_14::service::Service<http_02x::Uri>,
    C::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    C::Future: Unpin + Send + 'static,
    C::Error: Into<BoxError>,
{
    fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
        use hyper_0_14::service::Service;

        let mut request = match request.try_into_http02x() {
            Ok(request) => request,
            Err(err) => {
                return HttpConnectorFuture::ready(Err(ConnectorError::other(err.into(), None)));
            }
        };
        let capture_connection = capture_connection(&mut request);
        if let Some(capture_smithy_connection) =
            request.extensions().get::<CaptureSmithyConnection>()
        {
            capture_smithy_connection
                .set_connection_retriever(move || extract_smithy_connection(&capture_connection));
        }
        let mut client = self.client.clone();
        let fut = client.call(request);
        HttpConnectorFuture::new(async move {
            let response = fut
                .await
                .map_err(downcast_error)?
                .map(SdkBody::from_body_0_4);
            match HttpResponse::try_from(response) {
                Ok(response) => Ok(response),
                Err(err) => Err(ConnectorError::other(err.into(), None)),
            }
        })
    }
}

/// Downcast errors coming out of hyper into an appropriate `ConnectorError`
fn downcast_error(err: BoxError) -> ConnectorError {
    // is a `TimedOutError` (from aws_smithy_async::timeout) in the chain? if it is, this is a timeout
    if find_source::<TimedOutError>(err.as_ref()).is_some() {
        return ConnectorError::timeout(err);
    }
    // is the top of chain error actually already a `ConnectorError`? return that directly
    let err = match err.downcast::<ConnectorError>() {
        Ok(connector_error) => return *connector_error,
        Err(box_error) => box_error,
    };
    // generally, the top of chain will probably be a hyper error. Go through a set of hyper specific
    // error classifications
    let err = match err.downcast::<hyper_0_14::Error>() {
        Ok(hyper_error) => return to_connector_error(*hyper_error),
        Err(box_error) => box_error,
    };

    // otherwise, we have no idea!
    ConnectorError::other(err, None)
}

/// Convert a [`hyper_0_14::Error`] into a [`ConnectorError`]
fn to_connector_error(err: hyper_0_14::Error) -> ConnectorError {
    if err.is_timeout() || find_source::<HttpTimeoutError>(&err).is_some() {
        return ConnectorError::timeout(err.into());
    }
    if err.is_user() {
        return ConnectorError::user(err.into());
    }
    if err.is_closed() || err.is_canceled() || find_source::<std::io::Error>(&err).is_some() {
        return ConnectorError::io(err.into());
    }
    // We sometimes receive this from S3: hyper::Error(IncompleteMessage)
    if err.is_incomplete_message() {
        return ConnectorError::other(err.into(), Some(ErrorKind::TransientError));
    }
    if let Some(h2_err) = find_source::<h2_0_3::Error>(&err) {
        if h2_err.is_go_away()
            || (h2_err.is_reset() && h2_err.reason() == Some(Reason::REFUSED_STREAM))
        {
            return ConnectorError::io(err.into());
        }
    }

    tracing::warn!(err = %DisplayErrorContext(&err), "unrecognized error from Hyper. If this error should be retried, please file an issue.");
    ConnectorError::other(err.into(), None)
}

fn find_source<'a, E: Error + 'static>(err: &'a (dyn Error + 'static)) -> Option<&'a E> {
    let mut next = Some(err);
    while let Some(err) = next {
        if let Some(matching_err) = err.downcast_ref::<E>() {
            return Some(matching_err);
        }
        next = err.source();
    }
    None
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct CacheKey {
    connect_timeout: Option<Duration>,
    read_timeout: Option<Duration>,
}

impl From<&HttpConnectorSettings> for CacheKey {
    fn from(value: &HttpConnectorSettings) -> Self {
        Self {
            connect_timeout: value.connect_timeout(),
            read_timeout: value.read_timeout(),
        }
    }
}

struct HyperClient<F> {
    connector_cache: RwLock<HashMap<CacheKey, SharedHttpConnector>>,
    client_builder: hyper_0_14::client::Builder,
    tcp_connector_fn: F,
}

impl<F> fmt::Debug for HyperClient<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HyperClient")
            .field("connector_cache", &self.connector_cache)
            .field("client_builder", &self.client_builder)
            .finish()
    }
}

impl<C, F> HttpClient for HyperClient<F>
where
    F: Fn() -> C + Send + Sync,
    C: Clone + Send + Sync + 'static,
    C: hyper_0_14::service::Service<http_02x::Uri>,
    C::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    C::Future: Unpin + Send + 'static,
    C::Error: Into<BoxError>,
{
    fn http_connector(
        &self,
        settings: &HttpConnectorSettings,
        components: &RuntimeComponents,
    ) -> SharedHttpConnector {
        let key = CacheKey::from(settings);
        let mut connector = self.connector_cache.read().unwrap().get(&key).cloned();
        if connector.is_none() {
            let mut cache = self.connector_cache.write().unwrap();
            // Short-circuit if another thread already wrote a connector to the cache for this key
            if !cache.contains_key(&key) {
                let mut builder = HyperConnector::builder()
                    .hyper_builder(self.client_builder.clone())
                    .connector_settings(settings.clone());
                builder.set_sleep_impl(components.sleep_impl());

                let start = components.time_source().map(|ts| ts.now());
                let tcp_connector = (self.tcp_connector_fn)();
                let end = components.time_source().map(|ts| ts.now());
                if let (Some(start), Some(end)) = (start, end) {
                    if let Ok(elapsed) = end.duration_since(start) {
                        tracing::debug!("new TCP connector created in {:?}", elapsed);
                    }
                }
                let connector = SharedHttpConnector::new(builder.build(tcp_connector));
                cache.insert(key.clone(), connector);
            }
            connector = cache.get(&key).cloned();
        }

        connector.expect("cache populated above")
    }

    fn validate_base_client_config(
        &self,
        _: &RuntimeComponentsBuilder,
        _: &ConfigBag,
    ) -> Result<(), BoxError> {
        // Initialize the TCP connector at this point so that native certs load
        // at client initialization time instead of upon first request. We do it
        // here rather than at construction so that it won't run if this is not
        // the selected HTTP client for the base config (for example, if this was
        // the default HTTP client, and it was overridden by a later plugin).
        let _ = (self.tcp_connector_fn)();
        Ok(())
    }

    fn connector_metadata(&self) -> Option<ConnectorMetadata> {
        Some(ConnectorMetadata::new("hyper", Some(Cow::Borrowed("0.x"))))
    }
}

/// Builder for a hyper-backed [`HttpClient`] implementation.
///
/// This builder can be used to customize the underlying TCP connector used, as well as
/// hyper client configuration.
///
/// # Examples
///
/// Construct a Hyper client with the default TLS implementation (rustls).
/// This can be useful when you want to share a Hyper connector between multiple
/// generated Smithy clients.
///
/// ```no_run,ignore
/// use aws_smithy_http_client::hyper_014::HyperClientBuilder;
///
/// let http_client = HyperClientBuilder::new().build_https();
///
/// // This connector can then be given to a generated service Config
/// let config = my_service_client::Config::builder()
///     .endpoint_url("http://localhost:1234")
///     .http_client(http_client)
///     .build();
/// let client = my_service_client::Client::from_conf(config);
/// ```
///
/// ## Use a Hyper client with WebPKI roots
///
/// A use case for where you may want to use the [`HyperClientBuilder`] is when
/// setting Hyper client settings that aren't otherwise exposed by the `Config`
/// builder interface. Some examples include changing:
///
/// - Hyper client settings
/// - Allowed TLS cipher suites
/// - Using an alternative TLS connector library (not the default, rustls)
/// - CA trust root certificates (illustrated using WebPKI below)
///
/// ```no_run,ignore
/// use aws_smithy_http_client::hyper_014::HyperClientBuilder;
///
/// let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_webpki_roots()
///     .https_only()
///     .enable_http1()
///     .enable_http2()
///     .build();
/// let http_client = HyperClientBuilder::new().build(https_connector);
///
/// // This connector can then be given to a generated service Config
/// let config = my_service_client::Config::builder()
///     .endpoint_url("https://example.com")
///     .http_client(http_client)
///     .build();
/// let client = my_service_client::Client::from_conf(config);
/// ```
#[derive(Clone, Default, Debug)]
pub struct HyperClientBuilder {
    client_builder: Option<hyper_0_14::client::Builder>,
}

impl HyperClientBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the Hyper client [`Builder`](hyper_0_14::client::Builder) used to construct this client.
    ///
    /// This enables changing settings like forcing HTTP2 and modifying other default client behavior.
    pub fn hyper_builder(mut self, hyper_builder: hyper_0_14::client::Builder) -> Self {
        self.client_builder = Some(hyper_builder);
        self
    }

    /// Override the Hyper client [`Builder`](hyper_0_14::client::Builder) used to construct this client.
    ///
    /// This enables changing settings like forcing HTTP2 and modifying other default client behavior.
    pub fn set_hyper_builder(
        &mut self,
        hyper_builder: Option<hyper_0_14::client::Builder>,
    ) -> &mut Self {
        self.client_builder = hyper_builder;
        self
    }

    /// Create a hyper client with the default rustls HTTPS implementation.
    ///
    /// The trusted certificates will be loaded later when this becomes the selected
    /// HTTP client for a Smithy client.
    #[cfg(feature = "legacy-rustls-ring")]
    pub fn build_https(self) -> SharedHttpClient {
        self.build_with_fn(default_connector::https)
    }

    /// Create a [`SharedHttpClient`] from this builder and a given connector.
    ///
    #[cfg_attr(
        feature = "legacy-rustls-ring",
        doc = "Use [`build_https`](HyperClientBuilder::build_https) if you don't want to provide a custom TCP connector."
    )]
    pub fn build<C>(self, tcp_connector: C) -> SharedHttpClient
    where
        C: Clone + Send + Sync + 'static,
        C: hyper_0_14::service::Service<http_02x::Uri>,
        C::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
        C::Future: Unpin + Send + 'static,
        C::Error: Into<BoxError>,
    {
        self.build_with_fn(move || tcp_connector.clone())
    }

    fn build_with_fn<C, F>(self, tcp_connector_fn: F) -> SharedHttpClient
    where
        F: Fn() -> C + Send + Sync + 'static,
        C: Clone + Send + Sync + 'static,
        C: hyper_0_14::service::Service<http_02x::Uri>,
        C::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
        C::Future: Unpin + Send + 'static,
        C::Error: Into<BoxError>,
    {
        SharedHttpClient::new(HyperClient {
            connector_cache: RwLock::new(HashMap::new()),
            client_builder: self.client_builder.unwrap_or_default(),
            tcp_connector_fn,
        })
    }
}

mod timeout_middleware {
    use aws_smithy_async::future::timeout::{TimedOutError, Timeout};
    use aws_smithy_async::rt::sleep::Sleep;
    use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep};
    use aws_smithy_runtime_api::box_error::BoxError;
    use pin_project_lite::pin_project;
    use std::error::Error;
    use std::fmt::Formatter;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::time::Duration;

    #[derive(Debug)]
    pub(crate) struct HttpTimeoutError {
        kind: &'static str,
        duration: Duration,
    }

    impl std::fmt::Display for HttpTimeoutError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} timeout occurred after {:?}",
                self.kind, self.duration
            )
        }
    }

    impl Error for HttpTimeoutError {
        // We implement the `source` function as returning a `TimedOutError` because when `downcast_error`
        // or `find_source` is called with an `HttpTimeoutError` (or another error wrapping an `HttpTimeoutError`)
        // this method will be checked to determine if it's a timeout-related error.
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&TimedOutError)
        }
    }

    /// Timeout wrapper that will timeout on the initial TCP connection
    ///
    /// # Stability
    /// This interface is unstable.
    #[derive(Clone, Debug)]
    pub(super) struct ConnectTimeout<I> {
        inner: I,
        timeout: Option<(SharedAsyncSleep, Duration)>,
    }

    impl<I> ConnectTimeout<I> {
        /// Create a new `ConnectTimeout` around `inner`.
        ///
        /// Typically, `I` will implement [`hyper_0_14::client::connect::Connect`].
        pub(crate) fn new(inner: I, sleep: SharedAsyncSleep, timeout: Duration) -> Self {
            Self {
                inner,
                timeout: Some((sleep, timeout)),
            }
        }

        pub(crate) fn no_timeout(inner: I) -> Self {
            Self {
                inner,
                timeout: None,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub(crate) struct HttpReadTimeout<I> {
        inner: I,
        timeout: Option<(SharedAsyncSleep, Duration)>,
    }

    impl<I> HttpReadTimeout<I> {
        /// Create a new `HttpReadTimeout` around `inner`.
        ///
        /// Typically, `I` will implement [`hyper_0_14::service::Service<http::Request<SdkBody>>`].
        pub(crate) fn new(inner: I, sleep: SharedAsyncSleep, timeout: Duration) -> Self {
            Self {
                inner,
                timeout: Some((sleep, timeout)),
            }
        }

        pub(crate) fn no_timeout(inner: I) -> Self {
            Self {
                inner,
                timeout: None,
            }
        }
    }

    pin_project! {
        /// Timeout future for Tower services
        ///
        /// Timeout future to handle timing out, mapping errors, and the possibility of not timing out
        /// without incurring an additional allocation for each timeout layer.
        #[project = MaybeTimeoutFutureProj]
        pub enum MaybeTimeoutFuture<F> {
            Timeout {
                #[pin]
                timeout: Timeout<F, Sleep>,
                error_type: &'static str,
                duration: Duration,
            },
            NoTimeout {
                #[pin]
                future: F
            }
        }
    }

    impl<F, T, E> Future for MaybeTimeoutFuture<F>
    where
        F: Future<Output = Result<T, E>>,
        E: Into<BoxError>,
    {
        type Output = Result<T, BoxError>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let (timeout_future, kind, &mut duration) = match self.project() {
                MaybeTimeoutFutureProj::NoTimeout { future } => {
                    return future.poll(cx).map_err(|err| err.into());
                }
                MaybeTimeoutFutureProj::Timeout {
                    timeout,
                    error_type,
                    duration,
                } => (timeout, error_type, duration),
            };
            match timeout_future.poll(cx) {
                Poll::Ready(Ok(response)) => Poll::Ready(response.map_err(|err| err.into())),
                Poll::Ready(Err(_timeout)) => {
                    Poll::Ready(Err(HttpTimeoutError { kind, duration }.into()))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }

    impl<I> hyper_0_14::service::Service<http_02x::Uri> for ConnectTimeout<I>
    where
        I: hyper_0_14::service::Service<http_02x::Uri>,
        I::Error: Into<BoxError>,
    {
        type Response = I::Response;
        type Error = BoxError;
        type Future = MaybeTimeoutFuture<I::Future>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx).map_err(|err| err.into())
        }

        fn call(&mut self, req: http_02x::Uri) -> Self::Future {
            match &self.timeout {
                Some((sleep, duration)) => {
                    let sleep = sleep.sleep(*duration);
                    MaybeTimeoutFuture::Timeout {
                        timeout: Timeout::new(self.inner.call(req), sleep),
                        error_type: "HTTP connect",
                        duration: *duration,
                    }
                }
                None => MaybeTimeoutFuture::NoTimeout {
                    future: self.inner.call(req),
                },
            }
        }
    }

    impl<I, B> hyper_0_14::service::Service<http_02x::Request<B>> for HttpReadTimeout<I>
    where
        I: hyper_0_14::service::Service<http_02x::Request<B>, Error = hyper_0_14::Error>,
    {
        type Response = I::Response;
        type Error = BoxError;
        type Future = MaybeTimeoutFuture<I::Future>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx).map_err(|err| err.into())
        }

        fn call(&mut self, req: http_02x::Request<B>) -> Self::Future {
            match &self.timeout {
                Some((sleep, duration)) => {
                    let sleep = sleep.sleep(*duration);
                    MaybeTimeoutFuture::Timeout {
                        timeout: Timeout::new(self.inner.call(req), sleep),
                        error_type: "HTTP read",
                        duration: *duration,
                    }
                }
                None => MaybeTimeoutFuture::NoTimeout {
                    future: self.inner.call(req),
                },
            }
        }
    }

    #[cfg(test)]
    pub(crate) mod test {
        use crate::hyper_014::HyperConnector;
        use aws_smithy_async::assert_elapsed;
        use aws_smithy_async::future::never::Never;
        use aws_smithy_async::rt::sleep::{SharedAsyncSleep, TokioSleep};
        use aws_smithy_runtime_api::box_error::BoxError;
        use aws_smithy_runtime_api::client::http::HttpConnectorSettings;
        use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
        use aws_smithy_runtime_api::client::result::ConnectorError;
        use aws_smithy_types::error::display::DisplayErrorContext;
        use hyper_0_14::client::connect::{Connected, Connection};
        use std::future::Future;
        use std::pin::Pin;
        use std::task::{Context, Poll};
        use std::time::Duration;
        use tokio::io::ReadBuf;
        use tokio::io::{AsyncRead, AsyncWrite};
        use tokio::net::TcpStream;

        #[allow(unused)]
        fn connect_timeout_is_correct<T: Send + Sync + Clone + 'static>() {
            is_send_sync::<super::ConnectTimeout<T>>();
        }

        #[allow(unused)]
        fn is_send_sync<T: Send + Sync>() {}

        /// A service that will never return whatever it is you want
        ///
        /// Returned futures will return Pending forever
        #[non_exhaustive]
        #[derive(Clone, Default, Debug)]
        pub(crate) struct NeverConnects;
        impl hyper_0_14::service::Service<http_02x::Uri> for NeverConnects {
            type Response = TcpStream;
            type Error = ConnectorError;
            type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

            fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }

            fn call(&mut self, _uri: http_02x::Uri) -> Self::Future {
                Box::pin(async move {
                    Never::new().await;
                    unreachable!()
                })
            }
        }

        /// A service that will connect but never send any data
        #[derive(Clone, Debug, Default)]
        struct NeverReplies;
        impl hyper_0_14::service::Service<http_02x::Uri> for NeverReplies {
            type Response = EmptyStream;
            type Error = BoxError;
            type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

            fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
                Poll::Ready(Ok(()))
            }

            fn call(&mut self, _req: http_02x::Uri) -> Self::Future {
                std::future::ready(Ok(EmptyStream))
            }
        }

        /// A stream that will never return or accept any data
        #[non_exhaustive]
        #[derive(Debug, Default)]
        struct EmptyStream;
        impl AsyncRead for EmptyStream {
            fn poll_read(
                self: Pin<&mut Self>,
                _cx: &mut Context<'_>,
                _buf: &mut ReadBuf<'_>,
            ) -> Poll<std::io::Result<()>> {
                Poll::Pending
            }
        }
        impl AsyncWrite for EmptyStream {
            fn poll_write(
                self: Pin<&mut Self>,
                _cx: &mut Context<'_>,
                _buf: &[u8],
            ) -> Poll<Result<usize, std::io::Error>> {
                Poll::Pending
            }

            fn poll_flush(
                self: Pin<&mut Self>,
                _cx: &mut Context<'_>,
            ) -> Poll<Result<(), std::io::Error>> {
                Poll::Pending
            }

            fn poll_shutdown(
                self: Pin<&mut Self>,
                _cx: &mut Context<'_>,
            ) -> Poll<Result<(), std::io::Error>> {
                Poll::Pending
            }
        }
        impl Connection for EmptyStream {
            fn connected(&self) -> Connected {
                Connected::new()
            }
        }

        #[tokio::test]
        async fn http_connect_timeout_works() {
            let tcp_connector = NeverConnects::default();
            let connector_settings = HttpConnectorSettings::builder()
                .connect_timeout(Duration::from_secs(1))
                .build();
            let hyper = HyperConnector::builder()
                .connector_settings(connector_settings)
                .sleep_impl(SharedAsyncSleep::new(TokioSleep::new()))
                .build(tcp_connector)
                .adapter;
            let now = tokio::time::Instant::now();
            tokio::time::pause();
            let resp = hyper
                .call(HttpRequest::get("https://static-uri.com").unwrap())
                .await
                .unwrap_err();
            assert!(
                resp.is_timeout(),
                "expected resp.is_timeout() to be true but it was false, resp == {:?}",
                resp
            );
            let message = DisplayErrorContext(&resp).to_string();
            let expected =
                "timeout: error trying to connect: HTTP connect timeout occurred after 1s";
            assert!(
                message.contains(expected),
                "expected '{message}' to contain '{expected}'"
            );
            assert_elapsed!(now, Duration::from_secs(1));
        }

        #[tokio::test]
        async fn http_read_timeout_works() {
            let tcp_connector = NeverReplies;
            let connector_settings = HttpConnectorSettings::builder()
                .connect_timeout(Duration::from_secs(1))
                .read_timeout(Duration::from_secs(2))
                .build();
            let hyper = HyperConnector::builder()
                .connector_settings(connector_settings)
                .sleep_impl(SharedAsyncSleep::new(TokioSleep::new()))
                .build(tcp_connector)
                .adapter;
            let now = tokio::time::Instant::now();
            tokio::time::pause();
            let err = hyper
                .call(HttpRequest::get("https://fake-uri.com").unwrap())
                .await
                .unwrap_err();
            assert!(
                err.is_timeout(),
                "expected err.is_timeout() to be true but it was false, err == {err:?}",
            );
            let message = format!("{}", DisplayErrorContext(&err));
            let expected = "timeout: HTTP read timeout occurred after 2s";
            assert!(
                message.contains(expected),
                "expected '{message}' to contain '{expected}'"
            );
            assert_elapsed!(now, Duration::from_secs(2));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::hyper_legacy::timeout_middleware::test::NeverConnects;
    use crate::hyper_legacy::{HyperClientBuilder, HyperConnector};
    use aws_smithy_async::time::SystemTimeSource;
    use aws_smithy_runtime_api::box_error::BoxError;
    use aws_smithy_runtime_api::client::http::{HttpClient, HttpConnectorSettings};
    use aws_smithy_runtime_api::client::orchestrator::HttpRequest;
    use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
    use hyper_0_14::client::connect::{Connected, Connection};
    use std::io::{Error, ErrorKind};
    use std::pin::Pin;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::task::{Context, Poll};
    use std::time::Duration;
    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

    #[tokio::test]
    async fn connector_selection() {
        // Create a client that increments a count every time it creates a new HyperConnector
        let creation_count = Arc::new(AtomicU32::new(0));
        let http_client = HyperClientBuilder::new().build_with_fn({
            let count = creation_count.clone();
            move || {
                count.fetch_add(1, Ordering::Relaxed);
                NeverConnects::default()
            }
        });

        // This configuration should result in 4 separate connectors with different timeout settings
        let settings = [
            HttpConnectorSettings::builder()
                .connect_timeout(Duration::from_secs(3))
                .build(),
            HttpConnectorSettings::builder()
                .read_timeout(Duration::from_secs(3))
                .build(),
            HttpConnectorSettings::builder()
                .connect_timeout(Duration::from_secs(3))
                .read_timeout(Duration::from_secs(3))
                .build(),
            HttpConnectorSettings::builder()
                .connect_timeout(Duration::from_secs(5))
                .read_timeout(Duration::from_secs(3))
                .build(),
        ];

        // Kick off thousands of parallel tasks that will try to create a connector
        let components = RuntimeComponentsBuilder::for_tests()
            .with_time_source(Some(SystemTimeSource::new()))
            .build()
            .unwrap();
        let mut handles = Vec::new();
        for setting in &settings {
            for _ in 0..1000 {
                let client = http_client.clone();
                handles.push(tokio::spawn({
                    let setting = setting.clone();
                    let components = components.clone();
                    async move {
                        let _ = client.http_connector(&setting, &components);
                    }
                }));
            }
        }
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify only 4 connectors were created amidst the chaos
        assert_eq!(4, creation_count.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn hyper_io_error() {
        let connector = TestConnection {
            inner: HangupStream,
        };
        let adapter = HyperConnector::builder().build(connector).adapter;
        let err = adapter
            .call(HttpRequest::get("https://socket-hangup.com").unwrap())
            .await
            .expect_err("socket hangup");
        assert!(err.is_io(), "{:?}", err);
    }

    // ---- machinery to make a Hyper connector that responds with an IO Error
    #[derive(Clone)]
    struct HangupStream;

    impl Connection for HangupStream {
        fn connected(&self) -> Connected {
            Connected::new()
        }
    }

    impl AsyncRead for HangupStream {
        fn poll_read(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            _buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            Poll::Ready(Err(Error::new(
                ErrorKind::ConnectionReset,
                "connection reset",
            )))
        }
    }

    impl AsyncWrite for HangupStream {
        fn poll_write(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            _buf: &[u8],
        ) -> Poll<Result<usize, Error>> {
            Poll::Pending
        }

        fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
            Poll::Pending
        }

        fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
            Poll::Pending
        }
    }

    #[derive(Clone)]
    struct TestConnection<T> {
        inner: T,
    }

    impl<T> hyper_0_14::service::Service<http_02x::Uri> for TestConnection<T>
    where
        T: Clone + Connection,
    {
        type Response = T;
        type Error = BoxError;
        type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: http_02x::Uri) -> Self::Future {
            std::future::ready(Ok(self.inner.clone()))
        }
    }
}
