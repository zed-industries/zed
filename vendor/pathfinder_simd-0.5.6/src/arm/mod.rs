// pathfinder/simd/src/arm.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::arch::aarch64::{self, float32x2_t, float32x4_t, int32x2_t, int32x4_t};
use std::arch::aarch64::{uint32x2_t, uint32x4_t};
use std::f32;
use std::fmt::{self, Debug, Formatter};
use std::intrinsics::simd::*;
use std::mem;
use std::ops::{Add, BitAnd, BitOr, Div, Index, IndexMut, Mul, Not, Shr, Sub};

mod swizzle_f32x4;
mod swizzle_i32x4;

#[repr(simd)]
pub(crate) struct Simd<T, const N: usize>([T; N]);

macro_rules! simd_shuffle2 {
    ($x:expr, $y:expr, <$(const $imm:ident : $ty:ty),+> $idx:expr $(,)?) => {{
        struct ConstParam<$(const $imm: $ty),+>;
        impl<$(const $imm: $ty),+> ConstParam<$($imm),+> {
            const IDX: Simd<u32, 2> = Simd($idx);
        }

        simd_shuffle($x, $y, ConstParam::<$($imm),+>::IDX)
    }};
    ($x:expr, $y:expr, $idx:expr $(,)?) => {{
        const IDX: Simd<u32, 2>  = Simd($idx);
        simd_shuffle($x, $y, IDX)
    }};
}

macro_rules! simd_shuffle4 {
    ($x:expr, $y:expr, <$(const $imm:ident : $ty:ty),+> $idx:expr $(,)?) => {{
        struct ConstParam<$(const $imm: $ty),+>;
        impl<$(const $imm: $ty),+> ConstParam<$($imm),+> {
            const IDX: Simd<u32; 4> = Simd($idx);
        }

        simd_shuffle($x, $y, ConstParam::<$($imm),+>::IDX)
    }};
    ($x:expr, $y:expr, $idx:expr $(,)?) => {{
        const IDX: Simd<u32, 4> = Simd($idx);
        simd_shuffle($x, $y, IDX)
    }};
}

// Two 32-bit floats

#[derive(Clone, Copy)]
pub struct F32x2(pub float32x2_t);

impl F32x2 {
    // Constructors

    #[inline]
    pub fn new(a: f32, b: f32) -> F32x2 {
        unsafe { F32x2(mem::transmute([a, b])) }
    }

    #[inline]
    pub fn splat(x: f32) -> F32x2 {
        F32x2::new(x, x)
    }

    // Basic operations

    #[inline]
    pub fn approx_recip(self) -> F32x2 {
        unsafe { F32x2(vrecpe_v2f32(self.0)) }
    }

    #[inline]
    pub fn min(self, other: F32x2) -> F32x2 {
        unsafe { F32x2(simd_minimum_number_nsz(self.0, other.0)) }
    }

    #[inline]
    pub fn max(self, other: F32x2) -> F32x2 {
        unsafe { F32x2(simd_maximum_number_nsz(self.0, other.0)) }
    }

    #[inline]
    pub fn clamp(self, min: F32x2, max: F32x2) -> F32x2 {
        self.max(min).min(max)
    }

    #[inline]
    pub fn abs(self) -> F32x2 {
        unsafe { F32x2(fabs_v2f32(self.0)) }
    }

    #[inline]
    pub fn floor(self) -> F32x2 {
        unsafe { F32x2(floor_v2f32(self.0)) }
    }

    #[inline]
    pub fn ceil(self) -> F32x2 {
        unsafe { F32x2(ceil_v2f32(self.0)) }
    }

    #[inline]
    pub fn sqrt(self) -> F32x2 {
        unsafe { F32x2(sqrt_v2f32(self.0)) }
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: F32x2) -> U32x2 {
        unsafe { U32x2(simd_eq(self.0, other.0)) }
    }

    #[inline]
    pub fn packed_gt(self, other: F32x2) -> U32x2 {
        unsafe { U32x2(simd_gt(self.0, other.0)) }
    }

