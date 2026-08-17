// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation date objects.

use core_foundation_sys::base::kCFAllocatorDefault;
pub use core_foundation_sys::date::*;

use crate::base::TCFType;

declare_TCFType! {
    /// A date.
    CFDate, CFDateRef
}
impl_TCFType!(CFDate, CFDateRef, CFDateGetTypeID);
impl_CFTypeDescription!(CFDate);
impl_CFComparison!(CFDate, CFDateCompare);

impl CFDate {
    #[inline]
    pub fn new(time: CFAbsoluteTime) -> CFDate {
        unsafe {
            let date_ref = CFDateCreate(kCFAllocatorDefault, time);
            TCFType::wrap_under_create_rule(date_ref)
        }
    }

    #[inline]
    pub fn now() -> CFDate {
        CFDate::new(unsafe { CFAbsoluteTimeGetCurrent() })
    }

    #[inline]
    pub fn abs_time(&self) -> CFAbsoluteTime {
        unsafe { CFDateGetAbsoluteTime(self.0) }
    }
}

#[cfg(test)]
mod test {
    use super::CFDate;
    use std::cmp::Ordering;

    #[test]
    fn date_comparison() {
        let now = CFDate::now();
        let past = CFDate::new(now.abs_time() - 1.0);
        assert_eq!(now.cmp(&past), Ordering::Greater);
        assert_eq!(now.cmp(&now), Ordering::Equal);
        assert_eq!(past.cmp(&now), Ordering::Less);
    }

    #[test]
    fn date_equality() {
        let now = CFDate::now();
        let same_time = CFDate::new(now.abs_time());
        assert_eq!(now, same_time);
    }
}
