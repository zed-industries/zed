// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::array::CFArrayRef;
use crate::base::{Boolean, CFIndex};
use crate::dictionary::CFDictionaryRef;
use crate::propertylist::CFPropertyListRef;
use crate::string::CFStringRef;

extern "C" {
    /*
     * CFPreferences.h
     */
    /* Application, Host, and User Keys */
    pub static kCFPreferencesAnyApplication: CFStringRef;
    pub static kCFPreferencesCurrentApplication: CFStringRef;
    pub static kCFPreferencesAnyHost: CFStringRef;
    pub static kCFPreferencesCurrentHost: CFStringRef;
    pub static kCFPreferencesAnyUser: CFStringRef;
    pub static kCFPreferencesCurrentUser: CFStringRef;

    /* Getting Preference Values */
    pub fn CFPreferencesCopyAppValue(
        key: CFStringRef,
        applicationID: CFStringRef,
    ) -> CFPropertyListRef;
    pub fn CFPreferencesCopyKeyList(
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> CFArrayRef;
    pub fn CFPreferencesCopyMultiple(
        keysToFetch: CFArrayRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> CFDictionaryRef;
    pub fn CFPreferencesCopyValue(
        key: CFStringRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> CFPropertyListRef;
    pub fn CFPreferencesGetAppBooleanValue(
        key: CFStringRef,
        applicationID: CFStringRef,
        keyExistsAndHasValidFormat: *mut Boolean,
    ) -> Boolean;
    pub fn CFPreferencesGetAppIntegerValue(
        key: CFStringRef,
        applicationID: CFStringRef,
        keyExistsAndHasValidFormat: *mut Boolean,
    ) -> CFIndex;

    /* Setting Preference Values */
    pub fn CFPreferencesSetAppValue(
        key: CFStringRef,
        value: CFPropertyListRef,
        applicationID: CFStringRef,
    );
    pub fn CFPreferencesSetMultiple(
        keysToSet: CFDictionaryRef,
        keysToRemove: CFArrayRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    );
    pub fn CFPreferencesSetValue(
        key: CFStringRef,
        value: CFPropertyListRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    );

    /* Synchronizing Preferences */
    pub fn CFPreferencesAppSynchronize(applicationID: CFStringRef) -> Boolean;
    pub fn CFPreferencesSynchronize(
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> Boolean;

    /* Adding and Removing Suite Preferences */
    pub fn CFPreferencesAddSuitePreferencesToApp(applicationID: CFStringRef, suiteID: CFStringRef);
    pub fn CFPreferencesRemoveSuitePreferencesFromApp(
        applicationID: CFStringRef,
        suiteID: CFStringRef,
    );

    /* Miscellaneous Functions */
    pub fn CFPreferencesAppValueIsForced(key: CFStringRef, applicationID: CFStringRef) -> Boolean;
    pub fn CFPreferencesCopyApplicationList(
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> CFArrayRef; // deprecated since macos 10.9
}
