// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{CFAllocatorRef, CFIndex, CFOptionFlags, CFTypeID, SInt32};
use crate::date::CFTimeInterval;
use crate::dictionary::CFDictionaryRef;
use crate::runloop::CFRunLoopSourceRef;
use crate::string::CFStringRef;
use crate::url::CFURLRef;

#[repr(C)]
pub struct __CFUserNotification(c_void);

pub type CFUserNotificationCallBack =
    extern "C" fn(userNotification: CFUserNotificationRef, responseFlags: CFOptionFlags);
pub type CFUserNotificationRef = *mut __CFUserNotification;

/* Alert Levels */
pub const kCFUserNotificationStopAlertLevel: CFOptionFlags = 0;
pub const kCFUserNotificationNoteAlertLevel: CFOptionFlags = 1;
pub const kCFUserNotificationCautionAlertLevel: CFOptionFlags = 2;
pub const kCFUserNotificationPlainAlertLevel: CFOptionFlags = 3;

/* Response Codes */
pub const kCFUserNotificationDefaultResponse: CFOptionFlags = 0;
pub const kCFUserNotificationAlternateResponse: CFOptionFlags = 1;
pub const kCFUserNotificationOtherResponse: CFOptionFlags = 2;
pub const kCFUserNotificationCancelResponse: CFOptionFlags = 3;

/* Button Flags */
pub const kCFUserNotificationNoDefaultButtonFlag: CFOptionFlags = 1usize << 5;
pub const kCFUserNotificationUseRadioButtonsFlag: CFOptionFlags = 1usize << 6;

#[inline(always)]
pub fn CFUserNotificationCheckBoxChecked(i: CFIndex) -> CFOptionFlags {
    (1u32 << (8 + i)) as CFOptionFlags
}

#[inline(always)]
pub fn CFUserNotificationSecureTextField(i: CFIndex) -> CFOptionFlags {
    (1u32 << (16 + i)) as CFOptionFlags
}

#[inline(always)]
pub fn CFUserNotificationPopUpSelection(n: CFIndex) -> CFOptionFlags {
    (n << 24) as CFOptionFlags
}

extern "C" {
    /*
     * CFUserNotification.h
     */

    /* Dialog Description Keys */
    pub static kCFUserNotificationIconURLKey: CFStringRef;
    pub static kCFUserNotificationSoundURLKey: CFStringRef;
    pub static kCFUserNotificationLocalizationURLKey: CFStringRef;
    pub static kCFUserNotificationAlertHeaderKey: CFStringRef;
    pub static kCFUserNotificationAlertMessageKey: CFStringRef;
    pub static kCFUserNotificationDefaultButtonTitleKey: CFStringRef;
    pub static kCFUserNotificationAlternateButtonTitleKey: CFStringRef;
    pub static kCFUserNotificationOtherButtonTitleKey: CFStringRef;
    pub static kCFUserNotificationProgressIndicatorValueKey: CFStringRef;
    pub static kCFUserNotificationPopUpTitlesKey: CFStringRef;
    pub static kCFUserNotificationTextFieldTitlesKey: CFStringRef;
    pub static kCFUserNotificationCheckBoxTitlesKey: CFStringRef;
    pub static kCFUserNotificationTextFieldValuesKey: CFStringRef;
    pub static kCFUserNotificationPopUpSelectionKey: CFStringRef;
    pub static kCFUserNotificationAlertTopMostKey: CFStringRef;
    pub static kCFUserNotificationKeyboardTypesKey: CFStringRef;

    /* CFUserNotification Miscellaneous Functions */
    pub fn CFUserNotificationCancel(userNotification: CFUserNotificationRef) -> SInt32;
    pub fn CFUserNotificationCreate(
        allocator: CFAllocatorRef,
        timeout: CFTimeInterval,
        flags: CFOptionFlags,
        error: *mut SInt32,
        dictionary: CFDictionaryRef,
    ) -> CFUserNotificationRef;
    pub fn CFUserNotificationCreateRunLoopSource(
        allocator: CFAllocatorRef,
        userNotification: CFUserNotificationRef,
        callout: CFUserNotificationCallBack,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
    pub fn CFUserNotificationDisplayAlert(
        timeout: CFTimeInterval,
        flags: CFOptionFlags,
        iconURL: CFURLRef,
        soundURL: CFURLRef,
        localizationURL: CFURLRef,
        alertHeader: CFStringRef,
        alertMessage: CFStringRef,
        defaultButtonTitle: CFStringRef,
        alternateButtonTitle: CFStringRef,
        otherButtonTitle: CFStringRef,
        responseFlags: *mut CFOptionFlags,
    ) -> SInt32;
    pub fn CFUserNotificationDisplayNotice(
        timeout: CFTimeInterval,
        flags: CFOptionFlags,
        iconURL: CFURLRef,
        soundURL: CFURLRef,
        localizationURL: CFURLRef,
        alertHeader: CFStringRef,
        alertMessage: CFStringRef,
        defaultButtonTitle: CFStringRef,
    ) -> SInt32;
    pub fn CFUserNotificationGetTypeID() -> CFTypeID;
    pub fn CFUserNotificationGetResponseDictionary(
        userNotification: CFUserNotificationRef,
    ) -> CFDictionaryRef;
    pub fn CFUserNotificationGetResponseValue(
        userNotification: CFUserNotificationRef,
        key: CFStringRef,
        idx: CFIndex,
    ) -> CFStringRef;
    pub fn CFUserNotificationReceiveResponse(
        userNotification: CFUserNotificationRef,
        timeout: CFTimeInterval,
        responseFlags: *mut CFOptionFlags,
    ) -> SInt32;
    pub fn CFUserNotificationUpdate(
        userNotification: CFUserNotificationRef,
        timeout: CFTimeInterval,
        flags: CFOptionFlags,
        dictionary: CFDictionaryRef,
    ) -> SInt32;
}
