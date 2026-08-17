// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::scalar::Scalar;

pub use strict_num::{FiniteF32, NonZeroPositiveF32, NormalizedF32};

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use crate::NoStdFloat;

pub(crate) const FLOAT_PI: f32 = 3.14159265;

const MAX_I32_FITS_IN_F32: f32 = 2147483520.0;
const MIN_I32_FITS_IN_F32: f32 = -MAX_I32_FITS_IN_F32;

// TODO: is there an std alternative?
/// Custom float to integer conversion routines.
pub trait SaturateCast<T>: Sized {
    /// Return the closest integer for the given float.
    fn saturate_from(n: T) -> Self;
}

impl SaturateCast<f32> for i32 {
    /// Return the closest integer for the given float.
    ///
    /// Returns MAX_I32_FITS_IN_F32 for NaN.
    fn saturate_from(mut x: f32) -> Self {
        x = if x < MAX_I32_FITS_IN_F32 {
            x
        } else {
            MAX_I32_FITS_IN_F32
        };
        x = if x > MIN_I32_FITS_IN_F32 {
            x
        } else {
            MIN_I32_FITS_IN_F32
        };
        x as i32
    }
}

impl SaturateCast<f64> for i32 {
    /// Return the closest integer for the given double.
    ///
    /// Returns i32::MAX for NaN.
    fn saturate_from(mut x: f64) -> Self {
        x = if x < i32::MAX as f64 {
            x
        } else {
            i32::MAX as f64
        };
        x = if x > i32::MIN as f64 {
            x
        } else {
            i32::MIN as f64
        };
        x as i32
    }
}

/// Custom float to integer rounding routines.
#[allow(missing_docs)]
pub trait SaturateRound<T>: SaturateCast<T> {
    fn saturate_floor(n: T) -> Self;
    fn saturate_ceil(n: T) -> Self;
    fn saturate_round(n: T) -> Self;
}

impl SaturateRound<f32> for i32 {
    fn saturate_floor(x: f32) -> Self {
        Self::saturate_from(x.floor())
    }

    fn saturate_ceil(x: f32) -> Self {
        Self::saturate_from(x.ceil())
    }

    fn saturate_round(x: f32) -> Self {
        Self::saturate_from(x.floor() + 0.5)
    }
}

/// Return the float as a 2s compliment int. Just to be used to compare floats
/// to each other or against positive float-bit-constants (like 0). This does
/// not return the int equivalent of the float, just something cheaper for
/// compares-only.
pub(crate) fn f32_as_2s_compliment(x: f32) -> i32 {
    sign_bit_to_2s_compliment(bytemuck::cast(x))
}

/// Convert a sign-bit int (i.e. float interpreted as int) into a 2s compliement
/// int. This also converts -0 (0x80000000) to 0. Doing this to a float allows
/// it to be compared using normal C operators (<, <=, etc.)
fn sign_bit_to_2s_compliment(mut x: i32) -> i32 {
    if x < 0 {
        x &= 0x7FFFFFFF;
        x = -x;
    }

    x
}

/// An immutable `f32` that is larger than 0 but less then 1.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug)]
#[repr(transparent)]
pub struct NormalizedF32Exclusive(FiniteF32);

impl NormalizedF32Exclusive {
    /// Just a random, valid number.
    pub const ANY: Self = Self::HALF;

    /// A predefined 0.5 value.
    pub const HALF: Self = NormalizedF32Exclusive(unsafe { FiniteF32::new_unchecked(0.5) });

    /// Creates a `NormalizedF32Exclusive`.
    pub fn new(n: f32) -> Option<Self> {
        if n > 0.0 && n < 1.0 {
            // `n` is guarantee to be finite after the bounds check.
            FiniteF32::new(n).map(NormalizedF32Exclusive)
        } else {
            None
        }
    }

    /// Creates a `NormalizedF32Exclusive` clamping the given value.
    ///
    /// Returns zero in case of NaN or infinity.
    pub fn new_bounded(n: f32) -> Self {
        let n = n.bound(f32::EPSILON, 1.0 - f32::EPSILON);
        // `n` is guarantee to be finite after clamping.
        debug_assert!(n.is_finite());
        NormalizedF32Exclusive(unsafe { FiniteF32::new_unchecked(n) })
    }

    /// Returns the value as a primitive type.
    pub fn get(self) -> f32 {
        self.0.get()
    }

    /// Returns the value as a `FiniteF32`.
    pub fn to_normalized(self) -> NormalizedF32 {
        // NormalizedF32 is (0,1), while NormalizedF32 is [0,1], so it will always fit.
        unsafe { NormalizedF32::new_unchecked(self.0.get()) }
    }
}
