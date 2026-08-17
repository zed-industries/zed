// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::{c_char, c_void};

use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, CFRange, CFTypeID};
use crate::date::{CFAbsoluteTime, CFTimeInterval};
use crate::locale::{CFCalendarIdentifier, CFLocaleRef};
use crate::timezone::CFTimeZoneRef;

#[repr(C)]
pub struct __CFCalendar(c_void);
pub type CFCalendarRef = *mut __CFCalendar;

pub type CFCalendarUnit = CFOptionFlags;
pub const kCFCalendarUnitEra: CFCalendarUnit = 1 << 1;
pub const kCFCalendarUnitYear: CFCalendarUnit = 1 << 2;
pub const kCFCalendarUnitMonth: CFCalendarUnit = 1 << 3;
pub const kCFCalendarUnitDay: CFCalendarUnit = 1 << 4;
pub const kCFCalendarUnitHour: CFCalendarUnit = 1 << 5;
pub const kCFCalendarUnitMinute: CFCalendarUnit = 1 << 6;
pub const kCFCalendarUnitSecond: CFCalendarUnit = 1 << 7;
pub const kCFCalendarUnitWeek: CFCalendarUnit = 1 << 8; // deprecated since macos 10.10
pub const kCFCalendarUnitWeekday: CFCalendarUnit = 1 << 9;
pub const kCFCalendarUnitWeekdayOrdinal: CFCalendarUnit = 1 << 10;
pub const kCFCalendarUnitQuarter: CFCalendarUnit = 1 << 11;
pub const kCFCalendarUnitWeekOfMonth: CFCalendarUnit = 1 << 12;
pub const kCFCalendarUnitWeekOfYear: CFCalendarUnit = 1 << 13;
pub const kCFCalendarUnitYearForWeekOfYear: CFCalendarUnit = 1 << 14;

pub const kCFCalendarComponentsWrap: CFOptionFlags = 1 << 0;

extern "C" {
    /*
     * CFCalendar.h
     */

    /* Creating a Calendar */
    pub fn CFCalendarCopyCurrent() -> CFCalendarRef;
    pub fn CFCalendarCreateWithIdentifier(
        allocator: CFAllocatorRef,
        identifier: CFCalendarIdentifier,
    ) -> CFCalendarRef;

    /* Calendrical Calculations */
    pub fn CFCalendarAddComponents(
        identifier: CFCalendarIdentifier,
        /* inout */ at: *mut CFAbsoluteTime,
        options: CFOptionFlags,
        componentDesc: *const char,
        ...
    ) -> Boolean;
    pub fn CFCalendarComposeAbsoluteTime(
        identifier: CFCalendarIdentifier,
        /* out */ at: *mut CFAbsoluteTime,
        componentDesc: *const c_char,
        ...
    ) -> Boolean;
    pub fn CFCalendarDecomposeAbsoluteTime(
        identifier: CFCalendarIdentifier,
        at: CFAbsoluteTime,
        componentDesc: *const c_char,
        ...
    ) -> Boolean;
    pub fn CFCalendarGetComponentDifference(
        identifier: CFCalendarIdentifier,
        startingAT: CFAbsoluteTime,
        resultAT: CFAbsoluteTime,
        options: CFOptionFlags,
        componentDesc: *const c_char,
        ...
    ) -> Boolean;

    /* Getting Ranges of Units */
    pub fn CFCalendarGetRangeOfUnit(
        identifier: CFCalendarIdentifier,
        smallerUnit: CFCalendarUnit,
        biggerUnit: CFCalendarUnit,
        at: CFAbsoluteTime,
    ) -> CFRange;
    pub fn CFCalendarGetOrdinalityOfUnit(
        identifier: CFCalendarIdentifier,
        smallerUnit: CFCalendarUnit,
        biggerUnit: CFCalendarUnit,
        at: CFAbsoluteTime,
    ) -> CFIndex;
    pub fn CFCalendarGetTimeRangeOfUnit(
        identifier: CFCalendarIdentifier,
        unit: CFCalendarUnit,
        at: CFAbsoluteTime,
        startp: *mut CFAbsoluteTime,
        tip: *mut CFTimeInterval,
    ) -> Boolean;
    pub fn CFCalendarGetMaximumRangeOfUnit(
        identifier: CFCalendarIdentifier,
        unit: CFCalendarUnit,
    ) -> CFRange;
    pub fn CFCalendarGetMinimumRangeOfUnit(
        identifier: CFCalendarIdentifier,
        unit: CFCalendarUnit,
    ) -> CFRange;

    /* Getting and Setting the Time Zone */
    pub fn CFCalendarCopyTimeZone(identifier: CFCalendarIdentifier) -> CFTimeZoneRef;
    pub fn CFCalendarSetTimeZone(identifier: CFCalendarIdentifier, tz: CFTimeZoneRef);

    /* Getting the Identifier */
    pub fn CFCalendarGetIdentifier(identifier: CFCalendarIdentifier) -> CFCalendarIdentifier;

    /* Getting and Setting the Locale */
    pub fn CFCalendarCopyLocale(identifier: CFCalendarIdentifier) -> CFLocaleRef;
    pub fn CFCalendarSetLocale(identifier: CFCalendarIdentifier, locale: CFLocaleRef);

    /* Getting and Setting Day Information */
    pub fn CFCalendarGetFirstWeekday(identifier: CFCalendarIdentifier) -> CFIndex;
    pub fn CFCalendarSetFirstWeekday(identifier: CFCalendarIdentifier, wkdy: CFIndex);
    pub fn CFCalendarGetMinimumDaysInFirstWeek(identifier: CFCalendarIdentifier) -> CFIndex;
    pub fn CFCalendarSetMinimumDaysInFirstWeek(identifier: CFCalendarIdentifier, mwd: CFIndex);

    /* Getting the Type ID */
    pub fn CFCalendarGetTypeID() -> CFTypeID;
}
