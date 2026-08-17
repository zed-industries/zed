// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `sample` module defines the core audio sample trait and any non-primitive sample data types.

use std::fmt;

use crate::util::clamp::{clamp_f32, clamp_f64, clamp_i24, clamp_u24};

/// SampleFormat describes the data encoding for an audio sample.
#[derive(Copy, Clone, Debug)]
pub enum SampleFormat {
    /// Unsigned 8-bit integer.
    U8,
    /// Unsigned 16-bit integer.
    U16,
    /// Unsigned 24-bit integer.
    U24,
    /// Unsigned 32-bit integer.
    U32,
    /// Signed 8-bit integer.
    S8,
    /// Signed 16-bit integer.
    S16,
    /// Signed 24-bit integer.
    S24,
    /// Signed 32-bit integer.
    S32,
    /// Single precision (32-bit) floating point.
    F32,
    /// Double precision (64-bit) floating point.
    F64,
}

/// `Sample` provides a common interface for manipulating sample's regardless of the
/// underlying data type. Additionally, `Sample` provides information regarding the
/// format of underlying data types representing the sample when in memory, but also
/// when exported.
pub trait Sample:
    Copy
    + Clone
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + Default
    + PartialOrd
    + PartialEq
    + Sized
{
    /// A unique enum value representing the sample format. This constant may be used to dynamically
    /// choose how to process the sample at runtime.
    const FORMAT: SampleFormat;

    /// The effective number of bits of the valid (clamped) sample range. Quantifies the dynamic
    /// range of the sample format in bits.
    const EFF_BITS: u32;

    /// The mid-point value between the maximum and minimum sample value. If a sample is set to this
    /// value it is silent.
    const MID: Self;

    /// If the sample format does not use the full range of the underlying data type, returns the
    /// sample clamped to the valid range. Otherwise, returns the sample unchanged.
    fn clamped(self) -> Self;
}

/// An unsigned 24-bit integer sample with an internal unsigned 32-bit integer representation.
///
/// There are **no** guarantees the sample is within the valid range 24-bit range. Use the
/// [`Sample::clamped`] function to clamp the sample to the valid range.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct u24(pub u32);

/// A signed 24-bit integer sample with an internal signed 32-bit integer representation.
///
/// There are **no** guarantees the sample is within the valid range 24-bit range. Use the
/// [`Sample::clamped`] function to clamp the sample to the valid range.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct i24(pub i32);

impl Sample for u8 {
    const FORMAT: SampleFormat = SampleFormat::U8;
    const EFF_BITS: u32 = 8;
    const MID: u8 = 128;

    #[inline(always)]
    fn clamped(self) -> Self {
        self
    }
}

impl Sample for i8 {
    const FORMAT: SampleFormat = SampleFormat::S8;
    const EFF_BITS: u32 = 8;
    const MID: i8 = 0;

    #[inline(always)]
    fn clamped(self) -> Self {
        self
    }
}

impl Sample for u16 {
    const FORMAT: SampleFormat = SampleFormat::U16;
    const EFF_BITS: u32 = 16;
    const MID: u16 = 32_768;

    #[inline(always)]
    fn clamped(self) -> Self {
        self
    }
}

impl Sample for i16 {
    const FORMAT: SampleFormat = SampleFormat::S16;
    const EFF_BITS: u32 = 16;
    const MID: i16 = 0;

    #[inline(always)]
    fn clamped(self) -> Self {
        self
    }
}

impl Sample for u24 {
    const FORMAT: SampleFormat = SampleFormat::U24;
    const EFF_BITS: u32 = 24;
    const MID: u24 = u24(8_388_608);

    #[inline(always)]
    fn clamped(self) -> Self {
        u24(clamp_u24(self.0))
    }
}

impl Sample for i24 {
    const FORMAT: SampleFormat = SampleFormat::S24;
    const EFF_BITS: u32 = 24;
    const MID: i24 = i24(0);

    #[inline(always)]
    fn clamped(self) -> Self {
        i24(clamp_i24(self.0))
    }
}

impl Sample for u32 {
    const FORMAT: SampleFormat = SampleFormat::U32;
    const EFF_BITS: u32 = 32;
    const MID: u32 = 2_147_483_648;

    #[inline(always)]
    fn clamped(self) -> Self {
        self
    }
}

impl Sample for i32 {
    const FORMAT: SampleFormat = SampleFormat::S32;
    const EFF_BITS: u32 = 32;
    const MID: i32 = 0;

