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

#[cfg(feature = "with-chrono")]
use chrono::{FixedOffset, NaiveDateTime};

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

    #[cfg(feature = "with-chrono")]
    pub fn offset_at_date(&self, date: NaiveDateTime) -> FixedOffset {
        let date = CFDate::from_naive_utc(date);
        FixedOffset::east(self.seconds_from_gmt(date) as i32)
    }

    #[cfg(feature = "with-chrono")]
    pub fn from_offset(offset: FixedOffset) -> CFTimeZone {
        CFTimeZone::new(offset.local_minus_utc() as f64)
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

    #[cfg(feature = "with-chrono")]
    use chrono::{FixedOffset, NaiveDateTime};

    #[test]
    fn timezone_comparison() {
        let system = CFTimeZone::system();
        let default = CFTimeZone::default();
        assert_eq!(system, default);
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn timezone_chrono_conversion() {
        let offset = FixedOffset::west(28800);
        let tz = CFTimeZone::from_offset(offset);
        let converted = tz.offset_at_date(NaiveDateTime::from_timestamp(0, 0));
        assert_eq!(offset, converted);
    }
}
