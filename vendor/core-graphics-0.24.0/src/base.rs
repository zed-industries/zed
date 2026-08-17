// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// this file defines CGFloat, as well as stubbed data types.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub use core_graphics_types::base::*;

pub const kCGImageAlphaNone: u32 = 0;
pub const kCGImageAlphaPremultipliedLast: u32 = 1;
pub const kCGImageAlphaPremultipliedFirst: u32 = 2;
pub const kCGImageAlphaLast: u32 = 3;
pub const kCGImageAlphaFirst: u32 = 4;
pub const kCGImageAlphaNoneSkipLast: u32 = 5;
pub const kCGImageAlphaNoneSkipFirst: u32 = 6;

pub const kCGBitmapByteOrderDefault: u32 = 0 << 12;
pub const kCGBitmapByteOrder16Little: u32 = 1 << 12;
pub const kCGBitmapByteOrder32Little: u32 = 2 << 12;
pub const kCGBitmapByteOrder16Big: u32 = 3 << 12;
pub const kCGBitmapByteOrder32Big: u32 = 4 << 12;

pub const kCGRenderingIntentDefault: u32 = 0;
pub const kCGRenderingIntentAbsoluteColorimetric: u32 = 1;
pub const kCGRenderingIntentRelativeColorimetric: u32 = 2;
pub const kCGRenderingIntentPerceptual: u32 = 3;
pub const kCGRenderingIntentSaturation: u32 = 4;

#[cfg(target_endian = "big")]
pub const kCGBitmapByteOrder16Host: u32 = kCGBitmapByteOrder16Big;
#[cfg(target_endian = "big")]
pub const kCGBitmapByteOrder32Host: u32 = kCGBitmapByteOrder32Big;

#[cfg(target_endian = "little")]
pub const kCGBitmapByteOrder16Host: u32 = kCGBitmapByteOrder16Little;
#[cfg(target_endian = "little")]
pub const kCGBitmapByteOrder32Host: u32 = kCGBitmapByteOrder32Little;
