//! Big unsigned integers.

#![allow(
    clippy::needless_range_loop,
    clippy::many_single_char_names,
    clippy::derive_hash_xor_eq
)]

#[macro_use]
mod concat;
#[macro_use]
mod split;

mod add;
mod add_mod;
mod bit_and;
mod bit_not;
mod bit_or;
mod bit_xor;
mod bits;
mod cmp;
mod div;
mod encoding;
mod from;
mod inv_mod;
mod mul;
mod mul_mod;
mod neg_mod;
mod resize;
mod shl;
mod shr;
mod sqrt;
mod sub;
mod sub_mod;

#[cfg(feature = "generic-array")]
mod array;

#[cfg(feature = "rand_core")]
mod rand;

use crate::{Concat, Encoding, Integer, Limb, Split, Word, Zero};
use core::{fmt, mem};
use subtle::{Choice, ConditionallySelectable};

#[cfg(feature = "serde")]
use serdect::serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "zeroize")]
use zeroize::DefaultIsZeroes;

/// Big unsigned integer.
///
/// Generic over the given number of `LIMBS`
///
/// # Encoding support
/// This type supports many different types of encodings, either via the
/// [`Encoding`][`crate::Encoding`] trait or various `const fn` decoding and
/// encoding functions that can be used with [`UInt`] constants.
///
/// Optional crate features for encoding (off-by-default):
/// - `generic-array`: enables [`ArrayEncoding`][`crate::ArrayEncoding`] trait which can be used to
///   [`UInt`] as `GenericArray<u8, N>` and a [`ArrayDecoding`][`crate::ArrayDecoding`] trait which
///   can be used to `GenericArray<u8, N>` as [`UInt`].
/// - `rlp`: support for [Recursive Length Prefix (RLP)][RLP] encoding.
///
/// [RLP]: https://eth.wiki/fundamentals/rlp
// TODO(tarcieri): make generic around a specified number of bits.
#[derive(Copy, Clone, Debug, Hash)]
pub struct UInt<const LIMBS: usize> {
    /// Inner limb array. Stored from least significant to most significant.
    limbs: [Limb; LIMBS],
}

impl<const LIMBS: usize> UInt<LIMBS> {
    /// The value `0`.
    pub const ZERO: Self = Self::from_u8(0);

    /// The value `1`.
    pub const ONE: Self = Self::from_u8(1);

    /// The number of limbs used on this platform.
    pub const LIMBS: usize = LIMBS;

    /// Maximum value this [`UInt`] can express.
    pub const MAX: Self = Self {
        limbs: [Limb::MAX; LIMBS],
    };

    /// Const-friendly [`UInt`] constructor.
    pub const fn new(limbs: [Limb; LIMBS]) -> Self {
        Self { limbs }
    }

    /// Create a [`UInt`] from an array of [`Word`]s (i.e. word-sized unsigned
    /// integers).
    #[inline]
    pub const fn from_words(arr: [Word; LIMBS]) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            limbs[i] = Limb(arr[i]);
            i += 1;
        }

        Self { limbs }
    }

    /// Create an array of [`Word`]s (i.e. word-sized unsigned integers) from
    /// a [`UInt`].
    #[inline]
    pub const fn to_words(self) -> [Word; LIMBS] {
        let mut arr = [0; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            arr[i] = self.limbs[i].0;
            i += 1;
        }

        arr
    }

    /// Borrow the inner limbs as an array of [`Word`]s.
    pub const fn as_words(&self) -> &[Word; LIMBS] {
        // SAFETY: `Limb` is a `repr(transparent)` newtype for `Word`
        #[allow(unsafe_code)]
        unsafe {
            // TODO(tarcieri): use &*((&self.limbs as *const _) as *const [Word; LIMBS])
            mem::transmute(&self.limbs)
        }
    }

    /// Borrow the inner limbs as a mutable array of [`Word`]s.
    pub fn as_words_mut(&mut self) -> &mut [Word; LIMBS] {
        // SAFETY: `Limb` is a `repr(transparent)` newtype for `Word`
        #[allow(trivial_casts, unsafe_code)]
        unsafe {
            &mut *((&mut self.limbs as *mut _) as *mut [Word; LIMBS])
        }
    }

    /// Deprecated: borrow the inner limbs as an array of [`Word`]s.
    #[deprecated(since = "0.4.8", note = "please use `as_words` instead")]
    pub const fn as_uint_array(&self) -> &[Word; LIMBS] {
        self.as_words()
    }

    /// Deprecated: create a [`UInt`] from an array of [`Word`]s.
    #[deprecated(since = "0.4.8", note = "please use `from_words` instead")]
    pub const fn from_uint_array(words: [Word; LIMBS]) -> Self {
        Self::from_words(words)
    }

    /// Deprecated: create an array of [`Word`]s from a [`UInt`].
    #[deprecated(since = "0.4.8", note = "please use `to_words` instead")]
    pub const fn to_uint_array(self) -> [Word; LIMBS] {
        self.to_words()
    }

    /// Borrow the limbs of this [`UInt`].
    // TODO(tarcieri): rename to `as_limbs` for consistency with `as_words`
    pub const fn limbs(&self) -> &[Limb; LIMBS] {
        &self.limbs
    }

    /// Borrow the limbs of this [`UInt`] mutably.
    // TODO(tarcieri): rename to `as_limbs_mut` for consistency with `as_words_mut`
    pub fn limbs_mut(&mut self) -> &mut [Limb; LIMBS] {
        &mut self.limbs
    }

    /// Convert this [`UInt`] into its inner limbs.
    // TODO(tarcieri): rename to `to_limbs` for consistency with `to_words`
    pub const fn into_limbs(self) -> [Limb; LIMBS] {
        self.limbs
    }
}

