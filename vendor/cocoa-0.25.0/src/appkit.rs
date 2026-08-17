// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use base::{id, BOOL, SEL};
use block::Block;
use foundation::{NSInteger, NSUInteger, NSTimeInterval,
                 NSPoint, NSSize, NSRect, NSRange, NSRectEdge};
use libc;

pub use core_graphics::base::CGFloat;
pub use core_graphics::geometry::CGPoint;

pub use self::NSApplicationActivationPolicy::*;
pub use self::NSApplicationActivationOptions::*;
pub use self::NSBackingStoreType::*;
pub use self::NSOpenGLPixelFormatAttribute::*;
pub use self::NSOpenGLPFAOpenGLProfiles::*;
pub use self::NSEventType::*;
use std::os::raw::c_void;

pub type CGLContextObj = *mut c_void;

pub type GLint = i32;

#[link(name = "AppKit", kind = "framework")]
extern {
    pub static NSAppKitVersionNumber: f64;

    // Types for Standard Data - OS X v10.6 and later. (NSString *const)
    pub static NSPasteboardTypeString: id;
    pub static NSPasteboardTypePDF: id;
    pub static NSPasteboardTypeTIFF: id;
    pub static NSPasteboardTypePNG: id;
    pub static NSPasteboardTypeRTF: id;
    pub static NSPasteboardTypeRTFD: id;
    pub static NSPasteboardTypeHTML: id;
    pub static NSPasteboardTypeTabularText: id;
    pub static NSPasteboardTypeFont: id;
    pub static NSPasteboardTypeRuler: id;
    pub static NSPasteboardTypeColor: id;
    pub static NSPasteboardTypeSound: id;
    pub static NSPasteboardTypeMultipleTextSelection: id;
    pub static NSPasteboardTypeFindPanelSearchOptions: id;

    // Types for Standard Data - OS X v10.5 and earlier. (NSString *)
    pub static NSStringPboardType: id;
    pub static NSFilenamesPboardType: id;
    pub static NSPostScriptPboardType: id;
    pub static NSTIFFPboardType: id;
    pub static NSRTFPboardType: id;
    pub static NSTabularTextPboardType: id;
    pub static NSFontPboardType: id;
    pub static NSRulerPboardType: id;
    pub static NSFileContentsPboardType: id;
    pub static NSColorPboardType: id;
    pub static NSRTFDPboardType: id;
    pub static NSHTMLPboardType: id;
    pub static NSPICTPboardType: id;
    pub static NSURLPboardType: id;
    pub static NSPDFPboardType: id;
    pub static NSVCardPboardType: id;
    pub static NSFilesPromisePboardType: id;
    pub static NSMultipleTextSelectionPboardType: id;
    pub static NSSoundPboardType: id;

    // Names of provided pasteboards. (NSString *)
    pub static NSGeneralPboard: id;
    pub static NSFontPboard: id;
    pub static NSRulerPboard: id;
    pub static NSFindPboard: id;
    pub static NSDragPboard: id;

    // Pasteboard reading options - OS X v10.6 and later. (NSString *)
    pub static NSPasteboardURLReadingFileURLsOnlyKey: id;
    pub static NSPasteboardURLReadingContentsConformToTypesKey: id;

    // NSAppearance names. (NSString *)
    pub static NSAppearanceNameVibrantDark: id;
    pub static NSAppearanceNameVibrantLight: id;

    // Fullscreen mode options - OS X v10.5 and later. (NSString *)
    pub static NSFullScreenModeAllScreens: id;
    pub static NSFullScreenModeSetting: id;
    pub static NSFullScreenModeWindowLevel: id;
    pub static NSFullScreenModeApplicationPresentationOptions: id;
}

pub const NSAppKitVersionNumber10_0: f64 = 577.0;
pub const NSAppKitVersionNumber10_1: f64 = 620.0;
pub const NSAppKitVersionNumber10_2: f64 = 663.0;
pub const NSAppKitVersionNumber10_2_3: f64 = 663.6;
pub const NSAppKitVersionNumber10_3: f64 = 743.0;
pub const NSAppKitVersionNumber10_3_2: f64 = 743.14;
pub const NSAppKitVersionNumber10_3_3: f64 = 743.2;
pub const NSAppKitVersionNumber10_3_5: f64 = 743.24;
pub const NSAppKitVersionNumber10_3_7: f64 = 743.33;
pub const NSAppKitVersionNumber10_3_9: f64 = 743.36;
pub const NSAppKitVersionNumber10_4: f64 = 824.0;
pub const NSAppKitVersionNumber10_4_1: f64 = 824.1;
pub const NSAppKitVersionNumber10_4_3: f64 = 824.23;
pub const NSAppKitVersionNumber10_4_4: f64 = 824.33;
pub const NSAppKitVersionNumber10_4_7: f64 = 824.41;
pub const NSAppKitVersionNumber10_5: f64 = 949.0;
pub const NSAppKitVersionNumber10_5_2: f64 = 949.27;
pub const NSAppKitVersionNumber10_5_3: f64 = 949.33;
pub const NSAppKitVersionNumber10_6: f64 = 1038.0;
pub const NSAppKitVersionNumber10_7: f64 = 1138.0;
pub const NSAppKitVersionNumber10_7_2: f64 = 1138.23;
pub const NSAppKitVersionNumber10_7_3: f64 = 1138.32;
pub const NSAppKitVersionNumber10_7_4: f64 = 1138.47;
pub const NSAppKitVersionNumber10_8: f64 = 1187.0;
pub const NSAppKitVersionNumber10_9: f64 = 1265.0;
pub const NSAppKitVersionNumber10_10: f64 = 1343.0;
pub const NSAppKitVersionNumber10_10_2: f64 = 1344.0;
pub const NSAppKitVersionNumber10_10_3: f64 = 1347.0;
pub const NSAppKitVersionNumber10_10_4: f64 = 1348.0;
pub const NSAppKitVersionNumber10_10_5: f64 = 1348.0;
pub const NSAppKitVersionNumber10_10_Max: f64 = 1349.0;
pub const NSAppKitVersionNumber10_11: f64 = 1404.0;
pub const NSAppKitVersionNumber10_11_1: f64 = 1404.13;
pub const NSAppKitVersionNumber10_11_2: f64 = 1404.34;
pub const NSAppKitVersionNumber10_11_3: f64 = 1404.34;
pub const NSAppKitVersionNumber10_12: f64 = 1504.0;
pub const NSAppKitVersionNumber10_12_1: f64 = 1504.60;
pub const NSAppKitVersionNumber10_12_2: f64 = 1504.76;
pub const NSAppKitVersionNumber10_13: f64 = 1561.0;
pub const NSAppKitVersionNumber10_13_1: f64 = 1561.1;
pub const NSAppKitVersionNumber10_13_2: f64 = 1561.2;
pub const NSAppKitVersionNumber10_13_4: f64 = 1561.4;
pub const NSAppKitVersionNumber10_14: f64 = 1671.0;
pub const NSAppKitVersionNumber10_14_1: f64 = 1671.1;
pub const NSAppKitVersionNumber10_14_2: f64 = 1671.2;
pub const NSAppKitVersionNumber10_14_3: f64 = 1671.3;
pub const NSAppKitVersionNumber10_14_4: f64 = 1671.4;
pub const NSAppKitVersionNumber10_14_5: f64 = 1671.5;
pub const NSAppKitVersionNumber10_15: f64 = 1894.0;
pub const NSAppKitVersionNumber10_15_1: f64 = 1894.1;
pub const NSAppKitVersionNumber10_15_2: f64 = 1894.2;
pub const NSAppKitVersionNumber10_15_3: f64 = 1894.3;
pub const NSAppKitVersionNumber10_15_4: f64 = 1894.4;
pub const NSAppKitVersionNumber10_15_5: f64 = 1894.5;
pub const NSAppKitVersionNumber10_15_6: f64 = 1894.6;
pub const NSAppKitVersionNumber11_0: f64 = 2022.0;
pub const NSAppKitVersionNumber11_1: f64 = 2022.2;
pub const NSAppKitVersionNumber11_2: f64 = 2022.3;
pub const NSAppKitVersionNumber11_3: f64 = 2022.4;
pub const NSAppKitVersionNumber11_4: f64 = 2022.5;

