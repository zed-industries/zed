use super::xlib::Display;
use std::os::raw::c_void;

x11_link! { Xlib_xcb, xlib_xcb, ["libX11-xcb.so.1", "libX11-xcb.so"], 2,
    pub fn XGetXCBConnection(_1: *mut Display) -> *mut xcb_connection_t,
    pub fn XSetEventQueueOwner(_1: *mut Display, _2: XEventQueueOwner) -> (),
    variadic:
    globals:
}

pub enum XEventQueueOwner {
    XlibOwnsEventQueue = 0,
    XCBOwnsEventQueue = 1,
}

pub type xcb_connection_t = c_void;
