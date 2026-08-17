//! Development-related functionality.
//!
//! Helpers and types for writing tests against concrete implementations of
//! the traits in this crate.

use crate::{
    bigint::{Limb, U256},
    error::{Error, Result},
    ops::{LinearCombination, Reduce},
    pkcs8,
    rand_core::RngCore,
    sec1::{FromEncodedPoint, ToEncodedPoint},
    subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption},
    zeroize::DefaultIsZeroes,
    AffineArithmetic, AffineXCoordinate, Curve, IsHigh, PrimeCurve, ProjectiveArithmetic,
    ScalarArithmetic,
};
use core::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use ff::{Field, PrimeField};
use generic_array::arr;
use hex_literal::hex;
use pkcs8::AssociatedOid;

#[cfg(feature = "bits")]
use crate::group::ff::PrimeFieldBits;

#[cfg(feature = "jwk")]
use crate::JwkParameters;

/// Pseudo-coordinate for fixed-based scalar mult output
pub const PSEUDO_COORDINATE_FIXED_BASE_MUL: [u8; 32] =
    hex!("deadbeef00000000000000000000000000000000000000000000000000000001");

/// SEC1 encoded point.
pub type EncodedPoint = crate::sec1::EncodedPoint<MockCurve>;

/// Field element bytes.
pub type FieldBytes = crate::FieldBytes<MockCurve>;

/// Non-zero scalar value.
pub type NonZeroScalar = crate::NonZeroScalar<MockCurve>;

/// Public key.
pub type PublicKey = crate::PublicKey<MockCurve>;

/// Secret key.
pub type SecretKey = crate::SecretKey<MockCurve>;

/// Scalar core.
// TODO(tarcieri): make this the scalar type
pub type ScalarCore = crate::ScalarCore<MockCurve>;

/// Scalar bits.
#[cfg(feature = "bits")]
pub type ScalarBits = crate::ScalarBits<MockCurve>;

/// Mock elliptic curve type useful for writing tests which require a concrete
/// curve type.
///
/// Note: this type is roughly modeled off of NIST P-256, but does not provide
/// an actual cure arithmetic implementation.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct MockCurve;

impl Curve for MockCurve {
    type UInt = U256;

    const ORDER: U256 =
        U256::from_be_hex("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");
}

impl PrimeCurve for MockCurve {}

impl AffineArithmetic for MockCurve {
    type AffinePoint = AffinePoint;
}

impl ProjectiveArithmetic for MockCurve {
    type ProjectivePoint = ProjectivePoint;
}

impl ScalarArithmetic for MockCurve {
    type Scalar = Scalar;
}

impl AssociatedOid for MockCurve {
    /// OID for NIST P-256
    const OID: pkcs8::ObjectIdentifier = pkcs8::ObjectIdentifier::new_unwrap("1.2.840.10045.3.1.7");
}

#[cfg(feature = "jwk")]
#[cfg_attr(docsrs, doc(cfg(feature = "jwk")))]
impl JwkParameters for MockCurve {
    const CRV: &'static str = "P-256";
}

/// Example scalar type
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Scalar(ScalarCore);

impl Field for Scalar {
    fn random(mut rng: impl RngCore) -> Self {
        let mut bytes = FieldBytes::default();

        loop {
            rng.fill_bytes(&mut bytes);
            if let Some(scalar) = Self::from_repr(bytes).into() {
                return scalar;
            }
        }
    }

    fn zero() -> Self {
        Self(ScalarCore::ZERO)
    }

    fn one() -> Self {
        Self(ScalarCore::ONE)
    }

    fn is_zero(&self) -> Choice {
        self.0.is_zero()
    }

    #[must_use]
    fn square(&self) -> Self {
        unimplemented!();
    }

    #[must_use]
    fn double(&self) -> Self {
        self.add(self)
    }

    fn invert(&self) -> CtOption<Self> {
        unimplemented!();
    }

