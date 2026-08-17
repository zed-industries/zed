/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::provider_config::ProviderConfig;
use aws_runtime::env_config::EnvConfigValue;
use aws_smithy_types::error::display::DisplayErrorContext;
use aws_types::sdk_config::{RequestChecksumCalculation, ResponseChecksumValidation};
use std::str::FromStr;

mod env {
    pub(super) const REQUEST_CHECKSUM_CALCULATION: &str = "AWS_REQUEST_CHECKSUM_CALCULATION";
    pub(super) const RESPONSE_CHECKSUM_VALIDATION: &str = "AWS_RESPONSE_CHECKSUM_VALIDATION";
}

mod profile_key {
    pub(super) const REQUEST_CHECKSUM_CALCULATION: &str = "request_checksum_calculation";
    pub(super) const RESPONSE_CHECKSUM_VALIDATION: &str = "response_checksum_validation";
}

/// Load the value for `request_checksum_calculation`
///
/// This checks the following sources:
/// 1. The environment variable `AWS_REQUEST_CHECKSUM_CALCULATION=WHEN_SUPPORTED/WHEN_REQUIRED`
/// 2. The profile key `request_checksum_calculation=WHEN_SUPPORTED/WHEN_REQUIRED`
///
/// If invalid values are found, the provider will return `None` and an error will be logged.
pub async fn request_checksum_calculation_provider(
    provider_config: &ProviderConfig,
) -> Option<RequestChecksumCalculation> {
    let env = provider_config.env();
    let profiles = provider_config.profile().await;

    let loaded = EnvConfigValue::new()
         .env(env::REQUEST_CHECKSUM_CALCULATION)
         .profile(profile_key::REQUEST_CHECKSUM_CALCULATION)
         .validate(&env, profiles, RequestChecksumCalculation::from_str)
         .map_err(
             |err| tracing::warn!(err = %DisplayErrorContext(&err), "invalid value for request_checksum_calculation setting"),
         )
         .unwrap_or(None);

    // request_checksum_calculation should always have a non-None value and the
    // default is WhenSupported
    loaded.or(Some(RequestChecksumCalculation::WhenSupported))
}

/// Load the value for `response_checksum_validation`
///
/// This checks the following sources:
/// 1. The environment variable `AWS_RESPONSE_CHECKSUM_VALIDATION=WHEN_SUPPORTED/WHEN_REQUIRED`
/// 2. The profile key `response_checksum_validation=WHEN_SUPPORTED/WHEN_REQUIRED`
///
/// If invalid values are found, the provider will return `None` and an error will be logged.
pub async fn response_checksum_validation_provider(
    provider_config: &ProviderConfig,
) -> Option<ResponseChecksumValidation> {
    let env = provider_config.env();
    let profiles = provider_config.profile().await;

    let loaded = EnvConfigValue::new()
         .env(env::RESPONSE_CHECKSUM_VALIDATION)
         .profile(profile_key::RESPONSE_CHECKSUM_VALIDATION)
         .validate(&env, profiles, ResponseChecksumValidation::from_str)
         .map_err(
             |err| tracing::warn!(err = %DisplayErrorContext(&err), "invalid value for response_checksum_validation setting"),
         )
         .unwrap_or(None);

    // response_checksum_validation should always have a non-None value and the
    // default is WhenSupported
    loaded.or(Some(ResponseChecksumValidation::WhenSupported))
}

#[cfg(test)]
mod test {
    use crate::default_provider::checksums::{
        request_checksum_calculation_provider, response_checksum_validation_provider,
    };
    #[allow(deprecated)]
    use crate::profile::profile_file::{ProfileFileKind, ProfileFiles};
    use crate::provider_config::ProviderConfig;
    use aws_smithy_types::checksum_config::{
        RequestChecksumCalculation, ResponseChecksumValidation,
    };
    use aws_types::os_shim_internal::{Env, Fs};
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn log_error_on_invalid_value_request() {
        let conf = ProviderConfig::empty().with_env(Env::from_slice(&[(
            "AWS_REQUEST_CHECKSUM_CALCULATION",
            "not-a-valid-value",
        )]));
        assert_eq!(
            request_checksum_calculation_provider(&conf).await,
            Some(RequestChecksumCalculation::WhenSupported)
        );
        assert!(logs_contain(
            "invalid value for request_checksum_calculation setting"
        ));
        assert!(logs_contain("AWS_REQUEST_CHECKSUM_CALCULATION"));
    }

    #[tokio::test]
    #[traced_test]
    async fn environment_priority_request() {
        let conf = ProviderConfig::empty()
            .with_env(Env::from_slice(&[(
                "AWS_REQUEST_CHECKSUM_CALCULATION",
                "WHEN_REQUIRED",
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
                "[default]\nrequest_checksum_calculation = WHEN_SUPPORTED",
            )]));
        assert_eq!(
            request_checksum_calculation_provider(&conf).await,
            Some(RequestChecksumCalculation::WhenRequired)
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn profile_works_request() {
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
                "[default]\nrequest_checksum_calculation = WHEN_REQUIRED",
            )]));
        assert_eq!(
            request_checksum_calculation_provider(&conf).await,
            Some(RequestChecksumCalculation::WhenRequired)
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn default_works_request() {
        let conf = ProviderConfig::empty();
        assert_eq!(
            request_checksum_calculation_provider(&conf).await,
            Some(RequestChecksumCalculation::WhenSupported)
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn log_error_on_invalid_value_response() {
        let conf = ProviderConfig::empty().with_env(Env::from_slice(&[(
            "AWS_RESPONSE_CHECKSUM_VALIDATION",
            "not-a-valid-value",
        )]));
        assert_eq!(
            response_checksum_validation_provider(&conf).await,
            Some(ResponseChecksumValidation::WhenSupported)
        );
        assert!(logs_contain(
            "invalid value for response_checksum_validation setting"
        ));
        assert!(logs_contain("AWS_RESPONSE_CHECKSUM_VALIDATION"));
    }

    #[tokio::test]
    #[traced_test]
    async fn environment_priority_response() {
        let conf = ProviderConfig::empty()
            .with_env(Env::from_slice(&[(
                "AWS_RESPONSE_CHECKSUM_VALIDATION",
                "WHEN_SUPPORTED",
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
                "[default]\response_checksum_validation = WHEN_REQUIRED",
            )]));
        assert_eq!(
            response_checksum_validation_provider(&conf).await,
            Some(ResponseChecksumValidation::WhenSupported)
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn profile_works_response() {
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
                "[default]\nresponse_checksum_validation = WHEN_REQUIRED",
            )]));
        assert_eq!(
            response_checksum_validation_provider(&conf).await,
            Some(ResponseChecksumValidation::WhenRequired)
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn default_works_response() {
        let conf = ProviderConfig::empty();
        assert_eq!(
            response_checksum_validation_provider(&conf).await,
            Some(ResponseChecksumValidation::WhenSupported)
        );
    }
}
