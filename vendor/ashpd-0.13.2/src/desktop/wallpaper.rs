//! Set a wallpaper on lockscreen, background or both.
//!
//! Wrapper of the DBus interface: [`org.freedesktop.portal.Wallpaper`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Wallpaper.html).
//!
//! # Examples
//!
//! ## Sets a wallpaper from a file:
//!
//! ```rust,no_run
//! use std::{fs::File, os::fd::AsFd};
//!
//! use ashpd::desktop::wallpaper::{SetOn, WallpaperRequest};
//!
//! async fn run() -> ashpd::Result<()> {
//!     let file = File::open("/home/bilelmoussaoui/adwaita-day.jpg").unwrap();
//!     WallpaperRequest::default()
//!         .set_on(SetOn::Both)
//!         .show_preview(true)
//!         .build_file(&file.as_fd())
//!         .await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Sets a wallpaper from a URI:
//!
//! ```rust,no_run
//! use ashpd::desktop::wallpaper::{SetOn, WallpaperRequest};
//!
//! async fn run() -> ashpd::Result<()> {
//!     let uri =
//!         ashpd::Uri::parse("file:///home/bilelmoussaoui/Downloads/adwaita-night.jpg").unwrap();
//!     WallpaperRequest::default()
//!         .set_on(SetOn::Both)
//!         .show_preview(true)
//!         .build_uri(&uri)
//!         .await?;
//!     Ok(())
//! }
//! ```

use std::{fmt, os::fd::AsFd, str::FromStr};

use serde::{self, Deserialize, Serialize};
use zbus::zvariant::{
    Fd, Optional, Type,
    as_value::{self, optional},
};

use super::Request;
use crate::{Error, Uri, WindowIdentifier, desktop::HandleToken, proxy::Proxy};

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdSetOn"))]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "lowercase")]
/// Where to set the wallpaper on.
pub enum SetOn {
    /// Set the wallpaper only on the lock-screen.
    Lockscreen,
    /// Set the wallpaper only on the background.
    Background,
    /// Set the wallpaper on both lock-screen and background.
    Both,
}

impl fmt::Display for SetOn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lockscreen => write!(f, "Lockscreen"),
            Self::Background => write!(f, "Background"),
            Self::Both => write!(f, "Both"),
        }
    }
}

impl AsRef<str> for SetOn {
    fn as_ref(&self) -> &str {
        match self {
            Self::Lockscreen => "Lockscreen",
            Self::Background => "Background",
            Self::Both => "Both",
        }
    }
}

impl From<SetOn> for &'static str {
    fn from(s: SetOn) -> Self {
        match s {
            SetOn::Lockscreen => "Lockscreen",
            SetOn::Background => "Background",
            SetOn::Both => "Both",
        }
    }
}

impl FromStr for SetOn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lockscreen" => Ok(SetOn::Lockscreen),
            "Background" => Ok(SetOn::Background),
            "Both" => Ok(SetOn::Both),
            _ => Err(Error::ParseError("Failed to parse SetOn, invalid value")),
        }
    }
}

/// Options for setting a wallpaper.
#[derive(Serialize, Deserialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
#[serde(rename_all = "kebab-case")]
pub struct WallpaperOptions {
    #[serde(rename = "handle_token", with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    show_preview: Option<bool>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    set_on: Option<SetOn>,
}

impl WallpaperOptions {
    /// Sets whether to show a preview of the wallpaper.
    #[must_use]
    pub fn set_show_preview(mut self, show_preview: impl Into<Option<bool>>) -> Self {
        self.show_preview = show_preview.into();
        self
    }

    /// Gets whether to show a preview of the wallpaper.
    #[cfg(feature = "backend")]
    pub fn show_preview(&self) -> Option<bool> {
        self.show_preview
    }

    /// Sets where to set the wallpaper.
    #[must_use]
    pub fn set_set_on(mut self, set_on: impl Into<Option<SetOn>>) -> Self {
        self.set_on = set_on.into();
        self
    }

