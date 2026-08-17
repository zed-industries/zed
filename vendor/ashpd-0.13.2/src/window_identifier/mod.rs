use std::{fmt, str::FromStr};

#[cfg(all(
    any(feature = "gtk4_wayland", feature = "gtk4_x11"),
    feature = "backend"
))]
use ::gtk4 as gtk;
#[cfg(all(feature = "raw_handle", feature = "gtk4"))]
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
#[cfg(feature = "raw_handle")]
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use serde::{Deserialize, Serialize, ser::Serializer};
use zbus::zvariant::Type;
/// Most portals interact with the user by showing dialogs.
///
/// These dialogs should generally be placed on top of the application window
/// that triggered them. To arrange this, the compositor needs to know about the
/// application window. Many portal requests expect a [`WindowIdentifier`] for
/// this reason.
///
/// Under X11, the [`WindowIdentifier`] should have the form `x11:XID`, where
/// XID is the XID of the application window in hexadecimal. Under Wayland, it
/// should have the form `wayland:HANDLE`, where HANDLE is a surface handle
/// obtained with the [xdg-foreign](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/unstable/xdg-foreign/xdg-foreign-unstable-v2.xml) protocol.
///
/// See also [Parent window identifiers](https://flatpak.github.io/xdg-desktop-portal/docs/window-identifiers.html).
///
/// # Usage
///
/// ## From an X11 XID
///
/// ```rust,ignore
/// let identifier = WindowIdentifier::from_xid(212321);
///
/// /// Open some portals
/// ```
///
/// ## From a Wayland Surface
///
/// The `wayland` feature must be enabled. The exported surface handle will be
/// unexported on `Drop`.
///
/// ```text
/// // let wl_surface = some_surface;
/// // let identifier = WindowIdentifier::from_wayland(wl_surface).await;
///
/// /// Open some portals
/// ```
///
/// Or using a raw `wl_surface` pointer
///
/// ```text
/// // let wl_surface_ptr = some_surface;
/// // let wl_display_ptr = corresponding_display;
/// // let identifier = WindowIdentifier::from_wayland_raw(wl_surface_ptr, wl_display_ptr).await;
///
/// /// Open some portals
/// ```
///
/// ## With GTK 4
///
/// The feature `gtk4` must be enabled. You can get a
/// [`WindowIdentifier`] from a [`IsA<gtk4::Native>`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Native.html) using `WindowIdentifier::from_native`
///
/// ```rust, ignore
/// let widget = gtk4::Button::new();
///
/// let ctx = glib::MainContext::default();
/// ctx.spawn_async(async move {
///     let identifier = WindowIdentifier::from_native(&widget.native().unwrap()).await;
///
///     /// Open some portals
/// });
/// ```
/// The constructor should return a valid identifier under both X11 and Wayland
/// and fallback to the [`Default`] implementation otherwise.
///
/// ## Other Toolkits
///
/// If you have access to `RawWindowHandle` you can convert it to a
/// [`WindowIdentifier`] with
///
/// ```rust, ignore
/// let handle = RawWindowHandle::Xlib(XlibHandle::empty());
/// let identifier = WindowIdentifier::from_raw_handle(handle, None);
///
/// /// Open some portals
/// ```
#[derive(Type)]
#[zvariant(signature = "s")]
#[doc(alias = "XdpParent")]
#[non_exhaustive]
pub enum WindowIdentifier {
    /// Gtk 4 Window Identifier
    #[cfg(any(feature = "gtk4_wayland", feature = "gtk4_x11"))]
    #[doc(hidden)]
    Gtk4(Gtk4WindowIdentifier),
    #[cfg(feature = "wayland")]
    #[doc(hidden)]
    Wayland(WaylandWindowIdentifier),
    #[doc(hidden)]
    Raw(WindowIdentifierType),
}

impl zbus::zvariant::NoneValue for WindowIdentifier {
    type NoneType = String;

    fn null_value() -> Self::NoneType {
        String::default()
    }
}

impl zbus::zvariant::NoneValue for &WindowIdentifier {
    type NoneType = String;

    fn null_value() -> Self::NoneType {
        String::default()
    }
}
unsafe impl Send for WindowIdentifier {}
unsafe impl Sync for WindowIdentifier {}

