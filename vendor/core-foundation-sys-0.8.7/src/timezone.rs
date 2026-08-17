// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::array::CFArrayRef;
use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFTypeID};
use crate::data::CFDataRef;
use crate::date::{CFAbsoluteTime, CFTimeInterval};
use crate::dictionary::CFDictionaryRef;
use crate::locale::CFLocaleRef;
use crate::notification_center::CFNotificationName;
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFTimeZone(c_void);

pub type CFTimeZoneRef = *const __CFTimeZone;
pub type CFTimeZoneNameStyle = CFIndex;

/* Constants to specify styles for time zone names */
pub const kCFTimeZoneNameStyleStandard: CFTimeZoneNameStyle = 0;
pub const kCFTimeZoneNameStyleShortStandard: CFTimeZoneNameStyle = 1;
pub const kCFTimeZoneNameStyleDaylightSaving: CFTimeZoneNameStyle = 2;
pub const kCFTimeZoneNameStyleShortDaylightSaving: CFTimeZoneNameStyle = 3;
pub const kCFTimeZoneNameStyleGeneric: CFTimeZoneNameStyle = 4;
pub const kCFTimeZoneNameStyleShortGeneric: CFTimeZoneNameStyle = 5;

extern "C" {
    /*
     * CFTimeZone.h
     */

    pub static kCFTimeZoneSystemTimeZoneDidChangeNotification: CFNotificationName;

    /* Creating a Time Zone */
    pub fn CFTimeZoneCreate(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        data: CFDataRef,
    ) -> CFTimeZoneRef;
    pub fn CFTimeZoneCreateWithName(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        tryAbbrev: Boolean,
    ) -> CFTimeZoneRef;
    pub fn CFTimeZoneCreateWithTimeIntervalFromGMT(
        allocator: CFAllocatorRef,
        interval: CFTimeInterval,
    ) -> CFTimeZoneRef;

    /* System and Default Time Zones and Information */
    pub fn CFTimeZoneCopyAbbreviationDictionary() -> CFDictionaryRef;
    pub fn CFTimeZoneCopyAbbreviation(tz: CFTimeZoneRef, at: CFAbsoluteTime) -> CFStringRef;
    pub fn CFTimeZoneCopyDefault() -> CFTimeZoneRef;
    pub fn CFTimeZoneCopySystem() -> CFTimeZoneRef;
    pub fn CFTimeZoneSetDefault(tz: CFTimeZoneRef);
    pub fn CFTimeZoneCopyKnownNames() -> CFArrayRef;
    pub fn CFTimeZoneResetSystem();
    pub fn CFTimeZoneSetAbbreviationDictionary(dict: CFDictionaryRef);

    /* Getting Information About Time Zones */
    pub fn CFTimeZoneGetName(tz: CFTimeZoneRef) -> CFStringRef;
    pub fn CFTimeZoneCopyLocalizedName(
        tz: CFTimeZoneRef,
        style: CFTimeZoneNameStyle,
        locale: CFLocaleRef,
    ) -> CFStringRef;
    pub fn CFTimeZoneGetSecondsFromGMT(tz: CFTimeZoneRef, time: CFAbsoluteTime) -> CFTimeInterval;
    pub fn CFTimeZoneGetData(tz: CFTimeZoneRef) -> CFDataRef;

    /* Getting Daylight Savings Time Information */
    pub fn CFTimeZoneIsDaylightSavingTime(tz: CFTimeZoneRef, at: CFAbsoluteTime) -> Boolean;
    pub fn CFTimeZoneGetDaylightSavingTimeOffset(
        tz: CFTimeZoneRef,
        at: CFAbsoluteTime,
    ) -> CFTimeInterval;
    pub fn CFTimeZoneGetNextDaylightSavingTimeTransition(
        tz: CFTimeZoneRef,
        at: CFAbsoluteTime,
    ) -> CFAbsoluteTime;

    /* Getting the CFTimeZone Type ID */
    pub fn CFTimeZoneGetTypeID() -> CFTypeID;
}