    #[inline]
    pub fn packed_lt(self, other: F32x2) -> U32x2 {
        unsafe { U32x2(simd_lt(self.0, other.0)) }
    }

    #[inline]
    pub fn packed_le(self, other: F32x2) -> U32x2 {
        unsafe { U32x2(simd_le(self.0, other.0)) }
    }

    // Conversions

    #[inline]
    pub fn to_f32x4(self) -> F32x4 {
        self.concat_xy_xy(F32x2::default())
    }

    /// Converts these packed floats to integers via rounding.
    #[inline]
    pub fn to_i32x2(self) -> I32x2 {
        unsafe { I32x2(simd_cast(round_v2f32(self.0))) }
    }

    #[inline]
    pub fn to_i32x4(self) -> I32x4 {
        self.to_i32x2().concat_xy_xy(I32x2::default())
    }

    // Swizzle

    #[inline]
    pub fn yx(self) -> F32x2 {
        unsafe { F32x2(simd_shuffle2!(self.0, self.0, [1, 0])) }
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: F32x2) -> F32x4 {
        unsafe { F32x4(simd_shuffle4!(self.0, other.0, [0, 1, 2, 3])) }
    }
}

impl Default for F32x2 {
    #[inline]
    fn default() -> F32x2 {
        F32x2::new(0.0, 0.0)
    }
}

impl Debug for F32x2 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}, {}>", self[0], self[1])
    }
}

impl Index<usize> for F32x2 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &f32 {
        unsafe {
            assert!(index < 2);
            let ptr = &self.0 as *const float32x2_t as *const f32;
            mem::transmute::<*const f32, &f32>(ptr.offset(index as isize))
        }
    }
}

impl IndexMut<usize> for F32x2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        unsafe {
            assert!(index < 2);
            let ptr = &mut self.0 as *mut float32x2_t as *mut f32;
            mem::transmute::<*mut f32, &mut f32>(ptr.offset(index as isize))
        }
    }
}

impl Add<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn add(self, other: F32x2) -> F32x2 {
        unsafe { F32x2(simd_add(self.0, other.0)) }
    }
}

impl Div<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn div(self, other: F32x2) -> F32x2 {
        unsafe { F32x2(simd_div(self.0, other.0)) }
    }
}

impl Mul<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn mul(self, other: F32x2) -> F32x2 {
        unsafe { F32x2(simd_mul(self.0, other.0)) }
    }
}

impl Sub<F32x2> for F32x2 {
    type Output = F32x2;
    #[inline]
    fn sub(self, other: F32x2) -> F32x2 {
        unsafe { F32x2(simd_sub(self.0, other.0)) }
    }
}

impl PartialEq for F32x2 {
    #[inline]
    fn eq(&self, other: &F32x2) -> bool {
        self.packed_eq(*other).all_true()
    }
}

// Four 32-bit floats

#[derive(Clone, Copy)]
pub struct F32x4(pub float32x4_t);

impl F32x4 {
    #[inline]
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> F32x4 {
        unsafe { F32x4(mem::transmute([a, b, c, d])) }
    }

    #[inline]
    pub fn splat(x: f32) -> F32x4 {
        F32x4::new(x, x, x, x)
    }

    // Basic operations

    #[inline]
    pub fn approx_recip(self) -> F32x4 {
        unsafe { F32x4(vrecpe_v4f32(self.0)) }
    }

