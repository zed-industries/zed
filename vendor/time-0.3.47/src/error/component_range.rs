//! Component range error

use core::fmt;

use crate::error;

/// An error type indicating that a component provided to a method was out of range, causing a
/// failure.
// i64 is the narrowest type fitting all use cases. This eliminates the need for a type parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentRange {
    /// Name of the component.
    pub(crate) name: &'static str,
    /// Whether an input with the same value could have succeeded if the values of other components
    /// were different.
    pub(crate) is_conditional: bool,
}

impl ComponentRange {
    /// Create a new `ComponentRange` error that is not conditional.
    #[inline]
    pub(crate) const fn unconditional(name: &'static str) -> Self {
        Self {
            name,
            is_conditional: false,
        }
    }

    /// Create a new `ComponentRange` error that is conditional.
    #[inline]
    pub(crate) const fn conditional(name: &'static str) -> Self {
        Self {
            name,
            is_conditional: true,
        }
    }

    /// Obtain the name of the component whose value was out of range.
    #[inline]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Whether the value's permitted range is conditional, i.e. whether an input with this
    /// value could have succeeded if the values of other components were different.
    #[inline]
    pub const fn is_conditional(self) -> bool {
        self.is_conditional
    }
}

impl fmt::Display for ComponentRange {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} was not in range", self.name)
    }
}

impl From<ComponentRange> for crate::Error {
    #[inline]
    fn from(original: ComponentRange) -> Self {
        Self::ComponentRange(original)
    }
}

impl TryFrom<crate::Error> for ComponentRange {
    type Error = error::DifferentVariant;

    #[inline]
    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::ComponentRange(err) => Ok(err),
            _ => Err(error::DifferentVariant),
        }
    }
}

/// **This trait implementation is deprecated and will be removed in a future breaking release.**
#[cfg(feature = "serde")]
impl serde_core::de::Expected for ComponentRange {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("an in-range value")
    }
}

#[cfg(feature = "serde")]
impl ComponentRange {
    /// Convert the error to a deserialization error.
    #[inline]
    pub(crate) fn into_de_error<E>(self) -> E
    where
        E: serde_core::de::Error,
    {
        serde_core::de::Error::custom(format_args!(
            "invalid {}, expected an in-range value",
            self.name
        ))
    }
}

impl core::error::Error for ComponentRange {}
