/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_credential_types::attributes::AccountId;
use aws_credential_types::provider::{self, error::CredentialsError};
use aws_credential_types::Credentials as AwsCredentials;
use aws_sdk_sts::types::{AssumedRoleUser, Credentials as StsCredentials};

use std::time::{SystemTime, UNIX_EPOCH};

/// Convert STS credentials to aws_auth::Credentials
pub(crate) fn into_credentials(
    sts_credentials: Option<StsCredentials>,
    assumed_role_user: Option<AssumedRoleUser>,
    provider_name: &'static str,
) -> provider::Result {
    let sts_credentials = sts_credentials
        .ok_or_else(|| CredentialsError::unhandled("STS credentials must be defined"))?;
    let expiration = SystemTime::try_from(sts_credentials.expiration).map_err(|_| {
        CredentialsError::unhandled(
            "credential expiration time cannot be represented by a SystemTime",
        )
    })?;
    let mut builder = AwsCredentials::builder()
        .access_key_id(sts_credentials.access_key_id)
        .secret_access_key(sts_credentials.secret_access_key)
        .session_token(sts_credentials.session_token)
        .expiry(expiration)
        .provider_name(provider_name);
    if let Some(AssumedRoleUser { arn, .. }) = assumed_role_user {
        builder.set_account_id(Some(parse_account_id(&arn)?));
    }
    Ok(builder.build())
}

/// Create a default STS session name
///
/// STS Assume Role providers MUST assign a name to their generated session. When a user does not
/// provide a name for the session, the provider will choose a name composed of a base + a timestamp,
/// e.g. `profile-file-provider-123456789`
pub(crate) fn default_session_name(base: &str, ts: SystemTime) -> String {
    let now = ts.duration_since(UNIX_EPOCH).expect("post epoch");
    format!("{}-{}", base, now.as_millis())
}

// A subset of functionality extracted from `endpoint_lib::arn::Arn::parse`.
// `Arn` is `pub(crate)` within generated SDKs, making it inaccessible from `aws-config`.
// As a result, a subset is inlined here, with less defensive verification
// since it only deals with the string-form ARN returned by STS.
//
// TODO(https://github.com/smithy-lang/smithy-rs/issues/4090): Consider making a `pub` Arn parser
fn parse_account_id(arn: &str) -> Result<AccountId, CredentialsError> {
    let mut split = arn.splitn(6, ':');
    let invalid_format =
        || CredentialsError::unhandled("ARN must have 6 components delimited by `:`");
    let _arn = split.next().ok_or_else(invalid_format)?;
    let _partition = split.next().ok_or_else(invalid_format)?;
    let _service = split.next().ok_or_else(invalid_format)?;
    let _region = split.next().ok_or_else(invalid_format)?;
    let account_id = split.next().ok_or_else(invalid_format)?;

    Ok(account_id.into())
}
