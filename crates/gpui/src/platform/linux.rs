mod dispatcher;
mod headless;
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
pub(crate) use platform::*;
#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) use text_system::*;
#[cfg(feature = "wayland")]
pub(crate) use wayland::*;
#[cfg(feature = "x11")]
pub(crate) use x11::*;

// TODO(mgsloan): This type won't make sense for frame capture. A `type VideoFrame` with this type
// should be added to `live_kit_client`.
pub(crate) type PlatformScreenCaptureFrame = std::sync::Arc<crate::RenderImage>;