pub unsafe fn NSApp() -> id {
    msg_send![class!(NSApplication), sharedApplication]
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationActivationPolicy {
    NSApplicationActivationPolicyRegular = 0,
    NSApplicationActivationPolicyAccessory = 1,
    NSApplicationActivationPolicyProhibited = 2,
    NSApplicationActivationPolicyERROR = -1
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationActivationOptions {
    NSApplicationActivateAllWindows = 1 << 0,
    NSApplicationActivateIgnoringOtherApps = 1 << 1
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSApplicationTerminateReply {
    NSTerminateCancel = 0,
    NSTerminateNow = 1,
    NSTerminateLater = 2,
}

bitflags! {
    pub struct NSApplicationPresentationOptions : NSUInteger {
        const NSApplicationPresentationDefault = 0;
        const NSApplicationPresentationAutoHideDock = 1 << 0;
        const NSApplicationPresentationHideDock = 1 << 1;
        const NSApplicationPresentationAutoHideMenuBar = 1 << 2;
        const NSApplicationPresentationHideMenuBar = 1 << 3;
        const NSApplicationPresentationDisableAppleMenu = 1 << 4;
        const NSApplicationPresentationDisableProcessSwitching = 1 << 5;
        const NSApplicationPresentationDisableForceQuit = 1 << 6;
        const NSApplicationPresentationDisableSessionTermination = 1 << 7;
        const NSApplicationPresentationDisableHideApplication = 1 << 8;
        const NSApplicationPresentationDisableMenuBarTransparency = 1 << 9;
        const NSApplicationPresentationFullScreen = 1 << 10;
        const NSApplicationPresentationAutoHideToolbar = 1 << 11;
    }
}

bitflags! {
    pub struct NSWindowStyleMask: NSUInteger {
        const NSBorderlessWindowMask      = 0;
        const NSTitledWindowMask          = 1 << 0;
        const NSClosableWindowMask        = 1 << 1;
        const NSMiniaturizableWindowMask  = 1 << 2;
        const NSResizableWindowMask       = 1 << 3;

        const NSTexturedBackgroundWindowMask  = 1 << 8;

        const NSUnifiedTitleAndToolbarWindowMask  = 1 << 12;

        const NSFullScreenWindowMask      = 1 << 14;

        const NSFullSizeContentViewWindowMask = 1 << 15;
    }
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSWindowTitleVisibility {
    NSWindowTitleVisible = 0,
    NSWindowTitleHidden = 1
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSWindowTabbingMode {
    NSWindowTabbingModeAutomatic = 0,
    NSWindowTabbingModeDisallowed = 1,
    NSWindowTabbingModePreferred = 2
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSBackingStoreType {
    NSBackingStoreRetained      = 0,
    NSBackingStoreNonretained   = 1,
    NSBackingStoreBuffered      = 2
}

pub enum NSWindowToolbarStyle {
    NSWindowToolbarStyleAutomatic      = 0,
    NSWindowToolbarStyleExpanded       = 1,
    NSWindowToolbarStylePreference     = 2,
    NSWindowToolbarStyleUnified        = 3,
    NSWindowToolbarStyleUnifiedCompact = 4,
}

bitflags! {
    pub struct NSWindowOrderingMode: NSInteger {
        const NSWindowAbove =  1;
        const NSWindowBelow = -1;
        const NSWindowOut   =  0;
    }
}

bitflags! {
    pub struct NSAlignmentOptions: libc::c_ulonglong {
        const NSAlignMinXInward         = 1 << 0;
        const NSAlignMinYInward         = 1 << 1;
        const NSAlignMaxXInward         = 1 << 2;
        const NSAlignMaxYInward         = 1 << 3;
        const NSAlignWidthInward        = 1 << 4;
        const NSAlignHeightInward       = 1 << 5;
        const NSAlignMinXOutward        = 1 << 8;
        const NSAlignMinYOutward        = 1 << 9;
        const NSAlignMaxXOutward        = 1 << 10;
        const NSAlignMaxYOutward        = 1 << 11;
        const NSAlignWidthOutward       = 1 << 12;
        const NSAlignHeightOutward      = 1 << 13;
        const NSAlignMinXNearest        = 1 << 16;
        const NSAlignMinYNearest        = 1 << 17;
        const NSAlignMaxXNearest        = 1 << 18;
        const NSAlignMaxYNearest        = 1 << 19;
        const NSAlignWidthNearest       = 1 << 20;
        const NSAlignHeightNearest      = 1 << 21;
        const NSAlignRectFlipped        = 1 << 63;
        const NSAlignAllEdgesInward     = NSAlignmentOptions::NSAlignMinXInward.bits
                                        | NSAlignmentOptions::NSAlignMaxXInward.bits
                                        | NSAlignmentOptions::NSAlignMinYInward.bits
                                        | NSAlignmentOptions::NSAlignMaxYInward.bits;
        const NSAlignAllEdgesOutward    = NSAlignmentOptions::NSAlignMinXOutward.bits
                                        | NSAlignmentOptions::NSAlignMaxXOutward.bits
                                        | NSAlignmentOptions::NSAlignMinYOutward.bits
                                        | NSAlignmentOptions::NSAlignMaxYOutward.bits;
        const NSAlignAllEdgesNearest    = NSAlignmentOptions::NSAlignMinXNearest.bits
                                        | NSAlignmentOptions::NSAlignMaxXNearest.bits
                                        | NSAlignmentOptions::NSAlignMinYNearest.bits
                                        | NSAlignmentOptions::NSAlignMaxYNearest.bits;
    }
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPixelFormatAttribute {
    NSOpenGLPFAAllRenderers             = 1,
    NSOpenGLPFATripleBuffer             = 3,
    NSOpenGLPFADoubleBuffer             = 5,
    NSOpenGLPFAStereo                   = 6,
    NSOpenGLPFAAuxBuffers               = 7,
    NSOpenGLPFAColorSize                = 8,
    NSOpenGLPFAAlphaSize                = 11,
    NSOpenGLPFADepthSize                = 12,
    NSOpenGLPFAStencilSize              = 13,
    NSOpenGLPFAAccumSize                = 14,
    NSOpenGLPFAMinimumPolicy            = 51,
    NSOpenGLPFAMaximumPolicy            = 52,
    NSOpenGLPFAOffScreen                = 53,
    NSOpenGLPFAFullScreen               = 54,
    NSOpenGLPFASampleBuffers            = 55,
    NSOpenGLPFASamples                  = 56,
    NSOpenGLPFAAuxDepthStencil          = 57,
    NSOpenGLPFAColorFloat               = 58,
    NSOpenGLPFAMultisample              = 59,
    NSOpenGLPFASupersample              = 60,
    NSOpenGLPFASampleAlpha              = 61,
    NSOpenGLPFARendererID               = 70,
    NSOpenGLPFASingleRenderer           = 71,
    NSOpenGLPFANoRecovery               = 72,
    NSOpenGLPFAAccelerated              = 73,
    NSOpenGLPFAClosestPolicy            = 74,
    NSOpenGLPFARobust                   = 75,
    NSOpenGLPFABackingStore             = 76,
    NSOpenGLPFAMPSafe                   = 78,
    NSOpenGLPFAWindow                   = 80,
    NSOpenGLPFAMultiScreen              = 81,
    NSOpenGLPFACompliant                = 83,
    NSOpenGLPFAScreenMask               = 84,
    NSOpenGLPFAPixelBuffer              = 90,
    NSOpenGLPFARemotePixelBuffer        = 91,
    NSOpenGLPFAAllowOfflineRenderers    = 96,
    NSOpenGLPFAAcceleratedCompute       = 97,
    NSOpenGLPFAOpenGLProfile            = 99,
    NSOpenGLPFAVirtualScreenCount       = 128,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPFAOpenGLProfiles {
    NSOpenGLProfileVersionLegacy = 0x1000,
    NSOpenGLProfileVersion3_2Core = 0x3200,
    NSOpenGLProfileVersion4_1Core = 0x4100,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLContextParameter {
    NSOpenGLCPSwapInterval          = 222,
    NSOpenGLCPSurfaceOrder          = 235,
    NSOpenGLCPSurfaceOpacity        = 236,
    NSOpenGLCPSurfaceBackingSize    = 304,
    NSOpenGLCPReclaimResources      = 308,
    NSOpenGLCPCurrentRendererID     = 309,
    NSOpenGLCPGPUVertexProcessing   = 310,
    NSOpenGLCPGPUFragmentProcessing = 311,
    NSOpenGLCPHasDrawable           = 314,
    NSOpenGLCPMPSwapsInFlight       = 315,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSWindowButton {
    NSWindowCloseButton            = 0,
    NSWindowMiniaturizeButton      = 1,
    NSWindowZoomButton             = 2,
    NSWindowToolbarButton          = 3,
    NSWindowDocumentIconButton     = 4,
    NSWindowDocumentVersionsButton = 6,
    NSWindowFullScreenButton       = 7,
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSBezelStyle {
    NSRoundedBezelStyle            = 1,
    NSRegularSquareBezelStyle      = 2,
    NSDisclosureBezelStyle         = 5,
    NSShadowlessSquareBezelStyle   = 6,
    NSCircularBezelStyle           = 7,
    NSTexturedSquareBezelStyle     = 8,
    NSHelpButtonBezelStyle         = 9,
    NSSmallSquareBezelStyle        = 10,
    NSTexturedRoundedBezelStyle    = 11,
    NSRoundRectBezelStyle          = 12,
    NSRecessedBezelStyle           = 13,
    NSRoundedDisclosureBezelStyle  = 14,
}

// https://developer.apple.com/documentation/appkit/nsvisualeffectview/blendingmode
#[allow(dead_code)]
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSVisualEffectBlendingMode {
    BehindWindow = 0,
    WithinWindow = 1
}

// https://developer.apple.com/documentation/appkit/nsvisualeffectview/state
#[allow(dead_code)]
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSVisualEffectState {
    FollowsWindowActiveState = 0,
    Active = 1,
    Inactive = 2
}

/// <https://developer.apple.com/documentation/appkit/nsvisualeffectview/material>
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSVisualEffectMaterial {
    /// A default material for the view's effectiveAppearance.
    #[deprecated = "Use a semantic material instead."]
    AppearanceBased = 0,
    #[deprecated = "Use a semantic material instead."]
    Light = 1,
    #[deprecated = "Use a semantic material instead."]
    Dark = 2,
    #[deprecated = "Use a semantic material instead."]
    MediumLight = 8,
    #[deprecated = "Use a semantic material instead."]
    UltraDark = 9,

    // macOS 10.10+
    Titlebar = 3,
    Selection = 4,

    // macOS 10.11+
    Menu = 5,
    Popover = 6,
    Sidebar = 7,

    // macOS 10.14+
    HeaderView = 10,
    Sheet = 11,
    WindowBackground = 12,
    HudWindow = 13,
    FullScreenUI = 15,
    Tooltip = 17,
    ContentBackground = 18,
    UnderWindowBackground = 21,
    UnderPageBackground = 22
}

// macOS 10.10+ - https://developer.apple.com/documentation/appkit/nsvisualeffectview
#[allow(non_snake_case)]
pub trait NSVisualEffectView: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSVisualEffectView), alloc]
    }

    unsafe fn init(self) -> id;
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
    unsafe fn bounds(self) -> NSRect;
    unsafe fn frame(self) -> NSRect;
    unsafe fn setFrameSize(self, frameSize: NSSize);
    unsafe fn setFrameOrigin(self, frameOrigin: NSPoint);

    unsafe fn superview(self) -> id;
    unsafe fn removeFromSuperview(self);
    // API_AVAILABLE(macos(10.12));
    unsafe fn isEmphasized(self) -> BOOL;
    // API_AVAILABLE(macos(10.12));
    unsafe fn setEmphasized_(self, emphasized: BOOL);

    unsafe fn setMaterial_(self, material: NSVisualEffectMaterial);
    unsafe fn setState_(self, state: NSVisualEffectState);
    unsafe fn setBlendingMode_(self, mode: NSVisualEffectBlendingMode);
}

#[allow(non_snake_case)]
impl NSVisualEffectView for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send![self, initWithFrame: frameRect]
    }

    unsafe fn bounds(self) -> NSRect {
        msg_send![self, bounds]
    }

    unsafe fn frame(self) -> NSRect {
        msg_send![self, frame]
    }

    unsafe fn setFrameSize(self, frameSize: NSSize) {
        msg_send![self, setFrameSize: frameSize]
    }

    unsafe fn setFrameOrigin(self, frameOrigin: NSPoint) {
        msg_send![self, setFrameOrigin: frameOrigin]
    }

    unsafe fn superview(self) -> id {
        msg_send![self, superview]
    }

    unsafe fn removeFromSuperview(self) {
        msg_send![self, removeFromSuperview]
    }

    // API_AVAILABLE(macos(10.12));
    unsafe fn isEmphasized(self) -> BOOL {
        msg_send![self, isEmphasized]
    }

    // API_AVAILABLE(macos(10.12));
    unsafe fn setEmphasized_(self, emphasized: BOOL) {
        msg_send![self, setEmphasized: emphasized]
    }

    unsafe fn setMaterial_(self, material: NSVisualEffectMaterial) {
        msg_send![self, setMaterial: material]
    }

    unsafe fn setState_(self, state: NSVisualEffectState) {
        msg_send![self, setState: state]
    }

    unsafe fn setBlendingMode_(self, mode: NSVisualEffectBlendingMode) {
        msg_send![self, setBlendingMode: mode]
    }
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSRequestUserAttentionType {
    NSCriticalRequest      = 0,
    NSInformationalRequest = 10,
}

pub static NSMainMenuWindowLevel: i32 = 24;

pub trait NSApplication: Sized {
    unsafe fn sharedApplication(_: Self) -> id {
        msg_send![class!(NSApplication), sharedApplication]
    }

    unsafe fn mainMenu(self) -> id;
    unsafe fn setActivationPolicy_(self, policy: NSApplicationActivationPolicy) -> BOOL;
    unsafe fn setPresentationOptions_(self, options: NSApplicationPresentationOptions) -> BOOL;
    unsafe fn presentationOptions_(self) -> NSApplicationPresentationOptions;
    unsafe fn setMainMenu_(self, menu: id);
    unsafe fn setServicesMenu_(self, menu: id);
    unsafe fn setWindowsMenu_(self, menu: id);
    unsafe fn activateIgnoringOtherApps_(self, ignore: BOOL);
    unsafe fn run(self);
    unsafe fn finishLaunching(self);
    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(self,
                                                              mask: NSUInteger,
                                                              expiration: id,
                                                              in_mode: id,
                                                              dequeue: BOOL) -> id;
    unsafe fn sendEvent_(self, an_event: id);
    unsafe fn postEvent_atStart_(self, anEvent: id, flag: BOOL);
    unsafe fn stop_(self, sender: id);
    unsafe fn setApplicationIconImage_(self, image: id);
    unsafe fn requestUserAttention_(self, requestType: NSRequestUserAttentionType);
}

impl NSApplication for id {
    unsafe fn mainMenu(self) -> id {
        msg_send![self, mainMenu]
    }

    unsafe fn setActivationPolicy_(self, policy: NSApplicationActivationPolicy) -> BOOL {
        msg_send![self, setActivationPolicy:policy as NSInteger]
    }

    unsafe fn setPresentationOptions_(self, options: NSApplicationPresentationOptions) -> BOOL {
        msg_send![self, setPresentationOptions:options.bits]
    }

    unsafe fn presentationOptions_(self) -> NSApplicationPresentationOptions {
        let options = msg_send![self, presentationOptions];
        return NSApplicationPresentationOptions::from_bits(options).unwrap();
    }

    unsafe fn setMainMenu_(self, menu: id) {
        msg_send![self, setMainMenu:menu]
    }

    unsafe fn setServicesMenu_(self, menu: id) {
        msg_send![self, setServicesMenu:menu]
    }

    unsafe fn setWindowsMenu_(self, menu: id) {
        msg_send![self, setWindowsMenu:menu]
    }

    unsafe fn activateIgnoringOtherApps_(self, ignore: BOOL) {
        msg_send![self, activateIgnoringOtherApps:ignore]
    }

    unsafe fn run(self) {
        msg_send![self, run]
    }

    unsafe fn finishLaunching(self) {
        msg_send![self, finishLaunching]
    }

    unsafe fn nextEventMatchingMask_untilDate_inMode_dequeue_(self,
                                                              mask: NSUInteger,
                                                              expiration: id,
                                                              in_mode: id,
                                                              dequeue: BOOL) -> id {
        msg_send![self, nextEventMatchingMask:mask
                                    untilDate:expiration
                                       inMode:in_mode
                                      dequeue:dequeue]
    }

    unsafe fn sendEvent_(self, an_event: id) {
        msg_send![self, sendEvent:an_event]
    }

    unsafe fn postEvent_atStart_(self, anEvent: id, flag: BOOL) {
        msg_send![self, postEvent:anEvent atStart:flag]
    }

    unsafe fn stop_(self, sender: id) {
        msg_send![self, stop:sender]
    }

    unsafe fn setApplicationIconImage_(self, icon: id) {
        msg_send![self, setApplicationIconImage:icon]
    }

    unsafe fn requestUserAttention_(self, requestType: NSRequestUserAttentionType) {
        msg_send![self, requestUserAttention:requestType]
    }
}

pub trait NSRunningApplication: Sized {
    unsafe fn currentApplication(_: Self) -> id {
        msg_send![class!(NSRunningApplication), currentApplication]
    }

    unsafe fn activateWithOptions_(self, options: NSApplicationActivationOptions) -> BOOL;

    unsafe fn runningApplicationWithProcessIdentifier(_: Self, pid: libc::pid_t) -> id {
        msg_send![class!(NSRunningApplication), runningApplicationWithProcessIdentifier:pid]
    }
}

impl NSRunningApplication for id {
    unsafe fn activateWithOptions_(self, options: NSApplicationActivationOptions) -> BOOL {
        msg_send![self, activateWithOptions:options as NSUInteger]
    }
}

pub trait NSPasteboard: Sized {
    unsafe fn generalPasteboard(_: Self) -> id {
        msg_send![class!(NSPasteboard), generalPasteboard]
    }

    unsafe fn pasteboardByFilteringData_ofType(_: Self, data: id, _type: id) -> id {
        msg_send![class!(NSPasteboard), pasteboardByFilteringData:data ofType:_type]
    }

    unsafe fn pasteboardByFilteringFile(_: Self, file: id) -> id {
        msg_send![class!(NSPasteboard), pasteboardByFilteringFile:file]
    }

    unsafe fn pasteboardByFilteringTypesInPasteboard(_: Self, pboard: id) -> id {
        msg_send![class!(NSPasteboard), pasteboardByFilteringTypesInPasteboard:pboard]
    }

    unsafe fn pasteboardWithName(_: Self, name: id) -> id {
        msg_send![class!(NSPasteboard), pasteboardWithName:name]
    }

    unsafe fn pasteboardWithUniqueName(_: Self) -> id {
        msg_send![class!(NSPasteboard), pasteboardWithUniqueName]
    }

    unsafe fn releaseGlobally(self);

    unsafe fn clearContents(self) -> NSInteger;
    unsafe fn writeObjects(self, objects: id) -> BOOL;
    unsafe fn setData_forType(self, data: id, dataType: id) -> BOOL;
    unsafe fn setPropertyList_forType(self, plist: id, dataType: id) -> BOOL;
    unsafe fn setString_forType(self, string: id, dataType: id) -> BOOL;

    unsafe fn readObjectsForClasses_options(self, classArray: id, options: id) -> id;
    unsafe fn pasteboardItems(self) -> id;
    unsafe fn indexOfPasteboardItem(self, pasteboardItem: id) -> NSInteger;
    unsafe fn dataForType(self, dataType: id) -> id;
    unsafe fn propertyListForType(self, dataType: id) -> id;
    unsafe fn stringForType(self, dataType: id) -> id;

    unsafe fn availableTypeFromArray(self, types: id) -> id;
    unsafe fn canReadItemWithDataConformingToTypes(self, types: id) -> BOOL;
    unsafe fn canReadObjectForClasses_options(self, classArray: id, options: id) -> BOOL;
    unsafe fn types(self) -> id;
    unsafe fn typesFilterableTo(_: Self, _type: id) -> id {
        msg_send![class!(NSPasteboard), typesFilterableTo:_type]
    }

    unsafe fn name(self) -> id;
    unsafe fn changeCount(self) -> NSInteger;

    unsafe fn declareTypes_owner(self, newTypes: id, newOwner: id) -> NSInteger;
    unsafe fn addTypes_owner(self, newTypes: id, newOwner: id) -> NSInteger;
    unsafe fn writeFileContents(self, filename: id) -> BOOL;
    unsafe fn writeFileWrapper(self, wrapper: id) -> BOOL;

    unsafe fn readFileContentsType_toFile(self, _type: id, filename: id) -> id;
    unsafe fn readFileWrapper(self) -> id;
}

impl NSPasteboard for id {
    unsafe fn releaseGlobally(self) {
        msg_send![self, releaseGlobally]
    }

    unsafe fn clearContents(self) -> NSInteger {
        msg_send![self, clearContents]
    }

    unsafe fn writeObjects(self, objects: id) -> BOOL {
        msg_send![self, writeObjects:objects]
    }

    unsafe fn setData_forType(self, data: id, dataType: id) -> BOOL {
        msg_send![self, setData:data forType:dataType]
    }

    unsafe fn setPropertyList_forType(self, plist: id, dataType: id) -> BOOL {
        msg_send![self, setPropertyList:plist forType:dataType]
    }

    unsafe fn setString_forType(self, string: id, dataType: id) -> BOOL {
        msg_send![self, setString:string forType:dataType]
    }

    unsafe fn readObjectsForClasses_options(self, classArray: id, options: id) -> id {
        msg_send![self, readObjectsForClasses:classArray options:options]
    }

    unsafe fn pasteboardItems(self) -> id {
        msg_send![self, pasteboardItems]
    }

    unsafe fn indexOfPasteboardItem(self, pasteboardItem: id) -> NSInteger {
        msg_send![self, indexOfPasteboardItem:pasteboardItem]
    }

    unsafe fn dataForType(self, dataType: id) -> id {
        msg_send![self, dataForType:dataType]
    }

    unsafe fn propertyListForType(self, dataType: id) -> id {
        msg_send![self, propertyListForType:dataType]
    }

    unsafe fn stringForType(self, dataType: id) -> id {
        msg_send![self, stringForType:dataType]
    }

    unsafe fn availableTypeFromArray(self, types: id) -> id {
        msg_send![self, availableTypeFromArray:types]
    }

    unsafe fn canReadItemWithDataConformingToTypes(self, types: id) -> BOOL {
        msg_send![self, canReadItemWithDataConformingToTypes:types]
    }

    unsafe fn canReadObjectForClasses_options(self, classArray: id, options: id) -> BOOL {
        msg_send![self, canReadObjectForClasses:classArray options:options]
    }

    unsafe fn types(self) -> id {
        msg_send![self, types]
    }

    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn changeCount(self) -> NSInteger {
        msg_send![self, changeCount]
    }

    unsafe fn declareTypes_owner(self, newTypes: id, newOwner: id) -> NSInteger {
        msg_send![self, declareTypes:newTypes owner:newOwner]
    }

    unsafe fn addTypes_owner(self, newTypes: id, newOwner: id) -> NSInteger {
        msg_send![self, addTypes:newTypes owner:newOwner]
    }

    unsafe fn writeFileContents(self, filename: id) -> BOOL {
        msg_send![self, writeFileContents:filename]
    }

    unsafe fn writeFileWrapper(self, wrapper: id) -> BOOL {
        msg_send![self, writeFileWrapper:wrapper]
    }

    unsafe fn readFileContentsType_toFile(self, _type: id, filename: id) -> id {
        msg_send![self, readFileContentsType:_type toFile:filename]
    }

    unsafe fn readFileWrapper(self) -> id {
        msg_send![self, readFileWrapper]
    }

}

pub trait NSPasteboardItem: Sized {
    unsafe fn types(self) -> id;

    unsafe fn setDataProvider_forTypes(self, dataProvider: id, types: id) -> BOOL;
    unsafe fn setData_forType(self, data: id, _type: id) -> BOOL;
    unsafe fn setString_forType(self, string: id, _type: id) -> BOOL;
    unsafe fn setPropertyList_forType(self, propertyList: id, _type: id) -> BOOL;

    unsafe fn dataForType(self, _type: id) -> id;
    unsafe fn stringForType(self, _type: id) -> id;
    unsafe fn propertyListForType(self, _type: id) -> id;
}

impl NSPasteboardItem for id {
    unsafe fn types(self) -> id {
        msg_send![self, types]
    }

    unsafe fn setDataProvider_forTypes(self, dataProvider: id, types: id) -> BOOL {
        msg_send![self, setDataProvider:dataProvider forTypes:types]
    }

    unsafe fn setData_forType(self, data: id, _type: id) -> BOOL {
        msg_send![self, setData:data forType:_type]
    }

    unsafe fn setString_forType(self, string: id, _type: id) -> BOOL {
        msg_send![self, setString:string forType:_type]
    }

    unsafe fn setPropertyList_forType(self, propertyList: id, _type: id) -> BOOL {
        msg_send![self, setPropertyList:propertyList forType:_type]
    }

    unsafe fn dataForType(self, _type: id) -> id {
        msg_send![self, dataForType:_type]
    }

    unsafe fn stringForType(self, _type: id) -> id {
        msg_send![self, stringForType:_type]
    }

    unsafe fn propertyListForType(self, _type: id) -> id {
        msg_send![self, propertyListForType:_type]
    }
}

pub trait NSPasteboardItemDataProvider: Sized {
    unsafe fn pasteboard_item_provideDataForType(self, pasteboard: id, item: id, _type: id);
    unsafe fn pasteboardFinishedWithDataProvider(self, pasteboard: id);
}

impl NSPasteboardItemDataProvider for id {
    unsafe fn pasteboard_item_provideDataForType(self, pasteboard: id, item: id, _type: id) {
        msg_send![self, pasteboard:pasteboard item:item provideDataForType:_type]
    }

    unsafe fn pasteboardFinishedWithDataProvider(self, pasteboard: id) {
        msg_send![self, pasteboardFinishedWithDataProvider:pasteboard]
    }
}

pub trait NSPasteboardWriting: Sized {
    unsafe fn writableTypesForPasteboard(self, pasteboard: id) -> id;
    unsafe fn writingOptionsForType_pasteboard(self, _type: id, pasteboard: id) -> NSPasteboardWritingOptions;

    unsafe fn pasteboardPropertyListForType(self, _type: id) -> id;
}

impl NSPasteboardWriting for id {
    unsafe fn writableTypesForPasteboard(self, pasteboard: id) -> id {
        msg_send![self, writableTypesForPasteboard:pasteboard]
    }

    unsafe fn writingOptionsForType_pasteboard(self, _type: id, pasteboard: id) -> NSPasteboardWritingOptions {
        msg_send![self, writingOptionsForType:_type pasteboard:pasteboard]
    }

    unsafe fn pasteboardPropertyListForType(self, _type: id) -> id {
        msg_send![self, pasteboardPropertyListForType:_type]
    }
}

pub trait NSPasteboardReading: Sized {
    unsafe fn initWithPasteboardPropertyList_ofType(self, propertyList: id, _type: id) -> id;

    unsafe fn readableTypesForPasteboard(self, pasteboard: id) -> id;
    unsafe fn readingOptionsForType_pasteboard(self, _type: id, pasteboard: id) -> NSPasteboardReadingOptions;
}

impl NSPasteboardReading for id {
    unsafe fn initWithPasteboardPropertyList_ofType(self, propertyList: id, _type: id) -> id {
        msg_send![self, initWithPasteboardPropertyList:propertyList ofType:_type]
    }

    unsafe fn readableTypesForPasteboard(self, pasteboard: id) -> id {
        let class: id = msg_send![self, class];
        msg_send![class, readableTypesForPasteboard:pasteboard]
    }
    unsafe fn readingOptionsForType_pasteboard(self, _type: id, pasteboard: id) -> NSPasteboardReadingOptions {
        let class: id = msg_send![self, class];
        msg_send![class, readingOptionsForType:_type pasteboard:pasteboard]
    }
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSPasteboardReadingOptions {
    NSPasteboardReadingAsData = 0,
    NSPasteboardReadingAsString = 1 << 0,
    NSPasteboardReadingAsPropertyList = 1 << 1,
    NSPasteboardReadingAsKeyedArchive = 1 << 2
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSPasteboardWritingOptions {
    NSPasteboardWritingPromised = 1 << 9,
}

pub trait NSMenu: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSMenu), alloc]
    }

    unsafe fn new(_: Self) -> id {
        msg_send![class!(NSMenu), new]
    }

    unsafe fn initWithTitle_(self, title: id /* NSString */) -> id;
    unsafe fn setAutoenablesItems(self, state: BOOL);

    unsafe fn addItem_(self, menu_item: id);
    unsafe fn addItemWithTitle_action_keyEquivalent(self, title: id, action: SEL, key: id) -> id;
    unsafe fn itemAtIndex_(self, index: NSInteger) -> id;
}

impl NSMenu for id {
    unsafe fn initWithTitle_(self, title: id /* NSString */) -> id {
        msg_send![self, initWithTitle:title]
    }

    unsafe fn setAutoenablesItems(self, state: BOOL) {
        msg_send![self, setAutoenablesItems: state]
    }

    unsafe fn addItem_(self, menu_item: id) {
        msg_send![self, addItem:menu_item]
    }

    unsafe fn addItemWithTitle_action_keyEquivalent(self, title: id, action: SEL, key: id) -> id {
        msg_send![self, addItemWithTitle:title action:action keyEquivalent:key]
    }

    unsafe fn itemAtIndex_(self, index: NSInteger) -> id {
        msg_send![self, itemAtIndex:index]
    }
}

pub trait NSMenuItem: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSMenuItem), alloc]
    }

    unsafe fn new(_: Self) -> id {
        msg_send![class!(NSMenuItem), new]
    }

    unsafe fn separatorItem(_: Self) -> id {
        msg_send![class!(NSMenuItem), separatorItem]
    }

    unsafe fn initWithTitle_action_keyEquivalent_(self, title: id, action: SEL, key: id) -> id;
    unsafe fn setKeyEquivalentModifierMask_(self, mask: NSEventModifierFlags);
    unsafe fn setSubmenu_(self, submenu: id);
    unsafe fn setTarget_(self, target: id);
}

impl NSMenuItem for id {
    unsafe fn initWithTitle_action_keyEquivalent_(self, title: id, action: SEL, key: id) -> id {
        msg_send![self, initWithTitle:title action:action keyEquivalent:key]
    }

    unsafe fn setKeyEquivalentModifierMask_(self, mask: NSEventModifierFlags) {
        msg_send![self, setKeyEquivalentModifierMask:mask]
    }

    unsafe fn setSubmenu_(self, submenu: id) {
        msg_send![self, setSubmenu:submenu]
    }

    unsafe fn setTarget_(self, target: id) {
        msg_send![self, setTarget:target]
    }
}

pub type NSWindowDepth = libc::c_int;

bitflags! {
    pub struct NSWindowCollectionBehavior: NSUInteger {
        const NSWindowCollectionBehaviorDefault = 0;
        const NSWindowCollectionBehaviorCanJoinAllSpaces = 1 << 0;
        const NSWindowCollectionBehaviorMoveToActiveSpace = 1 << 1;

        const NSWindowCollectionBehaviorManaged = 1 << 2;
        const NSWindowCollectionBehaviorTransient = 1 << 3;
        const NSWindowCollectionBehaviorStationary = 1 << 4;

        const NSWindowCollectionBehaviorParticipatesInCycle = 1 << 5;
        const NSWindowCollectionBehaviorIgnoresCycle = 1 << 6;

        const NSWindowCollectionBehaviorFullScreenPrimary = 1 << 7;
        const NSWindowCollectionBehaviorFullScreenAuxiliary = 1 << 8;
    }
}

bitflags! {
    pub struct NSWindowOcclusionState: NSUInteger {
        const NSWindowOcclusionStateVisible = 1 << 1;
    }
}

