use xcb::{randr, x};

fn main() -> xcb::Result<()> {
    let (conn, screen_num) = xcb::Connection::connect(None)?;
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let window_dummy: x::Window = conn.generate_id();

    conn.send_request(&x::CreateWindow {
        depth: 0,
        wid: window_dummy,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 1,
        height: 1,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        value_list: &[],
    });

    conn.flush()?;

    let screen_res = conn.wait_for_reply(conn.send_request(&randr::GetScreenResources {
        window: window_dummy,
    }))?;

    let crtc_cookies = screen_res
        .crtcs()
        .iter()
        .map(|crtc| {
            conn.send_request(&randr::GetCrtcInfo {
                crtc: *crtc,
                config_timestamp: 0,
            })
        })
        .collect::<Vec<_>>();

    for (i, cookie) in crtc_cookies.into_iter().enumerate() {
        let reply = conn.wait_for_reply(cookie)?;
        if i != 0 {
            println!("");
        }
        println!("CRTC[{}] INFO:", i);
        println!(" x-off\t: {}", reply.x());
        println!(" y-off\t: {}", reply.y());
        println!(" width\t: {}", reply.width());
        println!(" height\t: {}", reply.height());
    }

    Ok(())
}
