use bitflags::bitflags;
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFType, TCFType},
    dictionary::CFDictionary,
    string::{CFString, CFStringRef},
};

use crate::{
    geometry::CGRect,
    image::{CGImage, CGImageRef},
};

pub type CGWindowID = u32;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGWindowSharingType {
    #[doc(alias = "kCGWindowSharingNone")]
    None      = 0,
    #[doc(alias = "kCGWindowSharingReadOnly")]
    ReadOnly  = 1,
    #[doc(alias = "kCGWindowSharingReadWrite")]
    ReadWrite = 2,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGWindowBackingType {
    #[doc(alias = "kCGWindowBackingStoreRetained")]
    Retained    = 0,
    #[doc(alias = "kCGWindowBackingStoreNonretained")]
    Nonretained = 1,
    #[doc(alias = "kCGWindowBackingStoreBuffered")]
    Buffered    = 2,
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct CGWindowListOption: u32 {
        #[doc(alias = "kCGWindowListOptionAll")]
        const All = 0;
        #[doc(alias = "kCGWindowListOptionOnScreenOnly")]
        const OnScreenOnly = 1 << 0;
        #[doc(alias = "kCGWindowListOptionOnScreenAboveWindow")]
        const OnScreenAboveWindow = 1 << 1;
        #[doc(alias = "kCGWindowListOptionOnScreenBelowWindow")]
        const OnScreenBelowWindow = 1 << 2;
        #[doc(alias = "kCGWindowListOptionIncludingWindow")]
        const IncludingWindow = 1 << 3;
        #[doc(alias = "kCGWindowListExcludeDesktopElements")]
        const ExcludeDesktopElements = 1 << 4;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct CGWindowImageOption: u32 {
        #[doc(alias = "kCGWindowImageDefault")]
        const Default = 0;
        #[doc(alias = "kCGWindowImageBoundsIgnoreFraming")]
        const BoundsIgnoreFraming = 1 << 0;
        #[doc(alias = "kCGWindowImageShouldBeOpaque")]
        const ShouldBeOpaque = 1 << 1;
        #[doc(alias = "kCGWindowImageOnlyShadows")]
        const OnlyShadows = 1 << 2;
        #[doc(alias = "kCGWindowImageBestResolution")]
        const BestResolution = 1 << 3;
        #[doc(alias = "kCGWindowImageNominalResolution")]
        const NominalResolution = 1 << 4;
    }
}

pub const kCGNullWindowID: CGWindowID = 0;

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

    pub fn CGWindowListCopyWindowInfo(option: CGWindowListOption, relativeToWindow: CGWindowID) -> CFArrayRef;
    pub fn CGWindowListCreate(option: CGWindowListOption, relativeToWindow: CGWindowID) -> CFArrayRef;
    pub fn CGWindowListCreateDescriptionFromArray(windowArray: CFArrayRef) -> CFArrayRef;
    pub fn CGWindowListCreateImage(
        screenBounds: CGRect,
        listOption: CGWindowListOption,
        windowID: CGWindowID,
        imageOption: CGWindowImageOption,
    ) -> CGImageRef;
    pub fn CGWindowListCreateImageFromArray(screenBounds: CGRect, windowArray: CFArrayRef, imageOption: CGWindowImageOption) -> CGImageRef;
}

extern "C" {
    pub fn CGRequestScreenCaptureAccess() -> bool;
    pub fn CGPreflightScreenCaptureAccess() -> bool;
}

pub fn copy_window_info(option: CGWindowListOption, relative_to_window: CGWindowID) -> Option<CFArray> {
    unsafe {
        let array = CGWindowListCopyWindowInfo(option, relative_to_window);
        if array.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(array))
        }
    }
}

pub fn new_window_list(option: CGWindowListOption, relative_to_window: CGWindowID) -> Option<CFArray<CGWindowID>> {
    unsafe {
        let array = CGWindowListCreate(option, relative_to_window);
        if array.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(array))
        }
    }
}

pub fn new_description_from_array(window_array: CFArray<CGWindowID>) -> Option<CFArray<CFDictionary<CFString, CFType>>> {
    unsafe {
        let array = CGWindowListCreateDescriptionFromArray(window_array.as_concrete_TypeRef());
        if array.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(array))
        }
    }
}

pub fn new_image(
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
            Some(TCFType::wrap_under_create_rule(image))
        }
    }
}

pub fn new_image_from_array(screen_bounds: CGRect, window_array: CFArray<CGWindowID>, image_option: CGWindowImageOption) -> Option<CGImage> {
    unsafe {
        let image = CGWindowListCreateImageFromArray(screen_bounds, window_array.as_concrete_TypeRef(), image_option);
        if image.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_create_rule(image))
        }
    }
}

pub fn request_screen_capture_access() -> bool {
    unsafe { CGRequestScreenCaptureAccess() }
}

pub fn preflight_screen_capture_access() -> bool {
    unsafe { CGPreflightScreenCaptureAccess() }
}

pub enum WindowKeys {
    Number,
    StoreType,
    Layer,
    Bounds,
    SharingState,
    Alpha,
    OwnerPID,
    MemoryUsage,
    Workspace,
    OwnerName,
    Name,
    IsOnscreen,
    BackingLocationVideoMemory,
}

impl From<WindowKeys> for CFStringRef {
    fn from(key: WindowKeys) -> Self {
        unsafe {
            match key {
                WindowKeys::Number => kCGWindowNumber,
                WindowKeys::StoreType => kCGWindowStoreType,
                WindowKeys::Layer => kCGWindowLayer,
                WindowKeys::Bounds => kCGWindowBounds,
                WindowKeys::SharingState => kCGWindowSharingState,
                WindowKeys::Alpha => kCGWindowAlpha,
                WindowKeys::OwnerPID => kCGWindowOwnerPID,
                WindowKeys::MemoryUsage => kCGWindowMemoryUsage,
                WindowKeys::Workspace => kCGWindowWorkspace,
                WindowKeys::OwnerName => kCGWindowOwnerName,
                WindowKeys::Name => kCGWindowName,
                WindowKeys::IsOnscreen => kCGWindowIsOnscreen,
                WindowKeys::BackingLocationVideoMemory => kCGWindowBackingLocationVideoMemory,
            }
        }
    }
}

impl From<WindowKeys> for CFString {
    fn from(key: WindowKeys) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}
