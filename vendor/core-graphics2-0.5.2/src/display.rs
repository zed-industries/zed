use std::ptr::null;

use core_foundation::{array::CFArray, base::TCFType, impl_CFTypeDescription, impl_TCFType, string::CFString};
use libc::c_double;

#[cfg(feature = "window")]
use crate::window::CGWindowID;
use crate::{
    color_space::CGColorSpace,
    context::CGContext,
    error::CGError,
    geometry::{CGPoint, CGRect, CGSize},
    image::CGImage,
};
pub use crate::{direct_display::*, display_configuration::*, display_fade::*, remote_operation::*};

// IOKit/graphics/IOGraphicsTypes.h
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

pub struct CGDisplayMode(CGDisplayModeRef);

impl Drop for CGDisplayMode {
    fn drop(&mut self) {
        unsafe { CGDisplayModeRelease(self.0) }
    }
}

impl_TCFType!(CGDisplayMode, CGDisplayModeRef, CGDisplayModeGetTypeID);
impl_CFTypeDescription!(CGDisplayMode);

impl CGDisplayMode {
    pub fn width(&self) -> usize {
        unsafe { CGDisplayModeGetWidth(self.as_concrete_TypeRef()) as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { CGDisplayModeGetHeight(self.as_concrete_TypeRef()) as usize }
    }

    pub fn pixel_width(&self) -> usize {
        unsafe { CGDisplayModeGetPixelWidth(self.as_concrete_TypeRef()) as usize }
    }

    pub fn pixel_height(&self) -> usize {
        unsafe { CGDisplayModeGetPixelHeight(self.as_concrete_TypeRef()) as usize }
    }

    pub fn copy_pixel_encoding(&self) -> Option<CFString> {
        unsafe {
            let encoding = CGDisplayModeCopyPixelEncoding(self.as_concrete_TypeRef());
            if encoding.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(encoding))
            }
        }
    }

    pub fn bit_depth(&self) -> usize {
        match self.copy_pixel_encoding() {
            None => 0,
            Some(encoding) => match encoding.to_string().as_str() {
                kIO32BitFloatPixels => 96,
                kIO64BitDirectPixels => 64,
                kIO16BitFloatPixels => 48,
                kIO30BitDirectPixels => 30,
                IO32BitDirectPixels => 32,
                IO16BitDirectPixels => 16,
                IOYUV422Pixels => 16,
                IO8BitOverlayPixels => 8,
                IO8BitIndexedPixels => 8,
                IO4BitIndexedPixels => 4,
                IO2BitIndexedPixels => 2,
                IO1BitIndexedPixels => 1,
                _ => 0,
            },
        }
    }

    pub fn refresh_rate(&self) -> f64 {
        unsafe { CGDisplayModeGetRefreshRate(self.as_concrete_TypeRef()) }
    }

    pub fn io_flags(&self) -> u32 {
        unsafe { CGDisplayModeGetIOFlags(self.as_concrete_TypeRef()) }
    }

    pub fn io_display_mode_id(&self) -> i32 {
        unsafe { CGDisplayModeGetIODisplayModeID(self.as_concrete_TypeRef()) }
    }

    pub fn is_usable_for_desktop_gui(&self) -> bool {
        unsafe { CGDisplayModeIsUsableForDesktopGUI(self.as_concrete_TypeRef()) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CGDisplay {
    pub id: CGDirectDisplayID,
}

impl CGDisplay {
    pub fn new(id: CGDirectDisplayID) -> Self {
        CGDisplay {
            id,
        }
    }

    pub fn main() -> Self {
        unsafe { CGDisplay::new(CGMainDisplayID()) }
    }

    pub fn copy_display_modes(&self) -> Option<CFArray<CGDisplayMode>> {
        unsafe {
            let modes = CGDisplayCopyAllDisplayModes(self.id, null());
            if modes.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(modes))
            }
        }
    }

    pub fn copy_display_mode(&self) -> Option<CGDisplayMode> {
        unsafe {
            let mode = CGDisplayCopyDisplayMode(self.id);
            if mode.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(mode))
            }
        }
    }

    pub fn bounds(&self) -> CGRect {
        unsafe { CGDisplayBounds(self.id) }
    }

    pub fn pixels_wide(&self) -> usize {
        unsafe { CGDisplayPixelsWide(self.id) as usize }
    }

    pub fn pixels_high(&self) -> usize {
        unsafe { CGDisplayPixelsHigh(self.id) as usize }
    }

    pub fn set_display_mode(&self, mode: &CGDisplayMode) -> Result<(), CGError> {
        let result = unsafe { CGDisplaySetDisplayMode(self.id, mode.as_concrete_TypeRef(), null()) };
        if result == CGError::Success {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn capture(&self) -> Result<(), CGError> {
        let result = unsafe { CGDisplayCapture(self.id) };
        if result == CGError::Success {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn capture_with_options(&self, options: CGCaptureOptions) -> Result<(), CGError> {
        let result = unsafe { CGDisplayCaptureWithOptions(self.id, options) };
        if result == CGError::Success {
            Ok(())
        } else {
            Err(result)
        }
    }

    #[cfg(feature = "window")]
    pub fn shielding_window_id(&self) -> CGWindowID {
        unsafe { CGShieldingWindowID(self.id) }
    }

    pub fn new_image(&self) -> Option<CGImage> {
        unsafe {
            let image = CGDisplayCreateImage(self.id);
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn new_image_for_rect(&self, rect: CGRect) -> Option<CGImage> {
        unsafe {
            let image = CGDisplayCreateImageForRect(self.id, rect);
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn hide_cursor(&self) -> Result<(), CGError> {
        let result = unsafe { CGDisplayHideCursor(self.id) };
        if result == CGError::Success {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn show_cursor(&self) -> Result<(), CGError> {
        let result = unsafe { CGDisplayShowCursor(self.id) };
        if result == CGError::Success {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn move_cursor_to_point(&self, point: CGPoint) -> Result<(), CGError> {
        let result = unsafe { CGDisplayMoveCursorToPoint(self.id, point) };
        if result == CGError::Success {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn warp_mouse_cursor_position(&self, point: CGPoint) -> Result<(), CGError> {
        let result = unsafe { CGWarpMouseCursorPosition(point) };
        if result == CGError::Success {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn get_drawing_context(&self) -> Option<CGContext> {
        unsafe {
            let context = CGDisplayGetDrawingContext(self.id);
            if context.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(context))
            }
        }
    }

    // display configuration

    pub fn set_stereo_operation(&self, stereo: bool, force_blue_line: bool, option: CGConfigureOption) -> Result<(), CGError> {
        let result = unsafe { CGDisplaySetStereoOperation(self.id, stereo as _, force_blue_line as _, option) };
        if result == CGError::Success {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe { CGDisplayIsActive(self.id) != 0 }
    }

    pub fn is_asleep(&self) -> bool {
        unsafe { CGDisplayIsAsleep(self.id) != 0 }
    }

    pub fn is_online(&self) -> bool {
        unsafe { CGDisplayIsOnline(self.id) != 0 }
    }

    pub fn is_main(&self) -> bool {
        unsafe { CGDisplayIsMain(self.id) != 0 }
    }

    pub fn is_built_in(&self) -> bool {
        unsafe { CGDisplayIsBuiltin(self.id) != 0 }
    }

    pub fn is_in_mirror_set(&self) -> bool {
        unsafe { CGDisplayIsInMirrorSet(self.id) != 0 }
    }

    pub fn is_always_in_mirror_set(&self) -> bool {
        unsafe { CGDisplayIsInHWMirrorSet(self.id) != 0 }
    }

    pub fn is_in_hw_mirror_set(&self) -> bool {
        unsafe { CGDisplayIsInHWMirrorSet(self.id) != 0 }
    }

    pub fn is_stereo(&self) -> bool {
        unsafe { CGDisplayIsStereo(self.id) != 0 }
    }

    pub fn uses_opengl_acceleration(&self) -> bool {
        unsafe { CGDisplayUsesOpenGLAcceleration(self.id) != 0 }
    }

    pub fn mirrors_display(&self) -> Self {
        unsafe { CGDisplay::new(CGDisplayMirrorsDisplay(self.id)) }
    }

    pub fn primary_display(&self) -> Self {
        unsafe { CGDisplay::new(CGDisplayPrimaryDisplay(self.id)) }
    }

    pub fn unit_number(&self) -> u32 {
        unsafe { CGDisplayUnitNumber(self.id) }
    }

    pub fn vendor_number(&self) -> u32 {
        unsafe { CGDisplayVendorNumber(self.id) }
    }

    pub fn model_number(&self) -> u32 {
        unsafe { CGDisplayModelNumber(self.id) }
    }

    pub fn serial_number(&self) -> u32 {
        unsafe { CGDisplaySerialNumber(self.id) }
    }

    pub fn screen_size(&self) -> CGSize {
        unsafe { CGDisplayScreenSize(self.id) }
    }

    pub fn rotation(&self) -> c_double {
        unsafe { CGDisplayRotation(self.id) }
    }

    pub fn copy_color_space(&self) -> Option<CGColorSpace> {
        unsafe {
            let space = CGDisplayCopyColorSpace(self.id);
            if space.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(space))
            }
        }
    }
}

fn get_displays_from_ids(ids: &mut Vec<CGDirectDisplayID>, count: usize) -> Option<Vec<CGDisplay>> {
    ids.truncate(count as usize);
    Some(ids.into_iter().map(|id| CGDisplay::new(*id)).collect())
}

pub fn get_displays_with_point(point: CGPoint, max_displays: usize) -> Option<Vec<CGDisplay>> {
    unsafe {
        let mut count = max_displays as u32;
        let mut ids: Vec<CGDirectDisplayID> = vec![0; max_displays];
        let result = CGGetDisplaysWithPoint(point, count, ids.as_mut_ptr(), &mut count);
        if result == CGError::Success {
            get_displays_from_ids(&mut ids, count as usize)
        } else {
            None
        }
    }
}

pub fn get_displays_with_rect(rect: CGRect, max_displays: usize) -> Option<Vec<CGDisplay>> {
    unsafe {
        let mut count = max_displays as u32;
        let mut ids = vec![0; max_displays];
        let result = CGGetDisplaysWithRect(rect, count, ids.as_mut_ptr(), &mut count);
        if result == CGError::Success {
            get_displays_from_ids(&mut ids, count as usize)
        } else {
            None
        }
    }
}

pub fn get_displays_with_opengl_display_mask(mask: CGOpenGLDisplayMask, max_displays: usize) -> Option<Vec<CGDisplay>> {
    unsafe {
        let mut count = max_displays as u32;
        let mut ids = vec![0; max_displays];
        let result = CGGetDisplaysWithOpenGLDisplayMask(mask, count, ids.as_mut_ptr(), &mut count);
        if result == CGError::Success {
            get_displays_from_ids(&mut ids, count as usize)
        } else {
            None
        }
    }
}

pub fn get_active_display_list(max_displays: usize) -> Option<Vec<CGDisplay>> {
    unsafe {
        let mut count = max_displays as u32;
        let mut ids = vec![0; max_displays];
        let result = CGGetActiveDisplayList(count, ids.as_mut_ptr(), &mut count);
        if result == CGError::Success {
            get_displays_from_ids(&mut ids, count as usize)
        } else {
            None
        }
    }
}

pub fn get_online_display_list(max_displays: usize) -> Option<Vec<CGDisplay>> {
    unsafe {
        let mut count = max_displays as u32;
        let mut ids = vec![0; max_displays];
        let result = CGGetOnlineDisplayList(count, ids.as_mut_ptr(), &mut count);
        if result == CGError::Success {
            get_displays_from_ids(&mut ids, count as usize)
        } else {
            None
        }
    }
}

pub fn display_id_to_opengl_display_mask(display_id: CGDirectDisplayID) -> CGOpenGLDisplayMask {
    unsafe { CGDisplayIDToOpenGLDisplayMask(display_id) }
}

pub fn opengl_display_mask_to_display_id(mask: CGOpenGLDisplayMask) -> CGDirectDisplayID {
    unsafe { CGOpenGLDisplayMaskToDisplayID(mask) }
}

pub fn capture_all_displays() -> Result<(), CGError> {
    let result = unsafe { CGCaptureAllDisplays() };
    if result == CGError::Success {
        Ok(())
    } else {
        Err(result)
    }
}

pub fn capture_all_displays_with_options(options: CGCaptureOptions) -> Result<(), CGError> {
    let result = unsafe { CGCaptureAllDisplaysWithOptions(options) };
    if result == CGError::Success {
        Ok(())
    } else {
        Err(result)
    }
}

pub fn release_all_displays() -> Result<(), CGError> {
    let result = unsafe { CGReleaseAllDisplays() };
    if result == CGError::Success {
        Ok(())
    } else {
        Err(result)
    }
}