pub trait NSWindow: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSWindow), alloc]
    }

    // Creating Windows
    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSWindowStyleMask,
                                                           backing: NSBackingStoreType,
                                                           defer: BOOL) -> id;
    unsafe fn initWithContentRect_styleMask_backing_defer_screen_(self,
                                                                  rect: NSRect,
                                                                  style: NSWindowStyleMask,
                                                                  backing: NSBackingStoreType,
                                                                  defer: BOOL,
                                                                  screen: id) -> id;

    // Configuring Windows
    unsafe fn styleMask(self) -> NSWindowStyleMask;
    unsafe fn setStyleMask_(self, styleMask: NSWindowStyleMask);
    unsafe fn toggleFullScreen_(self, sender: id);
    unsafe fn worksWhenModal(self) -> BOOL;
    unsafe fn alphaValue(self) -> CGFloat;
    unsafe fn setAlphaValue_(self, windowAlpha: CGFloat);
    unsafe fn backgroundColor(self) -> id;
    unsafe fn setBackgroundColor_(self, color: id);
    unsafe fn colorSpace(self) -> id;
    unsafe fn setColorSpace_(self, colorSpace: id);
    unsafe fn contentView(self) -> id;
    unsafe fn setContentView_(self, view: id);
    unsafe fn canHide(self) -> BOOL;
    unsafe fn setCanHide_(self, canHide: BOOL);
    unsafe fn hidesOnDeactivate(self) -> BOOL;
    unsafe fn setHidesOnDeactivate_(self, hideOnDeactivate: BOOL);
    unsafe fn collectionBehavior(self) -> NSWindowCollectionBehavior;
    unsafe fn setCollectionBehavior_(self, collectionBehavior: NSWindowCollectionBehavior);
    unsafe fn setOpaque_(self, opaque: BOOL);
    unsafe fn hasShadow(self) -> BOOL;
    unsafe fn setHasShadow_(self, hasShadow: BOOL);
    unsafe fn invalidateShadow(self);
    unsafe fn autorecalculatesContentBorderThicknessForEdge_(self, edge: NSRectEdge) -> BOOL;
    unsafe fn setAutorecalculatesContentBorderThickness_forEdge_(self,
                                                                 autorecalculateContentBorderThickness: BOOL,
                                                                 edge: NSRectEdge) -> BOOL;
    unsafe fn contentBorderThicknessForEdge_(self, edge: NSRectEdge) -> CGFloat;
    unsafe fn setContentBorderThickness_forEdge_(self, borderThickness: CGFloat, edge: NSRectEdge);
    unsafe fn delegate(self) -> id;
    unsafe fn setDelegate_(self, delegate: id);
    unsafe fn preventsApplicationTerminationWhenModal(self) -> BOOL;
    unsafe fn setPreventsApplicationTerminationWhenModal_(self, flag: BOOL);

    // TODO: Accessing Window Information

    // Getting Layout Information
    unsafe fn contentRectForFrameRect_styleMask_(self, windowFrame: NSRect, windowStyle: NSWindowStyleMask) -> NSRect;
    unsafe fn frameRectForContentRect_styleMask_(self, windowContentRect: NSRect, windowStyle: NSWindowStyleMask) -> NSRect;
    unsafe fn minFrameWidthWithTitle_styleMask_(self, windowTitle: id, windowStyle: NSWindowStyleMask) -> CGFloat;
    unsafe fn contentRectForFrameRect_(self, windowFrame: NSRect) -> NSRect;
    unsafe fn frameRectForContentRect_(self, windowContent: NSRect) -> NSRect;

    // Managing Windows
    unsafe fn drawers(self) -> id;
    unsafe fn windowController(self) -> id;
    unsafe fn setWindowController_(self, windowController: id);

    // TODO: Managing Sheets

    // Sizing Windows
    unsafe fn frame(self) -> NSRect;
    unsafe fn setFrameOrigin_(self, point: NSPoint);
    unsafe fn setFrameTopLeftPoint_(self, point: NSPoint);
    unsafe fn constrainFrameRect_toScreen_(self, frameRect: NSRect, screen: id);
    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: NSPoint) -> NSPoint;
    unsafe fn setFrame_display_(self, windowFrame: NSRect, display: BOOL);
    unsafe fn setFrame_display_animate_(self, windowFrame: NSRect, display: BOOL, animate: BOOL);
    unsafe fn setFrame_displayViews_(self, windowFrame: NSRect, display: BOOL);
    unsafe fn aspectRatio(self) -> NSSize;
    unsafe fn setAspectRatio_(self, aspectRatio: NSSize);
    unsafe fn minSize(self) -> NSSize;
    unsafe fn setMinSize_(self, minSize: NSSize);
    unsafe fn maxSize(self) -> NSSize;
    unsafe fn setMaxSize_(self, maxSize: NSSize);
    unsafe fn performZoom_(self, sender: id);
    unsafe fn zoom_(self, sender: id);
    unsafe fn resizeFlags(self) -> NSInteger;
    unsafe fn showsResizeIndicator(self) -> BOOL;
    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: BOOL);
    unsafe fn resizeIncrements(self) -> NSSize;
    unsafe fn setResizeIncrements_(self, resizeIncrements: NSSize);
    unsafe fn preservesContentDuringLiveResize(self) -> BOOL;
    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: BOOL);
    unsafe fn inLiveResize(self) -> BOOL;

    // Sizing Content
    unsafe fn contentAspectRatio(self) -> NSSize;
    unsafe fn setContentAspectRatio_(self, contentAspectRatio: NSSize);
    unsafe fn contentMinSize(self) -> NSSize;
    unsafe fn setContentMinSize_(self, contentMinSize: NSSize);
    unsafe fn contentSize(self) -> NSSize;
    unsafe fn setContentSize_(self, contentSize: NSSize);
    unsafe fn contentMaxSize(self) -> NSSize;
    unsafe fn setContentMaxSize_(self, contentMaxSize: NSSize);
    unsafe fn contentResizeIncrements(self) -> NSSize;
    unsafe fn setContentResizeIncrements_(self, contentResizeIncrements: NSSize);

    // Managing Window Visibility and Occlusion State
    unsafe fn isVisible(self) -> BOOL; // NOTE: Deprecated in 10.9
    unsafe fn occlusionState(self) -> NSWindowOcclusionState;

    // Managing Window Layers
    unsafe fn orderOut_(self, sender: id);
    unsafe fn orderBack_(self, sender: id);
    unsafe fn orderFront_(self, sender: id);
    unsafe fn orderFrontRegardless(self);
    unsafe fn orderFrontWindow_relativeTo_(self, orderingMode: NSWindowOrderingMode, otherWindowNumber: NSInteger);
    unsafe fn level(self) -> NSInteger;
    unsafe fn setLevel_(self, level: NSInteger);

    // Managing Key Status
    unsafe fn isKeyWindow(self) -> BOOL;
    unsafe fn canBecomeKeyWindow(self) -> BOOL;
    unsafe fn makeKeyWindow(self);
    unsafe fn makeKeyAndOrderFront_(self, sender: id);
    // skipped: becomeKeyWindow (should not be invoked directly, according to Apple's documentation)
    // skipped: resignKeyWindow (should not be invoked directly, according to Apple's documentation)

    // Managing Main Status
    unsafe fn canBecomeMainWindow(self) -> BOOL;
    unsafe fn makeMainWindow(self);
    // skipped: becomeMainWindow (should not be invoked directly, according to Apple's documentation)
    // skipped: resignMainWindow (should not be invoked directly, according to Apple's documentation)

    // Managing Toolbars
    unsafe fn toolbar(self) -> id /* NSToolbar */;
    unsafe fn toolbarStyle(self) -> NSWindowToolbarStyle;
    unsafe fn setToolbar_(self, toolbar: id /* NSToolbar */);
    unsafe fn setToolbarStyle_(self, toolbarStyle: NSWindowToolbarStyle);
    unsafe fn runToolbarCustomizationPalette(self, sender: id);

    // TODO: Managing Attached Windows
    // TODO: Managing Window Buffers
    // TODO: Managing Default Buttons
    // TODO: Managing Field Editors
    // TODO: Managing the Window Menu
    // TODO: Managing Cursor Rectangles

    // Managing Title Bars
    unsafe fn standardWindowButton_(self, windowButtonKind: NSWindowButton) -> id;

    // Managing Window Tabs
    unsafe fn allowsAutomaticWindowTabbing(_: Self) -> BOOL;
    unsafe fn setAllowsAutomaticWindowTabbing_(_: Self, allowsAutomaticWindowTabbing: BOOL);
    unsafe fn tabbingIdentifier(self) -> id;
    unsafe fn tabbingMode(self) -> NSWindowTabbingMode;
    unsafe fn setTabbingMode_(self, tabbingMode: NSWindowTabbingMode);
    unsafe fn addTabbedWindow_ordered_(self, window: id, ordering_mode: NSWindowOrderingMode);
    unsafe fn toggleTabBar_(self, sender: id);

    // TODO: Managing Tooltips
    // TODO: Handling Events

    // Managing Responders
    unsafe fn initialFirstResponder(self) -> id;
    unsafe fn firstResponder(self) -> id;
    unsafe fn setInitialFirstResponder_(self, responder: id);
    unsafe fn makeFirstResponder_(self, responder: id) -> BOOL;

    // TODO: Managing the Key View Loop

    // Handling Keyboard Events
    unsafe fn keyDown_(self, event: id);

    // Handling Mouse Events
    unsafe fn acceptsMouseMovedEvents(self) -> BOOL;
    unsafe fn ignoresMouseEvents(self) -> BOOL;
    unsafe fn setIgnoresMouseEvents_(self, ignoreMouseEvents: BOOL);
    unsafe fn mouseLocationOutsideOfEventStream(self) -> NSPoint;
    unsafe fn setAcceptsMouseMovedEvents_(self, acceptMouseMovedEvents: BOOL);
    unsafe fn windowNumberAtPoint_belowWindowWithWindowNumber_(_: Self,
                                                               point: NSPoint,
                                                               windowNumber: NSInteger) -> NSInteger;

    // TODO: Handling Window Restoration
    // TODO: Bracketing Drawing Operations
    // TODO: Drawing Windows
    // TODO: Window Animation
    // TODO: Updating Windows
    // TODO: Dragging Items

    // Converting Coordinates
    unsafe fn backingScaleFactor(self) -> CGFloat;
    unsafe fn backingAlignedRect_options_(self, rect: NSRect, options: NSAlignmentOptions) -> NSRect;
    unsafe fn convertRectFromBacking_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectToBacking_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectToScreen_(self, rect: NSRect) -> NSRect;
    unsafe fn convertRectFromScreen_(self, rect: NSRect) -> NSRect;

    // Accessing Edited Status
    unsafe fn isDocumentEdited(self) -> BOOL;
    unsafe fn setDocumentEdited_(self, documentEdited: BOOL);

    // Managing Titles
    unsafe fn title(self) -> id;
    unsafe fn setTitle_(self, title: id);
    unsafe fn setTitleWithRepresentedFilename_(self, filePath: id);
    unsafe fn setTitleVisibility_(self, visibility: NSWindowTitleVisibility);
    unsafe fn setTitlebarAppearsTransparent_(self, transparent: BOOL);
    unsafe fn representedFilename(self) -> id;
    unsafe fn setRepresentedFilename_(self, filePath: id);
    unsafe fn representedURL(self) -> id;
    unsafe fn setRepresentedURL_(self, representedURL: id);

    // Accessing Screen Information
    unsafe fn screen(self) -> id;
    unsafe fn deepestScreen(self) -> id;
    unsafe fn displaysWhenScreenProfileChanges(self) -> BOOL;
    unsafe fn setDisplaysWhenScreenProfileChanges_(self, displaysWhenScreenProfileChanges: BOOL);

    // Moving Windows
    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: BOOL);
    unsafe fn setMovable_(self, movable: BOOL);
    unsafe fn center(self);

    // Closing Windows
    unsafe fn performClose_(self, sender: id);
    unsafe fn close(self);
    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: BOOL);

    // Minimizing Windows
    unsafe fn performMiniaturize_(self, sender: id);
    unsafe fn miniaturize_(self, sender: id);
    unsafe fn deminiaturize_(self, sender: id);
    unsafe fn miniwindowImage(self) -> id;
    unsafe fn setMiniwindowImage_(self, miniwindowImage: id);
    unsafe fn miniwindowTitle(self) -> id;
    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id);
    unsafe fn setAppearance(self, appearance: id);

    // TODO: Getting the Dock Tile
    // TODO: Printing Windows
    // TODO: Providing Services
    // TODO: Working with Carbon
    // TODO: Triggering Constraint-Based Layout
    // TODO: Debugging Constraint-Based Layout
    // TODO: Constraint-Based Layouts
}

impl NSWindow for id {
    // Creating Windows

    unsafe fn initWithContentRect_styleMask_backing_defer_(self,
                                                           rect: NSRect,
                                                           style: NSWindowStyleMask,
                                                           backing: NSBackingStoreType,
                                                           defer: BOOL) -> id {
        msg_send![self, initWithContentRect:rect
                                  styleMask:style.bits
                                    backing:backing as NSUInteger
                                      defer:defer]
    }

    unsafe fn initWithContentRect_styleMask_backing_defer_screen_(self,
                                                                  rect: NSRect,
                                                                  style: NSWindowStyleMask,
                                                                  backing: NSBackingStoreType,
                                                                  defer: BOOL,
                                                                  screen: id) -> id {
        msg_send![self, initWithContentRect:rect
                                  styleMask:style.bits
                                    backing:backing as NSUInteger
                                      defer:defer
                                     screen:screen]
    }

    // Configuring Windows

    unsafe fn styleMask(self) -> NSWindowStyleMask {
        NSWindowStyleMask::from_bits_truncate(msg_send![self, styleMask])
    }

    unsafe fn setStyleMask_(self, styleMask: NSWindowStyleMask) {
        msg_send![self, setStyleMask:styleMask.bits]
    }

    unsafe fn toggleFullScreen_(self, sender: id) {
        msg_send![self, toggleFullScreen:sender]
    }

    unsafe fn worksWhenModal(self) -> BOOL {
        msg_send![self, worksWhenModal]
    }

    unsafe fn alphaValue(self) -> CGFloat {
        msg_send![self, alphaValue]
    }

    unsafe fn setAlphaValue_(self, windowAlpha: CGFloat) {
        msg_send![self, setAlphaValue:windowAlpha]
    }

    unsafe fn backgroundColor(self) -> id {
        msg_send![self, backgroundColor]
    }

    unsafe fn setBackgroundColor_(self, color: id) {
        msg_send![self, setBackgroundColor:color]
    }

    unsafe fn colorSpace(self) -> id {
        msg_send![self, colorSpace]
    }

    unsafe fn setColorSpace_(self, colorSpace: id) {
        msg_send![self, setColorSpace:colorSpace]
    }

    unsafe fn contentView(self) -> id {
        msg_send![self, contentView]
    }

    unsafe fn setContentView_(self, view: id) {
        msg_send![self, setContentView:view]
    }

    unsafe fn canHide(self) -> BOOL {
        msg_send![self, canHide]
    }

    unsafe fn setCanHide_(self, canHide: BOOL) {
        msg_send![self, setCanHide:canHide]
    }

    unsafe fn hidesOnDeactivate(self) -> BOOL {
        msg_send![self, hidesOnDeactivate]
    }

    unsafe fn setHidesOnDeactivate_(self, hideOnDeactivate: BOOL) {
        msg_send![self, setHidesOnDeactivate:hideOnDeactivate]
    }

    unsafe fn collectionBehavior(self) -> NSWindowCollectionBehavior {
        msg_send![self, collectionBehavior]
    }

    unsafe fn setCollectionBehavior_(self, collectionBehavior: NSWindowCollectionBehavior) {
        msg_send![self, setCollectionBehavior:collectionBehavior]
    }

    unsafe fn setOpaque_(self, opaque: BOOL) {
        msg_send![self, setOpaque:opaque]
    }

    unsafe fn hasShadow(self) -> BOOL {
        msg_send![self, hasShadow]
    }

    unsafe fn setHasShadow_(self, hasShadow: BOOL) {
        msg_send![self, setHasShadow:hasShadow]
    }

    unsafe fn invalidateShadow(self) {
        msg_send![self, invalidateShadow]
    }

    unsafe fn autorecalculatesContentBorderThicknessForEdge_(self, edge: NSRectEdge) -> BOOL {
        msg_send![self, autorecalculatesContentBorderThicknessForEdge:edge]
    }

    unsafe fn setAutorecalculatesContentBorderThickness_forEdge_(self,
                                                                 autorecalculateContentBorderThickness: BOOL,
                                                                 edge: NSRectEdge) -> BOOL {
        msg_send![self, setAutorecalculatesContentBorderThickness:
                        autorecalculateContentBorderThickness forEdge:edge]
    }

    unsafe fn contentBorderThicknessForEdge_(self, edge: NSRectEdge) -> CGFloat {
        msg_send![self, contentBorderThicknessForEdge:edge]
    }

    unsafe fn setContentBorderThickness_forEdge_(self, borderThickness: CGFloat, edge: NSRectEdge) {
        msg_send![self, setContentBorderThickness:borderThickness forEdge:edge]
    }

    unsafe fn delegate(self) -> id {
        msg_send![self, delegate]
    }

    unsafe fn setDelegate_(self, delegate: id) {
        msg_send![self, setDelegate:delegate]
    }

    unsafe fn preventsApplicationTerminationWhenModal(self) -> BOOL {
        msg_send![self, preventsApplicationTerminationWhenModal]
    }

    unsafe fn setPreventsApplicationTerminationWhenModal_(self, flag: BOOL) {
        msg_send![self, setPreventsApplicationTerminationWhenModal:flag]
    }

    // TODO: Accessing Window Information

    // Getting Layout Information

    unsafe fn contentRectForFrameRect_styleMask_(self, windowFrame: NSRect, windowStyle: NSWindowStyleMask) -> NSRect {
        msg_send![self, contentRectForFrameRect:windowFrame styleMask:windowStyle.bits]
    }

    unsafe fn frameRectForContentRect_styleMask_(self, windowContentRect: NSRect, windowStyle: NSWindowStyleMask) -> NSRect {
        msg_send![self, frameRectForContentRect:windowContentRect styleMask:windowStyle.bits]
    }

    unsafe fn minFrameWidthWithTitle_styleMask_(self, windowTitle: id, windowStyle: NSWindowStyleMask) -> CGFloat {
        msg_send![self, minFrameWidthWithTitle:windowTitle styleMask:windowStyle.bits]
    }

    unsafe fn contentRectForFrameRect_(self, windowFrame: NSRect) -> NSRect {
        msg_send![self, contentRectForFrameRect:windowFrame]
    }

    unsafe fn frameRectForContentRect_(self, windowContent: NSRect) -> NSRect {
        msg_send![self, frameRectForContentRect:windowContent]
    }

    // Managing Windows

    unsafe fn drawers(self) -> id {
        msg_send![self, drawers]
    }

    unsafe fn windowController(self) -> id {
        msg_send![self, windowController]
    }

    unsafe fn setWindowController_(self, windowController: id) {
        msg_send![self, setWindowController:windowController]
    }

    // TODO: Managing Sheets

    // Sizing Windows

    unsafe fn frame(self) -> NSRect {
        msg_send![self, frame]
    }

    unsafe fn setFrameOrigin_(self, point: NSPoint) {
        msg_send![self, setFrameOrigin:point]
    }

    unsafe fn setFrameTopLeftPoint_(self, point: NSPoint) {
        msg_send![self, setFrameTopLeftPoint:point]
    }

    unsafe fn constrainFrameRect_toScreen_(self, frameRect: NSRect, screen: id) {
        msg_send![self, constrainFrameRect:frameRect toScreen:screen]
    }

    unsafe fn cascadeTopLeftFromPoint_(self, topLeft: NSPoint) -> NSPoint {
        msg_send![self, cascadeTopLeftFromPoint:topLeft]
    }

    unsafe fn setFrame_display_(self, windowFrame: NSRect, display: BOOL) {
        msg_send![self, setFrame:windowFrame display:display]
    }

    unsafe fn setFrame_display_animate_(self, windowFrame: NSRect, display: BOOL, animate: BOOL) {
        msg_send![self, setFrame:windowFrame display:display animate: animate]
    }

    unsafe fn setFrame_displayViews_(self, windowFrame: NSRect, display: BOOL) {
        msg_send![self, setFrame:windowFrame displayViews:display]
    }

    unsafe fn aspectRatio(self) -> NSSize {
        msg_send![self, aspectRatio]
    }

    unsafe fn setAspectRatio_(self, aspectRatio: NSSize) {
        msg_send![self, setAspectRatio:aspectRatio]
    }

    unsafe fn minSize(self) -> NSSize {
        msg_send![self, minSize]
    }

    unsafe fn setMinSize_(self, minSize: NSSize) {
        msg_send![self, setMinSize:minSize]
    }

    unsafe fn maxSize(self) -> NSSize {
        msg_send![self, maxSize]
    }

    unsafe fn setMaxSize_(self, maxSize: NSSize) {
        msg_send![self, setMaxSize:maxSize]
    }

    unsafe fn performZoom_(self, sender: id) {
        msg_send![self, performZoom:sender]
    }

    unsafe fn zoom_(self, sender: id) {
        msg_send![self, zoom:sender]
    }

    unsafe fn resizeFlags(self) -> NSInteger {
        msg_send![self, resizeFlags]
    }

    unsafe fn showsResizeIndicator(self) -> BOOL {
        msg_send![self, showsResizeIndicator]
    }

    unsafe fn setShowsResizeIndicator_(self, showsResizeIndicator: BOOL) {
        msg_send![self, setShowsResizeIndicator:showsResizeIndicator]
    }

    unsafe fn resizeIncrements(self) -> NSSize {
        msg_send![self, resizeIncrements]
    }

    unsafe fn setResizeIncrements_(self, resizeIncrements: NSSize) {
        msg_send![self, setResizeIncrements:resizeIncrements]
    }

    unsafe fn preservesContentDuringLiveResize(self) -> BOOL {
        msg_send![self, preservesContentDuringLiveResize]
    }

    unsafe fn setPreservesContentDuringLiveResize_(self, preservesContentDuringLiveResize: BOOL) {
        msg_send![self, setPreservesContentDuringLiveResize:preservesContentDuringLiveResize]
    }

    unsafe fn inLiveResize(self) -> BOOL {
        msg_send![self, inLiveResize]
    }

    // Sizing Content

    unsafe fn contentAspectRatio(self) -> NSSize {
        msg_send![self, contentAspectRatio]
    }

    unsafe fn setContentAspectRatio_(self, contentAspectRatio: NSSize) {
        msg_send![self, setContentAspectRatio:contentAspectRatio]
    }

    unsafe fn contentMinSize(self) -> NSSize {
        msg_send![self, contentMinSize]
    }

    unsafe fn setContentMinSize_(self, contentMinSize: NSSize) {
        msg_send![self, setContentMinSize:contentMinSize]
    }

    unsafe fn contentSize(self) -> NSSize {
        msg_send![self, contentSize]
    }

    unsafe fn setContentSize_(self, contentSize: NSSize) {
        msg_send![self, setContentSize:contentSize]
    }

    unsafe fn contentMaxSize(self) -> NSSize {
        msg_send![self, contentMaxSize]
    }

    unsafe fn setContentMaxSize_(self, contentMaxSize: NSSize) {
        msg_send![self, setContentMaxSize:contentMaxSize]
    }

    unsafe fn contentResizeIncrements(self) -> NSSize {
        msg_send![self, contentResizeIncrements]
    }

    unsafe fn setContentResizeIncrements_(self, contentResizeIncrements: NSSize) {
        msg_send![self, setContentResizeIncrements:contentResizeIncrements]
    }

    // Managing Window Visibility and Occlusion State

    unsafe fn isVisible(self) -> BOOL {
        msg_send![self, isVisible]
    }

    unsafe fn occlusionState(self) -> NSWindowOcclusionState {
        msg_send![self, occlusionState]
    }

    // Managing Window Layers

    unsafe fn orderOut_(self, sender: id) {
        msg_send![self, orderOut:sender]
    }

    unsafe fn orderBack_(self, sender: id) {
        msg_send![self, orderBack:sender]
    }

    unsafe fn orderFront_(self, sender: id) {
        msg_send![self, orderFront:sender]
    }

    unsafe fn orderFrontRegardless(self) {
        msg_send![self, orderFrontRegardless]
    }

    unsafe fn orderFrontWindow_relativeTo_(self, ordering_mode: NSWindowOrderingMode, other_window_number: NSInteger) {
        msg_send![self, orderWindow:ordering_mode relativeTo:other_window_number]
    }

    unsafe fn level(self) -> NSInteger {
        msg_send![self, level]
    }

    unsafe fn setLevel_(self, level: NSInteger) {
        msg_send![self, setLevel:level]
    }

    // Managing Key Status

    unsafe fn isKeyWindow(self) -> BOOL {
        msg_send![self, isKeyWindow]
    }

    unsafe fn canBecomeKeyWindow(self) -> BOOL {
        msg_send![self, canBecomeKeyWindow]
    }

    unsafe fn makeKeyWindow(self) {
        msg_send![self, makeKeyWindow]
    }

    unsafe fn makeKeyAndOrderFront_(self, sender: id) {
        msg_send![self, makeKeyAndOrderFront:sender]
    }

    // Managing Main Status

    unsafe fn canBecomeMainWindow(self) -> BOOL {
        msg_send![self, canBecomeMainWindow]
    }

    unsafe fn makeMainWindow(self) {
        msg_send![self, makeMainWindow]
    }

    // Managing Toolbars

    unsafe fn toolbar(self) -> id /* NSToolbar */ {
        msg_send![self, toolbar]
    }

    unsafe fn toolbarStyle(self) -> NSWindowToolbarStyle {
        msg_send![self, toolbarStyle]
    }