    fn sqrt(&self) -> CtOption<Self> {
        unimplemented!();
    }
}

impl PrimeField for Scalar {
    type Repr = FieldBytes;

    const NUM_BITS: u32 = 256;
    const CAPACITY: u32 = 255;
    const S: u32 = 4;

    fn from_repr(bytes: FieldBytes) -> CtOption<Self> {
        ScalarCore::from_be_bytes(bytes).map(Self)
    }

    fn to_repr(&self) -> FieldBytes {
        self.0.to_be_bytes()
    }

    fn is_odd(&self) -> Choice {
        self.0.is_odd()
    }

    fn multiplicative_generator() -> Self {
        7u64.into()
    }

    fn root_of_unity() -> Self {
        Self::from_repr(arr![u8;
            0xff, 0xc9, 0x7f, 0x06, 0x2a, 0x77, 0x09, 0x92, 0xba, 0x80, 0x7a, 0xce, 0x84, 0x2a,
            0x3d, 0xfc, 0x15, 0x46, 0xca, 0xd0, 0x04, 0x37, 0x8d, 0xaf, 0x05, 0x92, 0xd7, 0xfb,
            0xb4, 0x1e, 0x66, 0x02,
        ])
        .unwrap()
    }
}

#[cfg(feature = "bits")]
impl PrimeFieldBits for Scalar {
    #[cfg(target_pointer_width = "32")]
    type ReprBits = [u32; 8];

    #[cfg(target_pointer_width = "64")]
    type ReprBits = [u64; 4];

    fn to_le_bits(&self) -> ScalarBits {
        self.0.as_uint().to_words().into()
    }

    fn char_le_bits() -> ScalarBits {
        MockCurve::ORDER.to_words().into()
    }
}

impl TryFrom<U256> for Scalar {
    type Error = Error;

    fn try_from(w: U256) -> Result<Self> {
        Option::from(ScalarCore::new(w)).map(Self).ok_or(Error)
    }
}

impl From<Scalar> for U256 {
    fn from(scalar: Scalar) -> U256 {
        *scalar.0.as_uint()
    }
}

impl ConditionallySelectable for Scalar {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(ScalarCore::conditional_select(&a.0, &b.0, choice))
    }
}

impl ConstantTimeEq for Scalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl DefaultIsZeroes for Scalar {}

impl Add<Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Scalar {
        self.add(&other)
    }
}

impl Add<&Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: &Scalar) -> Scalar {
        Self(self.0.add(&other.0))
    }
}

impl AddAssign<Scalar> for Scalar {
    fn add_assign(&mut self, other: Scalar) {
        *self = *self + other;
    }
}

impl AddAssign<&Scalar> for Scalar {
    fn add_assign(&mut self, other: &Scalar) {
        *self = *self + other;
    }
}

impl Sub<Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Scalar {
        self.sub(&other)
    }
}

impl Sub<&Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: &Scalar) -> Scalar {
        Self(self.0.sub(&other.0))
    }
}

impl SubAssign<Scalar> for Scalar {
    fn sub_assign(&mut self, other: Scalar) {
        *self = *self - other;
    }
}

impl SubAssign<&Scalar> for Scalar {
    fn sub_assign(&mut self, other: &Scalar) {
        *self = *self - other;
    }
}

impl Mul<Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, _other: Scalar) -> Scalar {
        unimplemented!();
    }
}

impl Mul<&Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, _other: &Scalar) -> Scalar {
        unimplemented!();
    }
}

impl MulAssign<Scalar> for Scalar {
    fn mul_assign(&mut self, _rhs: Scalar) {
        unimplemented!();
    }
}

