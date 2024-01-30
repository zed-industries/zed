use std::ops::{Deref, DerefMut};

use crate::SharedString;

/// A URI stored in a [`SharedString`].
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum SharedUri {
    /// A path to a local file.
    File(SharedString),
    /// A URL to a remote resource.
    Network(SharedString),
}

impl SharedUri {
    /// Creates a [`SharedUri`] pointing to a local file.
    pub fn file<S: Into<SharedString>>(s: S) -> Self {
        Self::File(s.into())
    }

    /// Creates a [`SharedUri`] pointing to a remote resource.
    pub fn network<S: Into<SharedString>>(s: S) -> Self {
        Self::Network(s.into())
    }
}

impl Default for SharedUri {
    fn default() -> Self {
        Self::Network(SharedString::default())
    }
}

impl Deref for SharedUri {
    type Target = SharedString;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::File(s) => s,
            Self::Network(s) => s,
        }
    }
}

impl DerefMut for SharedUri {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::File(s) => s,
            Self::Network(s) => s,
        }
    }
}

impl std::fmt::Debug for SharedUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(s) => write!(f, "File({:?})", s),
            Self::Network(s) => write!(f, "Network({:?})", s),
        }
    }
}

impl std::fmt::Display for SharedUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
