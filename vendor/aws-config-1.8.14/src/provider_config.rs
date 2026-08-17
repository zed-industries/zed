/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Configuration Options for Credential Providers

use crate::env_service_config::EnvServiceConfig;
use crate::profile;
#[allow(deprecated)]
use crate::profile::profile_file::ProfileFiles;
use crate::profile::{ProfileFileLoadError, ProfileSet};
use aws_smithy_async::rt::sleep::{default_async_sleep, AsyncSleep, SharedAsyncSleep};
use aws_smithy_async::time::{SharedTimeSource, TimeSource};
use aws_smithy_runtime_api::client::http::HttpClient;
use aws_smithy_runtime_api::shared::IntoShared;
use aws_smithy_types::error::display::DisplayErrorContext;
use aws_smithy_types::retry::RetryConfig;
use aws_types::os_shim_internal::{Env, Fs};
use aws_types::region::Region;
use aws_types::sdk_config::SharedHttpClient;
use aws_types::SdkConfig;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::sync::OnceCell;

/// Configuration options for Credential Providers
///
/// Most credential providers builders offer a `configure` method which applies general provider configuration
/// options.
///
/// To use a region from the default region provider chain use [`ProviderConfig::with_default_region`].
/// Otherwise, use [`ProviderConfig::without_region`]. Note that some credentials providers require a region
/// to be explicitly set.
#[derive(Clone)]
pub struct ProviderConfig {
    env: Env,
    fs: Fs,
    time_source: SharedTimeSource,
    http_client: Option<SharedHttpClient>,
    retry_config: Option<RetryConfig>,
    sleep_impl: Option<SharedAsyncSleep>,
    region: Option<Region>,
    use_fips: Option<bool>,
    use_dual_stack: Option<bool>,
    /// An AWS profile created from `ProfileFiles` and a `profile_name`
    parsed_profile: Arc<OnceCell<Result<ProfileSet, ProfileFileLoadError>>>,
    /// A list of [std::path::Path]s to profile files
    #[allow(deprecated)]
    profile_files: ProfileFiles,
    /// An override to use when constructing a `ProfileSet`
    profile_name_override: Option<Cow<'static, str>>,
}

impl Debug for ProviderConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderConfig")
            .field("env", &self.env)
            .field("fs", &self.fs)
            .field("time_source", &self.time_source)
            .field("http_client", &self.http_client)
            .field("retry_config", &self.retry_config)
            .field("sleep_impl", &self.sleep_impl)
            .field("region", &self.region)
            .field("use_fips", &self.use_fips)
            .field("use_dual_stack", &self.use_dual_stack)
            .field("profile_name_override", &self.profile_name_override)
            .finish()
    }
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            env: Env::default(),
            fs: Fs::default(),
            time_source: SharedTimeSource::default(),
            http_client: None,
            retry_config: None,
            sleep_impl: default_async_sleep(),
            region: None,
            use_fips: None,
            use_dual_stack: None,
            parsed_profile: Default::default(),
            #[allow(deprecated)]
            profile_files: ProfileFiles::default(),
            profile_name_override: None,
        }
    }
}

#[cfg(test)]
impl ProviderConfig {
    /// ProviderConfig with all configuration removed
    ///
    /// Unlike [`ProviderConfig::empty`] where `env` and `fs` will use their non-mocked implementations,
    /// this method will use an empty mock environment and an empty mock file system.
    pub fn no_configuration() -> Self {
        use aws_smithy_async::time::StaticTimeSource;
        use std::collections::HashMap;
        use std::time::UNIX_EPOCH;
        let fs = Fs::from_raw_map(HashMap::new());
        let env = Env::from_slice(&[]);
        Self {
            parsed_profile: Default::default(),
            #[allow(deprecated)]
            profile_files: ProfileFiles::default(),
            env,
            fs,
            time_source: SharedTimeSource::new(StaticTimeSource::new(UNIX_EPOCH)),
            http_client: None,
            retry_config: None,
            sleep_impl: None,
            region: None,
            use_fips: None,
            use_dual_stack: None,
            profile_name_override: None,
        }
    }
}

