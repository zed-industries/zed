use std::{fmt, io::Error as IoError, result::Result as StdResult};

/// Possible errors returned by prompts.
#[derive(Debug)]
pub enum Error {
    /// Error while executing IO operations.
    IO(IoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(io) => write!(f, "IO error: {}", io),
        }
    }
}

impl std::error::Error for Error {}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Self::IO(err)
    }
}

/// Result type where errors are of type [Error](enum@Error).
pub type Result<T = ()> = StdResult<T, Error>;

impl From<Error> for IoError {
    fn from(value: Error) -> Self {
        match value {
            Error::IO(err) => err,
            // If other error types are added in the future:
            // err => IoError::new(std::io::ErrorKind::Other, err),
        }
    }
}
