/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::provider_config::ProviderConfig;
use crate::retry::error::{RetryConfigError, RetryConfigErrorKind};
use aws_runtime::env_config::{EnvConfigError, EnvConfigValue};
use aws_smithy_types::error::display::DisplayErrorContext;
use aws_smithy_types::retry::{RetryConfig, RetryMode};
use std::str::FromStr;

/// Default RetryConfig Provider chain
///
/// Unlike other "providers" `RetryConfig` has no related `RetryConfigProvider` trait. Instead,
/// a builder struct is returned which has a similar API.
///
/// This provider will check the following sources in order:
/// 1. Environment variables: `AWS_MAX_ATTEMPTS` & `AWS_RETRY_MODE`
/// 2. Profile file: `max_attempts` and `retry_mode`
///
/// # Example
///
/// When running [`aws_config::from_env()`](crate::from_env()), a [`ConfigLoader`](crate::ConfigLoader)
/// is created that will then create a [`RetryConfig`] from the default_provider. There is no
/// need to call `default_provider` and the example below is only for illustration purposes.
///
/// ```no_run
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// use aws_config::default_provider::retry_config;
///
/// // Load a retry config from a specific profile
/// let retry_config = retry_config::default_provider()
///     .profile_name("other_profile")
///     .retry_config()
///     .await;
/// let config = aws_config::from_env()
///     // Override the retry config set by the default profile
///     .retry_config(retry_config)
///     .load()
///     .await;
/// // instantiate a service client:
/// // <my_aws_service>::Client::new(&config);
/// #     Ok(())
/// # }
/// ```
pub fn default_provider() -> Builder {
    Builder::default()
}

mod env {
    pub(super) const MAX_ATTEMPTS: &str = "AWS_MAX_ATTEMPTS";
    pub(super) const RETRY_MODE: &str = "AWS_RETRY_MODE";
}

mod profile_keys {
    pub(super) const MAX_ATTEMPTS: &str = "max_attempts";
    pub(super) const RETRY_MODE: &str = "retry_mode";
}

/// Builder for RetryConfig that checks the environment and aws profile for configuration
#[derive(Debug, Default)]
pub struct Builder {
    provider_config: ProviderConfig,
}

impl Builder {
    /// Configure the default chain
    ///
    /// Exposed for overriding the environment when unit-testing providers
    pub fn configure(mut self, configuration: &ProviderConfig) -> Self {
        self.provider_config = configuration.clone();
        self
    }

    /// Override the profile name used by this provider
    pub fn profile_name(mut self, name: &str) -> Self {
        self.provider_config = self.provider_config.with_profile_name(name.to_string());
        self
    }

    /// Attempt to create a [`RetryConfig`] from following sources in order:
    /// 1. Environment variables: `AWS_MAX_ATTEMPTS` & `AWS_RETRY_MODE`
    /// 2. Profile file: `max_attempts` and `retry_mode`
    /// 3. [RetryConfig::standard()](aws_smithy_types::retry::RetryConfig::standard)
    ///
    /// Precedence is considered on a per-field basis
    ///
    /// # Panics
    ///
    /// - Panics if the `AWS_MAX_ATTEMPTS` env var or `max_attempts` profile var is set to 0
    /// - Panics if the `AWS_RETRY_MODE` env var or `retry_mode` profile var is set to "adaptive" (it's not yet supported)
    pub async fn retry_config(self) -> RetryConfig {
        match self.try_retry_config().await {
            Ok(conf) => conf,
            Err(e) => panic!("{}", DisplayErrorContext(e)),
        }
    }

