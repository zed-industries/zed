// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use bitflags::bitflags;
use libc;
use std::ops::Deref;
use std::ptr;

pub use crate::base::{boolean_t, CGError};
pub use crate::geometry::{CGPoint, CGRect, CGSize};

use crate::image::CGImage;
use core_foundation::base::{CFRetain, TCFType};
use core_foundation::string::{CFString, CFStringRef};
use foreign_types::{foreign_type, ForeignType};

pub type CGDirectDisplayID = u32;
pub type CGWindowID = u32;
pub type CGWindowLevel = i32;

pub const kCGNullWindowID: CGWindowID = 0 as CGWindowID;
pub const kCGNullDirectDisplayID: CGDirectDisplayID = 0 as CGDirectDisplayID;

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

pub const kDisplayModeValidFlag: u32 = 0x00000001;
pub const kDisplayModeSafeFlag: u32 = 0x00000002;
pub const kDisplayModeDefaultFlag: u32 = 0x00000004;
pub const kDisplayModeAlwaysShowFlag: u32 = 0x00000008;
pub const kDisplayModeNeverShowFlag: u32 = 0x00000080;
pub const kDisplayModeNotResizeFlag: u32 = 0x00000010;
pub const kDisplayModeRequiresPanFlag: u32 = 0x00000020;
pub const kDisplayModeInterlacedFlag: u32 = 0x00000040;
pub const kDisplayModeSimulscanFlag: u32 = 0x00000100;
pub const kDisplayModeBuiltInFlag: u32 = 0x00000400;
pub const kDisplayModeNotPresetFlag: u32 = 0x00000200;
pub const kDisplayModeStretchedFlag: u32 = 0x00000800;
pub const kDisplayModeNotGraphicsQualityFlag: u32 = 0x00001000;
pub const kDisplayModeValidateAgainstDisplay: u32 = 0x00002000;
pub const kDisplayModeTelevisionFlag: u32 = 0x00100000;
pub const kDisplayModeValidForMirroringFlag: u32 = 0x00200000;
pub const kDisplayModeAcceleratorBackedFlag: u32 = 0x00400000;
pub const kDisplayModeValidForHiResFlag: u32 = 0x00800000;
pub const kDisplayModeValidForAirPlayFlag: u32 = 0x01000000;
pub const kDisplayModeNativeFlag: u32 = 0x02000000;

pub const kDisplayModeSafetyFlags: u32 = 0x00000007;

pub type CGDisplayBlendFraction = f32;
pub const kCGDisplayBlendNormal: CGDisplayBlendFraction = 0.0;
pub const kCGDisplayBlendSolidColor: CGDisplayBlendFraction = 1.0;

pub type CGDisplayFadeReservationToken = u32;
pub const kCGDisplayFadeReservationInvalidToken: CGDisplayFadeReservationToken = 0;

pub type CGDisplayFadeInterval = f32;
pub type CGDisplayReservationInterval = f32;
pub const kCGMaxDisplayReservationInterval: CGDisplayReservationInterval = 15.0;

pub const IO1BitIndexedPixels: &str = "P";
pub const IO2BitIndexedPixels: &str = "PP";
pub const IO4BitIndexedPixels: &str = "PPPP";
pub const IO8BitIndexedPixels: &str = "PPPPPPPP";
pub const IO16BitDirectPixels: &str = "-RRRRRGGGGGBBBBB";
pub const IO32BitDirectPixels: &str = "--------RRRRRRRRGGGGGGGGBBBBBBBB";
pub const kIO30BitDirectPixels: &str = "--RRRRRRRRRRGGGGGGGGGGBBBBBBBBBB";
pub const kIO64BitDirectPixels: &str = "-16R16G16B16";
pub const kIO16BitFloatPixels: &str = "-16FR16FG16FB16";
pub const kIO32BitFloatPixels: &str = "-32FR32FG32FB32";
pub const IOYUV422Pixels: &str = "Y4U2V2";
pub const IO8BitOverlayPixels: &str = "O8";

pub use core_foundation::array::{CFArray, CFArrayRef};
pub use core_foundation::array::{CFArrayGetCount, CFArrayGetValueAtIndex};
pub use core_foundation::base::{CFIndex, CFRelease, CFTypeRef};
pub use core_foundation::dictionary::{
    CFDictionary, CFDictionaryGetValueIfPresent, CFDictionaryRef,
};

