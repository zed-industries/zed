use xcb::{x, xkb, BaseEvent, Xid};

fn setup_xcb() -> xcb::Result<xcb::Connection> {
    let (conn, screen_num) =
        xcb::Connection::connect_with_extensions(None, &[xcb::Extension::Xkb], &[])?;

    // we need at least xkb-1.0 to be available on client machine
    {
        let xkb_ver = conn.wait_for_reply(conn.send_request(&xkb::UseExtension {
            wanted_major: 1,
            wanted_minor: 0,
        }))?;

        assert!(xkb_ver.supported(), "xkb-1.0 support is required");
    }

    // we now select what events we want to receive
    // such as map change, keyboard hotplug ...
    // note that key strokes are given directly by
    // the x::Event::KeyPress, not by xkb
    {
        let events = xkb::EventType::NEW_KEYBOARD_NOTIFY
            | xkb::EventType::MAP_NOTIFY
            | xkb::EventType::STATE_NOTIFY;

        let map_parts = xkb::MapPart::KEY_TYPES
            | xkb::MapPart::KEY_SYMS
            | xkb::MapPart::MODIFIER_MAP
            | xkb::MapPart::EXPLICIT_COMPONENTS
            | xkb::MapPart::KEY_ACTIONS
            | xkb::MapPart::KEY_BEHAVIORS
            | xkb::MapPart::VIRTUAL_MODS
            | xkb::MapPart::VIRTUAL_MOD_MAP;

        let cookie = conn.send_request_checked(&xkb::SelectEvents {
            device_spec: xkb::Id::UseCoreKbd as xkb::DeviceSpec,
            affect_which: events,
            clear: xkb::EventType::empty(),
            select_all: events,
            affect_map: map_parts,
            map: map_parts,
            details: &[],
        });

        conn.check_request(cookie)?;
    }

    // proceed with window creation
    {
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let window: x::Window = conn.generate_id();

        conn.send_request(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: window,
            parent: screen.root(),
            x: 0,
            y: 0,
            width: 500,
            height: 500,
            border_width: 10,
            class: x::WindowClass::InputOutput,
            visual: screen.root_visual(),
            value_list: &[
                x::Cw::BackPixel(screen.white_pixel()),
                // Register to receive keyboard-related events for this window
                x::Cw::EventMask(
                    x::EventMask::FOCUS_CHANGE
                        | x::EventMask::KEY_PRESS
                        | x::EventMask::KEY_RELEASE
                        | x::EventMask::BUTTON_PRESS
                        | x::EventMask::BUTTON_RELEASE
                        | x::EventMask::BUTTON_MOTION,
                ),
            ],
        });
        // Make the window visible
        conn.send_request(&x::MapWindow { window });

        // Set window title
        let title = "XCB Keyboard Event Tester";
        conn.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: title.as_bytes(),
        });

        conn.flush()?;
    }
    Ok(conn)
}

fn print_key_event(
    key_event: &x::KeyPressEvent,
    is_pressed: bool,
    last_xkb_state_event: Option<&xkb::StateNotifyEvent>,
) {
    let winid = key_event.event().resource_id();
    let keycode = key_event.detail();
    let event_mods = key_event.state();
    let down_up = if is_pressed { "DOWN" } else { "UP" };
    println!("Window {winid:#x} received key {down_up}");
    if key_event.is_from_send_event() {
        println!("  (event is synthetic, simulated)");
    }
    println!("  Keycode: {keycode}");
    println!("  Event modifiers: {event_mods:#?}");

    if let Some(xkb_state) = last_xkb_state_event {
        print_xkb_state(&xkb_state);
    }
}