    pub(crate) async fn try_retry_config(
        self,
    ) -> Result<RetryConfig, EnvConfigError<RetryConfigError>> {
        let env = self.provider_config.env();
        let profiles = self.provider_config.profile().await;
        // Both of these can return errors due to invalid config settings, and we want to surface those as early as possible
        // hence, we'll panic if any config values are invalid (missing values are OK though)
        // We match this instead of unwrapping, so we can print the error with the `Display` impl instead of the `Debug` impl that unwrap uses
        let mut retry_config = RetryConfig::standard();
        let max_attempts = EnvConfigValue::new()
            .env(env::MAX_ATTEMPTS)
            .profile(profile_keys::MAX_ATTEMPTS)
            .validate(&env, profiles, validate_max_attempts);

        let retry_mode = EnvConfigValue::new()
            .env(env::RETRY_MODE)
            .profile(profile_keys::RETRY_MODE)
            .validate(&env, profiles, |s| {
                RetryMode::from_str(s)
                    .map_err(|err| RetryConfigErrorKind::InvalidRetryMode { source: err }.into())
            });

        if let Some(max_attempts) = max_attempts? {
            retry_config = retry_config.with_max_attempts(max_attempts);
        }

        if let Some(retry_mode) = retry_mode? {
            retry_config = retry_config.with_retry_mode(retry_mode);
        }

        Ok(retry_config)
    }
}

fn validate_max_attempts(max_attempts: &str) -> Result<u32, RetryConfigError> {
    match max_attempts.parse::<u32>() {
        Ok(0) => Err(RetryConfigErrorKind::MaxAttemptsMustNotBeZero.into()),
        Ok(max_attempts) => Ok(max_attempts),
        Err(source) => Err(RetryConfigErrorKind::FailedToParseMaxAttempts { source }.into()),
    }
}

#[cfg(test)]
mod test {
    use crate::default_provider::retry_config::env;
    use crate::provider_config::ProviderConfig;
    use crate::retry::{
        error::RetryConfigError, error::RetryConfigErrorKind, RetryConfig, RetryMode,
    };
    use aws_runtime::env_config::EnvConfigError;
    use aws_types::os_shim_internal::{Env, Fs};

    async fn test_provider(
        vars: &[(&str, &str)],
    ) -> Result<RetryConfig, EnvConfigError<RetryConfigError>> {
        super::Builder::default()
            .configure(&ProviderConfig::no_configuration().with_env(Env::from_slice(vars)))
            .try_retry_config()
            .await
    }

    #[tokio::test]
    async fn test_returns_default_retry_config_from_empty_profile() {
        let env = Env::from_slice(&[("AWS_CONFIG_FILE", "config")]);
        let fs = Fs::from_slice(&[("config", "[default]\n")]);

        let provider_config = ProviderConfig::no_configuration().with_env(env).with_fs(fs);

        let actual_retry_config = super::default_provider()
            .configure(&provider_config)
            .retry_config()
            .await;

        let expected_retry_config = RetryConfig::standard();

        assert_eq!(actual_retry_config, expected_retry_config);
        // This is redundant, but it's really important to make sure that
        // we're setting these exact values by default, so we check twice
        assert_eq!(actual_retry_config.max_attempts(), 3);
        assert_eq!(actual_retry_config.mode(), RetryMode::Standard);
    }

    #[tokio::test]
    async fn test_no_retry_config_in_empty_profile() {
        let env = Env::from_slice(&[("AWS_CONFIG_FILE", "config")]);
        let fs = Fs::from_slice(&[("config", "[default]\n")]);

        let provider_config = ProviderConfig::no_configuration().with_env(env).with_fs(fs);

        let actual_retry_config = super::default_provider()
            .configure(&provider_config)
            .retry_config()
            .await;

        let expected_retry_config = RetryConfig::standard();

        assert_eq!(actual_retry_config, expected_retry_config)
    }

    #[tokio::test]
    async fn test_creation_of_retry_config_from_profile() {
        let env = Env::from_slice(&[("AWS_CONFIG_FILE", "config")]);
        // TODO(https://github.com/awslabs/aws-sdk-rust/issues/247): standard is the default mode;
        // this test would be better if it was setting it to adaptive mode
        // adaptive mode is currently unsupported so that would panic
        let fs = Fs::from_slice(&[(
            "config",
            // If the lines with the vars have preceding spaces, they don't get read
            r#"[default]
max_attempts = 1
retry_mode = standard
            "#,
        )]);

        let provider_config = ProviderConfig::no_configuration().with_env(env).with_fs(fs);

        let actual_retry_config = super::default_provider()
            .configure(&provider_config)
            .retry_config()
            .await;

        let expected_retry_config = RetryConfig::standard().with_max_attempts(1);

        assert_eq!(actual_retry_config, expected_retry_config)
    }

