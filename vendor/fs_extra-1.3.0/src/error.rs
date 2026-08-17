use std::error::Error as StdError;
use std::ffi::OsString;
use std::fmt;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::path::StripPrefixError;

/// A list specifying general categories of fs_extra error.
#[derive(Debug)]
pub enum ErrorKind {
    /// An entity was not found.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// An entity already exists.
    AlreadyExists,
    /// This operation was interrupted.
    Interrupted,
    /// Path does not a directory.
    InvalidFolder,
    /// Path does not a file.
    InvalidFile,
    /// Invalid file name.
    InvalidFileName,
    /// Invalid path.
    InvalidPath,
    /// Any I/O error.
    Io(IoError),
    /// Any StripPrefix error.
    StripPrefix(StripPrefixError),
    /// Any OsString error.
    OsString(OsString),
    /// Any fs_extra error not part of this list.
    Other,
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ErrorKind::NotFound => "entity not found",
            ErrorKind::PermissionDenied => "permission denied",
            ErrorKind::AlreadyExists => "entity already exists",
            ErrorKind::Interrupted => "operation interrupted",
            ErrorKind::Other => "other os error",
            ErrorKind::InvalidFolder => "invalid folder error",
            ErrorKind::InvalidFile => "invalid file error",
            ErrorKind::InvalidFileName => "invalid file name error",
            ErrorKind::InvalidPath => "invalid path error",
            ErrorKind::Io(_) => "Io error",
            ErrorKind::StripPrefix(_) => "Strip prefix error",
            ErrorKind::OsString(_) => "OsString error",
        }
    }
}

/// A specialized Result type for fs_extra operations.
///
/// This typedef is generally used to avoid writing out fs_extra::Error directly
/// and is otherwise a direct mapping to Result.
///
///#Examples
///
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::create;
///
///fn get_string() -> io::Result<()> {
///
///     create("test_dir")?;
///
///     Ok(())
/// }
/// ```
pub type Result<T> = ::std::result::Result<T, Error>;

/// The error type for fs_extra operations with files and folder.
///
/// Errors mostly originate from the underlying OS, but custom instances of
/// `Error` can be created with crafted error messages and a particular value of
/// [`ErrorKind`].
///
/// [`ErrorKind`]: enum.ErrorKind.html
#[derive(Debug)]
pub struct Error {
    /// Type error
    pub kind: ErrorKind,
    message: String,
}

impl Error {
    /// Create a new fs_extra error from a kind of error error as well as an arbitrary error payload.
    ///
    ///#Examples
    /// ```rust,ignore
    ///
    /// extern crate fs_extra;
    /// use fs_extra::error::{Error, ErrorKind};
    ///
    /// errors can be created from strings
    /// let custom_error = Error::new(ErrorKind::Other, "Other Error!");
    /// // errors can also be created from other errors
    /// let custom_error2 = Error::new(ErrorKind::Interrupted, custom_error);
    ///
    /// ```
    pub fn new(kind: ErrorKind, message: &str) -> Error {
        Error {
            kind,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.kind.as_str()
    }
}
impl From<StripPrefixError> for Error {
    fn from(err: StripPrefixError) -> Error {
        Error::new(
            ErrorKind::StripPrefix(err),
            "StripPrefixError. Look inside for more details",
        )
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Error {
        Error::new(
            ErrorKind::OsString(err),
            "OsString. Look inside for more details",
        )
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        let err_kind: ErrorKind;
        match err.kind() {
            IoErrorKind::NotFound => err_kind = ErrorKind::NotFound,
            IoErrorKind::PermissionDenied => err_kind = ErrorKind::PermissionDenied,
            IoErrorKind::AlreadyExists => err_kind = ErrorKind::AlreadyExists,
            IoErrorKind::Interrupted => err_kind = ErrorKind::Interrupted,
            IoErrorKind::Other => err_kind = ErrorKind::Other,
            _ => {
                err_kind = ErrorKind::Io(err);
                return Error::new(err_kind, "Io error. Look inside err_kind for more details.");
            }
        }
        Error::new(err_kind, &err.to_string())
    }
}