    unsafe fn setToolbar_(self, toolbar: id /* NSToolbar */) {
        msg_send![self, setToolbar:toolbar]
    }

    unsafe fn setToolbarStyle_(self, toolbarStyle: NSWindowToolbarStyle) {
        msg_send![self, setToolbarStyle:toolbarStyle]
    }

    unsafe fn runToolbarCustomizationPalette(self, sender: id) {
        msg_send![self, runToolbarCustomizationPalette:sender]
    }

    // TODO: Managing Attached Windows
    // TODO: Managing Window Buffers
    // TODO: Managing Default Buttons
    // TODO: Managing Field Editors
    // TODO: Managing the Window Menu
    // TODO: Managing Cursor Rectangles

    // Managing Title Bars

    unsafe fn standardWindowButton_(self, windowButtonKind: NSWindowButton) -> id {
        msg_send![self, standardWindowButton:windowButtonKind]
    }

    // Managing Window Tabs
    unsafe fn allowsAutomaticWindowTabbing(_: Self) -> BOOL {
        msg_send![class!(NSWindow), allowsAutomaticWindowTabbing]
    }

    unsafe fn setAllowsAutomaticWindowTabbing_(_: Self, allowsAutomaticWindowTabbing: BOOL) {
        msg_send![class!(NSWindow), setAllowsAutomaticWindowTabbing:allowsAutomaticWindowTabbing]
    }

    unsafe fn tabbingIdentifier(self) -> id {
        msg_send![self, tabbingIdentifier]
    }

    unsafe fn tabbingMode(self) -> NSWindowTabbingMode {
        msg_send!(self, tabbingMode)
    }

    unsafe fn setTabbingMode_(self, tabbingMode: NSWindowTabbingMode) {
        msg_send![self, setTabbingMode: tabbingMode]
    }

    unsafe fn addTabbedWindow_ordered_(self, window: id, ordering_mode: NSWindowOrderingMode) {
        msg_send![self, addTabbedWindow:window ordered: ordering_mode]
    }

    unsafe fn toggleTabBar_(self, sender: id) {
        msg_send![self, toggleTabBar:sender]
    }
    // TODO: Managing Tooltips
    // TODO: Handling Events

    // Managing Responders

    unsafe fn initialFirstResponder(self) -> id {
        msg_send![self, initialFirstResponder]
    }

    unsafe fn firstResponder(self) -> id {
        msg_send![self, firstResponder]
    }

    unsafe fn setInitialFirstResponder_(self, responder: id) {
        msg_send![self, setInitialFirstResponder:responder]
    }

    unsafe fn makeFirstResponder_(self, responder: id) -> BOOL {
        msg_send![self, makeFirstResponder:responder]
    }

    // TODO: Managing the Key View Loop

    // Handling Keyboard Events

    unsafe fn keyDown_(self, event: id) {
        msg_send![self, keyDown:event]
    }

    // Handling Mouse Events

    unsafe fn acceptsMouseMovedEvents(self) -> BOOL {
        msg_send![self, acceptsMouseMovedEvents]
    }

    unsafe fn ignoresMouseEvents(self) -> BOOL {
        msg_send![self, ignoresMouseEvents]
    }

    unsafe fn setIgnoresMouseEvents_(self, ignoreMouseEvents: BOOL) {
        msg_send![self, setIgnoresMouseEvents:ignoreMouseEvents]
    }

    unsafe fn mouseLocationOutsideOfEventStream(self) -> NSPoint {
        msg_send![self, mouseLocationOutsideOfEventStream]
    }

    unsafe fn setAcceptsMouseMovedEvents_(self, acceptMouseMovedEvents: BOOL) {
        msg_send![self, setAcceptsMouseMovedEvents:acceptMouseMovedEvents]
    }

    unsafe fn windowNumberAtPoint_belowWindowWithWindowNumber_(_: Self,
                                                               point: NSPoint,
                                                               windowNumber: NSInteger) -> NSInteger {
        msg_send![class!(NSWindow), windowNumberAtPoint:point belowWindowWithWindowNumber:windowNumber]
    }

    // Converting Coordinates

    unsafe fn backingScaleFactor(self) -> CGFloat {
        msg_send![self, backingScaleFactor]
    }

    unsafe fn backingAlignedRect_options_(self, rect: NSRect, options: NSAlignmentOptions) -> NSRect {
        msg_send![self, backingAlignedRect:rect options:options]
    }

    unsafe fn convertRectFromBacking_(self, rect: NSRect) -> NSRect {
        msg_send![self, convertRectFromBacking:rect]
    }

    unsafe fn convertRectToBacking_(self, rect: NSRect) -> NSRect {
        msg_send![self, convertRectToBacking:rect]
    }

    unsafe fn convertRectToScreen_(self, rect: NSRect) -> NSRect {
        msg_send![self, convertRectToScreen:rect]
    }

    unsafe fn convertRectFromScreen_(self, rect: NSRect) -> NSRect {
        msg_send![self, convertRectFromScreen:rect]
    }

    // Accessing Edited Status

    unsafe fn isDocumentEdited(self) -> BOOL {
        msg_send![self, isDocumentEdited]
    }

    unsafe fn setDocumentEdited_(self, documentEdited: BOOL) {
        msg_send![self, setDocumentEdited:documentEdited]
    }

    // Managing Titles

    unsafe fn title(self) -> id {
        msg_send![self, title]
    }

    unsafe fn setTitle_(self, title: id) {
        msg_send![self, setTitle:title]
    }

    unsafe fn setTitleWithRepresentedFilename_(self, filePath: id) {
        msg_send![self, setTitleWithRepresentedFilename:filePath]
    }

    unsafe fn setTitleVisibility_(self, visibility: NSWindowTitleVisibility) {
        msg_send![self, setTitleVisibility:visibility]
    }

    unsafe fn setTitlebarAppearsTransparent_(self, transparent: BOOL) {
        msg_send![self, setTitlebarAppearsTransparent:transparent]
    }

    unsafe fn representedFilename(self) -> id {
        msg_send![self, representedFilename]
    }

    unsafe fn setRepresentedFilename_(self, filePath: id) {
        msg_send![self, setRepresentedFilename:filePath]
    }

    unsafe fn representedURL(self) -> id {
        msg_send![self, representedURL]
    }

    unsafe fn setRepresentedURL_(self, representedURL: id) {
        msg_send![self, setRepresentedURL:representedURL]
    }

    // Accessing Screen Information

    unsafe fn screen(self) -> id {
        msg_send![self, screen]
    }

    unsafe fn deepestScreen(self) -> id {
        msg_send![self, deepestScreen]
    }

    unsafe fn displaysWhenScreenProfileChanges(self) -> BOOL {
        msg_send![self, displaysWhenScreenProfileChanges]
    }

    unsafe fn setDisplaysWhenScreenProfileChanges_(self, displaysWhenScreenProfileChanges: BOOL) {
        msg_send![self, setDisplaysWhenScreenProfileChanges:displaysWhenScreenProfileChanges]
    }

    // Moving Windows

    unsafe fn setMovableByWindowBackground_(self, movableByWindowBackground: BOOL) {
        msg_send![self, setMovableByWindowBackground:movableByWindowBackground]
    }

    unsafe fn setMovable_(self, movable: BOOL) {
        msg_send![self, setMovable:movable]
    }

    unsafe fn center(self) {
        msg_send![self, center]
    }

    // Closing Windows

    unsafe fn performClose_(self, sender: id) {
        msg_send![self, performClose:sender]
    }

    unsafe fn close(self) {
        msg_send![self, close]
    }

    unsafe fn setReleasedWhenClosed_(self, releasedWhenClosed: BOOL) {
        msg_send![self, setReleasedWhenClosed:releasedWhenClosed]
    }

    // Minimizing Windows

    unsafe fn performMiniaturize_(self, sender: id) {
        msg_send![self, performMiniaturize:sender]
    }

    unsafe fn miniaturize_(self, sender: id) {
        msg_send![self, miniaturize:sender]
    }

    unsafe fn deminiaturize_(self, sender: id) {
        msg_send![self, deminiaturize:sender]
    }

    unsafe fn miniwindowImage(self) -> id {
        msg_send![self, miniwindowImage]
    }

    unsafe fn setMiniwindowImage_(self, miniwindowImage: id) {
        msg_send![self, setMiniwindowImage:miniwindowImage]
    }

    unsafe fn miniwindowTitle(self) -> id {
        msg_send![self, miniwindowTitle]
    }

    unsafe fn setMiniwindowTitle_(self, miniwindowTitle: id) {
        msg_send![self, setMiniwindowTitle:miniwindowTitle]
    }

    unsafe fn setAppearance(self, appearance: id) {
        msg_send![self, setAppearance: appearance]
    }

    // TODO: Getting the Dock Tile
    // TODO: Printing Windows
    // TODO: Providing Services
    // TODO: Working with Carbon
    // TODO: Triggering Constraint-Based Layout
    // TODO: Debugging Constraint-Based Layout
    // TODO: Constraint-Based Layouts
}

pub trait NSPanel: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSPanel), alloc]
    }

    // NSPanel subclasses NSWindow, hence we only add the added methods
    // https://developer.apple.com/documentation/appkit/nspanel
    unsafe fn setBecomesKeyOnlyIfNeeded(self, becomesKeyOnlyIfNeeded: BOOL);
    unsafe fn becomesKeyOnlyIfNeeded(self) -> BOOL;
    unsafe fn setFloatingPanel(self, floatingPanel: BOOL);
    unsafe fn floatingPanel(self) -> BOOL;
    unsafe fn setWorksWhenModal(self, worksWithPanel: BOOL);
}

impl NSPanel for id {
    // NSPanel subclasses NSWindow, hence we only add the added methods
    // https://developer.apple.com/documentation/appkit/nspanel
    unsafe fn setBecomesKeyOnlyIfNeeded(self, becomesKeyOnlyIfNeeded: BOOL) {
        msg_send![self, setBecomesKeyOnlyIfNeeded: becomesKeyOnlyIfNeeded]
    }

    unsafe fn becomesKeyOnlyIfNeeded(self) -> BOOL {
        msg_send![self, becomesKeyOnlyIfNeeded]
    }

    unsafe fn setFloatingPanel(self, floatingPanel: BOOL) {
        msg_send![self, setFloatingPanel: floatingPanel]
    }

    unsafe fn floatingPanel(self) -> BOOL {
        msg_send![self, isFloatingPanel]
    }

    unsafe fn setWorksWhenModal(self, worksWhenModal: BOOL) {
        msg_send![self, setWorksWhenModal: worksWhenModal]
    }
}

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSModalResponse {
    NSModalResponseOk = 1,
    NSModalResponseCancel = 0,
}

pub trait NSSavePanel: Sized {
    unsafe fn savePanel(_: Self) -> id {
        msg_send![class!(NSSavePanel), savePanel]
    }

    unsafe fn setDirectoryURL(self, url: id);
    unsafe fn setCanCreateDirectories(self, canCreateDirectories: BOOL);
    unsafe fn URL(self) -> id;
    unsafe fn runModal(self) -> NSModalResponse;
}

impl NSSavePanel for id {
    unsafe fn setDirectoryURL(self, url: id) {
        msg_send![self, setDirectoryURL: url]
    }

    unsafe fn setCanCreateDirectories(self, canCreateDirectories: BOOL) {
        msg_send![self, setCanCreateDirectories: canCreateDirectories]
    }

    unsafe fn URL(self) -> id {
        msg_send![self, URL]
    }

    unsafe fn runModal(self) -> NSModalResponse {
        msg_send![self, runModal]
    }
}

pub trait NSOpenPanel: NSSavePanel {
    unsafe fn openPanel(_: Self) -> id {
        msg_send![class!(NSOpenPanel), openPanel]
    }

    unsafe fn setCanChooseFiles_(self, canChooseFiles: BOOL);
    unsafe fn setCanChooseDirectories_(self, canChooseDirectories: BOOL);
    unsafe fn setResolvesAliases_(self, resolvesAliases: BOOL);
    unsafe fn setAllowsMultipleSelection_(self, allowsMultipleSelection: BOOL);
    unsafe fn URLs(self) -> id;
}

impl NSOpenPanel for id {
    unsafe fn setCanChooseFiles_(self, canChooseFiles: BOOL) {
        msg_send![self, setCanChooseFiles: canChooseFiles]
    }

    unsafe fn setCanChooseDirectories_(self, canChooseDirectories: BOOL) {
        msg_send![self, setCanChooseDirectories: canChooseDirectories]
    }

    unsafe fn setResolvesAliases_(self, resolvesAliases: BOOL) {
        msg_send![self, setResolvesAliases: resolvesAliases]
    }

    unsafe fn setAllowsMultipleSelection_(self, allowsMultipleSelection: BOOL) {
        msg_send![self, setAllowsMultipleSelection: allowsMultipleSelection]
    }

    unsafe fn URLs(self) -> id {
        msg_send![self, URLs]
    }
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NSViewLayerContentsPlacement {
    NSViewLayerContentsPlacementScaleAxesIndependently = 0,
    NSViewLayerContentsPlacementScaleProportionallyToFit = 1,
    NSViewLayerContentsPlacementScaleProportionallyToFill = 2,
    NSViewLayerContentsPlacementCenter = 3,
    NSViewLayerContentsPlacementTop = 4,
    NSViewLayerContentsPlacementTopRight = 5,
    NSViewLayerContentsPlacementRight = 6,
    NSViewLayerContentsPlacementBottomRight = 7,
    NSViewLayerContentsPlacementBottom = 8,
    NSViewLayerContentsPlacementBottomLeft = 9,
    NSViewLayerContentsPlacementLeft = 10,
    NSViewLayerContentsPlacementTopLeft = 11,
}

pub trait NSView: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSView), alloc]
    }

    unsafe fn init(self) -> id;
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
    unsafe fn bounds(self) -> NSRect;
    unsafe fn frame(self) -> NSRect;
    unsafe fn setFrameSize(self, frameSize: NSSize);
    unsafe fn setFrameOrigin(self, frameOrigin: NSPoint);
    unsafe fn display_(self);
    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: BOOL);
    unsafe fn convertPoint_fromView_(self, point: NSPoint, view: id) -> NSPoint;
    unsafe fn addSubview_(self, view: id);
    unsafe fn superview(self) -> id;
    unsafe fn removeFromSuperview(self);
    unsafe fn setAutoresizingMask_(self, autoresizingMask: NSAutoresizingMaskOptions);

    unsafe fn wantsLayer(self) -> BOOL;
    unsafe fn setWantsLayer(self, wantsLayer: BOOL);
    unsafe fn layer(self) -> id;
    unsafe fn setLayer(self, layer: id);

    unsafe fn widthAnchor(self) -> id;
    unsafe fn heightAnchor(self) -> id;
    unsafe fn convertRectToBacking(self, rect: NSRect) -> NSRect;

    unsafe fn layerContentsPlacement(self) -> NSViewLayerContentsPlacement;
    unsafe fn setLayerContentsPlacement(self, placement: NSViewLayerContentsPlacement);
    unsafe fn setAppearance(self, appearance: id);
}

impl NSView for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send![self, initWithFrame:frameRect]
    }

    unsafe fn bounds(self) -> NSRect {
        msg_send![self, bounds]
    }

    unsafe fn frame(self) -> NSRect {
        msg_send![self, frame]
    }

    unsafe fn setFrameSize(self, frameSize: NSSize) {
        msg_send![self, setFrameSize:frameSize]
    }

    unsafe fn setFrameOrigin(self, frameOrigin: NSPoint) {
        msg_send![self, setFrameOrigin:frameOrigin]
    }

    unsafe fn display_(self) {
        msg_send![self, display]
    }

    unsafe fn setWantsBestResolutionOpenGLSurface_(self, flag: BOOL) {
        msg_send![self, setWantsBestResolutionOpenGLSurface:flag]
    }

    unsafe fn convertPoint_fromView_(self, point: NSPoint, view: id) -> NSPoint {
        msg_send![self, convertPoint:point fromView:view]
    }

    unsafe fn addSubview_(self, view: id) {
        msg_send![self, addSubview:view]
    }

    unsafe fn superview(self) -> id {
        msg_send![self, superview]
    }

    unsafe fn removeFromSuperview(self) {
        msg_send![self, removeFromSuperview]
    }

    unsafe fn setAutoresizingMask_(self, autoresizingMask: NSAutoresizingMaskOptions) {
        msg_send![self, setAutoresizingMask:autoresizingMask]
    }

    unsafe fn wantsLayer(self) -> BOOL {
        msg_send![self, wantsLayer]
    }

    unsafe fn setWantsLayer(self, wantsLayer: BOOL) {
        msg_send![self, setWantsLayer:wantsLayer]
    }

    unsafe fn layer(self) -> id {
        msg_send![self, layer]
    }

    unsafe fn setLayer(self, layer: id) {
        msg_send![self, setLayer:layer]
    }

    unsafe fn widthAnchor(self) -> id {
        msg_send![self, widthAnchor]
    }

    unsafe fn heightAnchor(self) -> id {
        msg_send![self, heightAnchor]
    }

    unsafe fn convertRectToBacking(self, rect: NSRect) -> NSRect {
        msg_send![self, convertRectToBacking:rect]
    }

    unsafe fn layerContentsPlacement(self) -> NSViewLayerContentsPlacement {
        msg_send![self, layerContentsPlacement]
    }

    unsafe fn setLayerContentsPlacement(self, placement: NSViewLayerContentsPlacement) {
        msg_send![self, setLayerContentsPlacement: placement]
    }

    unsafe fn setAppearance(self, appearance: id) {
        msg_send![self, setAppearance: appearance]
    }
}

pub type NSAutoresizingMaskOptions = u64;

pub const NSViewNotSizable: u64 = 0;
pub const NSViewMinXMargin: u64 = 1;
pub const NSViewWidthSizable: u64 = 2;
pub const NSViewMaxXMargin: u64 = 4;
pub const NSViewMinYMargin: u64 = 8;
pub const NSViewHeightSizable: u64 = 16;
pub const NSViewMaxYMargin: u64 = 32;

pub trait NSOpenGLView: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSOpenGLView), alloc]
    }

    unsafe fn initWithFrame_pixelFormat_(self, frameRect: NSRect, format: id) -> id;
    unsafe fn display_(self);
    unsafe fn setOpenGLContext_(self, context: id);
    unsafe fn setPixelFormat_(self, pixelformat: id);
}

impl NSOpenGLView for id {
    unsafe fn initWithFrame_pixelFormat_(self,  frameRect: NSRect, format: id) -> id {
        msg_send![self, initWithFrame:frameRect pixelFormat:format]
    }

    unsafe fn display_(self) {
        msg_send![self, display]
    }

    unsafe fn setOpenGLContext_(self, context: id) {
        msg_send![self, setOpenGLContext:context]
    }

    unsafe fn setPixelFormat_(self, pixelformat: id) {
        msg_send![self, setPixelFormat:pixelformat]
    }
}

pub trait NSOpenGLPixelFormat: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSOpenGLPixelFormat), alloc]
    }

    // Creating an NSOpenGLPixelFormat Object

    unsafe fn initWithAttributes_(self, attributes: &[u32]) -> id;

    // Managing the Pixel Format

    unsafe fn getValues_forAttribute_forVirtualScreen_(self, val: *mut GLint, attrib: NSOpenGLPixelFormatAttribute, screen: GLint);
    unsafe fn numberOfVirtualScreens(self) -> GLint;

}

impl NSOpenGLPixelFormat for id {
    // Creating an NSOpenGLPixelFormat Object

    unsafe fn initWithAttributes_(self, attributes: &[u32]) -> id {
        msg_send![self, initWithAttributes:attributes]
    }

    // Managing the Pixel Format

    unsafe fn getValues_forAttribute_forVirtualScreen_(self, val: *mut GLint, attrib: NSOpenGLPixelFormatAttribute, screen: GLint) {
        msg_send![self, getValues:val forAttribute:attrib forVirtualScreen:screen]
    }

    unsafe fn numberOfVirtualScreens(self) -> GLint {
        msg_send![self, numberOfVirtualScreens]
    }
}

pub trait NSOpenGLContext: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSOpenGLContext), alloc]
    }

    // Context Creation
    unsafe fn initWithFormat_shareContext_(self, format: id /* (NSOpenGLPixelFormat *) */, shareContext: id /* (NSOpenGLContext *) */) -> id /* (instancetype) */;
    unsafe fn initWithCGLContextObj_(self, context: CGLContextObj) -> id /* (instancetype) */;

    // Managing the Current Context
    unsafe fn clearCurrentContext(_: Self);
    unsafe fn currentContext(_: Self) -> id /* (NSOpenGLContext *) */;
    unsafe fn makeCurrentContext(self);

    // Drawable Object Management
    unsafe fn setView_(self, view: id /* (NSView *) */);
    unsafe fn view(self) -> id /* (NSView *) */;
    unsafe fn clearDrawable(self);
    unsafe fn update(self);

    // Flushing the Drawing Buffer
    unsafe fn flushBuffer(self);

    // Context Parameter Handling
    unsafe fn setValues_forParameter_(self, vals: *const GLint, param: NSOpenGLContextParameter);
    unsafe fn getValues_forParameter_(self, vals: *mut GLint, param: NSOpenGLContextParameter);

    // Working with Virtual Screens
    unsafe fn setCurrentVirtualScreen_(self, screen: GLint);
    unsafe fn currentVirtualScreen(self) -> GLint;

    // Getting the CGL Context Object
    unsafe fn CGLContextObj(self) -> CGLContextObj;
}

impl NSOpenGLContext for id {
    // Context Creation

    unsafe fn initWithFormat_shareContext_(self, format: id /* (NSOpenGLPixelFormat *) */, shareContext: id /* (NSOpenGLContext *) */) -> id /* (instancetype) */ {
        msg_send![self, initWithFormat:format shareContext:shareContext]
    }

    unsafe fn initWithCGLContextObj_(self, context: CGLContextObj) -> id /* (instancetype) */ {
        msg_send![self, initWithCGLContextObj:context]
    }

    // Managing the Current Context

    unsafe fn clearCurrentContext(_: Self) {
        msg_send![class!(NSOpenGLContext), clearCurrentContext]
    }

