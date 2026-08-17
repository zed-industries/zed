// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{CFAllocatorRef, CFComparisonResult, CFTypeID};

#[repr(C)]
pub struct __CFDate(c_void);

pub type CFDateRef = *const __CFDate;

pub type CFTimeInterval = f64;
pub type CFAbsoluteTime = CFTimeInterval;

extern "C" {
    pub static kCFAbsoluteTimeIntervalSince1904: CFTimeInterval;
    pub static kCFAbsoluteTimeIntervalSince1970: CFTimeInterval;

    pub fn CFAbsoluteTimeGetCurrent() -> CFAbsoluteTime;

    pub fn CFDateCreate(allocator: CFAllocatorRef, at: CFAbsoluteTime) -> CFDateRef;
    pub fn CFDateGetAbsoluteTime(date: CFDateRef) -> CFAbsoluteTime;
    pub fn CFDateGetTimeIntervalSinceDate(date: CFDateRef, other: CFDateRef) -> CFTimeInterval;
    pub fn CFDateCompare(
        date: CFDateRef,
        other: CFDateRef,
        context: *mut c_void,
    ) -> CFComparisonResult;

    pub fn CFDateGetTypeID() -> CFTypeID;
}
