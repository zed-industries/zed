mod dispatcher;
mod headless;
mod keyboard;
mod platform;
mod system_notifications;
#[cfg(any(feature = "wayland", feature = "x11"))]
mod text_system;
#[cfg(feature = "wayland")]
mod wayland;
#[cfg(feature = "x11")]
mod x11;

#[cfg(any(feature = "wayland", feature = "x11"))]
mod xdg_desktop_portal;

pub use dispatcher::*;
pub(crate) use headless::*;
pub(crate) use keyboard::*;
pub(crate) use platform::*;
#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) use text_system::*;
#[cfg(feature = "wayland")]
pub(crate) use wayland::*;
#[cfg(feature = "x11")]
pub(crate) use x11::*;

use std::rc::Rc;

#[cfg(any(feature = "wayland", feature = "x11"))]
use anyhow::Context as _;
use anyhow::anyhow;

/// Returns the default platform implementation for the current OS.
pub fn current_platform(headless: bool) -> Rc<dyn gpui::Platform> {
    if headless {
        return Rc::new(LinuxPlatform::new(HeadlessClient::new()));
    }

    match gpui::guess_compositor() {
        #[cfg(feature = "wayland")]
        "Wayland" => Rc::new(LinuxPlatform::new(WaylandClient::new())),

        #[cfg(feature = "x11")]
        "X11" => Rc::new(LinuxPlatform::new(
            X11Client::new()
                .context("Failed to initialize X11 client.")
                .unwrap(),
        )),

        "Headless" => Rc::new(LinuxPlatform::new(HeadlessClient::new())),
        _ => unreachable!(
            r#"At least one of the "wayland" or "x11" features must be enabled on gpui_linux or gpui_platform."#
        ),
    }
}

/// Returns the native X11 or Wayland platform with a non-blocking event pump.
pub fn embedded_platform() -> anyhow::Result<Rc<dyn gpui::EmbeddedPlatform>> {
    match gpui::guess_compositor() {
        #[cfg(feature = "wayland")]
        "Wayland" => Ok(Rc::new(LinuxPlatform::new_embedded(
            WaylandClient::try_new().context("Failed to initialize Wayland client.")?,
        ))),

        #[cfg(feature = "x11")]
        "X11" => Ok(Rc::new(LinuxPlatform::new_embedded(
            X11Client::new().context("Failed to initialize X11 client.")?,
        ))),

        "Headless" => Err(anyhow!(
            "embedded Linux platforms require an X11 or Wayland display"
        )),
        _ => unreachable!(
            r#"At least one of the "wayland" or "x11" features must be enabled on gpui_linux or gpui_platform."#
        ),
    }
}
