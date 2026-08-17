/*
 * Copyright (C) 2013 James Miller <james@aatch.net>
 * Copyright (c) 2016
 *         Remi Thebault <remi.thebault@gmail.com>
 *         Thomas Bracht Laumann Jespersen <laumann.thomas@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any
 * person obtaining a copy of this software and associated
 * documentation files (the "Software"), to deal in the
 * Software without restriction, including without
 * limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software
 * is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice
 * shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
 * ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
 * TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
 * PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
 * SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 * OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
 * IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::ffi::xcb_connection_t;
use libc::c_uint;

use x11::xlib;

/// Type for [XSetEventQueueOwner] owner parameter
///
/// This item is behind the `xlib_xcb` cargo feature.
pub type XEventQueueOwner = c_uint;

/// Xlib owns the event queue
///
/// This item is behind the `xlib_xcb` cargo feature.
pub const XlibOwnsEventQueue: XEventQueueOwner = 0;

/// XCB owns the event queue
///
/// This item is behind the `xlib_xcb` cargo feature.
pub const XCBOwnsEventQueue: XEventQueueOwner = 1;

#[link(name = "X11-xcb")]
extern "C" {
    /// Get an XCB connection from the `xlib::Display`.
    ///
    /// This function is behind the `xlib_xcb` cargo feature.
    pub fn XGetXCBConnection(dpy: *mut xlib::Display) -> *mut xcb_connection_t;
    /// Set the owner of the X client event queue.
    ///
    /// This function is behind the `xlib_xcb` cargo feature.
    pub fn XSetEventQueueOwner(dpy: *mut xlib::Display, owner: XEventQueueOwner);
}
