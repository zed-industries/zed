// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Basic floating-point number distributions

use crate::distributions::utils::FloatSIMDUtils;
use crate::distributions::{Distribution, Standard};
use crate::Rng;
use core::mem;

#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};

/// A distribution to sample floating point numbers uniformly in the half-open
/// interval `(0, 1]`, i.e. including 1 but not 0.
///
/// All values that can be generated are of the form `n * ε/2`. For `f32`
/// the 24 most significant random bits of a `u32` are used and for `f64` the
/// 53 most significant bits of a `u64` are used. The conversion uses the
/// multiplicative method.
///
/// See also: [`Standard`] which samples from `[0, 1)`, [`Open01`]
/// which samples from `(0, 1)` and [`Uniform`] which samples from arbitrary
/// ranges.
///
/// # Example
/// ```
/// use rand::{thread_rng, Rng};
/// use rand::distributions::OpenClosed01;
///
/// let val: f32 = thread_rng().sample(OpenClosed01);
/// println!("f32 from (0, 1): {}", val);
/// ```
///
/// [`Standard`]: crate::distributions::Standard
/// [`Open01`]: crate::distributions::Open01
/// [`Uniform`]: crate::distributions::uniform::Uniform
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct OpenClosed01;

/// A distribution to sample floating point numbers uniformly in the open
/// interval `(0, 1)`, i.e. not including either endpoint.
///
/// All values that can be generated are of the form `n * ε + ε/2`. For `f32`
/// the 23 most significant random bits of an `u32` are used, for `f64` 52 from
/// an `u64`. The conversion uses a transmute-based method.
///
/// See also: [`Standard`] which samples from `[0, 1)`, [`OpenClosed01`]
/// which samples from `(0, 1]` and [`Uniform`] which samples from arbitrary
/// ranges.
///
/// # Example
/// ```
/// use rand::{thread_rng, Rng};
/// use rand::distributions::Open01;
///
/// let val: f32 = thread_rng().sample(Open01);
/// println!("f32 from (0, 1): {}", val);
/// ```
///
/// [`Standard`]: crate::distributions::Standard
/// [`OpenClosed01`]: crate::distributions::OpenClosed01
/// [`Uniform`]: crate::distributions::uniform::Uniform
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Open01;


// This trait is needed by both this lib and rand_distr hence is a hidden export
#[doc(hidden)]
pub trait IntoFloat {
    type F;

    /// Helper method to combine the fraction and a constant exponent into a
    /// float.
    ///
    /// Only the least significant bits of `self` may be set, 23 for `f32` and
    /// 52 for `f64`.
    /// The resulting value will fall in a range that depends on the exponent.
    /// As an example the range with exponent 0 will be
    /// [2<sup>0</sup>..2<sup>1</sup>), which is [1..2).
    fn into_float_with_exponent(self, exponent: i32) -> Self::F;
}

macro_rules! float_impls {
    ($ty:ident, $uty:ident, $f_scalar:ident, $u_scalar:ty,
     $fraction_bits:expr, $exponent_bias:expr) => {
        impl IntoFloat for $uty {
            type F = $ty;
            #[inline(always)]
            fn into_float_with_exponent(self, exponent: i32) -> $ty {
                // The exponent is encoded using an offset-binary representation
                let exponent_bits: $u_scalar =
                    (($exponent_bias + exponent) as $u_scalar) << $fraction_bits;
                $ty::from_bits(self | exponent_bits)
            }
        }

        impl Distribution<$ty> for Standard {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                // Multiply-based method; 24/53 random bits; [0, 1) interval.
                // We use the most significant bits because for simple RNGs
                // those are usually more random.
                let float_size = mem::size_of::<$f_scalar>() as u32 * 8;
                let precision = $fraction_bits + 1;
                let scale = 1.0 / ((1 as $u_scalar << precision) as $f_scalar);

                let value: $uty = rng.gen();
                let value = value >> (float_size - precision);
                scale * $ty::cast_from_int(value)
            }
        }

        impl Distribution<$ty> for OpenClosed01 {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                // Multiply-based method; 24/53 random bits; (0, 1] interval.
                // We use the most significant bits because for simple RNGs
                // those are usually more random.
                let float_size = mem::size_of::<$f_scalar>() as u32 * 8;
                let precision = $fraction_bits + 1;
                let scale = 1.0 / ((1 as $u_scalar << precision) as $f_scalar);

                let value: $uty = rng.gen();
                let value = value >> (float_size - precision);
                // Add 1 to shift up; will not overflow because of right-shift:
                scale * $ty::cast_from_int(value + 1)
            }
        }

        impl Distribution<$ty> for Open01 {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                // Transmute-based method; 23/52 random bits; (0, 1) interval.
                // We use the most significant bits because for simple RNGs
                // those are usually more random.
                use core::$f_scalar::EPSILON;
                let float_size = mem::size_of::<$f_scalar>() as u32 * 8;

                let value: $uty = rng.gen();
                let fraction = value >> (float_size - $fraction_bits);
                fraction.into_float_with_exponent(0) - (1.0 - EPSILON / 2.0)
            }
        }
    }
}

