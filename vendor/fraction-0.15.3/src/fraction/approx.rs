//! Approximate mathematical operations.
//!
//! This module implements operations that do not guarantee lossless results, but which are
//! nonetheless useful. Using any functionality from within this module requires a compromise to be
//! made between performance and accuracy.
//!
//! Approximations are grouped into modules; for information on a particular approximation or group
//! of approximations, consult the relevant module's documentation.

use crate::{generic::GenericInteger, BigFraction, GenericFraction};
use num::{rational::Ratio, traits::Pow, BigUint, Integer};

pub mod sqrt;

/// Levels of accuracy for an approximation.
#[derive(Clone, Debug)]
pub enum Accuracy {
    /// At least 20 digits correct after the decimal point.
    #[cfg(feature = "lazy_static")]
    Dp20,

    /// At least 100 digits correct after the decimal point.
    #[cfg(feature = "lazy_static")]
    Dp100,

    /// At least 500 digits correct after the decimal point.
    #[cfg(feature = "lazy_static")]
    Dp500,

    /// An arbitrary number of correct digits.
    Custom {
        /// The multiplier used to check values for equality to the desired accuracy. **You
        /// probably want this to be `10^{n}`, where `n` is the number of decimal places of
        /// accuracy you need.**
        ///
        /// Normally this will have the form `10^n` where `n` is the number of correct decimal
        /// places, but this also holds for other bases. For instance, a value of `2^n` here has
        /// little meaning when the result is printed as decimal, but if the result was represented
        /// as a binary string in the form `a.b`, `b` would be correct to `n` digits (and `a` would
        /// be completely correct).
        multiplier: BigUint,
    },
}

impl Accuracy {
    /// Returns an [`Accuracy`] of `n` decimal places.
    #[must_use]
    pub fn decimal_places<N: GenericInteger>(n: N) -> Self
    where
        BigUint: Pow<N>,
        <BigUint as Pow<N>>::Output: Into<BigUint>,
    {
        #[cfg(feature = "lazy_static")]
        {
            // If we have access to pre-allocated `Accuracy` values, use them instead of allocating
            // a new multiplier.
            match n.to_u16() {
                Some(20) => return Self::Dp20,
                Some(100) => return Self::Dp100,
                Some(500) => return Self::Dp500,
                _ => (),
            }
        }

        Self::Custom {
            multiplier: Pow::pow(BigUint::from(10_u8), n).into(),
        }
    }

    /// Returns an [`Accuracy`] of `n` digits after the point (`.`) in the representation of the
    /// result in the given `base`.
    ///
    /// For example, `base_places(2, 5)` means "correct to at least 5 digits after the `.` when
    /// printed as binary".
    ///
    /// Prefer using [`Accuracy::decimal_places`] when `base == 10`.
    pub fn base_places<B: GenericInteger, N: GenericInteger>(base: B, n: N) -> Self
    where
        // Assuming `n` is anything other than really small, `base^n` will likely be pretty big, so
        // we calculate the multiplier using `BigUint`.
        B: Into<BigUint>,

        // We need to be able to raise `BigUint(base)` to the power of `n`...
        BigUint: Pow<N>,

        // ...and get back something that we can convert straight to `BigUint`.
        <BigUint as Pow<N>>::Output: Into<BigUint>,
    {
        Self::Custom {
            multiplier: Pow::pow(base.into(), n).into(),
        }
    }

    /// Returns a [`BigFraction`] which is equal to `fraction` according to this [`Accuracy`] by
    /// "chopping off" any irrelevant digits.
    ///
    /// The result will be equal to `(fraction * self.multiplier()).floor() / self.multiplier()`.
    ///
    /// This method propagates infinity and NaN values.
    pub fn chop<T>(&self, fraction: &GenericFraction<T>) -> BigFraction
    where
        T: Clone + Integer,
        BigUint: From<T>,
    {
        match fraction {
            GenericFraction::Rational(sign, ratio) => BigFraction::Rational(*sign, {
                self.chop_ratio(&Ratio::new_raw(
                    ratio.numer().clone().into(),
                    ratio.denom().clone().into(),
                ))
            }),

            GenericFraction::Infinity(sign) => BigFraction::Infinity(*sign),
            GenericFraction::NaN => BigFraction::NaN,
        }
    }

    /// Returns a chopped and simplified version of `ratio`.
    #[must_use]
    fn chop_ratio(&self, ratio: &Ratio<BigUint>) -> Ratio<BigUint> {
        Ratio::new(
            self.chopped_numerator_raw(ratio.numer(), ratio.denom()),
            self.multiplier().clone(),
        )
    }

    /// Returns the numerator of the chopped but unsimplified version of `numer / denom`, where the
    /// implied denominator is `self.multiplier()`.
    fn chopped_numerator_raw(&self, numer: &BigUint, denom: &BigUint) -> BigUint {
        (numer * self.multiplier()) / denom
    }

    /// Returns a reference to the multiplier used by `self` to chop off irrelevant digits.
    #[must_use]
    pub fn multiplier(&self) -> &BigUint {
        #[cfg(feature = "lazy_static")]
        {
            lazy_static! {
                static ref DP20_MUL: BigUint = BigUint::from(10_u8).pow(20_u32);
                static ref DP100_MUL: BigUint = BigUint::from(10_u8).pow(100_u32);
                static ref DP500_MUL: BigUint = BigUint::from(10_u8).pow(500_u32);
            };

            return match self {
                Accuracy::Dp20 => &DP20_MUL,
                Accuracy::Dp100 => &DP100_MUL,
                Accuracy::Dp500 => &DP500_MUL,
                Accuracy::Custom { multiplier } => multiplier,
            };
        }

        // When `lazy_static` is enabled, this gets flagged as unreachable which it technically is,
        // but *only* when `lazy_static` is on.
        #[allow(unreachable_code)]
        {
            let Accuracy::Custom { multiplier } = self else {
                // `Custom` is the only available variant when `lazy_static` is off.
                unreachable!()
            };

            multiplier
        }
    }
}

impl Default for Accuracy {
    fn default() -> Self {
        Self::Dp100
    }
}