impl MulAssign<&Scalar> for Scalar {
    fn mul_assign(&mut self, _rhs: &Scalar) {
        unimplemented!();
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Scalar {
        Self(self.0.neg())
    }
}

impl Reduce<U256> for Scalar {
    fn from_uint_reduced(w: U256) -> Self {
        let (r, underflow) = w.sbb(&MockCurve::ORDER, Limb::ZERO);
        let underflow = Choice::from((underflow.0 >> (Limb::BIT_SIZE - 1)) as u8);
        let reduced = U256::conditional_select(&w, &r, !underflow);
        Self(ScalarCore::new(reduced).unwrap())
    }
}

impl From<u64> for Scalar {
    fn from(n: u64) -> Scalar {
        Self(n.into())
    }
}

impl From<ScalarCore> for Scalar {
    fn from(scalar: ScalarCore) -> Scalar {
        Self(scalar)
    }
}

impl From<Scalar> for FieldBytes {
    fn from(scalar: Scalar) -> Self {
        Self::from(&scalar)
    }
}

impl From<&Scalar> for FieldBytes {
    fn from(scalar: &Scalar) -> Self {
        scalar.to_repr()
    }
}

impl IsHigh for Scalar {
    fn is_high(&self) -> Choice {
        self.0.is_high()
    }
}

/// Example affine point type
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AffinePoint {
    /// Result of fixed-based scalar multiplication.
    FixedBaseOutput(Scalar),

    /// Identity.
    Identity,

    /// Base point.
    Generator,

    /// Point corresponding to a given [`EncodedPoint`].
    Other(EncodedPoint),
}

impl AffineXCoordinate<MockCurve> for AffinePoint {
    fn x(&self) -> FieldBytes {
        unimplemented!();
    }
}

impl ConstantTimeEq for AffinePoint {
    fn ct_eq(&self, _other: &Self) -> Choice {
        unimplemented!();
    }
}

impl ConditionallySelectable for AffinePoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        // Not really constant time, but this is dev code
        if choice.into() {
            *b
        } else {
            *a
        }
    }
}

impl Default for AffinePoint {
    fn default() -> Self {
        Self::Identity
    }
}

impl DefaultIsZeroes for AffinePoint {}

impl FromEncodedPoint<MockCurve> for AffinePoint {
    fn from_encoded_point(encoded_point: &EncodedPoint) -> CtOption<Self> {
        let point = if encoded_point.is_identity() {
            Self::Identity
        } else {
            Self::Other(*encoded_point)
        };

        CtOption::new(point, Choice::from(1))
    }
}

