//! Big integers are represented as an array of smaller CPU word-size integers
//! called "limbs".

#![allow(clippy::derive_hash_xor_eq)]

mod add;
mod bit_and;
mod bit_not;
mod bit_or;
mod bit_xor;
mod bits;
mod cmp;
mod encoding;
mod from;
mod mul;
mod shl;
mod shr;
mod sub;

#[cfg(feature = "rand_core")]
mod rand;

use crate::Zero;
use core::fmt;
use subtle::{Choice, ConditionallySelectable};

#[cfg(feature = "serde")]
use serdect::serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("this crate builds on 32-bit and 64-bit platforms only");

//
// 32-bit definitions
//

/// Inner integer type that the [`Limb`] newtype wraps.
#[cfg(target_pointer_width = "32")]
pub type Word = u32;

/// Signed integer type that corresponds to [`Word`].
#[cfg(target_pointer_width = "32")]
pub(crate) type SignedWord = i32;

/// Unsigned wide integer type: double the width of [`Word`].
#[cfg(target_pointer_width = "32")]
pub type WideWord = u64;

/// Signed wide integer type: double the width of [`Limb`].
#[cfg(target_pointer_width = "32")]
pub(crate) type WideSignedWord = i64;

//
// 64-bit definitions
//

/// Unsigned integer type that the [`Limb`] newtype wraps.
#[cfg(target_pointer_width = "64")]
pub type Word = u64;

/// Signed integer type that corresponds to [`Word`].
#[cfg(target_pointer_width = "64")]
pub(crate) type SignedWord = i64;

/// Wide integer type: double the width of [`Word`].
#[cfg(target_pointer_width = "64")]
pub type WideWord = u128;

/// Signed wide integer type: double the width of [`SignedWord`].
#[cfg(target_pointer_width = "64")]
pub(crate) type WideSignedWord = i128;

//
// Deprecated legacy names
//

// TODO(tarcieri): remove these in the next breaking release

/// Deprecated: unsigned integer type that the [`Limb`] newtype wraps.
#[deprecated(since = "0.4.8", note = "please use `Word` instead")]
pub type LimbUInt = Word;

/// Deprecated: wide integer type which is double the width of [`Word`].
#[deprecated(since = "0.4.8", note = "please use `WideWord` instead")]
pub type WideLimbUInt = WideWord;

/// Highest bit in a [`Limb`].
pub(crate) const HI_BIT: usize = Limb::BIT_SIZE - 1;

/// Big integers are represented as an array of smaller CPU word-size integers
/// called "limbs".
#[derive(Copy, Clone, Debug, Default, Hash)]
#[repr(transparent)]
pub struct Limb(pub Word);

impl Limb {
    /// The value `0`.
    pub const ZERO: Self = Limb(0);

    /// The value `1`.
    pub const ONE: Self = Limb(1);

    /// Maximum value this [`Limb`] can express.
    pub const MAX: Self = Limb(Word::MAX);

    // 32-bit

    /// Size of the inner integer in bits.
    #[cfg(target_pointer_width = "32")]
    pub const BIT_SIZE: usize = 32;
    /// Size of the inner integer in bytes.
    #[cfg(target_pointer_width = "32")]
    pub const BYTE_SIZE: usize = 4;

    // 64-bit

    /// Size of the inner integer in bits.
    #[cfg(target_pointer_width = "64")]
    pub const BIT_SIZE: usize = 64;
    /// Size of the inner integer in bytes.
    #[cfg(target_pointer_width = "64")]
    pub const BYTE_SIZE: usize = 8;

    /// Return `a` if `c`==0 or `b` if `c`==`Word::MAX`.
    ///
    /// Const-friendly: we can't yet use `subtle` in `const fn` contexts.
    #[inline]
    pub(crate) const fn ct_select(a: Self, b: Self, c: Word) -> Self {
        Self(a.0 ^ (c & (a.0 ^ b.0)))
    }
}

impl ConditionallySelectable for Limb {
    #[inline]
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(Word::conditional_select(&a.0, &b.0, choice))
    }
}

impl Zero for Limb {
    const ZERO: Self = Self::ZERO;
}

impl fmt::Display for Limb {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(self, f)
    }
}

impl fmt::LowerHex for Limb {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0width$x}", &self.0, width = Self::BYTE_SIZE * 2)
    }
}

impl fmt::UpperHex for Limb {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0width$X}", &self.0, width = Self::BYTE_SIZE * 2)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Deserialize<'de> for Limb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(Word::deserialize(deserializer)?))
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> Serialize for Limb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "zeroize")]
#[cfg_attr(docsrs, doc(cfg(feature = "zeroize")))]
impl zeroize::DefaultIsZeroes for Limb {}
