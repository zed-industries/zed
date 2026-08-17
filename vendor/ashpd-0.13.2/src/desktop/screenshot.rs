//! Take a screenshot or pick a color.
//!
//! Wrapper of the DBus interface: [`org.freedesktop.portal.Screenshot`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Screenshot.html).
//!
//! # Examples
//!
//! ## Taking a screenshot
//!
//! ```rust,no_run
//! use ashpd::desktop::screenshot::Screenshot;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let response = Screenshot::request()
//!         .interactive(true)
//!         .modal(true)
//!         .send()
//!         .await?
//!         .response()?;
//!     println!("URI: {}", response.uri());
//!     Ok(())
//! }
//! ```
//!
//! ## Picking a color
//!
//! ```rust,no_run
//! use ashpd::desktop::Color;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let color = Color::pick().send().await?.response()?;
//!     println!("({}, {}, {})", color.red(), color.green(), color.blue());
//!
//!     Ok(())
//! }
//! ```
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use zbus::zvariant::{
    Optional, Type,
    as_value::{self, optional},
};

use super::{HandleToken, Request};
use crate::{Error, Uri, WindowIdentifier, desktop::Color, proxy::Proxy};

/// Options for taking a screenshot.
#[derive(Serialize, Deserialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct ScreenshotOptions {
    #[serde(with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    modal: Option<bool>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    interactive: Option<bool>,
    #[serde(default, with = "optional", skip_serializing)]
    #[cfg_attr(not(feature = "backend"), allow(dead_code))]
    permission_store_checked: Option<bool>,
}

impl ScreenshotOptions {
    /// Sets whether the dialog should be modal.
    #[must_use]
    pub fn set_modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.modal = modal.into();
        self
    }

    /// Gets whether the dialog should be modal.
    #[cfg(feature = "backend")]
    pub fn modal(&self) -> Option<bool> {
        self.modal
    }

    /// Sets whether the dialog should offer customization.
    #[must_use]
    pub fn set_interactive(mut self, interactive: impl Into<Option<bool>>) -> Self {
        self.interactive = interactive.into();
        self
    }

    /// Gets whether the dialog should offer customization.
    #[cfg(feature = "backend")]
    pub fn interactive(&self) -> Option<bool> {
        self.interactive
    }

    /// Gets whether the permission store has been checked.
    #[cfg(feature = "backend")]
    pub fn permission_store_checked(&self) -> Option<bool> {
        self.permission_store_checked
    }
}

#[derive(Serialize, Deserialize, Type)]
#[zvariant(signature = "dict")]
/// The response of a [`ScreenshotRequest`] request.
pub struct Screenshot {
    #[serde(with = "as_value")]
    uri: Uri,
}

impl Screenshot {
    #[cfg(feature = "backend")]
    #[cfg_attr(docsrs, doc(cfg(feature = "backend")))]
    /// Create a new instance of the screenshot.
    pub fn new(uri: Uri) -> Self {
        Self { uri }
    }

    /// Creates a new builder-pattern struct instance to construct
    /// [`Screenshot`].
    ///
    /// This method returns an instance of [`ScreenshotRequest`].
    pub fn request() -> ScreenshotRequest {
        ScreenshotRequest::default()
    }

    /// The screenshot URI.
    pub fn uri(&self) -> &Uri {
        &self.uri
    }
}

impl Debug for Screenshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.uri.as_str())
    }
}

/// Options for picking a color.
#[derive(Serialize, Deserialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct ColorOptions {
    #[serde(with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
}