    #[inline]
    pub fn min(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(simd_minimum_number_nsz(self.0, other.0)) }
    }

    #[inline]
    pub fn max(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(simd_maximum_number_nsz(self.0, other.0)) }
    }

    #[inline]
    pub fn clamp(self, min: F32x4, max: F32x4) -> F32x4 {
        self.max(min).min(max)
    }

    #[inline]
    pub fn abs(self) -> F32x4 {
        unsafe { F32x4(fabs_v4f32(self.0)) }
    }

    #[inline]
    pub fn floor(self) -> F32x4 {
        unsafe { F32x4(floor_v4f32(self.0)) }
    }

    #[inline]
    pub fn ceil(self) -> F32x4 {
        unsafe { F32x4(ceil_v4f32(self.0)) }
    }

    #[inline]
    pub fn sqrt(self) -> F32x4 {
        unsafe { F32x4(sqrt_v4f32(self.0)) }
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: F32x4) -> U32x4 {
        unsafe { U32x4(simd_eq(self.0, other.0)) }
    }

    #[inline]
    pub fn packed_gt(self, other: F32x4) -> U32x4 {
        unsafe { U32x4(simd_gt(self.0, other.0)) }
    }

    #[inline]
    pub fn packed_le(self, other: F32x4) -> U32x4 {
        unsafe { U32x4(simd_le(self.0, other.0)) }
    }

    #[inline]
    pub fn packed_lt(self, other: F32x4) -> U32x4 {
        unsafe { U32x4(simd_lt(self.0, other.0)) }
    }

    // Swizzle conversions

    #[inline]
    pub fn xy(self) -> F32x2 {
        unsafe { F32x2(simd_shuffle2!(self.0, self.0, [0, 1])) }
    }

    #[inline]
    pub fn yx(self) -> F32x2 {
        unsafe { F32x2(simd_shuffle2!(self.0, self.0, [1, 0])) }
    }

    #[inline]
    pub fn xw(self) -> F32x2 {
        unsafe { F32x2(simd_shuffle2!(self.0, self.0, [0, 3])) }
    }

    #[inline]
    pub fn zy(self) -> F32x2 {
        unsafe { F32x2(simd_shuffle2!(self.0, self.0, [2, 1])) }
    }

    #[inline]
    pub fn zw(self) -> F32x2 {
        unsafe { F32x2(simd_shuffle2!(self.0, self.0, [2, 3])) }
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(simd_shuffle4!(self.0, other.0, [0, 1, 4, 5])) }
    }

    #[inline]
    pub fn concat_xy_zw(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(simd_shuffle4!(self.0, other.0, [0, 1, 6, 7])) }
    }

    #[inline]
    pub fn concat_zw_zw(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(simd_shuffle4!(self.0, other.0, [2, 3, 6, 7])) }
    }

    #[inline]
    pub fn concat_wz_yx(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(simd_shuffle4!(self.0, other.0, [3, 2, 5, 4])) }
    }

    // Conversions

    /// Converts these packed floats to integers via rounding.
    #[inline]
    pub fn to_i32x4(self) -> I32x4 {
        unsafe { I32x4(simd_cast(round_v4f32(self.0))) }
    }
}

impl Default for F32x4 {
    #[inline]
    fn default() -> F32x4 {
        F32x4::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl Index<usize> for F32x4 {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &f32 {
        unsafe {
            assert!(index < 4);
            let ptr = &self.0 as *const float32x4_t as *const f32;
            mem::transmute::<*const f32, &f32>(ptr.offset(index as isize))
        }
    }
}

impl IndexMut<usize> for F32x4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        unsafe {
            assert!(index < 4);
            let ptr = &mut self.0 as *mut float32x4_t as *mut f32;
            mem::transmute::<*mut f32, &mut f32>(ptr.offset(index as isize))
        }
    }
}

impl Debug for F32x4 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}, {}, {}, {}>", self[0], self[1], self[2], self[3])
    }
}

impl PartialEq for F32x4 {
    #[inline]
    fn eq(&self, other: &F32x4) -> bool {
        self.packed_eq(*other).all_true()
    }
}

impl Add<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn add(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(simd_add(self.0, other.0)) }
    }
}

impl Div<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn div(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(simd_div(self.0, other.0)) }
    }
}

impl Mul<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn mul(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(simd_mul(self.0, other.0)) }
    }
}

impl Sub<F32x4> for F32x4 {
    type Output = F32x4;
    #[inline]
    fn sub(self, other: F32x4) -> F32x4 {
        unsafe { F32x4(simd_sub(self.0, other.0)) }
    }
}

// Two 32-bit signed integers

#[derive(Clone, Copy, Debug)]
pub struct I32x2(pub int32x2_t);

impl I32x2 {
    #[inline]
    pub fn new(x: i32, y: i32) -> I32x2 {
        unsafe { I32x2(mem::transmute([x, y])) }
    }

