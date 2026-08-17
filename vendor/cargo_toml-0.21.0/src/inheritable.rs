use crate::Error;
use serde::{Deserialize, Serialize};

/// Placeholder for a property that may be missing from its package, and needs to be copied from a `Workspace`.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Inheritable<T> {
    Set(T),
    Inherited {
        /// Always `true` (this is for serde)
        workspace: bool,
    },
}

impl<T: PartialEq> PartialEq for Inheritable<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Set(a), Self::Set(b)) => a.eq(b),
            _ => false,
        }
    }
}

impl<T> Inheritable<T> {
    pub fn as_ref(&self) -> Inheritable<&T> {
        match self {
            Self::Set(t) => Inheritable::Set(t),
            Self::Inherited { .. } => Inheritable::Inherited { workspace: true },
        }
    }

    /// You can read the value
    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set(_))
    }

    pub fn get(&self) -> Result<&T, Error> {
        match self {
            Self::Set(t) => Ok(t),
            Self::Inherited { .. } => Err(Error::InheritedUnknownValue),
        }
    }

    pub fn set(&mut self, val: T) {
        *self = Self::Set(val);
    }

    pub fn as_mut(&mut self) -> Inheritable<&mut T> {
        match self {
            Self::Set(t) => Inheritable::Set(t),
            Self::Inherited { .. } => Inheritable::Inherited { workspace: true },
        }
    }

    /// Fails if inherited
    pub fn get_mut(&mut self) -> Result<&mut T, Error> {
        match self {
            Self::Set(t) => Ok(t),
            Self::Inherited { .. } => Err(Error::InheritedUnknownValue),
        }
    }

    /// Panics if inherited
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Self::Set(t) => t,
            Self::Inherited { .. } => panic!("inherited workspace value"),
        }
    }

    /// Copy from workspace if needed
    pub fn inherit(&mut self, other: &T) where T: Clone {
        if let Self::Inherited{..} = self {
            *self = Self::Set(other.clone());
        }
    }
}

impl<T: Default> Default for Inheritable<T> {
    fn default() -> Self {
        Self::Set(T::default())
    }
}

impl<T> Inheritable<Vec<T>> {
    /// False if inherited and unknown
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Inherited { .. } => false,
            Self::Set(v) => v.is_empty(),
        }
    }
}

impl<T: Default + PartialEq> Inheritable<T> {
    /// False if inherited and unknown
    pub fn is_default(&self) -> bool {
        match self {
            Self::Inherited { .. } => false,
            Self::Set(v) => T::default() == *v,
        }
    }
}

impl<T> From<Option<T>> for Inheritable<T> {
    /// Inherits if `None`
    fn from(val: Option<T>) -> Self {
        match val {
            Some(val) => Self::Set(val),
            None => Self::Inherited { workspace: true },
        }
    }
}

impl<T> From<Inheritable<T>> for Option<T> {
    /// `None` if inherited
    fn from(val: Inheritable<T>) -> Self {
        match val {
            Inheritable::Inherited { .. } => None,
            Inheritable::Set(val) => Some(val),
        }
    }
}
