// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::base::{CFAllocatorRef, CFIndex, CFOptionFlags, CFRange, CFTypeID};
use std::os::raw::c_void;

#[repr(C)]
pub struct __CFData(c_void);

pub type CFDataRef = *const __CFData;
pub type CFMutableDataRef = *mut __CFData;
pub type CFDataSearchFlags = CFOptionFlags;

// typedef CF_OPTIONS(CFOptionFlags, CFDataSearchFlags)
pub const kCFDataSearchBackwards: CFDataSearchFlags = 1usize << 0;
pub const kCFDataSearchAnchored: CFDataSearchFlags = 1usize << 1;

extern "C" {
    /*
     * CFData.h
     */

    /* CFData */
    /* Creating a CFData Object */
    pub fn CFDataCreate(allocator: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef;
    pub fn CFDataCreateCopy(allocator: CFAllocatorRef, theData: CFDataRef) -> CFDataRef;
    pub fn CFDataCreateWithBytesNoCopy(
        allocator: CFAllocatorRef,
        bytes: *const u8,
        length: CFIndex,
        bytesDeallocator: CFAllocatorRef,
    ) -> CFDataRef;

    /* Examining a CFData Object */
    pub fn CFDataGetBytePtr(theData: CFDataRef) -> *const u8;
    pub fn CFDataGetBytes(theData: CFDataRef, range: CFRange, buffer: *mut u8);
    pub fn CFDataGetLength(theData: CFDataRef) -> CFIndex;
    pub fn CFDataFind(
        theData: CFDataRef,
        dataToFind: CFDataRef,
        searchRange: CFRange,
        compareOptions: CFDataSearchFlags,
    ) -> CFRange;

    /* Getting the CFData Type ID */
    pub fn CFDataGetTypeID() -> CFTypeID;

    /* CFMutableData */
    /* Creating a Mutable Data Object */
    pub fn CFDataCreateMutable(allocator: CFAllocatorRef, capacity: CFIndex) -> CFMutableDataRef;
    pub fn CFDataCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        theData: CFDataRef,
    ) -> CFMutableDataRef;

    /* Accessing Data */
    pub fn CFDataGetMutableBytePtr(theData: CFMutableDataRef) -> *mut u8;

    /* Modifying a Mutable Data Object */
    pub fn CFDataAppendBytes(theData: CFMutableDataRef, bytes: *const u8, length: CFIndex);
    pub fn CFDataDeleteBytes(theData: CFMutableDataRef, range: CFRange);
    pub fn CFDataReplaceBytes(
        theData: CFMutableDataRef,
        range: CFRange,
        newBytes: *const u8,
        newLength: CFIndex,
    );
    pub fn CFDataIncreaseLength(theData: CFMutableDataRef, extraLength: CFIndex);
    pub fn CFDataSetLength(theData: CFMutableDataRef, length: CFIndex);
}
