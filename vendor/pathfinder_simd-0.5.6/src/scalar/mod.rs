// pathfinder/simd/src/scalar.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::f32;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, BitAnd, BitOr, Div, Index, IndexMut, Mul, Shr, Sub, Not};

mod swizzle_f32x4;
mod swizzle_i32x4;

// Two 32-bit floats

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct F32x2(pub [f32; 2]);

impl F32x2 {
    // Constructors

    #[inline]
    pub fn new(a: f32, b: f32) -> F32x2 {
        F32x2([a, b])
    }

    #[inline]
    pub fn splat(x: f32) -> F32x2 {
        F32x2([x, x])
    }

    // Basic operations

    #[inline]
    pub fn approx_recip(self) -> F32x2 {
        F32x2([1.0 / self[0], 1.0 / self[1]])
    }

    #[inline]
    pub fn min(self, other: F32x2) -> F32x2 {
        F32x2([f32::min(self[0], other[0]), f32::min(self[1], other[1])])
    }

    #[inline]
    pub fn max(self, other: F32x2) -> F32x2 {
        F32x2([f32::max(self[0], other[0]), f32::max(self[1], other[1])])
    }

    #[inline]
    pub fn clamp(self, min: F32x2, max: F32x2) -> F32x2 {
        self.max(min).min(max)
    }

    #[inline]
    pub fn abs(self) -> F32x2 {
        F32x2([self[0].abs(), self[1].abs()])
    }

    #[inline]
    pub fn floor(self) -> F32x2 {
        F32x2([self[0].floor(), self[1].floor()])
    }

    #[inline]
    pub fn ceil(self) -> F32x2 {
        F32x2([self[0].ceil(), self[1].ceil()])
    }