    #[inline(always)]
    fn clamped(self) -> Self {
        self
    }
}

impl Sample for f32 {
    const FORMAT: SampleFormat = SampleFormat::F32;
    const EFF_BITS: u32 = 24;
    const MID: f32 = 0.0;

    #[inline(always)]
    fn clamped(self) -> Self {
        clamp_f32(self)
    }
}

impl Sample for f64 {
    const FORMAT: SampleFormat = SampleFormat::F64;
    const EFF_BITS: u32 = 53;
    const MID: f64 = 0.0;

    #[inline(always)]
    fn clamped(self) -> Self {
        clamp_f64(self)
    }
}

// Helper macros

macro_rules! shl_impl {
    ($t:ident, $f:ty) => {
        impl core::ops::Shl<$f> for $t {
            type Output = $t;

            #[inline]
            fn shl(self, other: $f) -> $t {
                $t(self.0 << other)
            }
        }
    };
}

macro_rules! shr_impl {
    ($t:ident, $f:ty) => {
        impl core::ops::Shr<$f> for $t {
            type Output = $t;

            #[inline]
            fn shr(self, other: $f) -> $t {
                $t(self.0 >> other)
            }
        }
    };
}

macro_rules! impl_shifts {
    ($t:ident, $f:ty) => {
        shl_impl! { $t, $f }
        shr_impl! { $t, $f }
    };
}

// Implementation for i24

impl i24 {
    /// The largest value that can be represented by this integer type.
    pub const MAX: i24 = i24(8_388_607);
    /// The smallest value that can be represented by this integer type..
    pub const MIN: i24 = i24(-8_388_608);

    /// Get the underlying `i32` backing this `i24`.
    #[inline(always)]
    #[deprecated = "Superseded by `inner`."]
    pub fn into_i32(self) -> i32 {
        self.0
    }

    /// Get the underlying `i32` backing this `i24`.
    #[inline(always)]
    pub fn inner(self) -> i32 {
        self.0
    }

    /// Return the memory representation of this `i24` as a byte array in native byte order.
    #[inline]
    pub fn to_ne_bytes(self) -> [u8; 3] {
        let b = self.0.to_ne_bytes();

        if cfg!(target_endian = "little") {
            // In little-endian the MSB is the last byte. Drop it.
            [b[0], b[1], b[2]]
        }
        else {
            // In big-endian the MSB is the first byte. Drop it.
            [b[1], b[2], b[3]]
        }
    }
}

impl fmt::Display for i24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for i24 {
    fn from(val: i32) -> Self {
        i24(clamp_i24(val))
    }
}

impl From<i16> for i24 {
    fn from(val: i16) -> Self {
        i24(i32::from(val))
    }
}

impl From<i8> for i24 {
    fn from(val: i8) -> Self {
        i24(i32::from(val))
    }
}

impl core::ops::Add<i24> for i24 {
    type Output = i24;

    #[inline]
    fn add(self, other: Self) -> Self {
        i24(self.0 + other.0)
    }
}

impl core::ops::Sub<i24> for i24 {
    type Output = i24;

    #[inline]
    fn sub(self, other: Self) -> Self {
        i24(self.0 - other.0)
    }
}

impl core::ops::Mul<i24> for i24 {
    type Output = i24;

    #[inline]
    fn mul(self, other: Self) -> Self {
        i24(self.0 * other.0)
    }
}

impl core::ops::Div<i24> for i24 {
    type Output = i24;

    #[inline]
    fn div(self, other: Self) -> Self {
        i24(self.0 / other.0)
    }
}

impl core::ops::Not for i24 {
    type Output = i24;

    #[inline]
    fn not(self) -> Self {
        i24(!self.0)
    }
}

impl core::ops::Rem<i24> for i24 {
    type Output = i24;

    #[inline]
    fn rem(self, other: Self) -> Self {
        i24(self.0 % other.0)
    }
}

impl core::ops::Shl<i24> for i24 {
    type Output = i24;

    #[inline]
    fn shl(self, other: Self) -> Self {
        i24(self.0 << other.0)
    }
}

impl core::ops::Shr<i24> for i24 {
    type Output = i24;

    #[inline]
    fn shr(self, other: Self) -> Self {
        i24(self.0 >> other.0)
    }
}

impl_shifts! { i24, u8 }
impl_shifts! { i24, u16 }
impl_shifts! { i24, u32 }
impl_shifts! { i24, u64 }
impl_shifts! { i24, u128 }
impl_shifts! { i24, usize }

