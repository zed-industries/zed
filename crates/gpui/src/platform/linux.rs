mod dispatcher;
mod headless;
mod keyboard;
mod platform;
mod text_system;

#[cfg(feature = "wayland")]
mod wayland;
#[cfg(feature = "x11")]
mod x11;
#[cfg(any(feature = "wayland", feature = "x11"))]
mod xdg_desktop_portal;

pub(crate) use dispatcher::*;
pub(crate) use headless::*;
pub(crate) use keyboard::*;
pub(crate) use platform::*;
pub(crate) use text_system::*;
#[cfg(feature = "wayland")]
pub(crate) use wayland::*;
#[cfg(feature = "x11")]
pub(crate) use x11::*;

#[cfg(feature = "screen-capture")]
pub(crate) type PlatformScreenCaptureFrame = scap::frame::Frame;
#[cfg(not(feature = "screen-capture"))]
pub(crate) type PlatformScreenCaptureFrame = ();

#[cfg(feature = "wayland")]
pub use wayland::layer_shell;
