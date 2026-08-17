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

pub type CFDictionaryApplierFunction =
    extern "C" fn(key: *const c_void, value: *const c_void, context: *mut c_void);

pub type CFDictionaryRetainCallBack =
    extern "C" fn(allocator: CFAllocatorRef, value: *const c_void) -> *const c_void;
pub type CFDictionaryReleaseCallBack =
    extern "C" fn(allocator: CFAllocatorRef, value: *const c_void);
pub type CFDictionaryCopyDescriptionCallBack = extern "C" fn(value: *const c_void) -> CFStringRef;
pub type CFDictionaryEqualCallBack =
    extern "C" fn(value1: *const c_void, value2: *const c_void) -> Boolean;
pub type CFDictionaryHashCallBack = extern "C" fn(value: *const c_void) -> CFHashCode;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CFDictionaryKeyCallBacks {
    pub version: CFIndex,
    pub retain: CFDictionaryRetainCallBack,
    pub release: CFDictionaryReleaseCallBack,
    pub copyDescription: CFDictionaryCopyDescriptionCallBack,
    pub equal: CFDictionaryEqualCallBack,
    pub hash: CFDictionaryHashCallBack,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CFDictionaryValueCallBacks {
    pub version: CFIndex,
    pub retain: CFDictionaryRetainCallBack,
    pub release: CFDictionaryReleaseCallBack,
    pub copyDescription: CFDictionaryCopyDescriptionCallBack,
    pub equal: CFDictionaryEqualCallBack,
}

#[repr(C)]
pub struct __CFDictionary(c_void);

pub type CFDictionaryRef = *const __CFDictionary;
pub type CFMutableDictionaryRef = *mut __CFDictionary;

extern "C" {
    /*
     * CFDictionary.h
     */

    pub static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    pub static kCFCopyStringDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    pub static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;

    /* CFDictionary */
    /* Creating a dictionary */
    pub fn CFDictionaryCreate(
        allocator: CFAllocatorRef,
        keys: *const *const c_void,
        values: *const *const c_void,
        numValues: CFIndex,
        keyCallBacks: *const CFDictionaryKeyCallBacks,
        valueCallBacks: *const CFDictionaryValueCallBacks,
    ) -> CFDictionaryRef;
    pub fn CFDictionaryCreateCopy(
        allocator: CFAllocatorRef,
        theDict: CFDictionaryRef,
    ) -> CFDictionaryRef;

    /* Examining a dictionary */
    pub fn CFDictionaryContainsKey(theDict: CFDictionaryRef, key: *const c_void) -> Boolean;
    pub fn CFDictionaryContainsValue(theDict: CFDictionaryRef, value: *const c_void) -> Boolean;
    pub fn CFDictionaryGetCount(theDict: CFDictionaryRef) -> CFIndex;
    pub fn CFDictionaryGetCountOfKey(theDict: CFDictionaryRef, key: *const c_void) -> CFIndex;
    pub fn CFDictionaryGetCountOfValue(heDict: CFDictionaryRef, value: *const c_void) -> CFIndex;
    pub fn CFDictionaryGetKeysAndValues(
        theDict: CFDictionaryRef,
        keys: *mut *const c_void,
        values: *mut *const c_void,
    );
    pub fn CFDictionaryGetValue(theDict: CFDictionaryRef, key: *const c_void) -> *const c_void;
    pub fn CFDictionaryGetValueIfPresent(
        theDict: CFDictionaryRef,
        key: *const c_void,
        value: *mut *const c_void,
    ) -> Boolean;

    /* Applying a function to a dictionary */
    pub fn CFDictionaryApplyFunction(
        theDict: CFDictionaryRef,
        applier: CFDictionaryApplierFunction,
        context: *mut c_void,
    );

    /* Getting the CFDictionary type ID */
    pub fn CFDictionaryGetTypeID() -> CFTypeID;

    /* CFMutableDictionary */
    /* Creating a Mutable Dictionary */
    pub fn CFDictionaryCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        keyCallbacks: *const CFDictionaryKeyCallBacks,
        valueCallbacks: *const CFDictionaryValueCallBacks,
    ) -> CFMutableDictionaryRef;
    pub fn CFDictionaryCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        theDict: CFDictionaryRef,
    ) -> CFMutableDictionaryRef;

    /* Modifying a Dictionary */
    pub fn CFDictionaryAddValue(
        theDict: CFMutableDictionaryRef,
        key: *const c_void,
        value: *const c_void,
    );
    pub fn CFDictionaryRemoveAllValues(theDict: CFMutableDictionaryRef);
    pub fn CFDictionaryRemoveValue(theDict: CFMutableDictionaryRef, key: *const c_void);
    pub fn CFDictionaryReplaceValue(
        theDict: CFMutableDictionaryRef,
        key: *const c_void,
        value: *const c_void,
    );
    pub fn CFDictionarySetValue(
        theDict: CFMutableDictionaryRef,
        key: *const c_void,
        value: *const c_void,
    );

}
