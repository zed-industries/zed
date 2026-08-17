//! Error handling.
use std::fmt;

/// Enumeration of errors possible in this library
#[derive(Debug)]
pub enum Error {
    /// Cannot create the memfd
    Create(std::io::Error),
    /// Cannot add new seals to the memfd
    AddSeals(std::io::Error),
    /// Cannot read the seals of a memfd
    GetSeals(std::io::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Create(e) | Error::AddSeals(e) | Error::GetSeals(e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Error::Create(_) => "cannot create a memfd",
            Error::AddSeals(_) => "cannot add seals to the memfd",
            Error::GetSeals(_) => "cannot read seals for a memfd",
        })
    }
}

#[cfg(test)]
#[test]
fn error_send_sync() {
    fn assert_error<E: std::error::Error + Send + Sync + fmt::Display + fmt::Debug + 'static>() {}
    assert_error::<Error>();
}
