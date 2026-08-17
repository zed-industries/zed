//! [`NonMaxU32`], a 32-bit type that can represent any value except [`u32::MAX`].
//!
//! Naga would like `Option<Handle<T>>` to be a 32-bit value, which means we
//! need to exclude some index value for use in representing [`None`]. We could
//! have [`Handle`] store a [`NonZeroU32`], but zero is a very useful value for
//! indexing. We could have a [`Handle`] store a value one greater than its index,
//! but it turns out that it's not uncommon to want to work with [`Handle`]s'
//! indices, so that bias of 1 becomes more visible than one would like.
//!
//! This module defines the type [`NonMaxU32`], for which `Option<NonMaxU32>` is
//! still a 32-bit value, but which is directly usable as a [`Handle`] index
//! type. It still uses a bias of 1 under the hood, but that fact is isolated
//! within the implementation.
//!
//! [`Handle`]: crate::arena::Handle
//! [`NonZeroU32`]: core::num::NonZeroU32
#![allow(dead_code)]

use core::num::NonZeroU32;

/// An unsigned 32-bit value known not to be [`u32::MAX`].
///
/// A `NonMaxU32` value can represent any value in the range `0 .. u32::MAX -
/// 1`, and an `Option<NonMaxU32>` is still a 32-bit value. In other words,
/// `NonMaxU32` is just like [`NonZeroU32`], except that a different value is
/// missing from the full `u32` range.
///
/// Since zero is a very useful value in indexing, `NonMaxU32` is more useful
/// for representing indices than [`NonZeroU32`].
///
/// `NonMaxU32` values and `Option<NonMaxU32>` values both occupy 32 bits.
///
/// # Serialization and Deserialization
///
/// When the appropriate Cargo features are enabled, `NonMaxU32` implements
/// [`serde::Serialize`] and [`serde::Deserialize`] in the natural way, as the
/// integer value it represents. For example, serializing
/// `NonMaxU32::new(0).unwrap()` as JSON or RON yields the string `"0"`. This is
/// the case despite `NonMaxU32`'s implementation, described below.
///
/// # Implementation
///
/// Although this should not be observable to its users, a `NonMaxU32` whose
/// value is `n` is a newtype around a [`NonZeroU32`] whose value is `n + 1`.
/// This way, the range of values that `NonMaxU32` can represent,
/// `0..=u32::MAX - 1`, is mapped to the range `1..=u32::MAX`, which is the
/// range that /// [`NonZeroU32`] can represent. (And conversely, since
/// [`u32`] addition wraps around, the value unrepresentable in `NonMaxU32`,
/// [`u32::MAX`], becomes the value unrepresentable in [`NonZeroU32`], `0`.)
///
/// [`NonZeroU32`]: core::num::NonZeroU32
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct NonMaxU32(NonZeroU32);

impl NonMaxU32 {
    /// Construct a [`NonMaxU32`] whose value is `n`, if possible.
    pub const fn new(n: u32) -> Option<Self> {
        // If `n` is `u32::MAX`, then `n.wrapping_add(1)` is `0`,
        // so `NonZeroU32::new` returns `None` in exactly the case
        // where we must return `None`.
        match NonZeroU32::new(n.wrapping_add(1)) {
            Some(non_zero) => Some(NonMaxU32(non_zero)),
            None => None,
        }
    }

    /// Return the value of `self` as a [`u32`].
    pub const fn get(self) -> u32 {
        self.0.get() - 1
    }

    pub fn checked_add(self, n: u32) -> Option<Self> {
        // Adding `n` to `self` produces `u32::MAX` if and only if
        // adding `n` to `self.0` produces `0`. So we can simply
        // call `NonZeroU32::checked_add` and let its check for zero
        // determine whether our add would have produced `u32::MAX`.
        Some(NonMaxU32(self.0.checked_add(n)?))
    }
}

impl core::fmt::Debug for NonMaxU32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}

impl core::fmt::Display for NonMaxU32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}

#[cfg(feature = "serialize")]
impl serde::Serialize for NonMaxU32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.get())
    }
}

#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for NonMaxU32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Defer to `u32`'s `Deserialize` implementation.
        let n = <u32 as serde::Deserialize>::deserialize(deserializer)?;

        // Constrain the range of the value further.
        NonMaxU32::new(n).ok_or_else(|| {
            <D::Error as serde::de::Error>::invalid_value(
                serde::de::Unexpected::Unsigned(n as u64),
                &"a value no less than 0 and no greater than 4294967294 (2^32 - 2)",
            )
        })
    }
}

#[test]
fn size() {
    assert_eq!(size_of::<Option<NonMaxU32>>(), size_of::<u32>());
}
