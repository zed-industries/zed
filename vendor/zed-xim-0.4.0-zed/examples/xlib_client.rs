#![feature(trait_alias)]

use std::{mem::MaybeUninit, ptr};
use x11_dl::xlib;
use xim::{xlib::XlibClient, Client};
use xim_parser::ForwardEventFlag;

use self::handler::ExampleHandler;

#[path = "util/handler.rs"]
mod handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let xlib = xlib::Xlib::open()?;

    unsafe {
        let display = (xlib.XOpenDisplay)(ptr::null());
        let root = (xlib.XDefaultRootWindow)(display);
        let window = (xlib.XCreateSimpleWindow)(
            display,
            root,
            0,
            0,
            800,
            600,
            0,
            (xlib.XBlackPixel)(display, 0),
            (xlib.XBlackPixel)(display, 0),
        );
        (xlib.XMapWindow)(display, window);

        let mut client = XlibClient::init(&xlib, display, None)?;

        log::info!("Start event loop");

        let mut handler = ExampleHandler {
            window: window as _,
            ..ExampleHandler::default()
        };

        (xlib.XSelectInput)(display, window, xlib::KeyPressMask | xlib::KeyReleaseMask);

        loop {
            let mut e = MaybeUninit::uninit();
            (xlib.XNextEvent)(display, e.as_mut_ptr());
            let e = e.assume_init();

            log::trace!("Get event: {:?}", e);

            if client.filter_event(&e, &mut handler)? {
                continue;
            } else {
                match e.get_type() {
                    xlib::KeyPress | xlib::KeyRelease => {
                        if handler.connected {
                            client.forward_event(
                                handler.im_id,
                                handler.ic_id,
                                ForwardEventFlag::empty(),
                                &e.key,
                            )?;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
