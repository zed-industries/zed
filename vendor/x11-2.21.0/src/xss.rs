// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use super::xlib::{
    Atom, Bool, Display, Drawable, Status, Time, Visual, Window, XEvent, XSetWindowAttributes, XID,
};
use std::os::raw::{c_int, c_uint, c_ulong};

//
// functions
//

x11_link! { Xss, xscrnsaver, ["libXss.so.2", "libXss.so"], 11,
  pub fn XScreenSaverQueryExtension (_1: *mut Display, _2: *mut c_int, _3: *mut c_int) -> Bool,
  pub fn XScreenSaverQueryVersion (_1: *mut Display, _2: *mut c_int, _3: *mut c_int) -> Status,
  pub fn XScreenSaverAllocInfo () -> *mut XScreenSaverInfo,
  pub fn XScreenSaverQueryInfo (_1: *mut Display, _2: Drawable, _3: *mut XScreenSaverInfo) -> Status,
  pub fn XScreenSaverSelectInput (_1: *mut Display, _2: Drawable, _3: c_ulong) -> (),
  pub fn XScreenSaverSetAttributes (_1: *mut Display, _2: Drawable, _3: c_int, _4: c_int, _5: c_uint, _6: c_uint, _7: c_uint, _8: c_int, _9: c_uint, _10: *mut Visual, _11: c_ulong, _12: *mut XSetWindowAttributes) -> (),
  pub fn XScreenSaverUnsetAttributes (_1: *mut Display, _2: Drawable) -> (),
  pub fn XScreenSaverRegister (_1: *mut Display, _2: c_int, _3: XID, _4: Atom) -> Status,
  pub fn XScreenSaverUnregister (_1: *mut Display, _2: c_int) -> Status,
  pub fn XScreenSaverGetRegistered (_1: *mut Display, _2: c_int, _3: *mut XID, _4: *mut Atom) -> Status,
  pub fn XScreenSaverSuspend (_1: *mut Display, _2: Bool) -> (),
variadic:
globals:
}

//
// types
//

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XScreenSaverInfo {
    pub window: Window,
    pub state: c_int,
    pub kind: c_int,
    pub til_or_since: c_ulong,
    pub idle: c_ulong,
    pub eventMask: c_ulong,
}

//
// event structures
//

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XScreenSaverNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub state: c_int,
    pub kind: c_int,
    pub forced: Bool,
    pub time: Time,
}

event_conversions_and_tests! {
  xss_notify: XScreenSaverNotifyEvent,
}

//
// constants
//

pub const ScreenSaverName: &str = "MIT-SCREEN-SAVER";
pub const ScreenSaverPropertyName: &str = "_MIT_SCREEN_SAVER_ID";

pub const ScreenSaverNotifyMask: c_ulong = 0x00000001;
pub const ScreenSaverCycleMask: c_ulong = 0x00000002;

pub const ScreenSaverMajorVersion: c_int = 1;
pub const ScreenSaverMinorVersion: c_int = 1;

pub const ScreenSaverOff: c_int = 0;
pub const ScreenSaverOn: c_int = 1;
pub const ScreenSaverCycle: c_int = 2;
pub const ScreenSaverDisabled: c_int = 3;

pub const ScreenSaverBlanked: c_int = 0;
pub const ScreenSaverInternal: c_int = 1;
pub const ScreenSaverExternal: c_int = 2;

pub const ScreenSaverNotify: c_int = 0;
pub const ScreenSaverNumberEvents: c_int = 1;
