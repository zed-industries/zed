// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// An unsigned 16.16-bit fixed point value.
#[derive(Copy, Clone, Debug, Default)]
pub struct FpU16(u32);

impl FpU16 {
    pub fn new(val: u16) -> Self {
        Self(u32::from(val) << 16)
    }

    pub fn parse_raw(val: u32) -> Self {
        Self(val)
    }
}

impl From<FpU16> for f64 {
    fn from(fp: FpU16) -> Self {
        f64::from(fp.0) / f64::from(1u32 << 16)
    }
}

/// An unsigned 8.8-bit fixed point value.
#[derive(Copy, Clone, Debug, Default)]
pub struct FpU8(u16);

impl FpU8 {
    pub fn new(val: u8) -> Self {
        Self(u16::from(val) << 8)
    }

    pub fn parse_raw(val: u16) -> Self {
        Self(val)
    }
}

impl From<FpU8> for f64 {
    fn from(fp: FpU8) -> Self {
        f64::from(fp.0) / f64::from(1u16 << 8)
    }
}

impl From<FpU8> for f32 {
    fn from(fp: FpU8) -> Self {
        f32::from(fp.0) / f32::from(1u16 << 8)
    }
}

/// An unsigned 8.8-bit fixed point value.
#[derive(Copy, Clone, Debug, Default)]
pub struct FpI8(i16);

impl FpI8 {
    pub fn new(val: i8) -> Self {
        Self(i16::from(val) * 0x100)
    }

    pub fn parse_raw(val: i16) -> Self {
        Self(val)
    }
}

impl From<FpI8> for f64 {
    fn from(fp: FpI8) -> Self {
        f64::from(fp.0) / f64::from(1u16 << 8)
    }
}

impl From<FpI8> for f32 {
    fn from(fp: FpI8) -> Self {
        f32::from(fp.0) / f32::from(1u16 << 8)
    }
}
