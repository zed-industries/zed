/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![deny(missing_docs)]

//! AWS Shared Config
//!
//! This module contains a shared configuration representation that is agnostic from a specific service.

use crate::app_name::AppName;
use crate::docs_for;
use crate::endpoint_config::AccountIdEndpointMode;
use crate::origin::Origin;
use crate::region::Region;
use crate::service_config::LoadServiceConfig;
use aws_credential_types::provider::token::SharedTokenProvider;
pub use aws_credential_types::provider::SharedCredentialsProvider;
use aws_smithy_async::rt::sleep::AsyncSleep;
pub use aws_smithy_async::rt::sleep::SharedAsyncSleep;
pub use aws_smithy_async::time::{SharedTimeSource, TimeSource};
use aws_smithy_runtime_api::client::auth::AuthSchemePreference;
use aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;
use aws_smithy_runtime_api::client::http::HttpClient;
pub use aws_smithy_runtime_api::client::http::SharedHttpClient;
use aws_smithy_runtime_api::client::identity::{ResolveCachedIdentity, SharedIdentityCache};
pub use aws_smithy_runtime_api::client::stalled_stream_protection::StalledStreamProtectionConfig;
use aws_smithy_runtime_api::shared::IntoShared;
pub use aws_smithy_types::checksum_config::{
    RequestChecksumCalculation, ResponseChecksumValidation,
};
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_smithy_types::timeout::TimeoutConfig;
use std::collections::HashMap;
use std::sync::Arc;

