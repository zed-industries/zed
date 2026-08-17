// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFRange, CFTypeID, UInt32, UInt8};

#[repr(C)]
pub struct __CFBitVector(c_void);

pub type CFBitVectorRef = *const __CFBitVector;
pub type CFMutableBitVectorRef = *mut __CFBitVector;
pub type CFBit = UInt32;

extern "C" {
    /*
     * CFBitVector.h
     */

    /* CFBitVector */
    /* Creating a Bit Vector */
    pub fn CFBitVectorCreate(
        allocator: CFAllocatorRef,
        bytes: *const UInt8,
        numBits: CFIndex,
    ) -> CFBitVectorRef;
    pub fn CFBitVectorCreateCopy(allocator: CFAllocatorRef, bv: CFBitVectorRef) -> CFBitVectorRef;

    /* Getting Information About a Bit Vector */
    pub fn CFBitVectorContainsBit(bv: CFBitVectorRef, range: CFRange, value: CFBit) -> Boolean;
    pub fn CFBitVectorGetBitAtIndex(bv: CFBitVectorRef, idx: CFIndex) -> CFBit;
    pub fn CFBitVectorGetBits(bv: CFBitVectorRef, range: CFRange, bytes: *mut UInt8);
    pub fn CFBitVectorGetCount(bv: CFBitVectorRef) -> CFIndex;
    pub fn CFBitVectorGetCountOfBit(bv: CFBitVectorRef, range: CFRange, value: CFBit) -> CFIndex;
    pub fn CFBitVectorGetFirstIndexOfBit(
        bv: CFBitVectorRef,
        range: CFRange,
        value: CFBit,
    ) -> CFIndex;
    pub fn CFBitVectorGetLastIndexOfBit(
        bv: CFBitVectorRef,
        range: CFRange,
        value: CFBit,
    ) -> CFIndex;

    /* Getting the CFBitVector Type ID */
    pub fn CFBitVectorGetTypeID() -> CFTypeID;

    /* CFMutableBitVector */
    /* Creating a CFMutableBitVector Object */
    pub fn CFBitVectorCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
    ) -> CFMutableBitVectorRef;
    pub fn CFBitVectorCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        bv: CFBitVectorRef,
    ) -> CFMutableBitVectorRef;

    /* Modifying a Bit Vector */
    pub fn CFBitVectorFlipBitAtIndex(bv: CFMutableBitVectorRef, idx: CFIndex);
    pub fn CFBitVectorFlipBits(bv: CFMutableBitVectorRef, range: CFRange);
    pub fn CFBitVectorSetAllBits(bv: CFMutableBitVectorRef, value: CFBit);
    pub fn CFBitVectorSetBitAtIndex(bv: CFMutableBitVectorRef, idx: CFIndex, value: CFBit);
    pub fn CFBitVectorSetBits(bv: CFMutableBitVectorRef, range: CFRange, value: CFBit);
    pub fn CFBitVectorSetCount(bv: CFMutableBitVectorRef, count: CFIndex);
}
