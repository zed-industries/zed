//! **Note** This portal doesn't work for sandboxed applications.
//! # Examples
//!
//! ```rust,no_run
//! use ashpd::desktop::network_monitor::NetworkMonitor;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = NetworkMonitor::new().await?;
//!
//!     println!("{}", proxy.can_reach("www.google.com", 80).await?);
//!     println!("{}", proxy.is_available().await?);
//!     println!("{:#?}", proxy.connectivity().await?);
//!     println!("{}", proxy.is_metered().await?);
//!     println!("{:#?}", proxy.status().await?);
//!
//!     Ok(())
//! }
//! ```

use std::fmt;

use futures_util::Stream;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use zbus::zvariant::{Type, as_value};

use crate::{Error, proxy::Proxy};

#[derive(Deserialize, Type, Debug)]
/// The network status, composed of the availability, metered & connectivity
#[zvariant(signature = "dict")]
pub struct NetworkStatus {
    /// Whether the network is considered available.
    #[serde(with = "as_value")]
    available: bool,
    /// Whether the network is considered metered.
    #[serde(with = "as_value")]
    metered: bool,
    /// More detailed information about the host's network connectivity
    #[serde(with = "as_value")]
    connectivity: Connectivity,
}

impl NetworkStatus {
    /// Returns whether the network is considered available.
    pub fn is_available(&self) -> bool {
        self.available
    }

    /// Returns whether the network is considered metered.
    pub fn is_metered(&self) -> bool {
        self.metered
    }

    /// Returns more detailed information about the host's network connectivity.
    pub fn connectivity(&self) -> Connectivity {
        self.connectivity
    }
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdConnectivity"))]
#[derive(Deserialize_repr, PartialEq, Eq, Debug, Clone, Copy, Type)]
#[repr(u32)]
/// Host's network activity
pub enum Connectivity {
    /// The host is not configured with a route to the internet.
    Local = 1,
    /// The host is connected to a network, but can't reach the full internet.
    Limited = 2,
    /// The host is behind a captive portal and cannot reach the full internet.
    CaptivePortal = 3,
    /// The host connected to a network, and can reach the full internet.
    FullNetwork = 4,
}

impl fmt::Display for Connectivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let connectivity = match self {
            Self::Local => "local",
            Self::Limited => "limited",
            Self::CaptivePortal => "captive portal",
            Self::FullNetwork => "full network",
        };
        f.write_str(connectivity)
    }
}

/// The interface provides network status information to sandboxed applications.
///
/// It is not a portal in the strict sense, since it does not involve user
/// interaction. Applications are expected to use this interface indirectly,
/// via a library API such as the GLib [`gio::NetworkMonitor`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.NetworkMonitor.html) interface.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.NetworkMonitor`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.NetworkMonitor.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.NetworkMonitor")]
pub struct NetworkMonitor(Proxy<'static>);

impl NetworkMonitor {
    /// Create a new instance of [`NetworkMonitor`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.NetworkMonitor").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`NetworkMonitor`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.NetworkMonitor")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Returns whether the given hostname is believed to be reachable.
    ///
    /// # Arguments
    ///
    /// * `hostname` - The hostname to reach.
    /// * `port` - The port to reach.
    ///
    /// # Required version
    ///
    /// The method requires the 3nd version implementation of the portal and
    /// would fail with [`Error::RequiresVersion`] otherwise.
    ///
    /// # Specifications
    ///
    /// See also [`CanReach`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.NetworkMonitor.html#org-freedesktop-portal-networkmonitor-canreach).
    #[doc(alias = "CanReach")]
    pub async fn can_reach(&self, hostname: &str, port: u32) -> Result<bool, Error> {
        self.0
            .call_versioned("CanReach", &(hostname, port), 3)
            .await
    }

    /// Returns whether the network is considered available.
    /// That is, whether the system as a default route for at least one of IPv4
    /// or IPv6.
    ///
    /// # Required version
    ///
    /// The method requires the 2nd version implementation of the portal and
    /// would fail with [`Error::RequiresVersion`] otherwise.
    ///
    /// # Specifications
    ///
    /// See also [`GetAvailable`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.NetworkMonitor.html#org-freedesktop-portal-networkmonitor-getavailable).
    #[doc(alias = "GetAvailable")]
    #[doc(alias = "get_available")]
    pub async fn is_available(&self) -> Result<bool, Error> {
        self.0.call_versioned("GetAvailable", &(), 2).await
    }

    /// Returns more detailed information about the host's network connectivity.
    ///
    /// # Required version
    ///
    /// The method requires the 2nd version implementation of the portal and
    /// would fail with [`Error::RequiresVersion`] otherwise.
    ///
    /// # Specifications
    ///
    /// See also [`GetConnectivity`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.NetworkMonitor.html#org-freedesktop-portal-networkmonitor-getconnectivity).
    #[doc(alias = "GetConnectivity")]
    #[doc(alias = "get_connectivity")]
    pub async fn connectivity(&self) -> Result<Connectivity, Error> {
        self.0.call_versioned("GetConnectivity", &(), 2).await
    }

    /// Returns whether the network is considered metered.
    /// That is, whether the system as traffic flowing through the default
    /// connection that is subject to limitations by service providers.
    ///
    /// # Required version
    ///
    /// The method requires the 2nd version implementation of the portal and
    /// would fail with [`Error::RequiresVersion`] otherwise.
    ///
    /// # Specifications
    ///
    /// See also [`GetMetered`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.NetworkMonitor.html#org-freedesktop-portal-networkmonitor-getmetered).
    #[doc(alias = "GetMetered")]
    #[doc(alias = "get_metered")]
    pub async fn is_metered(&self) -> Result<bool, Error> {
        self.0.call_versioned("GetMetered", &(), 2).await
    }

    /// Returns the three values all at once.
    ///
    /// # Required version
    ///
    /// The method requires the 3nd version implementation of the portal and
    /// would fail with [`Error::RequiresVersion`] otherwise.
    ///
    /// # Specifications
    ///
    /// See also [`GetStatus`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.NetworkMonitor.html#org-freedesktop-portal-networkmonitor-getstatus).
    #[doc(alias = "GetStatus")]
    #[doc(alias = "get_status")]
    pub async fn status(&self) -> Result<NetworkStatus, Error> {
        self.0.call_versioned("GetStatus", &(), 3).await
    }

    /// Emitted when the network configuration changes.
    ///
    /// # Specifications
    ///
    /// See also [`changed`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.NetworkMonitor.html#org-freedesktop-portal-networkmonitor-changed).
    #[doc(alias = "changed")]
    pub async fn receive_changed(&self) -> Result<impl Stream<Item = ()>, Error> {
        self.0.signal("changed").await
    }
}

impl std::ops::Deref for NetworkMonitor {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
