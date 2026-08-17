use xcb::x;

fn main() -> xcb::Result<()> {
    let rectangles: &[x::Rectangle] = &[x::Rectangle {
        x: 200,
        y: 200,
        width: 400,
        height: 400,
    }];
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    /* get the root window of the screen */
    let window = screen.root();

    /* Request modification of the root window attributes: background color and
     * event mask */
    conn.send_request(&x::ChangeWindowAttributes {
        window,
        value_list: &[
            x::Cw::BackPixel(screen.white_pixel()),
            // events that will be waited for
            x::Cw::EventMask(x::EventMask::EXPOSURE),
        ],
    });

    let gc: x::Gcontext = conn.generate_id();
    /* Request the creation of the graphical context */
    conn.send_request(&x::CreateGc {
        cid: gc,
        drawable: x::Drawable::Window(window),
        value_list: &[
            x::Gc::Foreground(screen.black_pixel()),
            x::Gc::Background(screen.white_pixel()),
            x::Gc::LineWidth(1),
            x::Gc::LineStyle(x::LineStyle::OnOffDash),
            x::Gc::SubwindowMode(x::SubwindowMode::IncludeInferiors),
        ],
    });

    conn.send_request(&x::MapWindow { window });

    /* Flush the requests */
    conn.flush()?;
    loop {
        let event = match conn.wait_for_event() {
            Err(xcb::Error::Connection(xcb::ConnError::Connection)) => {
                // graceful shutdown, likely "x" close button clicked in title bar
                break Ok(());
            }
            Err(err) => {
                panic!("unexpected error: {:#?}", err);
            }
            Ok(event) => event,
        };
        match event {
            xcb::Event::X(x::Event::Expose(_ev)) => {
                let drawable = x::Drawable::Window(window);

                /* Draw a rectangle on screen using the graphical context */
                conn.send_request(&x::PolyRectangle {
                    drawable,
                    gc,
                    rectangles,
                });

                /* We flush the request */
                conn.flush()?;
            }
            _ => {}
        }
    }
}
