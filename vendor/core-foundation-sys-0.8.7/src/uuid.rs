// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{CFAllocatorRef, CFTypeID};
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFUUID(c_void);

pub type CFUUIDRef = *const __CFUUID;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CFUUIDBytes {
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
    pub byte3: u8,
    pub byte4: u8,
    pub byte5: u8,
    pub byte6: u8,
    pub byte7: u8,
    pub byte8: u8,
    pub byte9: u8,
    pub byte10: u8,
    pub byte11: u8,
    pub byte12: u8,
    pub byte13: u8,
    pub byte14: u8,
    pub byte15: u8,
}

extern "C" {
    /*
     * CFUUID.h
     */

    /* Creating CFUUID Objects */
    pub fn CFUUIDCreate(allocator: CFAllocatorRef) -> CFUUIDRef;
    pub fn CFUUIDCreateFromString(alloc: CFAllocatorRef, uuidStr: CFStringRef) -> CFUUIDRef;
    pub fn CFUUIDCreateFromUUIDBytes(allocator: CFAllocatorRef, bytes: CFUUIDBytes) -> CFUUIDRef;
    pub fn CFUUIDCreateWithBytes(
        alloc: CFAllocatorRef,
        byte0: u8,
        byte1: u8,
        byte2: u8,
        byte3: u8,
        byte4: u8,
        byte5: u8,
        byte6: u8,
        byte7: u8,
        byte8: u8,
        byte9: u8,
        byte10: u8,
        byte11: u8,
        byte12: u8,
        byte13: u8,
        byte14: u8,
        byte15: u8,
    ) -> CFUUIDRef;

    /* Getting Information About CFUUID Objects */
    pub fn CFUUIDCreateString(allocator: CFAllocatorRef, uid: CFUUIDRef) -> CFStringRef;
    pub fn CFUUIDGetConstantUUIDWithBytes(
        alloc: CFAllocatorRef,
        byte0: u8,
        byte1: u8,
        byte2: u8,
        byte3: u8,
        byte4: u8,
        byte5: u8,
        byte6: u8,
        byte7: u8,
        byte8: u8,
        byte9: u8,
        byte10: u8,
        byte11: u8,
        byte12: u8,
        byte13: u8,
        byte14: u8,
        byte15: u8,
    ) -> CFUUIDRef;
    pub fn CFUUIDGetUUIDBytes(uuid: CFUUIDRef) -> CFUUIDBytes;

    /* Getting the CFUUID Type Identifier */
    pub fn CFUUIDGetTypeID() -> CFTypeID;
}
