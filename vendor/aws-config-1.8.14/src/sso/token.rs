/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! SSO Token Provider
//!
//! This token provider enables loading an access token from `~/.aws/sso/cache`. For more information,
//! see [AWS Builder ID for developers](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/builder-id.html).
//!
//! This provider is included automatically when profiles are loaded.

use crate::identity::IdentityCache;
use crate::sso::cache::{
    load_cached_token, save_cached_token, CachedSsoToken, CachedSsoTokenError,
};
use aws_credential_types::provider::token::ProvideToken;
use aws_credential_types::provider::{
    error::TokenError, future::ProvideToken as ProvideTokenFuture,
};
use aws_sdk_ssooidc::error::DisplayErrorContext;
use aws_sdk_ssooidc::operation::create_token::CreateTokenOutput;
use aws_sdk_ssooidc::Client as SsoOidcClient;
use aws_smithy_async::time::SharedTimeSource;
use aws_smithy_runtime::expiring_cache::ExpiringCache;
use aws_smithy_runtime_api::client::identity::http::Token;
use aws_smithy_runtime_api::client::identity::{IdentityFuture, ResolveIdentity};
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;
use aws_types::os_shim_internal::{Env, Fs};
use aws_types::region::Region;
use aws_types::SdkConfig;
use std::error::Error as StdError;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use zeroize::Zeroizing;

const REFRESH_BUFFER_TIME: Duration = Duration::from_secs(5 * 60 /* 5 minutes */);
const MIN_TIME_BETWEEN_REFRESH: Duration = Duration::from_secs(30);

/// SSO Token Provider
///
/// This token provider will use cached SSO tokens stored in `~/.aws/sso/cache/<hash>.json`.
/// `<hash>` is computed based on the configured [`session_name`](Builder::session_name).
///
/// If possible, the cached token will be refreshed when it gets close to expiring.
#[derive(Debug)]
pub struct SsoTokenProvider {
    inner: Arc<Inner>,
    token_cache: ExpiringCache<CachedSsoToken, SsoTokenProviderError>,
}

#[derive(Debug)]
struct Inner {
    env: Env,
    fs: Fs,
    region: Region,
    session_name: String,
    start_url: String,
    sdk_config: SdkConfig,
    last_refresh_attempt: Mutex<Option<SystemTime>>,
}

impl SsoTokenProvider {
    /// Creates a `SsoTokenProvider` builder.
    pub fn builder() -> Builder {
        Default::default()
    }

    async fn refresh_cached_token(
        inner: &Inner,
        cached_token: &CachedSsoToken,
        identifier: &str,
        now: SystemTime,
    ) -> Result<Option<CachedSsoToken>, SsoTokenProviderError> {
        // TODO(enableNewSmithyRuntimeCleanup): Use `customize().config_override()` to set the region instead of creating a new client once middleware is removed
        let config = inner
            .sdk_config
            .to_builder()
            .region(Some(inner.region.clone()))
            .identity_cache(IdentityCache::no_cache())
            .build();
        let client = SsoOidcClient::new(&config);
        let resp = client
            .create_token()
            .grant_type("refresh_token")
            .client_id(
                cached_token
                    .client_id
                    .as_ref()
                    .expect("required for token refresh")
                    .clone(),
            )
            .client_secret(
                cached_token
                    .client_secret
                    .as_ref()
                    .expect("required for token refresh")
                    .as_str(),
            )
            .refresh_token(
                cached_token
                    .refresh_token
                    .as_ref()
                    .expect("required for token refresh")
                    .as_str(),
            )
            .send()
            .await;
        match resp {
            Ok(CreateTokenOutput {
                access_token: Some(access_token),
                refresh_token,
                expires_in,
                ..
            }) => {
                let refreshed_token = CachedSsoToken {
                    access_token: Zeroizing::new(access_token),
                    client_id: cached_token.client_id.clone(),
                    client_secret: cached_token.client_secret.clone(),
                    expires_at: now
                        + Duration::from_secs(
                            u64::try_from(expires_in)
                                .map_err(|_| SsoTokenProviderError::BadExpirationTimeFromSsoOidc)?,
                        ),
                    refresh_token: refresh_token
                        .map(Zeroizing::new)
                        .or_else(|| cached_token.refresh_token.clone()),
                    region: Some(inner.region.to_string()),
                    registration_expires_at: cached_token.registration_expires_at,
                    start_url: Some(inner.start_url.clone()),
                };
                save_cached_token(&inner.env, &inner.fs, identifier, &refreshed_token).await?;
                tracing::debug!("saved refreshed SSO token");
                Ok(Some(refreshed_token))
            }
            Ok(_) => {
                tracing::debug!("SSO OIDC CreateToken responded without an access token");
                Ok(None)
            }
            Err(err) => {
                tracing::debug!(
                    "call to SSO OIDC CreateToken for SSO token refresh failed: {}",
                    DisplayErrorContext(&err)
                );
                Ok(None)
            }
        }
    }

