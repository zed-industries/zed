// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Immutable numbers.

use core_foundation_sys::base::kCFAllocatorDefault;
pub use core_foundation_sys::number::*;
use std::os::raw::c_void;

use crate::base::TCFType;

declare_TCFType! {
    /// An immutable numeric value.
    CFNumber, CFNumberRef
}
impl_TCFType!(CFNumber, CFNumberRef, CFNumberGetTypeID);
impl_CFTypeDescription!(CFNumber);
impl_CFComparison!(CFNumber, CFNumberCompare);

impl CFNumber {
    #[inline]
    pub fn to_i32(&self) -> Option<i32> {
        unsafe {
            let mut value: i32 = 0;
            let ok = CFNumberGetValue(
                self.0,
                kCFNumberSInt32Type,
                &mut value as *mut i32 as *mut c_void,
            );
            if ok {
                Some(value)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn to_i64(&self) -> Option<i64> {
        unsafe {
            let mut value: i64 = 0;
            let ok = CFNumberGetValue(
                self.0,
                kCFNumberSInt64Type,
                &mut value as *mut i64 as *mut c_void,
            );
            if ok {
                Some(value)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn to_f32(&self) -> Option<f32> {
        unsafe {
            let mut value: f32 = 0.0;
            let ok = CFNumberGetValue(
                self.0,
                kCFNumberFloat32Type,
                &mut value as *mut f32 as *mut c_void,
            );
            if ok {
                Some(value)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn to_f64(&self) -> Option<f64> {
        unsafe {
            let mut value: f64 = 0.0;
            let ok = CFNumberGetValue(
                self.0,
                kCFNumberFloat64Type,
                &mut value as *mut f64 as *mut c_void,
            );
            if ok {
                Some(value)
            } else {
                None
            }
        }
    }
}

impl From<i32> for CFNumber {
    #[inline]
    fn from(value: i32) -> Self {
        unsafe {
            let number_ref = CFNumberCreate(
                kCFAllocatorDefault,
                kCFNumberSInt32Type,
                &value as *const i32 as *const c_void,
            );
            TCFType::wrap_under_create_rule(number_ref)
        }
    }
}

impl From<i64> for CFNumber {
    #[inline]
    fn from(value: i64) -> Self {
        unsafe {
            let number_ref = CFNumberCreate(
                kCFAllocatorDefault,
                kCFNumberSInt64Type,
                &value as *const i64 as *const c_void,
            );
            TCFType::wrap_under_create_rule(number_ref)
        }
    }
}

impl From<f32> for CFNumber {
    #[inline]
    fn from(value: f32) -> Self {
        unsafe {
            let number_ref = CFNumberCreate(
                kCFAllocatorDefault,
                kCFNumberFloat32Type,
                &value as *const f32 as *const c_void,
            );
            TCFType::wrap_under_create_rule(number_ref)
        }
    }
}

impl From<f64> for CFNumber {
    #[inline]
    fn from(value: f64) -> Self {
        unsafe {
            let number_ref = CFNumberCreate(
                kCFAllocatorDefault,
                kCFNumberFloat64Type,
                &value as *const f64 as *const c_void,
            );
            TCFType::wrap_under_create_rule(number_ref)
        }
    }
}
