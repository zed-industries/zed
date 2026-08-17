//! Implements an arbitrary-precision square-root approximation algorithm.
//!
//! This module extends [`GenericFraction`] with eight (8) methods:
//!  - [`sqrt`](GenericFraction::sqrt)
//!  - [`sqrt_raw`](GenericFraction::sqrt_raw)
//!  - [`sqrt_with_accuracy`](GenericFraction::sqrt_with_accuracy)
//!  - [`sqrt_with_accuracy_raw`](GenericFraction::sqrt_with_accuracy_raw)
//!  - [`sqrt_abs`](GenericFraction::sqrt_abs)
//!  - [`sqrt_abs_raw`](GenericFraction::sqrt_abs_raw)
//!  - [`sqrt_abs_with_accuracy`](GenericFraction::sqrt_abs_with_accuracy)
//!  - [`sqrt_abs_with_accuracy_raw`](GenericFraction::sqrt_abs_with_accuracy_raw)
//!
//! All of these methods return some form of an approximation for the square root, and they all use
//! the same algorithm but behave slightly differently.
//!
//! **A note on performance:** These methods are designed to be fast in release builds. An
//! unfortunate side effect of this is that performance is significantly degraded when compiler
//! optimisations are disabled. Please consider enabling compiler optimisations in your project if
//! you wish to use these methods.
//!
//! # Accuracy of the output
//!
//! *The behaviour detailed in this section is common to all methods.*
//!
//! The algorithm in use is iterative: an initial estimate is generated, and this is repeatedly
//! refined until it is suitably accurate. Of course, it is not possible to check the approximation
//! against the true square root value, because we don't know it. (If we did, we'd just return that
//! instead.) Because of this, the "closeness" must be determined by squaring the approximation to
//! see how close *it* comes to the original value.
//!
//! For example, let's say we wanted to find the square root of `0.0152399025`
//! (`6095961/400000000`). We might guess `0.1`. To check how close we are, we square our guess of
//! `0.1` and get `0.01`. This square is accurate to 2 decimal places (0, 1). We might guess again,
//! this time with `0.12`. This gives us `0.0144`, so we're still only accurate to 2 decimal
//! places. Guessing `0.123` yields a square of `0.015129`, which is now accurate to 3 decimal
//! places.
//!
//! The above is the checking process that happens when you call any of the methods defined in this
//! module. You must supply a value denoting how accurate you want the square of the approximation
//! to be. **It is very important to realise that this is *not* the same as the accuracy of the
//! square root!** However, for `x > 1`, `x.sqrt(n)` where `n` is the number of decimal places that
//! the square must be accurate to will actually produce a square root which is accurate to *more*
//! than `n` decimal places. In other words: as long as you aren't using small numbers, you can
//! consider `n` to be a lower bound on the accuracy of the square root itself. When you are
//! working with numbers less than 1, this is no longer the case, and the square root may be less
//! accurate than the square (although this is still not typically the case). **In all cases, the
//! square of the result you get will be accurate to the level of accuracy you gave to the
//! method.**
//!
//! Due to the nature of the algorithm, most outputs will actually be vastly more accurate than
//! requested. This should not be relied on, and is subject to change. In some instances, the
//! result will actually be exactly correct (e.g. for small square numbers like 4, 9, 16, 25,
//! etc.).
//!
//! # Method variants
//! ## `raw`
//! `raw` methods return the approximation in unsimplified form via [`RawApprox`], which can (in
//! most cases; see its documentation for more details) be destructured into a
//! [`Ratio`](num::rational::Ratio). These methods are typically faster than non-`raw` ones by an
//! order of magnitude or more, especially for extreme inputs (very large or very small). Do not
//! use `raw` methods unless you are writing *very* performance-critical code: most arithmetic
//! operations on [`Ratio`](num::rational::Ratio) and [`GenericFraction`] will automatically
//! simplify the result so although the `sqrt_raw` method will still be faster than the non-`raw`
//! version, you will still incur the overhead of simplification, just somewhere else instead.
//!
//! ```no_run
//! # extern crate num;
//! # use crate::fraction::approx::sqrt::RawApprox;
//! # use num::rational::Ratio;
//! # use num::BigUint;
//! # use num::Zero;
//! # let x: crate::fraction::GenericFraction<u8> = 2.into();
//! // We save time here by using `raw`...
//! match x.sqrt_raw(10_000) {
//!     RawApprox::Rational(ratio) => {
//!         // ...but we lose the time again here, because the `+` simplifies the ratio.
//!         let _ = ratio + Ratio::<BigUint>::zero();
//!     },
//!
//!     _ => (),
//! }
//! ```
//!
//! Be aware that unsimplified values can take up excessive amounts of memory (100 KB and beyond,
//! particularly when high levels of accuracy are used).
//!
//! For a few (limited) examples of how you can write performance-critical code with
//! [`Ratio`](num::rational::Ratio) and [`GenericFraction`], see this module's own implementation.
//!
//! ## `with_accuracy`
//! These methods accept an [`Accuracy`] value instead of a `u32` representing a number of decimal
//! places. It is recommended that you use `with_accuracy` whenever you need to approximate more
//! than one square root to the same level of accuracy. This avoids unnecessary heap allocations.
//!
//! ```no_run
//! # use crate::fraction::GenericFraction;
//! # use crate::fraction::approx::Accuracy;
//! # fn some_fn(_: GenericFraction<num::BigUint>) {}
//! // Don't do this! `sqrt` allocates a new `Accuracy` each time.
//! for n in 0..100 {
//!     let x: GenericFraction<u8> = n.into();
//!     some_fn(x.sqrt(1_000));
//! }
//!
//! // Instead, do this:
//! let accuracy = Accuracy::decimal_places(1_000_u32);
//!
//! for n in 0..100 {
//!     let x: GenericFraction<u8> = n.into();
//!     some_fn(x.sqrt_with_accuracy(&accuracy));
//! }
//! ```
//!
//! If you only perform one `sqrt` computation, there is no benefit to using `sqrt_with_accuracy`.
//!
//! Keep in mind that [`Accuracy`] has some pre-allocated values which you may wish to use.
//!
//! ## `abs`
//! These methods ignore the sign of the value and instead approximate the square root of the
//! *magnitude* only.
//!
//! The default behaviour is as follows:
//!
//! ```
//! # use crate::fraction::GenericFraction;
//! let something_positive: GenericFraction<u8> = 2.into();
//! let something_negative: GenericFraction<u8> = (-2).into();
//!
//! // Calling `sqrt` on a negative number will return `NaN`.
//! assert_eq!(something_negative.sqrt(100), GenericFraction::NaN);
//!
//! // Calling `sqrt_abs` on a negative number will not return `NaN`.
//! assert_ne!(something_negative.sqrt_abs(100), GenericFraction::NaN);
//!
//! // `sqrt +2` is equal to `sqrt_abs -2`.
//! assert_eq!(something_positive.sqrt(100), something_negative.sqrt_abs(100));
//! ```
//!
//! The following are guaranteed to hold for any pair of `abs` and non-`abs` methods:
//!
//! ```
//! # use crate::fraction::GenericFraction;
//! # let x: GenericFraction<u8> = 2.into();
//! // When `x` is positive:
//! assert_eq!(x.sqrt(100), (-x).sqrt_abs(100));
//!
//! # let x: GenericFraction<u8> = (-2).into();
//! // When `x` is negative:
//! assert_eq!((-x).sqrt(100), x.sqrt_abs(100));
//! ```

