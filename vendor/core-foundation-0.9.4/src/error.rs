// Copyright 2016 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation errors.

pub use core_foundation_sys::error::*;

use std::error::Error;
use std::fmt;

use crate::base::{CFIndex, TCFType};
use crate::string::CFString;

declare_TCFType! {
    /// An error value.
    CFError, CFErrorRef
}
impl_TCFType!(CFError, CFErrorRef, CFErrorGetTypeID);

impl fmt::Debug for CFError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CFError")
            .field("domain", &self.domain())
            .field("code", &self.code())
            .field("description", &self.description())
            .finish()
    }
}

impl fmt::Display for CFError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl Error for CFError {
    fn description(&self) -> &str {
        "a Core Foundation error"
    }
}

impl CFError {
    /// Returns a string identifying the domain with which this error is
    /// associated.
    pub fn domain(&self) -> CFString {
        unsafe {
            let s = CFErrorGetDomain(self.0);
            CFString::wrap_under_get_rule(s)
        }
    }

    /// Returns the code identifying this type of error.
    pub fn code(&self) -> CFIndex {
        unsafe { CFErrorGetCode(self.0) }
    }

    /// Returns a human-presentable description of the error.
    pub fn description(&self) -> CFString {
        unsafe {
            let s = CFErrorCopyDescription(self.0);
            CFString::wrap_under_create_rule(s)
        }
    }
}
