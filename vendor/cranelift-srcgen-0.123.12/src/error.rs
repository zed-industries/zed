//! Defines an `Error` returned during source code generation.

use std::fmt;
use std::io;

/// An error that occurred when the cranelift_codegen_meta crate was generating
/// source files for the cranelift_codegen crate.
#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorInner>,
}

impl Error {
    /// Create a new error object with the given message.
    pub fn with_msg<S: Into<String>>(msg: S) -> Error {
        Error {
            inner: Box::new(ErrorInner::Msg(msg.into())),
        }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            inner: Box::new(ErrorInner::IoError(e)),
        }
    }
}

#[derive(Debug)]
enum ErrorInner {
    Msg(String),
    IoError(io::Error),
}

impl fmt::Display for ErrorInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorInner::Msg(ref s) => write!(f, "{s}"),
            ErrorInner::IoError(ref e) => write!(f, "{e}"),
        }
    }
}