    /// Gets where to set the wallpaper.
    #[cfg(feature = "backend")]
    pub fn set_on(&self) -> Option<SetOn> {
        self.set_on
    }
}

/// Wrapper of the DBus interface: [`org.freedesktop.portal.Wallpaper`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Wallpaper.html).
#[doc(alias = "org.freedesktop.portal.Wallpaper")]
pub struct WallpaperProxy(Proxy<'static>);

impl WallpaperProxy {
    /// Create a new instance of [`WallpaperProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Wallpaper").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`WallpaperProxy`] with a specific connection.
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Wallpaper")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the portal interface version.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Sets the wallpaper from a file.
    #[doc(alias = "SetWallpaperFile")]
    pub async fn set_wallpaper_file(
        &self,
        identifier: Option<&WindowIdentifier>,
        file: &impl AsFd,
        options: WallpaperOptions,
    ) -> Result<Request<()>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .empty_request(
                &options.handle_token,
                "SetWallpaperFile",
                &(identifier, Fd::from(file), &options),
            )
            .await
    }

    /// Sets the wallpaper from a URI.
    #[doc(alias = "SetWallpaperURI")]
    pub async fn set_wallpaper_uri(
        &self,
        identifier: Option<&WindowIdentifier>,
        uri: &Uri,
        options: WallpaperOptions,
    ) -> Result<Request<()>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .empty_request(
                &options.handle_token,
                "SetWallpaperURI",
                &(identifier, uri, &options),
            )
            .await
    }
}

impl std::ops::Deref for WallpaperProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
#[doc(alias = "xdp_portal_set_wallpaper")]
#[doc(alias = "org.freedesktop.portal.Wallpaper")]
/// A [builder-pattern] type to set the wallpaper.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct WallpaperRequest {
    identifier: Option<WindowIdentifier>,
    options: WallpaperOptions,
    connection: Option<zbus::Connection>,
}

impl WallpaperRequest {
    #[must_use]
    /// Sets a window identifier.
    pub fn identifier(mut self, identifier: impl Into<Option<WindowIdentifier>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    /// Whether to show a preview of the picture.
    /// **Note** the portal may decide to show a preview even if this option is
    /// not set.
    #[must_use]
    pub fn show_preview(mut self, show_preview: impl Into<Option<bool>>) -> Self {
        self.options.show_preview = show_preview.into();
        self
    }

    /// Sets where to set the wallpaper on.
    #[must_use]
    pub fn set_on(mut self, set_on: impl Into<Option<SetOn>>) -> Self {
        self.options.set_on = set_on.into();
        self
    }

    #[must_use]
    /// Sets a connection to use other than the internal one.
    pub fn connection(mut self, connection: Option<zbus::Connection>) -> Self {
        self.connection = connection;
        self
    }

    /// Build using a URI.
    pub async fn build_uri(self, uri: &Uri) -> Result<Request<()>, Error> {
        let proxy = if let Some(connection) = self.connection {
            WallpaperProxy::with_connection(connection).await?
        } else {
            WallpaperProxy::new().await?
        };
        proxy
            .set_wallpaper_uri(self.identifier.as_ref(), uri, self.options)
            .await
    }

    /// Build using a file.
    pub async fn build_file(self, file: &impl AsFd) -> Result<Request<()>, Error> {
        let proxy = if let Some(connection) = self.connection {
            WallpaperProxy::with_connection(connection).await?
        } else {
            WallpaperProxy::new().await?
        };
        proxy
            .set_wallpaper_file(self.identifier.as_ref(), file, self.options)
            .await
    }
}
#[cfg(test)]
mod tests {
    use super::SetOn;

    #[test]
    fn serialize_deserialize() {
        let set_on = SetOn::Both;
        let string = serde_json::to_string(&set_on).unwrap();
        assert_eq!(string, "\"both\"");

        let decoded = serde_json::from_str(&string).unwrap();
        assert_eq!(set_on, decoded);
    }
}
