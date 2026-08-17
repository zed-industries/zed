// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use crate::NoStdFloat;

// Right now, there are no visible benefits of using SIMD for f32x2. So we don't.
/// A pair of f32 numbers.
///
/// Mainly for internal use. Do not rely on it!
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct f32x2(pub [f32; 2]);

impl f32x2 {
    /// Creates a new pair.
    pub fn new(a: f32, b: f32) -> f32x2 {
        f32x2([a, b])
    }

    /// Creates a new pair from a single value.
    pub fn splat(x: f32) -> f32x2 {
        f32x2([x, x])
    }

    /// Returns an absolute value.
    pub fn abs(self) -> f32x2 {
        f32x2([self.x().abs(), self.y().abs()])
    }

    /// Returns a minimum value.
    pub fn min(self, other: f32x2) -> f32x2 {
        f32x2([pmin(self.x(), other.x()), pmin(self.y(), other.y())])
    }

    /// Returns a maximum value.
    pub fn max(self, other: f32x2) -> f32x2 {
        f32x2([pmax(self.x(), other.x()), pmax(self.y(), other.y())])
    }

    /// Returns a maximum of both values.
    pub fn max_component(self) -> f32 {
        pmax(self.x(), self.y())
    }

    /// Returns the first value.
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    /// Returns the second value.
    pub fn y(&self) -> f32 {
        self.0[1]
    }
}

impl core::ops::Add<f32x2> for f32x2 {
    type Output = f32x2;

    fn add(self, other: f32x2) -> f32x2 {
        f32x2([self.x() + other.x(), self.y() + other.y()])
    }
}

impl core::ops::Sub<f32x2> for f32x2 {
    type Output = f32x2;

    fn sub(self, other: f32x2) -> f32x2 {
        f32x2([self.x() - other.x(), self.y() - other.y()])
    }
}

impl core::ops::Mul<f32x2> for f32x2 {
    type Output = f32x2;

    fn mul(self, other: f32x2) -> f32x2 {
        f32x2([self.x() * other.x(), self.y() * other.y()])
    }
}

impl core::ops::Div<f32x2> for f32x2 {
    type Output = f32x2;

    fn div(self, other: f32x2) -> f32x2 {
        f32x2([self.x() / other.x(), self.y() / other.y()])
    }
}

// A faster and more forgiving f32 min/max implementation.
//
// Unlike std one, we do not care about NaN.

fn pmax(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

fn pmin(a: f32, b: f32) -> f32 {
    if b < a {
        b
    } else {
        a
    }
}
