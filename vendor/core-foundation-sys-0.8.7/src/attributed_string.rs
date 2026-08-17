// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFRange, CFTypeID, CFTypeRef};
use crate::dictionary::CFDictionaryRef;
use crate::string::CFMutableStringRef;
use crate::string::CFStringRef;
use std::os::raw::c_void;

#[repr(C)]
pub struct __CFAttributedString(c_void);

pub type CFAttributedStringRef = *const __CFAttributedString;
pub type CFMutableAttributedStringRef = *mut __CFAttributedString;

extern "C" {
    /*
     * CFAttributedString.h
     */

    /* CFAttributedString */
    /* Creating a CFAttributedString */
    pub fn CFAttributedStringCreate(
        allocator: CFAllocatorRef,
        str: CFStringRef,
        attributes: CFDictionaryRef,
    ) -> CFAttributedStringRef;
    pub fn CFAttributedStringCreateCopy(
        alloc: CFAllocatorRef,
        aStr: CFAttributedStringRef,
    ) -> CFAttributedStringRef;
    pub fn CFAttributedStringCreateWithSubstring(
        alloc: CFAllocatorRef,
        aStr: CFAttributedStringRef,
        range: CFRange,
    ) -> CFAttributedStringRef;
    pub fn CFAttributedStringGetLength(astr: CFAttributedStringRef) -> CFIndex;
    pub fn CFAttributedStringGetString(aStr: CFAttributedStringRef) -> CFStringRef;

    /* Accessing Attributes */
    pub fn CFAttributedStringGetAttribute(
        aStr: CFAttributedStringRef,
        loc: CFIndex,
        attrName: CFStringRef,
        effectiveRange: *mut CFRange,
    ) -> CFTypeRef;
    pub fn CFAttributedStringGetAttributes(
        aStr: CFAttributedStringRef,
        loc: CFIndex,
        effectiveRange: *mut CFRange,
    ) -> CFDictionaryRef;
    pub fn CFAttributedStringGetAttributeAndLongestEffectiveRange(
        aStr: CFAttributedStringRef,
        loc: CFIndex,
        attrName: CFStringRef,
        inRange: CFRange,
        longestEffectiveRange: *mut CFRange,
    ) -> CFTypeRef;
    pub fn CFAttributedStringGetAttributesAndLongestEffectiveRange(
        aStr: CFAttributedStringRef,
        loc: CFIndex,
        inRange: CFRange,
        longestEffectiveRange: *mut CFRange,
    ) -> CFDictionaryRef;

    /* Getting Attributed String Properties */
    pub fn CFAttributedStringGetTypeID() -> CFTypeID;

    /* CFMutableAttributedString */
    /* Creating a CFMutableAttributedString */
    pub fn CFAttributedStringCreateMutable(
        allocator: CFAllocatorRef,
        max_length: CFIndex,
    ) -> CFMutableAttributedStringRef;
    pub fn CFAttributedStringCreateMutableCopy(
        allocator: CFAllocatorRef,
        max_length: CFIndex,
        astr: CFAttributedStringRef,
    ) -> CFMutableAttributedStringRef;

    /* Modifying a CFMutableAttributedString */
    pub fn CFAttributedStringBeginEditing(aStr: CFMutableAttributedStringRef);
    pub fn CFAttributedStringEndEditing(aStr: CFMutableAttributedStringRef);
    pub fn CFAttributedStringGetMutableString(
        aStr: CFMutableAttributedStringRef,
    ) -> CFMutableStringRef;
    pub fn CFAttributedStringRemoveAttribute(
        aStr: CFMutableAttributedStringRef,
        range: CFRange,
        attrName: CFStringRef,
    );
    pub fn CFAttributedStringReplaceString(
        aStr: CFMutableAttributedStringRef,
        range: CFRange,
        replacement: CFStringRef,
    );
    pub fn CFAttributedStringReplaceAttributedString(
        aStr: CFMutableAttributedStringRef,
        range: CFRange,
        replacement: CFAttributedStringRef,
    );
    pub fn CFAttributedStringSetAttribute(
        aStr: CFMutableAttributedStringRef,
        range: CFRange,
        attrName: CFStringRef,
        value: CFTypeRef,
    );
    pub fn CFAttributedStringSetAttributes(
        aStr: CFMutableAttributedStringRef,
        range: CFRange,
        replacement: CFDictionaryRef,
        clearOtherAttributes: Boolean,
    );
}
