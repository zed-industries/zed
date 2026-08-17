use bitflags::bitflags;
use core_foundation::{array::CFArrayRef, base::CFTypeID, dictionary::CFDictionaryRef, string::CFStringRef};
use libc::{c_double, c_float, c_void, size_t};

use crate::{
    context::CGContextRef,
    error::CGError,
    geometry::{CGPoint, CGRect},
    image::CGImageRef,
};
#[cfg(feature = "window")]
use crate::{window::CGWindowID, window_level::CGWindowLevel};

pub type CGDirectDisplayID = u32;
pub type CGOpenGLDisplayMask = u32;
pub type CGRefreshRate = c_double;
pub type CGGammaValue = c_float;

#[repr(C)]
pub struct __CGDisplayMode(c_void);

pub type CGDisplayModeRef = *mut __CGDisplayMode;

pub const kCGNullDirectDisplay: CGDirectDisplayID = 0;

extern "C" {
    pub fn CGMainDisplayID() -> CGDirectDisplayID;
    pub fn CGGetDisplaysWithPoint(point: CGPoint, maxDisplays: u32, displays: *mut CGDirectDisplayID, matchingDisplayCount: *mut u32) -> CGError;
    pub fn CGGetDisplaysWithRect(rect: CGRect, maxDisplays: u32, displays: *mut CGDirectDisplayID, matchingDisplayCount: *mut u32) -> CGError;
    pub fn CGGetDisplaysWithOpenGLDisplayMask(
        mask: CGOpenGLDisplayMask,
        maxDisplays: u32,
        displays: *mut CGDirectDisplayID,
        matchingDisplayCount: *mut u32,
    ) -> CGError;
    pub fn CGGetActiveDisplayList(maxDisplays: u32, activeDisplays: *mut CGDirectDisplayID, displayCount: *mut u32) -> CGError;
    pub fn CGGetOnlineDisplayList(maxDisplays: u32, onlineDisplays: *mut CGDirectDisplayID, displayCount: *mut u32) -> CGError;
    pub fn CGDisplayIDToOpenGLDisplayMask(display: CGDirectDisplayID) -> CGOpenGLDisplayMask;
    pub fn CGOpenGLDisplayMaskToDisplayID(mask: CGOpenGLDisplayMask) -> CGDirectDisplayID;
    pub fn CGDisplayBounds(display: CGDirectDisplayID) -> CGRect;
    pub fn CGDisplayPixelsWide(display: CGDirectDisplayID) -> size_t;
    pub fn CGDisplayPixelsHigh(display: CGDirectDisplayID) -> size_t;
    pub fn CGDisplayCopyAllDisplayModes(display: CGDirectDisplayID, options: CFDictionaryRef) -> CFArrayRef;
    pub fn CGDisplayCopyDisplayMode(display: CGDirectDisplayID) -> CGDisplayModeRef;
    pub fn CGDisplaySetDisplayMode(display: CGDirectDisplayID, mode: CGDisplayModeRef, options: CFDictionaryRef) -> CGError;
    pub fn CGDisplayModeGetWidth(mode: CGDisplayModeRef) -> size_t;
    pub fn CGDisplayModeGetHeight(mode: CGDisplayModeRef) -> size_t;
    pub fn CGDisplayModeCopyPixelEncoding(mode: CGDisplayModeRef) -> CFStringRef;
    pub fn CGDisplayModeGetRefreshRate(mode: CGDisplayModeRef) -> CGRefreshRate;
    pub fn CGDisplayModeGetIOFlags(mode: CGDisplayModeRef) -> u32;
    pub fn CGDisplayModeGetIODisplayModeID(mode: CGDisplayModeRef) -> i32;
    pub fn CGDisplayModeIsUsableForDesktopGUI(mode: CGDisplayModeRef) -> bool;
    pub fn CGDisplayModeGetTypeID() -> CFTypeID;
    pub fn CGDisplayModeRetain(mode: CGDisplayModeRef) -> CGDisplayModeRef;
    pub fn CGDisplayModeRelease(mode: CGDisplayModeRef);
    pub fn CGDisplayModeGetPixelWidth(mode: CGDisplayModeRef) -> size_t;
    pub fn CGDisplayModeGetPixelHeight(mode: CGDisplayModeRef) -> size_t;
    pub fn CGSetDisplayTransferByFormula(
        display: CGDirectDisplayID,
        redMin: CGGammaValue,
        redMax: CGGammaValue,
        redGamma: CGGammaValue,
        greenMin: CGGammaValue,
        greenMax: CGGammaValue,
        greenGamma: CGGammaValue,
        blueMin: CGGammaValue,
        blueMax: CGGammaValue,
        blueGamma: CGGammaValue,
    ) -> CGError;
    pub fn CGGetDisplayTransferByFormula(
        display: CGDirectDisplayID,
        redMin: *mut CGGammaValue,
        redMax: *mut CGGammaValue,
        redGamma: *mut CGGammaValue,
        greenMin: *mut CGGammaValue,
        greenMax: *mut CGGammaValue,
        greenGamma: *mut CGGammaValue,
        blueMin: *mut CGGammaValue,
        blueMax: *mut CGGammaValue,
        blueGamma: *mut CGGammaValue,
    ) -> CGError;
    pub fn CGDisplayGammaTableCapacity(display: CGDirectDisplayID) -> u32;
    pub fn CGSetDisplayTransferByTable(
        display: CGDirectDisplayID,
        tableSize: u32,
        redTable: *const CGGammaValue,
        greenTable: *const CGGammaValue,
        blueTable: *const CGGammaValue,
    ) -> CGError;
    pub fn CGGetDisplayTransferByTable(
        display: CGDirectDisplayID,
        capacity: u32,
        redTable: *mut CGGammaValue,
        greenTable: *mut CGGammaValue,
        blueTable: *mut CGGammaValue,
        sampleCount: *mut u32,
    ) -> CGError;
    pub fn CGSetDisplayTransferByByteTable(
        display: CGDirectDisplayID,
        tableSize: u32,
        redTable: *const u8,
        greenTable: *const u8,
        blueTable: *const u8,
    ) -> CGError;
    pub fn CGDisplayRestoreColorSyncSettings();

    pub static kCGDisplayShowDuplicateLowResolutionModes: CFDictionaryRef;
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct CGCaptureOptions: u32 {
        #[doc(alias = "kCGCaptureNoOptions")]
        const NoOptions = 0;
        #[doc(alias = "kCGCaptureNoFill")]
        const NoFill = 1;
    }
}

