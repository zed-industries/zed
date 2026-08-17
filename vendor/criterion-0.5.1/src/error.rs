#[cfg(feature = "csv_output")]
use csv::Error as CsvError;
use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    AccessError {
        path: PathBuf,
        inner: io::Error,
    },
    CopyError {
        from: PathBuf,
        to: PathBuf,
        inner: io::Error,
    },
    SerdeError {
        path: PathBuf,
        inner: SerdeError,
    },
    #[cfg(feature = "csv_output")]
    /// This API requires the following crate features to be activated: csv_output
    CsvError(CsvError),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AccessError { path, inner } => {
                write!(f, "Failed to access file {:?}: {}", path, inner)
            }
            Error::CopyError { from, to, inner } => {
                write!(f, "Failed to copy file {:?} to {:?}: {}", from, to, inner)
            }
            Error::SerdeError { path, inner } => write!(
                f,
                "Failed to read or write file {:?} due to serialization error: {}",
                path, inner
            ),
            #[cfg(feature = "csv_output")]
            Error::CsvError(inner) => write!(f, "CSV error: {}", inner),
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::AccessError { .. } => "AccessError",
            Error::CopyError { .. } => "CopyError",
            Error::SerdeError { .. } => "SerdeError",
            #[cfg(feature = "csv_output")]
            Error::CsvError(_) => "CsvError",
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            Error::AccessError { inner, .. } => Some(inner),
            Error::CopyError { inner, .. } => Some(inner),
            Error::SerdeError { inner, .. } => Some(inner),
            #[cfg(feature = "csv_output")]
            Error::CsvError(inner) => Some(inner),
        }
    }
}

#[cfg(feature = "csv_output")]
impl From<CsvError> for Error {
    fn from(other: CsvError) -> Error {
        Error::CsvError(other)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub(crate) fn log_error(e: &Error) {
    error!("error: {}", e);
}
