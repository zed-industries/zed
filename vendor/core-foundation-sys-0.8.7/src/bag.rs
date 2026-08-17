// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{Boolean, CFAllocatorRef, CFHashCode, CFIndex, CFTypeID};
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFBag(c_void);

pub type CFBagRef = *const __CFBag;
pub type CFMutableBagRef = *mut __CFBag;

pub type CFBagRetainCallBack =
    extern "C" fn(allocator: CFAllocatorRef, value: *const c_void) -> *const c_void;
pub type CFBagReleaseCallBack = extern "C" fn(allocator: CFAllocatorRef, value: *const c_void);
pub type CFBagCopyDescriptionCallBack = extern "C" fn(value: *const c_void) -> CFStringRef;
pub type CFBagEqualCallBack =
    extern "C" fn(value1: *const c_void, value2: *const c_void) -> Boolean;
pub type CFBagHashCallBack = extern "C" fn(value: *const c_void) -> CFHashCode;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFBagCallBacks {
    pub version: CFIndex,
    pub retain: CFBagRetainCallBack,
    pub release: CFBagReleaseCallBack,
    pub copyDescription: CFBagCopyDescriptionCallBack,
    pub equal: CFBagEqualCallBack,
    pub hash: CFBagHashCallBack,
}

pub type CFBagApplierFunction = extern "C" fn(value: *const c_void, context: *mut c_void);

extern "C" {
    /*
     * CFBag.h
     */
    /* Predefined Callback Structures */
    pub static kCFTypeBagCallBacks: CFBagCallBacks;
    pub static kCFCopyStringBagCallBacks: CFBagCallBacks;

    /* CFBag */
    /* Creating a Bag */
    pub fn CFBagCreate(
        allocator: CFAllocatorRef,
        values: *const *const c_void,
        numValues: CFIndex,
        callBacks: *const CFBagCallBacks,
    ) -> CFBagRef;
    pub fn CFBagCreateCopy(allocator: CFAllocatorRef, theBag: CFBagRef) -> CFBagRef;

    /* Examining a Bag */
    pub fn CFBagContainsValue(theBag: CFBagRef, value: *const c_void) -> Boolean;
    pub fn CFBagGetCount(theBag: CFBagRef) -> CFIndex;
    pub fn CFBagGetCountOfValue(theBag: CFBagRef, value: *const c_void) -> CFIndex;
    pub fn CFBagGetValue(theBag: CFBagRef, value: *const c_void) -> *const c_void;
    pub fn CFBagGetValueIfPresent(
        theBag: CFBagRef,
        candidate: *const c_void,
        value: *const *const c_void,
    ) -> Boolean;
    pub fn CFBagGetValues(theBag: CFBagRef, values: *const *const c_void);

    /* Applying a Function to the Contents of a Bag */
    pub fn CFBagApplyFunction(
        theBag: CFBagRef,
        applier: CFBagApplierFunction,
        context: *mut c_void,
    );

    /* Getting the CFBag Type ID */
    pub fn CFBagGetTypeID() -> CFTypeID;

    /* CFMutableBag */
    /* Creating a Mutable Bag */
    pub fn CFBagCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        callBacks: *const CFBagCallBacks,
    ) -> CFMutableBagRef;
    pub fn CFBagCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        theBag: CFBagRef,
    ) -> CFMutableBagRef;

    /* Modifying a Mutable Bag */
    pub fn CFBagAddValue(theBag: CFMutableBagRef, value: *const c_void);
    pub fn CFBagRemoveAllValues(theBag: CFMutableBagRef);
    pub fn CFBagRemoveValue(theBag: CFMutableBagRef, value: *const c_void);
    pub fn CFBagReplaceValue(theBag: CFMutableBagRef, value: *const c_void);
    pub fn CFBagSetValue(theBag: CFMutableBagRef, value: *const c_void);
}
