//! Generic scalar type with core functionality.

use crate::{
    bigint::{prelude::*, Limb, NonZero},
    rand_core::{CryptoRng, RngCore},
    subtle::{
        Choice, ConditionallySelectable, ConstantTimeEq, ConstantTimeGreater, ConstantTimeLess,
        CtOption,
    },
    Curve, Error, FieldBytes, IsHigh, Result,
};
use base16ct::HexDisplay;
use core::{
    cmp::Ordering,
    fmt,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
    str,
};
use generic_array::GenericArray;
use zeroize::DefaultIsZeroes;

#[cfg(feature = "arithmetic")]
use {
    super::{Scalar, ScalarArithmetic},
    group::ff::PrimeField,
};

#[cfg(feature = "serde")]
use serdect::serde::{de, ser, Deserialize, Serialize};

/// Generic scalar type with core functionality.
///
/// This type provides a baseline level of scalar arithmetic functionality
/// which is always available for all curves, regardless of if they implement
/// any arithmetic traits.
///
/// # `serde` support
///
/// When the optional `serde` feature of this create is enabled, [`Serialize`]
/// and [`Deserialize`] impls are provided for this type.
///
/// The serialization is a fixed-width big endian encoding. When used with
/// textual formats, the binary data is encoded as hexadecimal.
// TODO(tarcieri): make this a fully generic `Scalar` type and use it for `ScalarArithmetic`
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
pub struct ScalarCore<C: Curve> {
    /// Inner unsigned integer type.
    inner: C::UInt,
}

impl<C> ScalarCore<C>
where
    C: Curve,
{
    /// Zero scalar.
    pub const ZERO: Self = Self {
        inner: C::UInt::ZERO,
    };

    /// Multiplicative identity.
    pub const ONE: Self = Self {
        inner: C::UInt::ONE,
    };

    /// Scalar modulus.
    pub const MODULUS: C::UInt = C::ORDER;

    /// Generate a random [`ScalarCore`].
    pub fn random(rng: impl CryptoRng + RngCore) -> Self {
        Self {
            inner: C::UInt::random_mod(rng, &NonZero::new(Self::MODULUS).unwrap()),
        }
    }

    /// Create a new scalar from [`Curve::UInt`].
    pub fn new(uint: C::UInt) -> CtOption<Self> {
        CtOption::new(Self { inner: uint }, uint.ct_lt(&Self::MODULUS))
    }

    /// Decode [`ScalarCore`] from big endian bytes.
    pub fn from_be_bytes(bytes: FieldBytes<C>) -> CtOption<Self> {
        Self::new(C::UInt::from_be_byte_array(bytes))
    }

    /// Decode [`ScalarCore`] from a big endian byte slice.
    pub fn from_be_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() == C::UInt::BYTE_SIZE {
            Option::from(Self::from_be_bytes(GenericArray::clone_from_slice(slice))).ok_or(Error)
        } else {
            Err(Error)
        }
    }

    /// Decode [`ScalarCore`] from little endian bytes.
    pub fn from_le_bytes(bytes: FieldBytes<C>) -> CtOption<Self> {
        Self::new(C::UInt::from_le_byte_array(bytes))
    }

    /// Decode [`ScalarCore`] from a little endian byte slice.
    pub fn from_le_slice(slice: &[u8]) -> Result<Self> {
        if slice.len() == C::UInt::BYTE_SIZE {
            Option::from(Self::from_le_bytes(GenericArray::clone_from_slice(slice))).ok_or(Error)
        } else {
            Err(Error)
        }
    }

    /// Borrow the inner `C::UInt`.
    pub fn as_uint(&self) -> &C::UInt {
        &self.inner
    }

    /// Borrow the inner limbs as a slice.
    pub fn as_limbs(&self) -> &[Limb] {
        self.inner.as_ref()
    }

    /// Is this [`ScalarCore`] value equal to zero?
    pub fn is_zero(&self) -> Choice {
        self.inner.is_zero()
    }

    /// Is this [`ScalarCore`] value even?
    pub fn is_even(&self) -> Choice {
        self.inner.is_even()
    }

    /// Is this [`ScalarCore`] value odd?
    pub fn is_odd(&self) -> Choice {
        self.inner.is_odd()
    }

    /// Encode [`ScalarCore`] as big endian bytes.
    pub fn to_be_bytes(self) -> FieldBytes<C> {
        self.inner.to_be_byte_array()
    }

    /// Encode [`ScalarCore`] as little endian bytes.
    pub fn to_le_bytes(self) -> FieldBytes<C> {
        self.inner.to_le_byte_array()
    }
}

#[cfg(feature = "arithmetic")]
impl<C> ScalarCore<C>
where
    C: Curve + ScalarArithmetic,
{
    /// Convert [`ScalarCore`] into a given curve's scalar type
    // TODO(tarcieri): replace curve-specific scalars with `ScalarCore`
    pub(super) fn to_scalar(self) -> Scalar<C> {
        Scalar::<C>::from_repr(self.to_be_bytes()).unwrap()
    }
}