    unsafe fn currentContext(_: Self) -> id /* (NSOpenGLContext *) */ {
        msg_send![class!(NSOpenGLContext), currentContext]
    }

    unsafe fn makeCurrentContext(self) {
        msg_send![self, makeCurrentContext]
    }

    // Drawable Object Management

    unsafe fn setView_(self, view: id /* (NSView *) */) {
        msg_send![self, setView:view]
    }

    unsafe fn view(self) -> id /* (NSView *) */ {
        msg_send![self, view]
    }

    unsafe fn clearDrawable(self) {
        msg_send![self, clearDrawable]
    }

    unsafe fn update(self) {
        msg_send![self, update]
    }

    // Flushing the Drawing Buffer

    unsafe fn flushBuffer(self) {
        msg_send![self, flushBuffer]
    }

    // Context Parameter Handling

    unsafe fn setValues_forParameter_(self, vals: *const GLint, param: NSOpenGLContextParameter) {
        msg_send![self, setValues:vals forParameter:param]
    }

    unsafe fn getValues_forParameter_(self, vals: *mut GLint, param: NSOpenGLContextParameter) {
        msg_send![self, getValues:vals forParameter:param]
    }

    // Working with Virtual Screens

    unsafe fn setCurrentVirtualScreen_(self, screen: GLint) {
        msg_send![self, setCurrentVirtualScreen:screen]
    }

    unsafe fn currentVirtualScreen(self) -> GLint {
        msg_send![self, currentVirtualScreen]
    }

    // Getting the CGL Context Object

    unsafe fn CGLContextObj(self) -> CGLContextObj {
        msg_send![self, CGLContextObj]
    }
}

bitflags! {
    pub struct NSEventSwipeTrackingOptions: NSUInteger {
        const NSEventSwipeTrackingLockDirection         = 0x1 << 0;
        const NSEventSwipeTrackingClampGestureAmount    = 0x1 << 1;
    }
}

#[repr(i64)] // NSInteger
pub enum NSEventGestureAxis {
    NSEventGestureAxisNone = 0,
    NSEventGestureAxisHorizontal,
    NSEventGestureAxisVertical,
}

bitflags! {
    pub struct NSEventPhase: NSUInteger {
       const NSEventPhaseNone        = 0;
       const NSEventPhaseBegan       = 0x1 << 0;
       const NSEventPhaseStationary  = 0x1 << 1;
       const NSEventPhaseChanged     = 0x1 << 2;
       const NSEventPhaseEnded       = 0x1 << 3;
       const NSEventPhaseCancelled   = 0x1 << 4;
       const NSEventPhaseMayBegin    = 0x1 << 5;
    }
}

bitflags! {
    pub struct NSTouchPhase: NSUInteger {
        const NSTouchPhaseBegan         = 1 << 0;
        const NSTouchPhaseMoved         = 1 << 1;
        const NSTouchPhaseStationary    = 1 << 2;
        const NSTouchPhaseEnded         = 1 << 3;
        const NSTouchPhaseCancelled     = 1 << 4;
        const NSTouchPhaseTouching      = NSTouchPhase::NSTouchPhaseBegan.bits
                                        | NSTouchPhase::NSTouchPhaseMoved.bits
                                        | NSTouchPhase::NSTouchPhaseStationary.bits;
        const NSTouchPhaseAny           = !0; // NSUIntegerMax
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u64)] // NSUInteger
pub enum NSEventType {
    NSLeftMouseDown         = 1,
    NSLeftMouseUp           = 2,
    NSRightMouseDown        = 3,
    NSRightMouseUp          = 4,
    NSMouseMoved            = 5,
    NSLeftMouseDragged      = 6,
    NSRightMouseDragged     = 7,
    NSMouseEntered          = 8,
    NSMouseExited           = 9,
    NSKeyDown               = 10,
    NSKeyUp                 = 11,
    NSFlagsChanged          = 12,
    NSAppKitDefined         = 13,
    NSSystemDefined         = 14,
    NSApplicationDefined    = 15,
    NSPeriodic              = 16,
    NSCursorUpdate          = 17,
    NSScrollWheel           = 22,
    NSTabletPoint           = 23,
    NSTabletProximity       = 24,
    NSOtherMouseDown        = 25,
    NSOtherMouseUp          = 26,
    NSOtherMouseDragged     = 27,
    NSEventTypeGesture      = 29,
    NSEventTypeMagnify      = 30,
    NSEventTypeSwipe        = 31,
    NSEventTypeRotate       = 18,
    NSEventTypeBeginGesture = 19,
    NSEventTypeEndGesture   = 20,
    NSEventTypePressure     = 34,
}

bitflags! {
    pub struct NSEventMask: libc::c_ulonglong {
        const NSLeftMouseDownMask         = 1 << NSLeftMouseDown as libc::c_ulonglong;
        const NSLeftMouseUpMask           = 1 << NSLeftMouseUp as libc::c_ulonglong;
        const NSRightMouseDownMask        = 1 << NSRightMouseDown as libc::c_ulonglong;
        const NSRightMouseUpMask          = 1 << NSRightMouseUp as libc::c_ulonglong;
        const NSMouseMovedMask            = 1 << NSMouseMoved as libc::c_ulonglong;
        const NSLeftMouseDraggedMask      = 1 << NSLeftMouseDragged as libc::c_ulonglong;
        const NSRightMouseDraggedMask     = 1 << NSRightMouseDragged as libc::c_ulonglong;
        const NSMouseEnteredMask          = 1 << NSMouseEntered as libc::c_ulonglong;
        const NSMouseExitedMask           = 1 << NSMouseExited as libc::c_ulonglong;
        const NSKeyDownMask               = 1 << NSKeyDown as libc::c_ulonglong;
        const NSKeyUpMask                 = 1 << NSKeyUp as libc::c_ulonglong;
        const NSFlagsChangedMask          = 1 << NSFlagsChanged as libc::c_ulonglong;
        const NSAppKitDefinedMask         = 1 << NSAppKitDefined as libc::c_ulonglong;
        const NSSystemDefinedMask         = 1 << NSSystemDefined as libc::c_ulonglong;
        const NSApplicationDefinedMask    = 1 << NSApplicationDefined as libc::c_ulonglong;
        const NSPeriodicMask              = 1 << NSPeriodic as libc::c_ulonglong;
        const NSCursorUpdateMask          = 1 << NSCursorUpdate as libc::c_ulonglong;
        const NSScrollWheelMask           = 1 << NSScrollWheel as libc::c_ulonglong;
        const NSTabletPointMask           = 1 << NSTabletPoint as libc::c_ulonglong;
        const NSTabletProximityMask       = 1 << NSTabletProximity as libc::c_ulonglong;
        const NSOtherMouseDownMask        = 1 << NSOtherMouseDown as libc::c_ulonglong;
        const NSOtherMouseUpMask          = 1 << NSOtherMouseUp as libc::c_ulonglong;
        const NSOtherMouseDraggedMask     = 1 << NSOtherMouseDragged as libc::c_ulonglong;
        const NSEventMaskGesture          = 1 << NSEventTypeGesture as libc::c_ulonglong;
        const NSEventMaskSwipe            = 1 << NSEventTypeSwipe as libc::c_ulonglong;
        const NSEventMaskRotate           = 1 << NSEventTypeRotate as libc::c_ulonglong;
        const NSEventMaskBeginGesture     = 1 << NSEventTypeBeginGesture as libc::c_ulonglong;
        const NSEventMaskEndGesture       = 1 << NSEventTypeEndGesture as libc::c_ulonglong;
        const NSEventMaskPressure         = 1 << NSEventTypePressure as libc::c_ulonglong;
        const NSAnyEventMask              = 0xffffffffffffffff;
    }
}

impl NSEventMask {
    pub fn from_type(ty: NSEventType) -> NSEventMask {
        NSEventMask { bits: 1 << ty as libc::c_ulonglong }
    }
}

bitflags! {
    pub struct NSEventModifierFlags: NSUInteger {
        const NSAlphaShiftKeyMask                     = 1 << 16;
        const NSShiftKeyMask                          = 1 << 17;
        const NSControlKeyMask                        = 1 << 18;
        const NSAlternateKeyMask                      = 1 << 19;
        const NSCommandKeyMask                        = 1 << 20;
        const NSNumericPadKeyMask                     = 1 << 21;
        const NSHelpKeyMask                           = 1 << 22;
        const NSFunctionKeyMask                       = 1 << 23;
        const NSDeviceIndependentModifierFlagsMask    = 0xffff0000;
    }
}

// Not sure of the type here
pub enum NSPointingDeviceType {
    // TODO: Not sure what these values are
    // NSUnknownPointingDevice = NX_TABLET_POINTER_UNKNOWN,
    // NSPenPointingDevice     = NX_TABLET_POINTER_PEN,
    // NSCursorPointingDevice  = NX_TABLET_POINTER_CURSOR,
    // NSEraserPointingDevice  = NX_TABLET_POINTER_ERASER,
}

// Not sure of the type here
pub enum NSEventButtonMask {
    // TODO: Not sure what these values are
    // NSPenTipMask =       NX_TABLET_BUTTON_PENTIPMASK,
    // NSPenLowerSideMask = NX_TABLET_BUTTON_PENLOWERSIDEMASK,
    // NSPenUpperSideMask = NX_TABLET_BUTTON_PENUPPERSIDEMASK,
}

#[repr(i16)]
pub enum NSEventSubtype {
    // TODO: Not sure what these values are
    // NSMouseEventSubtype           = NX_SUBTYPE_DEFAULT,
    // NSTabletPointEventSubtype     = NX_SUBTYPE_TABLET_POINT,
    // NSTabletProximityEventSubtype = NX_SUBTYPE_TABLET_PROXIMITY
    // NSTouchEventSubtype           = NX_SUBTYPE_MOUSE_TOUCH,
    NSWindowExposedEventType = 0,
    NSApplicationActivatedEventType = 1,
    NSApplicationDeactivatedEventType = 2,
    NSWindowMovedEventType = 4,
    NSScreenChangedEventType = 8,
    NSAWTEventType = 16,
}

pub const NSUpArrowFunctionKey: libc::c_ushort = 0xF700;
pub const NSDownArrowFunctionKey: libc::c_ushort = 0xF701;
pub const NSLeftArrowFunctionKey: libc::c_ushort = 0xF702;
pub const NSRightArrowFunctionKey: libc::c_ushort = 0xF703;
pub const NSF1FunctionKey: libc::c_ushort = 0xF704;
pub const NSF2FunctionKey: libc::c_ushort = 0xF705;
pub const NSF3FunctionKey: libc::c_ushort = 0xF706;
pub const NSF4FunctionKey: libc::c_ushort = 0xF707;
pub const NSF5FunctionKey: libc::c_ushort = 0xF708;
pub const NSF6FunctionKey: libc::c_ushort = 0xF709;
pub const NSF7FunctionKey: libc::c_ushort = 0xF70A;
pub const NSF8FunctionKey: libc::c_ushort = 0xF70B;
pub const NSF9FunctionKey: libc::c_ushort = 0xF70C;
pub const NSF10FunctionKey: libc::c_ushort = 0xF70D;
pub const NSF11FunctionKey: libc::c_ushort = 0xF70E;
pub const NSF12FunctionKey: libc::c_ushort = 0xF70F;
pub const NSF13FunctionKey: libc::c_ushort = 0xF710;
pub const NSF14FunctionKey: libc::c_ushort = 0xF711;
pub const NSF15FunctionKey: libc::c_ushort = 0xF712;
pub const NSF16FunctionKey: libc::c_ushort = 0xF713;
pub const NSF17FunctionKey: libc::c_ushort = 0xF714;
pub const NSF18FunctionKey: libc::c_ushort = 0xF715;
pub const NSF19FunctionKey: libc::c_ushort = 0xF716;
pub const NSF20FunctionKey: libc::c_ushort = 0xF717;
pub const NSF21FunctionKey: libc::c_ushort = 0xF718;
pub const NSF22FunctionKey: libc::c_ushort = 0xF719;
pub const NSF23FunctionKey: libc::c_ushort = 0xF71A;
pub const NSF24FunctionKey: libc::c_ushort = 0xF71B;
pub const NSF25FunctionKey: libc::c_ushort = 0xF71C;
pub const NSF26FunctionKey: libc::c_ushort = 0xF71D;
pub const NSF27FunctionKey: libc::c_ushort = 0xF71E;
pub const NSF28FunctionKey: libc::c_ushort = 0xF71F;
pub const NSF29FunctionKey: libc::c_ushort = 0xF720;
pub const NSF30FunctionKey: libc::c_ushort = 0xF721;
pub const NSF31FunctionKey: libc::c_ushort = 0xF722;
pub const NSF32FunctionKey: libc::c_ushort = 0xF723;
pub const NSF33FunctionKey: libc::c_ushort = 0xF724;
pub const NSF34FunctionKey: libc::c_ushort = 0xF725;
pub const NSF35FunctionKey: libc::c_ushort = 0xF726;
pub const NSInsertFunctionKey: libc::c_ushort = 0xF727;
pub const NSDeleteFunctionKey: libc::c_ushort = 0xF728;
pub const NSHomeFunctionKey: libc::c_ushort = 0xF729;
pub const NSBeginFunctionKey: libc::c_ushort = 0xF72A;
pub const NSEndFunctionKey: libc::c_ushort = 0xF72B;
pub const NSPageUpFunctionKey: libc::c_ushort = 0xF72C;
pub const NSPageDownFunctionKey: libc::c_ushort = 0xF72D;
pub const NSPrintScreenFunctionKey: libc::c_ushort = 0xF72E;
pub const NSScrollLockFunctionKey: libc::c_ushort = 0xF72F;
pub const NSPauseFunctionKey: libc::c_ushort = 0xF730;
pub const NSSysReqFunctionKey: libc::c_ushort = 0xF731;
pub const NSBreakFunctionKey: libc::c_ushort = 0xF732;
pub const NSResetFunctionKey: libc::c_ushort = 0xF733;
pub const NSStopFunctionKey: libc::c_ushort = 0xF734;
pub const NSMenuFunctionKey: libc::c_ushort = 0xF735;
pub const NSUserFunctionKey: libc::c_ushort = 0xF736;
pub const NSSystemFunctionKey: libc::c_ushort = 0xF737;
pub const NSPrintFunctionKey: libc::c_ushort = 0xF738;
pub const NSClearLineFunctionKey: libc::c_ushort = 0xF739;
pub const NSClearDisplayFunctionKey: libc::c_ushort = 0xF73A;
pub const NSInsertLineFunctionKey: libc::c_ushort = 0xF73B;
pub const NSDeleteLineFunctionKey: libc::c_ushort = 0xF73C;
pub const NSInsertCharFunctionKey: libc::c_ushort = 0xF73D;
pub const NSDeleteCharFunctionKey: libc::c_ushort = 0xF73E;
pub const NSPrevFunctionKey: libc::c_ushort = 0xF73F;
pub const NSNextFunctionKey: libc::c_ushort = 0xF740;
pub const NSSelectFunctionKey: libc::c_ushort = 0xF741;
pub const NSExecuteFunctionKey: libc::c_ushort = 0xF742;
pub const NSUndoFunctionKey: libc::c_ushort = 0xF743;
pub const NSRedoFunctionKey: libc::c_ushort = 0xF744;
pub const NSFindFunctionKey: libc::c_ushort = 0xF745;
pub const NSHelpFunctionKey: libc::c_ushort = 0xF746;
pub const NSModeSwitchFunctionKey: libc::c_ushort = 0xF747;

pub trait NSEvent: Sized {
    // Creating Events
    unsafe fn keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        characters: id /* (NSString *) */,
        unmodCharacters: id /* (NSString *) */,
        repeatKey: BOOL,
        code: libc::c_ushort) -> id /* (NSEvent *) */;
    unsafe fn mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        clickCount: NSInteger,
        pressure: libc::c_float) -> id /* (NSEvent *) */;
    unsafe fn enterExitEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_trackingNumber_userData_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        trackingNumber: NSInteger,
        userData: *mut c_void) -> id /* (NSEvent *) */;
    unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        subtype: NSEventSubtype,
        data1: NSInteger,
        data2: NSInteger) -> id /* (NSEvent *) */;
    unsafe fn eventWithEventRef_(_: Self, eventRef: *const c_void) -> id;
    unsafe fn eventWithCGEvent_(_: Self, cgEvent: *mut c_void /* CGEventRef */) -> id;

    // Getting General Event Information
    unsafe fn context(self) -> id /* (NSGraphicsContext *) */;
    unsafe fn locationInWindow(self) -> NSPoint;
    unsafe fn modifierFlags(self) -> NSEventModifierFlags;
    unsafe fn timestamp(self) -> NSTimeInterval;
    // NOTE: renamed from `- type` due to Rust keyword collision
    unsafe fn eventType(self) -> NSEventType;
    unsafe fn window(self) -> id /* (NSWindow *) */;
    unsafe fn windowNumber(self) -> NSInteger;
    unsafe fn eventRef(self) -> *const c_void;
    unsafe fn CGEvent(self) -> *mut c_void /* CGEventRef */;

    // Getting Key Event Information
    // NOTE: renamed from `+ modifierFlags` due to conflict with `- modifierFlags`
    unsafe fn currentModifierFlags(_: Self) -> NSEventModifierFlags;
    unsafe fn keyRepeatDelay(_: Self) -> NSTimeInterval;
    unsafe fn keyRepeatInterval(_: Self) -> NSTimeInterval;
    unsafe fn characters(self) -> id /* (NSString *) */;
    unsafe fn charactersIgnoringModifiers(self) -> id /* (NSString *) */;
    unsafe fn keyCode(self) -> libc::c_ushort;
    unsafe fn isARepeat(self) -> BOOL;

    // Getting Mouse Event Information
    unsafe fn pressedMouseButtons(_: Self) -> NSUInteger;
    unsafe fn doubleClickInterval(_: Self) -> NSTimeInterval;
    unsafe fn mouseLocation(_: Self) -> NSPoint;
    unsafe fn buttonNumber(self) -> NSInteger;
    unsafe fn clickCount(self) -> NSInteger;
    unsafe fn pressure(self) -> libc::c_float;
    unsafe fn stage(self) -> NSInteger;
    unsafe fn setMouseCoalescingEnabled_(_: Self, flag: BOOL);
    unsafe fn isMouseCoalescingEnabled(_: Self) -> BOOL;

    // Getting Mouse-Tracking Event Information
    unsafe fn eventNumber(self) -> NSInteger;
    unsafe fn trackingNumber(self) -> NSInteger;
    unsafe fn trackingArea(self) -> id /* (NSTrackingArea *) */;
    unsafe fn userData(self) -> *const c_void;

    // Getting Custom Event Information
    unsafe fn data1(self) -> NSInteger;
    unsafe fn data2(self) -> NSInteger;
    unsafe fn subtype(self) -> NSEventSubtype;

    // Getting Scroll Wheel Event Information
    unsafe fn deltaX(self) -> CGFloat;
    unsafe fn deltaY(self) -> CGFloat;
    unsafe fn deltaZ(self) -> CGFloat;

    // Getting Tablet Proximity Information
    unsafe fn capabilityMask(self) -> NSUInteger;
    unsafe fn deviceID(self) -> NSUInteger;
    unsafe fn pointingDeviceID(self) -> NSUInteger;
    unsafe fn pointingDeviceSerialNumber(self) -> NSUInteger;
    unsafe fn pointingDeviceType(self) -> NSPointingDeviceType;
    unsafe fn systemTabletID(self) -> NSUInteger;
    unsafe fn tabletID(self) -> NSUInteger;
    unsafe fn uniqueID(self) -> libc::c_ulonglong;
    unsafe fn vendorID(self) -> NSUInteger;
    unsafe fn vendorPointingDeviceType(self) -> NSUInteger;

    // Getting Tablet Pointing Information
    unsafe fn absoluteX(self) -> NSInteger;
    unsafe fn absoluteY(self) -> NSInteger;
    unsafe fn absoluteZ(self) -> NSInteger;
    unsafe fn buttonMask(self) -> NSEventButtonMask;
    unsafe fn rotation(self) -> libc::c_float;
    unsafe fn tangentialPressure(self) -> libc::c_float;
    unsafe fn tilt(self) -> NSPoint;
    unsafe fn vendorDefined(self) -> id;

    // Requesting and Stopping Periodic Events
    unsafe fn startPeriodicEventsAfterDelay_withPeriod_(_: Self, delaySeconds: NSTimeInterval, periodSeconds: NSTimeInterval);
    unsafe fn stopPeriodicEvents(_: Self);

    // Getting Touch and Gesture Information
    unsafe fn magnification(self) -> CGFloat;
    unsafe fn touchesMatchingPhase_inView_(self, phase: NSTouchPhase, view: id /* (NSView *) */) -> id /* (NSSet *) */;
    unsafe fn isSwipeTrackingFromScrollEventsEnabled(_: Self) -> BOOL;

    // Monitoring Application Events
    // TODO: addGlobalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)
    // TODO: addLocalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)
    unsafe fn removeMonitor_(_: Self, eventMonitor: id);

    // Scroll Wheel and Flick Events
    unsafe fn hasPreciseScrollingDeltas(self) -> BOOL;
    unsafe fn scrollingDeltaX(self) -> CGFloat;
    unsafe fn scrollingDeltaY(self) -> CGFloat;
    unsafe fn momentumPhase(self) -> NSEventPhase;
    unsafe fn phase(self) -> NSEventPhase;
    // TODO: trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler_ (unsure how to bind to blocks)

    // Converting a Mouse Event’s Position into a Sprite Kit Node’s Coordinate Space
    unsafe fn locationInNode_(self, node: id /* (SKNode *) */) -> CGPoint;
}

impl NSEvent for id {
    // Creating Events

