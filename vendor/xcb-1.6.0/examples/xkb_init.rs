use xcb::xkb;

fn main() -> xcb::Result<()> {
    let (conn, _) = xcb::Connection::connect_with_extensions(None, &[xcb::Extension::Xkb], &[])?;

    // we need at least xkb-1.0 to be available on client
    // machine
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

    // proceed with create_window and event loop...

    Ok(())
}
