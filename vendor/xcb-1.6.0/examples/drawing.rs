use xcb::x;

fn main() -> xcb::Result<()> {
    let points: &[x::Point] = &[
        x::Point { x: 10, y: 10 },
        x::Point { x: 10, y: 20 },
        x::Point { x: 20, y: 10 },
        x::Point { x: 20, y: 20 },
    ];
    let polyline: &[x::Point] = &[
        x::Point { x: 50, y: 10 },
        x::Point { x: 5, y: 20 },
        x::Point { x: 25, y: -20 }, /* rest of points are relative */
        x::Point { x: 10, y: 10 },
    ];
    let segments: &[x::Segment] = &[
        x::Segment {
            x1: 100,
            y1: 10,
            x2: 140,
            y2: 30,
        },
        x::Segment {
            x1: 110,
            y1: 25,
            x2: 130,
            y2: 60,
        },
    ];
    let rectangles: &[x::Rectangle] = &[
        x::Rectangle {
            x: 10,
            y: 50,
            width: 40,
            height: 20,
        },
        x::Rectangle {
            x: 80,
            y: 50,
            width: 10,
            height: 40,
        },
    ];
    let arcs: &[x::Arc] = &[
        x::Arc {
            x: 10,
            y: 100,
            width: 60,
            height: 40,
            angle1: 0,
            angle2: 90 << 6,
        },
        x::Arc {
            x: 90,
            y: 100,
            width: 55,
            height: 40,
            angle1: 0,
            angle2: 270 << 6,
        },
    ];

    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let gc: x::Gcontext = conn.generate_id();

    let window: x::Window = conn.generate_id();
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

    conn.send_request(&x::CreateGc {
        cid: gc,
        drawable: x::Drawable::Window(window),
        value_list: &[
            x::Gc::Foreground(screen.black_pixel()),
            x::Gc::GraphicsExposures(false),
        ],
    });

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

                /* We draw the points */
                conn.send_request(&x::PolyPoint {
                    coordinate_mode: x::CoordMode::Origin,
                    drawable,
                    gc,
                    points,
                });

                /* We draw the polygonal line */
                conn.send_request(&x::PolyLine {
                    coordinate_mode: x::CoordMode::Previous,
                    drawable,
                    gc,
                    points: &polyline,
                });

                /* We draw the segements */
                conn.send_request(&x::PolySegment {
                    drawable,
                    gc,
                    segments,
                });

                /* We draw the rectangles */
                conn.send_request(&x::PolyRectangle {
                    drawable,
                    gc,
                    rectangles,
                });

                /* We draw the arcs */
                conn.send_request(&x::PolyArc { drawable, gc, arcs });

                /* We flush the request */
                conn.flush()?;
            }

            xcb::Event::X(x::Event::KeyPress(key_press)) => {
                println!("Key '{}' pressed", key_press.detail());
                if key_press.detail() == 0x18 {
                    // Q (on qwerty)
                    break Ok(());
                }
            }
            _ => {}
        }
    }
}
