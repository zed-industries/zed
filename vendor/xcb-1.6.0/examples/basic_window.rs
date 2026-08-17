use xcb::{x, Xid};

xcb::atoms_struct! {
    #[derive(Debug)]
    struct Atoms {
        wm_protocols    => b"WM_PROTOCOLS",
        wm_del_window   => b"WM_DELETE_WINDOW",
        wm_state        => b"_NET_WM_STATE",
        wm_state_maxv   => b"_NET_WM_STATE_MAXIMIZED_VERT",
        wm_state_maxh   => b"_NET_WM_STATE_MAXIMIZED_HORZ",
    }
}

fn main() -> xcb::Result<()> {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let window: x::Window = conn.generate_id();

    println!("window: {}", window.resource_id());
    println!("root: {}", screen.root().resource_id());
    println!("root_visual: {}", screen.root_visual());

    conn.send_request(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: window,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 150,
        height: 150,
        border_width: 10,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        value_list: &[
            x::Cw::BackPixel(screen.white_pixel()),
            x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS),
        ],
    });

    conn.send_request(&x::MapWindow { window });

    let title = "Basic Window";

    conn.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        data: title.as_bytes(),
    });

    conn.flush()?;

    // retrieving title
    let cookie = conn.send_request(&x::GetProperty {
        delete: false,
        window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        long_offset: 0,
        long_length: 1024,
    });
    let reply = conn.wait_for_reply(cookie)?;
    assert_eq!(reply.value::<u8>(), title.as_bytes());

    // retrieving a few atoms
    let atoms = Atoms::intern_all(&conn)?;
    println!("atoms = {:#?}", atoms);

    // activate the sending of close event through `x::Event::ClientMessage`
    // either the request must be checked as follow, or conn.flush() must be called before entering the loop
    conn.send_and_check_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: atoms.wm_protocols,
        r#type: x::ATOM_ATOM,
        data: &[atoms.wm_del_window],
    })?;

    let mut maximized = false;

    loop {
        let event = match conn.wait_for_event() {
            Err(xcb::Error::Connection(err)) => {
                panic!("unexpected I/O error: {}", err);
            }
            Err(xcb::Error::Protocol(err)) => {
                panic!("unexpected protocol error: {:#?}", err);
            }
            Ok(event) => event,
        };

        println!("Received event {:#?}", event);
        match event {
            xcb::Event::X(x::Event::KeyPress(ev)) => {
                println!("Key '{}' pressed", ev.detail());

                if ev.detail() == 0x3a {
                    // M (on qwerty)

                    // toggle maximized

                    let data = x::ClientMessageData::Data32([
                        if maximized { 0 } else { 1 },
                        atoms.wm_state_maxv.resource_id(),
                        atoms.wm_state_maxh.resource_id(),
                        0,
                        0,
                    ]);

                    let event = x::ClientMessageEvent::new(window, atoms.wm_state, data);

                    conn.send_request(&x::SendEvent {
                        propagate: false,
                        destination: x::SendEventDest::Window(screen.root()),
                        event_mask: x::EventMask::STRUCTURE_NOTIFY,
                        event: &event,
                    });

                    conn.flush()?;

                    maximized = !maximized;
                } else if ev.detail() == 0x18 {
                    // Q (on qwerty)
                    break Ok(());
                }
            }
            xcb::Event::X(x::Event::ClientMessage(ev)) => {
                if let x::ClientMessageData::Data32([atom, ..]) = ev.data() {
                    if atom == atoms.wm_del_window.resource_id() {
                        // window "x" button clicked by user, we gracefully exit
                        break Ok(());
                    }
                }
            }
            _ => {}
        }
    }
}