extern "C" {
    pub fn CGDisplayCapture(display: CGDirectDisplayID) -> CGError;
    pub fn CGDisplayCaptureWithOptions(display: CGDirectDisplayID, options: CGCaptureOptions) -> CGError;
    pub fn CGDisplayRelease(display: CGDirectDisplayID) -> CGError;
    pub fn CGCaptureAllDisplays() -> CGError;
    pub fn CGCaptureAllDisplaysWithOptions(options: CGCaptureOptions) -> CGError;
    pub fn CGReleaseAllDisplays() -> CGError;
    #[cfg(feature = "window")]
    pub fn CGShieldingWindowID(display: CGDirectDisplayID) -> CGWindowID;
    #[cfg(feature = "window")]
    pub fn CGShieldingWindowLevel() -> CGWindowLevel;
    pub fn CGDisplayCreateImage(display: CGDirectDisplayID) -> CGImageRef;
    pub fn CGDisplayCreateImageForRect(display: CGDirectDisplayID, rect: CGRect) -> CGImageRef;
    pub fn CGDisplayHideCursor(display: CGDirectDisplayID) -> CGError;
    pub fn CGDisplayShowCursor(display: CGDirectDisplayID) -> CGError;
    pub fn CGDisplayMoveCursorToPoint(display: CGDirectDisplayID, point: CGPoint) -> CGError;
    pub fn CGGetLastMouseDelta(deltaX: *mut i32, deltaY: *mut i32) -> CGError;
    pub fn CGDisplayGetDrawingContext(display: CGDirectDisplayID) -> CGContextRef;
}
