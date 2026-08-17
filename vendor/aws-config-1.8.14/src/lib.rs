/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/* Automatically managed default lints */
#![cfg_attr(docsrs, feature(doc_cfg))]
/* End of automatically managed default lints */
#![allow(clippy::derive_partial_eq_without_eq)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rustdoc::missing_crate_level_docs,
    unreachable_pub
)]
// Allow disallowed methods in tests
#![cfg_attr(test, allow(clippy::disallowed_methods))]

//! `aws-config` provides implementations of region and credential resolution.
//!
//! These implementations can be used either via the default chain implementation
//! [`from_env`]/[`ConfigLoader`] or ad-hoc individual credential and region providers.
//!
//! [`ConfigLoader`] can combine different configuration sources into an AWS shared-config:
//! [`SdkConfig`]. `SdkConfig` can be used configure an AWS service client.
//!
//! # Examples
//!
//! Load default SDK configuration:
//! ```no_run
//! use aws_config::BehaviorVersion;
//! mod aws_sdk_dynamodb {
//! #   pub struct Client;
//! #   impl Client {
//! #     pub fn new(config: &aws_types::SdkConfig) -> Self { Client }
//! #   }
//! # }
//! # async fn docs() {
//! let config = aws_config::load_defaults(BehaviorVersion::v2023_11_09()).await;
//! let client = aws_sdk_dynamodb::Client::new(&config);
//! # }
//! ```
//!
//! Load SDK configuration with a region override:
//! ```no_run
//! # mod aws_sdk_dynamodb {
//! #   pub struct Client;
//! #   impl Client {
//! #     pub fn new(config: &aws_types::SdkConfig) -> Self { Client }
//! #   }
//! # }
//! # async fn docs() {
//! # use aws_config::meta::region::RegionProviderChain;
//! let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
//! // Note: requires the `behavior-version-latest` feature enabled
//! let config = aws_config::from_env().region(region_provider).load().await;
//! let client = aws_sdk_dynamodb::Client::new(&config);
//! # }
//! ```
//!
//! Override configuration after construction of `SdkConfig`:
//!
//! ```no_run
//! # use aws_credential_types::provider::ProvideCredentials;
//! # use aws_types::SdkConfig;
//! # mod aws_sdk_dynamodb {
//! #   pub mod config {
//! #     pub struct Builder;
//! #     impl Builder {
//! #       pub fn credentials_provider(
//! #         self,
//! #         credentials_provider: impl aws_credential_types::provider::ProvideCredentials + 'static) -> Self { self }
//! #       pub fn build(self) -> Builder { self }
//! #     }
//! #     impl From<&aws_types::SdkConfig> for Builder {
//! #       fn from(_: &aws_types::SdkConfig) -> Self {
//! #           todo!()
//! #       }
//! #     }
//! #   }
//! #   pub struct Client;
//! #   impl Client {
//! #     pub fn from_conf(conf: config::Builder) -> Self { Client }
//! #     pub fn new(config: &aws_types::SdkConfig) -> Self { Client }
//! #   }
//! # }
//! # async fn docs() {
//! # use aws_config::meta::region::RegionProviderChain;
//! # fn custom_provider(base: &SdkConfig) -> impl ProvideCredentials {
//! #   base.credentials_provider().unwrap().clone()
//! # }
//! let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
//! let custom_credentials_provider = custom_provider(&sdk_config);
//! let dynamo_config = aws_sdk_dynamodb::config::Builder::from(&sdk_config)
//!   .credentials_provider(custom_credentials_provider)
//!   .build();
//! let client = aws_sdk_dynamodb::Client::from_conf(dynamo_config);
//! # }
//! ```

pub use aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;
// Re-export types from aws-types
pub use aws_types::{
    app_name::{AppName, InvalidAppName},
    region::Region,
    SdkConfig,
};
/// Load default sources for all configuration with override support
pub use loader::ConfigLoader;

/// Types for configuring identity caching.
pub mod identity {
    pub use aws_smithy_runtime::client::identity::IdentityCache;
    pub use aws_smithy_runtime::client::identity::LazyCacheBuilder;
}

#[allow(dead_code)]
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

mod http_credential_provider;
mod json_credentials;
#[cfg(test)]
mod test_case;

pub mod credential_process;
pub mod default_provider;
pub mod ecs;
mod env_service_config;
pub mod environment;
pub mod imds;
#[cfg(feature = "credentials-login")]
pub mod login;
pub mod meta;
pub mod profile;
pub mod provider_config;
pub mod retry;
mod sensitive_command;
#[cfg(feature = "sso")]
pub mod sso;
pub mod stalled_stream_protection;
pub mod sts;
pub mod timeout;
pub mod web_identity_token;

/// Create a config loader with the _latest_ defaults.
///
/// This loader will always set [`BehaviorVersion::latest`].
///
/// For more information about default configuration, refer to the AWS SDKs and Tools [shared configuration documentation](https://docs.aws.amazon.com/sdkref/latest/guide/creds-config-files.html).
///
/// # Examples
/// ```no_run
/// # async fn create_config() {
/// let config = aws_config::from_env().region("us-east-1").load().await;
/// # }
/// ```
#[cfg(feature = "behavior-version-latest")]
pub fn from_env() -> ConfigLoader {
    ConfigLoader::default().behavior_version(BehaviorVersion::latest())
}

/// Load default configuration with the _latest_ defaults.
///
/// Convenience wrapper equivalent to `aws_config::load_defaults(BehaviorVersion::latest()).await`
///
/// For more information about default configuration, refer to the AWS SDKs and Tools [shared configuration documentation](https://docs.aws.amazon.com/sdkref/latest/guide/creds-config-files.html).
#[cfg(feature = "behavior-version-latest")]
pub async fn load_from_env() -> SdkConfig {
    from_env().load().await
}

