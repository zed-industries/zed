/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::environment::parse_bool;
use crate::provider_config::ProviderConfig;
use aws_runtime::env_config::EnvConfigValue;
use aws_smithy_types::error::display::DisplayErrorContext;

mod env {
    pub(super) const DISABLE_REQUEST_COMPRESSION: &str = "AWS_DISABLE_REQUEST_COMPRESSION";
}

mod profile_key {
    pub(super) const DISABLE_REQUEST_COMPRESSION: &str = "disable_request_compression";
}

/// Load the value for "disable request compression".
///
/// This checks the following sources:
/// 1. The environment variable `AWS_DISABLE_REQUEST_COMPRESSION=true/false`
/// 2. The profile key `disable_request_compression=true/false`
///
/// If invalid values are found, the provider will return None and an error will be logged.
pub(crate) async fn disable_request_compression_provider(
    provider_config: &ProviderConfig,
) -> Option<bool> {
    let env = provider_config.env();
    let profiles = provider_config.profile().await;

    EnvConfigValue::new()
        .env(env::DISABLE_REQUEST_COMPRESSION)
        .profile(profile_key::DISABLE_REQUEST_COMPRESSION)
        .validate(&env, profiles, parse_bool)
        .map_err(
            |err| tracing::warn!(err = %DisplayErrorContext(&err), "invalid value for `disable request compression` setting"),
        )
        .unwrap_or(None)
}

#[cfg(test)]
mod test {
    use super::disable_request_compression_provider;
    #[allow(deprecated)]
    use crate::profile::profile_file::{ProfileFileKind, ProfileFiles};
    use crate::provider_config::ProviderConfig;
    use aws_types::os_shim_internal::{Env, Fs};
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn log_error_on_invalid_value() {
        let conf = ProviderConfig::empty().with_env(Env::from_slice(&[(
            "AWS_DISABLE_REQUEST_COMPRESSION",
            "not-a-boolean",
        )]));
        assert_eq!(disable_request_compression_provider(&conf).await, None);
        assert!(logs_contain(
            "invalid value for `disable request compression` setting"
        ));
        assert!(logs_contain("AWS_DISABLE_REQUEST_COMPRESSION"));
    }

    #[tokio::test]
    #[traced_test]
    async fn environment_priority() {
        let conf = ProviderConfig::empty()
            .with_env(Env::from_slice(&[(
                "AWS_DISABLE_REQUEST_COMPRESSION",
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
                "[default]\ndisable_request_compression = false",
            )]));
        assert_eq!(
            disable_request_compression_provider(&conf).await,
            Some(true)
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn profile_config_works() {
        let conf = ProviderConfig::empty()
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
                "[default]\ndisable_request_compression = true",
            )]));
        assert_eq!(
            disable_request_compression_provider(&conf).await,
            Some(true)
        );
    }
}
