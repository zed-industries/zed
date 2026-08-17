/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::environment::parse_bool;
use crate::provider_config::ProviderConfig;
use aws_runtime::env_config::EnvConfigValue;
use aws_smithy_types::error::display::DisplayErrorContext;

mod env {
    pub(super) const USE_FIPS: &str = "AWS_USE_FIPS_ENDPOINT";
}

mod profile_key {
    pub(super) const USE_FIPS: &str = "use_fips_endpoint";
}

/// Load the value for "use FIPS"
///
/// This checks the following sources:
/// 1. The environment variable `AWS_USE_FIPS_ENDPOINT=true/false`
/// 2. The profile key `use_fips_endpoint=true/false`
///
/// If invalid values are found, the provider will return None and an error will be logged.
pub async fn use_fips_provider(provider_config: &ProviderConfig) -> Option<bool> {
    let env = provider_config.env();
    let profiles = provider_config.profile().await;

    EnvConfigValue::new()
        .env(env::USE_FIPS)
        .profile(profile_key::USE_FIPS)
        .validate(&env, profiles, parse_bool)
        .map_err(
            |err| tracing::warn!(err = %DisplayErrorContext(&err), "invalid value for FIPS setting"),
        )
        .unwrap_or(None)
}

#[cfg(test)]
mod test {
    use crate::default_provider::use_fips::use_fips_provider;
    #[allow(deprecated)]
    use crate::profile::profile_file::{ProfileFileKind, ProfileFiles};
    use crate::provider_config::ProviderConfig;
    use aws_types::os_shim_internal::{Env, Fs};
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn log_error_on_invalid_value() {
        let conf = ProviderConfig::empty().with_env(Env::from_slice(&[(
            "AWS_USE_FIPS_ENDPOINT",
            "not-a-boolean",
        )]));
        assert_eq!(use_fips_provider(&conf).await, None);
        assert!(logs_contain("invalid value for FIPS setting"));
        assert!(logs_contain("AWS_USE_FIPS_ENDPOINT"));
    }

    #[tokio::test]
    #[traced_test]
    async fn environment_priority() {
        let conf = ProviderConfig::empty()
            .with_env(Env::from_slice(&[("AWS_USE_FIPS_ENDPOINT", "TRUE")]))
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
                "[default]\nuse_fips_endpoint = false",
            )]));
        assert_eq!(use_fips_provider(&conf).await, Some(true));
    }
}
