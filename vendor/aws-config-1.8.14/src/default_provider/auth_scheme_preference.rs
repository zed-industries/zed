/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::provider_config::ProviderConfig;
use aws_runtime::env_config::EnvConfigValue;
use aws_smithy_runtime_api::client::auth::{AuthSchemeId, AuthSchemePreference};
use aws_smithy_types::error::display::DisplayErrorContext;
use std::borrow::Cow;
use std::fmt;

mod env {
    pub(super) const AUTH_SCHEME_PREFERENCE: &str = "AWS_AUTH_SCHEME_PREFERENCE";
}

mod profile_key {
    pub(super) const AUTH_SCHEME_PREFERENCE: &str = "auth_scheme_preference";
}

/// Load the value for the auth scheme preference
///
/// This checks the following sources:
/// 1. The environment variable `AWS_AUTH_SCHEME_PREFERENCE=scheme1,scheme2,scheme3`
/// 2. The profile key `auth_scheme_preference=scheme1,scheme2,scheme3`
///
/// A scheme name can be either a fully qualified name or a shorthand with the namespace prefix trimmed.
/// For example, valid scheme names include "aws.auth#sigv4", "smithy.api#httpBasicAuth", "sigv4", and "httpBasicAuth".
/// Whitespace (spaces or tabs), including leading, trailing, and between names, is ignored.
///
/// Returns `None` if a parsed string component is empty when creating an `AuthSchemeId`.
pub(crate) async fn auth_scheme_preference_provider(
    provider_config: &ProviderConfig,
) -> Option<AuthSchemePreference> {
    let env = provider_config.env();
    let profiles = provider_config.profile().await;

    EnvConfigValue::new()
        .env(env::AUTH_SCHEME_PREFERENCE)
        .profile(profile_key::AUTH_SCHEME_PREFERENCE)
        .validate(&env, profiles, parse_auth_scheme_names)
        .map_err(|err| tracing::warn!(err = %DisplayErrorContext(&err), "invalid value for `AuthSchemePreference`"))
        .unwrap_or(None)
}

fn parse_auth_scheme_names(csv: &str) -> Result<AuthSchemePreference, InvalidAuthSchemeNamesCsv> {
    csv.split(',')
        .map(|s| {
            let trimmed = s.trim().replace([' ', '\t'], "");
            if trimmed.is_empty() {
                return Err(InvalidAuthSchemeNamesCsv {
                    value: format!("Empty name found in `{csv}`."),
                });
            }
            let scheme_name = trimmed.split('#').next_back().unwrap_or(&trimmed);
            Ok(AuthSchemeId::from(Cow::Owned(scheme_name.to_owned())))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(AuthSchemePreference::from)
}

#[derive(Debug)]
pub(crate) struct InvalidAuthSchemeNamesCsv {
    value: String,
}

impl fmt::Display for InvalidAuthSchemeNamesCsv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Not a valid comma-separated auth scheme names: {}",
            self.value
        )
    }
}

impl std::error::Error for InvalidAuthSchemeNamesCsv {}

#[cfg(test)]
mod test {
    use super::env;
    use crate::{
        default_provider::auth_scheme_preference::auth_scheme_preference_provider,
        provider_config::ProviderConfig,
    };
    use aws_types::os_shim_internal::Env;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn log_error_on_invalid_value() {
        let conf = ProviderConfig::empty().with_env(Env::from_slice(&[(
            env::AUTH_SCHEME_PREFERENCE,
            "scheme1, , \tscheme2",
        )]));
        assert_eq!(None, auth_scheme_preference_provider(&conf).await);
        assert!(logs_contain(
            "Not a valid comma-separated auth scheme names: Empty name found"
        ));
    }

    #[cfg(feature = "sso")] // for aws-smithy-runtime-api/http-auth
    mod http_auth_tests {
        use super::env;
        #[allow(deprecated)]
        use crate::profile::profile_file::{ProfileFileKind, ProfileFiles};
        use crate::{
            default_provider::auth_scheme_preference::auth_scheme_preference_provider,
            provider_config::ProviderConfig,
        };
        use aws_smithy_runtime_api::client::auth::AuthSchemePreference;
        use aws_types::os_shim_internal::{Env, Fs};

        #[tokio::test]
        async fn environment_priority() {
            let conf = ProviderConfig::empty()
            .with_env(Env::from_slice(&[(
                env::AUTH_SCHEME_PREFERENCE,
                "aws.auth#sigv4, smithy.api#httpBasicAuth, smithy.api#httpDigestAuth, smithy.api#httpBearerAuth, smithy.api#httpApiKeyAuth",
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
                "[default]\nauth_scheme_preference = scheme1, scheme2 , \tscheme3 \t",
            )]));
            assert_eq!(
                AuthSchemePreference::from([
                    aws_runtime::auth::sigv4::SCHEME_ID,
                    aws_smithy_runtime_api::client::auth::http::HTTP_BASIC_AUTH_SCHEME_ID,
                    aws_smithy_runtime_api::client::auth::http::HTTP_DIGEST_AUTH_SCHEME_ID,
                    aws_smithy_runtime_api::client::auth::http::HTTP_BEARER_AUTH_SCHEME_ID,
                    aws_smithy_runtime_api::client::auth::http::HTTP_API_KEY_AUTH_SCHEME_ID,
                ]),
                auth_scheme_preference_provider(&conf).await.unwrap()
            );
        }

        #[tokio::test]
        async fn load_from_profile() {
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
                "[default]\nauth_scheme_preference = sigv4, httpBasicAuth, httpDigestAuth, \thttpBearerAuth \t, httpApiKeyAuth ",
            )]));
            assert_eq!(
                AuthSchemePreference::from([
                    aws_runtime::auth::sigv4::SCHEME_ID,
                    aws_smithy_runtime_api::client::auth::http::HTTP_BASIC_AUTH_SCHEME_ID,
                    aws_smithy_runtime_api::client::auth::http::HTTP_DIGEST_AUTH_SCHEME_ID,
                    aws_smithy_runtime_api::client::auth::http::HTTP_BEARER_AUTH_SCHEME_ID,
                    aws_smithy_runtime_api::client::auth::http::HTTP_API_KEY_AUTH_SCHEME_ID,
                ]),
                auth_scheme_preference_provider(&conf).await.unwrap()
            );
        }
    }
}
