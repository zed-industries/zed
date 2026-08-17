// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_char, c_float, c_int, c_uchar, c_uint, c_ulong, c_ushort};

use super::xlib::{Bool, Display, Time, Window, XEvent};

//
// functions
//

x11_link! { Xf86vmode, xxf86vm, ["libXxf86vm.so.1", "libXxf86vm.so"], 22,
  pub fn XF86VidModeAddModeLine (_4: *mut Display, _3: c_int, _2: *mut XF86VidModeModeInfo, _1: *mut XF86VidModeModeInfo) -> c_int,
  pub fn XF86VidModeDeleteModeLine (_3: *mut Display, _2: c_int, _1: *mut XF86VidModeModeInfo) -> c_int,
  pub fn XF86VidModeGetAllModeLines (_4: *mut Display, _3: c_int, _2: *mut c_int, _1: *mut *mut *mut XF86VidModeModeInfo) -> c_int,
  pub fn XF86VidModeGetDotClocks (_6: *mut Display, _5: c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut *mut c_int) -> c_int,
  pub fn XF86VidModeGetGamma (_3: *mut Display, _2: c_int, _1: *mut XF86VidModeGamma) -> c_int,
  pub fn XF86VidModeGetGammaRamp (_6: *mut Display, _5: c_int, _4: c_int, _3: *mut c_ushort, _2: *mut c_ushort, _1: *mut c_ushort) -> c_int,
  pub fn XF86VidModeGetGammaRampSize (_3: *mut Display, _2: c_int, _1: *mut c_int) -> c_int,
  pub fn XF86VidModeGetModeLine (_4: *mut Display, _3: c_int, _2: *mut c_int, _1: *mut XF86VidModeModeLine) -> c_int,
  pub fn XF86VidModeGetMonitor (_3: *mut Display, _2: c_int, _1: *mut XF86VidModeMonitor) -> c_int,
  pub fn XF86VidModeGetPermissions (_3: *mut Display, _2: c_int, _1: *mut c_int) -> c_int,
  pub fn XF86VidModeGetViewPort (_4: *mut Display, _3: c_int, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XF86VidModeLockModeSwitch (_3: *mut Display, _2: c_int, _1: c_int) -> c_int,
  pub fn XF86VidModeModModeLine (_3: *mut Display, _2: c_int, _1: *mut XF86VidModeModeLine) -> c_int,
  pub fn XF86VidModeQueryExtension (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XF86VidModeQueryVersion (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XF86VidModeSetClientVersion (_1: *mut Display) -> c_int,
  pub fn XF86VidModeSetGamma (_3: *mut Display, _2: c_int, _1: *mut XF86VidModeGamma) -> c_int,
  pub fn XF86VidModeSetGammaRamp (_6: *mut Display, _5: c_int, _4: c_int, _3: *mut c_ushort, _2: *mut c_ushort, _1: *mut c_ushort) -> c_int,
  pub fn XF86VidModeSetViewPort (_4: *mut Display, _3: c_int, _2: c_int, _1: c_int) -> c_int,
  pub fn XF86VidModeSwitchMode (_3: *mut Display, _2: c_int, _1: c_int) -> c_int,
  pub fn XF86VidModeSwitchToMode (_3: *mut Display, _2: c_int, _1: *mut XF86VidModeModeInfo) -> c_int,
  pub fn XF86VidModeValidateModeLine (_3: *mut Display, _2: c_int, _1: *mut XF86VidModeModeInfo) -> c_int,
variadic:
globals:
}

//
// types
//

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XF86VidModeGamma {
    pub red: c_float,
    pub green: c_float,
    pub blue: c_float,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XF86VidModeModeInfo {
    pub dotclock: c_uint,
    pub hdisplay: c_ushort,
    pub hsyncstart: c_ushort,
    pub hsyncend: c_ushort,
    pub htotal: c_ushort,
    pub hskew: c_ushort,
    pub vdisplay: c_ushort,
    pub vsyncstart: c_ushort,
    pub vsyncend: c_ushort,
    pub vtotal: c_ushort,
    pub flags: c_uint,
    pub privsize: c_int,
    pub private: *mut i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XF86VidModeModeLine {
    pub hdisplay: c_ushort,
    pub hsyncstart: c_ushort,
    pub hsyncend: c_ushort,
    pub htotal: c_ushort,
    pub hskew: c_ushort,
    pub vdisplay: c_ushort,
    pub vsyncstart: c_ushort,
    pub vsyncend: c_ushort,
    pub vtotal: c_ushort,
    pub flags: c_uint,
    pub privsize: c_int,
    pub private: *mut i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XF86VidModeMonitor {
    pub vendor: *mut c_char,
    pub model: *mut c_char,
    pub EMPTY: c_float,
    pub nhsync: c_uchar,
    pub hsync: *mut XF86VidModeSyncRange,
    pub nvsync: c_uchar,
    pub vsync: *mut XF86VidModeSyncRange,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XF86VidModeSyncRange {
    pub hi: c_float,
    pub lo: c_float,
}

//
// event structures
//

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XF86VidModeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub root: Window,
    pub state: c_int,
    pub kind: c_int,
    pub forced: Bool,
    pub time: Time,
}

event_conversions_and_tests! {
  xf86vm_notify: XF86VidModeNotifyEvent,
}