impl ToEncodedPoint<MockCurve> for AffinePoint {
    fn to_encoded_point(&self, compress: bool) -> EncodedPoint {
        match self {
            Self::FixedBaseOutput(scalar) => EncodedPoint::from_affine_coordinates(
                &scalar.to_repr(),
                &PSEUDO_COORDINATE_FIXED_BASE_MUL.into(),
                false,
            ),
            Self::Other(point) => {
                if compress == point.is_compressed() {
                    *point
                } else {
                    unimplemented!();
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl Mul<NonZeroScalar> for AffinePoint {
    type Output = AffinePoint;

    fn mul(self, _scalar: NonZeroScalar) -> Self {
        unimplemented!();
    }
}

/// Example projective point type
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectivePoint {
    /// Result of fixed-based scalar multiplication
    FixedBaseOutput(Scalar),

    /// Is this point the identity point?
    Identity,

    /// Is this point the generator point?
    Generator,

    /// Is this point a different point corresponding to a given [`AffinePoint`]
    Other(AffinePoint),
}

impl ConstantTimeEq for ProjectivePoint {
    fn ct_eq(&self, _other: &Self) -> Choice {
        unimplemented!();
    }
}

impl ConditionallySelectable for ProjectivePoint {
    fn conditional_select(_a: &Self, _b: &Self, _choice: Choice) -> Self {
        unimplemented!();
    }
}

impl Default for ProjectivePoint {
    fn default() -> Self {
        Self::Identity
    }
}

impl DefaultIsZeroes for ProjectivePoint {}

impl From<AffinePoint> for ProjectivePoint {
    fn from(point: AffinePoint) -> ProjectivePoint {
        match point {
            AffinePoint::FixedBaseOutput(scalar) => ProjectivePoint::FixedBaseOutput(scalar),
            AffinePoint::Identity => ProjectivePoint::Identity,
            AffinePoint::Generator => ProjectivePoint::Generator,
            other => ProjectivePoint::Other(other),
        }
    }
}

impl From<ProjectivePoint> for AffinePoint {
    fn from(point: ProjectivePoint) -> AffinePoint {
        group::Curve::to_affine(&point)
    }
}

impl FromEncodedPoint<MockCurve> for ProjectivePoint {
    fn from_encoded_point(_point: &EncodedPoint) -> CtOption<Self> {
        unimplemented!();
    }
}

impl ToEncodedPoint<MockCurve> for ProjectivePoint {
    fn to_encoded_point(&self, _compress: bool) -> EncodedPoint {
        unimplemented!();
    }
}

impl group::Group for ProjectivePoint {
    type Scalar = Scalar;

    fn random(_rng: impl RngCore) -> Self {
        unimplemented!();
    }

    fn identity() -> Self {
        Self::Identity
    }

    fn generator() -> Self {
        Self::Generator
    }

    fn is_identity(&self) -> Choice {
        Choice::from((self == &Self::Identity) as u8)
    }

    #[must_use]
    fn double(&self) -> Self {
        unimplemented!();
    }
}

impl group::Curve for ProjectivePoint {
    type AffineRepr = AffinePoint;

    fn to_affine(&self) -> AffinePoint {
        match self {
            Self::FixedBaseOutput(scalar) => AffinePoint::FixedBaseOutput(*scalar),
            Self::Other(affine) => *affine,
            _ => unimplemented!(),
        }
    }
}

impl LinearCombination for ProjectivePoint {}

impl Add<ProjectivePoint> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn add(self, _other: ProjectivePoint) -> ProjectivePoint {
        unimplemented!();
    }
}

impl Add<&ProjectivePoint> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn add(self, _other: &ProjectivePoint) -> ProjectivePoint {
        unimplemented!();
    }
}

impl AddAssign<ProjectivePoint> for ProjectivePoint {
    fn add_assign(&mut self, _rhs: ProjectivePoint) {
        unimplemented!();
    }
}

impl AddAssign<&ProjectivePoint> for ProjectivePoint {
    fn add_assign(&mut self, _rhs: &ProjectivePoint) {
        unimplemented!();
    }
}

impl Sub<ProjectivePoint> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn sub(self, _other: ProjectivePoint) -> ProjectivePoint {
        unimplemented!();
    }
}

impl Sub<&ProjectivePoint> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn sub(self, _other: &ProjectivePoint) -> ProjectivePoint {
        unimplemented!();
    }
}

impl SubAssign<ProjectivePoint> for ProjectivePoint {
    fn sub_assign(&mut self, _rhs: ProjectivePoint) {
        unimplemented!();
    }
}

impl SubAssign<&ProjectivePoint> for ProjectivePoint {
    fn sub_assign(&mut self, _rhs: &ProjectivePoint) {
        unimplemented!();
    }
}

impl Add<AffinePoint> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn add(self, _other: AffinePoint) -> ProjectivePoint {
        unimplemented!();
    }
}

impl Add<&AffinePoint> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn add(self, _other: &AffinePoint) -> ProjectivePoint {
        unimplemented!();
    }
}

impl AddAssign<AffinePoint> for ProjectivePoint {
    fn add_assign(&mut self, _rhs: AffinePoint) {
        unimplemented!();
    }
}

impl AddAssign<&AffinePoint> for ProjectivePoint {
    fn add_assign(&mut self, _rhs: &AffinePoint) {
        unimplemented!();
    }
}

impl Sum for ProjectivePoint {
    fn sum<I: Iterator<Item = Self>>(_iter: I) -> Self {
        unimplemented!();
    }
}