/// Create a config loader with the _latest_ defaults.
#[cfg(not(feature = "behavior-version-latest"))]
#[deprecated(
    note = "Use the `aws_config::defaults` function. If you don't care about future default behavior changes, you can continue to use this function by enabling the `behavior-version-latest` feature. Doing so will make this deprecation notice go away."
)]
pub fn from_env() -> ConfigLoader {
    ConfigLoader::default().behavior_version(BehaviorVersion::latest())
}

/// Load default configuration with the _latest_ defaults.
#[cfg(not(feature = "behavior-version-latest"))]
#[deprecated(
    note = "Use the `aws_config::load_defaults` function. If you don't care about future default behavior changes, you can continue to use this function by enabling the `behavior-version-latest` feature. Doing so will make this deprecation notice go away."
)]
pub async fn load_from_env() -> SdkConfig {
    load_defaults(BehaviorVersion::latest()).await
}

/// Create a config loader with the defaults for the given behavior version.
///
/// For more information about default configuration, refer to the AWS SDKs and Tools [shared configuration documentation](https://docs.aws.amazon.com/sdkref/latest/guide/creds-config-files.html).
///
/// # Examples
/// ```no_run
/// # async fn create_config() {
/// use aws_config::BehaviorVersion;
/// let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
///     .region("us-east-1")
///     .load()
///     .await;
/// # }
/// ```
pub fn defaults(version: BehaviorVersion) -> ConfigLoader {
    ConfigLoader::default().behavior_version(version)
}

/// Load default configuration with the given behavior version.
///
/// Convenience wrapper equivalent to `aws_config::defaults(behavior_version).load().await`
///
/// For more information about default configuration, refer to the AWS SDKs and Tools [shared configuration documentation](https://docs.aws.amazon.com/sdkref/latest/guide/creds-config-files.html).
pub async fn load_defaults(version: BehaviorVersion) -> SdkConfig {
    defaults(version).load().await
}

mod loader {
    use crate::env_service_config::EnvServiceConfig;
    use aws_credential_types::provider::{
        token::{ProvideToken, SharedTokenProvider},
        ProvideCredentials, SharedCredentialsProvider,
    };
    use aws_credential_types::Credentials;
    use aws_smithy_async::rt::sleep::{default_async_sleep, AsyncSleep, SharedAsyncSleep};
    use aws_smithy_async::time::{SharedTimeSource, TimeSource};
    use aws_smithy_runtime::client::identity::IdentityCache;
    use aws_smithy_runtime_api::client::auth::AuthSchemePreference;
    use aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;
    use aws_smithy_runtime_api::client::http::HttpClient;
    use aws_smithy_runtime_api::client::identity::{ResolveCachedIdentity, SharedIdentityCache};
    use aws_smithy_runtime_api::client::stalled_stream_protection::StalledStreamProtectionConfig;
    use aws_smithy_runtime_api::shared::IntoShared;
    use aws_smithy_types::checksum_config::{
        RequestChecksumCalculation, ResponseChecksumValidation,
    };
    use aws_smithy_types::retry::RetryConfig;
    use aws_smithy_types::timeout::TimeoutConfig;
    use aws_types::app_name::AppName;
    use aws_types::docs_for;
    use aws_types::endpoint_config::AccountIdEndpointMode;
    use aws_types::origin::Origin;
    use aws_types::os_shim_internal::{Env, Fs};
    use aws_types::sdk_config::SharedHttpClient;
    use aws_types::SdkConfig;

    use crate::default_provider::{
        account_id_endpoint_mode, app_name, auth_scheme_preference, checksums, credentials,
        disable_request_compression, endpoint_url, ignore_configured_endpoint_urls as ignore_ep,
        region, request_min_compression_size_bytes, retry_config, timeout_config, use_dual_stack,
        use_fips,
    };
    use crate::meta::region::ProvideRegion;
    #[allow(deprecated)]
    use crate::profile::profile_file::ProfileFiles;
    use crate::provider_config::ProviderConfig;

    #[derive(Default, Debug)]
    enum TriStateOption<T> {
        /// No option was set by the user. We can set up the default.
        #[default]
        NotSet,
        /// The option was explicitly unset. Do not set up a default.
        ExplicitlyUnset,
        /// Use the given user provided option.
        Set(T),
    }

    /// Load a cross-service [`SdkConfig`] from the environment
    ///
    /// This builder supports overriding individual components of the generated config. Overriding a component
    /// will skip the standard resolution chain from **for that component**. For example,
    /// if you override the region provider, _even if that provider returns None_, the default region provider
    /// chain will not be used.
    #[derive(Default, Debug)]
    pub struct ConfigLoader {
        app_name: Option<AppName>,
        auth_scheme_preference: Option<AuthSchemePreference>,
        identity_cache: Option<SharedIdentityCache>,
        credentials_provider: TriStateOption<SharedCredentialsProvider>,
        token_provider: Option<SharedTokenProvider>,
        account_id_endpoint_mode: Option<AccountIdEndpointMode>,
        endpoint_url: Option<String>,
        region: Option<Box<dyn ProvideRegion>>,
        retry_config: Option<RetryConfig>,
        sleep: Option<SharedAsyncSleep>,
        timeout_config: Option<TimeoutConfig>,
        provider_config: Option<ProviderConfig>,
        http_client: Option<SharedHttpClient>,
        profile_name_override: Option<String>,
        #[allow(deprecated)]
        profile_files_override: Option<ProfileFiles>,
        use_fips: Option<bool>,
        use_dual_stack: Option<bool>,
        time_source: Option<SharedTimeSource>,
        disable_request_compression: Option<bool>,
        request_min_compression_size_bytes: Option<u32>,
        stalled_stream_protection_config: Option<StalledStreamProtectionConfig>,
        env: Option<Env>,
        fs: Option<Fs>,
        behavior_version: Option<BehaviorVersion>,
        request_checksum_calculation: Option<RequestChecksumCalculation>,
        response_checksum_validation: Option<ResponseChecksumValidation>,
    }

    impl ConfigLoader {
        /// Sets the [`BehaviorVersion`] used to build [`SdkConfig`].
        pub fn behavior_version(mut self, behavior_version: BehaviorVersion) -> Self {
            self.behavior_version = Some(behavior_version);
            self
        }