use super::Accuracy;
use crate::{generic::GenericInteger, GenericFraction, Sign};
use num::{
    bigint::{ToBigInt, ToBigUint},
    rational::Ratio,
    BigUint, Integer, ToPrimitive, Zero,
};
use std::borrow::Borrow;

/// An unsimplified approximation of a square root.
pub enum RawApprox {
    /// A rational (i.e. fractional) approximation.
    ///
    /// Depending on the accuracy used, these numbers can be *very* large to store (>100 KB with
    /// excessive accuracy), so cloning is likely to be expensive.
    Rational(Ratio<BigUint>),

    /// Positive infinity. This is returned when the square root of positive infinity is requested.
    PlusInf,

    /// Zero. This only occurs when the input is zero.
    Zero,

    /// An invalid number.
    ///
    /// `abs` methods will only return this for `NaN` input, but other methods will return `NaN` if
    /// the value to calculate the square root of is negative.
    NaN,
}

impl RawApprox {
    /// Returns `self` in the simplest form. This only modifies `Rational` values.
    #[must_use]
    pub fn simplified(self) -> Self {
        match self {
            RawApprox::Rational(ratio) => {
                // We could call `ratio.reduced()`, but that would clone the numerator and
                // denominator. If we take ownership of them and recreate the ratio using `new`,
                // we can reduce it without the clone.

                let (n, d) = ratio.into();
                RawApprox::Rational(Ratio::new(n, d))
            }

            other => other,
        }
    }
}

