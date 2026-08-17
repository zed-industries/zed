//! Wrapping arithmetic.

use crate::Zero;
use core::fmt;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

#[cfg(feature = "rand_core")]
use {crate::Random, rand_core::CryptoRngCore};

#[cfg(feature = "serde")]
use serdect::serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Provides intentionally-wrapped arithmetic on `T`.
///
/// This is analogous to [`core::num::Wrapping`] but allows this crate to
/// define trait impls for this type.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Wrapping<T>(pub T);

impl<T: Zero> Zero for Wrapping<T> {
    const ZERO: Self = Self(T::ZERO);
}

impl<T: fmt::Display> fmt::Display for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Binary> fmt::Binary for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Octal> fmt::Octal for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::LowerHex> fmt::LowerHex for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::UpperHex> fmt::UpperHex for Wrapping<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: ConditionallySelectable> ConditionallySelectable for Wrapping<T> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Wrapping(T::conditional_select(&a.0, &b.0, choice))
    }
}

impl<T: ConstantTimeEq> ConstantTimeEq for Wrapping<T> {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

#[cfg(feature = "rand_core")]
impl<T: Random> Random for Wrapping<T> {
    fn random(rng: &mut impl CryptoRngCore) -> Self {
        Wrapping(Random::random(rng))
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Wrapping<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(T::deserialize(deserializer)?))
    }
}

#[cfg(feature = "serde")]
impl<T: Serialize> Serialize for Wrapping<T> {
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
    use crate::{Wrapping, U64};

    #[test]
    fn serde() {
        const TEST: Wrapping<U64> = Wrapping(U64::from_u64(0x0011223344556677));

        let serialized = bincode::serialize(&TEST).unwrap();
        let deserialized: Wrapping<U64> = bincode::deserialize(&serialized).unwrap();

        assert_eq!(TEST, deserialized);
    }

    #[test]
    fn serde_owned() {
        const TEST: Wrapping<U64> = Wrapping(U64::from_u64(0x0011223344556677));

        let serialized = bincode::serialize(&TEST).unwrap();
        let deserialized: Wrapping<U64> = bincode::deserialize_from(serialized.as_slice()).unwrap();

        assert_eq!(TEST, deserialized);
    }
}