        /// Override the region used to build [`SdkConfig`].
        ///
        /// # Examples
        /// ```no_run
        /// # async fn create_config() {
        /// use aws_types::region::Region;
        /// let config = aws_config::from_env()
        ///     .region(Region::new("us-east-1"))
        ///     .load().await;
        /// # }
        /// ```
        pub fn region(mut self, region: impl ProvideRegion + 'static) -> Self {
            self.region = Some(Box::new(region));
            self
        }

        /// Override the retry_config used to build [`SdkConfig`].
        ///
        /// # Examples
        /// ```no_run
        /// # async fn create_config() {
        /// use aws_config::retry::RetryConfig;
        ///
        /// let config = aws_config::from_env()
        ///     .retry_config(RetryConfig::standard().with_max_attempts(2))
        ///     .load()
        ///     .await;
        /// # }
        /// ```
        pub fn retry_config(mut self, retry_config: RetryConfig) -> Self {
            self.retry_config = Some(retry_config);
            self
        }

        /// Override the timeout config used to build [`SdkConfig`].
        ///
        /// This will be merged with timeouts coming from the timeout information provider, which
        /// currently includes a default `CONNECT` timeout of `3.1s`.
        ///
        /// If you want to disable timeouts, use [`TimeoutConfig::disabled`]. If you want to disable
        /// a specific timeout, use `TimeoutConfig::set_<type>(None)`.
        ///
        /// **Note: This only sets timeouts for calls to AWS services.** Timeouts for the credentials
        /// provider chain are configured separately.
        ///
        /// # Examples
        /// ```no_run
        /// # use std::time::Duration;
        /// # async fn create_config() {
        /// use aws_config::timeout::TimeoutConfig;
        ///
        /// let config = aws_config::from_env()
        ///    .timeout_config(
        ///        TimeoutConfig::builder()
        ///            .operation_timeout(Duration::from_secs(5))
        ///            .build()
        ///    )
        ///    .load()
        ///    .await;
        /// # }
        /// ```
        pub fn timeout_config(mut self, timeout_config: TimeoutConfig) -> Self {
            self.timeout_config = Some(timeout_config);
            self
        }

        /// Override the sleep implementation for this [`ConfigLoader`].
        ///
        /// The sleep implementation is used to create timeout futures.
        /// You generally won't need to change this unless you're using an async runtime other
        /// than Tokio.
        pub fn sleep_impl(mut self, sleep: impl AsyncSleep + 'static) -> Self {
            // it's possible that we could wrapping an `Arc in an `Arc` and that's OK
            self.sleep = Some(sleep.into_shared());
            self
        }

        /// Set the time source used for tasks like signing requests.
        ///
        /// You generally won't need to change this unless you're compiling for a target
        /// that can't provide a default, such as WASM, or unless you're writing a test against
        /// the client that needs a fixed time.
        pub fn time_source(mut self, time_source: impl TimeSource + 'static) -> Self {
            self.time_source = Some(time_source.into_shared());
            self
        }

        /// Override the [`HttpClient`] for this [`ConfigLoader`].
        ///
        /// The HTTP client will be used for both AWS services and credentials providers.
        ///
        /// If you wish to use a separate HTTP client for credentials providers when creating clients,
        /// then override the HTTP client set with this function on the client-specific `Config`s.
        pub fn http_client(mut self, http_client: impl HttpClient + 'static) -> Self {
            self.http_client = Some(http_client.into_shared());
            self
        }

        #[doc = docs_for!(auth_scheme_preference)]
        ///
        /// # Examples
        /// ```no_run
        /// # use aws_smithy_runtime_api::client::auth::AuthSchemeId;
        /// # async fn create_config() {
        /// let config = aws_config::from_env()
        ///     // Favors a custom auth scheme over the SigV4 auth scheme.
        ///     // Note: This will not result in an error, even if the custom scheme is missing from the resolved auth schemes.
        ///     .auth_scheme_preference([AuthSchemeId::from("custom"), aws_runtime::auth::sigv4::SCHEME_ID])
        ///     .load()
        ///     .await;
        /// # }
        /// ```
        pub fn auth_scheme_preference(
            mut self,
            auth_scheme_preference: impl Into<AuthSchemePreference>,
        ) -> Self {
            self.auth_scheme_preference = Some(auth_scheme_preference.into());
            self
        }

        /// Override the identity cache used to build [`SdkConfig`].
        ///
        /// The identity cache caches AWS credentials and SSO tokens. By default, a lazy cache is used
        /// that will load credentials upon first request, cache them, and then reload them during
        /// another request when they are close to expiring.
        ///
        /// # Examples
        ///
        /// Change a setting on the default lazy caching implementation:
        /// ```no_run
        /// use aws_config::identity::IdentityCache;
        /// use std::time::Duration;
        ///
        /// # async fn create_config() {
        /// let config = aws_config::from_env()
        ///     .identity_cache(
        ///         IdentityCache::lazy()
        ///             // Change the load timeout to 10 seconds.
        ///             // Note: there are other timeouts that could trigger if the load timeout is too long.
        ///             .load_timeout(Duration::from_secs(10))
        ///             .build()
        ///     )
        ///     .load()
        ///     .await;
        /// # }
        /// ```
        pub fn identity_cache(
            mut self,
            identity_cache: impl ResolveCachedIdentity + 'static,
        ) -> Self {
            self.identity_cache = Some(identity_cache.into_shared());
            self
        }

        /// Override the credentials provider used to build [`SdkConfig`].
        ///
        /// # Examples
        ///
        /// Override the credentials provider but load the default value for region:
        /// ```no_run
        /// # use aws_credential_types::Credentials;
        /// # fn create_my_credential_provider() -> Credentials {
        /// #     Credentials::new("example", "example", None, None, "example")
        /// # }
        /// # async fn create_config() {
        /// let config = aws_config::from_env()
        ///     .credentials_provider(create_my_credential_provider())
        ///     .load()
        ///     .await;
        /// # }
        /// ```
        pub fn credentials_provider(
            mut self,
            credentials_provider: impl ProvideCredentials + 'static,
        ) -> Self {
            self.credentials_provider =
                TriStateOption::Set(SharedCredentialsProvider::new(credentials_provider));
            self
        }

