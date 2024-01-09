use derive_more::{Deref, DerefMut};

use crate::SharedString;

/// A [`SharedString`] containing a URL.
#[derive(Deref, DerefMut, Default, PartialEq, Eq, Hash, Clone)]
pub struct SharedUrl(SharedString);

impl std::fmt::Debug for SharedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for SharedUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_ref())
    }
}

impl<T: Into<SharedString>> From<T> for SharedUrl {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