impl<const LIMBS: usize> AsRef<[Word; LIMBS]> for UInt<LIMBS> {
    fn as_ref(&self) -> &[Word; LIMBS] {
        self.as_words()
    }
}

impl<const LIMBS: usize> AsMut<[Word; LIMBS]> for UInt<LIMBS> {
    fn as_mut(&mut self) -> &mut [Word; LIMBS] {
        self.as_words_mut()
    }
}

// TODO(tarcieri): eventually phase this out in favor of `limbs()`?
impl<const LIMBS: usize> AsRef<[Limb]> for UInt<LIMBS> {
    fn as_ref(&self) -> &[Limb] {
        self.limbs()
    }
}

// TODO(tarcieri): eventually phase this out in favor of `limbs_mut()`?
impl<const LIMBS: usize> AsMut<[Limb]> for UInt<LIMBS> {
    fn as_mut(&mut self) -> &mut [Limb] {
        self.limbs_mut()
    }
}

impl<const LIMBS: usize> ConditionallySelectable for UInt<LIMBS> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];

        for i in 0..LIMBS {
            limbs[i] = Limb::conditional_select(&a.limbs[i], &b.limbs[i], choice);
        }

        Self { limbs }
    }
}

impl<const LIMBS: usize> Default for UInt<LIMBS> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const LIMBS: usize> Integer for UInt<LIMBS> {
    const ONE: Self = Self::ONE;
    const MAX: Self = Self::MAX;

    fn is_odd(&self) -> Choice {
        self.limbs
            .first()
            .map(|limb| limb.is_odd())
            .unwrap_or_else(|| Choice::from(0))
    }
}

impl<const LIMBS: usize> Zero for UInt<LIMBS> {
    const ZERO: Self = Self::ZERO;
}

impl<const LIMBS: usize> fmt::Display for UInt<LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(self, f)
    }
}

impl<const LIMBS: usize> fmt::LowerHex for UInt<LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for limb in self.limbs.iter().rev() {
            fmt::LowerHex::fmt(limb, f)?;
        }
        Ok(())
    }
}

impl<const LIMBS: usize> fmt::UpperHex for UInt<LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for limb in self.limbs.iter().rev() {
            fmt::UpperHex::fmt(limb, f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de, const LIMBS: usize> Deserialize<'de> for UInt<LIMBS>
where
    UInt<LIMBS>: Encoding,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut buffer = Self::ZERO.to_le_bytes();
        serdect::array::deserialize_hex_or_bin(buffer.as_mut(), deserializer)?;

        Ok(Self::from_le_bytes(buffer))
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de, const LIMBS: usize> Serialize for UInt<LIMBS>
where
    UInt<LIMBS>: Encoding,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serdect::array::serialize_hex_lower_or_bin(&Encoding::to_le_bytes(self), serializer)
    }
}

#[cfg(feature = "zeroize")]
#[cfg_attr(docsrs, doc(cfg(feature = "zeroize")))]
impl<const LIMBS: usize> DefaultIsZeroes for UInt<LIMBS> {}

// TODO(tarcieri): use `const_evaluatable_checked` when stable to make generic around bits.
macro_rules! impl_uint_aliases {
    ($(($name:ident, $bits:expr, $doc:expr)),+) => {
        $(
            #[doc = $doc]
            #[doc="unsigned big integer."]
            pub type $name = UInt<{nlimbs!($bits)}>;

            impl Encoding for $name {
                const BIT_SIZE: usize = $bits;
                const BYTE_SIZE: usize = $bits / 8;

                type Repr = [u8; $bits / 8];

                #[inline]
                fn from_be_bytes(bytes: Self::Repr) -> Self {
                    Self::from_be_slice(&bytes)
                }

                #[inline]
                fn from_le_bytes(bytes: Self::Repr) -> Self {
                    Self::from_le_slice(&bytes)
                }

                #[inline]
                fn to_be_bytes(&self) -> Self::Repr {
                    let mut result = [0u8; $bits / 8];
                    self.write_be_bytes(&mut result);
                    result
                }

                #[inline]
                fn to_le_bytes(&self) -> Self::Repr {
                    let mut result = [0u8; $bits / 8];
                    self.write_le_bytes(&mut result);
                    result
                }
            }
        )+
     };
}

