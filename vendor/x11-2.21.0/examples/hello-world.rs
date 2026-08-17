// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#![cfg_attr(not(feature = "xlib"), allow(dead_code))]
#![cfg_attr(not(feature = "xlib"), allow(unused_imports))]

extern crate x11;

use std::ffi::CString;
use std::mem;
use std::os::raw::*;
use std::ptr;

use x11::xlib;

#[cfg(not(feature = "xlib"))]
fn main() {
    panic!("this example requires `--features xlib`");
}

#[cfg(feature = "xlib")]
fn main() {
    unsafe {
        // Open display connection.
        let display = xlib::XOpenDisplay(ptr::null());

        if display.is_null() {
            panic!("XOpenDisplay failed");
        }

        // Create window.
        let screen = xlib::XDefaultScreen(display);
        let root = xlib::XRootWindow(display, screen);

        let mut attributes: xlib::XSetWindowAttributes = mem::MaybeUninit::uninit().assume_init();
        attributes.background_pixel = xlib::XWhitePixel(display, screen);

        let window = xlib::XCreateWindow(
            display,
            root,
            0,
            0,
            400,
            300,
            0,
            0,
            xlib::InputOutput as c_uint,
            ptr::null_mut(),
            xlib::CWBackPixel,
            &mut attributes,
        );

        // Set window title.
        let title_str = CString::new("hello-world").unwrap();
        xlib::XStoreName(display, window, title_str.as_ptr() as *mut c_char);

        // Hook close requests.
        let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
        let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

        let wm_protocols = xlib::XInternAtom(display, wm_protocols_str.as_ptr(), xlib::False);
        let wm_delete_window =
            xlib::XInternAtom(display, wm_delete_window_str.as_ptr(), xlib::False);

        let mut protocols = [wm_delete_window];

        xlib::XSetWMProtocols(
            display,
            window,
            protocols.as_mut_ptr(),
            protocols.len() as c_int,
        );

        // Show window.
        xlib::XMapWindow(display, window);

        // Main loop.
        let mut event: xlib::XEvent = mem::MaybeUninit::uninit().assume_init();

        loop {
            xlib::XNextEvent(display, &mut event);

            match event.get_type() {
                xlib::ClientMessage => {
                    let xclient = xlib::XClientMessageEvent::from(event);

                    if xclient.message_type == wm_protocols && xclient.format == 32 {
                        let protocol = xclient.data.get_long(0) as xlib::Atom;

                        if protocol == wm_delete_window {
                            break;
                        }
                    }
                }

                _ => (),
            }
        }

        // Shut down.
        xlib::XCloseDisplay(display);
    }
}