    pub(super) fn resolve_token(
        &self,
        time_source: SharedTimeSource,
    ) -> impl std::future::Future<Output = Result<CachedSsoToken, TokenError>> + 'static {
        let token_cache = self.token_cache.clone();
        let inner = self.inner.clone();

        async move {
            if let Some(token) = token_cache
                .yield_or_clear_if_expired(time_source.now())
                .await
            {
                tracing::debug!("using cached SSO token");
                return Ok(token);
            }
            let token = token_cache
                .get_or_load(|| async move {
                    tracing::debug!("expiring cache asked for an updated SSO token");
                    let mut token =
                        load_cached_token(&inner.env, &inner.fs, &inner.session_name).await?;
                    tracing::debug!("loaded cached SSO token");

                    let now = time_source.now();
                    let expired = token.expires_at <= now;
                    let expires_soon = token.expires_at - REFRESH_BUFFER_TIME <= now;
                    let last_refresh = *inner.last_refresh_attempt.lock().unwrap();
                    let min_time_passed = last_refresh
                        .map(|lr| {
                            now.duration_since(lr).expect("last_refresh is in the past")
                                >= MIN_TIME_BETWEEN_REFRESH
                        })
                        .unwrap_or(true);
                    let registration_expired = token
                        .registration_expires_at
                        .map(|t| t <= now)
                        .unwrap_or(true);
                    let refreshable =
                        token.refreshable() && min_time_passed && !registration_expired;

                    tracing::debug!(
                        expired = ?expired,
                        expires_soon = ?expires_soon,
                        min_time_passed = ?min_time_passed,
                        registration_expired = ?registration_expired,
                        refreshable = ?refreshable,
                        will_refresh = ?(expires_soon && refreshable),
                        "cached SSO token refresh decision"
                    );

                    // Fail fast if the token has expired and we can't refresh it
                    if expired && !refreshable {
                        tracing::debug!("cached SSO token is expired and cannot be refreshed");
                        return Err(SsoTokenProviderError::ExpiredToken);
                    }

                    // Refresh the token if it is going to expire soon
                    if expires_soon && refreshable {
                        tracing::debug!("attempting to refresh SSO token");
                        if let Some(refreshed_token) =
                            Self::refresh_cached_token(&inner, &token, &inner.session_name, now)
                                .await?
                        {
                            token = refreshed_token;
                        }
                        *inner.last_refresh_attempt.lock().unwrap() = Some(now);
                    }

                    let expires_at = token.expires_at;
                    Ok((token, expires_at))
                })
                .await
                .map_err(TokenError::provider_error)?;

            Ok(token)
        }
    }
}

impl ProvideToken for SsoTokenProvider {
    fn provide_token<'a>(&'a self) -> ProvideTokenFuture<'a>
    where
        Self: 'a,
    {
        let time_source = self
            .inner
            .sdk_config
            .time_source()
            .expect("a time source required by SsoTokenProvider");
        let token_future = self.resolve_token(time_source);
        ProvideTokenFuture::new(Box::pin(async move {
            let token = token_future.await?;
            Ok(Token::new(
                token.access_token.as_str(),
                Some(token.expires_at),
            ))
        }))
    }
}

impl ResolveIdentity for SsoTokenProvider {
    fn resolve_identity<'a>(
        &'a self,
        runtime_components: &'a RuntimeComponents,
        config_bag: &'a ConfigBag,
    ) -> IdentityFuture<'a> {
        IdentityFuture::new(Box::pin(async move {
            self.provide_token()
                .await?
                .resolve_identity(runtime_components, config_bag)
                .await
        }))
    }
}

