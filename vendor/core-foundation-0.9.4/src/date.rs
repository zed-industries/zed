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

#[cfg(feature = "with-chrono")]
use chrono::NaiveDateTime;

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

    #[cfg(feature = "with-chrono")]
    pub fn naive_utc(&self) -> NaiveDateTime {
        let ts = unsafe { self.abs_time() + kCFAbsoluteTimeIntervalSince1970 };
        let (secs, nanos) = if ts.is_sign_positive() {
            (ts.trunc() as i64, ts.fract())
        } else {
            // nanoseconds can't be negative in NaiveDateTime
            (ts.trunc() as i64 - 1, 1.0 - ts.fract().abs())
        };
        NaiveDateTime::from_timestamp(secs, (nanos * 1e9).floor() as u32)
    }

    #[cfg(feature = "with-chrono")]
    pub fn from_naive_utc(time: NaiveDateTime) -> CFDate {
        let secs = time.timestamp();
        let nanos = time.timestamp_subsec_nanos();
        let ts = unsafe { secs as f64 + (nanos as f64 / 1e9) - kCFAbsoluteTimeIntervalSince1970 };
        CFDate::new(ts)
    }
}

#[cfg(test)]
mod test {
    use super::CFDate;
    use std::cmp::Ordering;

    #[cfg(feature = "with-chrono")]
    use chrono::NaiveDateTime;

    #[cfg(feature = "with-chrono")]
    fn approx_eq(a: f64, b: f64) -> bool {
        use std::f64;

        let same_sign = a.is_sign_positive() == b.is_sign_positive();
        let equal = ((a - b).abs() / f64::min(a.abs() + b.abs(), f64::MAX)) < f64::EPSILON;
        same_sign && equal
    }

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

    #[test]
    #[cfg(feature = "with-chrono")]
    fn date_chrono_conversion_positive() {
        let date = CFDate::now();
        let datetime = date.naive_utc();
        let converted = CFDate::from_naive_utc(datetime);
        assert!(approx_eq(date.abs_time(), converted.abs_time()));
    }

    #[test]
    #[cfg(feature = "with-chrono")]
    fn date_chrono_conversion_negative() {
        use super::kCFAbsoluteTimeIntervalSince1970;

        let ts = unsafe { kCFAbsoluteTimeIntervalSince1970 - 420.0 };
        let date = CFDate::new(ts);
        let datetime: NaiveDateTime = date.naive_utc();
        let converted = CFDate::from_naive_utc(datetime);
        assert!(approx_eq(date.abs_time(), converted.abs_time()));
    }
}
