/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Flattened Representation of an AssumeRole chain
//!
//! Assume Role credentials in profile files can chain together credentials from multiple
//! different providers with subsequent credentials being used to configure subsequent providers.
//!
//! This module can parse and resolve the profile chain into a flattened representation with
//! 1-credential-per row (as opposed to a direct profile file representation which can combine
//! multiple actions into the same profile).

use crate::profile::credentials::ProfileFileError;
use crate::profile::{Profile, ProfileSet};
use crate::sensitive_command::CommandWithSensitiveArgs;
use aws_credential_types::attributes::AccountId;
use aws_credential_types::Credentials;

/// Chain of Profile Providers
///
/// Within a profile file, a chain of providers is produced. Starting with a base provider,
/// subsequent providers use the credentials from previous providers to perform their task.
///
/// ProfileChain is a direct representation of the Profile. It can contain named providers
/// that don't actually have implementations.
#[derive(Debug)]
pub(crate) struct ProfileChain<'a> {
    pub(crate) base: BaseProvider<'a>,
    pub(crate) chain: Vec<RoleArn<'a>>,
}

impl<'a> ProfileChain<'a> {
    pub(crate) fn base(&self) -> &BaseProvider<'a> {
        &self.base
    }

    pub(crate) fn chain(&self) -> &[RoleArn<'a>] {
        self.chain.as_slice()
    }
}

/// A base member of the profile chain
///
/// Base providers do not require input credentials to provide their own credentials,
/// e.g. IMDS, ECS, Environment variables
#[derive(Clone, Debug)]
#[non_exhaustive]
pub(crate) enum BaseProvider<'a> {
    /// A profile that specifies a named credential source
    /// Eg: `credential_source = Ec2InstanceMetadata`
    ///
    /// The following profile produces two separate `ProfileProvider` rows:
    /// 1. `BaseProvider::NamedSource("Ec2InstanceMetadata")`
    /// 2. `RoleArn { role_arn: "...", ... }
    /// ```ini
    /// [profile assume-role]
    /// role_arn = arn:aws:iam::123456789:role/MyRole
    /// credential_source = Ec2InstanceMetadata
    /// ```
    NamedSource(&'a str),

    /// A profile with explicitly configured access keys
    ///
    /// Example
    /// ```ini
    /// [profile C]
    /// aws_access_key_id = abc123
    /// aws_secret_access_key = def456
    /// ```
    AccessKey(Credentials),

    WebIdentityTokenRole {
        role_arn: &'a str,
        web_identity_token_file: &'a str,
        session_name: Option<&'a str>,
    },

    /// An SSO Provider
    Sso {
        sso_session_name: Option<&'a str>,
        sso_region: &'a str,
        sso_start_url: &'a str,

        // Credentials from SSO fields
        sso_account_id: Option<&'a str>,
        sso_role_name: Option<&'a str>,
    },

    /// A profile that specifies a `credential_process`
    /// ```ini
    /// [profile assume-role]
    /// credential_process = /opt/bin/awscreds-custom --username helen
    /// ```
    CredentialProcess {
        command_with_sensitive_args: CommandWithSensitiveArgs<&'a str>,
        // The account ID that the credential process falls back to
        // if the process execution result does not provide one.
        account_id: Option<&'a str>,
    },

    /// A profile that specifies an active console session vended by AWS Sign-In.
    /// ```ini
    /// [profile console]
    /// login_session = arn:aws:iam::0123456789012:user/Admin
    /// ```
    LoginSession { login_session_arn: &'a str },
}

/// A profile that specifies a role to assume
///
/// A RoleArn can only be created from either a profile with `source_profile`
/// or one with `credential_source`.
#[derive(Debug)]
pub(crate) struct RoleArn<'a> {
    /// Role to assume
    pub(crate) role_arn: &'a str,
    /// external_id parameter to pass to the assume role provider
    pub(crate) external_id: Option<&'a str>,

    /// session name parameter to pass to the assume role provider
    pub(crate) session_name: Option<&'a str>,
}

