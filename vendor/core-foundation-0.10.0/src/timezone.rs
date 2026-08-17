// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation time zone objects.

use core_foundation_sys::base::kCFAllocatorDefault;
pub use core_foundation_sys::timezone::*;

use crate::base::TCFType;
use crate::date::{CFDate, CFTimeInterval};
use crate::string::CFString;

declare_TCFType! {
    /// A time zone.
    CFTimeZone, CFTimeZoneRef
}
impl_TCFType!(CFTimeZone, CFTimeZoneRef, CFTimeZoneGetTypeID);
impl_CFTypeDescription!(CFTimeZone);

impl Default for CFTimeZone {
    fn default() -> CFTimeZone {
        unsafe {
            let tz_ref = CFTimeZoneCopyDefault();
            TCFType::wrap_under_create_rule(tz_ref)
        }
    }
}

impl CFTimeZone {
    #[inline]
    pub fn new(interval: CFTimeInterval) -> CFTimeZone {
        unsafe {
            let tz_ref = CFTimeZoneCreateWithTimeIntervalFromGMT(kCFAllocatorDefault, interval);
            TCFType::wrap_under_create_rule(tz_ref)
        }
    }

    #[inline]
    pub fn system() -> CFTimeZone {
        unsafe {
            let tz_ref = CFTimeZoneCopySystem();
            TCFType::wrap_under_create_rule(tz_ref)
        }
    }

    pub fn seconds_from_gmt(&self, date: CFDate) -> CFTimeInterval {
        unsafe { CFTimeZoneGetSecondsFromGMT(self.0, date.abs_time()) }
    }

    /// The timezone database ID that identifies the time zone. E.g. `"America/Los_Angeles" `or
    /// `"Europe/Paris"`.
    pub fn name(&self) -> CFString {
        unsafe { CFString::wrap_under_get_rule(CFTimeZoneGetName(self.0)) }
    }
}

#[cfg(test)]
mod test {
    use super::CFTimeZone;

    #[test]
    fn timezone_comparison() {
        let system = CFTimeZone::system();
        let default = CFTimeZone::default();
        assert_eq!(system, default);
    }
}
