use std::fmt;

/// An error that can occur when attaching or detaching a [`crate::CrashHandler`]
#[derive(Debug)]
pub enum Error {
    /// Unable to `mmap` memory
    OutOfMemory,
    /// For simplicity sake, only one [`crate::CrashHandler`] can be registered
    /// at any one time.
    HandlerAlreadyInstalled,
    /// An I/O or other syscall failed
    Io(std::io::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(inner) => Some(inner),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("unable to allocate memory"),
            Self::HandlerAlreadyInstalled => {
                f.write_str("an exception handler is already installed")
            }
            Self::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