    #[inline]
    pub fn splat(x: i32) -> I32x2 {
        I32x2::new(x, x)
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
    pub fn packed_eq(self, other: I32x2) -> U32x2 {
        unsafe { U32x2(simd_eq(self.0, other.0)) }
    }

    // Basic operations

    #[inline]
    pub fn max(self, other: I32x2) -> I32x2 {
        self.to_i32x4().max(other.to_i32x4()).xy()
    }

    #[inline]
    pub fn min(self, other: I32x2) -> I32x2 {
        self.to_i32x4().min(other.to_i32x4()).xy()
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: I32x2) -> I32x4 {
        unsafe { I32x4(simd_shuffle4!(self.0, other.0, [0, 1, 2, 3])) }
    }

    // Conversions

    /// Converts these packed integers to floats.
    #[inline]
    pub fn to_f32x2(self) -> F32x2 {
        unsafe { F32x2(simd_cast(self.0)) }
    }

    #[inline]
    pub fn to_i32x4(self) -> I32x4 {
        self.concat_xy_xy(I32x2::default())
    }
}

impl Default for I32x2 {
    #[inline]
    fn default() -> I32x2 {
        I32x2::splat(0)
    }
}

impl PartialEq for I32x2 {
    #[inline]
    fn eq(&self, other: &I32x2) -> bool {
        self.packed_eq(*other).all_true()
    }
}

impl Index<usize> for I32x2 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &i32 {
        unsafe {
            assert!(index < 2);
            let ptr = &self.0 as *const int32x2_t as *const i32;
            mem::transmute::<*const i32, &i32>(ptr.offset(index as isize))
        }
    }
}

impl IndexMut<usize> for I32x2 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        unsafe {
            assert!(index < 2);
            let ptr = &mut self.0 as *mut int32x2_t as *mut i32;
            mem::transmute::<*mut i32, &mut i32>(ptr.offset(index as isize))
        }
    }
}

impl Add<I32x2> for I32x2 {
    type Output = I32x2;
    #[inline]
    fn add(self, other: I32x2) -> I32x2 {
        unsafe { I32x2(simd_add(self.0, other.0)) }
    }
}

impl Sub<I32x2> for I32x2 {
    type Output = I32x2;
    #[inline]
    fn sub(self, other: I32x2) -> I32x2 {
        unsafe { I32x2(simd_sub(self.0, other.0)) }
    }
}

impl Mul<I32x2> for I32x2 {
    type Output = I32x2;
    #[inline]
    fn mul(self, other: I32x2) -> I32x2 {
        unsafe { I32x2(simd_mul(self.0, other.0)) }
    }
}

// Four 32-bit signed integers

#[derive(Clone, Copy, Debug)]
pub struct I32x4(pub int32x4_t);

impl I32x4 {
    #[inline]
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> I32x4 {
        unsafe { I32x4(mem::transmute([a, b, c, d])) }
    }

    #[inline]
    pub fn splat(x: i32) -> I32x4 {
        I32x4::new(x, x, x, x)
    }

    // Basic operations

