// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_int, c_long, c_uint, c_ulong};

use crate::sync::XSyncFence;
use crate::xfixes::XserverRegion;
use crate::xlib::{Bool, Display, Pixmap, Status, Window, XID};
use crate::xrandr::RRCrtc;

//
// functions
//

x11_link! { Xpresent, xpresent, ["libXpresent.so.1.0.0", "libXpresent.so"], 8,
    pub fn XPresentQueryExtension( dpy: *mut Display, major_opcode_return: *mut c_int, event_base_return: *mut c_int, error_base_return: *mut c_int) -> Bool,
    pub fn XPresentQueryVersion( dpy: *mut Display, major_version_return: *mut c_int, minor_version_return: *mut c_int ) -> Status,
    pub fn XPresentVersion() -> c_int,
    pub fn XPresentPixmap( dpy: *mut Display, window: Window, pixmap: Pixmap, serial: u32, valid: XserverRegion, update: XserverRegion, x_off: c_int, y_off: c_int, target_crtc: RRCrtc, wait_fence: XSyncFence, idle_fence: XSyncFence, options: u32, target_msc: u64, divisor: u64, remainder: u64, notifies: *mut XPresentNotify, nnotifies: c_int ) -> (),
    pub fn XPresentNotifyMSC( dpy: *mut Display, window: Window, serial: u32, target_msc: u64, divisor: u64, remainder: u64 ) -> (),
    pub fn XPresentSelectInput( dpy: *mut Display, window: Window, event_mask: c_uint ) -> XID,
    pub fn XPresentFreeInput(dpy: *mut Display, window: Window, event_id: XID) -> (),
    pub fn XPresentQueryCapabilities(dpy: *mut Display, target: XID) -> u32,
variadic:
globals:
}

//
// Types
//

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPresentNotify {
    pub window: Window,
    pub serial: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPresentEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub extension: c_int,
    pub evtype: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPresentIdleNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: c_int,
    pub display: *mut Display,
    pub extension: c_int,
    pub evtype: c_int,
    pub eid: u32,
    pub window: Window,
    pub serial_number: u32,
    pub pixmap: Pixmap,
    pub idle_fence: XSyncFence,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPresentCompleteNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub extension: c_int,
    pub evtype: c_int,
    pub eid: u32,
    pub window: Window,
    pub serial_number: u32,
    pub ust: u64,
    pub msc: u64,
    pub kind: u8,
    pub mode: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct XPresentConfigureNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub extension: c_int,
    pub evtype: c_int,
    pub eid: u32,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_uint,
    pub height: c_uint,
    pub off_x: c_int,
    pub off_y: c_int,
    pub pixmap_width: c_int,
    pub pixmap_height: c_int,
    pub pixmap_flags: c_long,
}

//
// constants
//

pub const PRESENT_NAME: &str = "Present";
pub const PRESENT_MAJOR: c_int = 1;
pub const PRESENT_MINOR: c_int = 2;

pub const PRESENT_REVISION: c_int = 0;
pub const PRESENT_VERSION: c_int = PRESENT_MAJOR * 10000 + PRESENT_MINOR * 100 + PRESENT_REVISION;

pub const PresentNumberErrors: c_int = 0;
pub const PresentNumberEvents: c_int = 0;

pub const X_PresentQueryVersion: c_int = 0;
pub const X_PresentPixmap: c_int = 1;
pub const X_PresentNotifyMSC: c_int = 2;
pub const X_PresentSelectInput: c_int = 3;
pub const X_PresentQueryCapabilities: c_int = 4;

pub const PresentNumberRequests: c_int = 5;

pub const PresentOptionNone: c_int = 0;
pub const PresentOptionAsync: c_int = 1;
pub const PresentOptionCopy: c_int = 2;
pub const PresentOptionUST: c_int = 4;
pub const PresentOptionSuboptimal: c_int = 8;

pub const PresentAllOptions: c_int = PresentOptionNone
    | PresentOptionAsync
    | PresentOptionCopy
    | PresentOptionUST
    | PresentOptionSuboptimal;

pub const PresentCapabilityNone: c_int = 0;
pub const PresentCapabilityAsync: c_int = 1;
pub const PresentCapabilityFence: c_int = 2;
pub const PresentCapabilityUST: c_int = 4;

pub const PresentAllCapabilities: c_int =
    PresentCapabilityAsync | PresentCapabilityFence | PresentCapabilityUST;

pub const PresentConfigureNotify: c_int = 0;
pub const PresentCompleteNotify: c_int = 1;
pub const PresentIdleNotify: c_int = 2;

pub const PresentConfigureNotifyMask: c_int = 1;
pub const PresentCompleteNotifyMask: c_int = 2;
pub const PresentIdleNotifyMask: c_int = 4;

pub const PresentAllEvents: c_int =
    PresentConfigureNotifyMask | PresentCompleteNotify | PresentIdleNotifyMask;

pub const PresentCompleteKindPixmap: c_int = 0;
pub const PresentCompleteKindNotifyMSC: c_int = 1;

pub const PresentCompleteModeCopy: c_int = 0;
pub const PresentCompleteModeFlip: c_int = 1;
pub const PresentCompleteModeSkip: c_int = 2;
pub const PresentCompleteModeSuboptimalCopy: c_int = 3;
