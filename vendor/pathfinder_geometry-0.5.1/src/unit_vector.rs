// pathfinder/geometry/src/unit_vector.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A utility module that allows unit vectors to be treated like angles.

use crate::vector::Vector2F;
use pathfinder_simd::default::F32x2;

#[derive(Clone, Copy, Debug)]
pub struct UnitVector(pub Vector2F);

impl UnitVector {
    #[inline]
    pub fn from_angle(theta: f32) -> UnitVector {
        UnitVector(Vector2F::new(theta.cos(), theta.sin()))
    }

    /// Angle addition formula.
    #[inline]
    pub fn rotate_by(&self, other: UnitVector) -> UnitVector {
        let products = (self.0).0.to_f32x4().xyyx() * (other.0).0.to_f32x4().xyxy();
        UnitVector(Vector2F::new(products[0] - products[1], products[2] + products[3]))
    }

    /// Angle subtraction formula.
    #[inline]
    pub fn rev_rotate_by(&self, other: UnitVector) -> UnitVector {
        let products = (self.0).0.to_f32x4().xyyx() * (other.0).0.to_f32x4().xyxy();
        UnitVector(Vector2F::new(products[0] + products[1], products[2] - products[3]))
    }

    /// Half angle formula.
    #[inline]
    pub fn halve_angle(&self) -> UnitVector {
        let x = self.0.x();
        let term = F32x2::new(x, -x);
        UnitVector(Vector2F((F32x2::splat(0.5) * (F32x2::splat(1.0) + term)).max(F32x2::default())
                                                                            .sqrt()))
    }
}