/// Resolve a ProfileChain from a ProfileSet or return an error
pub(crate) fn resolve_chain(
    profile_set: &ProfileSet,
) -> Result<ProfileChain<'_>, ProfileFileError> {
    // If there are no profiles, allow flowing into the next provider
    if profile_set.is_empty() {
        return Err(ProfileFileError::NoProfilesDefined);
    }

    // If:
    // - There is no explicit profile override
    // - We're looking for the default profile (no configuration)
    // - There is no default profile
    // Then:
    // - Treat this situation as if no profiles were defined
    if profile_set.selected_profile() == "default" && profile_set.get_profile("default").is_none() {
        tracing::debug!("No default profile defined");
        return Err(ProfileFileError::NoProfilesDefined);
    }
    let mut source_profile_name = profile_set.selected_profile();
    let mut visited_profiles = vec![];
    let mut chain = vec![];
    let base = loop {
        // Get the next profile in the chain
        let profile = profile_set.get_profile(source_profile_name).ok_or(
            ProfileFileError::MissingProfile {
                profile: source_profile_name.into(),
                message: format!(
                    "could not find source profile {} referenced from {}",
                    source_profile_name,
                    visited_profiles.last().unwrap_or(&"the root profile")
                )
                .into(),
            },
        )?;
        // If the profile we just got is one we've already seen, we're in a loop and
        // need to break out with a CredentialLoop error
        if visited_profiles.contains(&source_profile_name) {
            return Err(ProfileFileError::CredentialLoop {
                profiles: visited_profiles
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect(),
                next: source_profile_name.to_string(),
            });
        }
        // otherwise, store the name of the profile in case we see it again later
        visited_profiles.push(source_profile_name);
        // After the first item in the chain, we will prioritize static credentials if they exist
        if visited_profiles.len() > 1 {
            let try_static = static_creds_from_profile(profile);
            if let Ok(static_credentials) = try_static {
                break BaseProvider::AccessKey(static_credentials);
            }
        }

        let next_profile = {
            // The existence of a `role_arn` is the only signal that multiple profiles will be chained.
            // We check for one here and then process the profile accordingly as either a "chain provider"
            // or a "base provider"
            if let Some(role_provider) = role_arn_from_profile(profile) {
                let next = chain_provider(profile)?;
                chain.push(role_provider);
                next
            } else {
                break base_provider(profile_set, profile).map_err(|err| {
                    // It's possible for base_provider to return a `ProfileFileError::ProfileDidNotContainCredentials`
                    // if we're still looking at the first provider we want to surface it. However,
                    // if we're looking at any provider after the first we want to instead return a `ProfileFileError::InvalidCredentialSource`
                    if visited_profiles.len() == 1 {
                        err
                    } else {
                        ProfileFileError::InvalidCredentialSource {
                            profile: profile.name().into(),
                            message: format!("could not load source profile: {err}").into(),
                        }
                    }
                })?;
            }
        };

        match next_profile {
            NextProfile::SelfReference => {
                // self referential profile, don't go through the loop because it will error
                // on the infinite loop check. Instead, reload this profile as a base profile
                // and exit.
                break base_provider(profile_set, profile)?;
            }
            NextProfile::Named(name) => source_profile_name = name,
        }
    };
    chain.reverse();
    Ok(ProfileChain { base, chain })
}

mod role {
    pub(super) const ROLE_ARN: &str = "role_arn";
    pub(super) const EXTERNAL_ID: &str = "external_id";
    pub(super) const SESSION_NAME: &str = "role_session_name";

    pub(super) const CREDENTIAL_SOURCE: &str = "credential_source";
    pub(super) const SOURCE_PROFILE: &str = "source_profile";
}

mod sso {
    pub(super) const ACCOUNT_ID: &str = "sso_account_id";
    pub(super) const REGION: &str = "sso_region";
    pub(super) const ROLE_NAME: &str = "sso_role_name";
    pub(super) const START_URL: &str = "sso_start_url";
    pub(super) const SESSION_NAME: &str = "sso_session";
}

mod web_identity_token {
    pub(super) const TOKEN_FILE: &str = "web_identity_token_file";
}

