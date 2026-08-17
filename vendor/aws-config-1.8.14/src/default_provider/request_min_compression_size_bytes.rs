/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::environment::parse_uint;
use crate::provider_config::ProviderConfig;
use aws_runtime::env_config::EnvConfigValue;
use aws_smithy_types::error::display::DisplayErrorContext;

mod env {
    pub(super) const REQUEST_MIN_COMPRESSION_SIZE_BYTES: &str =
        "AWS_REQUEST_MIN_COMPRESSION_SIZE_BYTES";
}

mod profile_key {
    pub(super) const REQUEST_MIN_COMPRESSION_SIZE_BYTES: &str =
        "request_min_compression_size_bytes";
}

/// Load the value for "request minimum compression size bytes".
///
/// This checks the following sources:
/// 1. The environment variable `AWS_REQUEST_MIN_COMPRESSION_SIZE_BYTES=10240`
/// 2. The profile key `request_min_compression_size_bytes=10240`
///
/// If invalid values are found, the provider will return None and an error will be logged.
pub(crate) async fn request_min_compression_size_bytes_provider(
    provider_config: &ProviderConfig,
) -> Option<u32> {
    let env = provider_config.env();
    let profiles = provider_config.profile().await;

    EnvConfigValue::new()
        .env(env::REQUEST_MIN_COMPRESSION_SIZE_BYTES)
        .profile(profile_key::REQUEST_MIN_COMPRESSION_SIZE_BYTES)
        .validate(&env, profiles, parse_uint)
        .map_err(
            |err| tracing::warn!(err = %DisplayErrorContext(&err), "invalid value for `request minimum compression size bytes` setting"),
        )
        .unwrap_or(None)
}

#[cfg(test)]
mod test {
    use super::request_min_compression_size_bytes_provider;
    #[allow(deprecated)]
    use crate::profile::profile_file::{ProfileFileKind, ProfileFiles};
    use crate::provider_config::ProviderConfig;
    use aws_types::os_shim_internal::{Env, Fs};
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn log_error_on_invalid_value() {
        let conf = ProviderConfig::empty().with_env(Env::from_slice(&[(
            "AWS_REQUEST_MIN_COMPRESSION_SIZE_BYTES",
            "not-a-uint",
        )]));
        assert_eq!(
            request_min_compression_size_bytes_provider(&conf).await,
            None
        );
        assert!(logs_contain(
            "invalid value for `request minimum compression size bytes` setting"
        ));
        assert!(logs_contain("AWS_REQUEST_MIN_COMPRESSION_SIZE_BYTES"));
    }

    #[tokio::test]
    #[traced_test]
    async fn environment_priority() {
        let conf = ProviderConfig::empty()
            .with_env(Env::from_slice(&[(
                "AWS_REQUEST_MIN_COMPRESSION_SIZE_BYTES",
                "99",
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
                "[default]\nrequest_min_compression_size_bytes = 100",
            )]));
        assert_eq!(
            request_min_compression_size_bytes_provider(&conf).await,
            Some(99)
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
                "[default]\nrequest_min_compression_size_bytes = 22",
            )]));
        assert_eq!(
            request_min_compression_size_bytes_provider(&conf).await,
            Some(22)
        );
    }
}
