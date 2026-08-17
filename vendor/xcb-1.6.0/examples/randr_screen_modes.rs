use xcb::{randr, x};

// per https://gitlab.freedesktop.org/xorg/app/xrandr/-/blob/master/xrandr.c#L576
fn mode_refresh(mode_info: &randr::ModeInfo) -> f64 {
    let vtotal = {
        let mut val = mode_info.vtotal;
        if mode_info.mode_flags.contains(randr::ModeFlag::DOUBLE_SCAN) {
            val *= 2;
        }
        if mode_info.mode_flags.contains(randr::ModeFlag::INTERLACE) {
            val /= 2;
        }
        val
    };

    if vtotal != 0 && mode_info.htotal != 0 {
        (mode_info.dot_clock as f64) / (vtotal as f64 * mode_info.htotal as f64)
    } else {
        0.0
    }
}

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

    let reply = conn.wait_for_reply(conn.send_request(&randr::GetScreenResources {
        window: window_dummy,
    }))?;

    for (i, mode) in reply.modes().iter().enumerate() {
        println!("mode {}", i + 1);
        println!("\tresolution = {}x{}", mode.width, mode.height);
        println!("\trefresh rate = {:.1}Hz", mode_refresh(mode));
    }

    Ok(())
}
