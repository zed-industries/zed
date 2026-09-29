mod dispatcher;
mod display_connection;
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
pub(crate) use display_connection::{Backend, DisplayConnection, select_backend};
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

use gpui::{DisplayModes, GraphicalEnvironment};

/// Returns the default platform implementation for the current OS.
///
/// A windowed platform connects to the display server the process environment names, or starts
/// headless when it names none, and can switch between those modes later. A headless platform
/// stays headless.
pub fn current_platform(headless: bool) -> Rc<dyn gpui::Platform> {
    if headless || std::env::var_os("ZED_HEADLESS").is_some() {
        linux_platform(DisplayModes::HEADLESS, None)
    } else {
        linux_platform(DisplayModes::all(), Some(GraphicalEnvironment::detect()))
    }
}

/// Returns a platform that may switch among `allowed_modes`. It starts connected to the display
/// server `graphical_environment` names, or headless when that's `None` or names none.
///
/// # Panics
///
/// Panics if `allowed_modes` doesn't allow the starting mode, or if the display server can't be
/// reached.
pub fn linux_platform(
    allowed_modes: DisplayModes,
    graphical_environment: Option<GraphicalEnvironment>,
) -> Rc<dyn gpui::Platform> {
    Rc::new(LinuxPlatform::new(allowed_modes, graphical_environment))
}
