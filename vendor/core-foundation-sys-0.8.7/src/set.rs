// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
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

pub type CFSetApplierFunction = extern "C" fn(value: *const c_void, context: *const c_void);
pub type CFSetRetainCallBack =
    extern "C" fn(allocator: CFAllocatorRef, value: *const c_void) -> *const c_void;
pub type CFSetReleaseCallBack = extern "C" fn(allocator: CFAllocatorRef, value: *const c_void);
pub type CFSetCopyDescriptionCallBack = extern "C" fn(value: *const c_void) -> CFStringRef;
pub type CFSetEqualCallBack =
    extern "C" fn(value1: *const c_void, value2: *const c_void) -> Boolean;
pub type CFSetHashCallBack = extern "C" fn(value: *const c_void) -> CFHashCode;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CFSetCallBacks {
    pub version: CFIndex,
    pub retain: CFSetRetainCallBack,
    pub release: CFSetReleaseCallBack,
    pub copyDescription: CFSetCopyDescriptionCallBack,
    pub equal: CFSetEqualCallBack,
    pub hash: CFSetHashCallBack,
}

#[repr(C)]
pub struct __CFSet(c_void);

pub type CFSetRef = *const __CFSet;
pub type CFMutableSetRef = *mut __CFSet;

extern "C" {
    /*
     * CFSet.h
     */

    pub static kCFTypeSetCallBacks: CFSetCallBacks;
    pub static kCFCopyStringSetCallBacks: CFSetCallBacks;

    /* CFSet */
    /* Creating Sets */
    pub fn CFSetCreate(
        allocator: CFAllocatorRef,
        values: *const *const c_void,
        numValues: CFIndex,
        callBacks: *const CFSetCallBacks,
    ) -> CFSetRef;
    pub fn CFSetCreateCopy(allocator: CFAllocatorRef, theSet: CFSetRef) -> CFSetRef;

    /* Examining a Set */
    pub fn CFSetContainsValue(theSet: CFSetRef, value: *const c_void) -> Boolean;
    pub fn CFSetGetCount(theSet: CFSetRef) -> CFIndex;
    pub fn CFSetGetCountOfValue(theSet: CFSetRef, value: *const c_void) -> CFIndex;
    pub fn CFSetGetValue(theSet: CFSetRef, value: *const c_void) -> *const c_void;
    pub fn CFSetGetValueIfPresent(
        theSet: CFSetRef,
        candidate: *const c_void,
        value: *mut *const c_void,
    ) -> Boolean;
    pub fn CFSetGetValues(theSet: CFSetRef, values: *mut *const c_void);

    /* Applying a Function to Set Members */
    pub fn CFSetApplyFunction(
        theSet: CFSetRef,
        applier: CFSetApplierFunction,
        context: *const c_void,
    );

    /* Getting the CFSet Type ID */
    pub fn CFSetGetTypeID() -> CFTypeID;

    /* CFMutableSet */
    /* CFMutableSet Miscellaneous Functions */
    pub fn CFSetAddValue(theSet: CFMutableSetRef, value: *const c_void);
    pub fn CFSetCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        callBacks: *const CFSetCallBacks,
    ) -> CFMutableSetRef;
    pub fn CFSetCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        theSet: CFSetRef,
    ) -> CFMutableSetRef;
    pub fn CFSetRemoveAllValues(theSet: CFMutableSetRef);
    pub fn CFSetRemoveValue(theSet: CFMutableSetRef, value: *const c_void);
    pub fn CFSetReplaceValue(theSet: CFMutableSetRef, value: *const c_void);
    pub fn CFSetSetValue(theSet: CFMutableSetRef, value: *const c_void);
}
