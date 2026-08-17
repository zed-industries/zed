// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This module was written from scratch, therefore there is no Google copyright.

// f32x16, i32x16 and u32x16 are implemented as [Tx8; 2] and not as [T; 16].
// This way we still can use some SIMD.
//
// We doesn't use #[inline] that much in this module.
// The compiler will inline most of the methods automatically.
// The only exception is U16x16, were we have to force inlining,
// otherwise the performance will be horrible.

#![allow(non_camel_case_types)]

mod f32x16_t;
mod f32x4_t;
mod f32x8_t;
mod i32x4_t;
mod i32x8_t;
mod u16x16_t;
mod u32x4_t;
mod u32x8_t;

pub use f32x16_t::f32x16;
pub use f32x4_t::f32x4;
pub use f32x8_t::f32x8;
pub use i32x4_t::i32x4;
pub use i32x8_t::i32x8;
pub use tiny_skia_path::f32x2;
pub use u16x16_t::u16x16;
pub use u32x4_t::u32x4;
pub use u32x8_t::u32x8;

#[allow(dead_code)]
#[inline]
pub fn generic_bit_blend<T>(mask: T, y: T, n: T) -> T
where
    T: Copy + core::ops::BitXor<Output = T> + core::ops::BitAnd<Output = T>,
{
    n ^ ((n ^ y) & mask)
}

/// A faster and more forgiving f32 min/max implementation.
///
/// Unlike std one, we do not care about NaN.
#[allow(dead_code)]
pub trait FasterMinMax {
    fn faster_min(self, rhs: f32) -> f32;
    fn faster_max(self, rhs: f32) -> f32;
}

#[allow(dead_code)]
impl FasterMinMax for f32 {
    fn faster_min(self, rhs: f32) -> f32 {
        if rhs < self {
            rhs
        } else {
            self
        }
    }

    fn faster_max(self, rhs: f32) -> f32 {
        if self < rhs {
            rhs
        } else {
            self
        }
    }
}