        /// Don't use credentials to sign requests.
        ///
        /// Turning off signing with credentials is necessary in some cases, such as using
        /// anonymous auth for S3, calling operations in STS that don't require a signature,
        /// or using token-based auth.
        ///
        /// **Note**: For tests, e.g. with a service like DynamoDB Local, this is **not** what you
        /// want. If credentials are disabled, requests cannot be signed. For these use cases, use
        /// [`test_credentials`](Self::test_credentials).
        ///
        /// # Examples
        ///
        /// Turn off credentials in order to call a service without signing:
        /// ```no_run
        /// # async fn create_config() {
        /// let config = aws_config::from_env()
        ///     .no_credentials()
        ///     .load()
        ///     .await;
        /// # }
        /// ```
        pub fn no_credentials(mut self) -> Self {
            self.credentials_provider = TriStateOption::ExplicitlyUnset;
            self
        }

        /// Set test credentials for use when signing requests
        pub fn test_credentials(self) -> Self {
            #[allow(unused_mut)]
            let mut ret = self.credentials_provider(Credentials::for_tests());
            #[cfg(feature = "sso")]
            {
                use aws_smithy_runtime_api::client::identity::http::Token;
                ret = ret.token_provider(Token::for_tests());
            }
            ret
        }

        /// Ignore any environment variables on the host during config resolution
        ///
        /// This allows for testing in a reproducible environment that ensures any
        /// environment variables from the host do not influence environment variable
        /// resolution.
        pub fn empty_test_environment(mut self) -> Self {
            self.env = Some(Env::from_slice(&[]));
            self
        }

        /// Override the access token provider used to build [`SdkConfig`].
        ///
        /// # Examples
        ///
        /// Override the token provider but load the default value for region:
        /// ```no_run
        /// # use aws_credential_types::Token;
        /// # fn create_my_token_provider() -> Token {
        /// #     Token::new("example", None)
        /// # }
        /// # async fn create_config() {
        /// let config = aws_config::from_env()
        ///     .token_provider(create_my_token_provider())
        ///     .load()
        ///     .await;
        /// # }
        /// ```
        pub fn token_provider(mut self, token_provider: impl ProvideToken + 'static) -> Self {
            self.token_provider = Some(SharedTokenProvider::new(token_provider));
            self
        }

        /// Override the name of the app used to build [`SdkConfig`].
        ///
        /// This _optional_ name is used to identify the application in the user agent header that
        /// gets sent along with requests.
        ///
        /// The app name is selected from an ordered list of sources:
        /// 1. This override.
        /// 2. The value of the `AWS_SDK_UA_APP_ID` environment variable.
        /// 3. Profile files from the key `sdk_ua_app_id`
        ///
        /// If none of those sources are set the value is `None` and it is not added to the user agent header.
        ///
        /// # Examples
        /// ```no_run
        /// # async fn create_config() {
        /// use aws_config::AppName;
        /// let config = aws_config::from_env()
        ///     .app_name(AppName::new("my-app-name").expect("valid app name"))
        ///     .load().await;
        /// # }
        /// ```
        pub fn app_name(mut self, app_name: AppName) -> Self {
            self.app_name = Some(app_name);
            self
        }

        /// Provides the ability to programmatically override the profile files that get loaded by the SDK.
        ///
        /// The [`Default`] for `ProfileFiles` includes the default SDK config and credential files located in
        /// `~/.aws/config` and `~/.aws/credentials` respectively.
        ///
        /// Any number of config and credential files may be added to the `ProfileFiles` file set, with the
        /// only requirement being that there is at least one of each. Profile file locations will produce an
        /// error if they don't exist, but the default config/credentials files paths are exempt from this validation.
        ///
        /// # Example: Using a custom profile file path
        ///
        /// ```no_run
        /// use aws_config::profile::{ProfileFileCredentialsProvider, ProfileFileRegionProvider};
        /// use aws_config::profile::profile_file::{ProfileFiles, ProfileFileKind};
        ///
        /// # async fn example() {
        /// let profile_files = ProfileFiles::builder()
        ///     .with_file(ProfileFileKind::Credentials, "some/path/to/credentials-file")
        ///     .build();
        /// let sdk_config = aws_config::from_env()
        ///     .profile_files(profile_files)
        ///     .load()
        ///     .await;
        /// # }
        #[allow(deprecated)]
        pub fn profile_files(mut self, profile_files: ProfileFiles) -> Self {
            self.profile_files_override = Some(profile_files);
            self
        }

        /// Override the profile name used by configuration providers
        ///
        /// Profile name is selected from an ordered list of sources:
        /// 1. This override.
        /// 2. The value of the `AWS_PROFILE` environment variable.
        /// 3. `default`
        ///
        /// Each AWS profile has a name. For example, in the file below, the profiles are named
        /// `dev`, `prod` and `staging`:
        /// ```ini
        /// [dev]
        /// ec2_metadata_service_endpoint = http://my-custom-endpoint:444
        ///
        /// [staging]
        /// ec2_metadata_service_endpoint = http://my-custom-endpoint:444
        ///
        /// [prod]
        /// ec2_metadata_service_endpoint = http://my-custom-endpoint:444
        /// ```
        ///
        /// See [Named profiles](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-profiles.html)
        /// for more information about naming profiles.
        ///
        /// # Example: Using a custom profile name
        ///
        /// ```no_run
        /// use aws_config::profile::{ProfileFileCredentialsProvider, ProfileFileRegionProvider};
        ///
        /// # async fn example() {
        /// let sdk_config = aws_config::from_env()
        ///     .profile_name("prod")
        ///     .load()
        ///     .await;
        /// # }
        pub fn profile_name(mut self, profile_name: impl Into<String>) -> Self {
            self.profile_name_override = Some(profile_name.into());
            self
        }

