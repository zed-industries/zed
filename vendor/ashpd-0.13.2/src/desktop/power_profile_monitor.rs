use crate::{Error, proxy::Proxy};

/// The interface provides information about the user-selected system-wide power
/// profile, to sandboxed applications.
///
/// It is not a portal in the strict sense,
/// since it does not involve user interaction.
///
/// Applications are expected to use this interface indirectly, via a library
/// API such as the GLib [`gio::PowerProfileMonitor`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.PowerProfileMonitor.html) interface.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.PowerProfileMonitor`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.PowerProfileMonitor.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.PowerProfileMonitor")]
pub struct PowerProfileMonitor(Proxy<'static>);

impl PowerProfileMonitor {
    /// Create a new instance of [`PowerProfileMonitor`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.PowerProfileMonitor").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`PowerProfileMonitor`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy = Proxy::new_desktop_with_connection(
            connection,
            "org.freedesktop.portal.PowerProfileMonitor",
        )
        .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Whether the power saver is enabled.
    ///
    /// # Specifications
    ///
    /// See also [`power-saver-enabled`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.PowerProfileMonitor.html#org-freedesktop-portal-powerprofilemonitor-power-saver-enabled)
    #[doc(alias = "power-saver-enabled")]
    pub async fn is_enabled(&self) -> Result<bool, Error> {
        self.0.property("power-saver-enabled").await
    }
}

impl std::ops::Deref for PowerProfileMonitor {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
