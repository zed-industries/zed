// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::LengthU32;

// Perfectly safe.
pub const LENGTH_U32_ONE: LengthU32 = unsafe { LengthU32::new_unchecked(1) };

pub fn left_shift(value: i32, shift: i32) -> i32 {
    ((value as u32) << shift) as i32
}

pub fn left_shift64(value: i64, shift: i32) -> i64 {
    ((value as u64) << shift) as i64
}

pub fn bound<T: Ord + Copy>(min: T, value: T, max: T) -> T {
    max.min(value).max(min)
}