/// Unified docstrings to keep crates in sync. Not intended for public use
pub mod unified_docs {
    /// A macro that generates docs for selected fields of `SdkConfig`.
    #[macro_export]
    macro_rules! docs_for {
        (use_fips) => {
"When true, send this request to the FIPS-compliant regional endpoint.

If no FIPS-compliant endpoint can be determined, dispatching the request will return an error."
        };
        (use_dual_stack) => {
"When true, send this request to the dual-stack endpoint.

If no dual-stack endpoint is available the request MAY return an error.

**Note**: Some services do not offer dual-stack as a configurable parameter (e.g. Code Catalyst). For
these services, this setting has no effect"
        };
        (time_source) => {
"The time source use to use for this client.

This only needs to be required for creating deterministic tests or platforms where `SystemTime::now()` is not supported."};
        (disable_request_compression) => {
"When `true`, disable request compression. Defaults to `false`.

**Only some services support request compression.** For services
that don't support request compression, this setting does nothing.
" };
        (request_min_compression_size_bytes) => {
"The minimum size of request that should be compressed. Defaults to `10240` bytes.

When a request body's size is lower than this, request compression will be skipped.
This is useful for request bodies because, for small request bodies, compression may actually increase their size.

**Only some services support request compression.** For services
that don't support request compression, this setting does nothing.
" };
        (account_id_endpoint_mode) => {
"Controls the account ID-based routing behavior.

By default, the routing behavior is set to `preferred`.
Customers can adjust this setting to other values to switch between different routing patterns or temporarily disable the feature.

See the developer guide on [account-based endpoints](https://docs.aws.amazon.com/sdkref/latest/guide/feature-account-endpoints.html)
for more information.

For services that do not use the account-based endpoints, this setting does nothing.
" };
        (auth_scheme_preference) => {
"Set the auth scheme preference for an auth scheme resolver
(typically the default auth scheme resolver).

Each operation has a predefined order of auth schemes, as determined by the service,
for auth scheme resolution. By using the auth scheme preference, customers
can reorder the schemes resolved by the auth scheme resolver.

The preference list is intended as a hint rather than a strict override.
Any schemes not present in the originally resolved auth schemes will be ignored.
" };
    }
}

/// AWS Shared Configuration
#[derive(Debug, Clone)]
pub struct SdkConfig {
    app_name: Option<AppName>,
    auth_scheme_preference: Option<AuthSchemePreference>,
    identity_cache: Option<SharedIdentityCache>,
    credentials_provider: Option<SharedCredentialsProvider>,
    token_provider: Option<SharedTokenProvider>,
    region: Option<Region>,
    account_id_endpoint_mode: Option<AccountIdEndpointMode>,
    endpoint_url: Option<String>,
    retry_config: Option<RetryConfig>,
    sleep_impl: Option<SharedAsyncSleep>,
    time_source: Option<SharedTimeSource>,
    timeout_config: Option<TimeoutConfig>,
    stalled_stream_protection_config: Option<StalledStreamProtectionConfig>,
    http_client: Option<SharedHttpClient>,
    use_fips: Option<bool>,
    use_dual_stack: Option<bool>,
    behavior_version: Option<BehaviorVersion>,
    service_config: Option<Arc<dyn LoadServiceConfig>>,
    config_origins: HashMap<&'static str, Origin>,
    disable_request_compression: Option<bool>,
    request_min_compression_size_bytes: Option<u32>,
    request_checksum_calculation: Option<RequestChecksumCalculation>,
    response_checksum_validation: Option<ResponseChecksumValidation>,
}

/// Builder for AWS Shared Configuration
///
/// _Important:_ Using the `aws-config` crate to configure the SDK is preferred to invoking this
/// builder directly. Using this builder directly won't pull in any AWS recommended default
/// configuration values.
#[derive(Debug, Default)]
pub struct Builder {
    app_name: Option<AppName>,
    auth_scheme_preference: Option<AuthSchemePreference>,
    identity_cache: Option<SharedIdentityCache>,
    credentials_provider: Option<SharedCredentialsProvider>,
    token_provider: Option<SharedTokenProvider>,
    region: Option<Region>,
    account_id_endpoint_mode: Option<AccountIdEndpointMode>,
    endpoint_url: Option<String>,
    retry_config: Option<RetryConfig>,
    sleep_impl: Option<SharedAsyncSleep>,
    time_source: Option<SharedTimeSource>,
    timeout_config: Option<TimeoutConfig>,
    stalled_stream_protection_config: Option<StalledStreamProtectionConfig>,
    http_client: Option<SharedHttpClient>,
    use_fips: Option<bool>,
    use_dual_stack: Option<bool>,
    behavior_version: Option<BehaviorVersion>,
    service_config: Option<Arc<dyn LoadServiceConfig>>,
    config_origins: HashMap<&'static str, Origin>,
    disable_request_compression: Option<bool>,
    request_min_compression_size_bytes: Option<u32>,
    request_checksum_calculation: Option<RequestChecksumCalculation>,
    response_checksum_validation: Option<ResponseChecksumValidation>,
}

impl Builder {
    /// Set the region for the builder
    ///
    /// # Examples
    /// ```rust
    /// use aws_types::SdkConfig;
    /// use aws_types::region::Region;
    /// let config = SdkConfig::builder().region(Region::new("us-east-1")).build();
    /// ```
    pub fn region(mut self, region: impl Into<Option<Region>>) -> Self {
        self.set_region(region);
        self
    }

    /// Set the region for the builder
    ///
    /// # Examples
    /// ```rust
    /// fn region_override() -> Option<Region> {
    ///     // ...
    ///     # None
    /// }
    /// use aws_types::SdkConfig;
    /// use aws_types::region::Region;
    /// let mut builder = SdkConfig::builder();
    /// if let Some(region) = region_override() {
    ///     builder.set_region(region);
    /// }
    /// let config = builder.build();
    /// ```
    pub fn set_region(&mut self, region: impl Into<Option<Region>>) -> &mut Self {
        self.region = region.into();
        self
    }

    #[doc = docs_for!(account_id_endpoint_mode)]
    pub fn account_id_endpoint_mode(
        mut self,
        account_id_endpoint_mode: AccountIdEndpointMode,
    ) -> Self {
        self.set_account_id_endpoint_mode(Some(account_id_endpoint_mode));
        self
    }

    #[doc = docs_for!(account_id_endpoint_mode)]
    pub fn set_account_id_endpoint_mode(
        &mut self,
        account_id_endpoint_mode: Option<AccountIdEndpointMode>,
    ) -> &mut Self {
        self.account_id_endpoint_mode = account_id_endpoint_mode;
        self
    }

    /// Set the endpoint URL to use when making requests.
    /// # Examples
    /// ```
    /// use aws_types::SdkConfig;
    /// let config = SdkConfig::builder().endpoint_url("http://localhost:8080").build();
    /// ```
    pub fn endpoint_url(mut self, endpoint_url: impl Into<String>) -> Self {
        self.set_endpoint_url(Some(endpoint_url.into()));
        self
    }

    /// Set the endpoint URL to use when making requests.
    pub fn set_endpoint_url(&mut self, endpoint_url: Option<String>) -> &mut Self {
        self.endpoint_url = endpoint_url;
        self
    }

    /// Set the checksum calculation strategy to use when making requests.
    /// # Examples
    /// ```
    /// use aws_types::SdkConfig;
    /// use aws_smithy_types::checksum_config::RequestChecksumCalculation;
    /// let config = SdkConfig::builder().request_checksum_calculation(RequestChecksumCalculation::WhenSupported).build();
    /// ```
    pub fn request_checksum_calculation(
        mut self,
        request_checksum_calculation: RequestChecksumCalculation,
    ) -> Self {
        self.set_request_checksum_calculation(Some(request_checksum_calculation));
        self
    }

    /// Set the checksum calculation strategy to use when making requests.
    pub fn set_request_checksum_calculation(
        &mut self,
        request_checksum_calculation: Option<RequestChecksumCalculation>,
    ) -> &mut Self {
        self.request_checksum_calculation = request_checksum_calculation;
        self
    }

    /// Set the checksum calculation strategy to use for responses.
    /// # Examples
    /// ```
    /// use aws_types::SdkConfig;
    /// use aws_smithy_types::checksum_config::ResponseChecksumValidation;
    /// let config = SdkConfig::builder().response_checksum_validation(ResponseChecksumValidation::WhenSupported).build();
    /// ```
    pub fn response_checksum_validation(
        mut self,
        response_checksum_validation: ResponseChecksumValidation,
    ) -> Self {
        self.set_response_checksum_validation(Some(response_checksum_validation));
        self
    }

    /// Set the checksum calculation strategy to use for responses.
    pub fn set_response_checksum_validation(
        &mut self,
        response_checksum_validation: Option<ResponseChecksumValidation>,
    ) -> &mut Self {
        self.response_checksum_validation = response_checksum_validation;
        self
    }

    /// Set the retry_config for the builder
    ///
    /// _Note:_ Retries require a sleep implementation in order to work. When enabling retry, make
    /// sure to set one with [Self::sleep_impl] or [Self::set_sleep_impl].
    ///
    /// # Examples
    /// ```rust
    /// use aws_types::SdkConfig;
    /// use aws_smithy_types::retry::RetryConfig;
    ///
    /// let retry_config = RetryConfig::standard().with_max_attempts(5);
    /// let config = SdkConfig::builder().retry_config(retry_config).build();
    /// ```
    pub fn retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.set_retry_config(Some(retry_config));
        self
    }

    /// Set the retry_config for the builder
    ///
    /// _Note:_ Retries require a sleep implementation in order to work. When enabling retry, make
    /// sure to set one with [Self::sleep_impl] or [Self::set_sleep_impl].
    ///
    /// # Examples
    /// ```rust
    /// use aws_types::sdk_config::{SdkConfig, Builder};
    /// use aws_smithy_types::retry::RetryConfig;
    ///
    /// fn disable_retries(builder: &mut Builder) {
    ///     let retry_config = RetryConfig::standard().with_max_attempts(1);
    ///     builder.set_retry_config(Some(retry_config));
    /// }
    ///
    /// let mut builder = SdkConfig::builder();
    /// disable_retries(&mut builder);
    /// ```
    pub fn set_retry_config(&mut self, retry_config: Option<RetryConfig>) -> &mut Self {
        self.retry_config = retry_config;
        self
    }

    /// Set the [`TimeoutConfig`] for the builder
    ///
    /// _Note:_ Timeouts require a sleep implementation in order to work.
    /// When enabling timeouts, be sure to set one with [Self::sleep_impl] or
    /// [Self::set_sleep_impl].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// use aws_types::SdkConfig;
    /// use aws_smithy_types::timeout::TimeoutConfig;
    ///
    /// let timeout_config = TimeoutConfig::builder()
    ///     .operation_attempt_timeout(Duration::from_secs(2))
    ///     .operation_timeout(Duration::from_secs(5))
    ///     .build();
    /// let config = SdkConfig::builder()
    ///     .timeout_config(timeout_config)
    ///     .build();
    /// ```
    pub fn timeout_config(mut self, timeout_config: TimeoutConfig) -> Self {
        self.set_timeout_config(Some(timeout_config));
        self
    }

    /// Set the [`TimeoutConfig`] for the builder
    ///
    /// _Note:_ Timeouts require a sleep implementation in order to work.
    /// When enabling timeouts, be sure to set one with [Self::sleep_impl] or
    /// [Self::set_sleep_impl].
    ///
    /// # Examples
    /// ```rust
    /// # use std::time::Duration;
    /// use aws_types::sdk_config::{SdkConfig, Builder};
    /// use aws_smithy_types::timeout::TimeoutConfig;
    ///
    /// fn set_preferred_timeouts(builder: &mut Builder) {
    ///     let timeout_config = TimeoutConfig::builder()
    ///         .operation_attempt_timeout(Duration::from_secs(2))
    ///         .operation_timeout(Duration::from_secs(5))
    ///         .build();
    ///     builder.set_timeout_config(Some(timeout_config));
    /// }
    ///
    /// let mut builder = SdkConfig::builder();
    /// set_preferred_timeouts(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_timeout_config(&mut self, timeout_config: Option<TimeoutConfig>) -> &mut Self {
        self.timeout_config = timeout_config;
        self
    }

    /// Set the sleep implementation for the builder.
    ///
    /// The sleep implementation is used to create timeout futures.
    ///
    /// _Note:_ If you're using the Tokio runtime, a `TokioSleep` implementation is available in
    /// the `aws-smithy-async` crate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep, Sleep};
    /// use aws_types::SdkConfig;
    ///
    /// ##[derive(Debug)]
    /// pub struct ForeverSleep;
    ///
    /// impl AsyncSleep for ForeverSleep {
    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
    ///         Sleep::new(std::future::pending())
    ///     }
    /// }
    ///
    /// let sleep_impl = SharedAsyncSleep::new(ForeverSleep);
    /// let config = SdkConfig::builder().sleep_impl(sleep_impl).build();
    /// ```
    pub fn sleep_impl(mut self, sleep_impl: impl AsyncSleep + 'static) -> Self {
        self.set_sleep_impl(Some(sleep_impl.into_shared()));
        self
    }

    /// Set the sleep implementation for the builder. The sleep implementation is used to create
    /// timeout futures.
    ///
    /// _Note:_ If you're using the Tokio runtime, a `TokioSleep` implementation is available in
    /// the `aws-smithy-async` crate.
    ///
    /// # Examples
    /// ```rust
    /// # use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep, Sleep};
    /// # use aws_types::sdk_config::{Builder, SdkConfig};
    /// #[derive(Debug)]
    /// pub struct ForeverSleep;
    ///
    /// impl AsyncSleep for ForeverSleep {
    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
    ///         Sleep::new(std::future::pending())
    ///     }
    /// }
    ///
    /// fn set_never_ending_sleep_impl(builder: &mut Builder) {
    ///     let sleep_impl = SharedAsyncSleep::new(ForeverSleep);
    ///     builder.set_sleep_impl(Some(sleep_impl));
    /// }
    ///
    /// let mut builder = SdkConfig::builder();
    /// set_never_ending_sleep_impl(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_sleep_impl(&mut self, sleep_impl: Option<SharedAsyncSleep>) -> &mut Self {
        self.sleep_impl = sleep_impl;
        self
    }

    /// Set the identity cache for caching credentials and SSO tokens.
    ///
    /// The default identity cache will wait until the first request that requires authentication
    /// to load an identity. Once the identity is loaded, it is cached until shortly before it
    /// expires.
    ///
    /// # Examples
    /// Disabling identity caching:
    /// ```rust
    /// # use aws_types::SdkConfig;
    /// use aws_smithy_runtime::client::identity::IdentityCache;
    /// let config = SdkConfig::builder()
    ///     .identity_cache(IdentityCache::no_cache())
    ///     .build();
    /// ```
    /// Changing settings on the default cache implementation:
    /// ```rust
    /// # use aws_types::SdkConfig;
    /// use aws_smithy_runtime::client::identity::IdentityCache;
    /// use std::time::Duration;
    ///
    /// let config = SdkConfig::builder()
    ///     .identity_cache(
    ///         IdentityCache::lazy()
    ///             .load_timeout(Duration::from_secs(10))
    ///             .build()
    ///     )
    ///     .build();
    /// ```
    pub fn identity_cache(mut self, cache: impl ResolveCachedIdentity + 'static) -> Self {
        self.set_identity_cache(Some(cache.into_shared()));
        self
    }

    /// Set the identity cache for caching credentials and SSO tokens.
    ///
    /// The default identity cache will wait until the first request that requires authentication
    /// to load an identity. Once the identity is loaded, it is cached until shortly before it
    /// expires.
    ///
    /// # Examples
    /// ```rust
    /// # use aws_types::SdkConfig;
    /// use aws_smithy_runtime::client::identity::IdentityCache;
    ///
    /// fn override_identity_cache() -> bool {
    ///   // ...
    ///   # true
    /// }
    ///
    /// let mut builder = SdkConfig::builder();
    /// if override_identity_cache() {
    ///     builder.set_identity_cache(Some(IdentityCache::lazy().build()));
    /// }
    /// let config = builder.build();
    /// ```
    pub fn set_identity_cache(&mut self, cache: Option<SharedIdentityCache>) -> &mut Self {
        self.identity_cache = cache;
        self
    }

    /// Set the credentials provider for the builder
    ///
    /// # Examples
    /// ```rust
    /// use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
    /// use aws_types::SdkConfig;
    /// fn make_provider() -> impl ProvideCredentials {
    ///   // ...
    ///   # use aws_credential_types::Credentials;
    ///   # Credentials::new("test", "test", None, None, "example")
    /// }
    ///
    /// let config = SdkConfig::builder()
    ///     .credentials_provider(SharedCredentialsProvider::new(make_provider()))
    ///     .build();
    /// ```
    pub fn credentials_provider(mut self, provider: SharedCredentialsProvider) -> Self {
        self.set_credentials_provider(Some(provider));
        self
    }

    /// Set the credentials provider for the builder
    ///
    /// # Examples
    /// ```rust
    /// use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
    /// use aws_types::SdkConfig;
    /// fn make_provider() -> impl ProvideCredentials {
    ///   // ...
    ///   # use aws_credential_types::Credentials;
    ///   # Credentials::new("test", "test", None, None, "example")
    /// }
    ///
    /// fn override_provider() -> bool {
    ///   // ...
    ///   # true
    /// }
    ///
    /// let mut builder = SdkConfig::builder();
    /// if override_provider() {
    ///     builder.set_credentials_provider(Some(SharedCredentialsProvider::new(make_provider())));
    /// }
    /// let config = builder.build();
    /// ```
    pub fn set_credentials_provider(
        &mut self,
        provider: Option<SharedCredentialsProvider>,
    ) -> &mut Self {
        self.credentials_provider = provider;
        self
    }

    /// Set the bearer auth token provider for the builder
    ///
    /// # Examples
    /// ```rust
    /// use aws_credential_types::provider::token::{ProvideToken, SharedTokenProvider};
    /// use aws_types::SdkConfig;
    ///
    /// fn make_provider() -> impl ProvideToken {
    ///   // ...
    ///   # aws_credential_types::Token::new("example", None)
    /// }
    ///
    /// let config = SdkConfig::builder()
    ///     .token_provider(SharedTokenProvider::new(make_provider()))
    ///     .build();
    /// ```
    pub fn token_provider(mut self, provider: SharedTokenProvider) -> Self {
        self.set_token_provider(Some(provider));
        self
    }

    /// Set the bearer auth token provider for the builder
    ///
    /// # Examples
    /// ```rust
    /// use aws_credential_types::provider::token::{ProvideToken, SharedTokenProvider};
    /// use aws_types::SdkConfig;
    ///
    /// fn make_provider() -> impl ProvideToken {
    ///   // ...
    ///   # aws_credential_types::Token::new("example", None)
    /// }
    ///
    /// fn override_provider() -> bool {
    ///   // ...
    ///   # true
    /// }
    ///
    /// let mut builder = SdkConfig::builder();
    /// if override_provider() {
    ///     builder.set_token_provider(Some(SharedTokenProvider::new(make_provider())));
    /// }
    /// let config = builder.build();
    /// ```
    pub fn set_token_provider(&mut self, provider: Option<SharedTokenProvider>) -> &mut Self {
        self.token_provider = provider;
        self
    }

    /// Sets the name of the app that is using the client.
    ///
    /// This _optional_ name is used to identify the application in the user agent that
    /// gets sent along with requests.
    pub fn app_name(mut self, app_name: AppName) -> Self {
        self.set_app_name(Some(app_name));
        self
    }

    /// Sets the name of the app that is using the client.
    ///
    /// This _optional_ name is used to identify the application in the user agent that
    /// gets sent along with requests.
    pub fn set_app_name(&mut self, app_name: Option<AppName>) -> &mut Self {
        self.app_name = app_name;
        self
    }

    /// Sets the HTTP client to use when making requests.
    ///
    /// ## Examples
    /// ```no_run
    /// # #[cfg(feature = "examples")]
    /// # fn example() {
    /// use aws_types::sdk_config::{SdkConfig, TimeoutConfig};
    /// use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
    /// use std::time::Duration;
    ///
    /// // Create a connector that will be used to establish TLS connections
    /// let tls_connector = hyper_rustls::HttpsConnectorBuilder::new()
    ///     .with_webpki_roots()
    ///     .https_only()
    ///     .enable_http1()
    ///     .enable_http2()
    ///     .build();
    /// // Create a HTTP client that uses the TLS connector. This client is
    /// // responsible for creating and caching a HttpConnector when given HttpConnectorSettings.
    /// // This hyper client will create HttpConnectors backed by hyper and the tls_connector.
    /// let http_client = HyperClientBuilder::new().build(tls_connector);
    /// let sdk_config = SdkConfig::builder()
    ///     .http_client(http_client)
    ///     // Connect/read timeouts are passed to the HTTP client when servicing a request
    ///     .timeout_config(
    ///         TimeoutConfig::builder()
    ///             .connect_timeout(Duration::from_secs(5))
    ///             .build()
    ///     )
    ///     .build();
    /// # }
    /// ```
    pub fn http_client(mut self, http_client: impl HttpClient + 'static) -> Self {
        self.set_http_client(Some(http_client.into_shared()));
        self
    }

    /// Sets the HTTP client to use when making requests.
    ///
    /// ## Examples
    /// ```no_run
    /// # #[cfg(feature = "examples")]
    /// # fn example() {
    /// use aws_types::sdk_config::{Builder, SdkConfig, TimeoutConfig};
    /// use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
    /// use std::time::Duration;
    ///
    /// fn override_http_client(builder: &mut Builder) {
    ///     // Create a connector that will be used to establish TLS connections
    ///     let tls_connector = hyper_rustls::HttpsConnectorBuilder::new()
    ///         .with_webpki_roots()
    ///         .https_only()
    ///         .enable_http1()
    ///         .enable_http2()
    ///         .build();
    ///     // Create a HTTP client that uses the TLS connector. This client is
    ///     // responsible for creating and caching a HttpConnector when given HttpConnectorSettings.
    ///     // This hyper client will create HttpConnectors backed by hyper and the tls_connector.
    ///     let http_client = HyperClientBuilder::new().build(tls_connector);
    ///
    ///     builder.set_http_client(Some(http_client));
    /// }
    ///
    /// let mut builder = SdkConfig::builder();
    /// override_http_client(&mut builder);
    /// let config = builder.build();
    /// # }
    /// ```
    pub fn set_http_client(&mut self, http_client: Option<SharedHttpClient>) -> &mut Self {
        self.http_client = http_client;
        self
    }

    #[doc = docs_for!(use_fips)]
    pub fn use_fips(mut self, use_fips: bool) -> Self {
        self.set_use_fips(Some(use_fips));
        self
    }

    #[doc = docs_for!(use_fips)]
    pub fn set_use_fips(&mut self, use_fips: Option<bool>) -> &mut Self {
        self.use_fips = use_fips;
        self
    }

    #[doc = docs_for!(use_dual_stack)]
    pub fn use_dual_stack(mut self, use_dual_stack: bool) -> Self {
        self.set_use_dual_stack(Some(use_dual_stack));
        self
    }

    #[doc = docs_for!(use_dual_stack)]
    pub fn set_use_dual_stack(&mut self, use_dual_stack: Option<bool>) -> &mut Self {
        self.use_dual_stack = use_dual_stack;
        self
    }

    #[doc = docs_for!(time_source)]
    pub fn time_source(mut self, time_source: impl TimeSource + 'static) -> Self {
        self.set_time_source(Some(SharedTimeSource::new(time_source)));
        self
    }

    #[doc = docs_for!(time_source)]
    pub fn set_time_source(&mut self, time_source: Option<SharedTimeSource>) -> &mut Self {
        self.time_source = time_source;
        self
    }

    #[doc = docs_for!(disable_request_compression)]
    pub fn disable_request_compression(mut self, disable_request_compression: bool) -> Self {
        self.set_disable_request_compression(Some(disable_request_compression));
        self
    }

    #[doc = docs_for!(disable_request_compression)]
    pub fn set_disable_request_compression(
        &mut self,
        disable_request_compression: Option<bool>,
    ) -> &mut Self {
        self.disable_request_compression = disable_request_compression;
        self
    }

    #[doc = docs_for!(request_min_compression_size_bytes)]
    pub fn request_min_compression_size_bytes(
        mut self,
        request_min_compression_size_bytes: u32,
    ) -> Self {
        self.set_request_min_compression_size_bytes(Some(request_min_compression_size_bytes));
        self
    }

    #[doc = docs_for!(request_min_compression_size_bytes)]
    pub fn set_request_min_compression_size_bytes(
        &mut self,
        request_min_compression_size_bytes: Option<u32>,
    ) -> &mut Self {
        self.request_min_compression_size_bytes = request_min_compression_size_bytes;
        self
    }

    /// Sets the [`BehaviorVersion`] for the [`SdkConfig`]
    pub fn behavior_version(mut self, behavior_version: BehaviorVersion) -> Self {
        self.set_behavior_version(Some(behavior_version));
        self
    }

    /// Sets the [`BehaviorVersion`] for the [`SdkConfig`]
    pub fn set_behavior_version(&mut self, behavior_version: Option<BehaviorVersion>) -> &mut Self {
        self.behavior_version = behavior_version;
        self
    }

    /// Sets the service config provider for the [`SdkConfig`].
    ///
    /// This provider is used when creating a service-specific config from an
    /// `SdkConfig` and provides access to config defined in the environment
    /// which would otherwise be inaccessible.
    pub fn service_config(mut self, service_config: impl LoadServiceConfig + 'static) -> Self {
        self.set_service_config(Some(service_config));
        self
    }

    /// Sets the service config provider for the [`SdkConfig`].
    ///
    /// This provider is used when creating a service-specific config from an
    /// `SdkConfig` and provides access to config defined in the environment
    /// which would otherwise be inaccessible.
    pub fn set_service_config(
        &mut self,
        service_config: Option<impl LoadServiceConfig + 'static>,
    ) -> &mut Self {
        self.service_config = service_config.map(|it| Arc::new(it) as Arc<dyn LoadServiceConfig>);
        self
    }

    #[doc = docs_for!(auth_scheme_preference)]
    pub fn auth_scheme_preference(
        mut self,
        auth_scheme_preference: impl Into<AuthSchemePreference>,
    ) -> Self {
        self.set_auth_scheme_preference(Some(auth_scheme_preference));
        self
    }

    #[doc = docs_for!(auth_scheme_preference)]
    pub fn set_auth_scheme_preference(
        &mut self,
        auth_scheme_preference: Option<impl Into<AuthSchemePreference>>,
    ) -> &mut Self {
        self.auth_scheme_preference = auth_scheme_preference.map(|pref| pref.into());
        self
    }

    /// Set the origin of a setting.
    ///
    /// This is used internally to understand how to merge config structs while
    /// respecting precedence of origins.
    pub fn insert_origin(&mut self, setting: &'static str, origin: Origin) {
        self.config_origins.insert(setting, origin);
    }

    /// Build a [`SdkConfig`] from this builder.
    pub fn build(self) -> SdkConfig {
        SdkConfig {
            app_name: self.app_name,
            auth_scheme_preference: self.auth_scheme_preference,
            identity_cache: self.identity_cache,
            credentials_provider: self.credentials_provider,
            token_provider: self.token_provider,
            region: self.region,
            account_id_endpoint_mode: self.account_id_endpoint_mode,
            endpoint_url: self.endpoint_url,
            retry_config: self.retry_config,
            sleep_impl: self.sleep_impl,
            timeout_config: self.timeout_config,
            http_client: self.http_client,
            use_fips: self.use_fips,
            use_dual_stack: self.use_dual_stack,
            time_source: self.time_source,
            behavior_version: self.behavior_version,
            stalled_stream_protection_config: self.stalled_stream_protection_config,
            service_config: self.service_config,
            config_origins: self.config_origins,
            disable_request_compression: self.disable_request_compression,
            request_min_compression_size_bytes: self.request_min_compression_size_bytes,
            request_checksum_calculation: self.request_checksum_calculation,
            response_checksum_validation: self.response_checksum_validation,
        }
    }
}

impl Builder {
    /// Set the [`StalledStreamProtectionConfig`] to configure protection for stalled streams.
    ///
    /// This configures stalled stream protection. When enabled, download streams
    /// that stall (stream no data) for longer than a configured grace period will return an error.
    ///
    /// _Note:_ Stalled stream protection requires both a sleep implementation and a time source
    /// in order to work. When enabling stalled stream protection, make sure to set
    /// - A sleep impl with [Self::sleep_impl] or [Self::set_sleep_impl].
    /// - A time source with [Self::time_source] or [Self::set_time_source].
    ///
    /// # Examples
    /// ```rust
    /// use std::time::Duration;
    /// use aws_types::SdkConfig;
    /// pub use aws_smithy_runtime_api::client::stalled_stream_protection::StalledStreamProtectionConfig;
    ///
    /// let stalled_stream_protection_config = StalledStreamProtectionConfig::enabled()
    ///     .grace_period(Duration::from_secs(1))
    ///     .build();
    /// let config = SdkConfig::builder()
    ///     .stalled_stream_protection(stalled_stream_protection_config)
    ///     .build();
    /// ```
    pub fn stalled_stream_protection(
        mut self,
        stalled_stream_protection_config: StalledStreamProtectionConfig,
    ) -> Self {
        self.set_stalled_stream_protection(Some(stalled_stream_protection_config));
        self
    }

    /// Set the [`StalledStreamProtectionConfig`] to configure protection for stalled streams.
    ///
    /// This configures stalled stream protection. When enabled, download streams
    /// that stall (stream no data) for longer than a configured grace period will return an error.
    ///
    /// By default, streams that transmit less than one byte per-second for five seconds will
    /// be cancelled.
    ///
    /// _Note:_ Stalled stream protection requires both a sleep implementation and a time source
    /// in order to work. When enabling stalled stream protection, make sure to set
    /// - A sleep impl with [Self::sleep_impl] or [Self::set_sleep_impl].
    /// - A time source with [Self::time_source] or [Self::set_time_source].
    ///
    /// # Examples
    /// ```rust
    /// use std::time::Duration;
    /// use aws_types::sdk_config::{SdkConfig, Builder};
    /// pub use aws_smithy_runtime_api::client::stalled_stream_protection::StalledStreamProtectionConfig;
    ///
    /// fn set_stalled_stream_protection(builder: &mut Builder) {
    ///     let stalled_stream_protection_config = StalledStreamProtectionConfig::enabled()
    ///         .grace_period(Duration::from_secs(1))
    ///         .build();
    ///     builder.set_stalled_stream_protection(Some(stalled_stream_protection_config));
    /// }
    ///
    /// let mut builder = SdkConfig::builder();
    /// set_stalled_stream_protection(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_stalled_stream_protection(
        &mut self,
        stalled_stream_protection_config: Option<StalledStreamProtectionConfig>,
    ) -> &mut Self {
        self.stalled_stream_protection_config = stalled_stream_protection_config;
        self
    }
}

impl SdkConfig {
    /// Configured region
    pub fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    /// Configured account ID endpoint mode
    pub fn account_id_endpoint_mode(&self) -> Option<&AccountIdEndpointMode> {
        self.account_id_endpoint_mode.as_ref()
    }

    /// Configured auth scheme preference
    pub fn auth_scheme_preference(&self) -> Option<&AuthSchemePreference> {
        self.auth_scheme_preference.as_ref()
    }

    /// Configured endpoint URL
    pub fn endpoint_url(&self) -> Option<&str> {
        self.endpoint_url.as_deref()
    }

    /// Configured retry config
    pub fn retry_config(&self) -> Option<&RetryConfig> {
        self.retry_config.as_ref()
    }

    /// Configured timeout config
    pub fn timeout_config(&self) -> Option<&TimeoutConfig> {
        self.timeout_config.as_ref()
    }

    /// Configured sleep implementation
    pub fn sleep_impl(&self) -> Option<SharedAsyncSleep> {
        self.sleep_impl.clone()
    }

    /// Configured identity cache
    pub fn identity_cache(&self) -> Option<SharedIdentityCache> {
        self.identity_cache.clone()
    }

    /// Configured credentials provider
    pub fn credentials_provider(&self) -> Option<SharedCredentialsProvider> {
        self.credentials_provider.clone()
    }

    /// Configured bearer auth token provider
    pub fn token_provider(&self) -> Option<SharedTokenProvider> {
        self.token_provider.clone()
    }

    /// Configured time source
    pub fn time_source(&self) -> Option<SharedTimeSource> {
        self.time_source.clone()
    }

    /// Configured app name
    pub fn app_name(&self) -> Option<&AppName> {
        self.app_name.as_ref()
    }

    /// Configured HTTP client
    pub fn http_client(&self) -> Option<SharedHttpClient> {
        self.http_client.clone()
    }

    /// Use FIPS endpoints
    pub fn use_fips(&self) -> Option<bool> {
        self.use_fips
    }

    /// Use dual-stack endpoint
    pub fn use_dual_stack(&self) -> Option<bool> {
        self.use_dual_stack
    }

    /// When true, request compression is disabled.
    pub fn disable_request_compression(&self) -> Option<bool> {
        self.disable_request_compression
    }

    /// Configured checksum request behavior.
    pub fn request_checksum_calculation(&self) -> Option<RequestChecksumCalculation> {
        self.request_checksum_calculation
    }

    /// Configured checksum response behavior.
    pub fn response_checksum_validation(&self) -> Option<ResponseChecksumValidation> {
        self.response_checksum_validation
    }

    /// Configured minimum request compression size.
    pub fn request_min_compression_size_bytes(&self) -> Option<u32> {
        self.request_min_compression_size_bytes
    }

    /// Configured stalled stream protection
    pub fn stalled_stream_protection(&self) -> Option<StalledStreamProtectionConfig> {
        self.stalled_stream_protection_config.clone()
    }

    /// Behavior version configured for this client
    pub fn behavior_version(&self) -> Option<BehaviorVersion> {
        self.behavior_version
    }

    /// Return an immutable reference to the service config provider configured for this client.
    pub fn service_config(&self) -> Option<&dyn LoadServiceConfig> {
        self.service_config.as_deref()
    }

    /// Config builder
    ///
    /// _Important:_ Using the `aws-config` crate to configure the SDK is preferred to invoking this
    /// builder directly. Using this builder directly won't pull in any AWS recommended default
    /// configuration values.
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Convert this [`SdkConfig`] into a [`Builder`] by cloning it first
    pub fn to_builder(&self) -> Builder {
        self.clone().into_builder()
    }

    /// Get the origin of a setting.
    ///
    /// This is used internally to understand how to merge config structs while
    /// respecting precedence of origins.
    pub fn get_origin(&self, setting: &'static str) -> Origin {
        self.config_origins
            .get(setting)
            .cloned()
            .unwrap_or_default()
    }

    /// Convert this [`SdkConfig`] back to a builder to enable modification
    pub fn into_builder(self) -> Builder {
        Builder {
            app_name: self.app_name,
            auth_scheme_preference: self.auth_scheme_preference,
            identity_cache: self.identity_cache,
            credentials_provider: self.credentials_provider,
            token_provider: self.token_provider,
            region: self.region,
            account_id_endpoint_mode: self.account_id_endpoint_mode,
            endpoint_url: self.endpoint_url,
            retry_config: self.retry_config,
            sleep_impl: self.sleep_impl,
            time_source: self.time_source,
            timeout_config: self.timeout_config,
            http_client: self.http_client,
            use_fips: self.use_fips,
            use_dual_stack: self.use_dual_stack,
            behavior_version: self.behavior_version,
            stalled_stream_protection_config: self.stalled_stream_protection_config,
            service_config: self.service_config,
            config_origins: self.config_origins,
            disable_request_compression: self.disable_request_compression,
            request_min_compression_size_bytes: self.request_min_compression_size_bytes,
            request_checksum_calculation: self.request_checksum_calculation,
            response_checksum_validation: self.response_checksum_validation,
        }
    }
}
