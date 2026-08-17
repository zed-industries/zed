// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::array::CFArrayRef;
use crate::base::{CFAllocatorRef, CFIndex, CFTypeID, CFTypeRef, LangCode, RegionCode};
use crate::dictionary::CFDictionaryRef;
use crate::notification_center::CFNotificationName;
use crate::string::CFStringRef;
use std::os::raw::c_void;

#[repr(C)]
pub struct __CFLocale(c_void);
pub type CFLocaleRef = *const __CFLocale;

pub type CFLocaleIdentifier = CFStringRef;
pub type CFLocaleKey = CFStringRef;
pub type CFCalendarIdentifier = CFStringRef;
pub type CFLocaleLanguageDirection = CFIndex;

pub const kCFLocaleLanguageDirectionUnknown: CFLocaleLanguageDirection = 0;
pub const kCFLocaleLanguageDirectionLeftToRight: CFLocaleLanguageDirection = 1;
pub const kCFLocaleLanguageDirectionRightToLeft: CFLocaleLanguageDirection = 2;
pub const kCFLocaleLanguageDirectionTopToBottom: CFLocaleLanguageDirection = 3;
pub const kCFLocaleLanguageDirectionBottomToTop: CFLocaleLanguageDirection = 4;

extern "C" {
    /*
     * CFLocale.h
     */

    /* Locale Change Notification */
    pub static kCFLocaleCurrentLocaleDidChangeNotification: CFNotificationName;

    /* Locale Property Keys */
    pub static kCFLocaleIdentifier: CFLocaleKey;
    pub static kCFLocaleLanguageCode: CFLocaleKey;
    pub static kCFLocaleCountryCode: CFLocaleKey;
    pub static kCFLocaleScriptCode: CFLocaleKey;
    pub static kCFLocaleVariantCode: CFLocaleKey;

    pub static kCFLocaleExemplarCharacterSet: CFLocaleKey;
    pub static kCFLocaleCalendarIdentifier: CFLocaleKey;
    pub static kCFLocaleCalendar: CFLocaleKey;
    pub static kCFLocaleCollationIdentifier: CFLocaleKey;
    pub static kCFLocaleUsesMetricSystem: CFLocaleKey;
    pub static kCFLocaleMeasurementSystem: CFLocaleKey;
    pub static kCFLocaleDecimalSeparator: CFLocaleKey;
    pub static kCFLocaleGroupingSeparator: CFLocaleKey;
    pub static kCFLocaleCurrencySymbol: CFLocaleKey;
    pub static kCFLocaleCurrencyCode: CFLocaleKey;
    pub static kCFLocaleCollatorIdentifier: CFLocaleKey;
    pub static kCFLocaleQuotationBeginDelimiterKey: CFLocaleKey;
    pub static kCFLocaleQuotationEndDelimiterKey: CFLocaleKey;
    pub static kCFLocaleAlternateQuotationBeginDelimiterKey: CFLocaleKey;
    pub static kCFLocaleAlternateQuotationEndDelimiterKey: CFLocaleKey;

    /* Locale Calendar Identifiers */
    pub static kCFGregorianCalendar: CFCalendarIdentifier;
    pub static kCFBuddhistCalendar: CFCalendarIdentifier;
    pub static kCFChineseCalendar: CFCalendarIdentifier;
    pub static kCFHebrewCalendar: CFCalendarIdentifier;
    pub static kCFIslamicCalendar: CFCalendarIdentifier;
    pub static kCFIslamicCivilCalendar: CFCalendarIdentifier;
    pub static kCFJapaneseCalendar: CFCalendarIdentifier;
    pub static kCFRepublicOfChinaCalendar: CFCalendarIdentifier;
    pub static kCFPersianCalendar: CFCalendarIdentifier;
    pub static kCFIndianCalendar: CFCalendarIdentifier;
    pub static kCFISO8601Calendar: CFCalendarIdentifier;
    //pub static kCFIslamicTabularCalendar: CFCalendarIdentifier; // macos(10.10)+
    //pub static kCFIslamicUmmAlQuraCalendar: CFCalendarIdentifier; // macos(10.10)+

    /* Creating a Locale */
    pub fn CFLocaleCopyCurrent() -> CFLocaleRef;
    pub fn CFLocaleCreate(
        allocator: CFAllocatorRef,
        localeIdentifier: CFLocaleIdentifier,
    ) -> CFLocaleRef;
    pub fn CFLocaleCreateCopy(allocator: CFAllocatorRef, locale: CFLocaleRef) -> CFLocaleRef;
    pub fn CFLocaleGetSystem() -> CFLocaleRef;

    /* Getting System Locale Information */
    pub fn CFLocaleCopyAvailableLocaleIdentifiers() -> CFArrayRef;

    /* Getting ISO Information */
    pub fn CFLocaleCopyISOCountryCodes() -> CFArrayRef;
    pub fn CFLocaleCopyISOLanguageCodes() -> CFArrayRef;
    pub fn CFLocaleCopyISOCurrencyCodes() -> CFArrayRef;
    pub fn CFLocaleCopyCommonISOCurrencyCodes() -> CFArrayRef;

    /* Language Preferences */
    pub fn CFLocaleCopyPreferredLanguages() -> CFArrayRef;

    /* Getting Information About a Locale */
    pub fn CFLocaleCopyDisplayNameForPropertyValue(
        displayLocale: CFLocaleRef,
        key: CFLocaleKey,
        value: CFStringRef,
    ) -> CFStringRef;
    pub fn CFLocaleGetValue(locale: CFLocaleRef, key: CFLocaleKey) -> CFTypeRef;
    pub fn CFLocaleGetIdentifier(locale: CFLocaleRef) -> CFLocaleIdentifier;

    /* Getting and Creating Locale Identifiers */
    pub fn CFLocaleCreateCanonicalLocaleIdentifierFromScriptManagerCodes(
        allocator: CFAllocatorRef,
        lcode: LangCode,
        rcode: RegionCode,
    ) -> CFLocaleIdentifier;
    pub fn CFLocaleCreateCanonicalLanguageIdentifierFromString(
        allocator: CFAllocatorRef,
        localeIdentifier: CFStringRef,
    ) -> CFLocaleIdentifier;
    pub fn CFLocaleCreateCanonicalLocaleIdentifierFromString(
        allocator: CFAllocatorRef,
        localeIdentifier: CFStringRef,
    ) -> CFLocaleIdentifier;
    pub fn CFLocaleCreateComponentsFromLocaleIdentifier(
        allocator: CFAllocatorRef,
        localeID: CFLocaleIdentifier,
    ) -> CFDictionaryRef;
    pub fn CFLocaleCreateLocaleIdentifierFromComponents(
        allocator: CFAllocatorRef,
        dictionary: CFDictionaryRef,
    ) -> CFLocaleIdentifier;
    pub fn CFLocaleCreateLocaleIdentifierFromWindowsLocaleCode(
        allocator: CFAllocatorRef,
        lcid: u32,
    ) -> CFLocaleIdentifier;
    pub fn CFLocaleGetWindowsLocaleCodeFromLocaleIdentifier(
        localeIdentifier: CFLocaleIdentifier,
    ) -> u32;

    /* Getting Line and Character Direction for a Language */
    pub fn CFLocaleGetLanguageCharacterDirection(
        isoLangCode: CFStringRef,
    ) -> CFLocaleLanguageDirection;
    pub fn CFLocaleGetLanguageLineDirection(isoLangCode: CFStringRef) -> CFLocaleLanguageDirection;

    /* Getting the CFLocale Type ID */
    pub fn CFLocaleGetTypeID() -> CFTypeID;
}
