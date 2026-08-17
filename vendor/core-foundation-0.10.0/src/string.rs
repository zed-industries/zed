// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Immutable strings.

pub use core_foundation_sys::string::*;

use crate::base::{CFIndexConvertible, TCFType};

use core_foundation_sys::base::{kCFAllocatorDefault, kCFAllocatorNull};
use core_foundation_sys::base::{Boolean, CFIndex, CFRange};
use std::borrow::Cow;
use std::ffi::CStr;
use std::fmt;
use std::ptr;
use std::str::{self, FromStr};

declare_TCFType! {
    /// An immutable string in one of a variety of encodings.
    CFString, CFStringRef
}
impl_TCFType!(CFString, CFStringRef, CFStringGetTypeID);

impl FromStr for CFString {
    type Err = ();

    /// See also [`CFString::new()`] for a variant of this which does not return a `Result`.
    #[inline]
    fn from_str(string: &str) -> Result<CFString, ()> {
        Ok(CFString::new(string))
    }
}

impl<'a> From<&'a str> for CFString {
    #[inline]
    fn from(string: &'a str) -> CFString {
        CFString::new(string)
    }
}

impl<'a> From<&'a CFString> for Cow<'a, str> {
    fn from(cf_str: &'a CFString) -> Cow<'a, str> {
        unsafe {
            // Do this without allocating if we can get away with it
            let c_string = CFStringGetCStringPtr(cf_str.0, kCFStringEncodingUTF8);
            if !c_string.is_null() {
                let c_str = CStr::from_ptr(c_string);
                Cow::Borrowed(str::from_utf8_unchecked(c_str.to_bytes()))
            } else {
                let char_len = cf_str.char_len();

                // First, ask how big the buffer ought to be.
                let mut bytes_required: CFIndex = 0;
                CFStringGetBytes(
                    cf_str.0,
                    CFRange {
                        location: 0,
                        length: char_len,
                    },
                    kCFStringEncodingUTF8,
                    0,
                    false as Boolean,
                    ptr::null_mut(),
                    0,
                    &mut bytes_required,
                );

                // Then, allocate the buffer and actually copy.
                let mut buffer = vec![b'\x00'; bytes_required as usize];

                let mut bytes_used: CFIndex = 0;
                let chars_written = CFStringGetBytes(
                    cf_str.0,
                    CFRange {
                        location: 0,
                        length: char_len,
                    },
                    kCFStringEncodingUTF8,
                    0,
                    false as Boolean,
                    buffer.as_mut_ptr(),
                    buffer.len().to_CFIndex(),
                    &mut bytes_used,
                );
                assert_eq!(chars_written, char_len);

                // This is dangerous; we over-allocate and null-terminate the string (during
                // initialization).
                assert_eq!(bytes_used, buffer.len().to_CFIndex());
                Cow::Owned(String::from_utf8_unchecked(buffer))
            }
        }
    }
}

impl fmt::Display for CFString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&Cow::from(self))
    }
}

impl fmt::Debug for CFString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl CFString {
    /// Creates a new `CFString` instance from a Rust string.
    #[inline]
    pub fn new(string: &str) -> CFString {
        unsafe {
            let string_ref = CFStringCreateWithBytes(
                kCFAllocatorDefault,
                string.as_ptr(),
                string.len().to_CFIndex(),
                kCFStringEncodingUTF8,
                false as Boolean,
            );
            CFString::wrap_under_create_rule(string_ref)
        }
    }

    /// Like `CFString::new`, but references a string that can be used as a backing store
    /// by virtue of being statically allocated.
    #[inline]
    pub fn from_static_string(string: &'static str) -> CFString {
        unsafe {
            let string_ref = CFStringCreateWithBytesNoCopy(
                kCFAllocatorDefault,
                string.as_ptr(),
                string.len().to_CFIndex(),
                kCFStringEncodingUTF8,
                false as Boolean,
                kCFAllocatorNull,
            );
            TCFType::wrap_under_create_rule(string_ref)
        }
    }

    /// Returns the number of characters in the string.
    #[inline]
    pub fn char_len(&self) -> CFIndex {
        unsafe { CFStringGetLength(self.0) }
    }
}

impl<'a> PartialEq<&'a str> for CFString {
    fn eq(&self, other: &&str) -> bool {
        unsafe {
            let temp = CFStringCreateWithBytesNoCopy(
                kCFAllocatorDefault,
                other.as_ptr(),
                other.len().to_CFIndex(),
                kCFStringEncodingUTF8,
                false as Boolean,
                kCFAllocatorNull,
            );
            self.eq(&CFString::wrap_under_create_rule(temp))
        }
    }
}

impl<'a> PartialEq<CFString> for &'a str {
    #[inline]
    fn eq(&self, other: &CFString) -> bool {
        other.eq(self)
    }
}

impl PartialEq<CFString> for String {
    #[inline]
    fn eq(&self, other: &CFString) -> bool {
        other.eq(&self.as_str())
    }
}

impl PartialEq<String> for CFString {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.eq(&other.as_str())
    }
}

#[test]
fn str_cmp() {
    let cfstr = CFString::new("hello");
    assert_eq!("hello", cfstr);
    assert_eq!(cfstr, "hello");
    assert_ne!(cfstr, "wrong");
    assert_ne!("wrong", cfstr);
    let hello = String::from("hello");
    assert_eq!(hello, cfstr);
    assert_eq!(cfstr, hello);
}

#[test]
fn string_and_back() {
    let original = "The quick brown fox jumped over the slow lazy dog.";
    let cfstr = CFString::from_static_string(original);
    let converted = cfstr.to_string();
    assert_eq!(converted, original);
}
