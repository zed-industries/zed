/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Error types for [`ImdsClient`](crate::imds::client::Client)

use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
use aws_smithy_runtime_api::client::result::SdkError;
use std::error::Error;
use std::fmt;

/// Error context for [`ImdsError::FailedToLoadToken`]
#[derive(Debug)]
pub struct FailedToLoadToken {
    source: SdkError<TokenError, HttpResponse>,
}

impl FailedToLoadToken {
    /// Returns `true` if a dispatch failure caused the token to fail to load
    pub fn is_dispatch_failure(&self) -> bool {
        matches!(self.source, SdkError::DispatchFailure(_))
    }

    pub(crate) fn into_source(self) -> SdkError<TokenError, HttpResponse> {
        self.source
    }
}

/// Error context for [`ImdsError::ErrorResponse`]
#[derive(Debug)]
pub struct ErrorResponse {
    raw: HttpResponse,
}

impl ErrorResponse {
    /// Returns the raw response from IMDS
    pub fn response(&self) -> &HttpResponse {
        &self.raw
    }
}

/// Error context for [`ImdsError::IoError`]
#[derive(Debug)]
pub struct IoError {
    source: Box<dyn Error + Send + Sync + 'static>,
}

/// Error context for [`ImdsError::Unexpected`]
#[derive(Debug)]
pub struct Unexpected {
    source: Box<dyn Error + Send + Sync + 'static>,
}

/// An error retrieving metadata from IMDS
#[derive(Debug)]
#[non_exhaustive]
pub enum ImdsError {
    /// An IMDSv2 Token could not be loaded
    ///
    /// Requests to IMDS must be accompanied by a token obtained via a `PUT` request. This is handled
    /// transparently by the [`Client`](crate::imds::client::Client).
    FailedToLoadToken(FailedToLoadToken),

    /// An error response was returned from IMDS
    ErrorResponse(ErrorResponse),

    /// IO Error
    ///
    /// An error occurred communication with IMDS
    IoError(IoError),

    /// An unexpected error occurred communicating with IMDS
    Unexpected(Unexpected),
}

impl ImdsError {
    pub(super) fn failed_to_load_token(source: SdkError<TokenError, HttpResponse>) -> Self {
        Self::FailedToLoadToken(FailedToLoadToken { source })
    }

    pub(super) fn error_response(raw: HttpResponse) -> Self {
        Self::ErrorResponse(ErrorResponse { raw })
    }

    pub(super) fn io_error(source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        Self::IoError(IoError {
            source: source.into(),
        })
    }

    pub(super) fn unexpected(source: impl Into<Box<dyn Error + Send + Sync + 'static>>) -> Self {
        Self::Unexpected(Unexpected {
            source: source.into(),
        })
    }
}

impl fmt::Display for ImdsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImdsError::FailedToLoadToken(_) => {
                write!(f, "failed to load IMDS session token")
            }
            ImdsError::ErrorResponse(context) => write!(
                f,
                "error response from IMDS (code: {}). {:?}",
                context.raw.status().as_u16(),
                context.raw
            ),
            ImdsError::IoError(_) => {
                write!(f, "an IO error occurred communicating with IMDS")
            }
            ImdsError::Unexpected(_) => {
                write!(f, "an unexpected error occurred communicating with IMDS",)
            }
        }
    }
}

impl Error for ImdsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            ImdsError::FailedToLoadToken(context) => Some(&context.source),
            ImdsError::IoError(context) => Some(context.source.as_ref()),
            ImdsError::Unexpected(context) => Some(context.source.as_ref()),
            ImdsError::ErrorResponse(_) => None,
        }
    }
}

#[derive(Debug)]
pub(super) enum InnerImdsError {
    BadStatus,
    InvalidUtf8,
}

impl fmt::Display for InnerImdsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InnerImdsError::BadStatus => write!(f, "failing status code returned from IMDS"),
            InnerImdsError::InvalidUtf8 => write!(f, "IMDS did not return valid UTF-8"),
        }
    }
}

impl Error for InnerImdsError {}

/// Invalid Endpoint Mode
#[derive(Debug)]
pub struct InvalidEndpointMode {
    mode: String,
}

impl InvalidEndpointMode {
    pub(super) fn new(mode: impl Into<String>) -> Self {
        Self { mode: mode.into() }
    }
}

impl fmt::Display for InvalidEndpointMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` is not a valid endpoint mode. Valid values are [`IPv4`, `IPv6`]",
            &self.mode
        )
    }
}

impl Error for InvalidEndpointMode {}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
enum BuildErrorKind {
    /// The endpoint mode was invalid
    InvalidEndpointMode(InvalidEndpointMode),

    /// The specified endpoint was not a valid URI
    InvalidEndpointUri(Box<dyn Error + Send + Sync + 'static>),
}

/// Error constructing IMDSv2 Client
#[derive(Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}

impl BuildError {
    pub(super) fn invalid_endpoint_mode(source: InvalidEndpointMode) -> Self {
        Self {
            kind: BuildErrorKind::InvalidEndpointMode(source),
        }
    }

    pub(super) fn invalid_endpoint_uri(
        source: impl Into<Box<dyn Error + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            kind: BuildErrorKind::InvalidEndpointUri(source.into()),
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        use BuildErrorKind::*;
        write!(f, "failed to build IMDS client: ")?;
        match self.kind {
            InvalidEndpointMode(_) => write!(f, "invalid endpoint mode"),
            InvalidEndpointUri(_) => write!(f, "invalid URI"),
        }
    }
}

impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use BuildErrorKind::*;
        match &self.kind {
            InvalidEndpointMode(e) => Some(e),
            InvalidEndpointUri(e) => Some(e.as_ref()),
        }
    }
}

#[derive(Debug)]
pub(super) enum TokenErrorKind {
    /// The token was invalid
    ///
    /// Because tokens must be eventually sent as a header, the token must be a valid header value.
    InvalidToken,

    /// No TTL was sent
    ///
    /// The token response must include a time-to-live indicating the lifespan of the token.
    NoTtl,

    /// The TTL was invalid
    ///
    /// The TTL must be a valid positive integer.
    InvalidTtl,

    /// Invalid Parameters
    ///
    /// The request to load a token was malformed. This indicates an SDK bug.
    InvalidParameters,

    /// Forbidden
    ///
    /// IMDS is disabled or has been disallowed via permissions.
    Forbidden,
}

/// Error retrieving token from IMDS
#[derive(Debug)]
pub struct TokenError {
    kind: TokenErrorKind,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenErrorKind::*;
        match self.kind {
            InvalidToken => write!(f, "invalid token"),
            NoTtl => write!(f, "token response did not contain a TTL header"),
            InvalidTtl => write!(f, "the returned TTL was invalid"),
            InvalidParameters => {
                write!(f, "invalid request parameters. This indicates an SDK bug.")
            }
            Forbidden => write!(
                f,
                "request forbidden: IMDS is disabled or the caller has insufficient permissions."
            ),
        }
    }
}

impl Error for TokenError {}

impl From<TokenErrorKind> for TokenError {
    fn from(kind: TokenErrorKind) -> Self {
        Self { kind }
    }
}