// TODO(tarcieri): better encapsulate this?
impl<C> AsRef<[Limb]> for ScalarCore<C>
where
    C: Curve,
{
    fn as_ref(&self) -> &[Limb] {
        self.as_limbs()
    }
}

impl<C> ConditionallySelectable for ScalarCore<C>
where
    C: Curve,
{
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            inner: C::UInt::conditional_select(&a.inner, &b.inner, choice),
        }
    }
}

impl<C> ConstantTimeEq for ScalarCore<C>
where
    C: Curve,
{
    fn ct_eq(&self, other: &Self) -> Choice {
        self.inner.ct_eq(&other.inner)
    }
}

impl<C> ConstantTimeLess for ScalarCore<C>
where
    C: Curve,
{
    fn ct_lt(&self, other: &Self) -> Choice {
        self.inner.ct_lt(&other.inner)
    }
}

impl<C> ConstantTimeGreater for ScalarCore<C>
where
    C: Curve,
{
    fn ct_gt(&self, other: &Self) -> Choice {
        self.inner.ct_gt(&other.inner)
    }
}

impl<C: Curve> DefaultIsZeroes for ScalarCore<C> {}

impl<C: Curve> Eq for ScalarCore<C> {}

impl<C> PartialEq for ScalarCore<C>
where
    C: Curve,
{
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

impl<C> PartialOrd for ScalarCore<C>
where
    C: Curve,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C> Ord for ScalarCore<C>
where
    C: Curve,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<C> From<u64> for ScalarCore<C>
where
    C: Curve,
{
    fn from(n: u64) -> Self {
        Self {
            inner: C::UInt::from(n),
        }
    }
}

impl<C> Add<ScalarCore<C>> for ScalarCore<C>
where
    C: Curve,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.add(&other)
    }
}

impl<C> Add<&ScalarCore<C>> for ScalarCore<C>
where
    C: Curve,
{
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self {
            inner: self.inner.add_mod(&other.inner, &Self::MODULUS),
        }
    }
}

impl<C> AddAssign<ScalarCore<C>> for ScalarCore<C>
where
    C: Curve,
{
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<C> AddAssign<&ScalarCore<C>> for ScalarCore<C>
where
    C: Curve,
{
    fn add_assign(&mut self, other: &Self) {
        *self = *self + other;
    }
}

impl<C> Sub<ScalarCore<C>> for ScalarCore<C>
where
    C: Curve,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.sub(&other)
    }
}

impl<C> Sub<&ScalarCore<C>> for ScalarCore<C>
where
    C: Curve,
{
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self {
            inner: self.inner.sub_mod(&other.inner, &Self::MODULUS),
        }
    }
}

impl<C> SubAssign<ScalarCore<C>> for ScalarCore<C>
where
    C: Curve,
{
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<C> SubAssign<&ScalarCore<C>> for ScalarCore<C>
where
    C: Curve,
{
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - other;
    }
}

impl<C> Neg for ScalarCore<C>
where
    C: Curve,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            inner: self.inner.neg_mod(&Self::MODULUS),
        }
    }
}

impl<C> Neg for &ScalarCore<C>
where
    C: Curve,
{
    type Output = ScalarCore<C>;

    fn neg(self) -> ScalarCore<C> {
        -*self
    }
}

impl<C> IsHigh for ScalarCore<C>
where
    C: Curve,
{
    fn is_high(&self) -> Choice {
        let n_2 = C::ORDER >> 1;
        self.inner.ct_gt(&n_2)
    }
}

impl<C> fmt::Display for ScalarCore<C>
where
    C: Curve,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl<C> fmt::LowerHex for ScalarCore<C>
where
    C: Curve,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", HexDisplay(&self.to_be_bytes()))
    }
}

impl<C> fmt::UpperHex for ScalarCore<C>
where
    C: Curve,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", HexDisplay(&self.to_be_bytes()))
    }
}

impl<C> str::FromStr for ScalarCore<C>
where
    C: Curve,
{
    type Err = Error;

    fn from_str(hex: &str) -> Result<Self> {
        let mut bytes = FieldBytes::<C>::default();
        base16ct::lower::decode(hex, &mut bytes)?;
        Option::from(Self::from_be_bytes(bytes)).ok_or(Error)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<C> Serialize for ScalarCore<C>
where
    C: Curve,
{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serdect::array::serialize_hex_upper_or_bin(&self.to_be_bytes(), serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de, C> Deserialize<'de> for ScalarCore<C>
where
    C: Curve,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let mut bytes = FieldBytes::<C>::default();
        serdect::array::deserialize_hex_or_bin(&mut bytes, deserializer)?;
        Option::from(Self::from_be_bytes(bytes))
            .ok_or_else(|| de::Error::custom("scalar out of range"))
    }
}
