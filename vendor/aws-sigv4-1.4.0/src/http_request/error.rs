/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use http::header::{InvalidHeaderName, InvalidHeaderValue};
use http::uri::InvalidUri;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum SigningErrorKind {
    FailedToCreateCanonicalRequest { source: CanonicalRequestError },
    UnsupportedIdentityType,
}

/// Error signing request
#[derive(Debug)]
pub struct SigningError {
    kind: SigningErrorKind,
}

impl SigningError {
    pub(crate) fn unsupported_identity_type() -> Self {
        Self {
            kind: SigningErrorKind::UnsupportedIdentityType,
        }
    }
}

impl fmt::Display for SigningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            SigningErrorKind::FailedToCreateCanonicalRequest { .. } => {
                write!(f, "failed to create canonical request")
            }
            SigningErrorKind::UnsupportedIdentityType => {
                write!(f, "only 'AWS credentials' are supported for signing")
            }
        }
    }
}

impl Error for SigningError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            SigningErrorKind::FailedToCreateCanonicalRequest { source } => Some(source),
            SigningErrorKind::UnsupportedIdentityType => None,
        }
    }
}

impl From<CanonicalRequestError> for SigningError {
    fn from(source: CanonicalRequestError) -> Self {
        Self {
            kind: SigningErrorKind::FailedToCreateCanonicalRequest { source },
        }
    }
}

#[derive(Debug)]
enum CanonicalRequestErrorKind {
    InvalidHeaderName { source: InvalidHeaderName },
    InvalidHeaderValue { source: InvalidHeaderValue },
    InvalidUri { source: InvalidUri },
    UnsupportedIdentityType,
}

#[derive(Debug)]
pub(crate) struct CanonicalRequestError {
    kind: CanonicalRequestErrorKind,
}

impl fmt::Display for CanonicalRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CanonicalRequestErrorKind::*;
        match self.kind {
            InvalidHeaderName { .. } => write!(f, "invalid header name"),
            InvalidHeaderValue { .. } => write!(f, "invalid header value"),
            InvalidUri { .. } => write!(f, "the uri was invalid"),
            UnsupportedIdentityType => {
                write!(f, "only AWS credentials are supported for signing")
            }
        }
    }
}

impl Error for CanonicalRequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use CanonicalRequestErrorKind::*;
        match &self.kind {
            InvalidHeaderName { source } => Some(source),
            InvalidHeaderValue { source } => Some(source),
            InvalidUri { source } => Some(source),
            UnsupportedIdentityType => None,
        }
    }
}

impl CanonicalRequestError {
    pub(crate) fn unsupported_identity_type() -> Self {
        Self {
            kind: CanonicalRequestErrorKind::UnsupportedIdentityType,
        }
    }
}

impl From<InvalidHeaderName> for CanonicalRequestError {
    fn from(source: InvalidHeaderName) -> Self {
        Self {
            kind: CanonicalRequestErrorKind::InvalidHeaderName { source },
        }
    }
}

impl From<InvalidHeaderValue> for CanonicalRequestError {
    fn from(source: InvalidHeaderValue) -> Self {
        Self {
            kind: CanonicalRequestErrorKind::InvalidHeaderValue { source },
        }
    }
}

impl From<InvalidUri> for CanonicalRequestError {
    fn from(source: InvalidUri) -> Self {
        Self {
            kind: CanonicalRequestErrorKind::InvalidUri { source },
        }
    }
}