    #[inline]
    pub fn max(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(simd_cast(simd_maximum_number_nsz(self.to_f32x4().0, other.to_f32x4().0))) }
    }

    #[inline]
    pub fn min(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(simd_cast(simd_minimum_number_nsz(self.to_f32x4().0, other.to_f32x4().0))) }
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: I32x4) -> U32x4 {
        unsafe { U32x4(simd_eq(self.0, other.0)) }
    }

    #[inline]
    pub fn packed_le(self, other: I32x4) -> U32x4 {
        unsafe { U32x4(simd_le(self.0, other.0)) }
    }

    #[inline]
    pub fn packed_lt(self, other: I32x4) -> U32x4 {
        unsafe { U32x4(simd_lt(self.0, other.0)) }
    }

    // Concatenations

    #[inline]
    pub fn concat_xy_xy(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(simd_shuffle4!(self.0, other.0, [0, 1, 4, 5])) }
    }

    #[inline]
    pub fn concat_zw_zw(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(simd_shuffle4!(self.0, other.0, [2, 3, 6, 7])) }
    }

    // Swizzle conversions

    #[inline]
    pub fn xy(self) -> I32x2 {
        unsafe { I32x2(simd_shuffle2!(self.0, self.0, [0, 1])) }
    }

    #[inline]
    pub fn yx(self) -> I32x2 {
        unsafe { I32x2(simd_shuffle2!(self.0, self.0, [1, 0])) }
    }

    #[inline]
    pub fn xw(self) -> I32x2 {
        unsafe { I32x2(simd_shuffle2!(self.0, self.0, [0, 3])) }
    }

    #[inline]
    pub fn zy(self) -> I32x2 {
        unsafe { I32x2(simd_shuffle2!(self.0, self.0, [2, 1])) }
    }

    #[inline]
    pub fn zw(self) -> I32x2 {
        unsafe { I32x2(simd_shuffle2!(self.0, self.0, [2, 3])) }
    }

    // Conversions

    /// Converts these packed integers to floats.
    #[inline]
    pub fn to_f32x4(self) -> F32x4 {
        unsafe { F32x4(simd_cast(self.0)) }
    }
}

impl Default for I32x4 {
    #[inline]
    fn default() -> I32x4 {
        I32x4::new(0, 0, 0, 0)
    }
}

impl Index<usize> for I32x4 {
    type Output = i32;
    #[inline]
    fn index(&self, index: usize) -> &i32 {
        unsafe {
            assert!(index < 4);
            let ptr = &self.0 as *const int32x4_t as *const i32;
            mem::transmute::<*const i32, &i32>(ptr.offset(index as isize))
        }
    }
}

impl IndexMut<usize> for I32x4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        unsafe {
            assert!(index < 4);
            let ptr = &mut self.0 as *mut int32x4_t as *mut i32;
            mem::transmute::<*mut i32, &mut i32>(ptr.offset(index as isize))
        }
    }
}

impl Add<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn add(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(simd_add(self.0, other.0)) }
    }
}

impl Sub<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn sub(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(simd_sub(self.0, other.0)) }
    }
}

impl Mul<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn mul(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(simd_mul(self.0, other.0)) }
    }
}

impl PartialEq for I32x4 {
    #[inline]
    fn eq(&self, other: &I32x4) -> bool {
        self.packed_eq(*other).all_true()
    }
}

impl BitAnd<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn bitand(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(simd_and(self.0, other.0)) }
    }
}

impl BitOr<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn bitor(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(simd_or(self.0, other.0)) }
    }
}

impl Shr<I32x4> for I32x4 {
    type Output = I32x4;
    #[inline]
    fn shr(self, other: I32x4) -> I32x4 {
        unsafe { I32x4(simd_shr(self.0, other.0)) }
    }
}

// Two 32-bit unsigned integers

#[derive(Clone, Copy)]
pub struct U32x2(pub uint32x2_t);

impl U32x2 {
    #[inline]
    pub fn new(x: u32, y: u32) -> U32x2 {
        unsafe { U32x2(mem::transmute([x, y])) }
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
        unsafe { aarch64::vminv_u32(self.0) == !0 }
    }

    /// Returns true if both booleans in this vector are false.
    ///
    /// The result is *undefined* if both values in this vector are not booleans. A boolean is a
    /// value with all bits set or all bits clear (i.e. !0 or 0).
    #[inline]
    pub fn all_false(&self) -> bool {
        unsafe { aarch64::vmaxv_u32(self.0) == 0 }
    }

    #[inline]
    pub fn to_i32x2(self) -> I32x2 {
        unsafe { I32x2(simd_cast(self.0)) }
    }
}

impl Index<usize> for U32x2 {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &u32 {
        unsafe {
            assert!(index < 2);
            let ptr = &self.0 as *const uint32x2_t as *const u32;
            mem::transmute::<*const u32, &u32>(ptr.offset(index as isize))
        }
    }
}

impl Not for U32x2 {
    type Output = U32x2;
    #[inline]
    fn not(self) -> U32x2 {
        // FIXME(pcwalton): Is there a better way to do this?
        unsafe { U32x2(simd_xor(self.0, U32x2::splat(!0).0)) }
    }
}

