use std::{path::PathBuf, rc::Rc};

use gpui::{
    AnyWindowHandle, ClipboardItem, CursorStyle, DisplayModes, GraphicalEnvironment,
    PlatformDisplay, PlatformKeyboardLayout, PlatformWindow, WindowParams,
};

#[cfg(feature = "wayland")]
use super::WaylandConnection;
#[cfg(feature = "x11")]
use super::X11Connection;
use super::{HeadlessConnection, LinuxKeyboardLayout};

/// Returns the display server `environment` selects among the windowed modes in `modes`:
/// Wayland, then X11.
#[cfg_attr(
    not(any(feature = "wayland", feature = "x11")),
    allow(
        unused_variables,
        reason = "no windowed modes without a display backend"
    )
)]
pub(crate) fn select_backend(
    modes: DisplayModes,
    environment: &GraphicalEnvironment,
) -> Option<Backend> {
    let is_set =
        |value: &Option<std::ffi::OsString>| value.as_ref().is_some_and(|value| !value.is_empty());
    #[cfg(feature = "wayland")]
    if modes.contains(DisplayModes::WAYLAND) && is_set(&environment.wayland_display) {
        return Some(Backend::Wayland);
    }
    #[cfg(feature = "x11")]
    if modes.contains(DisplayModes::X11) && is_set(&environment.x11_display) {
        return Some(Backend::X11);
    }
    None
}

/// A display server that [`DisplayConnection`] can connect to.
#[derive(Clone, Copy)]
pub(crate) enum Backend {
    #[cfg(feature = "wayland")]
    Wayland,
    #[cfg(feature = "x11")]
    X11,
}

/// The display server a `LinuxPlatform` is connected to.
///
/// Every connection follows the same lifecycle, so that switching display modes acquires and
/// releases the same things at the same points:
///
/// - **Attach** (`attach` on the Wayland or X11 connection): connect, create the connection's
///   state, then register its event sources on the platform's loop. If a step fails, dropping
///   the partial connection undoes the earlier ones.
/// - **Detach** (drop): the platform refuses to switch while a window is open. A closed window
///   has already released everything bound to the connection, including its GPU objects, so
///   nothing waits. The platform takes the connection out of its `RefCell` before dropping it,
///   because dropping can call back into the platform. Dropping removes the event sources, stops
///   the desktop portal listener, hands the clipboard to a clipboard manager (X11, on a
///   background thread) and closes the connection.
///
/// Nothing tied to a display server outlives its connection. What does, in `LinuxCommon`, is
/// session-independent: the executors, the text system, callbacks and the power listener.
pub(crate) enum DisplayConnection {
    Headless(HeadlessConnection),
    #[cfg(feature = "wayland")]
    Wayland(WaylandConnection),
    #[cfg(feature = "x11")]
    X11(X11Connection),
}

/// Evaluates `$connected` with `$connection` bound to the Wayland or X11 connection, or
/// `$headless` while headless, optionally binding the headless state.
macro_rules! dispatch {
    ($self:expr, $connection:ident => $connected:expr, headless => $headless:expr) => {
        dispatch!($self, $connection => $connected, headless(_) => $headless)
    };
    ($self:expr, $connection:ident => $connected:expr, headless($state:pat) => $headless:expr) => {
        match $self {
            DisplayConnection::Headless($state) => $headless,
            #[cfg(feature = "wayland")]
            DisplayConnection::Wayland($connection) => $connected,
            #[cfg(feature = "x11")]
            DisplayConnection::X11($connection) => $connected,
        }
    };
}

#[cfg_attr(
    not(any(feature = "wayland", feature = "x11")),
    allow(
        unused_variables,
        reason = "only headless mode exists without a display backend"
    )
)]
impl DisplayConnection {
    pub(crate) fn is_headless(&self) -> bool {
        matches!(self, DisplayConnection::Headless(_))
    }

