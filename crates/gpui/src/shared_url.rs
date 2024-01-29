use std::ops::{Deref, DerefMut};

use crate::SharedString;

/// A URL stored in a `SharedString` pointing to a file or a remote resource.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum SharedUrl {
    /// A path to a local file.
    File(SharedString),
    /// A URL to a remote resource.
    Network(SharedString),
}

impl SharedUrl {
    /// Create a URL pointing to a local file.
    pub fn file<S: Into<SharedString>>(s: S) -> Self {
        Self::File(s.into())
    }

    /// Create a URL pointing to a remote resource.
    pub fn network<S: Into<SharedString>>(s: S) -> Self {
        Self::Network(s.into())
    }
}

impl Default for SharedUrl {
    fn default() -> Self {
        Self::Network(SharedString::default())
    }
}

impl Deref for SharedUrl {
    type Target = SharedString;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::File(s) => s,
            Self::Network(s) => s,
        }
    }
}

impl DerefMut for SharedUrl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::File(s) => s,
            Self::Network(s) => s,
        }
    }
}

impl std::fmt::Debug for SharedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(s) => write!(f, "File({:?})", s),
            Self::Network(s) => write!(f, "Network({:?})", s),
        }
    }
}

impl std::fmt::Display for SharedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
