use http::uri::InvalidUri;

use snafu::{Backtrace, Snafu};

use std::fmt;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use tower::BoxError;

//This is workaround until I figure out how to get TryInto errors to work
#[derive(Debug)]
pub struct UriParseError;

impl Display for UriParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse URI")
    }
}

impl std::error::Error for UriParseError {}

/// An error that could have occurred while using [`crate::Octocrab`].
#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
#[non_exhaustive]
pub enum Error {
    GitHub {
        source: Box<GitHubError>,
        backtrace: Backtrace,
    },
    UriParse {
        source: UriParseError,
        backtrace: Backtrace,
    },
    Uri {
        source: InvalidUri,
        backtrace: Backtrace,
    },
    #[snafu(display("Installation Error: Github App authorization is required to target an installation.\n\nFound at {}", backtrace))]
    Installation { backtrace: Backtrace },
    #[snafu(display("Error getting installation access token: octocrab instance is not an installation.\n\nFound at {}", backtrace))]
    InstallationTokenInvalidAuth { backtrace: Backtrace },
    InvalidHeaderValue {
        source: http::header::InvalidHeaderValue,
        backtrace: Backtrace,
    },

    #[snafu(display("HTTP Error: {}\n\nFound at {}", source, backtrace))]
    Http {
        source: http::Error,
        backtrace: Backtrace,
    },

    InvalidUtf8 {
        source: FromUtf8Error,
        backtrace: Backtrace,
    },

    Encoder {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Service Error: {}\n\nFound at {}", source, backtrace))]
    Service {
        source: BoxError,
        backtrace: Backtrace,
    },

    #[snafu(display("Hyper Error: {}\n\nFound at {}", source, backtrace))]
    Hyper {
        source: hyper::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Serde Url Encode Error: {}\nFound at {}", source, backtrace))]
    SerdeUrlEncoded {
        source: serde_urlencoded::ser::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Serde Error: {}\nFound at {}", source, backtrace))]
    Serde {
        source: serde_json::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("JSON Error in {}: {}\nFound at {}", source.path(), source.inner(), backtrace))]
    Json {
        source: serde_path_to_error::Error<serde_json::Error>,
        backtrace: Backtrace,
    },
    #[snafu(display("JWT Error in {}\nFound at {}", source, backtrace))]
    JWT {
        source: jsonwebtoken::errors::Error,
        backtrace: Backtrace,
    },
    Other {
        source: Box<dyn std::error::Error + Send + Sync>,
        backtrace: Backtrace,
    },
}

/// An error returned from GitHub's API.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GitHubError {
    pub documentation_url: Option<String>,
    pub errors: Option<Vec<serde_json::Value>>,
    pub message: String,
    pub status_code: http::StatusCode,
}

impl fmt::Display for GitHubError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)?;

        if let Some(documentation_url) = &self.documentation_url {
            write!(f, "\nDocumentation URL: {documentation_url}")?;
        }

        if let Some(errors) = &self.errors.as_ref().filter(|errors| !errors.is_empty()) {
            write!(f, "\nErrors:")?;
            for error in errors.iter() {
                write!(f, "\n- {error}")?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for GitHubError {}
