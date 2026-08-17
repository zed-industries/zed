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

    let reply = conn.wait_for_reply(conn.send_request(&randr::GetScreenInfo {
        window: window_dummy,
    }))?;

    for (i, size) in reply.sizes().iter().enumerate() {
        if i != 0 {
            println!("");
        }
        println!("size of screen {}:", i + 1);
        println!(
            "   {} x {} ({}mm x {}mm)",
            size.width, size.height, size.mwidth, size.mheight,
        );
    }

    Ok(())
}
