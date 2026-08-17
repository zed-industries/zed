/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Credentials provider errors

use std::error::Error;
use std::fmt;
use std::time::Duration;

/// Details for [`CredentialsError::CredentialsNotLoaded`]
#[derive(Debug)]
pub struct CredentialsNotLoaded {
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

/// Details for [`CredentialsError::ProviderTimedOut`] or [`TokenError::ProviderTimedOut`]
#[derive(Debug)]
pub struct ProviderTimedOut {
    timeout_duration: Duration,
}

impl ProviderTimedOut {
    /// Returns the maximum allowed timeout duration that was exceeded
    pub fn timeout_duration(&self) -> Duration {
        self.timeout_duration
    }
}

/// Details for [`CredentialsError::InvalidConfiguration`] or [`TokenError::InvalidConfiguration`]
#[derive(Debug)]
pub struct InvalidConfiguration {
    source: Box<dyn Error + Send + Sync + 'static>,
}

/// Details for [`CredentialsError::ProviderError`] or [`TokenError::ProviderError`]
#[derive(Debug)]
pub struct ProviderError {
    source: Box<dyn Error + Send + Sync + 'static>,
}

/// Details for [`CredentialsError::Unhandled`] or [`TokenError::Unhandled`]
#[derive(Debug)]
pub struct Unhandled {
    source: Box<dyn Error + Send + Sync + 'static>,
}

/// Error returned when credentials failed to load.
#[derive(Debug)]
#[non_exhaustive]
pub enum CredentialsError {
    /// No credentials were available for this provider
    CredentialsNotLoaded(CredentialsNotLoaded),

    /// Loading credentials from this provider exceeded the maximum allowed duration
    ProviderTimedOut(ProviderTimedOut),

    /// The provider was given an invalid configuration
    ///
    /// For example:
    /// - syntax error in ~/.aws/config
    /// - assume role profile that forms an infinite loop
    InvalidConfiguration(InvalidConfiguration),

    /// The provider experienced an error during credential resolution
    ///
    /// This may include errors like a 503 from STS or a file system error when attempting to
    /// read a configuration file.
    ProviderError(ProviderError),

    /// An unexpected error occurred during credential resolution
    ///
    /// If the error is something that can occur during expected usage of a provider, `ProviderError`
    /// should be returned instead. Unhandled is reserved for exceptional cases, for example:
    /// - Returned data not UTF-8
    /// - A provider returns data that is missing required fields
    Unhandled(Unhandled),
}

impl CredentialsError {
    /// The credentials provider did not provide credentials
    ///
    /// This error indicates the credentials provider was not enable or no configuration was set.
    /// This contrasts with [`invalid_configuration`](CredentialsError::InvalidConfiguration), indicating
    /// that the provider was configured in some way, but certain settings were invalid.
    pub fn not_loaded(source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        CredentialsError::CredentialsNotLoaded(CredentialsNotLoaded {
            source: Some(source.into()),
        })
    }

    /// The credentials provider did not provide credentials
    ///
    /// This error indicates the credentials provider was not enable or no configuration was set.
    /// This contrasts with [`invalid_configuration`](CredentialsError::InvalidConfiguration), indicating
    /// that the provider was configured in some way, but certain settings were invalid.
    pub fn not_loaded_no_source() -> Self {
        CredentialsError::CredentialsNotLoaded(CredentialsNotLoaded { source: None })
    }

    /// An unexpected error occurred loading credentials from this provider
    ///
    /// Unhandled errors should not occur during normal operation and should be reserved for exceptional
    /// cases, such as a JSON API returning an output that was not parseable as JSON.
    pub fn unhandled(source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(Unhandled {
            source: source.into(),
        })
    }

    /// The credentials provider returned an error
    ///
    /// Provider errors may occur during normal use of a credentials provider, e.g. a 503 when
    /// retrieving credentials from IMDS.
    pub fn provider_error(source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        Self::ProviderError(ProviderError {
            source: source.into(),
        })
    }

    /// The provided configuration for a provider was invalid
    pub fn invalid_configuration(
        source: impl Into<Box<dyn Error + Send + Sync + 'static>>,
    ) -> Self {
        Self::InvalidConfiguration(InvalidConfiguration {
            source: source.into(),
        })
    }

