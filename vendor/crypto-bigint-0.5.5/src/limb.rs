//! Big integers are represented as an array of smaller CPU word-size integers
//! called "limbs".

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
mod neg;
mod shl;
mod shr;
mod sub;

#[cfg(feature = "rand_core")]
mod rand;

use crate::{Bounded, Zero};
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

/// Unsigned wide integer type: double the width of [`Word`].
#[cfg(target_pointer_width = "32")]
pub type WideWord = u64;

//
// 64-bit definitions
//

/// Unsigned integer type that the [`Limb`] newtype wraps.
#[cfg(target_pointer_width = "64")]
pub type Word = u64;

/// Wide integer type: double the width of [`Word`].
#[cfg(target_pointer_width = "64")]
pub type WideWord = u128;

/// Highest bit in a [`Limb`].
pub(crate) const HI_BIT: usize = Limb::BITS - 1;

/// Big integers are represented as an array of smaller CPU word-size integers
/// called "limbs".
// Our PartialEq impl only differs from the default one by being constant-time, so this is safe
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Copy, Clone, Default, Hash)]
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
    pub const BITS: usize = 32;
    /// Size of the inner integer in bytes.
    #[cfg(target_pointer_width = "32")]
    pub const BYTES: usize = 4;

    // 64-bit

    /// Size of the inner integer in bits.
    #[cfg(target_pointer_width = "64")]
    pub const BITS: usize = 64;
    /// Size of the inner integer in bytes.
    #[cfg(target_pointer_width = "64")]
    pub const BYTES: usize = 8;
}

impl Bounded for Limb {
    const BITS: usize = Self::BITS;
    const BYTES: usize = Self::BYTES;
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

impl fmt::Debug for Limb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Limb(0x{self:X})")
    }
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
        write!(f, "{:0width$x}", &self.0, width = Self::BYTES * 2)
    }
}

impl fmt::UpperHex for Limb {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0width$X}", &self.0, width = Self::BYTES * 2)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Limb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(Word::deserialize(deserializer)?))
    }
}

#[cfg(feature = "serde")]
impl Serialize for Limb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "zeroize")]
impl zeroize::DefaultIsZeroes for Limb {}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    use {super::Limb, alloc::format};

    #[cfg(feature = "alloc")]
    #[test]
    fn debug() {
        #[cfg(target_pointer_width = "32")]
        assert_eq!(format!("{:?}", Limb(42)), "Limb(0x0000002A)");

        #[cfg(target_pointer_width = "64")]
        assert_eq!(format!("{:?}", Limb(42)), "Limb(0x000000000000002A)");
    }
}
