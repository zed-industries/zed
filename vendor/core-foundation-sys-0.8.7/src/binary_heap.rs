// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{Boolean, CFAllocatorRef, CFComparisonResult, CFIndex, CFTypeID};
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFBinaryHeap(c_void);

pub type CFBinaryHeapRef = *mut __CFBinaryHeap;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFBinaryHeapCompareContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn(info: *const c_void) -> *const c_void,
    pub release: extern "C" fn(info: *const c_void),
    pub copyDescription: extern "C" fn(info: *const c_void) -> CFStringRef,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFBinaryHeapCallBacks {
    pub version: CFIndex,
    pub retain: extern "C" fn(allocator: CFAllocatorRef, ptr: *const c_void) -> *const c_void,
    pub release: extern "C" fn(allocator: CFAllocatorRef, ptr: *const c_void),
    pub copyDescription: extern "C" fn(ptr: *const c_void) -> CFStringRef,
    pub compare: extern "C" fn(
        ptr1: *const c_void,
        ptr2: *const c_void,
        context: *mut c_void,
    ) -> CFComparisonResult,
}

pub type CFBinaryHeapApplierFunction = extern "C" fn(val: *const c_void, context: *const c_void);

extern "C" {
    /*
     * CFBinaryHeap.h
     */
    /* Predefined Callback Structures */
    pub static kCFStringBinaryHeapCallBacks: CFBinaryHeapCallBacks;

    /* CFBinaryHeap Miscellaneous Functions */
    pub fn CFBinaryHeapAddValue(heap: CFBinaryHeapRef, value: *const c_void);
    pub fn CFBinaryHeapApplyFunction(
        heap: CFBinaryHeapRef,
        applier: CFBinaryHeapApplierFunction,
        context: *mut c_void,
    );
    pub fn CFBinaryHeapContainsValue(heap: CFBinaryHeapRef, value: *const c_void) -> Boolean;
    pub fn CFBinaryHeapCreate(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        callBacks: *const CFBinaryHeapCallBacks,
        compareContext: *const CFBinaryHeapCompareContext,
    ) -> CFBinaryHeapRef;
    pub fn CFBinaryHeapCreateCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        heap: CFBinaryHeapRef,
    ) -> CFBinaryHeapRef;
    pub fn CFBinaryHeapGetCount(heap: CFBinaryHeapRef) -> CFIndex;
    pub fn CFBinaryHeapGetCountOfValue(heap: CFBinaryHeapRef, value: *const c_void) -> CFIndex;
    pub fn CFBinaryHeapGetMinimum(heap: CFBinaryHeapRef) -> *const c_void;
    pub fn CFBinaryHeapGetMinimumIfPresent(
        heap: CFBinaryHeapRef,
        value: *const *const c_void,
    ) -> Boolean;
    pub fn CFBinaryHeapGetTypeID() -> CFTypeID;
    pub fn CFBinaryHeapGetValues(heap: CFBinaryHeapRef, values: *const *const c_void);
    pub fn CFBinaryHeapRemoveAllValues(heap: CFBinaryHeapRef);
    pub fn CFBinaryHeapRemoveMinimumValue(heap: CFBinaryHeapRef);
}