float_impls! { f32, u32, f32, u32, 23, 127 }
float_impls! { f64, u64, f64, u64, 52, 1023 }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::rngs::mock::StepRng;

    const EPSILON32: f32 = ::core::f32::EPSILON;
    const EPSILON64: f64 = ::core::f64::EPSILON;

    macro_rules! test_f32 {
        ($fnn:ident, $ty:ident, $ZERO:expr, $EPSILON:expr) => {
            #[test]
            fn $fnn() {
                // Standard
                let mut zeros = StepRng::new(0, 0);
                assert_eq!(zeros.gen::<$ty>(), $ZERO);
                let mut one = StepRng::new(1 << 8 | 1 << (8 + 32), 0);
                assert_eq!(one.gen::<$ty>(), $EPSILON / 2.0);
                let mut max = StepRng::new(!0, 0);
                assert_eq!(max.gen::<$ty>(), 1.0 - $EPSILON / 2.0);

                // OpenClosed01
                let mut zeros = StepRng::new(0, 0);
                assert_eq!(zeros.sample::<$ty, _>(OpenClosed01), 0.0 + $EPSILON / 2.0);
                let mut one = StepRng::new(1 << 8 | 1 << (8 + 32), 0);
                assert_eq!(one.sample::<$ty, _>(OpenClosed01), $EPSILON);
                let mut max = StepRng::new(!0, 0);
                assert_eq!(max.sample::<$ty, _>(OpenClosed01), $ZERO + 1.0);

                // Open01
                let mut zeros = StepRng::new(0, 0);
                assert_eq!(zeros.sample::<$ty, _>(Open01), 0.0 + $EPSILON / 2.0);
                let mut one = StepRng::new(1 << 9 | 1 << (9 + 32), 0);
                assert_eq!(one.sample::<$ty, _>(Open01), $EPSILON / 2.0 * 3.0);
                let mut max = StepRng::new(!0, 0);
                assert_eq!(max.sample::<$ty, _>(Open01), 1.0 - $EPSILON / 2.0);
            }
        };
    }
    test_f32! { f32_edge_cases, f32, 0.0, EPSILON32 }

    macro_rules! test_f64 {
        ($fnn:ident, $ty:ident, $ZERO:expr, $EPSILON:expr) => {
            #[test]
            fn $fnn() {
                // Standard
                let mut zeros = StepRng::new(0, 0);
                assert_eq!(zeros.gen::<$ty>(), $ZERO);
                let mut one = StepRng::new(1 << 11, 0);
                assert_eq!(one.gen::<$ty>(), $EPSILON / 2.0);
                let mut max = StepRng::new(!0, 0);
                assert_eq!(max.gen::<$ty>(), 1.0 - $EPSILON / 2.0);

                // OpenClosed01
                let mut zeros = StepRng::new(0, 0);
                assert_eq!(zeros.sample::<$ty, _>(OpenClosed01), 0.0 + $EPSILON / 2.0);
                let mut one = StepRng::new(1 << 11, 0);
                assert_eq!(one.sample::<$ty, _>(OpenClosed01), $EPSILON);
                let mut max = StepRng::new(!0, 0);
                assert_eq!(max.sample::<$ty, _>(OpenClosed01), $ZERO + 1.0);

                // Open01
                let mut zeros = StepRng::new(0, 0);
                assert_eq!(zeros.sample::<$ty, _>(Open01), 0.0 + $EPSILON / 2.0);
                let mut one = StepRng::new(1 << 12, 0);
                assert_eq!(one.sample::<$ty, _>(Open01), $EPSILON / 2.0 * 3.0);
                let mut max = StepRng::new(!0, 0);
                assert_eq!(max.sample::<$ty, _>(Open01), 1.0 - $EPSILON / 2.0);
            }
        };
    }
    test_f64! { f64_edge_cases, f64, 0.0, EPSILON64 }

    #[test]
    fn value_stability() {
        fn test_samples<T: Copy + core::fmt::Debug + PartialEq, D: Distribution<T>>(
            distr: &D, zero: T, expected: &[T],
        ) {
            let mut rng = crate::test::rng(0x6f44f5646c2a7334);
            let mut buf = [zero; 3];
            for x in &mut buf {
                *x = rng.sample(&distr);
            }
            assert_eq!(&buf, expected);
        }

        test_samples(&Standard, 0f32, &[0.0035963655, 0.7346052, 0.09778172]);
        test_samples(&Standard, 0f64, &[
            0.7346051961657583,
            0.20298547462974248,
            0.8166436635290655,
        ]);

        test_samples(&OpenClosed01, 0f32, &[0.003596425, 0.73460525, 0.09778178]);
        test_samples(&OpenClosed01, 0f64, &[
            0.7346051961657584,
            0.2029854746297426,
            0.8166436635290656,
        ]);

        test_samples(&Open01, 0f32, &[0.0035963655, 0.73460525, 0.09778172]);
        test_samples(&Open01, 0f64, &[
            0.7346051961657584,
            0.20298547462974248,
            0.8166436635290656,
        ]);
    }
}