mod static_credentials {
    pub(super) const AWS_ACCESS_KEY_ID: &str = "aws_access_key_id";
    pub(super) const AWS_SECRET_ACCESS_KEY: &str = "aws_secret_access_key";
    pub(super) const AWS_SESSION_TOKEN: &str = "aws_session_token";
    pub(super) const AWS_ACCOUNT_ID: &str = "aws_account_id";
}

mod credential_process {
    pub(super) const CREDENTIAL_PROCESS: &str = "credential_process";
}

mod login_session {
    pub(super) const LOGIN_SESSION: &str = "login_session";
}

const PROVIDER_NAME: &str = "ProfileFile";

fn base_provider<'a>(
    profile_set: &'a ProfileSet,
    profile: &'a Profile,
) -> Result<BaseProvider<'a>, ProfileFileError> {
    // the profile must define either a `CredentialsSource` or a concrete set of access keys
    match profile.get(role::CREDENTIAL_SOURCE) {
        Some(source) => Ok(BaseProvider::NamedSource(source)),
        None => web_identity_token_from_profile(profile)
            .or_else(|| sso_from_profile(profile_set, profile).transpose())
            .or_else(|| login_session_from_profile(profile))
            .or_else(|| credential_process_from_profile(profile))
            .unwrap_or_else(|| Ok(BaseProvider::AccessKey(static_creds_from_profile(profile)?))),
    }
}

enum NextProfile<'a> {
    SelfReference,
    Named(&'a str),
}

fn chain_provider(profile: &Profile) -> Result<NextProfile<'_>, ProfileFileError> {
    let (source_profile, credential_source) = (
        profile.get(role::SOURCE_PROFILE),
        profile.get(role::CREDENTIAL_SOURCE),
    );
    match (source_profile, credential_source) {
        (Some(_), Some(_)) => Err(ProfileFileError::InvalidCredentialSource {
            profile: profile.name().to_string(),
            message: "profile contained both source_profile and credential_source. \
                Only one or the other can be defined"
                .into(),
        }),
        (None, None) => Err(ProfileFileError::InvalidCredentialSource {
            profile: profile.name().to_string(),
            message:
            "profile must contain `source_profile` or `credential_source` but neither were defined"
                .into(),
        }),
        (Some(source_profile), None) if source_profile == profile.name() => {
            Ok(NextProfile::SelfReference)
        }
        (Some(source_profile), None) => Ok(NextProfile::Named(source_profile)),
        // we want to loop back into this profile and pick up the credential source
        (None, Some(_credential_source)) => Ok(NextProfile::SelfReference),
    }
}

fn role_arn_from_profile(profile: &Profile) -> Option<RoleArn<'_>> {
    // Web Identity Tokens are root providers, not chained roles
    if profile.get(web_identity_token::TOKEN_FILE).is_some() {
        return None;
    }
    let role_arn = profile.get(role::ROLE_ARN)?;
    let session_name = profile.get(role::SESSION_NAME);
    let external_id = profile.get(role::EXTERNAL_ID);
    Some(RoleArn {
        role_arn,
        external_id,
        session_name,
    })
}

