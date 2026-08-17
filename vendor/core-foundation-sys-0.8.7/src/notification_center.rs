// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{Boolean, CFIndex, CFOptionFlags, CFTypeID};
use crate::dictionary::CFDictionaryRef;
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFNotificationCenter(c_void);

pub type CFNotificationCenterRef = *mut __CFNotificationCenter;

pub type CFNotificationName = CFStringRef;
pub type CFNotificationCallback = extern "C" fn(
    center: CFNotificationCenterRef,
    observer: *mut c_void,
    name: CFNotificationName,
    object: *const c_void,
    userInfo: CFDictionaryRef,
);
pub type CFNotificationSuspensionBehavior = CFIndex;

pub const CFNotificationSuspensionBehaviorDrop: CFNotificationSuspensionBehavior = 1;
pub const CFNotificationSuspensionBehaviorCoalesce: CFNotificationSuspensionBehavior = 2;
pub const CFNotificationSuspensionBehaviorHold: CFNotificationSuspensionBehavior = 3;
pub const CFNotificationSuspensionBehaviorDeliverImmediately: CFNotificationSuspensionBehavior = 4;

/* Notification Posting Options */
pub const kCFNotificationDeliverImmediately: CFOptionFlags = 1usize << 0;
pub const kCFNotificationPostToAllSessions: CFOptionFlags = 1usize << 1;

extern "C" {
    /*
     *  CFNotificationCenter.h
     */

    /* Accessing a Notification Center */
    pub fn CFNotificationCenterGetDarwinNotifyCenter() -> CFNotificationCenterRef;
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn CFNotificationCenterGetDistributedCenter() -> CFNotificationCenterRef;
    pub fn CFNotificationCenterGetLocalCenter() -> CFNotificationCenterRef;

    /* Posting a Notification */
    pub fn CFNotificationCenterPostNotification(
        center: CFNotificationCenterRef,
        name: CFNotificationName,
        object: *const c_void,
        userInfo: CFDictionaryRef,
        deliverImmediately: Boolean,
    );
    pub fn CFNotificationCenterPostNotificationWithOptions(
        center: CFNotificationCenterRef,
        name: CFNotificationName,
        object: *const c_void,
        userInfo: CFDictionaryRef,
        options: CFOptionFlags,
    );

    /* Adding and Removing Observers */
    pub fn CFNotificationCenterAddObserver(
        center: CFNotificationCenterRef,
        observer: *const c_void,
        callBack: CFNotificationCallback,
        name: CFStringRef,
        object: *const c_void,
        suspensionBehavior: CFNotificationSuspensionBehavior,
    );
    pub fn CFNotificationCenterRemoveEveryObserver(
        center: CFNotificationCenterRef,
        observer: *const c_void,
    );
    pub fn CFNotificationCenterRemoveObserver(
        center: CFNotificationCenterRef,
        observer: *const c_void,
        name: CFNotificationName,
        object: *const c_void,
    );

    /* Getting the CFNotificationCenter Type ID */
    pub fn CFNotificationCenterGetTypeID() -> CFTypeID;
}