    #[inline]
    pub fn sqrt(self) -> F32x2 {
        F32x2([self[0].sqrt(), self[1].sqrt()])
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: F32x2) -> U32x2 {
        U32x2([
            if self[0] == other[0] { !0 } else { 0 },
            if self[1] == other[1] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_gt(self, other: F32x2) -> U32x2 {
        U32x2([
            if self[0] > other[0] { !0 } else { 0 },
            if self[1] > other[1] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_lt(self, other: F32x2) -> U32x2 {
        U32x2([
            if self[0] < other[0] { !0 } else { 0 },
            if self[1] < other[1] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_le(self, other: F32x2) -> U32x2 {
        U32x2([
            if self[0] <= other[0] { !0 } else { 0 },
            if self[1] <= other[1] { !0 } else { 0 },
        ])
    }

    // Conversions

    #[inline]
    pub fn to_f32x4(self) -> F32x4 {
        F32x4([self[0] as f32, self[1] as f32, 0.0, 0.0])
    }

    /// Converts these packed floats to integers via rounding.
    #[inline]
    pub fn to_i32x2(self) -> I32x2 {
        I32x2([self[0].round_ties_even() as i32, self[1].round_ties_even() as i32])
    }

    /// Converts these packed floats to integers via rounding.
    #[inline]
    pub fn to_i32x4(self) -> I32x4 {
        I32x4([self[0].round_ties_even() as i32, self[1].round_ties_even() as i32, 0, 0])
    }

    // Swizzle

    #[inline]
    pub fn yx(self) -> F32x2 {
        F32x2([self[1], self[0]])
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: F32x2) -> F32x4 {
        F32x4([self[0], self[1], other[0], other[1]])
    }
}

impl Index<usize> for F32x2 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &f32 {
        &self.0[index]
    }
}

impl IndexMut<usize> for F32x2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.0[index]
    }
}

impl Add<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn add(self, other: F32x2) -> F32x2 {
        F32x2([self[0] + other[0], self[1] + other[1]])
    }
}

impl Div<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn div(self, other: F32x2) -> F32x2 {
        F32x2([self[0] / other[0], self[1] / other[1]])
    }
}

impl Mul<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn mul(self, other: F32x2) -> F32x2 {
        F32x2([self[0] * other[0], self[1] * other[1]])
    }
}

impl Sub<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn sub(self, other: F32x2) -> F32x2 {
        F32x2([self[0] - other[0], self[1] - other[1]])
    }
}

// Four 32-bit floats

#[derive(Clone, Copy, Default, PartialEq)]
pub struct F32x4(pub [f32; 4]);

impl F32x4 {
    #[inline]
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> F32x4 {
        F32x4([a, b, c, d])
    }

    #[inline]
    pub fn splat(x: f32) -> F32x4 {
        F32x4([x; 4])
    }

    // Basic operations

    #[inline]
    pub fn approx_recip(self) -> F32x4 {
        F32x4([1.0 / self[0], 1.0 / self[1], 1.0 / self[2], 1.0 / self[3]])
    }

    #[inline]
    pub fn min(self, other: F32x4) -> F32x4 {
        F32x4([
            self[0].min(other[0]),
            self[1].min(other[1]),
            self[2].min(other[2]),
            self[3].min(other[3]),
        ])
    }

    #[inline]
    pub fn max(self, other: F32x4) -> F32x4 {
        F32x4([
            self[0].max(other[0]),
            self[1].max(other[1]),
            self[2].max(other[2]),
            self[3].max(other[3]),
        ])
    }

    #[inline]
    pub fn clamp(self, min: F32x4, max: F32x4) -> F32x4 {
        self.max(min).min(max)
    }

    #[inline]
    pub fn abs(self) -> F32x4 {
        F32x4([self[0].abs(), self[1].abs(), self[2].abs(), self[3].abs()])
    }

    #[inline]
    pub fn floor(self) -> F32x4 {
        F32x4([
            self[0].floor(),
            self[1].floor(),
            self[2].floor(),
            self[3].floor(),
        ])
    }

    #[inline]
    pub fn ceil(self) -> F32x4 {
        F32x4([
            self[0].ceil(),
            self[1].ceil(),
            self[2].ceil(),
            self[3].ceil(),
        ])
    }

    #[inline]
    pub fn sqrt(self) -> F32x4 {
        F32x4([
            self[0].sqrt(),
            self[1].sqrt(),
            self[2].sqrt(),
            self[3].sqrt(),
        ])
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: F32x4) -> U32x4 {
        U32x4([
            if self[0] == other[0] { !0 } else { 0 },
            if self[1] == other[1] { !0 } else { 0 },
            if self[2] == other[2] { !0 } else { 0 },
            if self[3] == other[3] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_gt(self, other: F32x4) -> U32x4 {
        U32x4([
            if self[0] > other[0] { !0 } else { 0 },
            if self[1] > other[1] { !0 } else { 0 },
            if self[2] > other[2] { !0 } else { 0 },
            if self[3] > other[3] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_le(self, other: F32x4) -> U32x4 {
        U32x4([
            if self[0] <= other[0] { !0 } else { 0 },
            if self[1] <= other[1] { !0 } else { 0 },
            if self[2] <= other[2] { !0 } else { 0 },
            if self[3] <= other[3] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_lt(self, other: F32x4) -> U32x4 {
        U32x4([
            if self[0] < other[0] { !0 } else { 0 },
            if self[1] < other[1] { !0 } else { 0 },
            if self[2] < other[2] { !0 } else { 0 },
            if self[3] < other[3] { !0 } else { 0 },
        ])
    }

    /// Converts these packed floats to integers via rounding.
    #[inline]
    pub fn to_i32x4(self) -> I32x4 {
        I32x4([
            self[0].round_ties_even() as i32,
            self[1].round_ties_even() as i32,
            self[2].round_ties_even() as i32,
            self[3].round_ties_even() as i32,
        ])
    }

    // Swizzle conversions

    #[inline]
    pub fn xy(self) -> F32x2 {
        F32x2([self[0], self[1]])
    }

    #[inline]
    pub fn xw(self) -> F32x2 {
        F32x2([self[0], self[3]])
    }

    #[inline]
    pub fn yx(self) -> F32x2 {
        F32x2([self[1], self[0]])
    }

    #[inline]
    pub fn zy(self) -> F32x2 {
        F32x2([self[2], self[1]])
    }

    #[inline]
    pub fn zw(self) -> F32x2 {
        F32x2([self[2], self[3]])
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: F32x4) -> F32x4 {
        F32x4([self[0], self[1], other[0], other[1]])
    }

    #[inline]
    pub fn concat_xy_zw(self, other: F32x4) -> F32x4 {
        F32x4([self[0], self[1], other[2], other[3]])
    }

    #[inline]
    pub fn concat_zw_zw(self, other: F32x4) -> F32x4 {
        F32x4([self[2], self[3], other[2], other[3]])
    }

    #[inline]
    pub fn concat_wz_yx(self, other: F32x4) -> F32x4 {
        F32x4([self[3], self[2], other[1], other[0]])
    }
}

impl Index<usize> for F32x4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &f32 {
        &self.0[index]
    }
}

impl IndexMut<usize> for F32x4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.0[index]
    }
}

impl Debug for F32x4 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}, {}, {}, {}>", self[0], self[1], self[2], self[3])
    }
}

impl Add<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn add(self, other: F32x4) -> F32x4 {
        F32x4([
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ])
    }
}

impl Div<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn div(self, other: F32x4) -> F32x4 {
        F32x4([
            self[0] / other[0],
            self[1] / other[1],
            self[2] / other[2],
            self[3] / other[3],
        ])
    }
}

impl Mul<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn mul(self, other: F32x4) -> F32x4 {
        F32x4([
            self[0] * other[0],
            self[1] * other[1],
            self[2] * other[2],
            self[3] * other[3],
        ])
    }
}

impl Sub<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn sub(self, other: F32x4) -> F32x4 {
        F32x4([
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3],
        ])
    }
}

