// Copyright 2012 Google Inc.
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use tiny_skia_path::{Scalar, SCALAR_MAX};

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use tiny_skia_path::NoStdFloat;

// Must be first, because of macro scope rules.
#[macro_use]
pub mod point64;

pub mod cubic64;
pub mod line_cubic_intersections;
mod quad64;

// The code below is from SkPathOpsTypes.

const DBL_EPSILON_ERR: f64 = f64::EPSILON * 4.0;
const FLT_EPSILON_HALF: f64 = (f32::EPSILON / 2.0) as f64;
const FLT_EPSILON_CUBED: f64 = (f32::EPSILON * f32::EPSILON * f32::EPSILON) as f64;
const FLT_EPSILON_INVERSE: f64 = 1.0 / f32::EPSILON as f64;

pub trait Scalar64 {
    fn bound(self, min: Self, max: Self) -> Self;
    fn between(self, a: f64, b: f64) -> bool;
    fn precisely_zero(self) -> bool;
    fn approximately_zero_or_more(self) -> bool;
    fn approximately_one_or_less(self) -> bool;
    fn approximately_zero(self) -> bool;
    fn approximately_zero_inverse(self) -> bool;
    fn approximately_zero_cubed(self) -> bool;
    fn approximately_zero_half(self) -> bool;
    fn approximately_zero_when_compared_to(self, other: Self) -> bool;
    fn approximately_equal(self, other: Self) -> bool;
    fn approximately_equal_half(self, other: Self) -> bool;
    fn almost_dequal_ulps(self, other: Self) -> bool;
}

impl Scalar64 for f64 {
    // Works just like SkTPin, returning `max` for NaN/inf
    fn bound(self, min: Self, max: Self) -> Self {
        max.min(self).max(min)
    }

    /// Returns true if (a <= self <= b) || (a >= self >= b).
    fn between(self, a: f64, b: f64) -> bool {
        debug_assert!(
            ((a <= self && self <= b) || (a >= self && self >= b))
                == ((a - self) * (b - self) <= 0.0)
                || (a.precisely_zero() && self.precisely_zero() && b.precisely_zero())
        );

        (a - self) * (b - self) <= 0.0
    }

    fn precisely_zero(self) -> bool {
        self.abs() < DBL_EPSILON_ERR
    }

    fn approximately_zero_or_more(self) -> bool {
        self > -f64::EPSILON
    }

    fn approximately_one_or_less(self) -> bool {
        self < 1.0 + f64::EPSILON
    }

    fn approximately_zero(self) -> bool {
        self.abs() < f64::EPSILON
    }

    fn approximately_zero_inverse(self) -> bool {
        self.abs() > FLT_EPSILON_INVERSE
    }

    fn approximately_zero_cubed(self) -> bool {
        self.abs() < FLT_EPSILON_CUBED
    }

    fn approximately_zero_half(self) -> bool {
        self < FLT_EPSILON_HALF
    }

    fn approximately_zero_when_compared_to(self, other: Self) -> bool {
        self == 0.0 || self.abs() < (other * (f32::EPSILON as f64)).abs()
    }

    // Use this for comparing Ts in the range of 0 to 1. For general numbers (larger and smaller) use
    // AlmostEqualUlps instead.
    fn approximately_equal(self, other: Self) -> bool {
        (self - other).approximately_zero()
    }

    fn approximately_equal_half(self, other: Self) -> bool {
        (self - other).approximately_zero_half()
    }

    fn almost_dequal_ulps(self, other: Self) -> bool {
        if self.abs() < SCALAR_MAX as f64 && other.abs() < SCALAR_MAX as f64 {
            (self as f32).almost_dequal_ulps(other as f32)
        } else {
            (self - other).abs() / self.abs().max(other.abs()) < (f32::EPSILON * 16.0) as f64
        }
    }
}

pub fn cube_root(x: f64) -> f64 {
    if x.approximately_zero_cubed() {
        return 0.0;
    }

    let result = halley_cbrt3d(x.abs());
    if x < 0.0 {
        -result
    } else {
        result
    }
}

// cube root approximation using 3 iterations of Halley's method (double)
fn halley_cbrt3d(d: f64) -> f64 {
    let mut a = cbrt_5d(d);
    a = cbrta_halleyd(a, d);
    a = cbrta_halleyd(a, d);
    cbrta_halleyd(a, d)
}

// cube root approximation using bit hack for 64-bit float
// adapted from Kahan's cbrt
fn cbrt_5d(d: f64) -> f64 {
    let b1 = 715094163;
    let mut t: f64 = 0.0;
    let pt: &mut [u32; 2] = bytemuck::cast_mut(&mut t);
    let px: [u32; 2] = bytemuck::cast(d);
    pt[1] = px[1] / 3 + b1;
    t
}

// iterative cube root approximation using Halley's method (double)
fn cbrta_halleyd(a: f64, r: f64) -> f64 {
    let a3 = a * a * a;
    a * (a3 + r + r) / (a3 + a3 + r)
}

fn interp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
