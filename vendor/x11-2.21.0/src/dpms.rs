// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::c_int;

use super::xlib::{Bool, Display, Status};
use super::xmd::{BOOL, CARD16};

//
// functions
//

x11_link! { Xext, xext, ["libXext.so.6", "libXext.so"], 9,
  pub fn DPMSQueryExtension (_1: *mut Display, _2: *mut c_int, _3: *mut c_int) -> Bool,
  pub fn DPMSGetVersion (_1: *mut Display, _2: *mut c_int, _3: *mut c_int) -> Status,
  pub fn DPMSCapable (_1: *mut Display) -> Bool,
  pub fn DPMSSetTimeouts (_1: *mut Display, _2: CARD16, _3: CARD16, _4: CARD16) -> Status,
  pub fn DPMSGetTimeouts (_1: *mut Display, _2: *mut CARD16, _3: *mut CARD16, _4: *mut CARD16) -> Bool,
  pub fn DPMSEnable (_1: *mut Display) -> Status,
  pub fn DPMSDisable (_1: *mut Display) -> Status,
  pub fn DPMSForceLevel (_1: *mut Display, _2: CARD16) -> Status,
  pub fn DPMSInfo (_1: *mut Display, _2: *mut CARD16, _3: *mut BOOL) -> Status,
variadic:
globals:
}

//
// constants
//

pub const DPMSMajorVersion: c_int = 1;
pub const DPMSMinorVersion: c_int = 1;

pub const DPMSExtensionName: &str = "DPMS";

pub const DPMSModeOn: CARD16 = 0;
pub const DPMSModeStandby: CARD16 = 1;
pub const DPMSModeSuspend: CARD16 = 2;
pub const DPMSModeOff: CARD16 = 3;
