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

pub type Result<T, E = CratesIoError> = std::result::Result<T, E>;

#[async_trait]
pub trait CratesIoClient: Send + Sync {
    async fn get_site_metadata(&self) -> Result<SiteMetadata>;

    /// Lists every crate matching the query, requesting one page of 100 crates
    /// per second.
    async fn list_crates(&self, query: &CrateListQuery) -> Result<Vec<Crate>>;

    async fn get_crate(&self, name: &CrateName) -> Result<CrateDetails>;

    /// Requires a token with the `trusted-publishing` endpoint scope that
    /// belongs to an owner of the crate.
    async fn update_crate(&self, name: &CrateName, update: &CrateUpdate) -> Result<Crate>;

    /// Requires a token with the `trusted-publishing` endpoint scope that
    /// belongs to an owner of the crate.
    async fn list_github_configs(&self, crate_name: &CrateName) -> Result<Vec<GitHubConfig>>;

    /// Requires a token with the `trusted-publishing` endpoint scope that
    /// belongs to an owner of the crate with a verified email address.
    /// crates.io allows at most five configurations per crate.
    async fn create_github_config(&self, config: &NewGitHubConfig) -> Result<GitHubConfig>;

    /// Requires a token with the `trusted-publishing` endpoint scope that
    /// belongs to an owner of the configuration's crate.
    async fn delete_github_config(&self, id: GitHubConfigId) -> Result<()>;
}

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

#[derive(Clone)]
pub struct ApiToken(String);

impl ApiToken {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub(crate) fn value(&self) -> &str {
        &self.0
    }
}
