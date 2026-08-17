//! Compact representation of `Option<T>` for types with a reserved value.
//!
//! Small Cranelift types like the 32-bit entity references are often used in tables and linked
//! lists where an `Option<T>` is needed. Unfortunately, that would double the size of the tables
//! because `Option<T>` is twice as big as `T`.
//!
//! This module provides a `PackedOption<T>` for types that have a reserved value that can be used
//! to represent `None`.

use core::fmt;
use core::mem;

#[cfg(feature = "enable-serde")]
use serde_derive::{Deserialize, Serialize};

/// Types that have a reserved value which can't be created any other way.
pub trait ReservedValue {
    /// Create an instance of the reserved value.
    fn reserved_value() -> Self;
    /// Checks whether value is the reserved one.
    fn is_reserved_value(&self) -> bool;
}

/// Packed representation of `Option<T>`.
///
/// This is a wrapper around a `T`, using `T::reserved_value` to represent
/// `None`.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "enable-serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PackedOption<T: ReservedValue>(T);

impl<T: ReservedValue> PackedOption<T> {
    /// Returns `true` if the packed option is a `None` value.
    pub fn is_none(&self) -> bool {
        self.0.is_reserved_value()
    }

    /// Returns `true` if the packed option is a `Some` value.
    pub fn is_some(&self) -> bool {
        !self.0.is_reserved_value()
    }

    /// Expand the packed option into a normal `Option`.
    pub fn expand(self) -> Option<T> {
        if self.is_none() { None } else { Some(self.0) }
    }

    /// Maps a `PackedOption<T>` to `Option<U>` by applying a function to a contained value.
    pub fn map<U, F>(self, f: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        self.expand().map(f)
    }

    /// Unwrap a packed `Some` value or panic.
    #[track_caller]
    pub fn unwrap(self) -> T {
        self.expand().unwrap()
    }

    /// Unwrap a packed `Some` value or panic.
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        self.expand().expect(msg)
    }

    /// Takes the value out of the packed option, leaving a `None` in its place.
    pub fn take(&mut self) -> Option<T> {
        mem::replace(self, None.into()).expand()
    }
}

impl<T: ReservedValue> Default for PackedOption<T> {
    /// Create a default packed option representing `None`.
    fn default() -> Self {
        Self(T::reserved_value())
    }
}

impl<T: ReservedValue> From<T> for PackedOption<T> {
    /// Convert `t` into a packed `Some(x)`.
    fn from(t: T) -> Self {
        debug_assert!(
            !t.is_reserved_value(),
            "Can't make a PackedOption from the reserved value."
        );
        Self(t)
    }
}

impl<T: ReservedValue> From<Option<T>> for PackedOption<T> {
    /// Convert an option into its packed equivalent.
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => Self::default(),
            Some(t) => t.into(),
        }
    }
}

impl<T: ReservedValue> From<PackedOption<T>> for Option<T> {
    fn from(packed: PackedOption<T>) -> Option<T> {
        packed.expand()
    }
}

impl<T> fmt::Debug for PackedOption<T>
where
    T: ReservedValue + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_none() {
            write!(f, "None")
        } else {
            write!(f, "Some({:?})", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Dummy entity class, with no Copy or Clone.
    #[derive(Debug, PartialEq, Eq)]
    struct NoC(u32);

    impl ReservedValue for NoC {
        fn reserved_value() -> Self {
            NoC(13)
        }

        fn is_reserved_value(&self) -> bool {
            self.0 == 13
        }
    }

    #[test]
    fn moves() {
        let x = NoC(3);
        let somex: PackedOption<NoC> = x.into();
        assert!(!somex.is_none());
        assert_eq!(somex.expand(), Some(NoC(3)));

        let none: PackedOption<NoC> = None.into();
        assert!(none.is_none());
        assert_eq!(none.expand(), None);
    }

    // Dummy entity class, with Copy.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Ent(u32);

    impl ReservedValue for Ent {
        fn reserved_value() -> Self {
            Ent(13)
        }

        fn is_reserved_value(&self) -> bool {
            self.0 == 13
        }
    }

    #[test]
    fn copies() {
        let x = Ent(2);
        let some: PackedOption<Ent> = x.into();
        assert_eq!(some.expand(), x.into());
        assert_eq!(some, x.into());
    }
}
