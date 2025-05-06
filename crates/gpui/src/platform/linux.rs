mod dispatcher;
mod headless;
#[cfg(any(feature = "wayland", feature = "x11"))]
mod keyboard;
mod platform;
#[cfg(any(feature = "wayland", feature = "x11"))]
mod text_system;
#[cfg(feature = "wayland")]
mod wayland;
#[cfg(feature = "x11")]
mod x11;

#[cfg(any(feature = "wayland", feature = "x11"))]
mod xdg_desktop_portal;

pub(crate) use dispatcher::*;
pub(crate) use headless::*;
#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) use keyboard::*;
pub(crate) use platform::*;
#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) use text_system::*;
#[cfg(feature = "wayland")]
pub(crate) use wayland::*;
#[cfg(feature = "x11")]
pub(crate) use x11::*;

#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) type PlatformScreenCaptureFrame = scap::frame::Frame;
#[cfg(not(any(feature = "wayland", feature = "x11")))]
pub(crate) type PlatformScreenCaptureFrame = ();
