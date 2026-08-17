// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::{c_double, c_void};

use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, CFRange, CFTypeID, CFTypeRef};
use crate::locale::CFLocaleRef;
use crate::number::{CFNumberRef, CFNumberType};
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFNumberFormatter(c_void);

pub type CFNumberFormatterRef = *mut __CFNumberFormatter;

pub type CFNumberFormatterKey = CFStringRef;
pub type CFNumberFormatterStyle = CFIndex;
pub type CFNumberFormatterOptionFlags = CFOptionFlags;
pub type CFNumberFormatterRoundingMode = CFIndex;
pub type CFNumberFormatterPadPosition = CFIndex;

/* Number Formatter Styles */
pub const kCFNumberFormatterNoStyle: CFNumberFormatterStyle = 0;
pub const kCFNumberFormatterDecimalStyle: CFNumberFormatterStyle = 1;
pub const kCFNumberFormatterCurrencyStyle: CFNumberFormatterStyle = 2;
pub const kCFNumberFormatterPercentStyle: CFNumberFormatterStyle = 3;
pub const kCFNumberFormatterScientificStyle: CFNumberFormatterStyle = 4;
pub const kCFNumberFormatterSpellOutStyle: CFNumberFormatterStyle = 5;
//pub const kCFNumberFormatterOrdinalStyle: CFNumberFormatterStyle = 6; // macos(10.11)+
//pub const kCFNumberFormatterCurrencyISOCodeStyle: CFNumberFormatterStyle = 8; // macos(10.11)+
//pub const kCFNumberFormatterCurrencyPluralStyle: CFNumberFormatterStyle = 9; // macos(10.11)+
//pub const kCFNumberFormatterCurrencyAccountingStyle: CFNumberFormatterStyle = 10; // macos(10.11)+

/* Number Format Options */
pub const kCFNumberFormatterParseIntegersOnly: CFNumberFormatterOptionFlags = 1;

/* CFNumberFormatterRoundingMode */
pub const kCFNumberFormatterRoundCeiling: CFNumberFormatterRoundingMode = 0;
pub const kCFNumberFormatterRoundFloor: CFNumberFormatterRoundingMode = 1;
pub const kCFNumberFormatterRoundDown: CFNumberFormatterRoundingMode = 2;
pub const kCFNumberFormatterRoundUp: CFNumberFormatterRoundingMode = 3;
pub const kCFNumberFormatterRoundHalfEven: CFNumberFormatterRoundingMode = 4;
pub const kCFNumberFormatterRoundHalfDown: CFNumberFormatterRoundingMode = 5;
pub const kCFNumberFormatterRoundHalfUp: CFNumberFormatterRoundingMode = 6;

/* Padding Positions */
pub const kCFNumberFormatterPadBeforePrefix: CFNumberFormatterPadPosition = 0;
pub const kCFNumberFormatterPadAfterPrefix: CFNumberFormatterPadPosition = 1;
pub const kCFNumberFormatterPadBeforeSuffix: CFNumberFormatterPadPosition = 2;
pub const kCFNumberFormatterPadAfterSuffix: CFNumberFormatterPadPosition = 3;

extern "C" {
    /*
     * CFNumberFormatter.h
     */

    /* Number Formatter Property Keys */
    // The values for these keys are all CFType objects.
    // The specific types for each key are specified above.
    pub static kCFNumberFormatterCurrencyCode: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterDecimalSeparator: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterCurrencyDecimalSeparator: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterAlwaysShowDecimalSeparator: CFNumberFormatterKey; // CFBoolean
    pub static kCFNumberFormatterGroupingSeparator: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterUseGroupingSeparator: CFNumberFormatterKey; // CFBoolean
    pub static kCFNumberFormatterPercentSymbol: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterZeroSymbol: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterNaNSymbol: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterInfinitySymbol: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterMinusSign: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterPlusSign: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterCurrencySymbol: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterExponentSymbol: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterMinIntegerDigits: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterMaxIntegerDigits: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterMinFractionDigits: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterMaxFractionDigits: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterGroupingSize: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterSecondaryGroupingSize: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterRoundingMode: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterRoundingIncrement: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterFormatWidth: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterPaddingPosition: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterPaddingCharacter: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterDefaultFormat: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterMultiplier: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterPositivePrefix: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterPositiveSuffix: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterNegativePrefix: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterNegativeSuffix: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterPerMillSymbol: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterInternationalCurrencySymbol: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterCurrencyGroupingSeparator: CFNumberFormatterKey; // CFString
    pub static kCFNumberFormatterIsLenient: CFNumberFormatterKey; // CFBoolean
    pub static kCFNumberFormatterUseSignificantDigits: CFNumberFormatterKey; // CFBoolean
    pub static kCFNumberFormatterMinSignificantDigits: CFNumberFormatterKey; // CFNumber
    pub static kCFNumberFormatterMaxSignificantDigits: CFNumberFormatterKey; // CFNumber

    /* Creating a Number Formatter */
    pub fn CFNumberFormatterCreate(
        allocator: CFAllocatorRef,
        locale: CFLocaleRef,
        style: CFNumberFormatterStyle,
    ) -> CFNumberFormatterRef;

    /* Configuring a Number Formatter */
    pub fn CFNumberFormatterSetFormat(formatter: CFNumberFormatterRef, formatString: CFStringRef);
    pub fn CFNumberFormatterSetProperty(
        formatter: CFNumberFormatterRef,
        key: CFNumberFormatterKey,
        value: CFTypeRef,
    );

    /* Formatting Values */
    pub fn CFNumberFormatterCreateNumberFromString(
        allocator: CFAllocatorRef,
        formatter: CFNumberFormatterRef,
        string: CFStringRef,
        rangep: *mut CFRange,
        options: CFOptionFlags,
    ) -> CFNumberRef;
    pub fn CFNumberFormatterCreateStringWithNumber(
        allocator: CFAllocatorRef,
        formatter: CFNumberFormatterRef,
        number: CFNumberRef,
    ) -> CFStringRef;
    pub fn CFNumberFormatterCreateStringWithValue(
        allocator: CFAllocatorRef,
        formatter: CFNumberFormatterRef,
        numberType: CFNumberType,
        valuePtr: *const c_void,
    ) -> CFStringRef;
    pub fn CFNumberFormatterGetDecimalInfoForCurrencyCode(
        currencyCode: CFStringRef,
        defaultFractionDigits: *mut i32,
        roundingIncrement: *mut c_double,
    ) -> Boolean;
    pub fn CFNumberFormatterGetValueFromString(
        formatter: CFNumberFormatterRef,
        string: CFStringRef,
        rangep: *mut CFRange,
        numberType: CFNumberType,
        valuePtr: *mut c_void,
    ) -> Boolean;

    /* Examining a Number Formatter */
    pub fn CFNumberFormatterCopyProperty(
        formatter: CFNumberFormatterRef,
        key: CFNumberFormatterKey,
    ) -> CFTypeRef;
    pub fn CFNumberFormatterGetFormat(formatter: CFNumberFormatterRef) -> CFStringRef;
    pub fn CFNumberFormatterGetLocale(formatter: CFNumberFormatterRef) -> CFLocaleRef;
    pub fn CFNumberFormatterGetStyle(formatter: CFNumberFormatterRef) -> CFNumberFormatterStyle;

    /* Getting the CFNumberFormatter Type ID */
    pub fn CFNumberFormatterGetTypeID() -> CFTypeID;
}
