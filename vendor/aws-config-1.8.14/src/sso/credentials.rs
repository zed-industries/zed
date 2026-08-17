/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! SSO Credentials Provider
//!
//! This credentials provider enables loading credentials from `~/.aws/sso/cache`. For more information,
//! see [Using AWS SSO Credentials](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/sso-credentials.html)
//!
//! This provider is included automatically when profiles are loaded.

use super::cache::load_cached_token;
use crate::identity::IdentityCache;
use crate::provider_config::ProviderConfig;
use crate::sso::SsoTokenProvider;
use aws_credential_types::credential_feature::AwsCredentialFeature;
use aws_credential_types::provider::{self, error::CredentialsError, future, ProvideCredentials};
use aws_credential_types::Credentials;
use aws_sdk_sso::types::RoleCredentials;
use aws_sdk_sso::Client as SsoClient;
use aws_smithy_async::time::SharedTimeSource;
use aws_smithy_types::DateTime;
use aws_types::os_shim_internal::{Env, Fs};
use aws_types::region::Region;
use aws_types::SdkConfig;

/// SSO Credentials Provider
///
/// _Note: This provider is part of the default credentials chain and is integrated with the profile-file provider._
///
/// This credentials provider will use cached SSO tokens stored in `~/.aws/sso/cache/<hash>.json`.
/// Two different values will be tried for `<hash>` in order:
/// 1. The configured [`session_name`](Builder::session_name).
/// 2. The configured [`start_url`](Builder::start_url).
#[derive(Debug)]
pub struct SsoCredentialsProvider {
    fs: Fs,
    env: Env,
    sso_provider_config: SsoProviderConfig,
    sdk_config: SdkConfig,
    token_provider: Option<SsoTokenProvider>,
    time_source: SharedTimeSource,
}

impl SsoCredentialsProvider {
    /// Creates a builder for [`SsoCredentialsProvider`]
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub(crate) fn new(
        provider_config: &ProviderConfig,
        sso_provider_config: SsoProviderConfig,
    ) -> Self {
        let fs = provider_config.fs();
        let env = provider_config.env();

        let token_provider = if let Some(session_name) = &sso_provider_config.session_name {
            Some(
                SsoTokenProvider::builder()
                    .configure(&provider_config.client_config())
                    .start_url(&sso_provider_config.start_url)
                    .session_name(session_name)
                    .region(sso_provider_config.region.clone())
                    .build_with(env.clone(), fs.clone()),
            )
        } else {
            None
        };

        SsoCredentialsProvider {
            fs,
            env,
            sso_provider_config,
            sdk_config: provider_config.client_config(),
            token_provider,
            time_source: provider_config.time_source(),
        }
    }

    async fn credentials(&self) -> provider::Result {
        load_sso_credentials(
            &self.sso_provider_config,
            &self.sdk_config,
            self.token_provider.as_ref(),
            &self.env,
            &self.fs,
            self.time_source.clone(),
        )
        .await
        .map(|mut creds| {
            creds
                .get_property_mut_or_default::<Vec<AwsCredentialFeature>>()
                .push(AwsCredentialFeature::CredentialsSso);
            creds
        })
    }
}

impl ProvideCredentials for SsoCredentialsProvider {
    fn provide_credentials<'a>(&'a self) -> future::ProvideCredentials<'a>
    where
        Self: 'a,
    {
        future::ProvideCredentials::new(self.credentials())
    }
}

/// Builder for [`SsoCredentialsProvider`]
#[derive(Default, Debug, Clone)]
pub struct Builder {
    provider_config: Option<ProviderConfig>,
    account_id: Option<String>,
    region: Option<Region>,
    role_name: Option<String>,
    start_url: Option<String>,
    session_name: Option<String>,
}

impl Builder {
    /// Create a new builder for [`SsoCredentialsProvider`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the configuration used for this provider
    pub fn configure(mut self, provider_config: &ProviderConfig) -> Self {
        self.provider_config = Some(provider_config.clone());
        self
    }

    /// Set the account id used for SSO
    ///
    /// This is a required field.
    pub fn account_id(mut self, account_id: impl Into<String>) -> Self {
        self.account_id = Some(account_id.into());
        self
    }

    /// Set the account id used for SSO
    ///
    /// This is a required field.
    pub fn set_account_id(&mut self, account_id: Option<String>) -> &mut Self {
        self.account_id = account_id;
        self
    }

