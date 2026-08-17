// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A Boolean type.

pub use core_foundation_sys::number::{
    kCFBooleanFalse, kCFBooleanTrue, CFBooleanGetTypeID, CFBooleanRef,
};

use crate::base::TCFType;

declare_TCFType! {
    /// A Boolean type.
    ///
    /// FIXME(pcwalton): Should be a newtype struct, but that fails due to a Rust compiler bug.
    CFBoolean, CFBooleanRef
}
impl_TCFType!(CFBoolean, CFBooleanRef, CFBooleanGetTypeID);
impl_CFTypeDescription!(CFBoolean);

impl CFBoolean {
    pub fn true_value() -> CFBoolean {
        unsafe { TCFType::wrap_under_get_rule(kCFBooleanTrue) }
    }

    pub fn false_value() -> CFBoolean {
        unsafe { TCFType::wrap_under_get_rule(kCFBooleanFalse) }
    }
}

impl From<bool> for CFBoolean {
    fn from(value: bool) -> CFBoolean {
        if value {
            CFBoolean::true_value()
        } else {
            CFBoolean::false_value()
        }
    }
}

impl From<CFBoolean> for bool {
    fn from(value: CFBoolean) -> bool {
        value.0 == unsafe { kCFBooleanTrue }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_and_from_bool() {
        let b_false = CFBoolean::from(false);
        let b_true = CFBoolean::from(true);
        assert_ne!(b_false, b_true);
        assert_eq!(b_false, CFBoolean::false_value());
        assert_eq!(b_true, CFBoolean::true_value());
        assert!(!bool::from(b_false));
        assert!(bool::from(b_true));
    }
}
