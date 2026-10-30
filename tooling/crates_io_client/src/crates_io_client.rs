mod real_crates_io_client;
#[cfg(any(test, feature = "test-support"))]
pub mod simulated_crates_io_client;
mod types;

#[cfg(test)]
mod tests;

use async_trait::async_trait;
use http_client::StatusCode;
use itertools::Itertools as _;
use serde::Deserialize;

pub use real_crates_io_client::RealCratesIoClient;
pub use types::*;

#[derive(Debug, thiserror::Error)]
pub enum CratesIoError {
    /// The token is invalid, or lacks the scope or crate ownership the
    /// request requires.
    #[error("crates.io denied permission ({status}): {message}")]
    PermissionDenied { status: StatusCode, message: String },
    #[error("crates.io could not find the requested resource: {message}")]
    NotFound { message: String },
    #[error("crates.io request failed with {status}: {message}")]
    Upstream { status: StatusCode, message: String },
    /// The request could not be sent or its response could not be read.
    #[error(transparent)]
    Request(#[from] anyhow::Error),
}

impl CratesIoError {
    fn from_response(status: StatusCode, body: &str) -> Self {
        #[derive(Deserialize)]
        struct ErrorResponse {
            errors: Vec<ErrorDetail>,
        }

        #[derive(Deserialize)]
        struct ErrorDetail {
            detail: String,
        }

        let message = match serde_json::from_str::<ErrorResponse>(body) {
            Ok(response) => response
                .errors
                .into_iter()
                .map(|error| error.detail)
                .join("; "),
            Err(_) => body.to_owned(),
        };

        match status {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Self::PermissionDenied { status, message }
            }
            StatusCode::NOT_FOUND => Self::NotFound { message },
            _ => Self::Upstream { status, message },
        }
    }
}

pub type Result<T, E = CratesIoError> = std::result::Result<T, E>;

#[async_trait]
pub trait CratesIoClient: Send + Sync {
    async fn get_site_metadata(&self) -> Result<SiteMetadata>;
}
