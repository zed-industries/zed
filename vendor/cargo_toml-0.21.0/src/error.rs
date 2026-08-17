use std::error::Error as StdErr;
use std::{fmt, io};

/// In this crate's `Result`s.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// TOML parsing errors
    Parse(toml::de::Error),
    /// Filesystem access errors
    Io(io::Error),
    /// Manifest uses workspace inheritance, and the workspace failed to load
    Workspace(Box<Error>),
    /// Manifest uses workspace inheritance, and the data hasn't been inherited yet
    InheritedUnknownValue,
    /// Manifest uses workspace inheritance, but the root workspace is missing data
    WorkspaceIntegrity(String),
    /// ???
    Other(&'static str),
}

impl StdErr for Error {
    fn source(&self) -> Option<&(dyn StdErr + 'static)> {
        match *self {
            Error::Parse(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Workspace(ref err) => Some(err),
            Error::Other(_) | Error::InheritedUnknownValue | Error::WorkspaceIntegrity(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Parse(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::Other(msg) => f.write_str(msg),
            Error::WorkspaceIntegrity(ref s) => f.write_str(s),
            Error::Workspace(_) => f.write_str("can't load root workspace"),
            Error::InheritedUnknownValue => f.write_str("value from workspace hasn't been set"),
        }
    }
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match *self {
            Error::Parse(ref err) => Error::Parse(err.clone()),
            Error::Io(ref err) => Error::Io(io::Error::new(err.kind(), err.to_string())),
            Error::Other(msg) => Error::Other(msg),
            Error::WorkspaceIntegrity(ref msg) => Error::WorkspaceIntegrity(msg.clone()),
            Error::Workspace(ref e) => Error::Workspace(e.clone()),
            Error::InheritedUnknownValue => Error::InheritedUnknownValue,
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(o: toml::de::Error) -> Self {
        Error::Parse(o)
    }
}

impl From<io::Error> for Error {
    fn from(o: io::Error) -> Self {
        Error::Io(o)
    }
}