impl From<RawApprox> for GenericFraction<BigUint> {
    fn from(v: RawApprox) -> Self {
        match v {
            RawApprox::Rational(ratio) => GenericFraction::Rational(Sign::Plus, ratio),
            RawApprox::PlusInf => GenericFraction::infinity(),
            RawApprox::Zero => GenericFraction::zero(),
            RawApprox::NaN => GenericFraction::nan(),
        }
    }
}

/// Different setup outputs.
enum SqrtSetup {
    /// Setup result which can be used as an exact answer without further processing.
    ShortCircuited(RawApprox),

    /// Setup result providing an initial estimate, not an exact value.
    Estimated {
        /// The initial estimate of the square root.
        estimate: Ratio<BigUint>,

        /// The input value represented as a `Ratio<BigUint>`. This is produced as a byproduct of
        /// the setup but is useful in the rest of the algorithm too, so we return it.
        square_ratio: Ratio<BigUint>,
    },
}

impl SqrtSetup {
    /// Produces setup values for finding the square root of `value.abs()`.
    fn for_value_abs<Nd>(value: &GenericFraction<Nd>) -> SqrtSetup
    where
        Nd: Clone + GenericInteger + ToBigInt + ToBigUint,
    {
        match value {
            GenericFraction::Rational(_, ratio) if ratio.is_zero() => {
                SqrtSetup::ShortCircuited(RawApprox::Zero)
            }

            GenericFraction::Rational(_, ratio) => {
                // If we can convert the components of `ratio` into `f64`s, we can approximate the
                // square root using `f64::sqrt`. This gives an excellent starting point.
                let float_estimate = ratio
                    .to_f64()
                    .map(f64::sqrt)
                    // `from_float` will give `None` if the result of `sqrt` is not finite (incl.
                    // NaN), so we'll automatically fall back to the alternative method if `sqrt`
                    // fails here.
                    .and_then(|float| {
                        // Note: We use `abs` here even though `float` will never reasonably be
                        // negative. This is because `Nd` could *technically* be a signed type,
                        // in which case `float` is not guaranteed to be positive.
                        let (n, d) = Ratio::<num::BigInt>::from_float(float.abs())?.into();

                        // Using `into_parts` allows us to avoid having to clone the underlying
                        // `BigUint` data within the two values.
                        Some(Ratio::new_raw(n.into_parts().1, d.into_parts().1))
                    });

                // Safety: `to_bigint` is guaranteed not to fail for any integer type, and we know
                // that `Nd` is an integer type.
                let ratio = Ratio::new_raw(
                    // `.into_parts().1` just takes the `BigUint` component of the `BigInt`, so is
                    // equivalent to `.abs()` without the clone that happens inside `abs`.
                    ratio.numer().to_bigint().unwrap().into_parts().1,
                    ratio.denom().to_bigint().unwrap().into_parts().1,
                );

                SqrtSetup::Estimated {
                    estimate: float_estimate.unwrap_or_else(|| {
                        // If we couldn't use floats, we fall back to a crude estimate using
                        // truncated integer square roots. This still isn't too bad.
                        Ratio::new(ratio.numer().sqrt(), ratio.denom().sqrt())
                    }),

                    square_ratio: ratio,
                }
            }

            // The absolute value of `-inf` is just `+inf`, so we can handle both infinities the
            // same.
            GenericFraction::Infinity(_) => SqrtSetup::ShortCircuited(RawApprox::PlusInf),

            GenericFraction::NaN => SqrtSetup::ShortCircuited(RawApprox::NaN),
        }
    }
}

/// Halves `value` in-place while keeping it in simplest form. This is faster than standard
/// division.
fn halve_in_place_pos_rational(ratio: &mut Ratio<BigUint>) {
    // To mutate the numerator and denominator of the ratio we'll take ownership of both and then
    // put them back when we're done.
    let (mut numer, mut denom) = std::mem::take(ratio).into();

    if numer.is_even() {
        numer /= 2_u32;
    } else {
        denom *= 2_u32;
    }

    *ratio = Ratio::new_raw(numer, denom);
}

