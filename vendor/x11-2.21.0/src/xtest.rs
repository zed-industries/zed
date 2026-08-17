// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_int, c_uint, c_ulong};

use super::xinput::XDevice;
use super::xlib::{Display, Visual, GC};

//
// functions
//

x11_link! { Xf86vmode, xtst, ["libXtst.so.6", "libXtst.so"], 15,
  pub fn XTestCompareCurrentCursorWithWindow (_2: *mut Display, _1: c_ulong) -> c_int,
  pub fn XTestCompareCursorWithWindow (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int,
  pub fn XTestDiscard (_1: *mut Display) -> c_int,
  pub fn XTestFakeButtonEvent (_4: *mut Display, _3: c_uint, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeDeviceButtonEvent (_7: *mut Display, _6: *mut XDevice, _5: c_uint, _4: c_int, _3: *mut c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeDeviceKeyEvent (_7: *mut Display, _6: *mut XDevice, _5: c_uint, _4: c_int, _3: *mut c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeDeviceMotionEvent (_7: *mut Display, _6: *mut XDevice, _5: c_int, _4: c_int, _3: *mut c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeKeyEvent (_4: *mut Display, _3: c_uint, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeMotionEvent (_5: *mut Display, _4: c_int, _3: c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeProximityEvent (_6: *mut Display, _5: *mut XDevice, _4: c_int, _3: *mut c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeRelativeMotionEvent (_5: *mut Display, _4: c_int, _3: c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestGrabControl (_2: *mut Display, _1: c_int) -> c_int,
  pub fn XTestQueryExtension (_5: *mut Display, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XTestSetGContextOfGC (_2: GC, _1: c_ulong) -> (),
  pub fn XTestSetVisualIDOfVisual (_2: *mut Visual, _1: c_ulong) -> (),
variadic:
globals:
}