        #[doc = docs_for!(account_id_endpoint_mode)]
        pub fn account_id_endpoint_mode(
            mut self,
            account_id_endpoint_mode: AccountIdEndpointMode,
        ) -> Self {
            self.account_id_endpoint_mode = Some(account_id_endpoint_mode);
            self
        }

        /// Override the endpoint URL used for **all** AWS services.
        ///
        /// This method will override the endpoint URL used for **all** AWS services. This primarily
        /// exists to set a static endpoint for tools like `LocalStack`. When sending requests to
        /// production AWS services, this method should only be used for service-specific behavior.
        ///
        /// When this method is used, the [`Region`](aws_types::region::Region) is only used for signing;
        /// It is **not** used to route the request.
        ///
        /// # Examples
        ///
        /// Use a static endpoint for all services
        /// ```no_run
        /// # async fn create_config() {
        /// let sdk_config = aws_config::from_env()
        ///     .endpoint_url("http://localhost:1234")
        ///     .load()
        ///     .await;
        /// # }
        pub fn endpoint_url(mut self, endpoint_url: impl Into<String>) -> Self {
            self.endpoint_url = Some(endpoint_url.into());
            self
        }

        #[doc = docs_for!(use_fips)]
        pub fn use_fips(mut self, use_fips: bool) -> Self {
            self.use_fips = Some(use_fips);
            self
        }

        #[doc = docs_for!(use_dual_stack)]
        pub fn use_dual_stack(mut self, use_dual_stack: bool) -> Self {
            self.use_dual_stack = Some(use_dual_stack);
            self
        }

        #[doc = docs_for!(disable_request_compression)]
        pub fn disable_request_compression(mut self, disable_request_compression: bool) -> Self {
            self.disable_request_compression = Some(disable_request_compression);
            self
        }

        #[doc = docs_for!(request_min_compression_size_bytes)]
        pub fn request_min_compression_size_bytes(mut self, size: u32) -> Self {
            self.request_min_compression_size_bytes = Some(size);
            self
        }

        /// Override the [`StalledStreamProtectionConfig`] used to build [`SdkConfig`].
        ///
        /// This configures stalled stream protection. When enabled, download streams
        /// that stop (stream no data) for longer than a configured grace period will return an error.
        ///
        /// By default, streams that transmit less than one byte per-second for five seconds will
        /// be cancelled.
        ///
        /// _Note_: When an override is provided, the default implementation is replaced.
        ///
        /// # Examples
        /// ```no_run
        /// # async fn create_config() {
        /// use aws_config::stalled_stream_protection::StalledStreamProtectionConfig;
        /// use std::time::Duration;
        /// let config = aws_config::from_env()
        ///     .stalled_stream_protection(
        ///         StalledStreamProtectionConfig::enabled()
        ///             .grace_period(Duration::from_secs(1))
        ///             .build()
        ///     )
        ///     .load()
        ///     .await;
        /// # }
        /// ```
        pub fn stalled_stream_protection(
            mut self,
            stalled_stream_protection_config: StalledStreamProtectionConfig,
        ) -> Self {
            self.stalled_stream_protection_config = Some(stalled_stream_protection_config);
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
            self.request_checksum_calculation = Some(request_checksum_calculation);
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
            self.response_checksum_validation = Some(response_checksum_validation);
            self
        }

