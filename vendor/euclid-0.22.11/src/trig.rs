// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Trait for basic trigonometry functions, so they can be used on generic numeric types
pub trait Trig {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn fast_atan2(y: Self, x: Self) -> Self;
    fn degrees_to_radians(deg: Self) -> Self;
    fn radians_to_degrees(rad: Self) -> Self;
}

macro_rules! trig {
    ($ty:ident) => {
        impl Trig for $ty {
            #[inline]
            fn sin(self) -> $ty {
                num_traits::Float::sin(self)
            }
            #[inline]
            fn cos(self) -> $ty {
                num_traits::Float::cos(self)
            }
            #[inline]
            fn tan(self) -> $ty {
                num_traits::Float::tan(self)
            }

            /// A slightly faster approximation of `atan2`.
            ///
            /// Note that it does not deal with the case where both x and y are 0.
            #[inline]
            fn fast_atan2(y: $ty, x: $ty) -> $ty {
                // This macro is used with f32 and f64 and clippy warns about the extra
                // precision with f32.
                #![allow(clippy::excessive_precision)]

                // See https://math.stackexchange.com/questions/1098487/atan2-faster-approximation#1105038
                use core::$ty::consts;
                let x_abs = num_traits::Float::abs(x);
                let y_abs = num_traits::Float::abs(y);
                let a = x_abs.min(y_abs) / x_abs.max(y_abs);
                let s = a * a;
                let mut result =
                    ((-0.046_496_474_9 * s + 0.159_314_22) * s - 0.327_622_764) * s * a + a;
                if y_abs > x_abs {
                    result = consts::FRAC_PI_2 - result;
                }
                if x < 0.0 {
                    result = consts::PI - result
                }
                if y < 0.0 {
                    result = -result
                }

                result
            }

            #[inline]
            fn degrees_to_radians(deg: Self) -> Self {
                deg.to_radians()
            }

            #[inline]
            fn radians_to_degrees(rad: Self) -> Self {
                rad.to_degrees()
            }
        }
    };
}

trig!(f32);
trig!(f64);