impl Serialize for WindowIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for WindowIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(any(feature = "gtk4_wayland", feature = "gtk4_x11"))]
            Self::Gtk4(identifier) => identifier.fmt(f),
            #[cfg(feature = "wayland")]
            Self::Wayland(identifier) => identifier.fmt(f),
            Self::Raw(identifier) => identifier.fmt(f),
        }
    }
}

impl std::fmt::Debug for WindowIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WindowIdentifier")
            .field(&format_args!("{self}"))
            .finish()
    }
}

impl WindowIdentifier {
    #[cfg(any(feature = "gtk4_wayland", feature = "gtk4_x11"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "gtk4_wayland", feature = "gtk4_x11"))))]
    /// Creates a [`WindowIdentifier`] from a [`gtk4::Native`](https://docs.gtk.org/gtk4/class.Native.html).
    ///
    /// The constructor returns a valid handle under both Wayland & x11.
    ///
    /// **Note** the function has to be async as the Wayland handle retrieval
    /// API is async as well.
    #[doc(alias = "xdp_parent_new_gtk")]
    pub async fn from_native(native: &impl ::gtk4::prelude::IsA<::gtk4::Native>) -> Option<Self> {
        Gtk4WindowIdentifier::new(native).await.map(Self::Gtk4)
    }

    #[cfg(feature = "raw_handle")]
    #[cfg_attr(docsrs, doc(cfg(feature = "raw_handle")))]
    /// Create an instance of [`WindowIdentifier`] from a
    /// [`RawWindowHandle`](raw_window_handle::RawWindowHandle).
    ///
    /// The constructor returns a valid handle under both Wayland & X11.
    ///
    /// This method is only async and requires a `RawDisplayHandle` only for
    /// Wayland handles.
    pub async fn from_raw_handle(
        window_handle: &RawWindowHandle,
        display_handle: Option<&RawDisplayHandle>,
    ) -> Option<Self> {
        use raw_window_handle::RawWindowHandle::{Xcb, Xlib};
        #[cfg(feature = "wayland")]
        use raw_window_handle::{
            RawDisplayHandle::Wayland as DisplayHandle, RawWindowHandle::Wayland,
        };
        match (window_handle, display_handle) {
            #[cfg(feature = "wayland")]
            (Wayland(wl_handle), Some(DisplayHandle(wl_display))) => unsafe {
                Self::from_wayland_raw(wl_handle.surface.as_ptr(), wl_display.display.as_ptr())
                    .await
            },
            (Xlib(x_handle), _) => Some(Self::from_xid(x_handle.window)),
            (Xcb(x_handle), _) => Some(Self::from_xid(x_handle.window.get().into())),
            _ => None,
        }
    }

    /// Create an instance of [`WindowIdentifier`] from an X11 window's XID.
    pub fn from_xid(xid: std::os::raw::c_ulong) -> Self {
        Self::Raw(WindowIdentifierType::X11(xid))
    }

    /// Create an instance of [`WindowIdentifier`] from a Wayland exported
    /// handle.
    ///
    /// The developer takes responsibility of keeping the handle around till the
    /// interaction is over before un-exporting the handle.
    pub fn from_xdg_foreign_exported(handle: String) -> Self {
        Self::Raw(WindowIdentifierType::Wayland(handle))
    }

    #[cfg(feature = "wayland")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland")))]
    /// Create an instance of [`WindowIdentifier`] from a Wayland surface.
    ///
    /// # Safety
    ///
    /// Both surface and display pointers have to be valid . You must
    /// ensure the `display_ptr` lives longer than the returned
    /// `WindowIdentifier`.
    pub async unsafe fn from_wayland_raw(
        surface_ptr: *mut std::ffi::c_void,
        display_ptr: *mut std::ffi::c_void,
    ) -> Option<Self> {
        unsafe { WaylandWindowIdentifier::from_raw(surface_ptr, display_ptr).await }
            .map(Self::Wayland)
    }

    #[cfg(feature = "wayland")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wayland")))]
    /// Create an instance of [`WindowIdentifier`] from a Wayland surface.
    pub async fn from_wayland(
        surface: &wayland_client::protocol::wl_surface::WlSurface,
    ) -> Option<Self> {
        WaylandWindowIdentifier::new(surface)
            .await
            .map(Self::Wayland)
    }
}

