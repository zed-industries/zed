/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::environment::parse_url;
use crate::provider_config::ProviderConfig;
use aws_runtime::env_config::EnvConfigValue;
use aws_smithy_types::error::display::DisplayErrorContext;
use aws_types::origin::Origin;

mod env {
    pub(super) const ENDPOINT_URL: &str = "AWS_ENDPOINT_URL";
}

mod profile_key {
    pub(super) const ENDPOINT_URL: &str = "endpoint_url";
}

/// Load the value for an endpoint URL
///
/// This checks the following sources:
/// 1. The environment variable `AWS_ENDPOINT_URL=http://localhost`
/// 2. The profile key `endpoint_url=http://localhost`
///
/// If invalid values are found, the provider will return None and an error will be logged.
pub async fn endpoint_url_provider(provider_config: &ProviderConfig) -> Option<String> {
    let env = provider_config.env();
    let profiles = provider_config.profile().await;

    EnvConfigValue::new()
        .env(env::ENDPOINT_URL)
        .profile(profile_key::ENDPOINT_URL)
        .validate(&env, profiles, parse_url)
        .map_err(
            |err| tracing::warn!(err = %DisplayErrorContext(&err), "invalid value for endpoint URL setting"),
        )
        .unwrap_or(None)
}

/// Load the value for an endpoint URL
///
/// This checks the following sources:
/// 1. The environment variable `AWS_ENDPOINT_URL=http://localhost`
/// 2. The profile key `endpoint_url=http://localhost`
///
/// If invalid values are found, the provider will return None and an error will be logged.
pub async fn endpoint_url_provider_with_origin(
    provider_config: &ProviderConfig,
) -> (Option<String>, Origin) {
    let env = provider_config.env();
    let profiles = provider_config.profile().await;

    EnvConfigValue::new()
        .env(env::ENDPOINT_URL)
        .profile(profile_key::ENDPOINT_URL)
        .validate_and_return_origin(&env, profiles, parse_url)
        .map_err(
            |err| tracing::warn!(err = %DisplayErrorContext(&err), "invalid value for endpoint URL setting"),
        )
        .unwrap_or_default()
}

#[cfg(test)]
mod test {
    use super::endpoint_url_provider;
    use super::env;
    #[allow(deprecated)]
    use crate::profile::profile_file::{ProfileFileKind, ProfileFiles};
    use crate::provider_config::ProviderConfig;
    use aws_types::os_shim_internal::{Env, Fs};
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn log_error_on_invalid_value() {
        let conf =
            ProviderConfig::empty().with_env(Env::from_slice(&[(env::ENDPOINT_URL, "not-a-url")]));
        assert_eq!(None, endpoint_url_provider(&conf).await);
        assert!(logs_contain("invalid value for endpoint URL setting"));
        assert!(logs_contain(env::ENDPOINT_URL));
    }

    #[tokio::test]
    #[traced_test]
    async fn environment_priority() {
        let conf = ProviderConfig::empty()
            .with_env(Env::from_slice(&[(env::ENDPOINT_URL, "http://localhost")]))
            .with_profile_config(
                Some(
                    #[allow(deprecated)]
                    ProfileFiles::builder()
                        .with_file(
                            #[allow(deprecated)]
                            ProfileFileKind::Config,
                            "conf",
                        )
                        .build(),
                ),
                None,
            )
            .with_fs(Fs::from_slice(&[(
                "conf",
                "[default]\nendpoint_url = http://production",
            )]));
        assert_eq!(
            Some("http://localhost".to_owned()),
            endpoint_url_provider(&conf).await,
        );
    }
}