    unsafe fn keyEventWithType_location_modifierFlags_timestamp_windowNumber_context_characters_charactersIgnoringModifiers_isARepeat_keyCode_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        characters: id /* (NSString *) */,
        unmodCharacters: id /* (NSString *) */,
        repeatKey: BOOL,
        code: libc::c_ushort) -> id /* (NSEvent *) */
    {
        msg_send![class!(NSEvent), keyEventWithType:eventType
                                            location:location
                                       modifierFlags:modifierFlags
                                           timestamp:timestamp
                                        windowNumber:windowNumber
                                             context:context
                                          characters:characters
                         charactersIgnoringModifiers:unmodCharacters
                                           isARepeat:repeatKey
                                             keyCode:code]
    }

    unsafe fn mouseEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_clickCount_pressure_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        clickCount: NSInteger,
        pressure: libc::c_float) -> id /* (NSEvent *) */
    {
        msg_send![class!(NSEvent), mouseEventWithType:eventType
                                              location:location
                                         modifierFlags:modifierFlags
                                             timestamp:timestamp
                                          windowNumber:windowNumber
                                               context:context
                                           eventNumber:eventNumber
                                            clickCount:clickCount
                                              pressure:pressure]
    }

    unsafe fn enterExitEventWithType_location_modifierFlags_timestamp_windowNumber_context_eventNumber_trackingNumber_userData_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        eventNumber: NSInteger,
        trackingNumber: NSInteger,
        userData: *mut c_void) -> id /* (NSEvent *) */
    {
        msg_send![class!(NSEvent), enterExitEventWithType:eventType
                                                  location:location
                                             modifierFlags:modifierFlags
                                                 timestamp:timestamp
                                              windowNumber:windowNumber
                                                   context:context
                                               eventNumber:eventNumber
                                            trackingNumber:trackingNumber
                                                  userData:userData]
    }

    unsafe fn otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2_(
        _: Self,
        eventType: NSEventType,
        location: NSPoint,
        modifierFlags: NSEventModifierFlags,
        timestamp: NSTimeInterval,
        windowNumber: NSInteger,
        context: id /* (NSGraphicsContext *) */,
        subtype: NSEventSubtype,
        data1: NSInteger,
        data2: NSInteger) -> id /* (NSEvent *) */
    {
        msg_send![class!(NSEvent), otherEventWithType:eventType
                                              location:location
                                         modifierFlags:modifierFlags
                                             timestamp:timestamp
                                          windowNumber:windowNumber
                                               context:context
                                               subtype:subtype
                                                 data1:data1
                                                 data2:data2]
    }

    unsafe fn eventWithEventRef_(_: Self, eventRef: *const c_void) -> id {
        msg_send![class!(NSEvent), eventWithEventRef:eventRef]
    }

    unsafe fn eventWithCGEvent_(_: Self, cgEvent: *mut c_void /* CGEventRef */) -> id {
        msg_send![class!(NSEvent), eventWithCGEvent:cgEvent]
    }

    // Getting General Event Information

    unsafe fn context(self) -> id /* (NSGraphicsContext *) */ {
        msg_send![self, context]
    }

    unsafe fn locationInWindow(self) -> NSPoint {
        msg_send![self, locationInWindow]
    }

    unsafe fn modifierFlags(self) -> NSEventModifierFlags {
        msg_send![self, modifierFlags]
    }

    unsafe fn timestamp(self) -> NSTimeInterval {
        msg_send![self, timestamp]
    }
    // NOTE: renamed from `- type` due to Rust keyword collision

    unsafe fn eventType(self) -> NSEventType {
        msg_send![self, type]
    }

    unsafe fn window(self) -> id /* (NSWindow *) */ {
        msg_send![self, window]
    }

    unsafe fn windowNumber(self) -> NSInteger {
        msg_send![self, windowNumber]
    }

    unsafe fn eventRef(self) -> *const c_void {
        msg_send![self, eventRef]
    }

    unsafe fn CGEvent(self) -> *mut c_void /* CGEventRef */ {
        msg_send![self, CGEvent]
    }

    // Getting Key Event Information

    // NOTE: renamed from `+ modifierFlags` due to conflict with `- modifierFlags`

    unsafe fn currentModifierFlags(_: Self) -> NSEventModifierFlags {
        msg_send![class!(NSEvent), currentModifierFlags]
    }

    unsafe fn keyRepeatDelay(_: Self) -> NSTimeInterval {
        msg_send![class!(NSEvent), keyRepeatDelay]
    }

    unsafe fn keyRepeatInterval(_: Self) -> NSTimeInterval {
        msg_send![class!(NSEvent), keyRepeatInterval]
    }

    unsafe fn characters(self) -> id /* (NSString *) */ {
        msg_send![self, characters]
    }

    unsafe fn charactersIgnoringModifiers(self) -> id /* (NSString *) */ {
        msg_send![self, charactersIgnoringModifiers]
    }

    unsafe fn keyCode(self) -> libc::c_ushort {
        msg_send![self, keyCode]
    }

    unsafe fn isARepeat(self) -> BOOL {
        msg_send![self, isARepeat]
    }

    // Getting Mouse Event Information

    unsafe fn pressedMouseButtons(_: Self) -> NSUInteger {
        msg_send![class!(NSEvent), pressedMouseButtons]
    }

    unsafe fn doubleClickInterval(_: Self) -> NSTimeInterval {
        msg_send![class!(NSEvent), doubleClickInterval]
    }

    unsafe fn mouseLocation(_: Self) -> NSPoint {
        msg_send![class!(NSEvent), mouseLocation]
    }

    unsafe fn buttonNumber(self) -> NSInteger {
        msg_send![self, buttonNumber]
    }

    unsafe fn clickCount(self) -> NSInteger {
        msg_send![self, clickCount]
    }

    unsafe fn pressure(self) -> libc::c_float {
        msg_send![self, pressure]
    }

    unsafe fn stage(self) -> NSInteger{
        msg_send![self, stage]
    }

    unsafe fn setMouseCoalescingEnabled_(_: Self, flag: BOOL) {
        msg_send![class!(NSEvent), setMouseCoalescingEnabled:flag]
    }

    unsafe fn isMouseCoalescingEnabled(_: Self) -> BOOL {
        msg_send![class!(NSEvent), isMouseCoalescingEnabled]
    }

    // Getting Mouse-Tracking Event Information

    unsafe fn eventNumber(self) -> NSInteger {
        msg_send![self, eventNumber]
    }

    unsafe fn trackingNumber(self) -> NSInteger {
        msg_send![self, trackingNumber]
    }

    unsafe fn trackingArea(self) -> id /* (NSTrackingArea *) */ {
        msg_send![self, trackingArea]
    }

    unsafe fn userData(self) -> *const c_void {
        msg_send![self, userData]
    }

    // Getting Custom Event Information

    unsafe fn data1(self) -> NSInteger {
        msg_send![self, data1]
    }

    unsafe fn data2(self) -> NSInteger {
        msg_send![self, data2]
    }

    unsafe fn subtype(self) -> NSEventSubtype {
        msg_send![self, subtype]
    }

    // Getting Scroll Wheel Event Information

    unsafe fn deltaX(self) -> CGFloat {
        msg_send![self, deltaX]
    }

    unsafe fn deltaY(self) -> CGFloat {
        msg_send![self, deltaY]
    }

    unsafe fn deltaZ(self) -> CGFloat {
        msg_send![self, deltaZ]
    }

    // Getting Tablet Proximity Information

    unsafe fn capabilityMask(self) -> NSUInteger {
        msg_send![self, capabilityMask]
    }

    unsafe fn deviceID(self) -> NSUInteger {
        msg_send![self, deviceID]
    }

    unsafe fn pointingDeviceID(self) -> NSUInteger {
        msg_send![self, pointingDeviceID]
    }

    unsafe fn pointingDeviceSerialNumber(self) -> NSUInteger {
        msg_send![self, pointingDeviceSerialNumber]
    }

    unsafe fn pointingDeviceType(self) -> NSPointingDeviceType {
        msg_send![self, pointingDeviceType]
    }

    unsafe fn systemTabletID(self) -> NSUInteger {
        msg_send![self, systemTabletID]
    }

    unsafe fn tabletID(self) -> NSUInteger {
        msg_send![self, tabletID]
    }

    unsafe fn uniqueID(self) -> libc::c_ulonglong {
        msg_send![self, uniqueID]
    }

    unsafe fn vendorID(self) -> NSUInteger {
        msg_send![self, vendorID]
    }

    unsafe fn vendorPointingDeviceType(self) -> NSUInteger {
        msg_send![self, vendorPointingDeviceType]
    }

    // Getting Tablet Pointing Information

    unsafe fn absoluteX(self) -> NSInteger {
        msg_send![self, absoluteX]
    }

    unsafe fn absoluteY(self) -> NSInteger {
        msg_send![self, absoluteY]
    }

    unsafe fn absoluteZ(self) -> NSInteger {
        msg_send![self, absoluteZ]
    }

    unsafe fn buttonMask(self) -> NSEventButtonMask {
        msg_send![self, buttonMask]
    }

    unsafe fn rotation(self) -> libc::c_float {
        msg_send![self, rotation]
    }

    unsafe fn tangentialPressure(self) -> libc::c_float {
        msg_send![self, tangentialPressure]
    }

    unsafe fn tilt(self) -> NSPoint {
        msg_send![self, tilt]
    }

    unsafe fn vendorDefined(self) -> id {
        msg_send![self, vendorDefined]
    }

    // Requesting and Stopping Periodic Events

    unsafe fn startPeriodicEventsAfterDelay_withPeriod_(_: Self, delaySeconds: NSTimeInterval, periodSeconds: NSTimeInterval) {
        msg_send![class!(NSEvent), startPeriodicEventsAfterDelay:delaySeconds withPeriod:periodSeconds]
    }

    unsafe fn stopPeriodicEvents(_: Self) {
        msg_send![class!(NSEvent), stopPeriodicEvents]
    }

    // Getting Touch and Gesture Information

    unsafe fn magnification(self) -> CGFloat {
        msg_send![self, magnification]
    }

    unsafe fn touchesMatchingPhase_inView_(self, phase: NSTouchPhase, view: id /* (NSView *) */) -> id /* (NSSet *) */ {
        msg_send![self, touchesMatchingPhase:phase inView:view]
    }

    unsafe fn isSwipeTrackingFromScrollEventsEnabled(_: Self) -> BOOL {
        msg_send![class!(NSEvent), isSwipeTrackingFromScrollEventsEnabled]
    }

    // Monitoring Application Events

    // TODO: addGlobalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)
    // TODO: addLocalMonitorForEventsMatchingMask_handler_ (unsure how to bind to blocks)

    unsafe fn removeMonitor_(_: Self, eventMonitor: id) {
        msg_send![class!(NSEvent), removeMonitor:eventMonitor]
    }

    // Scroll Wheel and Flick Events

    unsafe fn hasPreciseScrollingDeltas(self) -> BOOL {
        msg_send![self, hasPreciseScrollingDeltas]
    }

    unsafe fn scrollingDeltaX(self) -> CGFloat {
        msg_send![self, scrollingDeltaX]
    }

    unsafe fn scrollingDeltaY(self) -> CGFloat {
        msg_send![self, scrollingDeltaY]
    }

    unsafe fn momentumPhase(self) -> NSEventPhase {
        msg_send![self, momentumPhase]
    }

    unsafe fn phase(self) -> NSEventPhase {
        msg_send![self, phase]
    }

    // TODO: trackSwipeEventWithOptions_dampenAmountThresholdMin_max_usingHandler_ (unsure how to bind to blocks)

    // Converting a Mouse Event’s Position into a Sprite Kit Node’s Coordinate Space
    unsafe fn locationInNode_(self, node: id /* (SKNode *) */) -> CGPoint {
        msg_send![self, locationInNode:node]
    }
}

pub trait NSScreen: Sized {
    // Getting NSScreen Objects
    unsafe fn mainScreen(_: Self) -> id /* (NSScreen *) */;
    unsafe fn deepestScreen(_: Self) -> id /* (NSScreen *) */;
    unsafe fn screens(_: Self) -> id /* (NSArray *) */;

    // Getting Screen Information
    unsafe fn depth(self) -> NSWindowDepth;
    unsafe fn frame(self) -> NSRect;
    unsafe fn supportedWindowDepths(self) -> *const NSWindowDepth;
    unsafe fn deviceDescription(self) -> id /* (NSDictionary *) */;
    unsafe fn visibleFrame(self) -> NSRect;
    unsafe fn colorSpace(self) -> id /* (NSColorSpace *) */;
    unsafe fn screensHaveSeparateSpaces(_: Self) -> BOOL;

    // Screen Backing Coordinate Conversion
    unsafe fn backingAlignedRect_options_(self, aRect: NSRect, options: NSAlignmentOptions) -> NSRect;
    unsafe fn backingScaleFactor(self) -> CGFloat;
    unsafe fn convertRectFromBacking_(self, aRect: NSRect) -> NSRect;
    unsafe fn convertRectToBacking_(self, aRect: NSRect) -> NSRect;
}

impl NSScreen for id {
    // Getting NSScreen Objects

    unsafe fn mainScreen(_: Self) -> id /* (NSScreen *) */ {
        msg_send![class!(NSScreen), mainScreen]
    }

    unsafe fn deepestScreen(_: Self) -> id /* (NSScreen *) */ {
        msg_send![class!(NSScreen), deepestScreen]
    }

    unsafe fn screens(_: Self) -> id /* (NSArray *) */ {
        msg_send![class!(NSScreen), screens]
    }

    // Getting Screen Information

    unsafe fn depth(self) -> NSWindowDepth {
        msg_send![self, depth]
    }

    unsafe fn frame(self) -> NSRect {
        msg_send![self, frame]
    }

    unsafe fn supportedWindowDepths(self) -> *const NSWindowDepth {
        msg_send![self, supportedWindowDepths]
    }

    unsafe fn deviceDescription(self) -> id /* (NSDictionary *) */ {
        msg_send![self, deviceDescription]
    }

    unsafe fn visibleFrame(self) -> NSRect {
        msg_send![self, visibleFrame]
    }

    unsafe fn colorSpace(self) -> id /* (NSColorSpace *) */ {
        msg_send![self, colorSpace]
    }

    unsafe fn screensHaveSeparateSpaces(_: Self) -> BOOL {
        msg_send![class!(NSScreen), screensHaveSeparateSpaces]
    }

    // Screen Backing Coordinate Conversion

    unsafe fn backingAlignedRect_options_(self, aRect: NSRect, options: NSAlignmentOptions) -> NSRect {
        msg_send![self, backingAlignedRect:aRect options:options]
    }

    unsafe fn backingScaleFactor(self) -> CGFloat {
        msg_send![self, backingScaleFactor]
    }

    unsafe fn convertRectFromBacking_(self, aRect: NSRect) -> NSRect {
        msg_send![self, convertRectFromBacking:aRect]
    }

    unsafe fn convertRectToBacking_(self, aRect: NSRect) -> NSRect {
        msg_send![self, convertRectToBacking:aRect]
    }
}

// https://developer.apple.com/documentation/appkit/nscontrol?language=objc
pub trait NSControl: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSControl), alloc]
    }
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
    unsafe fn isEnabled_(self) -> BOOL;
    unsafe fn setEnabled_(self, enabled: BOOL) -> BOOL;
}

impl NSControl for id {
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send![self, initWithFrame:frameRect]
    }
    unsafe fn isEnabled_(self) -> BOOL {
        msg_send![self, isEnabled]
    }
    unsafe fn setEnabled_(self, enabled: BOOL) -> BOOL {
        msg_send![self, setEnabled:enabled]
    }
}

pub trait NSImageView: Sized {
     unsafe fn alloc(_: Self) -> id {
         msg_send![class!(NSImageView), alloc]
     }
     unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
     unsafe fn setImage_(self, img: id /* (NSImage *) */);
}

impl NSImageView for id {
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send![self, initWithFrame:frameRect]
    }
    unsafe fn setImage_(self, img: id /* (NSImage *) */) {
        msg_send![self, setImage:img]
    }
}

pub trait NSButton: Sized {
     unsafe fn setImage_(self, img: id /* (NSImage *) */);
     unsafe fn setBezelStyle_(self, style: NSBezelStyle);
     unsafe fn setTitle_(self, title: id /* (NSString*) */);
     unsafe fn alloc(_: Self) -> id {
         msg_send![class!(NSButton), alloc]
     }
     unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
     unsafe fn setTarget_(self, target: id /* Instance */);
     unsafe fn setAction_(self, selector: objc::runtime::Sel /* (Instance *) */);
}

impl NSButton for id {
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send![self, initWithFrame:frameRect]
    }
    unsafe fn setBezelStyle_(self, style: NSBezelStyle) {
        msg_send![self, setBezelStyle:style]
    }
    unsafe fn setTitle_(self, title: id /* (NSString*) */) {
        msg_send![self, setTitle:title]
    }
    unsafe fn setImage_(self, img: id /* (NSImage *) */) {
        msg_send![self, setImage:img]
    }
    unsafe fn setTarget_(self, target: id /* (Instance *) */) {
        msg_send![self, setTarget:target]
    }

    unsafe fn setAction_(self, selector: objc::runtime::Sel /* (Instance method *) */) {
        msg_send![self, setAction:selector]
    }
}

pub trait NSImage: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSImage), alloc]
    }

    unsafe fn initByReferencingFile_(self, file_name: id /* (NSString *) */) -> id;
    unsafe fn initWithContentsOfFile_(self, file_name: id /* (NSString *) */) -> id;
    unsafe fn initWithData_(self, data: id /* (NSData *) */) -> id;
    unsafe fn initWithDataIgnoringOrientation_(self, data: id /* (NSData *) */) -> id;
    unsafe fn initWithPasteboard_(self, pasteboard: id /* (NSPasteboard *) */) -> id;
    unsafe fn initWithSize_flipped_drawingHandler_(self, size: NSSize,
                                                   drawingHandlerShouldBeCalledWithFlippedContext: BOOL,
                                                   drawingHandler: *mut Block<(NSRect,), BOOL>);
    unsafe fn initWithSize_(self, aSize: NSSize) -> id;

    unsafe fn imageNamed_(_: Self, name: id /* (NSString *) */) -> id {
        msg_send![class!(NSImage), imageNamed:name]
    }

    unsafe fn name(self) -> id /* (NSString *) */;
    unsafe fn setName_(self, name: id /* (NSString *) */) -> BOOL;

    unsafe fn size(self) -> NSSize;
    unsafe fn template(self) -> BOOL;

    unsafe fn canInitWithPasteboard_(self, pasteboard: id /* (NSPasteboard *) */) -> BOOL;
    unsafe fn imageTypes(self) -> id /* (NSArray<NSString *> ) */;
    unsafe fn imageUnfilteredTypes(self) -> id /* (NSArray<NSString *> ) */;

    unsafe fn addRepresentation_(self, imageRep: id /* (NSImageRep *) */);
    unsafe fn addRepresentations_(self, imageReps: id /* (NSArray<NSImageRep *> *) */);
    unsafe fn representations(self) -> id /* (NSArray<NSImageRep *> *) */;
    unsafe fn removeRepresentation_(self, imageRep: id /* (NSImageRep *) */);
    unsafe fn bestRepresentationForRect_context_hints_(self, rect: NSRect,
                                                       referenceContext: id /* (NSGraphicsContext *) */,
                                                       hints: id /* (NSDictionary<NSString *, id> *) */)
                                                       -> id /* (NSImageRep *) */;
    unsafe fn prefersColorMatch(self) -> BOOL;
    unsafe fn usesEPSOnResolutionMismatch(self) -> BOOL;
    unsafe fn matchesOnMultipleResolution(self) -> BOOL;

    unsafe fn drawInRect_(self, rect: NSRect);
    unsafe fn drawAtPoint_fromRect_operation_fraction_(self, point: NSPoint, srcRect: NSRect,
                                                       op: NSCompositingOperation, delta: CGFloat);
    unsafe fn drawInRect_fromRect_operation_fraction_(self, dstRect: NSRect, srcRect: NSRect,
                                                      op: NSCompositingOperation, delta: CGFloat);
    unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints_(self, dstSpacePortionRect: NSRect,
        srcSpacePortionRect: NSRect, op: NSCompositingOperation, delta: CGFloat, respectContextIsFlipped: BOOL,
        hints: id /* (NSDictionary<NSString *, id> *) */);
    unsafe fn drawRepresentation_inRect_(self, imageRep: id /* (NSImageRep *) */, dstRect: NSRect);

    unsafe fn isValid(self) -> BOOL;
    unsafe fn backgroundColor(self) -> id /* (NSColor *) */;

    unsafe fn lockFocus(self);
    unsafe fn lockFocusFlipped_(self, flipped: BOOL);
    unsafe fn unlockFocus(self);

    unsafe fn alignmentRect(self) -> NSRect;

    unsafe fn cacheMode(self) -> NSImageCacheMode;
    unsafe fn recache(self);

    unsafe fn delegate(self) -> id /* (id<NSImageDelegate *> *) */;

    unsafe fn TIFFRepresentation(self) -> id /* (NSData *) */;
    unsafe fn TIFFRepresentationUsingCompression_factor_(self, comp: NSTIFFCompression, aFloat: f32)
                                                         -> id /* (NSData *) */;

    unsafe fn cancelIncrementalLoad(self);

    unsafe fn hitTestRect_withImageDestinationRect_context_hints_flipped_(self, testRectDestSpace: NSRect,
        imageRectDestSpace: NSRect, referenceContext: id /* (NSGraphicsContext *) */,
        hints: id /* (NSDictionary<NSString *, id> *) */, flipped: BOOL) -> BOOL;

    unsafe fn accessibilityDescription(self) -> id /* (NSString *) */;

    unsafe fn layerContentsForContentsScale_(self, layerContentsScale: CGFloat) -> id /* (id) */;
    unsafe fn recommendedLayerContentsScale_(self, preferredContentsScale: CGFloat) -> CGFloat;

    unsafe fn matchesOnlyOnBestFittingAxis(self) -> BOOL;
}

impl NSImage for id {
    unsafe fn initByReferencingFile_(self, file_name: id /* (NSString *) */) -> id {
        msg_send![self, initByReferencingFile:file_name]
    }

    unsafe fn initWithContentsOfFile_(self, file_name: id /* (NSString *) */) -> id {
        msg_send![self, initWithContentsOfFile:file_name]
    }

    unsafe fn initWithData_(self, data: id /* (NSData *) */) -> id {
        msg_send![self, initWithData:data]
    }

