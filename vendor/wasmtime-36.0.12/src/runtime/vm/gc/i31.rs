//! Implementation of unboxed 31-bit integers.

use super::VMGcRef;
use core::fmt;
use wasmtime_environ::Unsigned;

/// A 31-bit integer for use with `i31ref`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct I31(pub(super) u32);

impl Default for I31 {
    #[inline]
    fn default() -> Self {
        Self::wrapping_u32(0)
    }
}

impl fmt::Debug for I31 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("I31")
            .field("as_u32", &self.get_u32())
            .field("as_i32", &self.get_i32())
            .finish()
    }
}

impl I31 {
    const DISCRIMINANT: u32 = VMGcRef::I31_REF_DISCRIMINANT;

    /// Construct a new `I31` from the given unsigned value.
    ///
    /// Returns `None` if the value does not fit in the bottom 31 bits.
    #[inline]
    pub fn new_u32(value: u32) -> Option<Self> {
        if ((value << 1) >> 1) == value {
            let i31 = Self::wrapping_u32(value);
            debug_assert_eq!(i31.get_u32(), value);
            Some(i31)
        } else {
            None
        }
    }

    /// Construct a new `I31` from the given signed value.
    ///
    /// Returns `None` if the value does not fit in the bottom 31 bits.
    #[inline]
    pub fn new_i32(value: i32) -> Option<Self> {
        if ((value << 1) >> 1) == value {
            let i31 = Self::wrapping_i32(value);
            debug_assert_eq!(i31.get_i32(), value);
            Some(i31)
        } else {
            None
        }
    }

    /// Construct a new `I31` from the given unsigned value.
    ///
    /// If the value doesn't fit in the bottom 31 bits, it is wrapped such that
    /// the wrapped value does.
    #[inline]
    pub fn wrapping_u32(value: u32) -> Self {
        Self((value << 1) | Self::DISCRIMINANT)
    }

    /// Construct a new `I31` from the given signed value.
    ///
    /// If the value doesn't fit in the bottom 31 bits, it is wrapped such that
    /// the wrapped value does.
    #[inline]
    pub fn wrapping_i32(value: i32) -> Self {
        Self::wrapping_u32(value.unsigned())
    }

    /// Get this `I31`'s value as an unsigned integer.
    #[inline]
    pub fn get_u32(&self) -> u32 {
        self.0 >> 1
    }

    /// Get this `I31`'s value as ansigned integer.
    #[inline]
    pub fn get_i32(&self) -> i32 {
        (self.0 as i32) >> 1
    }
}