/// Wrapper of the DBus interface: [`org.freedesktop.portal.Screenshot`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Screenshot.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Screenshot")]
pub struct ScreenshotProxy(Proxy<'static>);

impl ScreenshotProxy {
    /// Create a new instance of [`ScreenshotProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Screenshot").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`ScreenshotProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Screenshot")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the portal interface version.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Obtains the color of a single pixel.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier for the application window.
    ///
    /// # Specifications
    ///
    /// See also [`PickColor`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Screenshot.html#org-freedesktop-portal-screenshot-pickcolor).
    #[doc(alias = "PickColor")]
    #[doc(alias = "xdp_portal_pick_color")]
    pub async fn pick_color(
        &self,
        identifier: Option<&WindowIdentifier>,
        options: ColorOptions,
    ) -> Result<Request<Color>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .request(&options.handle_token, "PickColor", &(identifier, &options))
            .await
    }

    /// Takes a screenshot.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier for the application window.
    /// * `interactive` - Sets whether the dialog should offer customization
    ///   before a screenshot or not.
    /// * `modal` - Sets whether the dialog should be a modal.
    ///
    /// # Returns
    ///
    /// The screenshot URI.
    ///
    /// # Specifications
    ///
    /// See also [`Screenshot`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Screenshot.html#org-freedesktop-portal-screenshot-screenshot).
    #[doc(alias = "Screenshot")]
    #[doc(alias = "xdp_portal_take_screenshot")]
    pub async fn screenshot(
        &self,
        identifier: Option<&WindowIdentifier>,
        options: ScreenshotOptions,
    ) -> Result<Request<Screenshot>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .request(&options.handle_token, "Screenshot", &(identifier, &options))
            .await
    }
}

impl std::ops::Deref for ScreenshotProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
#[doc(alias = "xdp_portal_pick_color")]
/// A [builder-pattern] type to construct [`Color`].
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct ColorRequest {
    identifier: Option<WindowIdentifier>,
    options: ColorOptions,
    connection: Option<zbus::Connection>,
}

impl ColorRequest {
    #[must_use]
    /// Sets a window identifier.
    pub fn identifier(mut self, identifier: impl Into<Option<WindowIdentifier>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    #[must_use]
    /// Sets a connection to use other than the internal one.
    pub fn connection(mut self, connection: Option<zbus::Connection>) -> Self {
        self.connection = connection;
        self
    }

    /// Build the [`Color`].
    pub async fn send(self) -> Result<Request<Color>, Error> {
        let proxy = if let Some(connection) = self.connection {
            ScreenshotProxy::with_connection(connection).await?
        } else {
            ScreenshotProxy::new().await?
        };
        proxy
            .pick_color(self.identifier.as_ref(), self.options)
            .await
    }
}

impl Color {
    /// Creates a new builder-pattern struct instance to construct
    /// [`Color`].
    ///
    /// This method returns an instance of [`ColorRequest`].
    pub fn pick() -> ColorRequest {
        ColorRequest::default()
    }
}

#[derive(Debug, Default)]
#[doc(alias = "xdp_portal_take_screenshot")]
/// A [builder-pattern] type to construct a screenshot [`Screenshot`].
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct ScreenshotRequest {
    options: ScreenshotOptions,
    identifier: Option<WindowIdentifier>,
    connection: Option<zbus::Connection>,
}

impl ScreenshotRequest {
    #[must_use]
    /// Sets a window identifier.
    pub fn identifier(mut self, identifier: impl Into<Option<WindowIdentifier>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    /// Sets whether the dialog should be a modal.
    #[must_use]
    pub fn modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.options.modal = modal.into();
        self
    }

    /// Sets whether the dialog should offer customization before a screenshot
    /// or not.
    #[must_use]
    pub fn interactive(mut self, interactive: impl Into<Option<bool>>) -> Self {
        self.options.interactive = interactive.into();
        self
    }

    #[must_use]
    /// Sets a connection to use other than the internal one.
    pub fn connection(mut self, connection: Option<zbus::Connection>) -> Self {
        self.connection = connection;
        self
    }

    /// Build the [`Screenshot`].
    pub async fn send(self) -> Result<Request<Screenshot>, Error> {
        let proxy = if let Some(connection) = self.connection {
            ScreenshotProxy::with_connection(connection).await?
        } else {
            ScreenshotProxy::new().await?
        };
        proxy
            .screenshot(self.identifier.as_ref(), self.options)
            .await
    }
}
