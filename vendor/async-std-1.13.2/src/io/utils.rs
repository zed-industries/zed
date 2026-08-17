use crate::utils::Context;

use std::{error::Error as StdError, fmt, io};

/// Wrap `std::io::Error` with additional message
///
/// Keeps the original error kind and stores the original I/O error as `source`.
impl<T> Context for Result<T, std::io::Error> {
    fn context(self, message: impl Fn() -> String) -> Self {
        self.map_err(|e| VerboseError::wrap(e, message()))
    }
}

#[derive(Debug)]
pub(crate) struct VerboseError {
    source: io::Error,
    message: String,
}

impl VerboseError {
    pub(crate) fn wrap(source: io::Error, message: impl Into<String>) -> io::Error {
        io::Error::new(
            source.kind(),
            VerboseError {
                source,
                message: message.into(),
            },
        )
    }
}

impl fmt::Display for VerboseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for VerboseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.source)
    }
}