impl ProviderConfig {
    /// Create a default provider config with the region unset.
    ///
    /// Using this option means that you may need to set a region manually.
    ///
    /// This constructor will use a default value for the HTTPS connector and Sleep implementation
    /// when they are enabled as crate features which is usually the correct option. To construct
    /// a `ProviderConfig` without these fields set, use [`ProviderConfig::empty`].
    ///
    ///
    /// # Examples
    /// ```no_run
    /// # #[cfg(feature = "default-https-client")]
    /// # fn example() {
    /// use aws_config::provider_config::ProviderConfig;
    /// use aws_sdk_sts::config::Region;
    /// use aws_config::web_identity_token::WebIdentityTokenCredentialsProvider;
    /// let conf = ProviderConfig::without_region().with_region(Some(Region::new("us-east-1")));
    ///
    /// let credential_provider = WebIdentityTokenCredentialsProvider::builder().configure(&conf).build();
    /// # }
    /// ```
    pub fn without_region() -> Self {
        Self::default()
    }

    /// Constructs a ProviderConfig with no fields set
    pub fn empty() -> Self {
        ProviderConfig {
            env: Env::default(),
            fs: Fs::default(),
            time_source: SharedTimeSource::default(),
            http_client: None,
            retry_config: None,
            sleep_impl: None,
            region: None,
            use_fips: None,
            use_dual_stack: None,
            parsed_profile: Default::default(),
            #[allow(deprecated)]
            profile_files: ProfileFiles::default(),
            profile_name_override: None,
        }
    }

    /// Initializer for ConfigBag to avoid possibly setting incorrect defaults.
    pub(crate) fn init(
        time_source: SharedTimeSource,
        sleep_impl: Option<SharedAsyncSleep>,
    ) -> Self {
        Self {
            parsed_profile: Default::default(),
            #[allow(deprecated)]
            profile_files: ProfileFiles::default(),
            env: Env::default(),
            fs: Fs::default(),
            time_source,
            http_client: None,
            retry_config: None,
            sleep_impl,
            region: None,
            use_fips: None,
            use_dual_stack: None,
            profile_name_override: None,
        }
    }

    /// Create a default provider config with the region region automatically loaded from the default chain.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn test() {
    /// use aws_config::provider_config::ProviderConfig;
    /// use aws_sdk_sts::config::Region;
    /// use aws_config::web_identity_token::WebIdentityTokenCredentialsProvider;
    /// let conf = ProviderConfig::with_default_region().await;
    /// let credential_provider = WebIdentityTokenCredentialsProvider::builder().configure(&conf).build();
    /// }
    /// ```
    pub async fn with_default_region() -> Self {
        Self::without_region().load_default_region().await
    }

    /// Attempt to get a representation of `SdkConfig` from this `ProviderConfig`.
    ///
    ///
    /// **WARN**: Some options (e.g. `service_config`) can only be set if the profile has been
    /// parsed already (e.g. by calling [`ProviderConfig::profile()`]). This is an
    /// imperfect mapping and should be used sparingly.
    pub(crate) fn client_config(&self) -> SdkConfig {
        let profiles = self.parsed_profile.get().and_then(|v| v.as_ref().ok());
        let service_config = EnvServiceConfig {
            env: self.env(),
            env_config_sections: profiles.cloned().unwrap_or_default(),
        };

        let mut builder = SdkConfig::builder()
            .retry_config(
                self.retry_config
                    .as_ref()
                    .map_or(RetryConfig::standard(), |config| config.clone()),
            )
            .region(self.region())
            .time_source(self.time_source())
            .use_fips(self.use_fips().unwrap_or_default())
            .use_dual_stack(self.use_dual_stack().unwrap_or_default())
            .service_config(service_config)
            .behavior_version(crate::BehaviorVersion::latest());
        builder.set_http_client(self.http_client.clone());
        builder.set_sleep_impl(self.sleep_impl.clone());
        builder.build()
    }

    // When all crate features are disabled, these accessors are unused

