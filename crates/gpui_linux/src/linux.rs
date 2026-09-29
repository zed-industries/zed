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
pub use display_connection::LinuxDisplayModes;
pub(crate) use display_connection::{Backend, DisplayConnection};
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

use gpui::{DisplayEnvironment, DisplayMode};

/// Returns the default platform implementation for the current OS.
///
/// A windowed platform connects to the display server the process environment names, or starts
/// headless when it names none, and can switch between those modes later. A headless platform
/// stays headless.
pub fn current_platform(headless: bool) -> Rc<dyn gpui::Platform> {
    if headless || std::env::var_os("ZED_HEADLESS").is_some() {
        linux_platform(LinuxDisplayModes::HEADLESS, DisplayMode::Headless)
    } else {
        linux_platform(
            LinuxDisplayModes::all(),
            DisplayMode::Windowed(DisplayEnvironment::from_process_environment()),
        )
    }
}

/// Returns a platform that starts in `initial` mode and may switch among `modes`.
///
/// # Panics
///
/// Panics if `modes` doesn't allow `initial`, or if the display server `initial` selects can't be
/// reached.
pub fn linux_platform(modes: LinuxDisplayModes, initial: DisplayMode) -> Rc<dyn gpui::Platform> {
    Rc::new(LinuxPlatform::new(modes, initial))
}
