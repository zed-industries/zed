// SPDX-License-Identifier: MIT OR Apache-2.0 OR Zlib
// Copyright 2022-2023 John Nunley
//
// Licensed under the Apache License, Version 2.0, the MIT License, and
// the Zlib license ("the Licenses"), you may not use this file except in
// compliance with one of the the Licenses, at your option. You may obtain
//  a copy of the Licenses at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//     http://opensource.org/licenses/MIT
//     http://opensource.org/licenses/Zlib
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the Licenses is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the Licenses for the specific language governing permissions and
// limitations under the Licenses.

//! This example uses x11rb to read keyboard symbols.

use x11rb::{
    connection::Connection,
    protocol::{
        xproto::{self, ConnectionExt},
        Event,
    },
    rust_connection::RustConnection,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a connection to the X11 server.
    let (conn, screen) = RustConnection::connect(None)?;

    // Create a window.
    let root = conn.setup().roots[screen].root;
    let background = conn.setup().roots[screen].white_pixel;
    let window = conn.generate_id()?;

    conn.create_window(
        x11rb::COPY_DEPTH_FROM_PARENT,
        window,
        root,
        0,
        0,
        100,
        100,
        0,
        xproto::WindowClass::INPUT_OUTPUT,
        0,
        &xproto::CreateWindowAux::new()
            .background_pixel(background)
            .event_mask(xproto::EventMask::KEY_PRESS),
    )?
    .ignore_error();

    // Set the window for deletion.
    let (wm_protocols, wm_delete_window) = {
        let wmp_tok = conn.intern_atom(false, b"WM_PROTOCOLS")?;
        let wmdw_tok = conn.intern_atom(false, b"WM_DELETE_WINDOW")?;

        let wmp = wmp_tok.reply()?;
        let wmdw = wmdw_tok.reply()?;

        (wmp.atom, wmdw.atom)
    };

    conn.change_property(
        xproto::PropMode::REPLACE,
        window,
        wm_protocols,
        xproto::AtomEnum::ATOM,
        32,
        1,
        bytemuck::bytes_of(&wm_delete_window),
    )?
    .ignore_error();

    // Map the window.
    conn.map_window(window)?.ignore_error();

    // Get the keyboard mapping.
    let mapping = conn
        .get_keyboard_mapping(
            conn.setup().min_keycode,
            conn.setup().max_keycode - conn.setup().min_keycode + 1,
        )?
        .reply()?;

    // Set the window's title.
    let name = b"Type to see keyboard symbols";
    conn.change_property(
        xproto::PropMode::REPLACE,
        window,
        xproto::AtomEnum::WM_NAME,
        xproto::AtomEnum::STRING,
        8,
        name.len() as u32,
        name,
    )?
    .ignore_error();

    // Wait on events.
    loop {
        let event = conn.wait_for_event()?;

        match event {
            Event::ClientMessage(cme) => {
                if cme.type_ == wm_protocols
                    && cme.format == 32
                    && cme.data.as_data32()[0] == wm_delete_window
                {
                    break;
                }
            }
            Event::KeyPress(kpe) => {
                // Translate the keycode to a symbol.
                let keycode = kpe.detail;
                let keysym = xkeysym::keysym(
                    keycode.into(),
                    0,
                    conn.setup().min_keycode.into(),
                    mapping.keysyms_per_keycode,
                    mapping.keysyms.as_slice(),
                );

                // Print the name of the keysym.
                match keysym.and_then(xkeysym::Keysym::name) {
                    Some(name) => println!("{name}"),
                    None => println!("Unknown keysym"),
                }
            }
            _ => {}
        }
    }

    Ok(())
}
