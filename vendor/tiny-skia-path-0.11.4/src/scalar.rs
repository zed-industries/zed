// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::floating_point::f32_as_2s_compliment;

#[allow(missing_docs)]
pub const SCALAR_MAX: f32 = 3.402823466e+38;
#[allow(missing_docs)]
pub const SCALAR_NEARLY_ZERO: f32 = 1.0 / (1 << 12) as f32;
#[allow(missing_docs)]
pub const SCALAR_ROOT_2_OVER_2: f32 = 0.707106781;

/// Float number extension methods.
///
/// Mainly for internal use. Do not rely on it!
#[allow(missing_docs)]
pub trait Scalar {
    fn half(self) -> Self;
    fn ave(self, other: Self) -> Self;
    fn sqr(self) -> Self;
    fn invert(self) -> Self;
    fn bound(self, min: Self, max: Self) -> Self;
    fn is_nearly_equal(self, other: Self) -> bool;
    fn is_nearly_zero(self) -> bool;
    fn is_nearly_zero_within_tolerance(self, tolerance: Self) -> bool;
    fn almost_dequal_ulps(self, other: Self) -> bool;
}

impl Scalar for f32 {
    fn half(self) -> f32 {
        self * 0.5
    }

    fn ave(self, other: Self) -> f32 {
        (self + other) * 0.5
    }

    fn sqr(self) -> f32 {
        self * self
    }

    fn invert(self) -> f32 {
        1.0 / self
    }

    // Works just like SkTPin, returning `max` for NaN/inf
    /// A non-panicking clamp.
    fn bound(self, min: Self, max: Self) -> Self {
        max.min(self).max(min)
    }

    fn is_nearly_equal(self, other: Self) -> bool {
        (self - other).abs() <= SCALAR_NEARLY_ZERO
    }

    fn is_nearly_zero(self) -> bool {
        self.is_nearly_zero_within_tolerance(SCALAR_NEARLY_ZERO)
    }

    fn is_nearly_zero_within_tolerance(self, tolerance: Self) -> bool {
        debug_assert!(tolerance >= 0.0);
        self.abs() <= tolerance
    }

    // From SkPathOpsTypes.
    fn almost_dequal_ulps(self, other: Self) -> bool {
        const ULPS_EPSILON: i32 = 16;
        let a_bits = f32_as_2s_compliment(self);
        let b_bits = f32_as_2s_compliment(other);
        // Find the difference in ULPs.
        a_bits < b_bits + ULPS_EPSILON && b_bits < a_bits + ULPS_EPSILON
    }
}

#[allow(missing_docs)]
#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
pub trait NoStdFloat {
    fn trunc(self) -> Self;
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn ceil(self) -> Self;
    fn floor(self) -> Self;
    fn round(self) -> Self;
    fn powf(self, y: Self) -> Self;
    fn acos(self) -> Self;
}

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
impl NoStdFloat for f32 {
    fn trunc(self) -> Self {
        libm::truncf(self)
    }
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }
    fn abs(self) -> Self {
        libm::fabsf(self)
    }
    fn sin(self) -> Self {
        libm::sinf(self)
    }
    fn cos(self) -> Self {
        libm::cosf(self)
    }
    fn ceil(self) -> Self {
        libm::ceilf(self)
    }
    fn floor(self) -> Self {
        libm::floorf(self)
    }
    fn round(self) -> Self {
        libm::roundf(self)
    }
    fn powf(self, y: Self) -> Self {
        libm::powf(self, y)
    }
    fn acos(self) -> Self {
        libm::acosf(self)
    }
}

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
impl NoStdFloat for f64 {
    fn trunc(self) -> Self {
        libm::trunc(self)
    }
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }
    fn abs(self) -> Self {
        libm::fabs(self)
    }
    fn sin(self) -> Self {
        libm::sin(self)
    }
    fn cos(self) -> Self {
        libm::cos(self)
    }
    fn ceil(self) -> Self {
        libm::ceil(self)
    }
    fn floor(self) -> Self {
        libm::floor(self)
    }
    fn round(self) -> Self {
        libm::round(self)
    }
    fn powf(self, y: Self) -> Self {
        libm::pow(self, y)
    }
    fn acos(self) -> Self {
        libm::acos(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound() {
        assert_eq!(f32::NAN.bound(0.0, 1.0), 1.0);
        assert_eq!(f32::INFINITY.bound(0.0, 1.0), 1.0);
        assert_eq!(f32::NEG_INFINITY.bound(0.0, 1.0), 0.0);
        assert_eq!(f32::EPSILON.bound(0.0, 1.0), f32::EPSILON);
        assert_eq!(0.5.bound(0.0, 1.0), 0.5);
        assert_eq!((-1.0).bound(0.0, 1.0), 0.0);
        assert_eq!(2.0.bound(0.0, 1.0), 1.0);
    }
}