/// Builder for [`SsoTokenProvider`].
#[derive(Debug, Default)]
pub struct Builder {
    sdk_config: Option<SdkConfig>,
    region: Option<Region>,
    session_name: Option<String>,
    start_url: Option<String>,
}

impl Builder {
    /// Creates a new builder for [`SsoTokenProvider`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Override the configuration used for this provider
    pub fn configure(mut self, sdk_config: &SdkConfig) -> Self {
        self.sdk_config = Some(sdk_config.clone());
        self
    }

    /// Sets the SSO region.
    ///
    /// This is a required field.
    pub fn region(mut self, region: impl Into<Region>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Sets the SSO region.
    ///
    /// This is a required field.
    pub fn set_region(&mut self, region: Option<Region>) -> &mut Self {
        self.region = region;
        self
    }

    /// Sets the SSO session name.
    ///
    /// This is a required field.
    pub fn session_name(mut self, session_name: impl Into<String>) -> Self {
        self.session_name = Some(session_name.into());
        self
    }

    /// Sets the SSO session name.
    ///
    /// This is a required field.
    pub fn set_session_name(&mut self, session_name: Option<String>) -> &mut Self {
        self.session_name = session_name;
        self
    }

    /// Sets the SSO start URL.
    ///
    /// This is a required field.
    pub fn start_url(mut self, start_url: impl Into<String>) -> Self {
        self.start_url = Some(start_url.into());
        self
    }

    /// Sets the SSO start URL.
    ///
    /// This is a required field.
    pub fn set_start_url(&mut self, start_url: Option<String>) -> &mut Self {
        self.start_url = start_url;
        self
    }

    /// Builds the [`SsoTokenProvider`].
    ///
    /// # Panics
    ///
    /// This will panic if any of the required fields are not given.
    pub async fn build(mut self) -> SsoTokenProvider {
        if self.sdk_config.is_none() {
            self.sdk_config = Some(crate::load_defaults(crate::BehaviorVersion::latest()).await);
        }
        self.build_with(Env::real(), Fs::real())
    }

    pub(crate) fn build_with(self, env: Env, fs: Fs) -> SsoTokenProvider {
        SsoTokenProvider {
            inner: Arc::new(Inner {
                env,
                fs,
                region: self.region.expect("region is required"),
                session_name: self.session_name.expect("session_name is required"),
                start_url: self.start_url.expect("start_url is required"),
                sdk_config: self.sdk_config.expect("sdk_config is required"),
                last_refresh_attempt: Mutex::new(None),
            }),
            token_cache: ExpiringCache::new(REFRESH_BUFFER_TIME),
        }
    }
}

#[derive(Debug)]
pub(super) enum SsoTokenProviderError {
    BadExpirationTimeFromSsoOidc,
    FailedToLoadToken {
        source: Box<dyn StdError + Send + Sync>,
    },
    ExpiredToken,
}

impl fmt::Display for SsoTokenProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadExpirationTimeFromSsoOidc => {
                f.write_str("SSO OIDC responded with a negative expiration duration")
            }
            Self::ExpiredToken => f.write_str("the SSO token has expired and cannot be refreshed"),
            Self::FailedToLoadToken { .. } => f.write_str("failed to load the cached SSO token"),
        }
    }
}

impl StdError for SsoTokenProviderError {
    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            Self::BadExpirationTimeFromSsoOidc => None,
            Self::ExpiredToken => None,
            Self::FailedToLoadToken { source } => Some(source.as_ref()),
        }
    }
}

