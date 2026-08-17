// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use core_foundation_sys::attributed_string::*;

use crate::base::TCFType;
use crate::string::{CFString, CFStringRef};
use core_foundation_sys::base::{kCFAllocatorDefault, CFIndex, CFRange};
use std::ptr::null;

declare_TCFType! {
    CFAttributedString, CFAttributedStringRef
}
impl_TCFType!(
    CFAttributedString,
    CFAttributedStringRef,
    CFAttributedStringGetTypeID
);

impl CFAttributedString {
    #[inline]
    pub fn new(string: &CFString) -> Self {
        unsafe {
            let astr_ref =
                CFAttributedStringCreate(kCFAllocatorDefault, string.as_concrete_TypeRef(), null());

            CFAttributedString::wrap_under_create_rule(astr_ref)
        }
    }

    #[inline]
    pub fn char_len(&self) -> CFIndex {
        unsafe { CFAttributedStringGetLength(self.0) }
    }
}

declare_TCFType! {
    CFMutableAttributedString, CFMutableAttributedStringRef
}
impl_TCFType!(
    CFMutableAttributedString,
    CFMutableAttributedStringRef,
    CFAttributedStringGetTypeID
);

impl CFMutableAttributedString {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let astr_ref = CFAttributedStringCreateMutable(kCFAllocatorDefault, 0);

            CFMutableAttributedString::wrap_under_create_rule(astr_ref)
        }
    }

    #[inline]
    pub fn char_len(&self) -> CFIndex {
        unsafe { CFAttributedStringGetLength(self.0) }
    }

    #[inline]
    pub fn replace_str(&mut self, string: &CFString, range: CFRange) {
        unsafe {
            CFAttributedStringReplaceString(self.0, range, string.as_concrete_TypeRef());
        }
    }

    #[inline]
    pub fn set_attribute<T: TCFType>(&mut self, range: CFRange, name: CFStringRef, value: &T) {
        unsafe {
            CFAttributedStringSetAttribute(self.0, range, name, value.as_CFTypeRef());
        }
    }
}

impl Default for CFMutableAttributedString {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributed_string_type_id_comparison() {
        // CFMutableAttributedString TypeID must be equal to CFAttributedString TypeID.
        // Compilation must not fail.
        assert_eq!(
            <CFAttributedString as TCFType>::type_id(),
            <CFMutableAttributedString as TCFType>::type_id()
        );
    }
}
