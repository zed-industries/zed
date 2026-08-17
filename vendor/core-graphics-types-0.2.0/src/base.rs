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

use libc;

#[cfg(any(target_arch = "x86", target_arch = "arm", target_arch = "aarch64"))]
pub type boolean_t = libc::c_int;
#[cfg(target_arch = "x86_64")]
pub type boolean_t = libc::c_uint;

#[cfg(target_pointer_width = "64")]
pub type CGFloat = libc::c_double;
#[cfg(not(target_pointer_width = "64"))]
pub type CGFloat = libc::c_float;

pub type CGError = i32;

pub type CGGlyph = libc::c_ushort;

pub const kCGErrorSuccess: CGError = 0;
pub const kCGErrorFailure: CGError = 1000;
pub const kCGErrorIllegalArgument: CGError = 1001;
pub const kCGErrorInvalidConnection: CGError = 1002;
pub const kCGErrorInvalidContext: CGError = 1003;
pub const kCGErrorCannotComplete: CGError = 1004;
pub const kCGErrorNotImplemented: CGError = 1006;
pub const kCGErrorRangeCheck: CGError = 1007;
pub const kCGErrorTypeCheck: CGError = 1008;
pub const kCGErrorInvalidOperation: CGError = 1010;
pub const kCGErrorNoneAvailable: CGError = 1011;