impl From<CachedSsoTokenError> for SsoTokenProviderError {
    fn from(source: CachedSsoTokenError) -> Self {
        Self::FailedToLoadToken {
            source: source.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_sso::config::{AsyncSleep, SharedAsyncSleep};
    use aws_smithy_async::rt::sleep::TokioSleep;
    use aws_smithy_async::test_util::instant_time_and_sleep;
    use aws_smithy_async::time::{StaticTimeSource, TimeSource};
    use aws_smithy_http_client::test_util::{capture_request, ReplayEvent, StaticReplayClient};
    use aws_smithy_runtime::{
        assert_str_contains, test_util::capture_test_logs::capture_test_logs,
    };
    use aws_smithy_runtime_api::client::http::HttpClient;
    use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
    use aws_smithy_types::body::SdkBody;
    use aws_smithy_types::date_time::Format;
    use aws_smithy_types::retry::RetryConfig;
    use aws_smithy_types::DateTime;

    fn time(s: &str) -> SystemTime {
        SystemTime::try_from(DateTime::from_str(s, Format::DateTime).unwrap()).unwrap()
    }

    struct TestHarness {
        time_source: SharedTimeSource,
        token_provider: SsoTokenProvider,
        env: Env,
        fs: Fs,
    }

    impl TestHarness {
        fn new(
            time_source: impl TimeSource + 'static,
            sleep_impl: impl AsyncSleep + 'static,
            http_client: impl HttpClient + 'static,
            fs: Fs,
        ) -> Self {
            let env = Env::from_slice(&[("HOME", "/home/user")]);
            let time_source = SharedTimeSource::new(time_source);
            let config = SdkConfig::builder()
                .http_client(http_client)
                .time_source(time_source.clone())
                .sleep_impl(SharedAsyncSleep::new(sleep_impl))
                // disable retry to simplify testing
                .retry_config(RetryConfig::disabled())
                .behavior_version(crate::BehaviorVersion::latest())
                .build();
            Self {
                time_source,
                token_provider: SsoTokenProvider::builder()
                    .configure(&config)
                    .session_name("test")
                    .region(Region::new("us-west-2"))
                    .start_url("https://d-123.awsapps.com/start")
                    .build_with(env.clone(), fs.clone()),
                env,
                fs,
            }
        }

        async fn expect_sso_token(&self, value: &str, expires_at: &str) -> CachedSsoToken {
            let token = self
                .token_provider
                .resolve_token(self.time_source.clone())
                .await
                .unwrap();
            assert_eq!(value, token.access_token.as_str());
            assert_eq!(time(expires_at), token.expires_at);
            token
        }

        async fn expect_token(&self, value: &str, expires_at: &str) {
            let runtime_components = RuntimeComponentsBuilder::for_tests()
                .with_time_source(Some(self.time_source.clone()))
                .build()
                .unwrap();
            let config_bag = ConfigBag::base();
            let identity = self
                .token_provider
                .resolve_identity(&runtime_components, &config_bag)
                .await
                .unwrap();
            let token = identity.data::<Token>().unwrap().clone();
            assert_eq!(value, token.token());
            assert_eq!(time(expires_at), identity.expiration().unwrap());
        }

        async fn expect_expired_token_err(&self) {
            let err = DisplayErrorContext(
                &self
                    .token_provider
                    .resolve_token(self.time_source.clone())
                    .await
                    .expect_err("expected failure"),
            )
            .to_string();
            assert_str_contains!(err, "the SSO token has expired");
        }

        fn last_refresh_attempt_time(&self) -> Option<String> {
            self.token_provider
                .inner
                .last_refresh_attempt
                .lock()
                .unwrap()
                .map(|time| {
                    DateTime::try_from(time)
                        .unwrap()
                        .fmt(Format::DateTime)
                        .unwrap()
                })
        }
    }

    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is ignored on Windows because it uses Unix-style paths
    #[cfg_attr(windows, ignore)]
    #[tokio::test]
    async fn use_unexpired_cached_token() {
        let fs = Fs::from_slice(&[(
            "/home/user/.aws/sso/cache/a94a8fe5ccb19ba61c4c0873d391e987982fbbd3.json",
            r#"
            { "accessToken": "some-token",
              "expiresAt": "1975-01-01T00:00:00Z" }
            "#,
        )]);

        let now = time("1974-12-25T00:00:00Z");
        let time_source = SharedTimeSource::new(StaticTimeSource::new(now));

        let (conn, req_rx) = capture_request(None);
        let harness = TestHarness::new(time_source, TokioSleep::new(), conn, fs);

        harness
            .expect_token("some-token", "1975-01-01T00:00:00Z")
            .await;
        // it can't refresh this token
        req_rx.expect_no_request();
    }

    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is ignored on Windows because it uses Unix-style paths
    #[cfg_attr(windows, ignore)]
    #[tokio::test]
    async fn expired_cached_token() {
        let fs = Fs::from_slice(&[(
            "/home/user/.aws/sso/cache/a94a8fe5ccb19ba61c4c0873d391e987982fbbd3.json",
            r#"
            { "accessToken": "some-token",
              "expiresAt": "1999-12-15T00:00:00Z" }
            "#,
        )]);

        let now = time("2023-01-01T00:00:00Z");
        let time_source = SharedTimeSource::new(StaticTimeSource::new(now));

        let (conn, req_rx) = capture_request(None);
        let harness = TestHarness::new(time_source, TokioSleep::new(), conn, fs);

        harness.expect_expired_token_err().await;
        // it can't refresh this token
        req_rx.expect_no_request();
    }

    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is ignored on Windows because it uses Unix-style paths
    #[cfg_attr(windows, ignore)]
    #[tokio::test]
    async fn expired_token_and_expired_client_registration() {
        let fs = Fs::from_slice(&[(
            "/home/user/.aws/sso/cache/a94a8fe5ccb19ba61c4c0873d391e987982fbbd3.json",
            r#"
            { "startUrl": "https://d-123.awsapps.com/start",
              "region": "us-west-2",
              "accessToken": "cachedtoken",
              "expiresAt": "2021-10-25T13:00:00Z",
              "clientId": "clientid",
              "clientSecret": "YSBzZWNyZXQ=",
              "registrationExpiresAt": "2021-11-25T13:30:00Z",
              "refreshToken": "cachedrefreshtoken" }
            "#,
        )]);

        let now = time("2023-08-11T04:11:17Z");
        let time_source = SharedTimeSource::new(StaticTimeSource::new(now));

        let (conn, req_rx) = capture_request(None);
        let harness = TestHarness::new(time_source, TokioSleep::new(), conn, fs);

        // the registration has expired, so the token can't be refreshed
        harness.expect_expired_token_err().await;
        req_rx.expect_no_request();
    }

    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is ignored on Windows because it uses Unix-style paths
    #[cfg_attr(windows, ignore)]
    #[tokio::test]
    async fn expired_token_refresh_with_refresh_token() {
        let fs = Fs::from_slice(&[(
            "/home/user/.aws/sso/cache/a94a8fe5ccb19ba61c4c0873d391e987982fbbd3.json",
            r#"
            { "startUrl": "https://d-123.awsapps.com/start",
              "region": "us-west-2",
              "accessToken": "cachedtoken",
              "expiresAt": "2021-12-25T13:00:00Z",
              "clientId": "clientid",
              "clientSecret": "YSBzZWNyZXQ=",
              "registrationExpiresAt": "2022-12-25T13:30:00Z",
              "refreshToken": "cachedrefreshtoken" }
            "#,
        )]);

        let now = time("2021-12-25T13:30:00Z");
        let time_source = SharedTimeSource::new(StaticTimeSource::new(now));

        let (conn, req_rx) = capture_request(Some(
            http::Response::builder()
                .status(200)
                .body(SdkBody::from(
                    r#"
                    { "tokenType": "Bearer",
                      "accessToken": "newtoken",
                      "expiresIn": 28800,
                      "refreshToken": "newrefreshtoken" }
                    "#,
                ))
                .unwrap(),
        ));
        let harness = TestHarness::new(time_source, TokioSleep::new(), conn, fs);

        let returned_token = harness
            .expect_sso_token("newtoken", "2021-12-25T21:30:00Z")
            .await;
        let cached_token = load_cached_token(&harness.env, &harness.fs, "test")
            .await
            .unwrap();
        assert_eq!(returned_token, cached_token);
        assert_eq!(
            "newrefreshtoken",
            returned_token.refresh_token.unwrap().as_str()
        );
        assert_eq!(
            "https://d-123.awsapps.com/start",
            returned_token.start_url.unwrap()
        );
        assert_eq!("us-west-2", returned_token.region.unwrap().to_string());
        assert_eq!("clientid", returned_token.client_id.unwrap());
        assert_eq!(
            "YSBzZWNyZXQ=",
            returned_token.client_secret.unwrap().as_str()
        );
        assert_eq!(
            SystemTime::UNIX_EPOCH + Duration::from_secs(1_671_975_000),
            returned_token.registration_expires_at.unwrap()
        );

        let refresh_req = req_rx.expect_request();
        let parsed_req: serde_json::Value =
            serde_json::from_slice(refresh_req.body().bytes().unwrap()).unwrap();
        let parsed_req = parsed_req.as_object().unwrap();
        assert_eq!(
            "clientid",
            parsed_req.get("clientId").unwrap().as_str().unwrap()
        );
        assert_eq!(
            "YSBzZWNyZXQ=",
            parsed_req.get("clientSecret").unwrap().as_str().unwrap()
        );
        assert_eq!(
            "refresh_token",
            parsed_req.get("grantType").unwrap().as_str().unwrap()
        );
        assert_eq!(
            "cachedrefreshtoken",
            parsed_req.get("refreshToken").unwrap().as_str().unwrap()
        );
    }

    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is ignored on Windows because it uses Unix-style paths
    #[cfg_attr(windows, ignore)]
    #[tokio::test]
    async fn expired_token_refresh_fails() {
        let fs = Fs::from_slice(&[(
            "/home/user/.aws/sso/cache/a94a8fe5ccb19ba61c4c0873d391e987982fbbd3.json",
            r#"
            { "startUrl": "https://d-123.awsapps.com/start",
              "region": "us-west-2",
              "accessToken": "cachedtoken",
              "expiresAt": "2021-12-25T13:00:00Z",
              "clientId": "clientid",
              "clientSecret": "YSBzZWNyZXQ=",
              "registrationExpiresAt": "2022-12-25T13:30:00Z",
              "refreshToken": "cachedrefreshtoken" }
            "#,
        )]);

        let now = time("2021-12-25T13:30:00Z");
        let time_source = SharedTimeSource::new(StaticTimeSource::new(now));

        let (conn, req_rx) = capture_request(Some(
            http::Response::builder()
                .status(500)
                .body(SdkBody::from(""))
                .unwrap(),
        ));
        let harness = TestHarness::new(time_source, TokioSleep::new(), conn, fs);

        // it should return the previous token since refresh failed and it hasn't expired yet
        let returned_token = harness
            .expect_sso_token("cachedtoken", "2021-12-25T13:00:00Z")
            .await;
        let cached_token = load_cached_token(&harness.env, &harness.fs, "test")
            .await
            .unwrap();
        assert_eq!(returned_token, cached_token);

        let _ = req_rx.expect_request();
    }

    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is ignored on Windows because it uses Unix-style paths
    #[cfg_attr(windows, ignore)]
    // Expired token refresh without new refresh token
    #[tokio::test]
    async fn expired_token_refresh_without_new_refresh_token() {
        let fs = Fs::from_slice(&[(
            "/home/user/.aws/sso/cache/a94a8fe5ccb19ba61c4c0873d391e987982fbbd3.json",
            r#"
            { "startUrl": "https://d-123.awsapps.com/start",
              "region": "us-west-2",
              "accessToken": "cachedtoken",
              "expiresAt": "2021-12-25T13:00:00Z",
              "clientId": "clientid",
              "clientSecret": "YSBzZWNyZXQ=",
              "registrationExpiresAt": "2022-12-25T13:30:00Z",
              "refreshToken": "cachedrefreshtoken" }
            "#,
        )]);

        let now = time("2021-12-25T13:30:00Z");
        let time_source = SharedTimeSource::new(StaticTimeSource::new(now));

        let (conn, req_rx) = capture_request(Some(
            http::Response::builder()
                .status(200)
                .body(SdkBody::from(
                    r#"
                    { "tokenType": "Bearer",
                      "accessToken": "newtoken",
                      "expiresIn": 28800 }
                    "#,
                ))
                .unwrap(),
        ));
        let harness = TestHarness::new(time_source, TokioSleep::new(), conn, fs);

        let returned_token = harness
            .expect_sso_token("newtoken", "2021-12-25T21:30:00Z")
            .await;
        let cached_token = load_cached_token(&harness.env, &harness.fs, "test")
            .await
            .unwrap();
        assert_eq!(returned_token, cached_token);
        assert_eq!(
            "cachedrefreshtoken",
            returned_token.refresh_token.unwrap().as_str(),
            "it should have kept the old refresh token"
        );

        let _ = req_rx.expect_request();
    }

    // TODO(https://github.com/awslabs/aws-sdk-rust/issues/1117) This test is ignored on Windows because it uses Unix-style paths
    #[cfg_attr(windows, ignore)]
    #[tokio::test]
    async fn refresh_timings() {
        let _logs = capture_test_logs();

        let start_time = DateTime::from_str("2023-01-01T00:00:00Z", Format::DateTime).unwrap();
        let (time_source, sleep_impl) = instant_time_and_sleep(start_time.try_into().unwrap());
        let shared_time_source = SharedTimeSource::new(time_source.clone());

        let fs = Fs::from_slice(&[(
            "/home/user/.aws/sso/cache/a94a8fe5ccb19ba61c4c0873d391e987982fbbd3.json",
            r#"
            { "startUrl": "https://d-123.awsapps.com/start",
              "region": "us-west-2",
              "accessToken": "first_token",
              "_comment_expiresAt": "-------- Ten minutes after the start time: ------",
              "expiresAt": "2023-01-01T00:10:00Z",
              "clientId": "clientid",
              "clientSecret": "YSBzZWNyZXQ=",
              "registrationExpiresAt": "2023-01-02T12:00:00Z",
              "refreshToken": "cachedrefreshtoken" }
            "#,
        )]);

        let events = vec![
            // First refresh attempt should fail
            ReplayEvent::new(
                http::Request::new(SdkBody::from("")), // don't really care what the request looks like
                http::Response::builder()
                    .status(500)
                    .body(SdkBody::from(""))
                    .unwrap(),
            ),
            // Second refresh attempt should also fail
            ReplayEvent::new(
                http::Request::new(SdkBody::from("")), // don't really care what the request looks like
                http::Response::builder()
                    .status(500)
                    .body(SdkBody::from(""))
                    .unwrap(),
            ),
            // Third refresh attempt will succeed
            ReplayEvent::new(
                http::Request::new(SdkBody::from("")), // don't really care what the request looks like
                http::Response::builder()
                    .status(200)
                    .body(SdkBody::from(
                        r#"
                        { "tokenType": "Bearer",
                          "accessToken": "second_token",
                          "expiresIn": 28800 }
                        "#,
                    ))
                    .unwrap(),
            ),
        ];
        let http_client = StaticReplayClient::new(events);
        let harness = TestHarness::new(shared_time_source, sleep_impl, http_client, fs);

        tracing::info!("test: first token retrieval should return the cached token");
        assert!(
            harness.last_refresh_attempt_time().is_none(),
            "the last attempt time should start empty"
        );
        harness
            .expect_token("first_token", "2023-01-01T00:10:00Z")
            .await;
        assert!(
            harness.last_refresh_attempt_time().is_none(),
            "it shouldn't have tried to refresh, so the last refresh attempt time shouldn't be set"
        );

        tracing::info!("test: advance 3 minutes");
        time_source.advance(Duration::from_secs(3 * 60));

        tracing::info!("test: the token shouldn't get refreshed since it's not in the 5 minute buffer time yet");
        harness
            .expect_token("first_token", "2023-01-01T00:10:00Z")
            .await;
        assert!(
            harness.last_refresh_attempt_time().is_none(),
            "it shouldn't have tried to refresh since the token isn't expiring soon"
        );

        tracing::info!("test: advance 2 minutes");
        time_source.advance(Duration::from_secs(2 * 60));

        tracing::info!(
            "test: the token will fail to refresh, and the old cached token will be returned"
        );
        harness
            .expect_token("first_token", "2023-01-01T00:10:00Z")
            .await;
        assert_eq!(
            Some("2023-01-01T00:05:00Z"),
            harness.last_refresh_attempt_time().as_deref(),
            "it should update the last refresh attempt time since the expiration time is soon"
        );

        tracing::info!("test: advance 15 seconds");
        time_source.advance(Duration::from_secs(15));

        tracing::info!(
            "test: the token will not refresh because the minimum time hasn't passed between attempts"
        );
        harness
            .expect_token("first_token", "2023-01-01T00:10:00Z")
            .await;

        tracing::info!("test: advance 15 seconds");
        time_source.advance(Duration::from_secs(15));

        tracing::info!(
            "test: the token will fail to refresh, and the old cached token will be returned"
        );
        harness
            .expect_token("first_token", "2023-01-01T00:10:00Z")
            .await;

        tracing::info!("test: advance 30 seconds");
        time_source.advance(Duration::from_secs(30));

        tracing::info!("test: the token will refresh successfully");
        harness
            .expect_token("second_token", "2023-01-01T08:06:00Z")
            .await;
    }
}
