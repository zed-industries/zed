// pathfinder/geometry/src/angle.rs
//
// Copyright © 2020 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Angle utilities.

use std::f32::consts::PI;

#[inline]
pub fn angle_from_degrees(degrees: f32) -> f32 {
    const SCALE: f32 = 2.0 * PI / 360.0;
    degrees * SCALE
}
