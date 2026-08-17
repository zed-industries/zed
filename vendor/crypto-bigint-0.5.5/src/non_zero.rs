//! Wrapper type for non-zero integers.

use crate::{CtChoice, Encoding, Integer, Limb, Uint, Zero};
use core::{
    fmt,
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8},
    ops::Deref,
};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

#[cfg(feature = "generic-array")]
use crate::{ArrayEncoding, ByteArray};

#[cfg(feature = "rand_core")]
use {crate::Random, rand_core::CryptoRngCore};

#[cfg(feature = "serde")]
use serdect::serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

/// Wrapper type for non-zero integers.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct NonZero<T: Zero>(T);

impl NonZero<Limb> {
    /// Creates a new non-zero limb in a const context.
    /// The second return value is `FALSE` if `n` is zero, `TRUE` otherwise.
    pub const fn const_new(n: Limb) -> (Self, CtChoice) {
        (Self(n), n.ct_is_nonzero())
    }
}

impl<const LIMBS: usize> NonZero<Uint<LIMBS>> {
    /// Creates a new non-zero integer in a const context.
    /// The second return value is `FALSE` if `n` is zero, `TRUE` otherwise.
    pub const fn const_new(n: Uint<LIMBS>) -> (Self, CtChoice) {
        (Self(n), n.ct_is_nonzero())
    }
}

impl<T> NonZero<T>
where
    T: Zero,
{
    /// Create a new non-zero integer.
    pub fn new(n: T) -> CtOption<Self> {
        let is_zero = n.is_zero();
        CtOption::new(Self(n), !is_zero)
    }
}

impl<T> NonZero<T>
where
    T: Integer,
{
    /// The value `1`.
    pub const ONE: Self = Self(T::ONE);

    /// Maximum value this integer can express.
    pub const MAX: Self = Self(T::MAX);
}

impl<T> NonZero<T>
where
    T: Encoding + Zero,
{
    /// Decode from big endian bytes.
    pub fn from_be_bytes(bytes: T::Repr) -> CtOption<Self> {
        Self::new(T::from_be_bytes(bytes))
    }

    /// Decode from little endian bytes.
    pub fn from_le_bytes(bytes: T::Repr) -> CtOption<Self> {
        Self::new(T::from_le_bytes(bytes))
    }
}

#[cfg(feature = "generic-array")]
impl<T> NonZero<T>
where
    T: ArrayEncoding + Zero,
{
    /// Decode a non-zero integer from big endian bytes.
    pub fn from_be_byte_array(bytes: ByteArray<T>) -> CtOption<Self> {
        Self::new(T::from_be_byte_array(bytes))
    }

    /// Decode a non-zero integer from big endian bytes.
    pub fn from_le_byte_array(bytes: ByteArray<T>) -> CtOption<Self> {
        Self::new(T::from_be_byte_array(bytes))
    }
}

impl<T> AsRef<T> for NonZero<T>
where
    T: Zero,
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> ConditionallySelectable for NonZero<T>
where
    T: ConditionallySelectable + Zero,
{
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(T::conditional_select(&a.0, &b.0, choice))
    }
}

impl<T> ConstantTimeEq for NonZero<T>
where
    T: Zero,
{
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl<T> Deref for NonZero<T>
where
    T: Zero,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[cfg(feature = "rand_core")]
impl<T> Random for NonZero<T>
where
    T: Random + Zero,
{
    /// Generate a random `NonZero<T>`.
    fn random(mut rng: &mut impl CryptoRngCore) -> Self {
        // Use rejection sampling to eliminate zero values.
        // While this method isn't constant-time, the attacker shouldn't learn
        // anything about unrelated outputs so long as `rng` is a CSRNG.
        loop {
            if let Some(result) = Self::new(T::random(&mut rng)).into() {
                break result;
            }
        }
    }
}

impl<T> fmt::Display for NonZero<T>
where
    T: fmt::Display + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T> fmt::Binary for NonZero<T>
where
    T: fmt::Binary + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

impl<T> fmt::Octal for NonZero<T>
where
    T: fmt::Octal + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.0, f)
    }
}

impl<T> fmt::LowerHex for NonZero<T>
where
    T: fmt::LowerHex + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl<T> fmt::UpperHex for NonZero<T>
where
    T: fmt::UpperHex + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl NonZero<Limb> {
    /// Create a [`NonZero<Limb>`] from a [`NonZeroU8`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU8>` when stable
    pub const fn from_u8(n: NonZeroU8) -> Self {
        Self(Limb::from_u8(n.get()))
    }

    /// Create a [`NonZero<Limb>`] from a [`NonZeroU16`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU16>` when stable
    pub const fn from_u16(n: NonZeroU16) -> Self {
        Self(Limb::from_u16(n.get()))
    }

    /// Create a [`NonZero<Limb>`] from a [`NonZeroU32`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU32>` when stable
    pub const fn from_u32(n: NonZeroU32) -> Self {
        Self(Limb::from_u32(n.get()))
    }

    /// Create a [`NonZero<Limb>`] from a [`NonZeroU64`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU64>` when stable
    #[cfg(target_pointer_width = "64")]
    pub const fn from_u64(n: NonZeroU64) -> Self {
        Self(Limb::from_u64(n.get()))
    }
}