fn sso_from_profile<'a>(
    profile_set: &'a ProfileSet,
    profile: &'a Profile,
) -> Result<Option<BaseProvider<'a>>, ProfileFileError> {
    /*
    -- Sample without sso-session: --

    [profile sample-profile]
    sso_account_id = 012345678901
    sso_region = us-east-1
    sso_role_name = SampleRole
    sso_start_url = https://d-abc123.awsapps.com/start-beta

    -- Sample with sso-session: --

    [profile sample-profile]
    sso_session = dev
    sso_account_id = 012345678901
    sso_role_name = SampleRole

    [sso-session dev]
    sso_region = us-east-1
    sso_start_url = https://d-abc123.awsapps.com/start-beta
    */
    let sso_account_id = profile.get(sso::ACCOUNT_ID);
    let mut sso_region = profile.get(sso::REGION);
    let sso_role_name = profile.get(sso::ROLE_NAME);
    let mut sso_start_url = profile.get(sso::START_URL);
    let sso_session_name = profile.get(sso::SESSION_NAME);
    if [
        sso_account_id,
        sso_region,
        sso_role_name,
        sso_start_url,
        sso_session_name,
    ]
    .iter()
    .all(Option::is_none)
    {
        return Ok(None);
    }

    let invalid_sso_config = |s: &str| ProfileFileError::InvalidSsoConfig {
        profile: profile.name().into(),
        message: format!(
            "`{s}` can only be specified in the [sso-session] config when a session name is given"
        )
        .into(),
    };
    if let Some(sso_session_name) = sso_session_name {
        if sso_start_url.is_some() {
            return Err(invalid_sso_config(sso::START_URL));
        }
        if sso_region.is_some() {
            return Err(invalid_sso_config(sso::REGION));
        }
        if let Some(session) = profile_set.sso_session(sso_session_name) {
            sso_start_url = session.get(sso::START_URL);
            sso_region = session.get(sso::REGION);
        } else {
            return Err(ProfileFileError::MissingSsoSession {
                profile: profile.name().into(),
                sso_session: sso_session_name.into(),
            });
        }
    }

    let invalid_sso_creds = |left: &str, right: &str| ProfileFileError::InvalidSsoConfig {
        profile: profile.name().into(),
        message: format!("if `{left}` is set, then `{right}` must also be set").into(),
    };
    match (sso_account_id, sso_role_name) {
        (Some(_), Some(_)) | (None, None) => { /* good */ }
        (Some(_), None) => return Err(invalid_sso_creds(sso::ACCOUNT_ID, sso::ROLE_NAME)),
        (None, Some(_)) => return Err(invalid_sso_creds(sso::ROLE_NAME, sso::ACCOUNT_ID)),
    }

    let missing_field = |s| move || ProfileFileError::missing_field(profile, s);
    let sso_region = sso_region.ok_or_else(missing_field(sso::REGION))?;
    let sso_start_url = sso_start_url.ok_or_else(missing_field(sso::START_URL))?;
    Ok(Some(BaseProvider::Sso {
        sso_account_id,
        sso_region,
        sso_role_name,
        sso_start_url,
        sso_session_name,
    }))
}

fn web_identity_token_from_profile(
    profile: &Profile,
) -> Option<Result<BaseProvider<'_>, ProfileFileError>> {
    let session_name = profile.get(role::SESSION_NAME);
    match (
        profile.get(role::ROLE_ARN),
        profile.get(web_identity_token::TOKEN_FILE),
    ) {
        (Some(role_arn), Some(token_file)) => Some(Ok(BaseProvider::WebIdentityTokenRole {
            role_arn,
            web_identity_token_file: token_file,
            session_name,
        })),
        (None, None) => None,
        (Some(_role_arn), None) => None,
        (None, Some(_token_file)) => Some(Err(ProfileFileError::InvalidCredentialSource {
            profile: profile.name().to_string(),
            message: "`web_identity_token_file` was specified but `role_arn` was missing".into(),
        })),
    }
}

/// Load static credentials from a profile
///
/// Example:
/// ```ini
/// [profile B]
/// aws_access_key_id = abc123
/// aws_secret_access_key = def456
/// ```
fn static_creds_from_profile(profile: &Profile) -> Result<Credentials, ProfileFileError> {
    use static_credentials::*;
    let access_key = profile.get(AWS_ACCESS_KEY_ID);
    let secret_key = profile.get(AWS_SECRET_ACCESS_KEY);
    let session_token = profile.get(AWS_SESSION_TOKEN);
    // If all three fields are missing return a `ProfileFileError::ProfileDidNotContainCredentials`
    if let (None, None, None) = (access_key, secret_key, session_token) {
        return Err(ProfileFileError::ProfileDidNotContainCredentials {
            profile: profile.name().to_string(),
        });
    }
    // Otherwise, check to make sure the access and secret keys are defined
    let access_key = access_key.ok_or_else(|| ProfileFileError::InvalidCredentialSource {
        profile: profile.name().to_string(),
        message: "profile missing aws_access_key_id".into(),
    })?;
    let secret_key = secret_key.ok_or_else(|| ProfileFileError::InvalidCredentialSource {
        profile: profile.name().to_string(),
        message: "profile missing aws_secret_access_key".into(),
    })?;
    // There might not be an active session token so we don't error out if it's missing
    let mut builder = Credentials::builder()
        .access_key_id(access_key)
        .secret_access_key(secret_key)
        .provider_name(PROVIDER_NAME);
    builder.set_session_token(session_token.map(String::from));
    builder.set_account_id(profile.get(AWS_ACCOUNT_ID).map(AccountId::from));
    Ok(builder.build())
}

