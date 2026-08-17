// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use super::xlib::{Atom, Bool, Cursor, Display, Pixmap, Status, Time, Window, XRectangle, GC, XID};
use libc::{c_char, c_int, c_short, c_uint, c_ulong, c_ushort};

//
// functions
//

x11_link! { Xlib, x11, ["libXfixes.so.3", "libXfixes.so"], 35,
    pub fn XFixesQueryExtension(dpy: *mut Display, event_base: *mut c_int, error_base: *mut c_int) -> Bool,
    pub fn XFixesQueryVersion(dpy: *mut Display, major_version: *mut c_int, minor_version: *const c_int) -> Status,
    pub fn XFixesVersion() -> c_int,
    pub fn XFixesChangeSaveSet(dpy: *mut Display, win: Window, mode: c_int, target: c_int, map: c_int) -> (),
    pub fn XFixesSelectSelectionInput(dpy: *mut Display, win: Window, selection: Atom, event_mask: c_ulong) -> (),
    pub fn XFixesSelectCursorInput(dpy: Display, win: Window, event_mask: c_ulong) -> (),
    pub fn XFixesGetCursorImage(dpy: *mut Display) -> *mut XFixesCursorImage,
    pub fn XFixesCreateRegion(dpy: *mut Display, rectangles: *mut XRectangle, nrectangles: c_int) -> XserverRegion,
    pub fn XFixesCreateRegionFromBitmap(dpy: *mut Display, bitmap: Pixmap) -> XserverRegion,
    pub fn XFixesCreateRegionFromWindow(dpy: *mut Display, win: Window, kind: c_int) -> XserverRegion,
    pub fn XFixesCreateRegionFromGC(dpy: *mut Display, gc: GC) -> XserverRegion,
    pub fn XFixesCreateRegionFromPicture(dpy: *mut Display, picture: XID) -> XserverRegion,
    pub fn XFixesDestroyRegion(dpy: *mut Display, region: XserverRegion) -> (),
    pub fn XFixesSetRegion(dpy: *mut Display, region: XserverRegion, rectangles: *mut XRectangle, nrectangles: c_int) -> (),
    pub fn XFixesCopyRegion(dpy: *mut Display, dst: XserverRegion, src: XserverRegion) -> (),
    pub fn XFixesUnionRegion(dpy: *mut Display, dst: XserverRegion, src1: XserverRegion, src2: XserverRegion) -> (),
    pub fn XFixesIntersectRegion(dpy: *mut Display, dst: XserverRegion, src1: XserverRegion, src2: XserverRegion) -> (),
    pub fn XFixesSubtractRegion(dpy: *mut Display, dst: XserverRegion, src1: XserverRegion, src2: XserverRegion) -> (),
    pub fn XFixesInvertRegion(dpy: *mut Display, dst: XserverRegion, rect: *mut XRectangle, src: XserverRegion) -> (),
    pub fn XFixesTranslateRegion(dpy: *mut Display, region: XserverRegion, dx: c_int, dy: c_int) -> (),
    pub fn XFixesRegionExtents(dpy: *mut Display, dst: XserverRegion, src: XserverRegion) -> (),
    pub fn XFixesFetchRegion(dpy: *mut Display, region: XserverRegion, nrectangles: *mut c_int) -> *mut XRectangle,
    pub fn XFixesFetchRegionAndBounds(dpy: *mut Display, region: XserverRegion, nrectangles: *mut c_int, bounds: *mut XRectangle) -> *mut XRectangle,
    pub fn XFixesSetGCClipRegion(dpy: *mut Display, gc: GC, clip_x_origin: c_int, clip_y_origin: c_int, region: XserverRegion) -> (),
    pub fn XFixesSetWindowShapeRegion(dpy: *mut Display, win: Window, shape_kind: c_int, x_off: c_int, y_off: c_int, region: XserverRegion) -> (),
    pub fn XFixesSetPictureClipRegion(dpy: *mut Display, picture: XID, clip_x_origin: c_int, clip_y_origin: c_int, region: XserverRegion) -> (),
    pub fn XFixesSetCursorName(dpy: *mut Display, cursor: Cursor, name: *const c_char) -> (),
    pub fn XFixesGetCursorName(dpy: *mut Display, cursor: Cursor, atom: *mut Atom) -> *const c_char,
    pub fn XFixesChangeCursor(dpy: *mut Display, source: Cursor, destination: Cursor) -> (),
    pub fn XFixesChangeCursorByName(dpy: *mut Display, source: Cursor, name: *const c_char) -> (),
    pub fn XFixesExpandRegion(dpy: *mut Display, dst: XserverRegion, src: XserverRegion, left: c_uint, right: c_uint, top: c_uint, bottom: c_uint) -> (),
    pub fn XFixesHideCursor(dpy: *mut Display, win: Window) -> (),
    pub fn XFixesShowCursor(dpy: *mut Display, win: Window) -> (),
    pub fn XFixesCreatePointerBarrier(dpy: *mut Display, w: Window, x1: c_int, y1: c_int, x2: c_int, y2: c_int, directions: c_int, num_devices: c_int, devices: *mut c_int) -> PointerBarrier,
    pub fn XFixesDestroyPointerBarrier(dpy: *mut Display, b: PointerBarrier) -> (),
variadic:
globals:
}

//
// types
//

pub type PointerBarrier = XID;
pub type XserverRegion = XID;

//
// structs
//

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFixesSelectionNotifyEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub owner: Window,
    pub selection: Atom,
    pub timestamp: Time,
    pub selection_timestamp: Time,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFixesCursorNotifyEvent {
    pub _type: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub cursor_serial: c_ulong,
    pub timestamp: Time,
    pub cursor_name: Atom,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFixesCursorImage {
    pub x: c_short,
    pub y: c_short,
    pub width: c_ushort,
    pub height: c_ushort,
    pub xhot: c_ushort,
    pub yhot: c_ushort,
    pub cursor_serial: c_ulong,
    pub pixels: *mut c_ulong,
    pub atom: Atom,
    pub name: *const c_char,
}