impl<'a> Sum<&'a ProjectivePoint> for ProjectivePoint {
    fn sum<I: Iterator<Item = &'a ProjectivePoint>>(_iter: I) -> Self {
        unimplemented!();
    }
}

impl Sub<AffinePoint> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn sub(self, _other: AffinePoint) -> ProjectivePoint {
        unimplemented!();
    }
}

impl Sub<&AffinePoint> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn sub(self, _other: &AffinePoint) -> ProjectivePoint {
        unimplemented!();
    }
}

impl SubAssign<AffinePoint> for ProjectivePoint {
    fn sub_assign(&mut self, _rhs: AffinePoint) {
        unimplemented!();
    }
}

impl SubAssign<&AffinePoint> for ProjectivePoint {
    fn sub_assign(&mut self, _rhs: &AffinePoint) {
        unimplemented!();
    }
}

impl Mul<Scalar> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn mul(self, scalar: Scalar) -> ProjectivePoint {
        match self {
            Self::Generator => Self::FixedBaseOutput(scalar),
            _ => unimplemented!(),
        }
    }
}

impl Mul<&Scalar> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn mul(self, scalar: &Scalar) -> ProjectivePoint {
        self * *scalar
    }
}

impl MulAssign<Scalar> for ProjectivePoint {
    fn mul_assign(&mut self, _rhs: Scalar) {
        unimplemented!();
    }
}

impl MulAssign<&Scalar> for ProjectivePoint {
    fn mul_assign(&mut self, _rhs: &Scalar) {
        unimplemented!();
    }
}

impl Neg for ProjectivePoint {
    type Output = ProjectivePoint;

    fn neg(self) -> ProjectivePoint {
        unimplemented!();
    }
}

/// Constant representing the base field modulus
/// p = 2^{224}(2^{32} − 1) + 2^{192} + 2^{96} − 1
pub const MODULUS: U256 =
    U256::from_be_hex("ffffffff00000001000000000000000000000000ffffffffffffffffffffffff");

/// Example base field element.
#[derive(Clone, Copy, Debug)]
pub struct FieldElement(pub(crate) U256);

/// Internal field element representation.
#[cfg(target_pointer_width = "32")]
type FeWords = [u32; 8];

/// Internal field element representation.
#[cfg(target_pointer_width = "64")]
type FeWords = [u64; 4];

impl_field_element!(
    FieldElement,
    FieldBytes,
    U256,
    MODULUS,
    FeWords,
    p256_from_montgomery,
    p256_to_montgomery,
    p256_add,
    p256_sub,
    p256_mul,
    p256_opp,
    p256_square
);

impl FieldElement {
    /// Returns the multiplicative inverse of self, if self is non-zero.
    pub fn invert(&self) -> CtOption<Self> {
        unimplemented!()
    }

    /// Returns the square root of self mod p, or `None` if no square root exists.
    pub fn sqrt(&self) -> CtOption<Self> {
        unimplemented!()
    }
}

const fn p256_from_montgomery(_: &FeWords) -> FeWords {
    unimplemented!()
}

const fn p256_to_montgomery(w: &FeWords) -> FeWords {
    *w
}

const fn p256_add(_: &FeWords, _: &FeWords) -> FeWords {
    unimplemented!()
}

const fn p256_sub(_: &FeWords, _: &FeWords) -> FeWords {
    unimplemented!()
}

const fn p256_mul(_: &FeWords, _: &FeWords) -> FeWords {
    unimplemented!()
}

const fn p256_opp(_: &FeWords) -> FeWords {
    unimplemented!()
}

const fn p256_square(_: &FeWords) -> FeWords {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::Scalar;
    use ff::PrimeField;
    use hex_literal::hex;

    #[test]
    fn round_trip() {
        let bytes = hex!("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721");
        let scalar = Scalar::from_repr(bytes.into()).unwrap();
        assert_eq!(&bytes, scalar.to_repr().as_slice());
    }
}
