// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, CFRange, CFTypeID, CFTypeRef};
use crate::date::{CFAbsoluteTime, CFDateRef};
use crate::locale::CFLocaleRef;
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFDateFormatter(c_void);
pub type CFDateFormatterRef = *mut __CFDateFormatter;

pub type CFDateFormatterKey = CFStringRef;
pub type CFDateFormatterStyle = CFIndex;
pub type CFISO8601DateFormatOptions = CFOptionFlags;

/* Date Formatter Styles */
pub const kCFDateFormatterNoStyle: CFDateFormatterStyle = 0;
pub const kCFDateFormatterShortStyle: CFDateFormatterStyle = 1;
pub const kCFDateFormatterMediumStyle: CFDateFormatterStyle = 2;
pub const kCFDateFormatterLongStyle: CFDateFormatterStyle = 3;
pub const kCFDateFormatterFullStyle: CFDateFormatterStyle = 4;

//pub const kCFISO8601DateFormatWithYear: CFISO8601DateFormatOptions = 1 << 0; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithMonth: CFISO8601DateFormatOptions = 1 << 1; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithWeekOfYear: CFISO8601DateFormatOptions = 1 << 2; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithDay: CFISO8601DateFormatOptions = 1 << 4; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithTime: CFISO8601DateFormatOptions = 1 << 5; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithTimeZone: CFISO8601DateFormatOptions = 1 << 6; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithSpaceBetweenDateAndTime: CFISO8601DateFormatOptions = 1 << 7; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithDashSeparatorInDate: CFISO8601DateFormatOptions = 1 << 8; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithColonSeparatorInTime: CFISO8601DateFormatOptions = 1 << 9; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithColonSeparatorInTimeZone: CFISO8601DateFormatOptions = 1 << 10; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithFractionalSeconds: CFISO8601DateFormatOptions = 1 << 11; // macosx(10.13)+
//pub const kCFISO8601DateFormatWithFullDate: CFISO8601DateFormatOptions = kCFISO8601DateFormatWithYear | kCFISO8601DateFormatWithMonth | kCFISO8601DateFormatWithDay | kCFISO8601DateFormatWithDashSeparatorInDate; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithFullTime: CFISO8601DateFormatOptions = kCFISO8601DateFormatWithTime | kCFISO8601DateFormatWithColonSeparatorInTime | kCFISO8601DateFormatWithTimeZone | kCFISO8601DateFormatWithColonSeparatorInTimeZone; // macosx(10.12)+
//pub const kCFISO8601DateFormatWithInternetDateTime: CFISO8601DateFormatOptions = kCFISO8601DateFormatWithFullDate | kCFISO8601DateFormatWithFullTime; // macosx(10.12)+

extern "C" {
    /*
     * CFDateFormatter.h
     */

    /* Date Formatter Property Keys */
    // The values for these keys are all CFType objects.
    // The specific types for each key are specified above.
    pub static kCFDateFormatterIsLenient: CFDateFormatterKey; // CFBoolean
    pub static kCFDateFormatterTimeZone: CFDateFormatterKey; // CFTimeZone
    pub static kCFDateFormatterCalendarName: CFDateFormatterKey; // CFString
    pub static kCFDateFormatterDefaultFormat: CFDateFormatterKey; // CFString
    pub static kCFDateFormatterTwoDigitStartDate: CFDateFormatterKey; // CFDate
    pub static kCFDateFormatterDefaultDate: CFDateFormatterKey; // CFDate
    pub static kCFDateFormatterCalendar: CFDateFormatterKey; // CFCalendar
    pub static kCFDateFormatterEraSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterMonthSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterShortMonthSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterWeekdaySymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterShortWeekdaySymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterAMSymbol: CFDateFormatterKey; // CFString
    pub static kCFDateFormatterPMSymbol: CFDateFormatterKey; // CFString
    pub static kCFDateFormatterLongEraSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterVeryShortMonthSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterStandaloneMonthSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterShortStandaloneMonthSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterVeryShortStandaloneMonthSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterVeryShortWeekdaySymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterStandaloneWeekdaySymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterShortStandaloneWeekdaySymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterVeryShortStandaloneWeekdaySymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterQuarterSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterShortQuarterSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterStandaloneQuarterSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterShortStandaloneQuarterSymbols: CFDateFormatterKey; // CFArray of CFString
    pub static kCFDateFormatterGregorianStartDate: CFDateFormatterKey; // CFDate
    pub static kCFDateFormatterDoesRelativeDateFormattingKey: CFDateFormatterKey; // CFBoolean

    /* Creating a Date Formatter */
    pub fn CFDateFormatterCreate(
        allocator: CFAllocatorRef,
        locale: CFLocaleRef,
        dateStyle: CFDateFormatterStyle,
        timeStyle: CFDateFormatterStyle,
    ) -> CFDateFormatterRef;

    /* Configuring a Date Formatter */
    pub fn CFDateFormatterSetFormat(formatter: CFDateFormatterRef, formatString: CFStringRef);
    pub fn CFDateFormatterSetProperty(
        formatter: CFDateFormatterRef,
        key: CFStringRef,
        value: CFTypeRef,
    );

    /* Parsing Strings */
    pub fn CFDateFormatterCreateDateFromString(
        allocator: CFAllocatorRef,
        formatter: CFDateFormatterRef,
        string: CFStringRef,
        rangep: *mut CFRange,
    ) -> CFDateRef;
    pub fn CFDateFormatterGetAbsoluteTimeFromString(
        formatter: CFDateFormatterRef,
        string: CFStringRef,
        rangep: *mut CFRange,
        atp: *mut CFAbsoluteTime,
    ) -> Boolean;

    /* Creating Strings From Data */
    pub fn CFDateFormatterCreateStringWithAbsoluteTime(
        allocator: CFAllocatorRef,
        formatter: CFDateFormatterRef,
        at: CFAbsoluteTime,
    ) -> CFStringRef;
    pub fn CFDateFormatterCreateStringWithDate(
        allocator: CFAllocatorRef,
        formatter: CFDateFormatterRef,
        date: CFDateRef,
    ) -> CFStringRef;
    pub fn CFDateFormatterCreateDateFormatFromTemplate(
        allocator: CFAllocatorRef,
        tmplate: CFStringRef,
        options: CFOptionFlags,
        locale: CFLocaleRef,
    ) -> CFStringRef;

    /* Getting Information About a Date Formatter */
    pub fn CFDateFormatterCopyProperty(
        formatter: CFDateFormatterRef,
        key: CFDateFormatterKey,
    ) -> CFTypeRef;
    pub fn CFDateFormatterGetDateStyle(formatter: CFDateFormatterRef) -> CFDateFormatterStyle;
    pub fn CFDateFormatterGetFormat(formatter: CFDateFormatterRef) -> CFStringRef;
    pub fn CFDateFormatterGetLocale(formatter: CFDateFormatterRef) -> CFLocaleRef;
    pub fn CFDateFormatterGetTimeStyle(formatter: CFDateFormatterRef) -> CFDateFormatterStyle;

    /* Getting the CFDateFormatter Type ID */
    pub fn CFDateFormatterGetTypeID() -> CFTypeID;

    //pub fn CFDateFormatterCreateISO8601Formatter(allocator: CFAllocatorRef, formatOptions: CFISO8601DateFormatOptions) -> CFDateFormatterRef; // macosx(10.12)+
}