impl_shifts! { i24, i8 }
impl_shifts! { i24, i16 }
impl_shifts! { i24, i32 }
impl_shifts! { i24, i64 }
impl_shifts! { i24, i128 }
impl_shifts! { i24, isize }

impl core::ops::BitAnd<i24> for i24 {
    type Output = i24;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        i24(self.0 & other.0)
    }
}

impl core::ops::BitOr<i24> for i24 {
    type Output = i24;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        i24(self.0 | other.0)
    }
}

impl core::ops::BitXor<i24> for i24 {
    type Output = i24;

    #[inline]
    fn bitxor(self, other: Self) -> Self {
        i24(self.0 ^ other.0)
    }
}

// Implementation for u24

impl u24 {
    /// The largest value that can be represented by this integer type.
    pub const MAX: u24 = u24(16_777_215);
    /// The smallest value that can be represented by this integer type.
    pub const MIN: u24 = u24(0);

    /// Get the underlying `u32` backing this `u24`.
    #[inline(always)]
    #[deprecated = "Superseded by `inner`."]
    pub fn into_u32(self) -> u32 {
        self.0
    }

    /// Get the underlying `u32` backing this `u24`.
    #[inline(always)]
    pub fn inner(self) -> u32 {
        self.0
    }

    /// Return the memory representation of this `u24` as a byte array in native byte order.
    #[inline]
    pub fn to_ne_bytes(self) -> [u8; 3] {
        let b = self.0.to_ne_bytes();

        if cfg!(target_endian = "little") {
            // In little-endian the MSB is the last byte. Drop it.
            [b[0], b[1], b[2]]
        }
        else {
            // In big-endian the MSB is the first byte. Drop it.
            [b[1], b[2], b[3]]
        }
    }
}

impl fmt::Display for u24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for u24 {
    fn from(val: u32) -> Self {
        u24(clamp_u24(val))
    }
}

impl From<u16> for u24 {
    fn from(val: u16) -> Self {
        u24(u32::from(val))
    }
}

impl From<u8> for u24 {
    fn from(val: u8) -> Self {
        u24(u32::from(val))
    }
}

impl core::ops::Add<u24> for u24 {
    type Output = u24;

    #[inline]
    fn add(self, other: Self) -> Self {
        u24(self.0 + other.0)
    }
}

impl core::ops::Sub<u24> for u24 {
    type Output = u24;

    #[inline]
    fn sub(self, other: Self) -> Self {
        u24(self.0 - other.0)
    }
}

impl core::ops::Mul<u24> for u24 {
    type Output = u24;

    #[inline]
    fn mul(self, other: Self) -> Self {
        u24(self.0 * other.0)
    }
}

impl core::ops::Div<u24> for u24 {
    type Output = u24;

    #[inline]
    fn div(self, other: Self) -> Self {
        u24(self.0 / other.0)
    }
}

impl core::ops::Not for u24 {
    type Output = u24;

    #[inline]
    fn not(self) -> Self {
        u24(!self.0)
    }
}

impl core::ops::Rem<u24> for u24 {
    type Output = u24;

    #[inline]
    fn rem(self, other: Self) -> Self {
        u24(self.0 % other.0)
    }
}

impl core::ops::Shl<u24> for u24 {
    type Output = u24;

    #[inline]
    fn shl(self, other: Self) -> Self {
        u24(self.0 << other.0)
    }
}

impl core::ops::Shr<u24> for u24 {
    type Output = u24;

    #[inline]
    fn shr(self, other: Self) -> Self {
        u24(self.0 >> other.0)
    }
}

impl_shifts! { u24, u8 }
impl_shifts! { u24, u16 }
impl_shifts! { u24, u32 }
impl_shifts! { u24, u64 }
impl_shifts! { u24, u128 }
impl_shifts! { u24, usize }

impl_shifts! { u24, i8 }
impl_shifts! { u24, i16 }
impl_shifts! { u24, i32 }
impl_shifts! { u24, i64 }
impl_shifts! { u24, i128 }
impl_shifts! { u24, isize }

impl core::ops::BitAnd<u24> for u24 {
    type Output = u24;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        u24(self.0 & other.0)
    }
}

impl core::ops::BitOr<u24> for u24 {
    type Output = u24;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        u24(self.0 | other.0)
    }
}

impl core::ops::BitXor<u24> for u24 {
    type Output = u24;

    #[inline]
    fn bitxor(self, other: Self) -> Self {
        u24(self.0 ^ other.0)
    }
}
