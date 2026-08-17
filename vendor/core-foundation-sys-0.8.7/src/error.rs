// Copyright 2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{CFAllocatorRef, CFIndex, CFTypeID};
use crate::dictionary::CFDictionaryRef;
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFError(c_void);

pub type CFErrorRef = *mut __CFError;
pub type CFErrorDomain = CFStringRef;

extern "C" {
    /*
     * CFError.h
     */

    /* Error domains */
    pub static kCFErrorDomainPOSIX: CFStringRef;
    pub static kCFErrorDomainOSStatus: CFStringRef;
    pub static kCFErrorDomainMach: CFStringRef;
    pub static kCFErrorDomainCocoa: CFStringRef;

    /* Keys for the user info dictionary */
    pub static kCFErrorLocalizedDescriptionKey: CFStringRef;
    // pub static kCFErrorLocalizedFailureKey: CFStringRef; // macos(10.13)+
    pub static kCFErrorLocalizedFailureReasonKey: CFStringRef;
    pub static kCFErrorLocalizedRecoverySuggestionKey: CFStringRef;
    pub static kCFErrorDescriptionKey: CFStringRef;
    pub static kCFErrorUnderlyingErrorKey: CFStringRef;
    pub static kCFErrorURLKey: CFStringRef;
    pub static kCFErrorFilePathKey: CFStringRef;

    /* Creating a CFError */
    pub fn CFErrorCreate(
        allocator: CFAllocatorRef,
        domain: CFErrorDomain,
        code: CFIndex,
        userInfo: CFDictionaryRef,
    ) -> CFErrorRef;
    //pub fn CFErrorCreateWithUserInfoKeysAndValues

    /* Getting Information About an Error */
    pub fn CFErrorGetDomain(err: CFErrorRef) -> CFStringRef;
    pub fn CFErrorGetCode(err: CFErrorRef) -> CFIndex;
    pub fn CFErrorCopyUserInfo(err: CFErrorRef) -> CFDictionaryRef;
    pub fn CFErrorCopyDescription(err: CFErrorRef) -> CFStringRef;
    pub fn CFErrorCopyFailureReason(err: CFErrorRef) -> CFStringRef;
    pub fn CFErrorCopyRecoverySuggestion(err: CFErrorRef) -> CFStringRef;

    /* Getting the CFError Type ID */
    pub fn CFErrorGetTypeID() -> CFTypeID;
}
