//! The error types defined by this crate.

use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::Path;
use std::path::PathBuf;

/// The error returned when [`BasePath::try_new`] is given a path without a
/// prefix.
///
/// [`BasePath::try_new`]: super::BasePath::try_new
#[derive(Clone, Debug, PartialEq)]
pub struct MissingPrefixError(pub(super) ());

impl Display for MissingPrefixError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "path is missing a prefix".fmt(f)
    }
}

impl Error for MissingPrefixError {}

/// The error returned when [`BasePathBuf::try_new`] is given a path without a
/// prefix.
///
/// [`BasePathBuf::try_new`]: super::BasePathBuf::try_new
#[derive(Clone, Debug, PartialEq)]
pub struct MissingPrefixBufError(pub(super) PathBuf);

impl MissingPrefixBufError {
    /// Returns a reference to the path that caused this error.
    #[inline]
    #[must_use]
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    /// Returns the path that caused this error.
    #[inline]
    #[must_use]
    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }
}

impl Display for MissingPrefixBufError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "path is missing a prefix: \"{}\"", self.0.display())
    }
}

impl Error for MissingPrefixBufError {}

/// The error returned when [`BasePath::parent`] cannot remove the path's last
/// component.
///
/// [`BasePath::parent`]: super::BasePath::parent
#[derive(Clone, Debug, PartialEq)]
pub struct ParentError(pub(super) ());

impl Display for ParentError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "cannot remove the path's last component".fmt(f)
    }
}

impl Error for ParentError {}