    #[tokio::test]
    async fn test_env_retry_config_takes_precedence_over_profile_retry_config() {
        let env = Env::from_slice(&[
            ("AWS_CONFIG_FILE", "config"),
            ("AWS_MAX_ATTEMPTS", "42"),
            ("AWS_RETRY_MODE", "standard"),
        ]);
        // TODO(https://github.com/awslabs/aws-sdk-rust/issues/247) standard is the default mode;
        // this test would be better if it was setting it to adaptive mode
        // adaptive mode is currently unsupported so that would panic
        let fs = Fs::from_slice(&[(
            "config",
            // If the lines with the vars have preceding spaces, they don't get read
            r#"[default]
max_attempts = 88
retry_mode = standard
            "#,
        )]);

        let provider_config = ProviderConfig::no_configuration().with_env(env).with_fs(fs);

        let actual_retry_config = super::default_provider()
            .configure(&provider_config)
            .retry_config()
            .await;

        let expected_retry_config = RetryConfig::standard().with_max_attempts(42);

        assert_eq!(actual_retry_config, expected_retry_config)
    }

    #[tokio::test]
    #[should_panic = "failed to parse max attempts. source: global profile (`default`) key: `max_attempts`: invalid digit found in string"]
    async fn test_invalid_profile_retry_config_panics() {
        let env = Env::from_slice(&[("AWS_CONFIG_FILE", "config")]);
        let fs = Fs::from_slice(&[(
            "config",
            // If the lines with the vars have preceding spaces, they don't get read
            r#"[default]
max_attempts = potato
            "#,
        )]);

        let provider_config = ProviderConfig::no_configuration().with_env(env).with_fs(fs);

        let _ = super::default_provider()
            .configure(&provider_config)
            .retry_config()
            .await;
    }

    #[tokio::test]
    async fn defaults() {
        let built = test_provider(&[]).await.unwrap();

        assert_eq!(built.mode(), RetryMode::Standard);
        assert_eq!(built.max_attempts(), 3);
    }

    #[tokio::test]
    async fn max_attempts_is_read_correctly() {
        assert_eq!(
            test_provider(&[(env::MAX_ATTEMPTS, "88")]).await.unwrap(),
            RetryConfig::standard().with_max_attempts(88)
        );
    }

    #[tokio::test]
    async fn max_attempts_errors_when_it_cant_be_parsed_as_an_integer() {
        assert!(matches!(
            test_provider(&[(env::MAX_ATTEMPTS, "not an integer")])
                .await
                .unwrap_err()
                .err(),
            RetryConfigError {
                kind: RetryConfigErrorKind::FailedToParseMaxAttempts { .. }
            }
        ));
    }

    #[tokio::test]
    async fn retry_mode_is_read_correctly() {
        assert_eq!(
            test_provider(&[(env::RETRY_MODE, "standard")])
                .await
                .unwrap(),
            RetryConfig::standard()
        );
    }

    #[tokio::test]
    async fn both_fields_can_be_set_at_once() {
        assert_eq!(
            test_provider(&[(env::RETRY_MODE, "standard"), (env::MAX_ATTEMPTS, "13")])
                .await
                .unwrap(),
            RetryConfig::standard().with_max_attempts(13)
        );
    }

    #[tokio::test]
    async fn disallow_zero_max_attempts() {
        let err = test_provider(&[(env::MAX_ATTEMPTS, "0")])
            .await
            .unwrap_err();
        let err = err.err();
        assert!(matches!(
            err,
            RetryConfigError {
                kind: RetryConfigErrorKind::MaxAttemptsMustNotBeZero { .. }
            }
        ));
    }
}
