// pathfinder/geometry/src/util.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Various utilities.

use std::f32;

pub const EPSILON: f32 = 0.001;

/// Approximate equality.
#[inline]
pub fn approx_eq(a: f32, b: f32) -> bool {
    f32::abs(a - b) <= EPSILON
}

/// Linear interpolation.
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Clamping.
#[inline]
pub fn clamp(x: f32, min_val: f32, max_val: f32) -> f32 {
    f32::min(max_val, f32::max(min_val, x))
}

/// Divides `a` by `b`, rounding up.
#[inline]
pub fn alignup_i32(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}