    pub(crate) fn compositor_name(&self) -> &'static str {
        dispatch!(self, connection => connection.compositor_name(), headless => "headless")
    }

    /// Whether a window is open.
    pub(crate) fn has_windows(&self) -> bool {
        dispatch!(self, connection => connection.has_windows(), headless(connection) => connection.has_windows())
    }

    /// The display variables for programs launched while connected, or `None` while headless.
    #[cfg(any(feature = "wayland", feature = "x11"))]
    pub(crate) fn launch_environment(&self) -> Option<super::LaunchEnvironment> {
        dispatch!(self, connection => Some(connection.launch_environment()), headless => None)
    }

    pub(crate) fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        dispatch!(
            self,
            connection => connection.keyboard_layout(),
            headless => Box::new(LinuxKeyboardLayout::new("unknown".into()))
        )
    }

    pub(crate) fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        dispatch!(self, connection => connection.displays(), headless(connection) => connection.displays())
    }

    pub(crate) fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        dispatch!(
            self,
            connection => connection.primary_display(),
            headless(connection) => connection.primary_display()
        )
    }

    pub(crate) fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        dispatch!(
            self,
            connection => connection.open_window(handle, params),
            headless(connection) => connection.open_window(handle, params)
        )
    }

    pub(crate) fn active_window(&self) -> Option<AnyWindowHandle> {
        dispatch!(self, connection => connection.active_window(), headless => None)
    }

    pub(crate) fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        dispatch!(self, connection => connection.window_stack(), headless => None)
    }

    pub(crate) fn set_cursor_style(&self, style: CursorStyle) {
        dispatch!(self, connection => connection.set_cursor_style(style), headless => ())
    }

    pub(crate) fn hide_cursor_until_mouse_moves(&self) {
        dispatch!(self, connection => connection.hide_cursor_until_mouse_moves(), headless => ())
    }

    pub(crate) fn is_cursor_visible(&self) -> bool {
        dispatch!(self, connection => connection.is_cursor_visible(), headless => true)
    }

    pub(crate) fn open_uri(&self, uri: &str) {
        dispatch!(self, connection => connection.open_uri(uri), headless => ())
    }

    pub(crate) fn reveal_path(&self, path: PathBuf) {
        dispatch!(self, connection => connection.reveal_path(path), headless => ())
    }

    pub(crate) fn write_to_primary(&self, item: ClipboardItem) {
        dispatch!(self, connection => connection.write_to_primary(item), headless => ())
    }

    pub(crate) fn write_to_clipboard(&self, item: ClipboardItem) {
        dispatch!(self, connection => connection.write_to_clipboard(item), headless => ())
    }

    pub(crate) fn read_from_primary(&self) -> Option<ClipboardItem> {
        dispatch!(self, connection => connection.read_from_primary(), headless => None)
    }

    pub(crate) fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        dispatch!(self, connection => connection.read_from_clipboard(), headless => None)
    }

    #[cfg(any(feature = "wayland", feature = "x11"))]
    pub(crate) fn window_identifier(
        &self,
    ) -> futures::future::BoxFuture<'static, Option<ashpd::WindowIdentifier>> {
        dispatch!(
            self,
            connection => Box::pin(connection.window_identifier()),
            headless => Box::pin(std::future::ready(None))
        )
    }

    #[cfg(feature = "screen-capture")]
    pub(crate) fn is_screen_capture_supported(&self) -> bool {
        dispatch!(self, connection => connection.is_screen_capture_supported(), headless => false)
    }

    #[cfg(feature = "screen-capture")]
    pub(crate) fn screen_capture_sources(
        &self,
    ) -> futures::channel::oneshot::Receiver<anyhow::Result<Vec<Rc<dyn gpui::ScreenCaptureSource>>>>
    {
        dispatch!(self, connection => connection.screen_capture_sources(), headless => {
            let (sender, receiver) = futures::channel::oneshot::channel();
            sender
                .send(Err(anyhow::anyhow!("Headless mode does not support screen capture.")))
                .ok();
            receiver
        })
    }
}