        /// Load the default configuration chain
        ///
        /// If fields have been overridden during builder construction, the override values will be used.
        ///
        /// Otherwise, the default values for each field will be provided.
        ///
        /// NOTE: When an override is provided, the default implementation is **not** used as a fallback.
        /// This means that if you provide a region provider that does not return a region, no region will
        /// be set in the resulting [`SdkConfig`].
        pub async fn load(self) -> SdkConfig {
            let time_source = self.time_source.unwrap_or_default();

            let sleep_impl = if self.sleep.is_some() {
                self.sleep
            } else {
                if default_async_sleep().is_none() {
                    tracing::warn!(
                        "An implementation of AsyncSleep was requested by calling default_async_sleep \
                         but no default was set.
                         This happened when ConfigLoader::load was called during Config construction. \
                         You can fix this by setting a sleep_impl on the ConfigLoader before calling \
                         load or by enabling the rt-tokio feature"
                    );
                }
                default_async_sleep()
            };

            let conf = self
                .provider_config
                .unwrap_or_else(|| {
                    let mut config = ProviderConfig::init(time_source.clone(), sleep_impl.clone())
                        .with_fs(self.fs.unwrap_or_default())
                        .with_env(self.env.unwrap_or_default());
                    if let Some(http_client) = self.http_client.clone() {
                        config = config.with_http_client(http_client);
                    }
                    config
                })
                .with_profile_config(self.profile_files_override, self.profile_name_override);

            let use_fips = if let Some(use_fips) = self.use_fips {
                Some(use_fips)
            } else {
                use_fips::use_fips_provider(&conf).await
            };

            let use_dual_stack = if let Some(use_dual_stack) = self.use_dual_stack {
                Some(use_dual_stack)
            } else {
                use_dual_stack::use_dual_stack_provider(&conf).await
            };

            let conf = conf
                .with_use_fips(use_fips)
                .with_use_dual_stack(use_dual_stack);

            let region = if let Some(provider) = self.region {
                provider.region().await
            } else {
                region::Builder::default()
                    .configure(&conf)
                    .build()
                    .region()
                    .await
            };
            let conf = conf.with_region(region.clone());

            let retry_config = if let Some(retry_config) = self.retry_config {
                retry_config
            } else {
                retry_config::default_provider()
                    .configure(&conf)
                    .retry_config()
                    .await
            };

            let app_name = if self.app_name.is_some() {
                self.app_name
            } else {
                app_name::default_provider()
                    .configure(&conf)
                    .app_name()
                    .await
            };

            let disable_request_compression = if self.disable_request_compression.is_some() {
                self.disable_request_compression
            } else {
                disable_request_compression::disable_request_compression_provider(&conf).await
            };

            let request_min_compression_size_bytes =
                if self.request_min_compression_size_bytes.is_some() {
                    self.request_min_compression_size_bytes
                } else {
                    request_min_compression_size_bytes::request_min_compression_size_bytes_provider(
                        &conf,
                    )
                    .await
                };

            let base_config = timeout_config::default_provider()
                .configure(&conf)
                .timeout_config()
                .await;
            let mut timeout_config = self
                .timeout_config
                .unwrap_or_else(|| TimeoutConfig::builder().build());
            timeout_config.take_defaults_from(&base_config);

            let credentials_provider = match self.credentials_provider {
                TriStateOption::Set(provider) => Some(provider),
                TriStateOption::NotSet => {
                    let mut builder =
                        credentials::DefaultCredentialsChain::builder().configure(conf.clone());
                    builder.set_region(region.clone());
                    Some(SharedCredentialsProvider::new(builder.build().await))
                }
                TriStateOption::ExplicitlyUnset => None,
            };

            let profiles = conf.profile().await;
            let service_config = EnvServiceConfig {
                env: conf.env(),
                env_config_sections: profiles.cloned().unwrap_or_default(),
            };
            let mut builder = SdkConfig::builder()
                .region(region.clone())
                .retry_config(retry_config)
                .timeout_config(timeout_config)
                .time_source(time_source)
                .service_config(service_config);

            // If an endpoint URL is set programmatically, then our work is done.
            let endpoint_url = if self.endpoint_url.is_some() {
                builder.insert_origin("endpoint_url", Origin::shared_config());
                self.endpoint_url
            } else {
                // Otherwise, check to see if we should ignore EP URLs set in the environment.
                let ignore_configured_endpoint_urls =
                    ignore_ep::ignore_configured_endpoint_urls_provider(&conf)
                        .await
                        .unwrap_or_default();

                if ignore_configured_endpoint_urls {
                    // If yes, log a trace and return `None`.
                    tracing::trace!(
                        "`ignore_configured_endpoint_urls` is set, any endpoint URLs configured in the environment will be ignored. \
                        NOTE: Endpoint URLs set programmatically WILL still be respected"
                    );
                    None
                } else {
                    // Otherwise, attempt to resolve one.
                    let (v, origin) = endpoint_url::endpoint_url_provider_with_origin(&conf).await;
                    builder.insert_origin("endpoint_url", origin);
                    v
                }
            };

            let token_provider = match self.token_provider {
                Some(provider) => {
                    builder.insert_origin("token_provider", Origin::shared_config());
                    Some(provider)
                }
                None => {
                    #[cfg(feature = "sso")]
                    {
                        let mut builder =
                            crate::default_provider::token::DefaultTokenChain::builder()
                                .configure(conf.clone());
                        builder.set_region(region);
                        Some(SharedTokenProvider::new(builder.build().await))
                    }
                    #[cfg(not(feature = "sso"))]
                    {
                        None
                    }
                    // Not setting `Origin` in this arm, and that's good for now as long as we know
                    // it's not programmatically set in the shared config.
                    // We can consider adding `Origin::Default` if needed.
                }
            };

            builder.set_endpoint_url(endpoint_url);
            builder.set_behavior_version(self.behavior_version);
            builder.set_http_client(self.http_client);
            builder.set_app_name(app_name);

            let identity_cache = match self.identity_cache {
                None => match self.behavior_version {
                    #[allow(deprecated)]
                    Some(bv) if bv.is_at_least(BehaviorVersion::v2024_03_28()) => {
                        Some(IdentityCache::lazy().build())
                    }
                    _ => None,
                },
                Some(user_cache) => Some(user_cache),
            };

            let request_checksum_calculation =
                if let Some(request_checksum_calculation) = self.request_checksum_calculation {
                    Some(request_checksum_calculation)
                } else {
                    checksums::request_checksum_calculation_provider(&conf).await
                };

            let response_checksum_validation =
                if let Some(response_checksum_validation) = self.response_checksum_validation {
                    Some(response_checksum_validation)
                } else {
                    checksums::response_checksum_validation_provider(&conf).await
                };

            let account_id_endpoint_mode =
                if let Some(acccount_id_endpoint_mode) = self.account_id_endpoint_mode {
                    Some(acccount_id_endpoint_mode)
                } else {
                    account_id_endpoint_mode::account_id_endpoint_mode_provider(&conf).await
                };

            let auth_scheme_preference =
                if let Some(auth_scheme_preference) = self.auth_scheme_preference {
                    builder.insert_origin("auth_scheme_preference", Origin::shared_config());
                    Some(auth_scheme_preference)
                } else {
                    auth_scheme_preference::auth_scheme_preference_provider(&conf).await
                    // Not setting `Origin` in this arm, and that's good for now as long as we know
                    // it's not programmatically set in the shared config.
                };

            builder.set_request_checksum_calculation(request_checksum_calculation);
            builder.set_response_checksum_validation(response_checksum_validation);
            builder.set_identity_cache(identity_cache);
            builder.set_credentials_provider(credentials_provider);
            builder.set_token_provider(token_provider);
            builder.set_sleep_impl(sleep_impl);
            builder.set_use_fips(use_fips);
            builder.set_use_dual_stack(use_dual_stack);
            builder.set_disable_request_compression(disable_request_compression);
            builder.set_request_min_compression_size_bytes(request_min_compression_size_bytes);
            builder.set_stalled_stream_protection(self.stalled_stream_protection_config);
            builder.set_account_id_endpoint_mode(account_id_endpoint_mode);
            builder.set_auth_scheme_preference(auth_scheme_preference);
            builder.build()
        }
    }

    #[cfg(test)]
    impl ConfigLoader {
        pub(crate) fn env(mut self, env: Env) -> Self {
            self.env = Some(env);
            self
        }

        pub(crate) fn fs(mut self, fs: Fs) -> Self {
            self.fs = Some(fs);
            self
        }
    }

