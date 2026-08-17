/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::login::PROVIDER_NAME;
use aws_credential_types::provider::error::CredentialsError;
use aws_credential_types::Credentials;
use aws_sdk_signin::types::CreateOAuth2TokenResponseBody;
use aws_smithy_json::deserialize::EscapeError;
use aws_smithy_runtime_api::client::identity::Identity;
use std::error::Error as StdError;
use std::fmt;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use zeroize::Zeroizing;

/// A login session token created by CLI and loaded from cache
#[derive(Clone)]
pub(super) struct LoginToken {
    pub(super) access_token: Credentials,
    pub(super) token_type: Option<String>,
    pub(super) identity_token: Option<String>,
    pub(super) refresh_token: Zeroizing<String>,
    pub(super) client_id: String,
    pub(super) dpop_key: Zeroizing<String>,
}

impl LoginToken {
    pub(super) fn expires_at(&self) -> SystemTime {
        self.access_token
            .expiry()
            .expect("sign-in token access token expected expiry")
    }

    pub(super) fn from_refresh(
        old_token: &LoginToken,
        resp: CreateOAuth2TokenResponseBody,
        now: SystemTime,
    ) -> Self {
        let access_token_output = resp.access_token().expect("accessToken in response");
        let expires_in = resp.expires_in();
        let expiry = now + Duration::from_secs(expires_in as u64);

        let mut credentials = Credentials::builder()
            .access_key_id(access_token_output.access_key_id())
            .secret_access_key(access_token_output.secret_access_key())
            .session_token(access_token_output.session_token())
            .provider_name(PROVIDER_NAME)
            .expiry(expiry);
        credentials.set_account_id(old_token.access_token.account_id().cloned());
        let credentials = credentials.build();

        Self {
            access_token: credentials,
            token_type: old_token.token_type.clone(),
            identity_token: old_token.identity_token.clone(),
            refresh_token: Zeroizing::new(resp.refresh_token().to_string()),
            client_id: old_token.client_id.clone(),
            dpop_key: old_token.dpop_key.clone(),
        }
    }
}

impl fmt::Debug for LoginToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CachedSsoToken")
            .field("access_token", &self.access_token)
            .field("token_type", &self.token_type)
            .field("identity_token", &self.identity_token)
            .field("refresh_token", &self.refresh_token)
            .field("client_id", &self.client_id)
            .field("dpop_key", &"** redacted **")
            .finish()
    }
}

#[derive(Debug)]
pub(super) enum LoginTokenError {
    FailedToFormatDateTime {
        source: Box<dyn StdError + Send + Sync>,
    },
    IoError {
        what: &'static str,
        path: PathBuf,
        source: std::io::Error,
    },
    JsonError(Box<dyn StdError + Send + Sync>),
    MissingField(&'static str),
    NoHomeDirectory,
    ExpiredToken,
    WrongIdentityType(Identity),
    RefreshFailed {
        message: Option<String>,
        source: Box<dyn StdError + Send + Sync>,
    },
    Other {
        message: String,
        source: Option<Box<dyn StdError + Send + Sync>>,
    },
}

impl LoginTokenError {
    pub(super) fn other(
        message: impl Into<String>,
        source: Option<Box<dyn StdError + Send + Sync>>,
    ) -> Self {
        Self::Other {
            message: message.into(),
            source,
        }
    }
}

impl fmt::Display for LoginTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedToFormatDateTime { .. } => write!(f, "failed to format date time"),
            Self::IoError { what, path, .. } => write!(f, "failed to {what} `{}`", path.display()),
            Self::JsonError(_) => write!(f, "invalid JSON in cached Login token file"),
            Self::MissingField(field) => {
                write!(f, "missing field `{field}` in cached Login token file")
            }
            Self::NoHomeDirectory => write!(f, "couldn't resolve a home directory"),
            Self::ExpiredToken => write!(f, "cached Login token is expired"),
            Self::WrongIdentityType(identity) => {
                write!(f, "wrong identity type for Login. Expected DPoP private key but got `{identity:?}`")
            }
            Self::RefreshFailed { message, .. } => {
                if let Some(msg) = message {
                    write!(f, "failed to refresh cached Login token: {msg}")
                } else {
                    write!(f, "failed to refresh cached Login token")
                }
            }
            Self::Other { message, .. } => {
                write!(f, "failed to load cached Login token: {message}")
            }
        }
    }
}

impl StdError for LoginTokenError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            LoginTokenError::FailedToFormatDateTime { source } => Some(source.as_ref()),
            LoginTokenError::IoError { source, .. } => Some(source),
            LoginTokenError::JsonError(source) => Some(source.as_ref()),
            LoginTokenError::MissingField(_) => None,
            LoginTokenError::NoHomeDirectory => None,
            LoginTokenError::ExpiredToken => None,
            LoginTokenError::WrongIdentityType(_) => None,
            LoginTokenError::RefreshFailed { source, .. } => Some(source.as_ref()),
            LoginTokenError::Other { source, .. } => match source {
                Some(err) => Some(err.as_ref()),
                None => None,
            },
        }
    }
}

impl From<EscapeError> for LoginTokenError {
    fn from(err: EscapeError) -> Self {
        Self::JsonError(err.into())
    }
}

impl From<aws_smithy_json::deserialize::error::DeserializeError> for LoginTokenError {
    fn from(err: aws_smithy_json::deserialize::error::DeserializeError) -> Self {
        Self::JsonError(err.into())
    }
}

impl From<LoginTokenError> for CredentialsError {
    fn from(val: LoginTokenError) -> CredentialsError {
        match val {
            LoginTokenError::FailedToFormatDateTime { .. } => {
                CredentialsError::invalid_configuration(val)
            }
            LoginTokenError::IoError { .. } => CredentialsError::unhandled(val),
            LoginTokenError::JsonError(_) => CredentialsError::unhandled(val),
            LoginTokenError::MissingField(_) => CredentialsError::invalid_configuration(val),
            LoginTokenError::NoHomeDirectory => CredentialsError::unhandled(val),
            LoginTokenError::ExpiredToken => CredentialsError::unhandled(val),
            LoginTokenError::RefreshFailed { .. } => CredentialsError::provider_error(val),
            LoginTokenError::WrongIdentityType(_) => CredentialsError::invalid_configuration(val),
            LoginTokenError::Other { .. } => CredentialsError::unhandled(val),
        }
    }
}