// Two 32-bit signed integers

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct I32x2([i32; 2]);

impl I32x2 {
    #[inline]
    pub fn new(x: i32, y: i32) -> I32x2 {
        I32x2([x, y])
    }

    #[inline]
    pub fn splat(x: i32) -> I32x2 {
        I32x2([x, x])
    }

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
    pub fn concat_xy_xy(self, other: I32x2) -> I32x4 {
        I32x4([self[0], self[1], other[0], other[1]])
    }

    #[inline]
    pub fn min(self, other: I32x2) -> I32x2 {
        I32x2([
            self[0].min(other[0]),
            self[1].min(other[1]),
        ])
    }

    #[inline]
    pub fn max(self, other: I32x2) -> I32x2 {
        I32x2([
            self[0].max(other[0]),
            self[1].max(other[1]),
        ])
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: I32x2) -> U32x2 {
        U32x2([
            if self[0] == other[0] { !0 } else { 0 },
            if self[1] == other[1] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_gt(self, other: I32x2) -> U32x2 {
        U32x2([
            if self[0] > other[0] { !0 } else { 0 },
            if self[1] > other[1] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_le(self, other: I32x2) -> U32x2 {
        U32x2([
            if self[0] <= other[0] { !0 } else { 0 },
            if self[1] <= other[1] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_lt(self, other: I32x2) -> U32x2 {
        U32x2([
            if self[0] < other[0] { !0 } else { 0 },
            if self[1] < other[1] { !0 } else { 0 },
        ])
    } 

    // Conversions

    /// Converts these packed integers to floats.
    #[inline]
    pub fn to_f32x2(self) -> F32x2 {
        F32x2([self[0] as f32, self[1] as f32])
    }
}

impl Index<usize> for I32x2 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &i32 {
        &self.0[index]
    }
}

impl IndexMut<usize> for I32x2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        &mut self.0[index]
    }
}

impl Add<I32x2> for I32x2 {
    type Output = I32x2;
    #[inline]
    fn add(self, other: I32x2) -> I32x2 {
        I32x2([self[0] + other[0], self[1] + other[1]])
    }
}

impl Sub<I32x2> for I32x2 {
    type Output = I32x2;
    #[inline]
    fn sub(self, other: I32x2) -> I32x2 {
        I32x2([self[0] - other[0], self[1] - other[1]])
    }
}

impl Mul<I32x2> for I32x2 {
    type Output = I32x2;
    #[inline]
    fn mul(self, other: I32x2) -> I32x2 {
        I32x2([self[0] * other[0], self[1] * other[1]])
    }
}

// Four 32-bit signed integers

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct I32x4([i32; 4]);

impl I32x4 {
    #[inline]
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> I32x4 {
        I32x4([a, b, c, d])
    }

    #[inline]
    pub fn splat(x: i32) -> I32x4 {
        I32x4([x; 4])
    }

    // Basic operations

    #[inline]
    pub fn min(self, other: I32x4) -> I32x4 {
        I32x4([
            self[0].min(other[0]),
            self[1].min(other[1]),
            self[2].min(other[2]),
            self[3].min(other[3]),
        ])
    }

    #[inline]
    pub fn max(self, other: I32x4) -> I32x4 {
        I32x4([
            self[0].max(other[0]),
            self[1].max(other[1]),
            self[2].max(other[2]),
            self[3].max(other[3]),
        ])
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: I32x4) -> U32x4 {
        U32x4([
            if self[0] == other[0] { !0 } else { 0 },
            if self[1] == other[1] { !0 } else { 0 },
            if self[2] == other[2] { !0 } else { 0 },
            if self[3] == other[3] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_gt(self, other: I32x4) -> U32x4 {
        U32x4([
            if self[0] > other[0] { !0 } else { 0 },
            if self[1] > other[1] { !0 } else { 0 },
            if self[2] > other[2] { !0 } else { 0 },
            if self[3] > other[3] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_le(self, other: I32x4) -> U32x4 {
        U32x4([
            if self[0] <= other[0] { !0 } else { 0 },
            if self[1] <= other[1] { !0 } else { 0 },
            if self[2] <= other[2] { !0 } else { 0 },
            if self[3] <= other[3] { !0 } else { 0 },
        ])
    }

    #[inline]
    pub fn packed_lt(self, other: I32x4) -> U32x4 {
        U32x4([
            if self[0] < other[0] { !0 } else { 0 },
            if self[1] < other[1] { !0 } else { 0 },
            if self[2] < other[2] { !0 } else { 0 },
            if self[3] < other[3] { !0 } else { 0 },
        ])
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: I32x4) -> I32x4 {
        I32x4([self[0], self[1], other[0], other[1]])
    }

    #[inline]
    pub fn concat_zw_zw(self, other: I32x4) -> I32x4 {
        I32x4([self[2], self[3], other[2], other[3]])
    }

    // Swizzle conversions

    #[inline]
    pub fn xy(self) -> I32x2 {
        I32x2([self[0], self[1]])
    }

    #[inline]
    pub fn xw(self) -> I32x2 {
        I32x2([self[0], self[3]])
    }

    #[inline]
    pub fn zy(self) -> I32x2 {
        I32x2([self[2], self[1]])
    }

    #[inline]
    pub fn zw(self) -> I32x2 {
        I32x2([self[2], self[3]])
    }

    // Conversions

    /// Converts these packed integers to floats.
    #[inline]
    pub fn to_f32x4(self) -> F32x4 {
        F32x4([
            self[0] as f32,
            self[1] as f32,
            self[2] as f32,
            self[3] as f32,
        ])
    }

    /// Converts these packed signed integers to unsigned integers.
    ///
    /// Overflowing values will wrap around.
    ///
    /// FIXME(pcwalton): Should they? This will assert on overflow in debug.
    #[inline]
    pub fn to_u32x4(self) -> U32x4 {
        U32x4([self[0] as u32, self[1] as u32, self[2] as u32, self[3] as u32])
    }
}

impl Index<usize> for I32x4 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &i32 {
        &self.0[index]
    }
}

impl IndexMut<usize> for I32x4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        &mut self.0[index]
    }
}

impl Add<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn add(self, other: I32x4) -> I32x4 {
        I32x4([
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ])
    }
}

impl Sub<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn sub(self, other: I32x4) -> I32x4 {
        I32x4([
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3],
        ])
    }
}

impl Mul<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn mul(self, other: I32x4) -> I32x4 {
        I32x4([
            self[0] * other[0],
            self[1] * other[1],
            self[2] * other[2],
            self[3] * other[3],
        ])
    }
}

impl BitAnd<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn bitand(self, other: I32x4) -> I32x4 {
        I32x4([self[0] & other[0], self[1] & other[1], self[2] & other[2], self[3] & other[3]])
    }
}

impl BitOr<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn bitor(self, other: I32x4) -> I32x4 {
        I32x4([self[0] | other[0], self[1] | other[1], self[2] | other[2], self[3] | other[3]])
    }
}

impl Shr<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn shr(self, other: I32x4) -> I32x4 {
        I32x4([
            self[0] >> other[0],
            self[1] >> other[1],
            self[2] >> other[2],
            self[3] >> other[3],
        ])
    }
}

// Two 32-bit unsigned integers

#[derive(Clone, Copy)]
pub struct U32x2(pub [u32; 2]);

impl U32x2 {
    #[inline]
    pub fn new(x: u32, y: u32) -> U32x2 {
        U32x2([x, y])
    }

    #[inline]
    pub fn splat(x: u32) -> U32x2 {
        U32x2::new(x, x)
    }

    /// Returns true if both booleans in this vector are true.
    ///
    /// The result is *undefined* if both values in this vector are not booleans. A boolean is a
    /// value with all bits set or all bits clear (i.e. !0 or 0).
    #[inline]
    pub fn all_true(&self) -> bool {
        self[0] == !0 && self[1] == !0
    }

    /// Returns true if both booleans in this vector are false.
    ///
    /// The result is *undefined* if both values in this vector are not booleans. A boolean is a
    /// value with all bits set or all bits clear (i.e. !0 or 0).
    #[inline]
    pub fn all_false(&self) -> bool {
        self[0] == 0 && self[1] == 0
    }

    #[inline]
    pub fn to_i32x2(self) -> I32x2 {
        I32x2::new(self[0] as i32, self[1] as i32)
    }

}

impl BitAnd<U32x2> for U32x2 {
    type Output = U32x2;
    #[inline]
    fn bitand(self, other: U32x2) -> U32x2 {
        U32x2([self[0] & other[0], self[1] & other[1]])
    }
}

impl BitOr<U32x2> for U32x2 {
    type Output = U32x2;
    #[inline]
    fn bitor(self, other: U32x2) -> U32x2 {
        U32x2([self[0] | other[0], self[1] | other[1]])
    }
}

impl Not for U32x2 {
    type Output = U32x2;
    #[inline]
    fn not(self) -> U32x2 {
        U32x2([!self[0], !self[1]])
    }
}

impl Index<usize> for U32x2 {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &u32 {
        &self.0[index]
    }
}

// Four 32-bit unsigned integers

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct U32x4(pub [u32; 4]);

impl U32x4 {
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> U32x4 {
        U32x4([a, b, c, d])
    }

    // Conversions

    /// Converts these packed unsigned integers to signed integers.
    ///
    /// Overflowing values will wrap around.
    ///
    /// FIXME(pcwalton): Should they? This will assert on overflow in debug.
    #[inline]
    pub fn to_i32x4(self) -> I32x4 {
        I32x4([self[0] as i32, self[1] as i32, self[2] as i32, self[3] as i32])
    }

    // Basic operations

    /// Returns true if all four booleans in this vector are true.
    ///
    /// The result is *undefined* if all four values in this vector are not booleans. A boolean is
    /// a value with all bits set or all bits clear (i.e. !0 or 0).
    #[inline]
    pub fn all_true(&self) -> bool {
        self[0] == !0 && self[1] == !0 && self[2] == !0 && self[3] == !0
    }

    /// Returns true if all four booleans in this vector are false.
    ///
    /// The result is *undefined* if all four values in this vector are not booleans. A boolean is
    /// a value with all bits set or all bits clear (i.e. !0 or 0).
    #[inline]
    pub fn all_false(&self) -> bool {
        self[0] == 0 && self[1] == 0 && self[2] == 0 && self[3] == 0
    }
}

impl Index<usize> for U32x4 {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &u32 {
        &self.0[index]
    }
}

impl Shr<u32> for U32x4 {
    type Output = U32x4;
    #[inline]
    fn shr(self, amount: u32) -> U32x4 {
        U32x4([self[0] >> amount, self[1] >> amount, self[2] >> amount, self[3] >> amount])
    }
}
