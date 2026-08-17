// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{Boolean, CFAllocatorRef, CFComparisonResult, CFIndex, CFTypeID};

#[repr(C)]
pub struct __CFBoolean(c_void);

pub type CFBooleanRef = *const __CFBoolean;

pub type CFNumberType = u32;

// members of enum CFNumberType
pub const kCFNumberSInt8Type: CFNumberType = 1;
pub const kCFNumberSInt16Type: CFNumberType = 2;
pub const kCFNumberSInt32Type: CFNumberType = 3;
pub const kCFNumberSInt64Type: CFNumberType = 4;
pub const kCFNumberFloat32Type: CFNumberType = 5;
pub const kCFNumberFloat64Type: CFNumberType = 6;
pub const kCFNumberCharType: CFNumberType = 7;
pub const kCFNumberShortType: CFNumberType = 8;
pub const kCFNumberIntType: CFNumberType = 9;
pub const kCFNumberLongType: CFNumberType = 10;
pub const kCFNumberLongLongType: CFNumberType = 11;
pub const kCFNumberFloatType: CFNumberType = 12;
pub const kCFNumberDoubleType: CFNumberType = 13;
pub const kCFNumberCFIndexType: CFNumberType = 14;
pub const kCFNumberNSIntegerType: CFNumberType = 15;
pub const kCFNumberCGFloatType: CFNumberType = 16;
pub const kCFNumberMaxType: CFNumberType = 16;

// This is an enum due to zero-sized types warnings.
// For more details see https://github.com/rust-lang/rust/issues/27303
pub enum __CFNumber {}

pub type CFNumberRef = *const __CFNumber;

extern "C" {
    /*
     * CFNumber.h
     */
    pub static kCFBooleanTrue: CFBooleanRef;
    pub static kCFBooleanFalse: CFBooleanRef;
    pub static kCFNumberPositiveInfinity: CFNumberRef;
    pub static kCFNumberNegativeInfinity: CFNumberRef;
    pub static kCFNumberNaN: CFNumberRef;

    /* Creating a Number */
    pub fn CFNumberCreate(
        allocator: CFAllocatorRef,
        theType: CFNumberType,
        valuePtr: *const c_void,
    ) -> CFNumberRef;

    /* Getting Information About Numbers */
    pub fn CFNumberGetByteSize(number: CFNumberRef) -> CFIndex;
    pub fn CFNumberGetType(number: CFNumberRef) -> CFNumberType;
    pub fn CFNumberGetValue(
        number: CFNumberRef,
        theType: CFNumberType,
        valuePtr: *mut c_void,
    ) -> bool;
    pub fn CFNumberIsFloatType(number: CFNumberRef) -> Boolean;

    /* Comparing Numbers */
    pub fn CFNumberCompare(
        date: CFNumberRef,
        other: CFNumberRef,
        context: *mut c_void,
    ) -> CFComparisonResult;

    /* Getting the CFNumber Type ID */
    pub fn CFNumberGetTypeID() -> CFTypeID;

    pub fn CFBooleanGetValue(boolean: CFBooleanRef) -> bool;
    pub fn CFBooleanGetTypeID() -> CFTypeID;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_for_type_id_should_be_backwards_compatible() {
        let type_id = kCFNumberFloat32Type;
        // this is the old style of matching for static variables
        match type_id {
            vf64 if vf64 == kCFNumberFloat32Type => assert!(true),
            _ => panic!("should not happen"),
        };

        // this is new new style of matching for consts
        match type_id {
            kCFNumberFloat32Type => assert!(true),
            _ => panic!("should not happen"),
        };
    }
}