/// Load credentials from `credential_process`
///
/// Example:
/// ```ini
/// [profile B]
/// credential_process = /opt/bin/awscreds-custom --username helen
/// ```
fn credential_process_from_profile(
    profile: &Profile,
) -> Option<Result<BaseProvider<'_>, ProfileFileError>> {
    let account_id = profile.get(static_credentials::AWS_ACCOUNT_ID);
    profile
        .get(credential_process::CREDENTIAL_PROCESS)
        .map(|credential_process| {
            Ok(BaseProvider::CredentialProcess {
                command_with_sensitive_args: CommandWithSensitiveArgs::new(credential_process),
                account_id,
            })
        })
}

/// Load credentials from `login_session`
///
/// Example:
/// ```ini
/// [profile console]
/// login_session = arn:aws:iam::0123456789012:user/Admin
/// ```
fn login_session_from_profile(
    profile: &Profile,
) -> Option<Result<BaseProvider<'_>, ProfileFileError>> {
    profile
        .get(login_session::LOGIN_SESSION)
        .map(|login_session_arn| Ok(BaseProvider::LoginSession { login_session_arn }))
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "test-util")]
    use super::ProfileChain;
    use crate::profile::credentials::repr::BaseProvider;
    use crate::sensitive_command::CommandWithSensitiveArgs;
    use serde::Deserialize;
    #[cfg(feature = "test-util")]
    use std::collections::HashMap;

    #[cfg(feature = "test-util")]
    #[test]
    fn run_test_cases() -> Result<(), Box<dyn std::error::Error>> {
        let test_cases: Vec<TestCase> = serde_json::from_str(&std::fs::read_to_string(
            "./test-data/assume-role-tests.json",
        )?)?;
        for test_case in test_cases {
            print!("checking: {}...", test_case.docs);
            check(test_case);
            println!("ok")
        }
        Ok(())
    }

    #[cfg(feature = "test-util")]
    fn check(test_case: TestCase) {
        use super::resolve_chain;
        use aws_runtime::env_config::property::Properties;
        use aws_runtime::env_config::section::EnvConfigSections;
        let source = EnvConfigSections::new(
            test_case.input.profiles,
            test_case.input.selected_profile,
            test_case.input.sso_sessions,
            Properties::new(),
        );
        let actual = resolve_chain(&source);
        let expected = test_case.output;
        match (expected, actual) {
            (TestOutput::Error(s), Err(e)) => assert!(
                format!("{}", e).contains(&s),
                "expected\n{}\nto contain\n{}\n",
                e,
                s
            ),
            (TestOutput::ProfileChain(expected), Ok(actual)) => {
                assert_eq!(to_test_output(actual), expected)
            }
            (expected, actual) => panic!(
                "error/success mismatch. Expected:\n {:?}\nActual:\n {:?}",
                &expected, actual
            ),
        }
    }

    #[derive(Deserialize)]
    #[cfg(feature = "test-util")]
    struct TestCase {
        docs: String,
        input: TestInput,
        output: TestOutput,
    }

    #[derive(Deserialize)]
    #[cfg(feature = "test-util")]
    struct TestInput {
        profiles: HashMap<String, HashMap<String, String>>,
        selected_profile: String,
        #[serde(default)]
        sso_sessions: HashMap<String, HashMap<String, String>>,
    }

    #[cfg(feature = "test-util")]
    fn to_test_output(profile_chain: ProfileChain<'_>) -> Vec<Provider> {
        let mut output = vec![];
        match profile_chain.base {
            BaseProvider::NamedSource(name) => output.push(Provider::NamedSource(name.into())),
            BaseProvider::AccessKey(creds) => output.push(Provider::AccessKey {
                access_key_id: creds.access_key_id().into(),
                secret_access_key: creds.secret_access_key().into(),
                session_token: creds.session_token().map(|tok| tok.to_string()),
                account_id: creds.account_id().map(|id| id.as_str().to_string()),
            }),
            BaseProvider::CredentialProcess {
                command_with_sensitive_args,
                account_id,
            } => output.push(Provider::CredentialProcess {
                command: command_with_sensitive_args.unredacted().into(),
                account_id: account_id.map(|id| id.to_string()),
            }),
            BaseProvider::WebIdentityTokenRole {
                role_arn,
                web_identity_token_file,
                session_name,
            } => output.push(Provider::WebIdentityToken {
                role_arn: role_arn.into(),
                web_identity_token_file: web_identity_token_file.into(),
                role_session_name: session_name.map(|sess| sess.to_string()),
            }),
            BaseProvider::Sso {
                sso_region,
                sso_start_url,
                sso_session_name,
                sso_account_id,
                sso_role_name,
            } => output.push(Provider::Sso {
                sso_region: sso_region.into(),
                sso_start_url: sso_start_url.into(),
                sso_session: sso_session_name.map(|s| s.to_string()),
                sso_account_id: sso_account_id.map(|s| s.to_string()),
                sso_role_name: sso_role_name.map(|s| s.to_string()),
            }),
            BaseProvider::LoginSession { login_session_arn } => {
                output.push(Provider::LoginSession {
                    login_session_arn: login_session_arn.into(),
                })
            }
        };
        for role in profile_chain.chain {
            output.push(Provider::AssumeRole {
                role_arn: role.role_arn.into(),
                external_id: role.external_id.map(ToString::to_string),
                role_session_name: role.session_name.map(ToString::to_string),
            })
        }
        output
    }

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[allow(dead_code)]
    enum TestOutput {
        ProfileChain(Vec<Provider>),
        Error(String),
    }

    #[derive(Deserialize, Debug, Eq, PartialEq)]
    #[allow(dead_code)]
    enum Provider {
        AssumeRole {
            role_arn: String,
            external_id: Option<String>,
            role_session_name: Option<String>,
        },
        AccessKey {
            access_key_id: String,
            secret_access_key: String,
            session_token: Option<String>,
            account_id: Option<String>,
        },
        NamedSource(String),
        CredentialProcess {
            command: String,
            account_id: Option<String>,
        },
        WebIdentityToken {
            role_arn: String,
            web_identity_token_file: String,
            role_session_name: Option<String>,
        },
        Sso {
            sso_region: String,
            sso_start_url: String,
            sso_session: Option<String>,

            sso_account_id: Option<String>,
            sso_role_name: Option<String>,
        },
        LoginSession {
            login_session_arn: String,
        },
    }

    #[test]
    fn base_provider_process_credentials_args_redaction() {
        assert_eq!(
            r#"CredentialProcess { command_with_sensitive_args: "program", account_id: None }"#,
            format!(
                "{:?}",
                BaseProvider::CredentialProcess {
                    command_with_sensitive_args: CommandWithSensitiveArgs::new("program"),
                    account_id: None,
                }
            )
        );
        assert_eq!(
            r#"CredentialProcess { command_with_sensitive_args: "program ** arguments redacted **", account_id: None }"#,
            format!(
                "{:?}",
                BaseProvider::CredentialProcess {
                    command_with_sensitive_args: CommandWithSensitiveArgs::new("program arg1 arg2"),
                    account_id: None
                }
            )
        );
        assert_eq!(
            r#"CredentialProcess { command_with_sensitive_args: "program ** arguments redacted **", account_id: None }"#,
            format!(
                "{:?}",
                BaseProvider::CredentialProcess {
                    command_with_sensitive_args: CommandWithSensitiveArgs::new(
                        "program\targ1 arg2"
                    ),
                    account_id: None
                }
            )
        );
    }
}
