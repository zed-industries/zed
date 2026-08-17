use xcb::randr;

fn main() -> xcb::Result<()> {
    let (conn, screen_num) =
        xcb::Connection::connect_with_extensions(None, &[xcb::Extension::RandR], &[]).unwrap();
    let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();

    conn.check_request(conn.send_request_checked(&randr::SelectInput {
        window: screen.root(),
        enable: randr::NotifyMask::CRTC_CHANGE,
    }))?;

    loop {
        conn.flush()?;

        let event = match conn.wait_for_event() {
            Err(xcb::Error::Connection(err)) => {
                panic!("unexpected I/O error: {}", err);
            }
            Err(xcb::Error::Protocol(err)) => {
                panic!("unexpected protocol error: {:#?}", err);
            }
            Ok(event) => event,
        };

        match event {
            xcb::Event::RandR(randr::Event::Notify(ev)) => match ev.u() {
                randr::NotifyData::Cc(cc) => {
                    println!(
                        "received CRTC_CHANGE event:\n\
                     \ttimestamp: {}, window: {:?}, crtc: {:?}, mode: {:?}, rotation: {:?}\n\
                     \tx: {}, y: {}, width: {}, height: {}",
                        cc.timestamp(),
                        cc.window(),
                        cc.crtc(),
                        cc.mode(),
                        cc.rotation(),
                        cc.x(),
                        cc.y(),
                        cc.width(),
                        cc.height()
                    );
                }
                _ => {}
            },
            _ => {}
        }
    }
}