    #[cfg(test)]
    mod test {
        #[allow(deprecated)]
        use crate::profile::profile_file::{ProfileFileKind, ProfileFiles};
        use crate::test_case::{no_traffic_client, InstantSleep};
        use crate::BehaviorVersion;
        use crate::{defaults, ConfigLoader};
        use aws_credential_types::provider::ProvideCredentials;
        use aws_smithy_async::rt::sleep::TokioSleep;
        use aws_smithy_http_client::test_util::{infallible_client_fn, NeverClient};
        use aws_smithy_runtime::test_util::capture_test_logs::capture_test_logs;
        use aws_types::app_name::AppName;
        use aws_types::origin::Origin;
        use aws_types::os_shim_internal::{Env, Fs};
        use aws_types::sdk_config::{RequestChecksumCalculation, ResponseChecksumValidation};
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        #[tokio::test]
        async fn provider_config_used() {
            let (_guard, logs_rx) = capture_test_logs();
            let env = Env::from_slice(&[
                ("AWS_MAX_ATTEMPTS", "10"),
                ("AWS_REGION", "us-west-4"),
                ("AWS_ACCESS_KEY_ID", "akid"),
                ("AWS_SECRET_ACCESS_KEY", "secret"),
            ]);
            let fs =
                Fs::from_slice(&[("test_config", "[profile custom]\nsdk-ua-app-id = correct")]);
            let loader = defaults(BehaviorVersion::latest())
                .sleep_impl(TokioSleep::new())
                .env(env)
                .fs(fs)
                .http_client(NeverClient::new())
                .profile_name("custom")
                .profile_files(
                    #[allow(deprecated)]
                    ProfileFiles::builder()
                        .with_file(
                            #[allow(deprecated)]
                            ProfileFileKind::Config,
                            "test_config",
                        )
                        .build(),
                )
                .load()
                .await;
            assert_eq!(10, loader.retry_config().unwrap().max_attempts());
            assert_eq!("us-west-4", loader.region().unwrap().as_ref());
            assert_eq!(
                "akid",
                loader
                    .credentials_provider()
                    .unwrap()
                    .provide_credentials()
                    .await
                    .unwrap()
                    .access_key_id(),
            );
            assert_eq!(Some(&AppName::new("correct").unwrap()), loader.app_name());

            let num_config_loader_logs = logs_rx.contents()
                .lines()
                // The logger uses fancy formatting, so we have to account for that.
                .filter(|l| l.contains("config file loaded \u{1b}[3mpath\u{1b}[0m\u{1b}[2m=\u{1b}[0mSome(\"test_config\") \u{1b}[3msize\u{1b}[0m\u{1b}[2m=\u{1b}"))
                .count();

            match num_config_loader_logs {
                0 => panic!("no config file logs found!"),
                1 => (),
                more => panic!("the config file was parsed more than once! (parsed {more})",),
            };
        }

        fn base_conf() -> ConfigLoader {
            defaults(BehaviorVersion::latest())
                .sleep_impl(InstantSleep)
                .http_client(no_traffic_client())
        }

        #[tokio::test]
        async fn test_origin_programmatic() {
            let _ = tracing_subscriber::fmt::try_init();
            let loader = base_conf()
                .test_credentials()
                .profile_name("custom")
                .profile_files(
                    #[allow(deprecated)]
                    ProfileFiles::builder()
                        .with_contents(
                            #[allow(deprecated)]
                            ProfileFileKind::Config,
                            "[profile custom]\nendpoint_url = http://localhost:8989",
                        )
                        .build(),
                )
                .endpoint_url("http://localhost:1111")
                .load()
                .await;
            assert_eq!(Origin::shared_config(), loader.get_origin("endpoint_url"));
        }

        #[tokio::test]
        async fn test_origin_env() {
            let _ = tracing_subscriber::fmt::try_init();
            let env = Env::from_slice(&[("AWS_ENDPOINT_URL", "http://localhost:7878")]);
            let loader = base_conf()
                .test_credentials()
                .env(env)
                .profile_name("custom")
                .profile_files(
                    #[allow(deprecated)]
                    ProfileFiles::builder()
                        .with_contents(
                            #[allow(deprecated)]
                            ProfileFileKind::Config,
                            "[profile custom]\nendpoint_url = http://localhost:8989",
                        )
                        .build(),
                )
                .load()
                .await;
            assert_eq!(
                Origin::shared_environment_variable(),
                loader.get_origin("endpoint_url")
            );
        }

        #[tokio::test]
        async fn test_origin_fs() {
            let _ = tracing_subscriber::fmt::try_init();
            let loader = base_conf()
                .test_credentials()
                .profile_name("custom")
                .profile_files(
                    #[allow(deprecated)]
                    ProfileFiles::builder()
                        .with_contents(
                            #[allow(deprecated)]
                            ProfileFileKind::Config,
                            "[profile custom]\nendpoint_url = http://localhost:8989",
                        )
                        .build(),
                )
                .load()
                .await;
            assert_eq!(
                Origin::shared_profile_file(),
                loader.get_origin("endpoint_url")
            );
        }

        #[tokio::test]
        async fn load_use_fips() {
            let conf = base_conf().use_fips(true).load().await;
            assert_eq!(Some(true), conf.use_fips());
        }

        #[tokio::test]
        async fn load_dual_stack() {
            let conf = base_conf().use_dual_stack(false).load().await;
            assert_eq!(Some(false), conf.use_dual_stack());

            let conf = base_conf().load().await;
            assert_eq!(None, conf.use_dual_stack());
        }

        #[tokio::test]
        async fn load_disable_request_compression() {
            let conf = base_conf().disable_request_compression(true).load().await;
            assert_eq!(Some(true), conf.disable_request_compression());

            let conf = base_conf().load().await;
            assert_eq!(None, conf.disable_request_compression());
        }

        #[tokio::test]
        async fn load_request_min_compression_size_bytes() {
            let conf = base_conf()
                .request_min_compression_size_bytes(99)
                .load()
                .await;
            assert_eq!(Some(99), conf.request_min_compression_size_bytes());

            let conf = base_conf().load().await;
            assert_eq!(None, conf.request_min_compression_size_bytes());
        }

