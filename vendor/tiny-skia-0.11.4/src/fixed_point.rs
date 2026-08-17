// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Skia uses fixed points pretty chaotically, therefore we cannot use
// strongly typed wrappers. Which is unfortunate.

use tiny_skia_path::SaturateCast;

use crate::math::{bound, left_shift, left_shift64};

/// A 26.6 fixed point.
pub type FDot6 = i32;

/// A 24.8 fixed point.
pub type FDot8 = i32;

/// A 16.16 fixed point.
pub type FDot16 = i32;

pub mod fdot6 {
    use super::*;
    use core::convert::TryFrom;

    pub const ONE: FDot6 = 64;

    pub fn from_i32(n: i32) -> FDot6 {
        debug_assert!(n as i16 as i32 == n);
        n << 6
    }

    pub fn from_f32(n: f32) -> FDot6 {
        (n * 64.0) as i32
    }

    pub fn floor(n: FDot6) -> FDot6 {
        n >> 6
    }

    pub fn ceil(n: FDot6) -> FDot6 {
        (n + 63) >> 6
    }

    pub fn round(n: FDot6) -> FDot6 {
        (n + 32) >> 6
    }

    pub fn to_fdot16(n: FDot6) -> FDot16 {
        debug_assert!((left_shift(n, 10) >> 10) == n);
        left_shift(n, 10)
    }

    pub fn div(a: FDot6, b: FDot6) -> FDot16 {
        debug_assert_ne!(b, 0);

        if i16::try_from(a).is_ok() {
            left_shift(a, 16) / b
        } else {
            fdot16::div(a, b)
        }
    }

    pub fn can_convert_to_fdot16(n: FDot6) -> bool {
        let max_dot6 = i32::MAX >> (16 - 6);
        n.abs() <= max_dot6
    }

    pub fn small_scale(value: u8, dot6: FDot6) -> u8 {
        debug_assert!(dot6 as u32 <= 64);
        ((value as i32 * dot6) >> 6) as u8
    }
}

pub mod fdot8 {
    use super::*;

    // Extracted from SkScan_Antihair.cpp

    pub fn from_fdot16(x: FDot16) -> FDot8 {
        (x + 0x80) >> 8
    }
}

pub mod fdot16 {
    use super::*;

    pub const HALF: FDot16 = (1 << 16) / 2;
    pub const ONE: FDot16 = 1 << 16;

    // `from_f32` seems to lack a rounding step. For all fixed-point
    // values, this version is as accurate as possible for (fixed -> float -> fixed). Rounding reduces
    // accuracy if the intermediate floats are in the range that only holds integers (adding 0.5 to an
    // odd integer then snaps to nearest even). Using double for the rounding math gives maximum
    // accuracy for (float -> fixed -> float), but that's usually overkill.
    pub fn from_f32(x: f32) -> FDot16 {
        i32::saturate_from(x * ONE as f32)
    }

    pub fn floor_to_i32(x: FDot16) -> i32 {
        x >> 16
    }

    pub fn ceil_to_i32(x: FDot16) -> i32 {
        (x + ONE - 1) >> 16
    }

    pub fn round_to_i32(x: FDot16) -> i32 {
        (x + HALF) >> 16
    }

    // The divide may exceed 32 bits. Clamp to a signed 32 bit result.
    pub fn mul(a: FDot16, b: FDot16) -> FDot16 {
        ((i64::from(a) * i64::from(b)) >> 16) as FDot16
    }

    // The divide may exceed 32 bits. Clamp to a signed 32 bit result.
    pub fn div(numer: FDot6, denom: FDot6) -> FDot16 {
        let v = left_shift64(numer as i64, 16) / denom as i64;
        let n = bound(i32::MIN as i64, v, i32::MAX as i64);
        n as i32
    }

    pub fn fast_div(a: FDot6, b: FDot6) -> FDot16 {
        debug_assert!((left_shift(a, 16) >> 16) == a);
        debug_assert!(b != 0);
        left_shift(a, 16) / b
    }
}