impl From<NonZeroU8> for NonZero<Limb> {
    fn from(integer: NonZeroU8) -> Self {
        Self::from_u8(integer)
    }
}

impl From<NonZeroU16> for NonZero<Limb> {
    fn from(integer: NonZeroU16) -> Self {
        Self::from_u16(integer)
    }
}

impl From<NonZeroU32> for NonZero<Limb> {
    fn from(integer: NonZeroU32) -> Self {
        Self::from_u32(integer)
    }
}

#[cfg(target_pointer_width = "64")]
impl From<NonZeroU64> for NonZero<Limb> {
    fn from(integer: NonZeroU64) -> Self {
        Self::from_u64(integer)
    }
}

impl<const LIMBS: usize> NonZero<Uint<LIMBS>> {
    /// Create a [`NonZero<Uint>`] from a [`Uint`] (const-friendly)
    pub const fn from_uint(n: Uint<LIMBS>) -> Self {
        let mut i = 0;
        let mut found_non_zero = false;
        while i < LIMBS {
            if n.as_limbs()[i].0 != 0 {
                found_non_zero = true;
            }
            i += 1;
        }
        assert!(found_non_zero, "found zero");
        Self(n)
    }

    /// Create a [`NonZero<Uint>`] from a [`NonZeroU8`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU8>` when stable
    pub const fn from_u8(n: NonZeroU8) -> Self {
        Self(Uint::from_u8(n.get()))
    }

    /// Create a [`NonZero<Uint>`] from a [`NonZeroU16`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU16>` when stable
    pub const fn from_u16(n: NonZeroU16) -> Self {
        Self(Uint::from_u16(n.get()))
    }

    /// Create a [`NonZero<Uint>`] from a [`NonZeroU32`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU32>` when stable
    pub const fn from_u32(n: NonZeroU32) -> Self {
        Self(Uint::from_u32(n.get()))
    }

    /// Create a [`NonZero<Uint>`] from a [`NonZeroU64`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU64>` when stable
    pub const fn from_u64(n: NonZeroU64) -> Self {
        Self(Uint::from_u64(n.get()))
    }

    /// Create a [`NonZero<Uint>`] from a [`NonZeroU128`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU128>` when stable
    pub const fn from_u128(n: NonZeroU128) -> Self {
        Self(Uint::from_u128(n.get()))
    }
}

impl<const LIMBS: usize> From<NonZeroU8> for NonZero<Uint<LIMBS>> {
    fn from(integer: NonZeroU8) -> Self {
        Self::from_u8(integer)
    }
}

impl<const LIMBS: usize> From<NonZeroU16> for NonZero<Uint<LIMBS>> {
    fn from(integer: NonZeroU16) -> Self {
        Self::from_u16(integer)
    }
}

impl<const LIMBS: usize> From<NonZeroU32> for NonZero<Uint<LIMBS>> {
    fn from(integer: NonZeroU32) -> Self {
        Self::from_u32(integer)
    }
}

impl<const LIMBS: usize> From<NonZeroU64> for NonZero<Uint<LIMBS>> {
    fn from(integer: NonZeroU64) -> Self {
        Self::from_u64(integer)
    }
}

impl<const LIMBS: usize> From<NonZeroU128> for NonZero<Uint<LIMBS>> {
    fn from(integer: NonZeroU128) -> Self {
        Self::from_u128(integer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de> + Zero> Deserialize<'de> for NonZero<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: T = T::deserialize(deserializer)?;

        if bool::from(value.is_zero()) {
            Err(D::Error::invalid_value(
                Unexpected::Other("zero"),
                &"a non-zero value",
            ))
        } else {
            Ok(Self(value))
        }
    }
}

#[cfg(feature = "serde")]
impl<T: Serialize + Zero> Serialize for NonZero<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(all(test, feature = "serde"))]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::{NonZero, U64};
    use bincode::ErrorKind;

    #[test]
    fn serde() {
        let test =
            Option::<NonZero<U64>>::from(NonZero::new(U64::from_u64(0x0011223344556677))).unwrap();

        let serialized = bincode::serialize(&test).unwrap();
        let deserialized: NonZero<U64> = bincode::deserialize(&serialized).unwrap();

        assert_eq!(test, deserialized);

        let serialized = bincode::serialize(&U64::ZERO).unwrap();
        assert!(matches!(
            *bincode::deserialize::<NonZero<U64>>(&serialized).unwrap_err(),
            ErrorKind::Custom(message) if message == "invalid value: zero, expected a non-zero value"
        ));
    }

    #[test]
    fn serde_owned() {
        let test =
            Option::<NonZero<U64>>::from(NonZero::new(U64::from_u64(0x0011223344556677))).unwrap();

        let serialized = bincode::serialize(&test).unwrap();
        let deserialized: NonZero<U64> = bincode::deserialize_from(serialized.as_slice()).unwrap();

        assert_eq!(test, deserialized);

        let serialized = bincode::serialize(&U64::ZERO).unwrap();
        assert!(matches!(
            *bincode::deserialize_from::<_, NonZero<U64>>(serialized.as_slice()).unwrap_err(),
            ErrorKind::Custom(message) if message == "invalid value: zero, expected a non-zero value"
        ));
    }
}
