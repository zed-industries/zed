// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong};

use super::xlib::{Atom, Display, Time, XEvent, XModifierKeymap, XID};

//
// functions
//

x11_link! { XInput, xi, ["libXi.so.6", "libXi.so"], 44,
  pub fn XAllowDeviceEvents (_4: *mut Display, _3: *mut XDevice, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XChangeDeviceControl (_4: *mut Display, _3: *mut XDevice, _2: c_int, _1: *mut XDeviceControl) -> c_int,
  pub fn XChangeDeviceDontPropagateList (_5: *mut Display, _4: c_ulong, _3: c_int, _2: *mut c_ulong, _1: c_int) -> c_int,
  pub fn XChangeDeviceKeyMapping (_6: *mut Display, _5: *mut XDevice, _4: c_int, _3: c_int, _2: *mut c_ulong, _1: c_int) -> c_int,
  pub fn XChangeDeviceProperty (_8: *mut Display, _7: *mut XDevice, _6: c_ulong, _5: c_ulong, _4: c_int, _3: c_int, _2: *const c_uchar, _1: c_int) -> (),
  pub fn XChangeFeedbackControl (_4: *mut Display, _3: *mut XDevice, _2: c_ulong, _1: *mut XFeedbackControl) -> c_int,
  pub fn XChangeKeyboardDevice (_2: *mut Display, _1: *mut XDevice) -> c_int,
  pub fn XChangePointerDevice (_4: *mut Display, _3: *mut XDevice, _2: c_int, _1: c_int) -> c_int,
  pub fn XCloseDevice (_2: *mut Display, _1: *mut XDevice) -> c_int,
  pub fn XDeleteDeviceProperty (_3: *mut Display, _2: *mut XDevice, _1: c_ulong) -> (),
  pub fn XDeviceBell (_5: *mut Display, _4: *mut XDevice, _3: c_ulong, _2: c_ulong, _1: c_int) -> c_int,
  pub fn XFreeDeviceControl (_1: *mut XDeviceControl) -> (),
  pub fn XFreeDeviceList (_1: *mut XDeviceInfo) -> (),
  pub fn XFreeDeviceMotionEvents (_1: *mut XDeviceTimeCoord) -> (),
  pub fn XFreeDeviceState (_1: *mut XDeviceState) -> (),
  pub fn XFreeFeedbackList (_1: *mut XFeedbackState) -> (),
  pub fn XGetDeviceButtonMapping (_4: *mut Display, _3: *mut XDevice, _2: *mut c_uchar, _1: c_uint) -> c_int,
  pub fn XGetDeviceControl (_3: *mut Display, _2: *mut XDevice, _1: c_int) -> *mut XDeviceControl,
  pub fn XGetDeviceDontPropagateList (_3: *mut Display, _2: c_ulong, _1: *mut c_int) -> *mut c_ulong,
  pub fn XGetDeviceFocus (_5: *mut Display, _4: *mut XDevice, _3: *mut c_ulong, _2: *mut c_int, _1: *mut c_ulong) -> c_int,
  pub fn XGetDeviceKeyMapping (_5: *mut Display, _4: *mut XDevice, _3: c_uchar, _2: c_int, _1: *mut c_int) -> *mut c_ulong,
  pub fn XGetDeviceModifierMapping (_2: *mut Display, _1: *mut XDevice) -> *mut XModifierKeymap,
  pub fn XGetDeviceMotionEvents (_7: *mut Display, _6: *mut XDevice, _5: c_ulong, _4: c_ulong, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> *mut XDeviceTimeCoord,
  pub fn XGetDeviceProperty (_12: *mut Display, _11: *mut XDevice, _10: c_ulong, _9: c_long, _8: c_long, _7: c_int, _6: c_ulong, _5: *mut c_ulong, _4: *mut c_int, _3: *mut c_ulong, _2: *mut c_ulong, _1: *mut *mut c_uchar) -> c_int,
  pub fn XGetExtensionVersion (_2: *mut Display, _1: *const c_char) -> *mut XExtensionVersion,
  pub fn XGetFeedbackControl (_3: *mut Display, _2: *mut XDevice, _1: *mut c_int) -> *mut XFeedbackState,
  pub fn XGetSelectedExtensionEvents (_6: *mut Display, _5: c_ulong, _4: *mut c_int, _3: *mut *mut c_ulong, _2: *mut c_int, _1: *mut *mut c_ulong) -> c_int,
  pub fn XGrabDevice (_9: *mut Display, _8: *mut XDevice, _7: c_ulong, _6: c_int, _5: c_int, _4: *mut c_ulong, _3: c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XGrabDeviceButton (_11: *mut Display, _10: *mut XDevice, _9: c_uint, _8: c_uint, _7: *mut XDevice, _6: c_ulong, _5: c_int, _4: c_uint, _3: *mut c_ulong, _2: c_int, _1: c_int) -> c_int,
  pub fn XGrabDeviceKey (_11: *mut Display, _10: *mut XDevice, _9: c_uint, _8: c_uint, _7: *mut XDevice, _6: c_ulong, _5: c_int, _4: c_uint, _3: *mut c_ulong, _2: c_int, _1: c_int) -> c_int,

  pub fn XListDeviceProperties (_3: *mut Display, _2: *mut XDevice, _1: *mut c_int) -> *mut c_ulong,
  pub fn XListInputDevices (_2: *mut Display, _1: *mut c_int) -> *mut XDeviceInfo,
  pub fn XOpenDevice (_2: *mut Display, _1: c_ulong) -> *mut XDevice,
  pub fn XQueryDeviceState (_2: *mut Display, _1: *mut XDevice) -> *mut XDeviceState,
  pub fn XSelectExtensionEvent (_4: *mut Display, _3: c_ulong, _2: *mut c_ulong, _1: c_int) -> c_int,
  pub fn XSendExtensionEvent (_7: *mut Display, _6: *mut XDevice, _5: c_ulong, _4: c_int, _3: c_int, _2: *mut c_ulong, _1: *mut XEvent) -> c_int,
  pub fn XSetDeviceButtonMapping (_4: *mut Display, _3: *mut XDevice, _2: *mut c_uchar, _1: c_int) -> c_int,
  pub fn XSetDeviceFocus (_5: *mut Display, _4: *mut XDevice, _3: c_ulong, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XSetDeviceMode (_3: *mut Display, _2: *mut XDevice, _1: c_int) -> c_int,
  pub fn XSetDeviceModifierMapping (_3: *mut Display, _2: *mut XDevice, _1: *mut XModifierKeymap) -> c_int,
  pub fn XSetDeviceValuators (_5: *mut Display, _4: *mut XDevice, _3: *mut c_int, _2: c_int, _1: c_int) -> c_int,
  pub fn XUngrabDevice (_3: *mut Display, _2: *mut XDevice, _1: c_ulong) -> c_int,
  pub fn XUngrabDeviceButton (_6: *mut Display, _5: *mut XDevice, _4: c_uint, _3: c_uint, _2: *mut XDevice, _1: c_ulong) -> c_int,
  pub fn XUngrabDeviceKey (_6: *mut Display, _5: *mut XDevice, _4: c_uint, _3: c_uint, _2: *mut XDevice, _1: c_ulong) -> c_int,
variadic:
globals:
}

//
// types
//

pub enum _XAnyClassinfo {}

pub type XAnyClassPtr = *mut _XAnyClassinfo;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XDevice {
    pub device_id: XID,
    pub num_classes: c_int,
    pub classes: *mut XInputClassInfo,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XDeviceControl {
    pub control: XID,
    pub length: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XDeviceInfo {
    pub id: XID,
    pub type_: Atom,
    pub name: *mut c_char,
    pub num_classes: c_int,
    pub use_: c_int,
    pub inputclassinfo: XAnyClassPtr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XDeviceState {
    pub device_id: XID,
    pub num_classes: c_int,
    pub data: *mut XInputClass,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XDeviceTimeCoord {
    pub time: Time,
    pub data: *mut c_int,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XExtensionVersion {
    pub present: c_int,
    pub major_version: c_short,
    pub minor_version: c_short,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFeedbackControl {
    pub class: XID,
    pub length: c_int,
    pub id: XID,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFeedbackState {
    pub class: XID,
    pub length: c_int,
    pub id: XID,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XInputClass {
    pub class: c_uchar,
    pub length: c_uchar,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XInputClassInfo {
    pub input_class: c_uchar,
    pub event_type_base: c_uchar,
}
