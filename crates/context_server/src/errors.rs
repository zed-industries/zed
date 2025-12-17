use std::fmt;
use std::sync::Arc;

use crate::transport::HttpTransportError;

/// Typed error enum for context server operations
#[derive(Debug, Clone)]
pub enum ContextServerError {
    /// Authentication is required (initial auth or re-auth after expiry)
    AuthRequired(HttpTransportError),
    /// Connection failed for non-auth reasons
    Connection(Arc<str>),
    /// Protocol error
    Protocol(Arc<str>),
    /// Other errors
    Other(Arc<str>),
}

impl ContextServerError {
    /// Create from an anyhow::Error, detecting auth-related errors
    pub fn from_anyhow(err: &anyhow::Error) -> Self {
        if let Some(http_err) = err.downcast_ref::<HttpTransportError>() {
            if http_err.is_auth_required() {
                return Self::AuthRequired(http_err.clone());
            }
        }
        Self::Connection(err.to_string().into())
    }

    /// Returns true if this error indicates authentication is required
    pub fn is_auth_required(&self) -> bool {
        matches!(self, Self::AuthRequired(_))
    }

    /// Get the display message for this error
    pub fn message(&self) -> &str {
        match self {
            Self::AuthRequired(err) => match err {
                HttpTransportError::AuthenticationRequired { .. } => {
                    "Authentication required. Please click the Authenticate button."
                }
                HttpTransportError::AuthenticationExpiredManual { .. } => {
                    "Authentication expired. Please reauthenticate."
                }
                _ => "Authentication error",
            },
            Self::Connection(msg) => msg,
            Self::Protocol(msg) => msg,
            Self::Other(msg) => msg,
        }
    }
}

impl fmt::Display for ContextServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthRequired(err) => write!(f, "{}", err),
            Self::Connection(msg) => write!(f, "Connection failed: {}", msg),
            Self::Protocol(msg) => write!(f, "Protocol error: {}", msg),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ContextServerError {}

impl From<HttpTransportError> for ContextServerError {
    fn from(err: HttpTransportError) -> Self {
        if err.is_auth_required() {
            Self::AuthRequired(err)
        } else {
            Self::Connection(err.to_string().into())
        }
    }
}

impl From<anyhow::Error> for ContextServerError {
    fn from(err: anyhow::Error) -> Self {
        Self::from_anyhow(&err)
    }
}

impl From<&anyhow::Error> for ContextServerError {
    fn from(err: &anyhow::Error) -> Self {
        Self::from_anyhow(err)
    }
}
