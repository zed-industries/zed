use std::ptr::null_mut;

use bitflags::bitflags;
use core_foundation::dictionary::CFDictionaryRef;
use libc::{c_double, c_void};

use crate::{
    base::boolean_t,
    color_space::CGColorSpaceRef,
    direct_display::{CGDirectDisplayID, CGDisplayModeRef},
    error::CGError,
    geometry::CGSize,
};

#[repr(C)]
pub struct _CGDisplayConfigRef(c_void);

pub type CGDisplayConfigRef = *mut _CGDisplayConfigRef;

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct CGConfigureOption: u32 {
        #[doc(alias = "kCGConfigureForAppOnly")]
        const ForAppOnly = 0;
        #[doc(alias = "kCGConfigureForSession")]
        const ForSession = 1;
        #[doc(alias = "kCGConfigurePermanently")]
        const Permanently = 2;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct CGDisplayChangeSummaryFlags: u32 {
        #[doc(alias = "kCGDisplayBeginConfigurationFlag")]
        const BeginConfigurationFlag  = 1 << 0;
        #[doc(alias = "kCGDisplayMovedFlag")]
        const MovedFlag               = 1 << 1;
        #[doc(alias = "kCGDisplaySetMainFlag")]
        const SetMainFlag             = 1 << 2;
        #[doc(alias = "kCGDisplaySetModeFlag")]
        const SetModeFlag             = 1 << 3;
        #[doc(alias = "kCGDisplayAddFlag")]
        const AddFlag                 = 1 << 4;
        #[doc(alias = "kCGDisplayRemoveFlag")]
        const RemoveFlag              = 1 << 5;
        #[doc(alias = "kCGDisplayEnabledFlag")]
        const EnabledFlag             = 1 << 8;
        #[doc(alias = "kCGDisplayDisabledFlag")]
        const DisabledFlag            = 1 << 9;
        #[doc(alias = "kCGDisplayMirrorFlag")]
        const MirrorFlag              = 1 << 10;
        #[doc(alias = "kCGDisplayUnMirrorFlag")]
        const UnMirrorFlag            = 1 << 11;
        #[doc(alias = "kCGDisplayDesktopShapeChangedFlag")]
        const DesktopShapeChangedFlag = 1 << 12;
    }
}

pub type CGDisplayReconfigurationCallBack = extern "C" fn(display: CGDirectDisplayID, flags: CGDisplayChangeSummaryFlags, userInfo: *mut c_void);

