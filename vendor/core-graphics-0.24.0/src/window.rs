// Copyright 2018 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFType, TCFType};
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::{CFString, CFStringRef};
use foreign_types::ForeignType;

use crate::geometry::CGRect;
use crate::image::CGImage;
use crate::sys;

pub type CGWindowID = u32;

pub type CGWindowSharingType = u32;
pub const kCGWindowSharingNone: CGWindowSharingType = 0;
pub const kCGWindowSharingReadOnly: CGWindowSharingType = 1;
pub const kCGWindowSharingReadWrite: CGWindowSharingType = 2;

pub type CGWindowBackingType = u32;
pub const kCGWindowBackingStoreRetained: CGWindowBackingType = 0;
pub const kCGWindowBackingStoreNonretained: CGWindowBackingType = 1;
pub const kCGWindowBackingStoreBuffered: CGWindowBackingType = 2;

// https://developer.apple.com/documentation/coregraphics/quartz_window_services/window_list_option_constants?language=objc
pub type CGWindowListOption = u32;
pub const kCGWindowListOptionAll: CGWindowListOption = 0;
pub const kCGWindowListOptionOnScreenOnly: CGWindowListOption = 1 << 0;
pub const kCGWindowListOptionOnScreenAboveWindow: CGWindowListOption = 1 << 1;
pub const kCGWindowListOptionOnScreenBelowWindow: CGWindowListOption = 1 << 2;
pub const kCGWindowListOptionIncludingWindow: CGWindowListOption = 1 << 3;
pub const kCGWindowListExcludeDesktopElements: CGWindowListOption = 1 << 4;

pub type CGWindowImageOption = u32;
pub const kCGWindowImageDefault: CGWindowImageOption = 0;
pub const kCGWindowImageBoundsIgnoreFraming: CGWindowImageOption = 1 << 0;
pub const kCGWindowImageShouldBeOpaque: CGWindowImageOption = 1 << 1;
pub const kCGWindowImageOnlyShadows: CGWindowImageOption = 1 << 2;
pub const kCGWindowImageBestResolution: CGWindowImageOption = 1 << 3;
pub const kCGWindowImageNominalResolution: CGWindowImageOption = 1 << 4;

pub const kCGNullWindowID: CGWindowID = 0;

pub fn copy_window_info(
    option: CGWindowListOption,
    relative_to_window: CGWindowID,
) -> Option<CFArray> {
    unsafe {
        let array = CGWindowListCopyWindowInfo(option, relative_to_window);
        if array.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(array))
        }
    }
}

pub fn create_window_list(
    option: CGWindowListOption,
    relative_to_window: CGWindowID,
) -> Option<CFArray<CGWindowID>> {
    unsafe {
        let array = CGWindowListCreate(option, relative_to_window);
        if array.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(array))
        }
    }
}

pub fn create_description_from_array(
    window_array: CFArray<CGWindowID>,
) -> Option<CFArray<CFDictionary<CFString, CFType>>> {
    unsafe {
        let array = CGWindowListCreateDescriptionFromArray(window_array.as_concrete_TypeRef());
        if array.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(array))
        }
    }
}

pub fn create_image(
    screen_bounds: CGRect,
    list_option: CGWindowListOption,
    window_id: CGWindowID,
    image_option: CGWindowImageOption,
) -> Option<CGImage> {
    unsafe {
        let image = CGWindowListCreateImage(screen_bounds, list_option, window_id, image_option);
        if image.is_null() {
            None
        } else {
            Some(CGImage::from_ptr(image))
        }
    }
}

pub fn create_image_from_array(
    screen_bounds: CGRect,
    window_array: CFArray,
    image_option: CGWindowImageOption,
) -> Option<CGImage> {
    unsafe {
        let image = CGWindowListCreateImageFromArray(
            screen_bounds,
            window_array.as_concrete_TypeRef(),
            image_option,
        );
        if image.is_null() {
            None
        } else {
            Some(CGImage::from_ptr(image))
        }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    pub static kCGWindowNumber: CFStringRef;
    pub static kCGWindowStoreType: CFStringRef;
    pub static kCGWindowLayer: CFStringRef;
    pub static kCGWindowBounds: CFStringRef;
    pub static kCGWindowSharingState: CFStringRef;
    pub static kCGWindowAlpha: CFStringRef;
    pub static kCGWindowOwnerPID: CFStringRef;
    pub static kCGWindowMemoryUsage: CFStringRef;
    pub static kCGWindowWorkspace: CFStringRef;
    pub static kCGWindowOwnerName: CFStringRef;
    pub static kCGWindowName: CFStringRef;
    pub static kCGWindowIsOnscreen: CFStringRef;
    pub static kCGWindowBackingLocationVideoMemory: CFStringRef;

    pub fn CGWindowListCopyWindowInfo(
        option: CGWindowListOption,
        relativeToWindow: CGWindowID,
    ) -> CFArrayRef;
    pub fn CGWindowListCreate(
        option: CGWindowListOption,
        relativeToWindow: CGWindowID,
    ) -> CFArrayRef;
    pub fn CGWindowListCreateDescriptionFromArray(windowArray: CFArrayRef) -> CFArrayRef;
    pub fn CGWindowListCreateImage(
        screenBounds: CGRect,
        listOption: CGWindowListOption,
        windowID: CGWindowID,
        imageOption: CGWindowImageOption,
    ) -> *mut sys::CGImage;
    pub fn CGWindowListCreateImageFromArray(
        screenBounds: CGRect,
        windowArray: CFArrayRef,
        imageOption: CGWindowImageOption,
    ) -> *mut sys::CGImage;
}
