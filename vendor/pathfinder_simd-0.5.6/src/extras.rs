// pathfinder/simd/src/extras.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::default::{F32x2, F32x4, I32x2, I32x4};
use std::ops::{AddAssign, MulAssign, Neg, SubAssign};

// Two 32-bit floats

impl F32x2 {
    // Constructors

    #[inline]
    pub fn from_slice(slice: &[f32]) -> F32x2 {
        F32x2::new(slice[0], slice[1])
    }

    // Accessors

    #[inline]
    pub fn x(self) -> f32 {
        self[0]
    }

    #[inline]
    pub fn y(self) -> f32 {
        self[1]
    }

    // Mutators

    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self[0] = x
    }

    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self[1] = y
    }

    // Comparisons

    #[inline]
    pub fn approx_eq(self, other: F32x2, epsilon: f32) -> bool {
        (self - other).abs().packed_gt(F32x2::splat(epsilon)).all_false()
    }
}

impl AddAssign for F32x2 {
    #[inline]
    fn add_assign(&mut self, other: F32x2) {
        *self = *self + other
    }
}

impl SubAssign for F32x2 {
    #[inline]
    fn sub_assign(&mut self, other: F32x2) {
        *self = *self - other
    }
}

impl MulAssign for F32x2 {
    #[inline]
    fn mul_assign(&mut self, other: F32x2) {
        *self = *self * other
    }
}

impl Neg for F32x2 {
    type Output = F32x2;
    #[inline]
    fn neg(self) -> F32x2 {
        F32x2::default() - self
    }
}

// Four 32-bit floats

impl F32x4 {
    // Constructors

    #[inline]
    pub fn from_slice(slice: &[f32]) -> F32x4 {
        F32x4::new(slice[0], slice[1], slice[2], slice[3])
    }

    // Accessors

    #[inline]
    pub fn x(self) -> f32 {
        self[0]
    }

    #[inline]
    pub fn y(self) -> f32 {
        self[1]
    }

    #[inline]
    pub fn z(self) -> f32 {
        self[2]
    }

    #[inline]
    pub fn w(self) -> f32 {
        self[3]
    }

    // Mutators

    #[inline]
    pub fn set_x(&mut self, x: f32) {
        self[0] = x
    }

    #[inline]
    pub fn set_y(&mut self, y: f32) {
        self[1] = y
    }

    #[inline]
    pub fn set_z(&mut self, z: f32) {
        self[2] = z
    }

    #[inline]
    pub fn set_w(&mut self, w: f32) {
        self[3] = w
    }

    // Comparisons

    #[inline]
    pub fn approx_eq(self, other: F32x4, epsilon: f32) -> bool {
        (self - other).abs().packed_gt(F32x4::splat(epsilon)).all_false()
    }
}

impl AddAssign for F32x4 {
    #[inline]
    fn add_assign(&mut self, other: F32x4) {
        *self = *self + other
    }
}

impl SubAssign for F32x4 {
    #[inline]
    fn sub_assign(&mut self, other: F32x4) {
        *self = *self - other
    }
}

impl MulAssign for F32x4 {
    #[inline]
    fn mul_assign(&mut self, other: F32x4) {
        *self = *self * other
    }
}

impl Neg for F32x4 {
    type Output = F32x4;
    #[inline]
    fn neg(self) -> F32x4 {
        F32x4::default() - self
    }
}

// Two 32-bit integers

impl AddAssign for I32x2 {
    #[inline]
    fn add_assign(&mut self, other: I32x2) {
        *self = *self + other
    }
}

impl SubAssign for I32x2 {
    #[inline]
    fn sub_assign(&mut self, other: I32x2) {
        *self = *self - other
    }
}

impl MulAssign for I32x2 {
    #[inline]
    fn mul_assign(&mut self, other: I32x2) {
        *self = *self * other
    }
}

impl Neg for I32x2 {
    type Output = I32x2;
    #[inline]
    fn neg(self) -> I32x2 {
        I32x2::default() - self
    }
}

// Four 32-bit integers

impl I32x4 {
    // Accessors

    #[inline]
    pub fn x(self) -> i32 {
        self[0]
    }

    #[inline]
    pub fn y(self) -> i32 {
        self[1]
    }

    #[inline]
    pub fn z(self) -> i32 {
        self[2]
    }

    #[inline]
    pub fn w(self) -> i32 {
        self[3]
    }
}

impl AddAssign for I32x4 {
    #[inline]
    fn add_assign(&mut self, other: I32x4) {
        *self = *self + other
    }
}

impl SubAssign for I32x4 {
    #[inline]
    fn sub_assign(&mut self, other: I32x4) {
        *self = *self - other
    }
}

impl MulAssign for I32x4 {
    #[inline]
    fn mul_assign(&mut self, other: I32x4) {
        *self = *self * other
    }
}

impl Neg for I32x4 {
    type Output = I32x4;
    #[inline]
    fn neg(self) -> I32x4 {
        I32x4::default() - self
    }
}