/// Adds two `Ratio`s together without reducing the result to simplest form. This is significantly
/// faster than using the standard addition operator provided by `num`, and may be used as long as
/// the result does not need to be in simplest form (e.g. within an algorithm which reduces the
/// output ratio before returning).
fn add_ratios_raw(lhs: Ratio<BigUint>, rhs: Ratio<BigUint>) -> Ratio<BigUint> {
    // Don't bother comparing the denominators because it's highly unlikely that they're equal.
    // Instead, we just go straight to giving the fractions equal denominators.
    let (mut lhs_numer, lhs_denom) = lhs.into();
    let (mut rhs_numer, rhs_denom) = rhs.into();

    let common_denom = lhs_denom.lcm(&rhs_denom);

    let lhs_multiplier = &common_denom / &lhs_denom;
    let rhs_multiplier = &common_denom / &rhs_denom;

    lhs_numer *= lhs_multiplier;
    rhs_numer *= rhs_multiplier;

    Ratio::new_raw(lhs_numer + rhs_numer, common_denom)
}

/// Various square root operations for `GenericFraction`.
impl<T: Clone + Integer + ToBigUint + ToBigInt + GenericInteger> GenericFraction<T> {
    /// Returns an unsimplified rational approximation of the principal square root of the absolute
    /// value of `self`. `accuracy` is the accuracy of the square of the return value relative to
    /// `self`.
    ///
    /// See the [module-level documentation](`self`) for more details.
    pub fn sqrt_abs_with_accuracy_raw(&self, accuracy: impl Borrow<Accuracy>) -> RawApprox {
        let accuracy = accuracy.borrow();

        let (estimate, value_as_ratio) = match SqrtSetup::for_value_abs(self) {
            SqrtSetup::Estimated {
                estimate,
                square_ratio,
            } => (estimate, square_ratio),

            SqrtSetup::ShortCircuited(exact) => return exact,
        };

        // Take ownership of the two parts of the target ratio. This allows us to treat them
        // separately. For example, we must clone the numerator for the next step, but only a
        // reference to the denominator is needed. Therefore, we can avoid needlessly cloning both
        // halves.
        let (target_numer, target_denom) = value_as_ratio.into();

        // Truncate the target square so we can check against it to determine when to finish. The
        // implied denominator for the numerator returned by the chop operation (choperation?) is
        // `accuracy.multiplier()`, so we don't need to store it.
        let truncated_target_numerator =
            accuracy.chopped_numerator_raw(&target_numer, &target_denom);

        let mut current_approx = estimate;

        loop {
            // We're using a highly optimised version of Newton's method here, broken into three
            // steps. Mathematically we would write
            //      r1 = 0.5(r0 + A/r0)
            // where r0 is the current guess, r1 is the next guess, and A is the value we're
            // finding the square root of.

            // One of the key optimisations is to avoid using the `Ratio` type's `std::ops`
            // implementations, as they ensure that the resulting `Ratio` is always in simplest
            // form. That's normally a useful property, but here we need to be able to perform many
            // iterations very quickly, and the process of reducing a fraction to simplest form is
            // really quite slow. This is especially true when dealing with the size of numbers
            // that we'll be dealing with after only a couple of iterations - as the approximation
            // gets more accurate, the numerator and denominator become larger. To get around this,
            // we use a lot of `new_raw` and explicit manipulation of the numerators and
            // denominators of fractions.

            // A/r0
            let divided = Ratio::new_raw(
                &target_numer * current_approx.denom(),
                &target_denom * current_approx.numer(),
            );

            // r0 + A/r0
            current_approx = add_ratios_raw(current_approx, divided);

            // 0.5(r0 + A/r0)
            halve_in_place_pos_rational(&mut current_approx);

            // For checking the approximation, we square it to see how close the result is to the
            // original input value. Again, the implied denominator is `accuracy.multiplier()`.
            // This is the same as for `truncated_target_numerator`, so we just need to compare the
            // numerators.
            let squared_and_truncated_numerator = accuracy.chopped_numerator_raw(
                &(current_approx.numer() * current_approx.numer()),
                &(current_approx.denom() * current_approx.denom()),
            );

            // Stop and yield the current guess if it's close enough to the true value.
            if squared_and_truncated_numerator == truncated_target_numerator {
                // This is `_raw`, so we don't reduce.
                break RawApprox::Rational(current_approx);
            }
        }
    }

