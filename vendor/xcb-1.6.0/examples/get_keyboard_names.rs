use xcb::{self, xinput, xkb};
use xcb::{Connection, Extension};

fn main() -> xcb::Result<()> {
    let (conn, _screen_num) = Connection::connect_with_xlib_display_and_extensions(
        &[Extension::Input, Extension::Xkb],
        &[],
    )?;

    let cookie = conn.send_request(&xinput::ListInputDevices {});
    let devices = conn.wait_for_reply(cookie)?;
    devices
        .devices()
        .iter()
        .filter(|device| {
            device.device_use() == xinput::DeviceUse::IsXExtensionKeyboard
                || device.device_use() == xinput::DeviceUse::IsXKeyboard
        })
        .try_for_each::<_, xcb::Result<()>>(|device| {
            let device_spec = device.device_id() as xkb::DeviceSpec;
            let cookie = conn.send_request(&xcb::xkb::GetNames {
                device_spec,
                which: xkb::NameDetail::SYMBOLS,
            });
            let components = conn.wait_for_reply(cookie)?;
            println!(
                "Get components for device {} got result {:?}",
                device_spec,
                components.value_list()
            );
            Ok(())
        })?;
    Ok(())
}