// TODO(tarcieri): use `const_evaluatable_checked` when stable to make generic around bits.
impl_uint_aliases! {
    (U64, 64, "64-bit"),
    (U128, 128, "128-bit"),
    (U192, 192, "192-bit"),
    (U256, 256, "256-bit"),
    (U384, 384, "384-bit"),
    (U448, 448, "448-bit"),
    (U512, 512, "512-bit"),
    (U576, 576, "576-bit"),
    (U768, 768, "768-bit"),
    (U896, 896, "896-bit"),
    (U1024, 1024, "1024-bit"),
    (U1536, 1536, "1536-bit"),
    (U1792, 1792, "1792-bit"),
    (U2048, 2048, "2048-bit"),
    (U3072, 3072, "3072-bit"),
    (U3584, 3584, "3584-bit"),
    (U4096, 4096, "4096-bit"),
    (U6144, 6144, "6144-bit"),
    (U8192, 8192, "8192-bit")
}

// TODO(tarcieri): use `const_evaluatable_checked` when stable to make generic around bits.
impl_concat! {
    (U64, 64),
    (U128, 128),
    (U192, 192),
    (U256, 256),
    (U384, 384),
    (U448, 448),
    (U512, 512),
    (U768, 768),
    (U896, 896),
    (U1024, 1024),
    (U1536, 1536),
    (U1792, 1792),
    (U2048, 2048),
    (U3072, 3072),
    (U4096, 4096)
}

// TODO(tarcieri): use `const_evaluatable_checked` when stable to make generic around bits.
impl_split! {
    (U128, 128),
    (U192, 192),
    (U256, 256),
    (U384, 384),
    (U448, 448),
    (U512, 512),
    (U768, 768),
    (U896, 896),
    (U1024, 1024),
    (U1536, 1536),
    (U1792, 1792),
    (U2048, 2048),
    (U3072, 3072),
    (U3584, 3584),
    (U4096, 4096),
    (U6144, 6144),
    (U8192, 8192)
}

#[cfg(test)]
mod tests {
    use crate::{Encoding, U128};
    use subtle::ConditionallySelectable;

    #[cfg(feature = "serde")]
    use crate::U64;

    #[test]
    #[cfg(feature = "alloc")]
    fn display() {
        let hex = "AAAAAAAABBBBBBBBCCCCCCCCDDDDDDDD";
        let n = U128::from_be_hex(hex);

        use alloc::string::ToString;
        assert_eq!(hex, n.to_string());

        let hex = "AAAAAAAABBBBBBBB0000000000000000";
        let n = U128::from_be_hex(hex);
        assert_eq!(hex, n.to_string());

        let hex = "AAAAAAAABBBBBBBB00000000DDDDDDDD";
        let n = U128::from_be_hex(hex);
        assert_eq!(hex, n.to_string());

        let hex = "AAAAAAAABBBBBBBB0CCCCCCCDDDDDDDD";
        let n = U128::from_be_hex(hex);
        assert_eq!(hex, n.to_string());
    }

    #[test]
    fn from_bytes() {
        let a = U128::from_be_hex("AAAAAAAABBBBBBBB0CCCCCCCDDDDDDDD");

        let be_bytes = a.to_be_bytes();
        let le_bytes = a.to_le_bytes();
        for i in 0..16 {
            assert_eq!(le_bytes[i], be_bytes[15 - i]);
        }

        let a_from_be = U128::from_be_bytes(be_bytes);
        let a_from_le = U128::from_le_bytes(le_bytes);
        assert_eq!(a_from_be, a_from_le);
        assert_eq!(a_from_be, a);
    }

    #[test]
    fn conditional_select() {
        let a = U128::from_be_hex("00002222444466668888AAAACCCCEEEE");
        let b = U128::from_be_hex("11113333555577779999BBBBDDDDFFFF");

        let select_0 = U128::conditional_select(&a, &b, 0.into());
        assert_eq!(a, select_0);

        let select_1 = U128::conditional_select(&a, &b, 1.into());
        assert_eq!(b, select_1);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde() {
        const TEST: U64 = U64::from_u64(0x0011223344556677);

        let serialized = bincode::serialize(&TEST).unwrap();
        let deserialized: U64 = bincode::deserialize(&serialized).unwrap();

        assert_eq!(TEST, deserialized);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_owned() {
        const TEST: U64 = U64::from_u64(0x0011223344556677);

        let serialized = bincode::serialize(&TEST).unwrap();
        let deserialized: U64 = bincode::deserialize_from(serialized.as_slice()).unwrap();

        assert_eq!(TEST, deserialized);
    }
}