        #[tokio::test]
        async fn app_name() {
            let app_name = AppName::new("my-app-name").unwrap();
            let conf = base_conf().app_name(app_name.clone()).load().await;
            assert_eq!(Some(&app_name), conf.app_name());
        }

        #[tokio::test]
        async fn request_checksum_calculation() {
            let conf = base_conf()
                .request_checksum_calculation(RequestChecksumCalculation::WhenRequired)
                .load()
                .await;
            assert_eq!(
                Some(RequestChecksumCalculation::WhenRequired),
                conf.request_checksum_calculation()
            );
        }

        #[tokio::test]
        async fn response_checksum_validation() {
            let conf = base_conf()
                .response_checksum_validation(ResponseChecksumValidation::WhenRequired)
                .load()
                .await;
            assert_eq!(
                Some(ResponseChecksumValidation::WhenRequired),
                conf.response_checksum_validation()
            );
        }

        #[cfg(feature = "default-https-client")]
        #[tokio::test]
        async fn disable_default_credentials() {
            let config = defaults(BehaviorVersion::latest())
                .no_credentials()
                .load()
                .await;
            assert!(config.credentials_provider().is_none());
        }

        #[cfg(feature = "default-https-client")]
        #[tokio::test]
        async fn identity_cache_defaulted() {
            let config = defaults(BehaviorVersion::latest()).load().await;

            assert!(config.identity_cache().is_some());
        }

        #[cfg(feature = "default-https-client")]
        #[allow(deprecated)]
        #[tokio::test]
        async fn identity_cache_old_behavior_version() {
            let config = defaults(BehaviorVersion::v2023_11_09()).load().await;

            assert!(config.identity_cache().is_none());
        }

        #[tokio::test]
        async fn connector_is_shared() {
            let num_requests = Arc::new(AtomicUsize::new(0));
            let movable = num_requests.clone();
            let http_client = infallible_client_fn(move |_req| {
                movable.fetch_add(1, Ordering::Relaxed);
                http::Response::new("ok!")
            });
            let config = defaults(BehaviorVersion::latest())
                .fs(Fs::from_slice(&[]))
                .env(Env::from_slice(&[]))
                .http_client(http_client.clone())
                .load()
                .await;
            config
                .credentials_provider()
                .unwrap()
                .provide_credentials()
                .await
                .expect_err("did not expect credentials to be loaded—no traffic is allowed");
            let num_requests = num_requests.load(Ordering::Relaxed);
            assert!(num_requests > 0, "{}", num_requests);
        }

        #[tokio::test]
        async fn endpoint_urls_may_be_ignored_from_env() {
            let fs = Fs::from_slice(&[(
                "test_config",
                "[profile custom]\nendpoint_url = http://profile",
            )]);
            let env = Env::from_slice(&[("AWS_IGNORE_CONFIGURED_ENDPOINT_URLS", "true")]);

            let conf = base_conf().use_dual_stack(false).load().await;
            assert_eq!(Some(false), conf.use_dual_stack());

            let conf = base_conf().load().await;
            assert_eq!(None, conf.use_dual_stack());

            // Check that we get nothing back because the env said we should ignore endpoints
            let config = base_conf()
                .fs(fs.clone())
                .env(env)
                .profile_name("custom")
                .profile_files(
                    #[allow(deprecated)]
                    ProfileFiles::builder()
                        .with_file(
                            #[allow(deprecated)]
                            ProfileFileKind::Config,
                            "test_config",
                        )
                        .build(),
                )
                .load()
                .await;
            assert_eq!(None, config.endpoint_url());

            // Check that without the env, we DO get something back
            let config = base_conf()
                .fs(fs)
                .profile_name("custom")
                .profile_files(
                    #[allow(deprecated)]
                    ProfileFiles::builder()
                        .with_file(
                            #[allow(deprecated)]
                            ProfileFileKind::Config,
                            "test_config",
                        )
                        .build(),
                )
                .load()
                .await;
            assert_eq!(Some("http://profile"), config.endpoint_url());
        }

        #[tokio::test]
        async fn endpoint_urls_may_be_ignored_from_profile() {
            let fs = Fs::from_slice(&[(
                "test_config",
                "[profile custom]\nignore_configured_endpoint_urls = true",
            )]);
            let env = Env::from_slice(&[("AWS_ENDPOINT_URL", "http://environment")]);

            // Check that we get nothing back because the profile said we should ignore endpoints
            let config = base_conf()
                .fs(fs)
                .env(env.clone())
                .profile_name("custom")
                .profile_files(
                    #[allow(deprecated)]
                    ProfileFiles::builder()
                        .with_file(
                            #[allow(deprecated)]
                            ProfileFileKind::Config,
                            "test_config",
                        )
                        .build(),
                )
                .load()
                .await;
            assert_eq!(None, config.endpoint_url());

            // Check that without the profile, we DO get something back
            let config = base_conf().env(env).load().await;
            assert_eq!(Some("http://environment"), config.endpoint_url());
        }

        #[tokio::test]
        async fn programmatic_endpoint_urls_may_not_be_ignored() {
            let fs = Fs::from_slice(&[(
                "test_config",
                "[profile custom]\nignore_configured_endpoint_urls = true",
            )]);
            let env = Env::from_slice(&[("AWS_IGNORE_CONFIGURED_ENDPOINT_URLS", "true")]);

            // Check that we get something back because we explicitly set the loader's endpoint URL
            let config = base_conf()
                .fs(fs)
                .env(env)
                .endpoint_url("http://localhost")
                .profile_name("custom")
                .profile_files(
                    #[allow(deprecated)]
                    ProfileFiles::builder()
                        .with_file(
                            #[allow(deprecated)]
                            ProfileFileKind::Config,
                            "test_config",
                        )
                        .build(),
                )
                .load()
                .await;
            assert_eq!(Some("http://localhost"), config.endpoint_url());
        }
    }
}