extern "C" {
    pub fn CGBeginDisplayConfiguration(config: *mut CGDisplayConfigRef) -> CGError;
    pub fn CGConfigureDisplayOrigin(config: CGDisplayConfigRef, display: CGDirectDisplayID, x: i32, y: i32) -> CGError;
    pub fn CGConfigureDisplayWithDisplayMode(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        mode: CGDisplayModeRef,
        options: CFDictionaryRef,
    ) -> CGError;
    pub fn CGConfigureDisplayStereoOperation(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        stereo: boolean_t,
        forceBlueLine: boolean_t,
    ) -> CGError;
    pub fn CGConfigureDisplayMirrorOfDisplay(config: CGDisplayConfigRef, display: CGDirectDisplayID, master: CGDirectDisplayID) -> CGError;
    pub fn CGCancelDisplayConfiguration(config: CGDisplayConfigRef) -> CGError;
    pub fn CGCompleteDisplayConfiguration(config: CGDisplayConfigRef, options: CGConfigureOption) -> CGError;
    pub fn CGRestorePermanentDisplayConfiguration();
    pub fn CGDisplayRegisterReconfigurationCallback(callback: CGDisplayReconfigurationCallBack, userInfo: *mut c_void) -> CGError;
    pub fn CGDisplayRemoveReconfigurationCallback(callback: CGDisplayReconfigurationCallBack, userInfo: *mut c_void) -> CGError;
    pub fn CGDisplaySetStereoOperation(
        display: CGDirectDisplayID,
        stereo: boolean_t,
        forceBlueLine: boolean_t,
        options: CGConfigureOption,
    ) -> CGError;
    pub fn CGDisplayIsActive(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsAsleep(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsOnline(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsMain(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsBuiltin(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsAlwaysInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsInHWMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayMirrorsDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
    pub fn CGDisplayUsesOpenGLAcceleration(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsStereo(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayPrimaryDisplay(display: CGDirectDisplayID) -> CGDirectDisplayID;
    pub fn CGDisplayUnitNumber(display: CGDirectDisplayID) -> u32;
    pub fn CGDisplayVendorNumber(display: CGDirectDisplayID) -> u32;
    pub fn CGDisplayModelNumber(display: CGDirectDisplayID) -> u32;
    pub fn CGDisplaySerialNumber(display: CGDirectDisplayID) -> u32;
    pub fn CGDisplayScreenSize(display: CGDirectDisplayID) -> CGSize;
    pub fn CGDisplayRotation(display: CGDirectDisplayID) -> c_double;
    pub fn CGDisplayCopyColorSpace(display: CGDirectDisplayID) -> CGColorSpaceRef;
    pub fn CGConfigureDisplayMode(config: CGDisplayConfigRef, display: CGDirectDisplayID, mode: CFDictionaryRef) -> CGError;
}

pub fn begin_configuration() -> Result<CGDisplayConfigRef, CGError> {
    let mut config: CGDisplayConfigRef = null_mut();
    let result = unsafe { CGBeginDisplayConfiguration(&mut config) };
    if result == CGError::Success {
        Ok(config)
    } else {
        Err(result)
    }
}

pub fn configure_display_origin(config: &CGDisplayConfigRef, display: CGDirectDisplayID, x: i32, y: i32) -> Result<(), CGError> {
    let result = unsafe { CGConfigureDisplayOrigin(*config, display, x, y) };
    if result == CGError::Success {
        Ok(())
    } else {
        Err(result)
    }
}

pub fn configure_display_with_display_mode(
    config: &CGDisplayConfigRef,
    display: CGDirectDisplayID,
    mode: CGDisplayModeRef,
    options: CFDictionaryRef,
) -> Result<(), CGError> {
    let result = unsafe { CGConfigureDisplayWithDisplayMode(*config, display, mode, options) };
    if result == CGError::Success {
        Ok(())
    } else {
        Err(result)
    }
}

pub fn configure_display_stereo_operation(
    config: &CGDisplayConfigRef,
    display: CGDirectDisplayID,
    stereo: bool,
    force_blue_line: bool,
) -> Result<(), CGError> {
    let result = unsafe { CGConfigureDisplayStereoOperation(*config, display, stereo as boolean_t, force_blue_line as boolean_t) };
    if result == CGError::Success {
        Ok(())
    } else {
        Err(result)
    }
}

pub fn configure_display_mirror_of_display(
    config: &CGDisplayConfigRef,
    display: CGDirectDisplayID,
    master: CGDirectDisplayID,
) -> Result<(), CGError> {
    let result = unsafe { CGConfigureDisplayMirrorOfDisplay(*config, display, master) };
    if result == CGError::Success {
        Ok(())
    } else {
        Err(result)
    }
}

pub fn cancel_display_configuration(config: &CGDisplayConfigRef) -> Result<(), CGError> {
    let result = unsafe { CGCancelDisplayConfiguration(*config) };
    if result == CGError::Success {
        Ok(())
    } else {
        Err(result)
    }
}

pub fn complete_display_configuration(config: &CGDisplayConfigRef, options: CGConfigureOption) -> Result<(), CGError> {
    let result = unsafe { CGCompleteDisplayConfiguration(*config, options) };
    if result == CGError::Success {
        Ok(())
    } else {
        Err(result)
    }
}

pub fn restore_permanent_display_configuration() {
    unsafe { CGRestorePermanentDisplayConfiguration() };
}
