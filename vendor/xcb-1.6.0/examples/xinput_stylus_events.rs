use std::env;
use xcb::{x, xinput};

#[derive(Debug)]
struct StylusDevice {
    dev: xinput::Device,
    name: String,
}

fn main() -> xcb::Result<()> {
    // if none, will list devices and exit
    let find_device = {
        let mut args = env::args();
        if args.len() <= 1 {
            eprintln!(r#"warning: no argument, will default to find a "stylus" device"#);
            Some("stylus".to_string())
        } else {
            let arg = args.nth(1).unwrap();
            if arg == "--list" || arg == "-l" {
                None
            } else {
                Some(arg)
            }
        }
    };

    let (conn, screen_num) =
        xcb::Connection::connect_with_extensions(None, &[xcb::Extension::Input], &[]).unwrap();

    // Check if XI2 is supported
    conn.wait_for_reply(conn.send_request(&xinput::XiQueryVersion {
        major_version: 2,
        minor_version: 0,
    }))
    .expect("XI2 not supported");

    let cookie = conn.send_request(&xinput::ListInputDevices {});
    let device_list = conn.wait_for_reply(cookie)?;

    println!("{:#?}", device_list);

    if find_device.is_none() {
        println!("{:#?}", device_list);
        return Ok(());
    }

    let find_device = find_device.unwrap();

    let device = {
        let mut device: Option<StylusDevice> = None;
        for (i, dev) in device_list.devices().iter().enumerate() {
            let name = device_list.names().nth(i).unwrap().name().to_utf8();
            if name.contains(&find_device) {
                device = Some(StylusDevice {
                    dev: xinput::Device::from_id(dev.device_id() as _),
                    name: name.to_string(),
                });
                break;
            }
        }
        device.expect("could not find a stylus device")
    };

    println!("found device \"{}\" ({:?})", device.name, device.dev);

    let cookie = conn.send_request(&xinput::OpenDevice {
        device_id: device.dev.id() as u8,
    });
    conn.wait_for_reply(cookie)?;

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
            x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS),
        ],
    });

    conn.send_request(&x::MapWindow { window });

    let title = "Stylus Window";

    conn.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        data: title.as_bytes(),
    });

    let info =
        conn.wait_for_reply(conn.send_request(&xinput::XiQueryDevice { device: device.dev }));

    println!("{:#?}", info);

    conn.send_request(&xinput::XiSelectEvents {
        window,
        masks: &[xinput::EventMaskBuf::new(
            device.dev,
            &[xinput::XiEventMask::MOTION
                | xinput::XiEventMask::BUTTON_PRESS
                | xinput::XiEventMask::BUTTON_RELEASE],
        )],
    });

    conn.flush()?;

    loop {
        let ev = conn.wait_for_event()?;
        match ev {
            xcb::Event::Input(xinput::Event::Motion(ev)) => {
                // TODO, query valuator infos to ensure on which axis is the pressure
                // This works for me with a Wacom One, but could be different with another device/config
                println!("received stylus motion event");
                println!("  event_x = {}", fp1616_to_f64(ev.event_x()));
                println!("  event_y = {}", fp1616_to_f64(ev.event_y()));
                println!("  pressure = {}", ev.axisvalues()[2].integral);
                println!();
            }
            xcb::Event::Input(xinput::Event::ButtonPress(_ev)) => {
                println!("received stylus press");
            }
            xcb::Event::Input(xinput::Event::ButtonRelease(_ev)) => {
                println!("received stylus release");
            }
            _ => {}
        }
    }

    //Ok(())
}

fn fp1616_to_f64(val: xinput::Fp1616) -> f64 {
    (val as f64) / (u16::MAX as f64)
}