#[cfg(all(feature = "raw_handle", feature = "gtk4"))]
impl HasDisplayHandle for WindowIdentifier {
    /// Convert a [`WindowIdentifier`] to
    /// [`RawDisplayHandle`](raw_window_handle::RawDisplayHandle`).
    ///
    /// # Panics
    ///
    /// If you attempt to convert a [`WindowIdentifier`] created from a
    /// [`RawDisplayHandle`](raw_window_handle::RawDisplayHandle`) instead of
    /// the gtk4 constructors.
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        match self {
            #[cfg(feature = "gtk4")]
            Self::Gtk4(identifier) => Ok(identifier.as_raw_display_handle()),
            _ => unreachable!(),
        }
    }
}

#[cfg(all(feature = "raw_handle", feature = "gtk4"))]
impl HasWindowHandle for WindowIdentifier {
    /// Convert a [`WindowIdentifier`] to
    /// [`RawWindowHandle`](raw_window_handle::RawWindowHandle`).
    ///
    /// # Panics
    ///
    /// If you attempt to convert a [`WindowIdentifier`] created from a
    /// [`RawWindowHandle`](raw_window_handle::RawWindowHandle`) instead of
    /// the gtk4 constructors.
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match self {
            #[cfg(feature = "gtk4")]
            Self::Gtk4(identifier) => Ok(identifier.as_raw_window_handle()),
            _ => unreachable!(),
        }
    }
}

/// Supported WindowIdentifier kinds
#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[zvariant(signature = "s")]
pub enum WindowIdentifierType {
    /// X11.
    X11(std::os::raw::c_ulong),
    #[allow(dead_code)]
    /// Wayland.
    Wayland(String),
}

impl From<WindowIdentifierType> for WindowIdentifier {
    fn from(value: WindowIdentifierType) -> Self {
        Self::Raw(value)
    }
}

impl zbus::zvariant::NoneValue for WindowIdentifierType {
    type NoneType = String;

    fn null_value() -> String {
        String::default()
    }
}

impl fmt::Display for WindowIdentifierType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X11(xid) => {
                f.write_str("x11:")?;
                write!(f, "{xid:x}")
            }
            Self::Wayland(handle) => {
                f.write_str("wayland:")?;
                f.write_str(handle)
            }
        }
    }
}

impl FromStr for WindowIdentifierType {
    type Err = PortalError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, handle) = s
            .split_once(':')
            .ok_or_else(|| PortalError::InvalidArgument("Invalid Window Identifier".to_owned()))?;
        match kind {
            "x11" => {
                let handle = handle.trim_start_matches("0x");
                Ok(Self::X11(
                    std::os::raw::c_ulong::from_str_radix(handle, 16)
                        .map_err(|_| PortalError::InvalidArgument(format!("Wrong XID {handle}")))?,
                ))
            }
            "wayland" => Ok(Self::Wayland(handle.to_owned())),
            t => Err(PortalError::InvalidArgument(format!(
                "Invalid Window Identifier type {t}",
            ))),
        }
    }
}

impl TryFrom<String> for WindowIdentifierType {
    type Error = PortalError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl<'de> Deserialize<'de> for WindowIdentifierType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let handle = String::deserialize(deserializer)?;
        Self::from_str(&handle)
            .map_err(|e| serde::de::Error::custom(format!("Invalid Window identifier {e}")))
    }
}

