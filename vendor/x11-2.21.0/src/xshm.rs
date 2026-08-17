use super::xlib::{Bool, Display, Drawable, Pixmap, Visual, XImage, GC};
use std::os::raw::{c_char, c_int, c_uint, c_ulong};

x11_link! { Xext, xext, ["libXext.so.6", "libXext.so"], 10,
    pub fn XShmQueryExtension(_1: *mut Display) -> Bool,
    pub fn XShmGetEventBase(_1: *mut Display) -> c_int,
    pub fn XShmQueryVersion(_4: *mut Display, _3: *mut c_int, _2: *mut c_int, _1: *mut Bool) -> Bool,
    pub fn XShmPixmapFormat(_1: *mut Display) -> c_int,
    pub fn XShmAttach(_2: *mut Display, _1: *mut XShmSegmentInfo) -> Bool,
    pub fn XShmDetach(_2: *mut Display, _1: *mut XShmSegmentInfo) -> Bool,
    pub fn XShmPutImage(_11: *mut Display, _10: Drawable, _9: GC, _8: *mut XImage, _7: c_int, _6: c_int, _5: c_int, _4: c_int, _3: c_uint, _2: c_uint, _1: Bool) -> Bool,
    pub fn XShmGetImage(_6: *mut Display, _5: Drawable, _4: *mut XImage, _3: c_int, _2: c_int, _1: c_uint) -> Bool,
    pub fn XShmCreateImage(_8: *mut Display, _7: *mut Visual, _6: c_uint, _5: c_int, _4: *mut c_char, _3: *mut XShmSegmentInfo, _2: c_uint, _1: c_uint) -> *mut XImage,
    pub fn XShmCreatePixmap(_7: *mut Display, _6: Drawable, _5: *mut c_char, _4: *mut XShmSegmentInfo, _3: c_uint, _2: c_uint, _1: c_uint) -> Pixmap,

variadic:
globals:
}

pub type ShmSeg = c_ulong;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct XShmCompletionEvent {
    /// of event
    pub _type: c_int,
    /// # of last request processed by server
    pub serial: c_uint,
    /// true if this came from a SendEvent request
    pub send_event: Bool,
    /// Display the event was read from
    pub diplay: *mut Display,
    /// drawable of request
    pub drawable: *mut Drawable,
    /// ShmReqCode
    pub major_code: c_int,
    /// X_ShmPutImage
    pub minor_code: c_int,
    /// the ShmSeg used in the request
    pub shmseg: ShmSeg,
    /// the offset into ShmSeg used in the request
    pub offset: c_ulong,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct XShmSegmentInfo {
    /// resource id
    pub shmseg: ShmSeg,
    /// kernel id
    pub shmid: c_int,
    /// address in client
    pub shmaddr: *mut c_char,
    /// how the server should attach it
    pub readOnly: Bool,
}