impl BitAnd<U32x2> for U32x2 {
    type Output = U32x2;
    #[inline]
    fn bitand(self, other: U32x2) -> U32x2 {
        unsafe { U32x2(simd_and(self.0, other.0)) }
    }
}

impl BitOr<U32x2> for U32x2 {
    type Output = U32x2;
    #[inline]
    fn bitor(self, other: U32x2) -> U32x2 {
        unsafe { U32x2(simd_or(self.0, other.0)) }
    }
}

// Four 32-bit unsigned integers

#[derive(Clone, Copy)]
pub struct U32x4(pub uint32x4_t);

impl U32x4 {
    #[inline]
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> U32x4 {
        unsafe { U32x4(mem::transmute([a, b, c, d])) }
    }

    #[inline]
    pub fn splat(x: u32) -> U32x4 {
        U32x4::new(x, x, x, x)
    }

    /// Returns true if all four booleans in this vector are true.
    ///
    /// The result is *undefined* if all four values in this vector are not booleans. A boolean is
    /// a value with all bits set or all bits clear (i.e. !0 or 0).
    #[inline]
    pub fn all_true(&self) -> bool {
        unsafe { aarch64::vminvq_u32(self.0) == !0 }
    }

    /// Returns true if all four booleans in this vector are false.
    ///
    /// The result is *undefined* if all four values in this vector are not booleans. A boolean is
    /// a value with all bits set or all bits clear (i.e. !0 or 0).
    #[inline]
    pub fn all_false(&self) -> bool {
        unsafe { aarch64::vmaxvq_u32(self.0) == 0 }
    }

    // Packed comparisons

    #[inline]
    pub fn packed_eq(self, other: U32x4) -> U32x4 {
        unsafe { U32x4(simd_eq(self.0, other.0)) }
    }
}

impl Debug for U32x4 {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}, {}, {}, {}>", self[0], self[1], self[2], self[3])
    }
}

impl Index<usize> for U32x4 {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &u32 {
        unsafe {
            assert!(index < 4);
            let ptr = &self.0 as *const uint32x4_t as *const u32;
            mem::transmute::<*const u32, &u32>(ptr.offset(index as isize))
        }
    }
}

impl PartialEq for U32x4 {
    #[inline]
    fn eq(&self, other: &U32x4) -> bool {
        self.packed_eq(*other).all_true()
    }
}

extern "C" {
    #[link_name = "llvm.fabs.v2f32"]
    fn fabs_v2f32(a: float32x2_t) -> float32x2_t;
    #[link_name = "llvm.floor.v2f32"]
    fn floor_v2f32(a: float32x2_t) -> float32x2_t;
    #[link_name = "llvm.ceil.v2f32"]
    fn ceil_v2f32(a: float32x2_t) -> float32x2_t;
    #[link_name = "llvm.round.v2f32"]
    fn round_v2f32(a: float32x2_t) -> float32x2_t;
    #[link_name = "llvm.sqrt.v2f32"]
    fn sqrt_v2f32(a: float32x2_t) -> float32x2_t;

    #[link_name = "llvm.fabs.v4f32"]
    fn fabs_v4f32(a: float32x4_t) -> float32x4_t;
    #[link_name = "llvm.floor.v4f32"]
    fn floor_v4f32(a: float32x4_t) -> float32x4_t;
    #[link_name = "llvm.ceil.v4f32"]
    fn ceil_v4f32(a: float32x4_t) -> float32x4_t;
    #[link_name = "llvm.round.v4f32"]
    fn round_v4f32(a: float32x4_t) -> float32x4_t;
    #[link_name = "llvm.sqrt.v4f32"]
    fn sqrt_v4f32(a: float32x4_t) -> float32x4_t;

    #[link_name = "llvm.aarch64.neon.frecpe.v2f32"]
    fn vrecpe_v2f32(a: float32x2_t) -> float32x2_t;

    #[link_name = "llvm.aarch64.neon.frecpe.v4f32"]
    fn vrecpe_v4f32(a: float32x4_t) -> float32x4_t;
}