impl Serialize for WindowIdentifierType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl WindowIdentifierType {
    /// Sets the given window as a modal child of the window represented by
    /// self.
    ///
    /// The different combinations of window types and their support is as per
    /// follows.
    /// - Wayland child window, Wayland parent window - supported.
    /// - Wayland child window, X11 parent window - unsupported.
    /// - X11 child window, Wayland parent window - unsupported.
    /// - X11 child window, X11 parent window - supported.
    ///
    /// This is useful in backend implementations as the portal dialogs have to
    /// be modal and grouped together with the application that launched the
    /// request.
    ///
    /// Realizes the window if it is not realized yet.
    ///
    /// Returns `true` on success.
    #[cfg(all(
        any(feature = "gtk4_wayland", feature = "gtk4_x11"),
        feature = "backend"
    ))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(
            any(feature = "gtk4_wayland", feature = "gtk4_x11"),
            feature = "backend"
        )))
    )]
    pub fn set_parent_of(&self, window: &impl gtk::prelude::IsA<gtk::Window>) -> bool {
        use gtk::prelude::*;

        let window = window.as_ref();

        let surface = match window.surface() {
            Some(surface) => surface,
            None => {
                WidgetExt::realize(window);
                window.surface().unwrap()
            }
        };

        window.set_modal(true);

        match self {
            #[cfg(feature = "gtk4_x11")]
            WindowIdentifierType::X11(xid) => {
                use gdk4x11::{X11Display, X11Surface, x11::xlib};

                let display = match WidgetExt::display(window).dynamic_cast::<X11Display>() {
                    Ok(display) => display,
                    Err(_) => {
                        #[cfg(feature = "tracing")]
                        tracing::warn!("Failed to get X11 display");
                        return false;
                    }
                };
                let surface = match surface.dynamic_cast::<X11Surface>() {
                    Ok(surface) => surface,
                    Err(_) => {
                        #[cfg(feature = "tracing")]
                        tracing::warn!("Failed to get X11 surface");
                        return false;
                    }
                };
                unsafe {
                    // Based on GNOME's libgxdp -
                    // https://gitlab.gnome.org/GNOME/libgxdp/-/blob/e6c11f2812cad0a43e847ec97bfc1c67bf50be52/src/gxdp-external-window-x11.c#L90-105
                    let xdisplay = display.xdisplay();
                    let xlib_handle = xlib::Xlib::open().unwrap();
                    (xlib_handle.XSetTransientForHint)(xdisplay, surface.xid(), *xid);
                    let net_wm_window_type_atom =
                        gdk4x11::x11_get_xatom_by_name_for_display(&display, "_NET_WM_WINDOW_TYPE");
                    let net_wm_window_type_dialog_atom = gdk4x11::x11_get_xatom_by_name_for_display(
                        &display,
                        "_NET_WM_WINDOW_DIALOG_TYPE",
                    );
                    let data: *const u8 = &(net_wm_window_type_dialog_atom as u8);
                    (xlib_handle.XChangeProperty)(
                        xdisplay,
                        surface.xid(),
                        net_wm_window_type_atom,
                        xlib::XA_ATOM,
                        32,
                        xlib::PropModeReplace,
                        data,
                        1,
                    );
                    true
                }
            }
            #[cfg(feature = "gtk4_wayland")]
            WindowIdentifierType::Wayland(handle) => {
                use gdk4wayland::WaylandToplevel;

                let toplevel = match surface.dynamic_cast::<WaylandToplevel>() {
                    Ok(toplevel) => toplevel,
                    Err(_) => {
                        #[cfg(feature = "tracing")]
                        tracing::warn!("Failed to get toplevel from surface");
                        return false;
                    }
                };
                toplevel.set_transient_for_exported(handle)
            }
            #[cfg(not(all(feature = "gtk4_x11", feature = "gtk4_wayland")))]
            _ => false,
        }
    }
}

#[cfg(any(feature = "gtk4_wayland", feature = "gtk4_x11"))]
mod gtk4;

#[cfg(any(feature = "gtk4_wayland", feature = "gtk4_x11"))]
pub use self::gtk4::Gtk4WindowIdentifier;
use crate::PortalError;

#[cfg(feature = "wayland")]
mod wayland;

#[cfg(feature = "wayland")]
pub use self::wayland::WaylandWindowIdentifier;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::WindowIdentifier;
    use crate::window_identifier::WindowIdentifierType;

    #[test]
    fn test_serialize() {
        let x11 = WindowIdentifier::from_xid(1024);
        assert_eq!(x11.to_string(), "x11:400");

        assert_eq!(
            WindowIdentifierType::from_str("x11:11432").unwrap(),
            WindowIdentifierType::X11(70706)
        );

        // A valid x11 window identifier shouldn't be prefixed with 0x, this is kept for
        // backwards compatibility and compatibility with backends which
        // implicitly strip the prefix with e.g. `strtol`
        assert_eq!(
            WindowIdentifierType::from_str("x11:0x502a").unwrap(),
            WindowIdentifierType::X11(20522)
        );

        assert_eq!(
            WindowIdentifierType::from_str("wayland:Somerandomchars").unwrap(),
            WindowIdentifierType::Wayland("Somerandomchars".to_owned())
        );
        assert!(WindowIdentifierType::from_str("some_handle").is_err());
        assert!(WindowIdentifierType::from_str("some_type:some_handle").is_err());
    }
}