    /// Set the region used for SSO
    ///
    /// This is a required field.
    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Set the region used for SSO
    ///
    /// This is a required field.
    pub fn set_region(&mut self, region: Option<Region>) -> &mut Self {
        self.region = region;
        self
    }

    /// Set the role name used for SSO
    ///
    /// This is a required field.
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.role_name = Some(role_name.into());
        self
    }

    /// Set the role name used for SSO
    ///
    /// This is a required field.
    pub fn set_role_name(&mut self, role_name: Option<String>) -> &mut Self {
        self.role_name = role_name;
        self
    }

    /// Set the start URL used for SSO
    ///
    /// This is a required field.
    pub fn start_url(mut self, start_url: impl Into<String>) -> Self {
        self.start_url = Some(start_url.into());
        self
    }

    /// Set the start URL used for SSO
    ///
    /// This is a required field.
    pub fn set_start_url(&mut self, start_url: Option<String>) -> &mut Self {
        self.start_url = start_url;
        self
    }

    /// Set the session name used for SSO
    pub fn session_name(mut self, session_name: impl Into<String>) -> Self {
        self.session_name = Some(session_name.into());
        self
    }

    /// Set the session name used for SSO
    pub fn set_session_name(&mut self, session_name: Option<String>) -> &mut Self {
        self.session_name = session_name;
        self
    }

    /// Construct an SsoCredentialsProvider from the builder
    ///
    /// # Panics
    /// This method will panic if the any of the following required fields are unset:
    /// - [`start_url`](Self::start_url)
    /// - [`role_name`](Self::role_name)
    /// - [`account_id`](Self::account_id)
    /// - [`region`](Self::region)
    pub fn build(self) -> SsoCredentialsProvider {
        let provider_config = self.provider_config.unwrap_or_default();
        let sso_config = SsoProviderConfig {
            account_id: self.account_id.expect("account_id must be set"),
            region: self.region.expect("region must be set"),
            role_name: self.role_name.expect("role_name must be set"),
            start_url: self.start_url.expect("start_url must be set"),
            session_name: self.session_name,
        };
        SsoCredentialsProvider::new(&provider_config, sso_config)
    }
}

#[derive(Debug)]
pub(crate) struct SsoProviderConfig {
    pub(crate) account_id: String,
    pub(crate) role_name: String,
    pub(crate) start_url: String,
    pub(crate) region: Region,
    pub(crate) session_name: Option<String>,
}

async fn load_sso_credentials(
    sso_provider_config: &SsoProviderConfig,
    sdk_config: &SdkConfig,
    token_provider: Option<&SsoTokenProvider>,
    env: &Env,
    fs: &Fs,
    time_source: SharedTimeSource,
) -> provider::Result {
    let token = if let Some(token_provider) = token_provider {
        token_provider
            .resolve_token(time_source)
            .await
            .map_err(CredentialsError::provider_error)?
    } else {
        // Backwards compatible token loading that uses `start_url` instead of `session_name`
        load_cached_token(env, fs, &sso_provider_config.start_url)
            .await
            .map_err(CredentialsError::provider_error)?
    };

    let config = sdk_config
        .to_builder()
        .region(sso_provider_config.region.clone())
        .identity_cache(IdentityCache::no_cache())
        .build();
    // TODO(enableNewSmithyRuntimeCleanup): Use `customize().config_override()` to set the region instead of creating a new client once middleware is removed
    let client = SsoClient::new(&config);
    let resp = client
        .get_role_credentials()
        .role_name(&sso_provider_config.role_name)
        .access_token(&*token.access_token)
        .account_id(&sso_provider_config.account_id)
        .send()
        .await
        .map_err(CredentialsError::provider_error)?;
    let credentials: RoleCredentials = resp
        .role_credentials
        .ok_or_else(|| CredentialsError::unhandled("SSO did not return credentials"))?;
    let akid = credentials
        .access_key_id
        .ok_or_else(|| CredentialsError::unhandled("no access key id in response"))?;
    let secret_key = credentials
        .secret_access_key
        .ok_or_else(|| CredentialsError::unhandled("no secret key in response"))?;
    let expiration = DateTime::from_millis(credentials.expiration)
        .try_into()
        .map_err(|err| {
            CredentialsError::unhandled(format!(
                "expiration could not be converted into a system time: {err}",
            ))
        })?;
    let mut builder = Credentials::builder()
        .access_key_id(akid)
        .secret_access_key(secret_key)
        .account_id(&sso_provider_config.account_id)
        .expiry(expiration)
        .provider_name("SSO");
    builder.set_session_token(credentials.session_token);
    Ok(builder.build())
}
