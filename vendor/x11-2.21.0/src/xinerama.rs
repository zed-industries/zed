// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_int, c_short};

use super::xlib::{Bool, Display, Drawable, Status, Window, XID};

//
// functions
//

x11_link! { Xlib, xinerama, ["libXinerama.so.1", "libXinerama.so"], 10,
  pub fn XineramaIsActive (dpy: *mut Display) -> Bool,
  pub fn XineramaQueryExtension (dpy: *mut Display, event_base: *mut c_int, error_base: *mut c_int) -> Bool,
  pub fn XineramaQueryScreens (dpy: *mut Display, number: *mut c_int) -> *mut XineramaScreenInfo,
  pub fn XineramaQueryVersion (dpy: *mut Display, major_versionp: *mut c_int, minor_versionp: *mut c_int) -> Status,
  pub fn XPanoramiXAllocInfo () -> *mut XPanoramiXInfo,
  pub fn XPanoramiXGetScreenCount (dpy: *mut Display, drawable: Drawable, panoramiX_info: *mut XPanoramiXInfo) -> Status,
  pub fn XPanoramiXGetScreenSize (dpy: *mut Display, drawable: Drawable, screen_num: c_int, panoramiX_info: *mut XPanoramiXInfo) -> Status,
  pub fn XPanoramiXGetState (dpy: *mut Display, drawable: Drawable, panoramiX_info: *mut XPanoramiXInfo) -> Status,
  pub fn XPanoramiXQueryExtension (dpy: *mut Display, event_base_return: *mut c_int, error_base_return: *mut c_int) -> Bool,
  pub fn XPanoramiXQueryVersion (dpy: *mut Display, major_version_return: *mut c_int, minor_version_return: *mut c_int) -> Status,
variadic:
globals:
}

//
// types
//

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XineramaScreenInfo {
    pub screen_number: c_int,
    pub x_org: c_short,
    pub y_org: c_short,
    pub width: c_short,
    pub height: c_short,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XPanoramiXInfo {
    pub window: Window,
    pub screen: c_int,
    pub State: c_int,
    pub width: c_int,
    pub height: c_int,
    pub ScreenCount: c_int,
    pub eventMask: XID,
}