fn print_button_event(
    button_event: &x::ButtonPressEvent,
    is_pressed: bool,
    last_xkb_state_event: Option<&xkb::StateNotifyEvent>,
) {
    let winid = button_event.event().resource_id();
    let button = button_event.detail();
    let event_mods = button_event.state();
    let down_up = if is_pressed { "DOWN" } else { "UP" };
    println!("Window {winid:#x} received mouse button {down_up}");
    if button_event.is_from_send_event() {
        println!("  (event is synthetic, simulated)");
    }
    println!("  Button: {button:#?}");
    println!("  Event modifiers: {event_mods:#?}");

    if let Some(xkb_state) = last_xkb_state_event {
        print_xkb_state(&xkb_state);
    }
}

fn print_xkb_state(xkb_state: &xkb::StateNotifyEvent) {
    let has_mods = !xkb_state.mods().is_empty();
    let has_btns = !xkb_state.ptr_btn_state().is_empty();
    if has_mods || has_btns {
        println!("  XKB last state:");
        println!("    Change: {changed:#?}", changed = xkb_state.changed());
        if has_mods {
            println!("    Modifiers: {mods:#?}", mods = xkb_state.mods());
        }
        if has_btns {
            println!("    Buttons: {btns:#?}", btns = xkb_state.ptr_btn_state());
        }
    }
}

fn main() -> xcb::Result<()> {
    let conn = setup_xcb()?;

    println!();
    println!(">>> XCB & XKB initialized, watching events now");
    println!(">>> Press `Escape` to quit");

    // proceed with the event loop
    let mut last_xkb_state_event = None;
    let mut last_event_is_packed = false;
    while let Ok(event) = conn.wait_for_event() {
        // dbg!(&event); // debug event details

        // decide when to add a blank line before the event
        // used to pack motion events together (without blank line) as they can generate a lot of events
        match (last_event_is_packed, &event) {
            (false, xcb::Event::X(x::Event::MotionNotify(_))) => {
                println!();
                last_event_is_packed = false;
            }
            (true, xcb::Event::X(x::Event::MotionNotify(_))) => {}
            (_, xcb::Event::Xkb(xkb::Event::StateNotify(_))) => {}
            _ => {
                println!();
                last_event_is_packed = false;
            }
        }

        match event {
            xcb::Event::X(x::Event::FocusIn(focus_event)) => {
                let winid = focus_event.event().resource_id();
                println!("Window {winid:#x} got input focus");
            }
            xcb::Event::X(x::Event::FocusOut(focus_event)) => {
                let winid = focus_event.event().resource_id();
                println!("Window {winid:#x} lost input focus");
            }
            xcb::Event::X(x::Event::KeyPress(key_event)) => {
                print_key_event(&key_event, true, last_xkb_state_event.as_ref());
                if key_event.detail() == 9 {
                    println!();
                    println!("`Escape` was pressed, exiting..");
                    break;
                }
            }
            xcb::Event::X(x::Event::KeyRelease(key_event)) => {
                print_key_event(&key_event, false, last_xkb_state_event.as_ref());
            }
            xcb::Event::X(x::Event::ButtonPress(button_event)) => {
                print_button_event(&button_event, true, last_xkb_state_event.as_ref());
            }
            xcb::Event::X(x::Event::ButtonRelease(button_event)) => {
                print_button_event(&button_event, false, last_xkb_state_event.as_ref());
            }
            xcb::Event::X(x::Event::MotionNotify(motion_event)) => {
                let winid = motion_event.event().resource_id();
                let button = motion_event.state();
                // Single line info to use less space, as motions generates a lot of events
                print!("Window {winid:#x} received mouse button ({button:#?}) movement: ");
                println!(
                    "(root x={root_x} y={root_y}) | (win x={win_x} y={win_y})",
                    root_x = motion_event.root_x(),
                    root_y = motion_event.root_y(),
                    win_x = motion_event.event_x(),
                    win_y = motion_event.event_y(),
                );
                last_event_is_packed = true;
            }
            xcb::Event::Xkb(xkb::Event::StateNotify(state_event)) => {
                last_xkb_state_event = Some(state_event);
            }
            _ => {
                dbg!(event); // debug unhandled event details
            }
        }
    }

    Ok(())
}