    unsafe fn initWithDataIgnoringOrientation_(self, data: id /* (NSData *) */) -> id {
        msg_send![self, initWithDataIgnoringOrientation:data]
    }

    unsafe fn initWithPasteboard_(self, pasteboard: id /* (NSPasteboard *) */) -> id {
        msg_send![self, initWithPasteboard:pasteboard]
    }

    unsafe fn initWithSize_flipped_drawingHandler_(self, size: NSSize,
                                                   drawingHandlerShouldBeCalledWithFlippedContext: BOOL,
                                                   drawingHandler: *mut Block<(NSRect,), BOOL>) {
        msg_send![self, initWithSize:size
                             flipped:drawingHandlerShouldBeCalledWithFlippedContext
                      drawingHandler:drawingHandler]
    }

    unsafe fn initWithSize_(self, aSize: NSSize) -> id {
        msg_send![self, initWithSize:aSize]
    }

    unsafe fn name(self) -> id /* (NSString *) */ {
        msg_send![self, name]
    }

    unsafe fn setName_(self, name: id /* (NSString *) */) -> BOOL {
        msg_send![self, setName:name]
    }

    unsafe fn size(self) -> NSSize {
        msg_send![self, size]
    }

    unsafe fn template(self) -> BOOL {
        msg_send![self, template]
    }

    unsafe fn canInitWithPasteboard_(self, pasteboard: id /* (NSPasteboard *) */) -> BOOL {
        msg_send![self, canInitWithPasteboard:pasteboard]
    }

    unsafe fn imageTypes(self) -> id /* (NSArray<NSString *> ) */ {
        msg_send![self, imageTypes]
    }

    unsafe fn imageUnfilteredTypes(self) -> id /* (NSArray<NSString *> ) */ {
        msg_send![self, imageUnfilteredTypes]
    }

    unsafe fn addRepresentation_(self, imageRep: id /* (NSImageRep *) */) {
        msg_send![self, addRepresentation:imageRep]
    }

    unsafe fn addRepresentations_(self, imageReps: id /* (NSArray<NSImageRep *> *) */) {
        msg_send![self, addRepresentations:imageReps]
    }

    unsafe fn representations(self) -> id /* (NSArray<NSImageRep *> *) */ {
        msg_send![self, representations]
    }

    unsafe fn removeRepresentation_(self, imageRep: id /* (NSImageRep *) */) {
        msg_send![self, removeRepresentation:imageRep]
    }

    unsafe fn bestRepresentationForRect_context_hints_(self, rect: NSRect,
                                                       referenceContext: id /* (NSGraphicsContext *) */,
                                                       hints: id /* (NSDictionary<NSString *, id> *) */)
                                                       -> id /* (NSImageRep *) */ {
        msg_send![self, bestRepresentationForRect:rect context:referenceContext hints:hints]
    }

    unsafe fn prefersColorMatch(self) -> BOOL {
        msg_send![self, prefersColorMatch]
    }

    unsafe fn usesEPSOnResolutionMismatch(self) -> BOOL {
        msg_send![self, usesEPSOnResolutionMismatch]
    }

    unsafe fn matchesOnMultipleResolution(self) -> BOOL {
        msg_send![self, matchesOnMultipleResolution]
    }

    unsafe fn drawInRect_(self, rect: NSRect) {
        msg_send![self, drawInRect:rect]
    }

    unsafe fn drawAtPoint_fromRect_operation_fraction_(self, point: NSPoint, srcRect: NSRect,
                                                       op: NSCompositingOperation, delta: CGFloat) {
        msg_send![self, drawAtPoint:point fromRect:srcRect operation:op fraction:delta]
    }

    unsafe fn drawInRect_fromRect_operation_fraction_(self, dstRect: NSRect, srcRect: NSRect,
                                                      op: NSCompositingOperation, delta: CGFloat) {
        msg_send![self, drawInRect:dstRect fromRect:srcRect operation:op fraction:delta]
    }

    unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints_(self, dstSpacePortionRect: NSRect,
        srcSpacePortionRect: NSRect, op: NSCompositingOperation, delta: CGFloat, respectContextIsFlipped: BOOL,
        hints: id /* (NSDictionary<NSString *, id> *) */) {
        msg_send![self, drawInRect:dstSpacePortionRect
                          fromRect:srcSpacePortionRect
                         operation:op
                          fraction:delta
                    respectFlipped:respectContextIsFlipped
                             hints:hints]
    }

    unsafe fn drawRepresentation_inRect_(self, imageRep: id /* (NSImageRep *) */, dstRect: NSRect) {
        msg_send![self, drawRepresentation:imageRep inRect:dstRect]
    }

    unsafe fn isValid(self) -> BOOL {
        msg_send![self, isValid]
    }

    unsafe fn backgroundColor(self) -> id /* (NSColor *) */ {
        msg_send![self, backgroundColor]
    }

    unsafe fn lockFocus(self) {
        msg_send![self, lockFocus]
    }

    unsafe fn lockFocusFlipped_(self, flipped: BOOL) {
        msg_send![self, lockFocusFlipped:flipped]
    }

    unsafe fn unlockFocus(self) {
        msg_send![self, unlockFocus]
    }

    unsafe fn alignmentRect(self) -> NSRect {
        msg_send![self, alignmentRect]
    }

    unsafe fn cacheMode(self) -> NSImageCacheMode {
        msg_send![self, cacheMode]
    }

    unsafe fn recache(self) {
        msg_send![self, recache]
    }

    unsafe fn delegate(self) -> id /* (id<NSImageDelegate *> *) */ {
        msg_send![self, delegate]
    }

    unsafe fn TIFFRepresentation(self) -> id /* (NSData *) */ {
        msg_send![self, TIFFRepresentation]
    }

    unsafe fn TIFFRepresentationUsingCompression_factor_(self, comp: NSTIFFCompression, aFloat: f32)
                                                         -> id /* (NSData *) */ {
        msg_send![self, TIFFRepresentationUsingCompression:comp factor:aFloat]
    }

    unsafe fn cancelIncrementalLoad(self) {
        msg_send![self, cancelIncrementalLoad]
    }

    unsafe fn hitTestRect_withImageDestinationRect_context_hints_flipped_(self, testRectDestSpace: NSRect,
        imageRectDestSpace: NSRect, referenceContext: id /* (NSGraphicsContext *) */,
        hints: id /* (NSDictionary<NSString *, id> *) */, flipped: BOOL) -> BOOL {
        msg_send![self, hitTestRect:testRectDestSpace
           withImageDestinationRect:imageRectDestSpace
                            context:referenceContext
                              hints:hints
                            flipped:flipped]
    }

    unsafe fn accessibilityDescription(self) -> id /* (NSString *) */ {
        msg_send![self, accessibilityDescription]
    }

    unsafe fn layerContentsForContentsScale_(self, layerContentsScale: CGFloat) -> id /* (id) */ {
        msg_send![self, layerContentsForContentsScale:layerContentsScale]
    }

    unsafe fn recommendedLayerContentsScale_(self, preferredContentsScale: CGFloat) -> CGFloat {
        msg_send![self, recommendedLayerContentsScale:preferredContentsScale]
    }

    unsafe fn matchesOnlyOnBestFittingAxis(self) -> BOOL {
        msg_send![self, matchesOnlyOnBestFittingAxis]
    }
}

#[link(name = "AppKit", kind = "framework")]
extern {
    // Image hints (NSString* const)
    pub static NSImageHintCTM: id;
    pub static NSImageHintInterpolation: id;

    // System image names (NSString const*)
    pub static NSImageNameQuickLookTemplate: id;
    pub static NSImageNameBluetoothTemplate: id;
    pub static NSImageNameIChatTheaterTemplate: id;
    pub static NSImageNameSlideshowTemplate: id;
    pub static NSImageNameActionTemplate: id;
    pub static NSImageNameSmartBadgeTemplate: id;
    pub static NSImageNamePathTemplate: id;
    pub static NSImageNameInvalidDataFreestandingTemplate: id;
    pub static NSImageNameLockLockedTemplate: id;
    pub static NSImageNameLockUnlockedTemplate: id;
    pub static NSImageNameGoRightTemplate: id;
    pub static NSImageNameGoLeftTemplate: id;
    pub static NSImageNameRightFacingTriangleTemplate: id;
    pub static NSImageNameLeftFacingTriangleTemplate: id;
    pub static NSImageNameAddTemplate: id;
    pub static NSImageNameRemoveTemplate: id;
    pub static NSImageNameRevealFreestandingTemplate: id;
    pub static NSImageNameFollowLinkFreestandingTemplate: id;
    pub static NSImageNameEnterFullScreenTemplate: id;
    pub static NSImageNameExitFullScreenTemplate: id;
    pub static NSImageNameStopProgressTemplate: id;
    pub static NSImageNameStopProgressFreestandingTemplate: id;
    pub static NSImageNameRefreshTemplate: id;
    pub static NSImageNameRefreshFreestandingTemplate: id;

    pub static NSImageNameMultipleDocuments: id;

    pub static NSImageNameUser: id;
    pub static NSImageNameUserGroup: id;
    pub static NSImageNameEveryone: id;
    pub static NSImageNameUserGuest: id;

    pub static NSImageNameBonjour: id;
    pub static NSImageNameDotMac: id;
    pub static NSImageNameComputer: id;
    pub static NSImageNameFolderBurnable: id;
    pub static NSImageNameFolderSmart: id;
    pub static NSImageNameNetwork: id;

    pub static NSImageNameUserAccounts: id;
    pub static NSImageNamePreferencesGeneral: id;
    pub static NSImageNameAdvanced: id;
    pub static NSImageNameInfo: id;
    pub static NSImageNameFontPanel: id;
    pub static NSImageNameColorPanel: id;
    pub static NSImageNameFolder: id;
    pub static NSImageNameTrashEmpty: id;
    pub static NSImageNameTrashFull: id;
    pub static NSImageNameHomeTemplate: id;
    pub static NSImageNameBookmarksTemplate: id;
    pub static NSImageNameCaution: id;
    pub static NSImageNameStatusAvailable: id;
    pub static NSImageNameStatusPartiallyAvailable: id;
    pub static NSImageNameStatusUnavailable: id;
    pub static NSImageNameStatusNone: id;
    pub static NSImageNameApplicationIcon: id;
    pub static NSImageNameMenuOnStateTemplate: id;
    pub static NSImageNameMenuMixedStateTemplate: id;
    pub static NSImageNameMobileMe: id;

    pub static NSImageNameIconViewTemplate: id;
    pub static NSImageNameListViewTemplate: id;
    pub static NSImageNameColumnViewTemplate: id;
    pub static NSImageNameFlowViewTemplate: id;
    pub static NSImageNameShareTemplate: id;
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NSCompositingOperation {
   NSCompositeClear = 0,
   NSCompositeCopy = 1,
   NSCompositeSourceOver = 2,
   NSCompositeSourceIn = 3,
   NSCompositeSourceOut = 4,
   NSCompositeSourceAtop = 5,
   NSCompositeDestinationOver = 6,
   NSCompositeDestinationIn = 7,
   NSCompositeDestinationOut = 8,
   NSCompositeDestinationAtop = 9,
   NSCompositeXOR = 10,
   NSCompositePlusDarker = 11,
   NSCompositeHighlight = 12,
   NSCompositePlusLighter = 13
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NSImageCacheMode {
    NSImageCacheDefault,
    NSImageCacheAlways,
    NSImageCacheBySize,
    NSImageCacheNever
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NSTIFFCompression {
    NSTIFFCompressionNone = 1,
    NSTIFFCompressionCCITTFAX3 = 3,
    NSTIFFCompressionCCITTFAX4 = 4,
    NSTIFFCompressionLZW = 5,
    NSTIFFCompressionJPEG = 6,
    NSTIFFCompressionNEXT = 32766,
    NSTIFFCompressionPackBits = 32773,
    NSTIFFCompressionOldJPEG = 32865
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NSImageLoadStatus {
    NSImageLoadStatusCompleted,
    NSImageLoadStatusCancelled,
    NSImageLoadStatusInvalidData,
    NSImageLoadStatusUnexpectedEOF,
    NSImageLoadStatusReadError
}

pub trait NSSound: Sized {
    unsafe fn canInitWithPasteboard_(_: Self, pasteboard: id) -> BOOL {
        msg_send![class!(NSSound), canInitWithPasteboard:pasteboard]
    }

    unsafe fn initWithContentsOfFile_withReference_(self, filepath: id, byRef: BOOL) -> id;
    unsafe fn initWithContentsOfURL_withReference_(self, fileUrl: id, byRef: BOOL) -> id;
    unsafe fn initWithData_(self, audioData: id) -> id;
    unsafe fn initWithPasteboard_(self, pasteboard: id) -> id;

    unsafe fn name(self) -> id;
    unsafe fn volume(self) -> f32;
    unsafe fn currentTime(self) -> NSTimeInterval;
    unsafe fn loops(self) -> BOOL;
    unsafe fn playbackDeviceIdentifier(self) -> id;
    unsafe fn delegate(self) -> id;

    unsafe fn soundUnfilteredTypes(_: Self) -> id {
        msg_send![class!(NSSound), soundUnfilteredTypes]
    }

    unsafe fn soundNamed_(_: Self, soundName: id) -> id {
        msg_send![class!(NSSound), soundNamed:soundName]
    }

    unsafe fn duration(self) -> NSTimeInterval;

    unsafe fn playing(self) -> BOOL;
    unsafe fn pause(self) -> BOOL;
    unsafe fn play(self) -> BOOL;
    unsafe fn resume(self) -> BOOL;
    unsafe fn stop(self) -> BOOL;

    unsafe fn writeToPasteboard_(self, pasteboard: id);
}

impl NSSound for id {
    unsafe fn initWithContentsOfFile_withReference_(self, filepath: id, byRef: BOOL) -> id {
        msg_send![self, initWithContentsOfFile:filepath withReference:byRef]
    }

    unsafe fn initWithContentsOfURL_withReference_(self, fileUrl: id, byRef: BOOL) -> id {
        msg_send![self, initWithContentsOfURL:fileUrl withReference:byRef]
    }

    unsafe fn initWithData_(self, audioData: id) -> id {
        msg_send![self, initWithData:audioData]
    }

    unsafe fn initWithPasteboard_(self, pasteboard: id) -> id {
        msg_send![self, initWithPasteboard:pasteboard]
    }

    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn volume(self) -> f32 {
        msg_send![self, volume]
    }

    unsafe fn currentTime(self) -> NSTimeInterval {
        msg_send![self, currentTime]
    }

    unsafe fn loops(self) -> BOOL {
        msg_send![self, loops]
    }

    unsafe fn playbackDeviceIdentifier(self) -> id {
        msg_send![self, playbackDeviceIdentifier]
    }

    unsafe fn delegate(self) -> id {
        msg_send![self, delegate]
    }

    unsafe fn duration(self) -> NSTimeInterval {
        msg_send![self, duration]
    }

    unsafe fn playing(self) -> BOOL {
        msg_send![self, playing]
    }

    unsafe fn pause(self) -> BOOL {
        msg_send![self, pause]
    }

    unsafe fn play(self) -> BOOL {
        msg_send![self, play]
    }

    unsafe fn resume(self) -> BOOL {
        msg_send![self, resume]
    }

    unsafe fn stop(self) -> BOOL {
        msg_send![self, stop]
    }

    unsafe fn writeToPasteboard_(self, pasteboard: id) {
        msg_send![self, writeToPasteboard:pasteboard]
    }
}

pub const NSVariableStatusItemLength: CGFloat = -1.0;
pub const NSSquareStatusItemLength: CGFloat = -2.0;

pub trait NSStatusItem: Sized {
    unsafe fn statusBar(self) -> id /* (NSStatusBar *) */;
    unsafe fn button(self) -> id /* (NSStatusBarButton *) */;
    unsafe fn menu(self) -> id;
    unsafe fn setMenu_(self, menu: id);
    unsafe fn length(self) -> CGFloat;
    unsafe fn setLength_(self, length: CGFloat);
}

impl NSStatusItem for id {
    unsafe fn statusBar(self) -> id /* (NSStatusBar *) */ {
        msg_send![self, statusBar]
    }

    unsafe fn button(self) -> id /* (NSStatusBarButton *) */ {
        msg_send![self, button]
    }

    unsafe fn menu(self) -> id {
        msg_send![self, menu]
    }

    unsafe fn setMenu_(self, menu: id) {
        msg_send![self, setMenu:menu]
    }

    unsafe fn length(self) -> CGFloat {
        msg_send![self, length]
    }

    unsafe fn setLength_(self, length: CGFloat) {
        msg_send![self, setLength: length]
    }
}

pub trait NSStatusBar: Sized {
    unsafe fn systemStatusBar(_: Self) -> id {
        msg_send![class!(NSStatusBar), systemStatusBar]
    }

    unsafe fn statusItemWithLength_(self, length: CGFloat) -> id /* (NSStatusItem *) */;
    unsafe fn removeStatusItem_(self, item: id /* (NSStatusItem *) */);
    unsafe fn isVertical(self) -> BOOL;
}

impl NSStatusBar for id {
    unsafe fn statusItemWithLength_(self, length: CGFloat) -> id /* (NSStatusItem *) */ {
        msg_send![self, statusItemWithLength:length]
    }

    unsafe fn removeStatusItem_(self, item: id /* (NSStatusItem *) */) {
        msg_send![self, removeStatusItem:item]
    }

    unsafe fn isVertical(self) -> BOOL {
        msg_send![self, isVertical]
    }
}

extern {
    pub fn NSRectFill(rect: NSRect);
}

pub trait NSTextField: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSTextField), alloc]
    }
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
    unsafe fn setEditable_(self, editable: BOOL);
    unsafe fn setStringValue_(self, label: id /* NSString */);
}

impl NSTextField for id {
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send![self, initWithFrame:frameRect]
    }
    unsafe fn setEditable_(self, editable: BOOL) {
        msg_send![self, setEditable:editable]
    }
    unsafe fn setStringValue_(self, label: id) {
        msg_send![self, setStringValue:label]
    }
}

#[repr(u64)]
pub enum NSTabViewType {
    NSTopTabsBezelBorder     = 0,
    NSLeftTabsBezelBorder    = 1,
    NSBottomTabsBezelBorder  = 2,
    NSRightTabsBezelBorder   = 3,
    NSNoTabsBezelBorder      = 4,
    NSNoTabsLineBorder       = 5,
    NSNoTabsNoBorder         = 6
}

pub trait NSTabView: Sized {
    unsafe fn new(_: Self) -> id  {
        msg_send![class!(NSTabView), new]
    }

    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id;
    unsafe fn addTabViewItem_(self, tabViewItem: id);
    unsafe fn insertTabViewItem_atIndex_(self,tabViewItem:id, index:NSInteger);
    unsafe fn removeTabViewItem_(self,tabViewItem:id);
    unsafe fn indexOfTabViewItem_(self, tabViewItem:id) -> id;
    unsafe fn indexOfTabViewItemWithIdentifier_(self,identifier:id) -> id;
    unsafe fn numberOfTabViewItems(self) -> id;
    unsafe fn tabViewItemAtIndex_(self,index:id) -> id;
    unsafe fn tabViewItems(self) -> id;
    unsafe fn selectFirstTabViewItem_(self,sender:id);
    unsafe fn selectLastTabViewItem_(self,sender:id);
    unsafe fn selectNextTabViewItem_(self, sender:id);
    unsafe fn selectPreviousTabViewItem_(self,sender:id);
    unsafe fn selectTabViewItem_(self,tabViewItem:id);
    unsafe fn selectTabViewItemAtIndex_(self,index:id);
    unsafe fn selectTabViewItemWithIdentifier_(self,identifier:id);
    unsafe fn selectedTabViewItem(self) -> id;
    unsafe fn takeSelectedTabViewItemFromSender_(self,sender:id);
    unsafe fn font(self) -> id;
    unsafe fn setFont_(self, font:id);
    unsafe fn tabViewType(self) -> NSTabViewType;
    unsafe fn setTabViewType_(self,tabViewType: NSTabViewType);
    unsafe fn controlTint(self) -> id;
    unsafe fn setControlTint_(self,controlTint:id);
    unsafe fn drawsBackground(self) -> BOOL;
    unsafe fn setDrawsBackground_(self,drawsBackground:BOOL);
    unsafe fn minimumSize(self) -> id;
    unsafe fn contentRect(self) -> id;
    unsafe fn controlSize(self) -> id;
    unsafe fn setControlSize_(self,controlSize:id);
    unsafe fn allowsTruncatedLabels(self) -> BOOL;
    unsafe fn setAllowsTruncatedLabels_(self, allowTruncatedLabels:BOOL);
    unsafe fn setDelegate_(self, delegate:id);
    unsafe fn delegate(self) -> id;
    unsafe fn tabViewAtPoint_(self, point:id) -> id;
}

impl NSTabView for id {
    unsafe fn initWithFrame_(self, frameRect: NSRect) -> id {
        msg_send![self, initWithFrame:frameRect]
    }

    unsafe fn addTabViewItem_(self, tabViewItem: id) {
        msg_send![self, addTabViewItem:tabViewItem]
    }
    unsafe fn insertTabViewItem_atIndex_(self, tabViewItem: id,index:NSInteger) {
        msg_send![self, addTabViewItem:tabViewItem atIndex:index]
    }
    unsafe fn removeTabViewItem_(self,tabViewItem:id){
        msg_send![self, removeTabViewItem:tabViewItem]
    }

    unsafe fn indexOfTabViewItem_(self, tabViewItem:id) -> id{
        msg_send![self, indexOfTabViewItem:tabViewItem]
    }

    unsafe fn indexOfTabViewItemWithIdentifier_(self,identifier:id) -> id{
        msg_send![self, indexOfTabViewItemWithIdentifier:identifier]
    }
    unsafe fn numberOfTabViewItems(self) -> id{
        msg_send![self, numberOfTabViewItems]
    }

    unsafe fn tabViewItemAtIndex_(self,index:id)->id{
        msg_send![self, tabViewItemAtIndex:index]
    }