    /// The credentials provider did not provide credentials within an allotted duration
    pub fn provider_timed_out(timeout_duration: Duration) -> Self {
        Self::ProviderTimedOut(ProviderTimedOut { timeout_duration })
    }
}

impl fmt::Display for CredentialsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CredentialsError::CredentialsNotLoaded(_) => {
                write!(f, "the credential provider was not enabled")
            }
            CredentialsError::ProviderTimedOut(details) => write!(
                f,
                "credentials provider timed out after {} seconds",
                details.timeout_duration.as_secs()
            ),
            CredentialsError::InvalidConfiguration(_) => {
                write!(f, "the credentials provider was not properly configured")
            }
            CredentialsError::ProviderError(_) => {
                write!(f, "an error occurred while loading credentials")
            }
            CredentialsError::Unhandled(_) => {
                write!(f, "unexpected credentials error")
            }
        }
    }
}

impl Error for CredentialsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CredentialsError::CredentialsNotLoaded(details) => {
                details.source.as_ref().map(|s| s.as_ref() as _)
            }
            CredentialsError::ProviderTimedOut(_) => None,
            CredentialsError::InvalidConfiguration(details) => Some(details.source.as_ref() as _),
            CredentialsError::ProviderError(details) => Some(details.source.as_ref() as _),
            CredentialsError::Unhandled(details) => Some(details.source.as_ref() as _),
        }
    }
}

/// Details for [`TokenError::TokenNotLoaded`]
#[derive(Debug)]
pub struct TokenNotLoaded {
    source: Box<dyn Error + Send + Sync + 'static>,
}

/// Error returned when an access token provider fails to provide an access token.
#[derive(Debug)]
pub enum TokenError {
    /// This provider couldn't provide a token.
    TokenNotLoaded(TokenNotLoaded),

    /// Loading a token from this provider exceeded the maximum allowed time.
    ProviderTimedOut(ProviderTimedOut),

    /// The provider was given invalid configuration.
    ///
    /// For example, a syntax error in `~/.aws/config`.
    InvalidConfiguration(InvalidConfiguration),

    /// The provider experienced an error during credential resolution.
    ProviderError(ProviderError),

    /// An unexpected error occurred during token resolution.
    ///
    /// If the error is something that can occur during expected usage of a provider, `ProviderError`
    /// should be returned instead. Unhandled is reserved for exceptional cases, for example:
    /// - Returned data not UTF-8
    /// - A provider returns data that is missing required fields
    Unhandled(Unhandled),
}

impl TokenError {
    /// The access token provider couldn't provide a token.
    ///
    /// This error indicates the token provider was not enable or no configuration was set.
    /// This contrasts with [`invalid_configuration`](TokenError::InvalidConfiguration), indicating
    /// that the provider was configured in some way, but certain settings were invalid.
    pub fn not_loaded(source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        TokenError::TokenNotLoaded(TokenNotLoaded {
            source: source.into(),
        })
    }

    /// An unexpected error occurred loading an access token from this provider.
    ///
    /// Unhandled errors should not occur during normal operation and should be reserved for exceptional
    /// cases, such as a JSON API returning an output that was not parseable as JSON.
    pub fn unhandled(source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(Unhandled {
            source: source.into(),
        })
    }

    /// The access token provider returned an error.
    pub fn provider_error(source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        Self::ProviderError(ProviderError {
            source: source.into(),
        })
    }

    /// The provided configuration for a provider was invalid.
    pub fn invalid_configuration(
        source: impl Into<Box<dyn Error + Send + Sync + 'static>>,
    ) -> Self {
        Self::InvalidConfiguration(InvalidConfiguration {
            source: source.into(),
        })
    }

    /// The access token provider did not provide a token within an allotted amount of time.
    pub fn provider_timed_out(timeout_duration: Duration) -> Self {
        Self::ProviderTimedOut(ProviderTimedOut { timeout_duration })
    }
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenError::TokenNotLoaded(_) => {
                write!(f, "the access token provider was not enabled")
            }
            TokenError::ProviderTimedOut(details) => write!(
                f,
                "access token provider timed out after {} seconds",
                details.timeout_duration.as_secs()
            ),
            TokenError::InvalidConfiguration(_) => {
                write!(f, "the access token provider was not properly configured")
            }
            TokenError::ProviderError(_) => {
                write!(f, "an error occurred while loading an access token")
            }
            TokenError::Unhandled(_) => {
                write!(f, "unexpected access token providererror")
            }
        }
    }
}

impl Error for TokenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TokenError::TokenNotLoaded(details) => Some(details.source.as_ref() as _),
            TokenError::ProviderTimedOut(_) => None,
            TokenError::InvalidConfiguration(details) => Some(details.source.as_ref() as _),
            TokenError::ProviderError(details) => Some(details.source.as_ref() as _),
            TokenError::Unhandled(details) => Some(details.source.as_ref() as _),
        }
    }
}