    /// Returns a rational approximation of the principal square root of the absolute value of
    /// `self`. `accuracy` is the accuracy of the square of the return value relative to `self`.
    ///
    /// See the [module-level documentation](`self`) for more details.
    pub fn sqrt_abs_with_accuracy(
        &self,
        accuracy: impl Borrow<Accuracy>,
    ) -> GenericFraction<BigUint> {
        self.sqrt_abs_with_accuracy_raw(accuracy)
            .simplified()
            .into()
    }

    /// Returns an unsimplified rational approximation of the principal square root of the absolute
    /// value of `self`. `decimal_places` refers to the accuracy of the square of the return value
    /// relative to `self`.
    ///
    /// See the [module-level documentation](`self`) for more details.
    pub fn sqrt_abs_raw(&self, decimal_places: u32) -> RawApprox {
        self.sqrt_abs_with_accuracy_raw(Accuracy::decimal_places(decimal_places))
    }

    /// Returns a rational approximation of the principal square root of the absolute value of
    /// `self`. `decimal_places` refers to the accuracy of the square of the return value relative
    /// to `self`.
    ///
    /// See the [module-level documentation](`self`) for more details.
    pub fn sqrt_abs(&self, decimal_places: u32) -> GenericFraction<BigUint> {
        self.sqrt_abs_raw(decimal_places).simplified().into()
    }

    /// Returns an unsimplified rational approximation of the principal square root of `self`.
    /// `accuracy` refers to the square of the return value relative to `self`.
    ///
    /// This method returns `NaN` if `self` is negative.
    ///
    /// See the [module-level documentation](`self`) for more details.
    pub fn sqrt_with_accuracy_raw(&self, accuracy: impl Borrow<Accuracy>) -> RawApprox {
        match self {
            GenericFraction::Infinity(Sign::Minus) => RawApprox::NaN,

            // Short-circuit for zero so we don't have to worry about it when checking the signs.
            GenericFraction::Rational(_, r) if r.is_zero() => RawApprox::Zero,

            GenericFraction::Rational(sign, ratio)
                if {
                    // `num` doesn't seem to give us a way to check the sign of a `T` given the
                    // current trait bounds we have on it, but we can work around this by checking
                    // against zero.

                    let numer_sign = if ratio.numer() > &T::zero() {
                        Sign::Plus
                    } else {
                        Sign::Minus
                    };

                    // Combine the whole fraction's sign with the sign of the numerator. This
                    // allows us to recognise stuff like -(-1/2) as being positive, for example.
                    let numer_sign = *sign * numer_sign;

                    let denom_sign = if ratio.denom() > &T::zero() {
                        Sign::Plus
                    } else {
                        Sign::Minus
                    };

                    numer_sign != denom_sign
                } =>
            {
                RawApprox::NaN
            }

            _positive => self.sqrt_abs_with_accuracy_raw(accuracy),
        }
    }

    /// Returns a rational approximation of the principal square root of `self`. `accuracy` refers
    /// to the square of the return value relative to `self`.
    ///
    /// This method returns `NaN` if `self` is negative.
    ///
    /// See the [module-level documentation](`self`) for more details.
    pub fn sqrt_with_accuracy(&self, accuracy: impl Borrow<Accuracy>) -> GenericFraction<BigUint> {
        self.sqrt_with_accuracy_raw(accuracy).simplified().into()
    }

    /// Returns an unsimplified rational approximation of the principal square root of `self`.
    /// `decimal_places` refers to the accuracy of the square of the return value relative to
    /// `self`.
    ///
    /// This method returns `NaN` if `self` is negative.
    ///
    /// See the [module-level documentation](`self`) for more details.
    pub fn sqrt_raw(&self, decimal_places: u32) -> RawApprox {
        self.sqrt_with_accuracy_raw(Accuracy::decimal_places(decimal_places))
    }

