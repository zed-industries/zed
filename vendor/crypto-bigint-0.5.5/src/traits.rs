//! Traits provided by this crate

use crate::{Limb, NonZero};
use core::fmt::Debug;
use core::ops::{BitAnd, BitOr, BitXor, Div, Not, Rem, Shl, Shr};
use subtle::{
    Choice, ConditionallySelectable, ConstantTimeEq, ConstantTimeGreater, ConstantTimeLess,
    CtOption,
};

#[cfg(feature = "rand_core")]
use rand_core::CryptoRngCore;

/// Integer type.
pub trait Integer:
    'static
    + AsRef<[Limb]>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + for<'a> CheckedAdd<&'a Self, Output = Self>
    + for<'a> CheckedSub<&'a Self, Output = Self>
    + for<'a> CheckedMul<&'a Self, Output = Self>
    + Copy
    + ConditionallySelectable
    + ConstantTimeEq
    + ConstantTimeGreater
    + ConstantTimeLess
    + Debug
    + Default
    + Div<NonZero<Self>, Output = Self>
    + Eq
    + From<u64>
    + Not
    + Ord
    + Rem<NonZero<Self>, Output = Self>
    + Send
    + Sized
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Sync
    + Zero
{
    /// The value `1`.
    const ONE: Self;

    /// Maximum value this integer can express.
    const MAX: Self;

    /// Total size of the represented integer in bits.
    const BITS: usize;

    /// Total size of the represented integer in bytes.
    const BYTES: usize;

    /// The number of limbs used on this platform.
    const LIMBS: usize;

    /// Is this integer value an odd number?
    ///
    /// # Returns
    ///
    /// If odd, returns `Choice(1)`. Otherwise, returns `Choice(0)`.
    fn is_odd(&self) -> Choice;

    /// Is this integer value an even number?
    ///
    /// # Returns
    ///
    /// If even, returns `Choice(1)`. Otherwise, returns `Choice(0)`.
    fn is_even(&self) -> Choice {
        !self.is_odd()
    }
}

/// Zero values.
pub trait Zero: ConstantTimeEq + Sized {
    /// The value `0`.
    const ZERO: Self;

    /// Determine if this value is equal to zero.
    ///
    /// # Returns
    ///
    /// If zero, returns `Choice(1)`. Otherwise, returns `Choice(0)`.
    fn is_zero(&self) -> Choice {
        self.ct_eq(&Self::ZERO)
    }
}

/// Random number generation support.
#[cfg(feature = "rand_core")]
pub trait Random: Sized {
    /// Generate a cryptographically secure random value.
    fn random(rng: &mut impl CryptoRngCore) -> Self;
}

/// Modular random number generation support.
#[cfg(feature = "rand_core")]
pub trait RandomMod: Sized + Zero {
    /// Generate a cryptographically secure random number which is less than
    /// a given `modulus`.
    ///
    /// This function uses rejection sampling, a method which produces an
    /// unbiased distribution of in-range values provided the underlying
    /// CSRNG is unbiased, but runs in variable-time.
    ///
    /// The variable-time nature of the algorithm should not pose a security
    /// issue so long as the underlying random number generator is truly a
    /// CSRNG, where previous outputs are unrelated to subsequent
    /// outputs and do not reveal information about the RNG's internal state.
    fn random_mod(rng: &mut impl CryptoRngCore, modulus: &NonZero<Self>) -> Self;
}

/// Compute `self + rhs mod p`.
pub trait AddMod<Rhs = Self> {
    /// Output type.
    type Output;

    /// Compute `self + rhs mod p`.
    ///
    /// Assumes `self` and `rhs` are `< p`.
    fn add_mod(&self, rhs: &Rhs, p: &Self) -> Self::Output;
}

/// Compute `self - rhs mod p`.
pub trait SubMod<Rhs = Self> {
    /// Output type.
    type Output;

    /// Compute `self - rhs mod p`.
    ///
    /// Assumes `self` and `rhs` are `< p`.
    fn sub_mod(&self, rhs: &Rhs, p: &Self) -> Self::Output;
}

/// Compute `-self mod p`.
pub trait NegMod {
    /// Output type.
    type Output;

    /// Compute `-self mod p`.
    #[must_use]
    fn neg_mod(&self, p: &Self) -> Self::Output;
}

/// Compute `self * rhs mod p`.
///
/// Requires `p_inv = -(p^{-1} mod 2^{BITS}) mod 2^{BITS}` to be provided for efficiency.
pub trait MulMod<Rhs = Self> {
    /// Output type.
    type Output;

    /// Compute `self * rhs mod p`.
    ///
    /// Requires `p_inv = -(p^{-1} mod 2^{BITS}) mod 2^{BITS}` to be provided for efficiency.
    fn mul_mod(&self, rhs: &Rhs, p: &Self, p_inv: Limb) -> Self::Output;
}

/// Checked addition.
pub trait CheckedAdd<Rhs = Self>: Sized {
    /// Output type.
    type Output;

    /// Perform checked subtraction, returning a [`CtOption`] which `is_some`
    /// only if the operation did not overflow.
    fn checked_add(&self, rhs: Rhs) -> CtOption<Self>;
}

/// Checked multiplication.
pub trait CheckedMul<Rhs = Self>: Sized {
    /// Output type.
    type Output;

    /// Perform checked multiplication, returning a [`CtOption`] which `is_some`
    /// only if the operation did not overflow.
    fn checked_mul(&self, rhs: Rhs) -> CtOption<Self>;
}

/// Checked subtraction.
pub trait CheckedSub<Rhs = Self>: Sized {
    /// Output type.
    type Output;

