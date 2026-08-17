#![feature(trait_alias)]

use x11rb::connection::Connection;
use x11rb::protocol::{xproto::*, Event};
use xim::{x11rb::X11rbClient, Client};
use xim_parser::ForwardEventFlag;

use self::handler::ExampleHandler;

#[path = "util/handler.rs"]
mod handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_custom_env("XIM_RS_LOG");

    let (conn, screen_num) =
        x11rb::rust_connection::RustConnection::connect(None).expect("Connect X");
    let screen = &conn.setup().roots[screen_num];
    let window = conn.generate_id()?;
    conn.create_window(
        screen.root_depth,
        window,
        screen.root,
        0,
        0,
        800,
        600,
        0,
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,
        &CreateWindowAux::default()
            .background_pixel(screen.black_pixel)
            .event_mask(EventMask::KEY_PRESS | EventMask::KEY_RELEASE),
    )?;
    conn.map_window(window)?;
    conn.flush()?;

    let mut client = X11rbClient::init(&conn, screen_num, None)?;

    log::info!("Start event loop");

    let mut handler = ExampleHandler {
        window,
        ..ExampleHandler::default()
    };

    loop {
        let e = conn.wait_for_event()?;

        if client.filter_event(&e, &mut handler)? {
            continue;
        } else if let Event::Error(err) = e {
            log::error!("X11Error: {:?}", err);
            continue;
        } else {
            match e {
                Event::KeyPress(e) | Event::KeyRelease(e) => {
                    if handler.connected {
                        log::trace!("Send: {:?}", e);
                        client.forward_event(
                            handler.im_id,
                            handler.ic_id,
                            ForwardEventFlag::empty(),
                            &e,
                        )?;
                    }
                }
                _ => {}
            }
        }
    }
}