    #[allow(dead_code)]
    pub(crate) fn env(&self) -> Env {
        self.env.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn fs(&self) -> Fs {
        self.fs.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn time_source(&self) -> SharedTimeSource {
        self.time_source.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn http_client(&self) -> Option<SharedHttpClient> {
        self.http_client.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn retry_config(&self) -> Option<RetryConfig> {
        self.retry_config.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn sleep_impl(&self) -> Option<SharedAsyncSleep> {
        self.sleep_impl.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn region(&self) -> Option<Region> {
        self.region.clone()
    }

    #[allow(dead_code)]
    pub(crate) fn use_fips(&self) -> Option<bool> {
        self.use_fips
    }

    #[allow(dead_code)]
    pub(crate) fn use_dual_stack(&self) -> Option<bool> {
        self.use_dual_stack
    }

    pub(crate) async fn try_profile(&self) -> Result<&ProfileSet, &ProfileFileLoadError> {
        let parsed_profile = self
            .parsed_profile
            .get_or_init(|| async {
                let profile = profile::load(
                    &self.fs,
                    &self.env,
                    &self.profile_files,
                    self.profile_name_override.clone(),
                )
                .await;
                if let Err(err) = profile.as_ref() {
                    tracing::warn!(err = %DisplayErrorContext(&err), "failed to parse profile")
                }
                profile
            })
            .await;
        parsed_profile.as_ref()
    }

    pub(crate) async fn profile(&self) -> Option<&ProfileSet> {
        self.try_profile().await.ok()
    }

    /// Override the region for the configuration
    pub fn with_region(mut self, region: Option<Region>) -> Self {
        self.region = region;
        self
    }

    /// Override the `use_fips` setting.
    pub(crate) fn with_use_fips(mut self, use_fips: Option<bool>) -> Self {
        self.use_fips = use_fips;
        self
    }

    /// Override the `use_dual_stack` setting.
    pub(crate) fn with_use_dual_stack(mut self, use_dual_stack: Option<bool>) -> Self {
        self.use_dual_stack = use_dual_stack;
        self
    }

    pub(crate) fn with_profile_name(self, profile_name: String) -> Self {
        let profile_files = self.profile_files.clone();
        self.with_profile_config(Some(profile_files), Some(profile_name))
    }

    /// Override the profile file paths (`~/.aws/config` by default) and name (`default` by default)
    #[allow(deprecated)]
    pub(crate) fn with_profile_config(
        self,
        profile_files: Option<ProfileFiles>,
        profile_name_override: Option<String>,
    ) -> Self {
        // if there is no override, then don't clear out `parsed_profile`.
        if profile_files.is_none() && profile_name_override.is_none() {
            return self;
        }
        ProviderConfig {
            // clear out the profile since we need to reparse it
            parsed_profile: Default::default(),
            profile_files: profile_files.unwrap_or(self.profile_files),
            profile_name_override: profile_name_override
                .map(Cow::Owned)
                .or(self.profile_name_override),
            ..self
        }
    }

    /// Use the [default region chain](crate::default_provider::region) to set the
    /// region for this configuration
    ///
    /// Note: the `env` and `fs` already set on this provider will be used when loading the default region.
    pub async fn load_default_region(self) -> Self {
        use crate::default_provider::region::DefaultRegionChain;
        let provider_chain = DefaultRegionChain::builder().configure(&self).build();
        self.with_region(provider_chain.region().await)
    }

    pub(crate) fn with_fs(self, fs: Fs) -> Self {
        ProviderConfig {
            parsed_profile: Default::default(),
            fs,
            ..self
        }
    }

    pub(crate) fn with_env(self, env: Env) -> Self {
        ProviderConfig {
            parsed_profile: Default::default(),
            env,
            ..self
        }
    }

    /// Override the time source for this configuration
    pub fn with_time_source(self, time_source: impl TimeSource + 'static) -> Self {
        ProviderConfig {
            time_source: time_source.into_shared(),
            ..self
        }
    }

    /// Override the HTTP client for this configuration
    pub fn with_http_client(self, http_client: impl HttpClient + 'static) -> Self {
        ProviderConfig {
            http_client: Some(http_client.into_shared()),
            ..self
        }
    }

    /// Override the sleep implementation for this configuration
    pub fn with_sleep_impl(self, sleep_impl: impl AsyncSleep + 'static) -> Self {
        ProviderConfig {
            sleep_impl: Some(sleep_impl.into_shared()),
            ..self
        }
    }

    /// Override the retry config for this configuration
    pub fn with_retry_config(self, retry_config: RetryConfig) -> Self {
        ProviderConfig {
            retry_config: Some(retry_config),
            ..self
        }
    }
}
