use std::str;
use xcb::x;

fn main() -> xcb::Result<()> {
    let (conn, _) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();

    let wm_client_list = conn.send_request(&x::InternAtom {
        only_if_exists: true,
        name: "_NET_CLIENT_LIST".as_bytes(),
    });
    let wm_client_list = conn.wait_for_reply(wm_client_list)?.atom();
    assert!(wm_client_list != x::ATOM_NONE, "EWMH not supported");

    for screen in setup.roots() {
        let window = screen.root();

        let pointer = conn.send_request(&x::QueryPointer { window });
        let pointer = conn.wait_for_reply(pointer)?;

        if pointer.same_screen() {
            let list = conn.send_request(&x::GetProperty {
                delete: false,
                window,
                property: wm_client_list,
                r#type: x::ATOM_NONE,
                long_offset: 0,
                long_length: 100,
            });
            let list = conn.wait_for_reply(list)?;

            for client in list.value::<x::Window>() {
                let cookie = conn.send_request(&x::GetProperty {
                    delete: false,
                    window: *client,
                    property: x::ATOM_WM_NAME,
                    r#type: x::ATOM_STRING,
                    long_offset: 0,
                    long_length: 1024,
                });
                let reply = conn.wait_for_reply(cookie)?;
                let title = reply.value();
                let title = str::from_utf8(title).expect("invalid UTF-8");
                println!("{}", title);
            }
        }
    }

    Ok(())
}