    /// Perform checked subtraction, returning a [`CtOption`] which `is_some`
    /// only if the operation did not underflow.
    fn checked_sub(&self, rhs: Rhs) -> CtOption<Self>;
}

/// Concatenate two numbers into a "wide" double-width value, using the `lo`
/// value as the least significant value.
pub trait Concat: ConcatMixed<Self, MixedOutput = Self::Output> {
    /// Concatenated output: twice the width of `Self`.
    type Output;

    /// Concatenate the two halves, with `self` as most significant and `lo`
    /// as the least significant.
    fn concat(&self, lo: &Self) -> Self::Output {
        self.concat_mixed(lo)
    }
}

/// Concatenate two numbers into a "wide" combined-width value, using the `lo`
/// value as the least significant value.
pub trait ConcatMixed<Lo: ?Sized = Self> {
    /// Concatenated output: combination of `Lo` and `Self`.
    type MixedOutput;

    /// Concatenate the two values, with `self` as most significant and `lo`
    /// as the least significant.
    fn concat_mixed(&self, lo: &Lo) -> Self::MixedOutput;
}

/// Split a number in half, returning the most significant half followed by
/// the least significant.
pub trait Split: SplitMixed<Self::Output, Self::Output> {
    /// Split output: high/low components of the value.
    type Output;

    /// Split this number in half, returning its high and low components
    /// respectively.
    fn split(&self) -> (Self::Output, Self::Output) {
        self.split_mixed()
    }
}

/// Split a number into parts, returning the most significant part followed by
/// the least significant.
pub trait SplitMixed<Hi, Lo> {
    /// Split this number into parts, returning its high and low components
    /// respectively.
    fn split_mixed(&self) -> (Hi, Lo);
}

/// Integers whose representation takes a bounded amount of space.
pub trait Bounded {
    /// Size of this integer in bits.
    const BITS: usize;

    /// Size of this integer in bytes.
    const BYTES: usize;
}

/// Encoding support.
pub trait Encoding: Sized {
    /// Byte array representation.
    type Repr: AsRef<[u8]> + AsMut<[u8]> + Copy + Clone + Sized;

    /// Decode from big endian bytes.
    fn from_be_bytes(bytes: Self::Repr) -> Self;

    /// Decode from little endian bytes.
    fn from_le_bytes(bytes: Self::Repr) -> Self;

    /// Encode to big endian bytes.
    fn to_be_bytes(&self) -> Self::Repr;

    /// Encode to little endian bytes.
    fn to_le_bytes(&self) -> Self::Repr;
}

/// Support for optimized squaring
pub trait Square: Sized
where
    for<'a> &'a Self: core::ops::Mul<&'a Self, Output = Self>,
{
    /// Computes the same as `self.mul(self)`, but may be more efficient.
    fn square(&self) -> Self {
        self * self
    }
}

/// Constant-time exponentiation.
pub trait Pow<Exponent> {
    /// Raises to the `exponent` power.
    fn pow(&self, exponent: &Exponent) -> Self;
}

impl<T: PowBoundedExp<Exponent>, Exponent: Bounded> Pow<Exponent> for T {
    fn pow(&self, exponent: &Exponent) -> Self {
        self.pow_bounded_exp(exponent, Exponent::BITS)
    }
}

/// Constant-time exponentiation with exponent of a bounded bit size.
pub trait PowBoundedExp<Exponent> {
    /// Raises to the `exponent` power,
    /// with `exponent_bits` representing the number of (least significant) bits
    /// to take into account for the exponent.
    ///
    /// NOTE: `exponent_bits` may be leaked in the time pattern.
    fn pow_bounded_exp(&self, exponent: &Exponent, exponent_bits: usize) -> Self;
}

/// Performs modular multi-exponentiation using Montgomery's ladder.
///
/// See: Straus, E. G. Problems and solutions: Addition chains of vectors. American Mathematical Monthly 71 (1964), 806–808.
pub trait MultiExponentiate<Exponent, BasesAndExponents>: Pow<Exponent> + Sized
where
    BasesAndExponents: AsRef<[(Self, Exponent)]> + ?Sized,
{
    /// Calculates `x1 ^ k1 * ... * xn ^ kn`.
    fn multi_exponentiate(bases_and_exponents: &BasesAndExponents) -> Self;
}

impl<T, Exponent, BasesAndExponents> MultiExponentiate<Exponent, BasesAndExponents> for T
where
    T: MultiExponentiateBoundedExp<Exponent, BasesAndExponents>,
    Exponent: Bounded,
    BasesAndExponents: AsRef<[(Self, Exponent)]> + ?Sized,
{
    fn multi_exponentiate(bases_and_exponents: &BasesAndExponents) -> Self {
        Self::multi_exponentiate_bounded_exp(bases_and_exponents, Exponent::BITS)
    }
}

/// Performs modular multi-exponentiation using Montgomery's ladder.
/// `exponent_bits` represents the number of bits to take into account for the exponent.
///
/// See: Straus, E. G. Problems and solutions: Addition chains of vectors. American Mathematical Monthly 71 (1964), 806–808.
///
/// NOTE: this value is leaked in the time pattern.
pub trait MultiExponentiateBoundedExp<Exponent, BasesAndExponents>: Pow<Exponent> + Sized
where
    BasesAndExponents: AsRef<[(Self, Exponent)]> + ?Sized,
{
    /// Calculates `x1 ^ k1 * ... * xn ^ kn`.
    fn multi_exponentiate_bounded_exp(
        bases_and_exponents: &BasesAndExponents,
        exponent_bits: usize,
    ) -> Self;
}

/// Constant-time inversion.
pub trait Invert: Sized {
    /// Output of the inversion.
    type Output;

    /// Computes the inverse.
    fn invert(&self) -> Self::Output;
}