    unsafe fn tabViewItems(self)->id{
        msg_send![self, tabViewItems]
    }

    unsafe fn selectFirstTabViewItem_(self,sender:id){
        msg_send![self, selectFirstTabViewItem:sender]
    }

    unsafe fn selectLastTabViewItem_(self,sender:id){
        msg_send![self, selectLastTabViewItem:sender]
    }
    unsafe fn selectNextTabViewItem_(self, sender:id){
        msg_send![self, selectNextTabViewItem:sender]
    }
    unsafe fn selectPreviousTabViewItem_(self,sender:id){
        msg_send![self, selectPreviousTabViewItem:sender]
    }

    unsafe fn selectTabViewItem_(self,tabViewItem:id){
        msg_send![self, selectTabViewItem:tabViewItem]
    }

    unsafe fn selectTabViewItemAtIndex_(self,index:id){
        msg_send![self, selectTabViewItemAtIndex:index]
    }
    unsafe fn selectTabViewItemWithIdentifier_(self,identifier:id){
        msg_send![self, selectTabViewItemWithIdentifier:identifier]
    }
    unsafe fn selectedTabViewItem(self) -> id{
        msg_send![self, selectedTabViewItem]
    }
    unsafe fn takeSelectedTabViewItemFromSender_(self,sender:id){
        msg_send![self, takeSelectedTabViewItemFromSender:sender]
    }

    unsafe fn font(self)->id{
        msg_send![self, font]
    }

    unsafe fn setFont_(self, font:id){
        msg_send![self, setFont:font]
    }

    unsafe fn tabViewType(self)->NSTabViewType{
        msg_send![self, tabViewType]
    }
    unsafe fn setTabViewType_(self,tabViewType: NSTabViewType){
        msg_send![self, setTabViewType:tabViewType]
    }

    unsafe fn controlTint(self) -> id{
        msg_send![self, controlTint]
    }
    unsafe fn setControlTint_(self,controlTint:id){
        msg_send![self, setControlTint:controlTint]
    }

    unsafe fn drawsBackground(self) -> BOOL{
        msg_send![self, drawsBackground]
    }
    unsafe fn setDrawsBackground_(self,drawsBackground:BOOL){
        msg_send![self, setDrawsBackground:drawsBackground as libc::c_int]
    }

    unsafe fn minimumSize(self) -> id{
        msg_send![self, minimumSize]
    }
    unsafe fn contentRect(self) -> id{
        msg_send![self, contentRect]
    }
    unsafe fn controlSize(self) -> id{
        msg_send![self, controlSize]
    }
    unsafe fn setControlSize_(self,controlSize:id){
        msg_send![self, setControlSize:controlSize]
    }

    unsafe fn allowsTruncatedLabels(self) -> BOOL{
        msg_send![self, allowsTruncatedLabels]
    }
    unsafe fn setAllowsTruncatedLabels_(self, allowTruncatedLabels:BOOL){
        msg_send![self, setAllowsTruncatedLabels:allowTruncatedLabels as libc::c_int]
    }

    unsafe fn setDelegate_(self, delegate:id){
        msg_send![self, setDelegate:delegate]
    }
    unsafe fn delegate(self) -> id {
        msg_send![self, delegate]
    }

    unsafe fn tabViewAtPoint_(self, point:id) -> id{
        msg_send![self, tabViewAtPoint:point]
    }
}

#[repr(u64)]
pub enum NSTabState {
    NSSelectedTab = 0,
    NSBackgroundTab = 1,
    NSPressedTab = 2
}

pub trait NSTabViewItem: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSTabViewItem), alloc]
    }
    unsafe fn new(_: Self) -> id {
        msg_send![class!(NSTabViewItem), new]
    }

    unsafe fn initWithIdentifier_(self, identifier:id) -> id;
    unsafe fn drawLabel_inRect_(self,shouldTruncateLabel:BOOL,labelRect:NSRect);
    unsafe fn label(self) -> id;
    unsafe fn setLabel_(self,label:id);
    unsafe fn sizeOfLabel_(self, computeMin:BOOL);
    unsafe fn tabState(self) -> NSTabState;
    unsafe fn identifier(self)-> id;
    unsafe fn setIdentifier_(self,identifier:id);
    unsafe fn color(self)-> id;
    unsafe fn setColor_(self,color:id);
    unsafe fn view(self) -> id;
    unsafe fn setView_(self, view:id);
    unsafe fn initialFirstResponder(self)->id;
    unsafe fn setInitialFirstResponder_(self,initialFirstResponder:id);
    unsafe fn tabView(self) -> id;
    unsafe fn tooltip(self) -> id;
    unsafe fn setToolTip_(self,toolTip:id);
}

impl NSTabViewItem for id {
    unsafe fn initWithIdentifier_(self, identifier: id) -> id {
        msg_send![self, initWithIdentifier:identifier]
    }

    unsafe fn drawLabel_inRect_(self, shouldTruncateLabel:BOOL,labelRect:NSRect){
        msg_send![self, drawLabel:shouldTruncateLabel as libc::c_int inRect:labelRect]
    }

    unsafe fn label(self)->id{
        msg_send![self, label]
    }
    unsafe fn setLabel_(self,label : id){
        msg_send![self, setLabel:label]
    }

    unsafe fn sizeOfLabel_(self,computeMin:BOOL){
        msg_send![self, sizeOfLabel:computeMin as libc::c_int]
    }

    unsafe fn tabState(self) -> NSTabState{
        msg_send![self, tabState]
    }

    unsafe fn identifier(self)-> id {
        msg_send![self, identifier]
    }

    unsafe fn setIdentifier_(self,identifier:id){
        msg_send![self, identifier:identifier]
    }

    unsafe fn color(self)-> id{
        msg_send![self, color]
    }

    unsafe fn setColor_(self,color:id){
        msg_send![self, color:color]
    }

    unsafe fn view(self) -> id {
        msg_send![self, view]
    }

    unsafe fn setView_(self, view:id){
        msg_send![self, setView:view]
    }

    unsafe fn initialFirstResponder(self)->id{
        msg_send![self, initialFirstResponder]
    }

    unsafe fn setInitialFirstResponder_(self,initialFirstResponder:id){
        msg_send![self, setInitialFirstResponder:initialFirstResponder]
    }

    unsafe fn tabView(self) -> id{
        msg_send![self, tabView]
    }

    unsafe fn tooltip(self) -> id{
        msg_send![self, tooltip]
    }

    unsafe fn setToolTip_(self,toolTip:id){
        msg_send![self, setToolTip:toolTip]
    }
}

pub trait NSLayoutConstraint: Sized {
    unsafe fn activateConstraints(_: Self, constraints: id) -> id;
}

impl NSLayoutConstraint for id {
    unsafe fn activateConstraints(_: Self, constraints: id) -> id {
        msg_send![class!(NSLayoutConstraint), activateConstraints:constraints]
    }
}

pub trait NSLayoutDimension: Sized {
    unsafe fn constraintEqualToConstant(self, c: CGFloat) -> id;
    unsafe fn constraintLessThanOrEqualToConstant(self, c: CGFloat) -> id;
    unsafe fn constraintGreaterThanOrEqualToConstant(self, c: CGFloat) -> id;
}

impl NSLayoutDimension for id {
    unsafe fn constraintEqualToConstant(self, c: CGFloat) -> id {
        msg_send![self, constraintEqualToConstant:c]
    }

    unsafe fn constraintLessThanOrEqualToConstant(self, c: CGFloat) -> id {
        msg_send![self, constraintLessThanOrEqualToConstant:c]
    }

    unsafe fn constraintGreaterThanOrEqualToConstant(self, c: CGFloat) -> id {
        msg_send![self, constraintGreaterThanOrEqualToConstant:c]
    }
}

pub trait NSColorSpace: Sized {
    unsafe fn deviceRGBColorSpace(_:Self) -> id;
    unsafe fn genericRGBColorSpace(_:Self) -> id;
    unsafe fn deviceCMYKColorSpace(_:Self) -> id;
    unsafe fn genericCMYKColorSpace(_:Self) -> id;
    unsafe fn deviceGrayColorSpace(_:Self) -> id;
    unsafe fn genericGrayColorSpace(_:Self) -> id;
    unsafe fn sRGBColorSpace(_:Self) -> id;
    unsafe fn extendedSRGBColorSpace(_:Self) -> id;
    unsafe fn displayP3ColorSpace(_:Self) -> id;
    unsafe fn genericGamma22GrayColorSpace(_:Self) -> id;
    unsafe fn extendedGenericGamma22GrayColorSpace(_:Self) -> id;
    unsafe fn adobeRGB1998ColorSpace(_:Self) -> id;

    unsafe fn alloc(_: Self) -> id;

    unsafe fn initWithCGColorSpace_(self, cg_color_space: *const c_void /* (CGColorSpaceRef) */) -> id;
    unsafe fn CGColorSpace(self) -> *const c_void /* (CGColorSpaceRef) */;
    unsafe fn localizedName(self) -> id;
}

impl NSColorSpace for id {
    unsafe fn deviceRGBColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), deviceRGBColorSpace]
    }
    unsafe fn genericRGBColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), genericRGBColorSpace]
    }
    unsafe fn deviceCMYKColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), deviceCMYKColorSpace]
    }
    unsafe fn genericCMYKColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), genericCMYKColorSpace]
    }
    unsafe fn deviceGrayColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), deviceGrayColorSpace]
    }
    unsafe fn genericGrayColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), genericGrayColorSpace]
    }
    unsafe fn sRGBColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), sRGBColorSpace]
    }
    unsafe fn extendedSRGBColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), extendedSRGBColorSpace]
    }
    unsafe fn displayP3ColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), displayP3ColorSpace]
    }
    unsafe fn genericGamma22GrayColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), genericGamma22GrayColorSpace]
    }
    unsafe fn extendedGenericGamma22GrayColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), extendedGenericGamma22GrayColorSpace]
    }
    unsafe fn adobeRGB1998ColorSpace(_:Self) -> id {
        msg_send![class!(NSColorSpace), adobeRGB1998ColorSpace]
    }

    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSColorSpace), alloc]
    }

    unsafe fn initWithCGColorSpace_(self, cg_color_space: *const c_void /* (CGColorSpaceRef) */) -> id {
        msg_send![self, initWithCGColorSpace:cg_color_space]
    }
    unsafe fn CGColorSpace(self) -> *const c_void /* (CGColorSpaceRef) */ {
        msg_send![self, CGColorSpace]
    }
    unsafe fn localizedName(self) -> id {
        msg_send![self, localizedName]
    }
}

pub trait NSColor: Sized {
    unsafe fn clearColor(_: Self) -> id;
    unsafe fn colorWithRed_green_blue_alpha_(_:Self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> id;
    unsafe fn colorWithSRGBRed_green_blue_alpha_(_:Self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> id;
    unsafe fn colorWithDeviceRed_green_blue_alpha_(_:Self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> id;
    unsafe fn colorWithDisplayP3Red_green_blue_alpha_(_:Self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> id;
    unsafe fn colorWithCalibratedRed_green_blue_alpha_(_:Self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> id;

    unsafe fn colorUsingColorSpace_(self, color_space: id) -> id;

    unsafe fn alphaComponent(self) -> CGFloat;
    unsafe fn whiteComponent(self) -> CGFloat;
    unsafe fn redComponent(self) -> CGFloat;
    unsafe fn greenComponent(self) -> CGFloat;
    unsafe fn blueComponent(self) -> CGFloat;
    unsafe fn cyanComponent(self) -> CGFloat;
    unsafe fn magentaComponent(self) -> CGFloat;
    unsafe fn yellowComponent(self) -> CGFloat;
    unsafe fn blackComponent(self) -> CGFloat;
    unsafe fn hueComponent(self) -> CGFloat;
    unsafe fn saturationComponent(self) -> CGFloat;
    unsafe fn brightnessComponent(self) -> CGFloat;
}

impl NSColor for id {
    unsafe fn clearColor(_: Self) -> id {
        msg_send![class!(NSColor), clearColor]
    }
    unsafe fn colorWithRed_green_blue_alpha_(_:Self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> id {
        msg_send![class!(NSColor), colorWithRed:r green:g blue:b alpha:a]
    }
    unsafe fn colorWithSRGBRed_green_blue_alpha_(_:Self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> id {
        msg_send![class!(NSColor), colorWithSRGBRed:r green:g blue:b alpha:a]
    }
    unsafe fn colorWithDeviceRed_green_blue_alpha_(_:Self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> id {
        msg_send![class!(NSColor), colorWithDeviceRed:r green:g blue:b alpha:a]
    }
    unsafe fn colorWithDisplayP3Red_green_blue_alpha_(_:Self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> id {
        msg_send![class!(NSColor), colorWithDisplayP3Red:r green:g blue:b alpha:a]
    }
    unsafe fn colorWithCalibratedRed_green_blue_alpha_(_:Self, r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> id {
        msg_send![class!(NSColor), colorWithCalibratedRed:r green:g blue:b alpha:a]
    }

    unsafe fn colorUsingColorSpace_(self, color_space: id) -> id {
        msg_send![self, colorUsingColorSpace:color_space]
    }

    unsafe fn alphaComponent(self) -> CGFloat {
        msg_send![self, alphaComponent]
    }
    unsafe fn whiteComponent(self) -> CGFloat {
        msg_send![self, whiteComponent]
    }
    unsafe fn redComponent(self) -> CGFloat {
        msg_send![self, redComponent]
    }
    unsafe fn greenComponent(self) -> CGFloat {
        msg_send![self, greenComponent]
    }
    unsafe fn blueComponent(self) -> CGFloat {
        msg_send![self, blueComponent]
    }
    unsafe fn cyanComponent(self) -> CGFloat {
        msg_send![self, cyanComponent]
    }
    unsafe fn magentaComponent(self) -> CGFloat {
        msg_send![self, magentaComponent]
    }
    unsafe fn yellowComponent(self) -> CGFloat {
        msg_send![self, yellowComponent]
    }
    unsafe fn blackComponent(self) -> CGFloat {
        msg_send![self, blackComponent]
    }
    unsafe fn hueComponent(self) -> CGFloat {
        msg_send![self, hueComponent]
    }
    unsafe fn saturationComponent(self) -> CGFloat {
        msg_send![self, saturationComponent]
    }
    unsafe fn brightnessComponent(self) -> CGFloat {
        msg_send![self, brightnessComponent]
    }
}

pub trait NSToolbar: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSToolbar), alloc]
    }

    unsafe fn init_(self) -> id /* NSToolbar */;
    unsafe fn initWithIdentifier_(self, identifier: id) -> id /* NSToolbar */;

    unsafe fn showsBaselineSeparator(self) -> BOOL;
    unsafe fn setShowsBaselineSeparator_(self, value: BOOL);
}

impl NSToolbar for id {
    unsafe fn init_(self) -> id /* NSToolbar */ {
        msg_send![self, init]
    }

    unsafe fn initWithIdentifier_(self, identifier: id) -> id /* NSToolbar */ {
        msg_send![self, initWithIdentifier:identifier]
    }

    unsafe fn showsBaselineSeparator(self) -> BOOL {
        msg_send![self, showsBaselineSeparator]
    }

    unsafe fn setShowsBaselineSeparator_(self, value: BOOL) {
        msg_send![self, setShowsBaselineSeparator:value]
    }
}

pub trait NSSpellChecker : Sized {
    unsafe fn sharedSpellChecker(_: Self) -> id;
    unsafe fn checkSpellingOfString_startingAt(self,
                                               stringToCheck: id,
                                               startingOffset: NSInteger) -> NSRange;
    unsafe fn checkSpellingOfString_startingAt_language_wrap_inSpellDocumentWithTag_wordCount(
        self,
        stringToCheck: id,
        startingOffset: NSInteger,
        language: id,
        wrapFlag: BOOL,
        tag: NSInteger) -> (NSRange, NSInteger);
    unsafe fn uniqueSpellDocumentTag(_: Self) -> NSInteger;
    unsafe fn closeSpellDocumentWithTag(self, tag: NSInteger);
    unsafe fn ignoreWord_inSpellDocumentWithTag(self, wordToIgnore: id, tag: NSInteger);
}

impl NSSpellChecker for id {
    unsafe fn sharedSpellChecker(_: Self) -> id {
        msg_send![class!(NSSpellChecker), sharedSpellChecker]
    }

    unsafe fn checkSpellingOfString_startingAt(self,
                                               stringToCheck: id,
                                               startingOffset: NSInteger) -> NSRange {
        msg_send![self, checkSpellingOfString:stringToCheck startingAt:startingOffset]
    }

    unsafe fn checkSpellingOfString_startingAt_language_wrap_inSpellDocumentWithTag_wordCount(
        self,
        stringToCheck: id,
        startingOffset: NSInteger,
        language: id,
        wrapFlag: BOOL,
        tag: NSInteger) -> (NSRange, NSInteger) {
        let mut wordCount = 0;
        let range = msg_send![self,
            checkSpellingOfString:stringToCheck
            startingAt:startingOffset
            language:language
            wrap:wrapFlag
            inSpellDocumentWithTag:tag
            wordCount:&mut wordCount
        ];
        (range, wordCount)
    }

    unsafe fn uniqueSpellDocumentTag(_: Self) -> NSInteger {
        msg_send![class!(NSSpellChecker), uniqueSpellDocumentTag]
    }

    unsafe fn closeSpellDocumentWithTag(self, tag: NSInteger) {
        msg_send![self, closeSpellDocumentWithTag:tag]
    }

    unsafe fn ignoreWord_inSpellDocumentWithTag(self, wordToIgnore: id, tag: NSInteger) {
        msg_send![self, ignoreWord:wordToIgnore inSpellDocumentWithTag:tag]
    }
}

pub trait NSNib: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSNib), alloc]
    }

    unsafe fn initWithNibNamed_bundle_(self, name: id, bundle: id) -> id;
}

impl NSNib for id {
    unsafe fn initWithNibNamed_bundle_(self, name: id, bundle: id) -> id {
        msg_send![self, initWithNibNamed:name bundle:bundle]
    }
}

pub trait NSCursor: Sized {
    unsafe fn alloc(_: Self) -> id;
    unsafe fn current_cursor(_: Self) -> id;
    unsafe fn current_system_cursor(_: Self) -> id;
    unsafe fn arrow_cursor(_: Self) -> id;
    unsafe fn contextual_menu_cursor(_: Self) -> id;
    unsafe fn closed_hand_cursor(_: Self) -> id;
    unsafe fn crosshair_cursor(_: Self) -> id;
    unsafe fn disappearing_item_cursor(_: Self) -> id;
    unsafe fn drag_copy_cursor(_: Self) -> id;
    unsafe fn drag_link_cursor(_: Self) -> id;
    unsafe fn i_beam_cursor(_: Self) -> id;
    unsafe fn open_hand_cursor(_: Self) -> id;
    unsafe fn operation_not_allowed_cursor(_: Self) -> id;
    unsafe fn pointing_hand_cursor(_: Self) -> id;
    unsafe fn resize_down_cursor(_: Self) -> id;
    unsafe fn resize_left_cursor(_: Self) -> id;
    unsafe fn resize_left_right_cursor(_: Self) -> id;
    unsafe fn resize_right_cursor(_: Self) -> id;
    unsafe fn resize_up_cursor(_: Self) -> id;
    unsafe fn resize_up_down_cursor(_: Self) -> id;
    unsafe fn i_beam_cursor_for_vertical_layout(_: Self) -> id;
}

impl NSCursor for id {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSCursor), alloc]
    }

    unsafe fn current_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), currentCursor]
    }

    unsafe fn current_system_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), currentSystemCursor]
    }

    unsafe fn arrow_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), arrowCursor]
    }

    unsafe fn contextual_menu_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), contextualMenuCursor]
    }

    unsafe fn closed_hand_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), closedHandCursor]
    }

    unsafe fn crosshair_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), crosshairCursor]
    }

    unsafe fn disappearing_item_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), disappearingItemCursor]
    }

    unsafe fn drag_copy_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), dragCopyCursor]
    }

    unsafe fn drag_link_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), dragLinkCursor]
    }

    unsafe fn i_beam_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), IBeamCursor]
    }

    unsafe fn open_hand_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), openHandCursor]
    }

    unsafe fn operation_not_allowed_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), operationNotAllowedCursor]
    }

    unsafe fn pointing_hand_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), pointingHandCursor]
    }

    unsafe fn resize_down_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), resizeDownCursor]
    }

    unsafe fn resize_left_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), resizeLeftCursor]
    }

    unsafe fn resize_left_right_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), resizeLeftRightCursor]
    }

    unsafe fn resize_right_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), resizeRightCursor]
    }

    unsafe fn resize_up_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), resizeUpCursor]
    }

    unsafe fn resize_up_down_cursor(_: Self) -> id {
        msg_send![class!(NSCursor), resizeUpDownCursor]
    }

    unsafe fn i_beam_cursor_for_vertical_layout(_: Self) -> id {
        msg_send![class!(NSCursor), IBeamCursorForVerticalLayout]
    }
}

pub trait NSDockTile: Sized {
    unsafe fn showsApplicatinBadge(self) -> BOOL;
    unsafe fn setShowsApplicatinBadge_(self, value: BOOL) -> id;
    unsafe fn badgeLabel(self) -> id /* NSString */;
    unsafe fn setBadgeLabel_(self, label: id /* NSString */) -> id;
}

impl NSDockTile for id {
    unsafe fn showsApplicatinBadge(self) -> BOOL {
        msg_send![self, showsApplicationBadge]
    }

    unsafe fn setShowsApplicatinBadge_(self, value: BOOL) -> id {
        msg_send![self, setShowsApplicationBadge:value]
    }

    unsafe fn badgeLabel(self) -> id /* NSString */ {
        msg_send![self, badgeLabel]
    }

    unsafe fn setBadgeLabel_(self, label: id /* NSString */) -> id {
        msg_send![self, setBadgeLabel:label]
    }
}

pub unsafe fn NSAppearance(named: id /* NSAppearanceName */) -> id {
    objc::msg_send![class!(NSAppearance), appearanceNamed: named]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_nsapp() {
        unsafe {
            let _nsApp = NSApp();
        }
    }
}
