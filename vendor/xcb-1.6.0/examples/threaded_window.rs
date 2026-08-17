extern crate xcb;

use std::iter::Iterator;
use std::sync::Arc;
use std::{thread, time};

use xcb::x;

fn main() -> xcb::Result<()> {
    let (conn, screen_num) = {
        let (conn, screen_num) = xcb::Connection::connect(None)?;
        (Arc::new(conn), screen_num)
    };
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let window = conn.generate_id();

    let values = [
        x::Cw::BackPixel(screen.white_pixel()),
        x::Cw::EventMask(
            x::EventMask::EXPOSURE
                | x::EventMask::KEY_PRESS
                | x::EventMask::STRUCTURE_NOTIFY
                | x::EventMask::PROPERTY_CHANGE,
        ),
    ];

    let create_window_cookie = conn.send_request_checked(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: window,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 320,
        height: 240,
        border_width: 10,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        value_list: &values,
    });
    conn.check_request(create_window_cookie)?;

    let map_window_cookie = conn.send_request_checked(&x::MapWindow { window });
    conn.check_request(map_window_cookie)?;

    {
        let conn = conn.clone();
        thread::spawn(move || {
            let mut blink = false;
            loop {
                let title = if blink {
                    "Basic Threaded Window ;-)"
                } else {
                    "Basic Threaded Window :-)"
                };

                let cookie = conn.send_request_checked(&x::ChangeProperty {
                    mode: x::PropMode::Replace,
                    window,
                    property: x::ATOM_WM_NAME,
                    r#type: x::ATOM_STRING,
                    data: title.as_bytes(),
                });

                if conn.has_error().is_err() || conn.check_request(cookie).is_err() {
                    break;
                }

                blink = !blink;
                thread::sleep(time::Duration::from_millis(500));
            }
        });
    }

    loop {
        let event = conn.wait_for_event();
        match event {
            Ok(xcb::Event::X(x::Event::PropertyNotify(ev))) => {
                if ev.atom() == x::ATOM_WM_NAME {
                    // retrieving title
                    let cookie = conn.send_request(&x::GetProperty {
                        delete: false,
                        window,
                        property: x::ATOM_WM_NAME,
                        r#type: x::ATOM_STRING,
                        long_offset: 0,
                        long_length: 1024,
                    });
                    let title = conn.wait_for_reply(cookie)?;

                    println!(
                        "title changed to \"{}\"",
                        std::str::from_utf8(title.value()).unwrap()
                    );
                }
            }
            Ok(xcb::Event::X(x::Event::KeyPress(ev))) => {
                println!("Key '{}' pressed", ev.detail());

                if ev.detail() == 0x18 {
                    // Q (on qwerty)
                    break;
                }
            }
            Ok(_) => {
                continue;
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}