    /// Returns a rational approximation of the principal square root of `self`. `decimal_places`
    /// refers to the accuracy of the square of the return value relative to `self`.
    ///
    /// This method returns `NaN` if `self` is negative.
    ///
    /// See the [module-level documentation](`self`) for more details.
    pub fn sqrt(&self, decimal_places: u32) -> GenericFraction<BigUint> {
        self.sqrt_raw(decimal_places).simplified().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{approx::Accuracy, BigFraction, GenericFraction};
    use num::{traits::Pow, BigUint, Zero};
    use std::str::FromStr;

    /// Converts `f` to a string. This is useful because `f.to_string()` will round sometimes.
    /// Ideal tests would use this, but it takes far too long to convert `BigUint`s to strings for
    /// that to be realistic.
    #[allow(dead_code)]
    fn fraction_to_decimal_string(f: &BigFraction, decimal_places: usize) -> String {
        let GenericFraction::Rational(sign, ratio) = f else {
            return f.to_string();
        };

        let sign_str = match sign {
            crate::Sign::Plus => "",
            crate::Sign::Minus => "-",
        };

        let integer = ratio.to_integer();

        let multiplier: BigUint = Pow::pow(&BigUint::from(10_u8), decimal_places);
        let fractional = (ratio.fract() * multiplier).to_integer();

        // Since `fractional` is an integer, converting it to a string removes any leading zeros.
        // We're using it to represent the fractional part of the number, though, so we need to add
        // back those leading zeros.
        format!("{sign_str}{integer}.{fractional:0>decimal_places$}")
    }

    fn test_sqrt_of(value: impl std::borrow::Borrow<BigFraction>, simplify: bool, n: usize) {
        let value = value.borrow();

        let accuracy = Accuracy::decimal_places(n);

        println!("calculating...");
        let sqrt = value.sqrt_with_accuracy_raw(&accuracy);

        let sqrt: BigFraction = if simplify {
            {
                println!("simplifying...");
                sqrt.simplified()
            }
        } else {
            sqrt
        }
        .into();

        println!("checking...");
        assert_eq!(
            accuracy.chopped_numerator_raw(
                &(sqrt.numer().unwrap() * sqrt.numer().unwrap()),
                &(sqrt.denom().unwrap() * sqrt.denom().unwrap()),
            ),
            accuracy.chopped_numerator_raw(value.numer().unwrap(), value.denom().unwrap())
        );
    }

    lazy_static! {
        static ref VALUES: [BigFraction; 5] = [
            BigFraction::from(2),
            BigFraction::from(123_654),
            BigFraction::from(123_655),
            BigFraction::from_str(
                "5735874745115151552958367280658028638020529468164964850251033802750\
            727314244020586751748892724760644\
            /\
            478953213143537128483961697945367179924659061093095449962100933428918126\
            6216833845985099376094324166"
            )
            .unwrap(),
            BigFraction::from(0.2),
        ];
    }

    const ACCURACIES: [usize; 5] = [10, 100, 1000, 10000, 100_000];

    #[test]
    fn test_raw() {
        for value in &*VALUES {
            for accuracy in ACCURACIES {
                println!("sqrt({value}) to {accuracy} d.p., unsimplified");
                test_sqrt_of(value, false, accuracy);
            }
        }
    }

    #[test]
    fn test_negative() {
        let x: GenericFraction<u8> = (-2).into();
        assert_eq!(x.sqrt(10), GenericFraction::NaN);
    }

    #[test]
    fn test_abs() {
        let x: GenericFraction<u8> = 2.into();
        assert_eq!(x.sqrt(10), (-x).sqrt_abs(10));
    }

    #[test]
    fn test_weird_numbers() {
        let x: GenericFraction<u8> = f32::NAN.into();
        assert_eq!(GenericFraction::NaN, x.sqrt(10));

        let x: GenericFraction<u8> = f32::INFINITY.into();
        assert_eq!(GenericFraction::Infinity(crate::Sign::Plus), x.sqrt(10));

        let x: GenericFraction<u8> = f32::NEG_INFINITY.into();
        assert_eq!(GenericFraction::Infinity(crate::Sign::Plus), x.sqrt_abs(10));

        let x: GenericFraction<u8> = 0.into();
        assert!(x.sqrt(10).is_zero());
    }

    #[test]
    fn test_negative_inf() {
        let x: GenericFraction<u8> = f32::NEG_INFINITY.into();
        assert_eq!(x.sqrt(10), GenericFraction::NaN);
    }
}
