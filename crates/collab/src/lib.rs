use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
    Http(StatusCode, String),
    Internal(anyhow::Error),
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self::Internal(error)
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl From<axum::Error> for Error {
    fn from(error: axum::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Internal(error.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Http(code, message) => (code, message).into_response(),
            Error::Internal(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", &error)).into_response()
            }
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(code, message) => (code, message).fmt(f),
            Error::Internal(error) => error.fmt(f),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(code, message) => write!(f, "{code}: {message}"),
            Error::Internal(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