pub type CGDisplayConfigRef = *mut libc::c_void;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum CGConfigureOption {
    ConfigureForAppOnly = 0,
    ConfigureForSession = 1,
    ConfigurePermanently = 2,
}

/// A client-supplied callback function that’s invoked whenever the configuration of a local display is changed.
pub type CGDisplayReconfigurationCallBack =
    unsafe extern "C" fn(display: CGDirectDisplayID, flags: u32, user_info: *const libc::c_void);

bitflags! {
    /// The configuration parameters that are passed to a display reconfiguration callback function.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CGDisplayChangeSummaryFlags: u32 {
        /// The display configuration is about to change.
        const kCGDisplayBeginConfigurationFlag = 1;
        /// The location of the upper-left corner of the display in the global display coordinate space has changed.
        const kCGDisplayMovedFlag = 1 << 1;
        /// The display is now the main display.
        const kCGDisplaySetMainFlag = 1 << 2;
        /// The display mode has changed.
        const kCGDisplaySetModeFlag = 1 << 3;
        /// The display has been added to the active display list.
        const kCGDisplayAddFlag = 1 << 4;
        /// The display has been removed from the active display list.
        const kCGDisplayRemoveFlag = 1 << 5;
        /// The display has been enabled.
        const kCGDisplayEnabledFlag = 1 << 8;
        /// The display has been disabled.
        const kCGDisplayDisabledFlag = 1 << 9;
        /// The display is now mirroring another display.
        const kCGDisplayMirrorFlag = 1 << 10;
        /// The display is no longer mirroring another display.
        const kCGDisplayUnMirrorFlag = 1 << 11;
        /// The shape of the desktop (the union of display areas) has changed.
        const kCGDisplayDesktopShapeChangedFlag = 1 << 12;

        const _ = !0;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CGDisplay {
    pub id: CGDirectDisplayID,
}

foreign_type! {
    #[doc(hidden)]
    pub unsafe type CGDisplayMode {
        type CType = crate::sys::CGDisplayMode;
        fn drop = CGDisplayModeRelease;
        fn clone = |p| CFRetain(p as *const _) as *mut _;
    }
}

impl CGDisplay {
    #[inline]
    pub fn new(id: CGDirectDisplayID) -> CGDisplay {
        CGDisplay { id }
    }

    /// Returns the the main display.
    #[inline]
    pub fn main() -> CGDisplay {
        CGDisplay::new(unsafe { CGMainDisplayID() })
    }

    /// A value that will never correspond to actual hardware.
    pub fn null_display() -> CGDisplay {
        CGDisplay::new(kCGNullDirectDisplayID)
    }

    /// Returns the bounds of a display in the global display coordinate space.
    #[inline]
    pub fn bounds(&self) -> CGRect {
        unsafe { CGDisplayBounds(self.id) }
    }

    /// Returns information about a display's current configuration.
    #[inline]
    pub fn display_mode(&self) -> Option<CGDisplayMode> {
        unsafe {
            let mode_ref = CGDisplayCopyDisplayMode(self.id);
            if !mode_ref.is_null() {
                Some(CGDisplayMode::from_ptr(mode_ref))
            } else {
                None
            }
        }
    }

    /// Begins a new set of display configuration changes.
    pub fn begin_configuration(&self) -> Result<CGDisplayConfigRef, CGError> {
        unsafe {
            let mut config_ref: CGDisplayConfigRef = ptr::null_mut();
            let result = CGBeginDisplayConfiguration(&mut config_ref);
            if result == 0 {
                Ok(config_ref)
            } else {
                Err(result)
            }
        }
    }

    /// Cancels a set of display configuration changes.
    pub fn cancel_configuration(&self, config_ref: &CGDisplayConfigRef) -> Result<(), CGError> {
        let result = unsafe { CGCancelDisplayConfiguration(*config_ref) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Completes a set of display configuration changes.
    pub fn complete_configuration(
        &self,
        config_ref: &CGDisplayConfigRef,
        option: CGConfigureOption,
    ) -> Result<(), CGError> {
        let result = unsafe { CGCompleteDisplayConfiguration(*config_ref, option) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Configures the display mode of a display.
    pub fn configure_display_with_display_mode(
        &self,
        config_ref: &CGDisplayConfigRef,
        display_mode: &CGDisplayMode,
    ) -> Result<(), CGError> {
        let result = unsafe {
            CGConfigureDisplayWithDisplayMode(
                *config_ref,
                self.id,
                display_mode.as_ptr(),
                ptr::null(),
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Configures the origin of a display in the global display coordinate space.
    pub fn configure_display_origin(
        &self,
        config_ref: &CGDisplayConfigRef,
        x: i32,
        y: i32,
    ) -> Result<(), CGError> {
        let result = unsafe { CGConfigureDisplayOrigin(*config_ref, self.id, x, y) };

        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Changes the configuration of a mirroring set.
    pub fn configure_display_mirror_of_display(
        &self,
        config_ref: &CGDisplayConfigRef,
        master: &CGDisplay,
    ) -> Result<(), CGError> {
        let result = unsafe { CGConfigureDisplayMirrorOfDisplay(*config_ref, self.id, master.id) };

        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Returns an image containing the contents of the specified display.
    #[inline]
    pub fn image(&self) -> Option<CGImage> {
        unsafe {
            let image_ref = CGDisplayCreateImage(self.id);
            if !image_ref.is_null() {
                Some(CGImage::from_ptr(image_ref))
            } else {
                None
            }
        }
    }

    /// Returns an image containing the contents of a portion of the specified display.
    #[inline]
    pub fn image_for_rect(&self, bounds: CGRect) -> Option<CGImage> {
        unsafe {
            let image_ref = CGDisplayCreateImageForRect(self.id, bounds);
            if !image_ref.is_null() {
                Some(CGImage::from_ptr(image_ref))
            } else {
                None
            }
        }
    }

    /// Returns a composite image based on a dynamically generated list of
    /// windows.
    #[inline]
    pub fn screenshot(
        bounds: CGRect,
        list_option: CGWindowListOption,
        window_id: CGWindowID,
        image_option: CGWindowImageOption,
    ) -> Option<CGImage> {
        unsafe {
            let image_ref = CGWindowListCreateImage(bounds, list_option, window_id, image_option);
            if !image_ref.is_null() {
                Some(CGImage::from_ptr(image_ref))
            } else {
                None
            }
        }
    }

    /// Returns a composite image of the specified windows.
    #[inline]
    pub fn screenshot_from_windows(
        bounds: CGRect,
        windows: CFArray,
        image_option: CGWindowImageOption,
    ) -> Option<CGImage> {
        unsafe {
            let image_ref = CGWindowListCreateImageFromArray(
                bounds,
                windows.as_concrete_TypeRef(),
                image_option,
            );
            if !image_ref.is_null() {
                Some(CGImage::from_ptr(image_ref))
            } else {
                None
            }
        }
    }

    /// Generates and returns information about the selected windows in the
    /// current user session.
    pub fn window_list_info(
        option: CGWindowListOption,
        relative_to_window: Option<CGWindowID>,
    ) -> Option<CFArray> {
        let relative_to_window = relative_to_window.unwrap_or(kCGNullWindowID);
        let array_ref = unsafe { CGWindowListCopyWindowInfo(option, relative_to_window) };
        if !array_ref.is_null() {
            Some(unsafe { TCFType::wrap_under_create_rule(array_ref) })
        } else {
            None
        }
    }

    /// Returns a Boolean value indicating whether a display is active.
    #[inline]
    pub fn is_active(&self) -> bool {
        unsafe { CGDisplayIsActive(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is always in a
    /// mirroring set.
    #[inline]
    pub fn is_always_in_mirror_set(&self) -> bool {
        unsafe { CGDisplayIsAlwaysInMirrorSet(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is sleeping (and is
    /// therefore not drawable.)
    #[inline]
    pub fn is_asleep(&self) -> bool {
        unsafe { CGDisplayIsAsleep(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is built-in, such as
    /// the internal display in portable systems.
    #[inline]
    pub fn is_builtin(&self) -> bool {
        unsafe { CGDisplayIsBuiltin(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is in a hardware
    /// mirroring set.
    #[inline]
    pub fn is_in_hw_mirror_set(&self) -> bool {
        unsafe { CGDisplayIsInHWMirrorSet(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is in a mirroring set.
    #[inline]
    pub fn is_in_mirror_set(&self) -> bool {
        unsafe { CGDisplayIsInMirrorSet(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is the main display.
    #[inline]
    pub fn is_main(&self) -> bool {
        unsafe { CGDisplayIsMain(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is connected or online.
    #[inline]
    pub fn is_online(&self) -> bool {
        unsafe { CGDisplayIsOnline(self.id) != 0 }
    }

    /// Returns a boolean indicating whether Quartz is using OpenGL-based
    /// window acceleration (Quartz Extreme) to render in a display.
    #[inline]
    pub fn uses_open_gl_acceleration(&self) -> bool {
        unsafe { CGDisplayUsesOpenGLAcceleration(self.id) != 0 }
    }

    /// Returns a boolean indicating whether a display is running in a stereo
    /// graphics mode.
    #[inline]
    pub fn is_stereo(&self) -> bool {
        unsafe { CGDisplayIsStereo(self.id) != 0 }
    }

    /// For a secondary display in a mirroring set, returns the primary
    /// display.
    #[inline]
    pub fn mirrors_display(&self) -> CGDirectDisplayID {
        unsafe { CGDisplayMirrorsDisplay(self.id) }
    }

    /// Returns the primary display in a hardware mirroring set.
    #[inline]
    pub fn primary_display(&self) -> CGDirectDisplayID {
        unsafe { CGDisplayPrimaryDisplay(self.id) }
    }

    /// Returns the rotation angle of a display in degrees.
    #[inline]
    pub fn rotation(&self) -> f64 {
        unsafe { CGDisplayRotation(self.id) }
    }

    /// Returns the width and height of a display in millimeters.
    #[inline]
    pub fn screen_size(&self) -> CGSize {
        unsafe { CGDisplayScreenSize(self.id) }
    }

    /// Returns the serial number of a display monitor.
    #[inline]
    pub fn serial_number(&self) -> u32 {
        unsafe { CGDisplaySerialNumber(self.id) }
    }

    /// Returns the logical unit number of a display.
    #[inline]
    pub fn unit_number(&self) -> u32 {
        unsafe { CGDisplayUnitNumber(self.id) }
    }

    /// Returns the vendor number of the specified display's monitor.
    #[inline]
    pub fn vendor_number(&self) -> u32 {
        unsafe { CGDisplayVendorNumber(self.id) }
    }

    /// Returns the model number of a display monitor.
    #[inline]
    pub fn model_number(&self) -> u32 {
        unsafe { CGDisplayModelNumber(self.id) }
    }

    /// Returns the display height in pixel units.
    #[inline]
    pub fn pixels_high(&self) -> u64 {
        unsafe { CGDisplayPixelsHigh(self.id) as u64 }
    }

    /// Returns the display width in pixel units.
    #[inline]
    pub fn pixels_wide(&self) -> u64 {
        unsafe { CGDisplayPixelsWide(self.id) as u64 }
    }

    /// Provides a list of displays that are active (or drawable).
    #[inline]
    pub fn active_displays() -> Result<Vec<CGDirectDisplayID>, CGError> {
        let expected_count = CGDisplay::active_display_count()?;
        let mut buf: Vec<CGDirectDisplayID> = vec![0; expected_count as usize];

        let mut actual_count: u32 = 0;

        let result =
            unsafe { CGGetActiveDisplayList(expected_count, buf.as_mut_ptr(), &mut actual_count) };
        if result == 0 {
            buf.truncate(actual_count as usize);
            Ok(buf)
        } else {
            Err(result)
        }
    }

    /// Provides count of displays that are active (or drawable).
    #[inline]
    pub fn active_display_count() -> Result<u32, CGError> {
        let mut count: u32 = 0;
        let result = unsafe { CGGetActiveDisplayList(0, ptr::null_mut(), &mut count) };
        if result == 0 {
            Ok(count)
        } else {
            Err(result)
        }
    }

    /// Hides the mouse cursor, and increments the hide cursor count.
    #[inline]
    pub fn hide_cursor(&self) -> Result<(), CGError> {
        let result = unsafe { CGDisplayHideCursor(self.id) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Decrements the hide cursor count, and shows the mouse cursor if the
    /// count is 0.
    #[inline]
    pub fn show_cursor(&self) -> Result<(), CGError> {
        let result = unsafe { CGDisplayShowCursor(self.id) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Moves the mouse cursor to a specified point relative to the display
    /// origin (the upper-left corner of the display).
    #[inline]
    pub fn move_cursor_to_point(&self, point: CGPoint) -> Result<(), CGError> {
        let result = unsafe { CGDisplayMoveCursorToPoint(self.id, point) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Moves the mouse cursor without generating events.
    #[inline]
    pub fn warp_mouse_cursor_position(point: CGPoint) -> Result<(), CGError> {
        let result = unsafe { CGWarpMouseCursorPosition(point) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }

    /// Connects or disconnects the mouse and cursor while an application is
    /// in the foreground.
    #[inline]
    pub fn associate_mouse_and_mouse_cursor_position(connected: bool) -> Result<(), CGError> {
        let result = unsafe { CGAssociateMouseAndMouseCursorPosition(connected as boolean_t) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

impl CGDisplayMode {
    /// Returns all display modes for the specified display id.
    pub fn all_display_modes(
        display_id: CGDirectDisplayID,
        options: CFDictionaryRef,
    ) -> Option<Vec<CGDisplayMode>> {
        let array_opt: Option<CFArray> = unsafe {
            let array_ref = CGDisplayCopyAllDisplayModes(display_id, options);
            if !array_ref.is_null() {
                Some(CFArray::wrap_under_create_rule(array_ref))
            } else {
                None
            }
        };
        match array_opt {
            Some(modes) => {
                let vec: Vec<CGDisplayMode> = modes
                    .into_iter()
                    .map(|value0| {
                        let x = *value0.deref() as *mut crate::sys::CGDisplayMode;
                        unsafe { CGDisplayMode::from_ptr(x) }
                    })
                    .collect();
                Some(vec)
            }
            None => None,
        }
    }

    /// Returns the height of the specified display mode.
    #[inline]
    pub fn height(&self) -> u64 {
        unsafe { CGDisplayModeGetHeight(self.as_ptr()) as u64 }
    }

    /// Returns the width of the specified display mode.
    #[inline]
    pub fn width(&self) -> u64 {
        unsafe { CGDisplayModeGetWidth(self.as_ptr()) as u64 }
    }

    /// Returns the pixel height of the specified display mode.
    #[inline]
    pub fn pixel_height(&self) -> u64 {
        unsafe { CGDisplayModeGetPixelHeight(self.as_ptr()) as u64 }
    }

    /// Returns the pixel width of the specified display mode.
    #[inline]
    pub fn pixel_width(&self) -> u64 {
        unsafe { CGDisplayModeGetPixelWidth(self.as_ptr()) as u64 }
    }

    #[inline]
    pub fn refresh_rate(&self) -> f64 {
        unsafe { CGDisplayModeGetRefreshRate(self.as_ptr()) }
    }

    /// Returns the I/O Kit flags of the specified display mode.
    #[inline]
    pub fn io_flags(&self) -> u32 {
        unsafe { CGDisplayModeGetIOFlags(self.as_ptr()) }
    }

    /// Returns the pixel encoding of the specified display mode.
    #[inline]
    pub fn pixel_encoding(&self) -> CFString {
        unsafe { CFString::wrap_under_create_rule(CGDisplayModeCopyPixelEncoding(self.as_ptr())) }
    }

    /// Returns the number of bits per pixel of the specified display mode.
    pub fn bit_depth(&self) -> usize {
        let pixel_encoding = self.pixel_encoding().to_string();
        // my numerical representation for kIO16BitFloatPixels and kIO32bitFloatPixels
        // are made up and possibly non-sensical
        if pixel_encoding.eq_ignore_ascii_case(kIO32BitFloatPixels) {
            96
        } else if pixel_encoding.eq_ignore_ascii_case(kIO64BitDirectPixels) {
            64
        } else if pixel_encoding.eq_ignore_ascii_case(kIO16BitFloatPixels) {
            48
        } else if pixel_encoding.eq_ignore_ascii_case(IO32BitDirectPixels) {
            32
        } else if pixel_encoding.eq_ignore_ascii_case(kIO30BitDirectPixels) {
            30
        } else if pixel_encoding.eq_ignore_ascii_case(IO16BitDirectPixels) {
            16
        } else if pixel_encoding.eq_ignore_ascii_case(IO8BitIndexedPixels) {
            8
        } else {
            0
        }
    }

    pub fn mode_id(&self) -> i32 {
        unsafe { CGDisplayModeGetIODisplayModeID(self.as_ptr()) }
    }
}

#[cfg_attr(feature = "link", link(name = "CoreGraphics", kind = "framework"))]
extern "C" {
    pub static CGRectNull: CGRect;
    pub static CGRectInfinite: CGRect;

    pub static kCGDisplayShowDuplicateLowResolutionModes: CFStringRef;

    pub fn CGDisplayModeRetain(mode: crate::sys::CGDisplayModeRef);
    pub fn CGDisplayModeRelease(mode: crate::sys::CGDisplayModeRef);

    pub fn CGMainDisplayID() -> CGDirectDisplayID;
    pub fn CGDisplayIsActive(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsAlwaysInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsAsleep(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsBuiltin(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsInHWMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsMain(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsOnline(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsStereo(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayMirrorsDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
    pub fn CGDisplayPrimaryDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
    pub fn CGDisplayRotation(display: CGDirectDisplayID) -> libc::c_double;
    pub fn CGDisplayScreenSize(display: CGDirectDisplayID) -> CGSize;
    pub fn CGDisplaySerialNumber(display: CGDirectDisplayID) -> u32;
    pub fn CGDisplayUnitNumber(display: CGDirectDisplayID) -> u32;
    pub fn CGDisplayUsesOpenGLAcceleration(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayVendorNumber(display: CGDirectDisplayID) -> u32;
    pub fn CGGetActiveDisplayList(
        max_displays: u32,
        active_displays: *mut CGDirectDisplayID,
        display_count: *mut u32,
    ) -> CGError;
    pub fn CGGetDisplaysWithRect(
        rect: CGRect,
        max_displays: u32,
        displays: *mut CGDirectDisplayID,
        matching_display_count: *mut u32,
    ) -> CGError;
    pub fn CGDisplayModelNumber(display: CGDirectDisplayID) -> u32;
    pub fn CGDisplayPixelsHigh(display: CGDirectDisplayID) -> libc::size_t;
    pub fn CGDisplayPixelsWide(display: CGDirectDisplayID) -> libc::size_t;
    pub fn CGDisplayBounds(display: CGDirectDisplayID) -> CGRect;
    pub fn CGDisplayCreateImage(display: CGDirectDisplayID) -> crate::sys::CGImageRef;
    pub fn CGDisplayCreateImageForRect(
        display: CGDirectDisplayID,
        rect: CGRect,
    ) -> crate::sys::CGImageRef;

    // Capturing and Releasing Displays
    pub fn CGDisplayCapture(display: CGDirectDisplayID) -> CGError;
    pub fn CGDisplayRelease(display: CGDirectDisplayID) -> CGError;
    pub fn CGShieldingWindowLevel() -> CGWindowLevel;

    // Configuring Displays
    pub fn CGBeginDisplayConfiguration(config: *mut CGDisplayConfigRef) -> CGError;
    pub fn CGCancelDisplayConfiguration(config: CGDisplayConfigRef) -> CGError;
    pub fn CGCompleteDisplayConfiguration(
        config: CGDisplayConfigRef,
        option: CGConfigureOption,
    ) -> CGError;
    pub fn CGConfigureDisplayWithDisplayMode(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        mode: crate::sys::CGDisplayModeRef,
        options: CFDictionaryRef,
    ) -> CGError;
    pub fn CGConfigureDisplayMirrorOfDisplay(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        master: CGDirectDisplayID,
    ) -> CGError;
    pub fn CGConfigureDisplayOrigin(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        x: i32,
        y: i32,
    ) -> CGError;
    pub fn CGRestorePermanentDisplayConfiguration();
    pub fn CGDisplayRegisterReconfigurationCallback(
        callback: CGDisplayReconfigurationCallBack,
        user_info: *const libc::c_void,
    ) -> CGError;
    pub fn CGDisplayRemoveReconfigurationCallback(
        callback: CGDisplayReconfigurationCallBack,
        user_info: *const libc::c_void,
    ) -> CGError;

    pub fn CGDisplayCopyDisplayMode(display: CGDirectDisplayID) -> crate::sys::CGDisplayModeRef;
    pub fn CGDisplayModeGetHeight(mode: crate::sys::CGDisplayModeRef) -> libc::size_t;
    pub fn CGDisplayModeGetWidth(mode: crate::sys::CGDisplayModeRef) -> libc::size_t;
    pub fn CGDisplayModeGetPixelHeight(mode: crate::sys::CGDisplayModeRef) -> libc::size_t;
    pub fn CGDisplayModeGetPixelWidth(mode: crate::sys::CGDisplayModeRef) -> libc::size_t;
    pub fn CGDisplayModeGetRefreshRate(mode: crate::sys::CGDisplayModeRef) -> libc::c_double;
    pub fn CGDisplayModeGetIOFlags(mode: crate::sys::CGDisplayModeRef) -> u32;
    pub fn CGDisplayModeCopyPixelEncoding(mode: crate::sys::CGDisplayModeRef) -> CFStringRef;
    pub fn CGDisplayModeGetIODisplayModeID(mode: crate::sys::CGDisplayModeRef) -> i32;

    pub fn CGDisplayCopyAllDisplayModes(
        display: CGDirectDisplayID,
        options: CFDictionaryRef,
    ) -> CFArrayRef;
    pub fn CGDisplaySetDisplayMode(
        display: CGDirectDisplayID,
        mode: crate::sys::CGDisplayModeRef,
        options: CFDictionaryRef,
    ) -> CGError;

    // mouse stuff
    pub fn CGDisplayHideCursor(display: CGDirectDisplayID) -> CGError;
    pub fn CGDisplayShowCursor(display: CGDirectDisplayID) -> CGError;
    pub fn CGDisplayMoveCursorToPoint(display: CGDirectDisplayID, point: CGPoint) -> CGError;
    pub fn CGWarpMouseCursorPosition(point: CGPoint) -> CGError;
    pub fn CGAssociateMouseAndMouseCursorPosition(connected: boolean_t) -> CGError;

    // Display Fade Effects
    pub fn CGConfigureDisplayFadeEffect(
        config: CGDisplayConfigRef,
        fadeOutSeconds: CGDisplayFadeInterval,
        fadeInSeconds: CGDisplayFadeInterval,
        fadeRed: f32,
        fadeGreen: f32,
        fadeBlue: f32,
    ) -> CGError;
    pub fn CGAcquireDisplayFadeReservation(
        seconds: CGDisplayReservationInterval,
        token: *mut CGDisplayFadeReservationToken,
    ) -> CGError;
    pub fn CGDisplayFade(
        token: CGDisplayFadeReservationToken,
        duration: CGDisplayFadeInterval,
        startBlend: CGDisplayBlendFraction,
        endBlend: CGDisplayBlendFraction,
        redBlend: f32,
        greenBlend: f32,
        blueBlend: f32,
        synchronous: boolean_t,
    ) -> CGError;
    // CGDisplayFadeOperationInProgress
    pub fn CGReleaseDisplayFadeReservation(token: CGDisplayFadeReservationToken) -> CGError;

    // Window Services Reference
    pub fn CGWindowListCopyWindowInfo(
        option: CGWindowListOption,
        relativeToWindow: CGWindowID,
    ) -> CFArrayRef;
    pub fn CGWindowListCreateImage(
        screenBounds: CGRect,
        listOptions: CGWindowListOption,
        windowId: CGWindowID,
        imageOptions: CGWindowImageOption,
    ) -> crate::sys::CGImageRef;
    pub fn CGWindowListCreateImageFromArray(
        screenBounds: CGRect,
        windowArray: CFArrayRef,
        imageOptions: CGWindowImageOption,
    ) -> crate::sys::CGImageRef;
}
