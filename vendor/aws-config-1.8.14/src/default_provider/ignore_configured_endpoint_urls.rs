/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::environment::parse_bool;
use crate::provider_config::ProviderConfig;
use aws_runtime::env_config::EnvConfigValue;
use aws_smithy_types::error::display::DisplayErrorContext;

mod env {
    pub(super) const IGNORE_CONFIGURED_ENDPOINT_URLS: &str = "AWS_IGNORE_CONFIGURED_ENDPOINT_URLS";
}

mod profile_key {
    pub(super) const IGNORE_CONFIGURED_ENDPOINT_URLS: &str = "ignore_configured_endpoint_urls";
}

/// Load the value for "ignore configured endpoint URLs"
///
/// This checks the following sources:
/// 1. The environment variable `AWS_IGNORE_CONFIGURED_ENDPOINT_URLS_ENDPOINT=true/false`
/// 2. The profile key `ignore_configured_endpoint_urls=true/false`
///
/// If invalid values are found, the provider will return None and an error will be logged.
pub async fn ignore_configured_endpoint_urls_provider(
    provider_config: &ProviderConfig,
) -> Option<bool> {
    let env = provider_config.env();
    let profiles = provider_config.profile().await;

    EnvConfigValue::new()
        .env(env::IGNORE_CONFIGURED_ENDPOINT_URLS)
        .profile(profile_key::IGNORE_CONFIGURED_ENDPOINT_URLS)
        .validate(&env, profiles, parse_bool)
        .map_err(
            |err| tracing::warn!(err = %DisplayErrorContext(&err), "invalid value for 'ignore configured endpoint URLs' setting"),
        )
        .unwrap_or(None)
}

#[cfg(test)]
mod test {
    use super::env;
    use super::ignore_configured_endpoint_urls_provider;
    #[allow(deprecated)]
    use crate::profile::profile_file::{ProfileFileKind, ProfileFiles};
    use crate::provider_config::ProviderConfig;
    use aws_types::os_shim_internal::{Env, Fs};
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn log_error_on_invalid_value() {
        let conf = ProviderConfig::empty().with_env(Env::from_slice(&[(
            env::IGNORE_CONFIGURED_ENDPOINT_URLS,
            "not-a-boolean",
        )]));
        assert_eq!(None, ignore_configured_endpoint_urls_provider(&conf).await,);
        assert!(logs_contain(
            "invalid value for 'ignore configured endpoint URLs' setting"
        ));
        assert!(logs_contain(env::IGNORE_CONFIGURED_ENDPOINT_URLS));
    }

    #[tokio::test]
    #[traced_test]
    async fn environment_priority() {
        let conf = ProviderConfig::empty()
            .with_env(Env::from_slice(&[(
                env::IGNORE_CONFIGURED_ENDPOINT_URLS,
                "TRUE",
            )]))
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
                "[default]\nignore_configured_endpoint_urls = false",
            )]));
        assert_eq!(
            Some(true),
            ignore_configured_endpoint_urls_provider(&conf).await,
        );
    }
}
