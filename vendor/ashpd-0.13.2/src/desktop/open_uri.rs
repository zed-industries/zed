//! Open a URI or a directory.
//!
//! Wrapper of the DBus interface: [`org.freedesktop.portal.OpenURI`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.OpenURI.html).
//!
//! # Examples
//!
//! ## Open a file
//!
//! ```rust,no_run
//! use std::{fs::File, os::fd::AsFd};
//!
//! use ashpd::desktop::open_uri::OpenFileRequest;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let file = File::open("/home/bilelmoussaoui/adwaita-day.jpg").unwrap();
//!     OpenFileRequest::default()
//!         .ask(true)
//!         .send_file(&file.as_fd())
//!         .await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Open a file from a URI
//!
//!
//! ```rust,no_run
//! use ashpd::desktop::open_uri::OpenFileRequest;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let uri =
//!         ashpd::Uri::parse("file:///home/bilelmoussaoui/Downloads/adwaita-night.jpg").unwrap();
//!     OpenFileRequest::default().ask(true).send_uri(&uri).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Open a directory
//!
//! ```rust,no_run
//! use std::{fs::File, os::fd::AsFd};
//!
//! use ashpd::desktop::open_uri::OpenDirectoryRequest;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let directory = File::open("/home/bilelmoussaoui/Downloads").unwrap();
//!     OpenDirectoryRequest::default()
//!         .send(&directory.as_fd())
//!         .await?;
//!     Ok(())
//! }
//! ```

use std::os::fd::AsFd;

use serde::Serialize;
use zbus::zvariant::{
    Fd, Optional, Type,
    as_value::{self, optional},
};

use super::{HandleToken, Request};
use crate::{ActivationToken, Error, Uri, WindowIdentifier, proxy::Proxy};

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Options passed to [`OpenURIProxy::open_directory`].
pub struct OpenDirOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
    /// The activation token.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub activation_token: Option<ActivationToken>,
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Options passed to [`OpenURIProxy::open_file`].
pub struct OpenFileOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
    /// Whether the file should be writable.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub writeable: Option<bool>,
    /// Whether to ask the user.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub ask: Option<bool>,
    /// The activation token.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    pub activation_token: Option<ActivationToken>,
}

#[derive(Debug, Serialize, Type, Default)]
#[zvariant(signature = "dict")]
/// Options passed to [`OpenURIProxy::scheme_supported`].
pub struct SchemeSupportedOptions {}

/// The interface lets sandboxed applications open URIs
/// (e.g. a http: link to the applications homepage) under the control of the
/// user.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.OpenURI`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.OpenURI.html).
#[derive(Debug)]
pub struct OpenURIProxy(Proxy<'static>);

impl OpenURIProxy {
    /// Create a new instance of [`OpenURIProxy`].
    pub async fn new() -> Result<OpenURIProxy, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.OpenURI").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`OpenURIProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.OpenURI")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Asks to open the directory containing a local file in the file browser.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier for the application window.
    /// * `directory` - File descriptor for a file.
    /// * `options` - The options to pass to the request.
    ///
    /// # Specifications
    ///
    /// See also [`OpenDirectory`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.OpenURI.html#org-freedesktop-portal-openuri-opendirectory).
    #[doc(alias = "OpenDirectory")]
    pub async fn open_directory(
        &self,
        identifier: Option<&WindowIdentifier>,
        directory: &impl AsFd,
        options: OpenDirOptions,
    ) -> Result<Request<()>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .empty_request(
                &options.handle_token,
                "OpenDirectory",
                &(identifier, Fd::from(directory), &options),
            )
            .await
    }

    /// Asks to open a local file.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier for the application window.
    /// * `file` - File descriptor for the file to open.
    /// * `options` - The options to pass to the request.
    ///
    /// # Specifications
    ///
    /// See also [`OpenFile`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.OpenURI.html#org-freedesktop-portal-openuri-openfile).
    #[doc(alias = "OpenFile")]
    pub async fn open_file(
        &self,
        identifier: Option<&WindowIdentifier>,
        file: &impl AsFd,
        options: OpenFileOptions,
    ) -> Result<Request<()>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .empty_request(
                &options.handle_token,
                "OpenFile",
                &(identifier, Fd::from(file), &options),
            )
            .await
    }

    /// Asks to open a local file.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier for the application window.
    /// * `uri` - The uri to open.
    /// * `options` - The options to pass to the request.
    ///
    /// *Note* that `file` uris are explicitly not supported by this method.
    /// Use [`Self::open_file`] or [`Self::open_directory`] instead.
    ///
    /// # Specifications
    ///
    /// See also [`OpenURI`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.OpenURI.html#org-freedesktop-portal-openuri-openuri).
    #[doc(alias = "OpenURI")]
    pub async fn open_uri(
        &self,
        identifier: Option<&WindowIdentifier>,
        uri: &Uri,
        options: OpenFileOptions,
    ) -> Result<Request<()>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .empty_request(
                &options.handle_token,
                "OpenURI",
                &(identifier, uri, &options),
            )
            .await
    }

    /// Whether a scheme is supported or not.
    ///
    /// # Arguments
    ///
    /// * `scheme` - URI protocol scheme to check.
    /// * `options` - The options to pass to the request.
    ///
    /// # Specifications
    ///
    /// See also [`SchemeSupported`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.OpenURI.html#org-freedesktop-portal-openuri-schemesupported).
    #[doc(alias = "SchemeSupported")]
    pub async fn scheme_supported(
        &self,
        schema: &str,
        options: SchemeSupportedOptions,
    ) -> Result<bool, Error> {
        self.0.call("SchemeSupported", &(schema, &options)).await
    }
}

impl std::ops::Deref for OpenURIProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
#[doc(alias = "org.freedesktop.portal.OpenURI")]
#[doc(alias = "xdp_portal_open_uri")]
/// A [builder-pattern] type to open a file.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct OpenFileRequest {
    identifier: Option<WindowIdentifier>,
    options: OpenFileOptions,
    connection: Option<zbus::Connection>,
}

impl OpenFileRequest {
    #[must_use]
    /// Sets a window identifier.
    pub fn identifier(mut self, identifier: impl Into<Option<WindowIdentifier>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    #[must_use]
    /// Whether the file should be writeable or not.
    pub fn writeable(mut self, writeable: impl Into<Option<bool>>) -> Self {
        self.options.writeable = writeable.into();
        self
    }

    #[must_use]
    /// Whether to always ask the user which application to use or not.
    pub fn ask(mut self, ask: impl Into<Option<bool>>) -> Self {
        self.options.ask = ask.into();
        self
    }

    /// Sets the token that can be used to activate the chosen application.
    #[must_use]
    pub fn activation_token(
        mut self,
        activation_token: impl Into<Option<ActivationToken>>,
    ) -> Self {
        self.options.activation_token = activation_token.into();
        self
    }

    #[must_use]
    /// Sets a connection to use other than the internal one.
    pub fn connection(mut self, connection: Option<zbus::Connection>) -> Self {
        self.connection = connection;
        self
    }

    /// Send the request for a file.
    pub async fn send_file(self, file: &impl AsFd) -> Result<Request<()>, Error> {
        let proxy = if let Some(connection) = self.connection {
            OpenURIProxy::with_connection(connection).await?
        } else {
            OpenURIProxy::new().await?
        };
        proxy
            .open_file(self.identifier.as_ref(), file, self.options)
            .await
    }

    /// Send the request for a URI.
    pub async fn send_uri(self, uri: &Uri) -> Result<Request<()>, Error> {
        let proxy = if let Some(connection) = self.connection {
            OpenURIProxy::with_connection(connection).await?
        } else {
            OpenURIProxy::new().await?
        };
        proxy
            .open_uri(self.identifier.as_ref(), uri, self.options)
            .await
    }
}

#[derive(Debug, Default)]
#[doc(alias = "xdp_portal_open_directory")]
#[doc(alias = "org.freedesktop.portal.OpenURI")]
/// A [builder-pattern] type to open a directory.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct OpenDirectoryRequest {
    identifier: Option<WindowIdentifier>,
    options: OpenDirOptions,
    connection: Option<zbus::Connection>,
}

impl OpenDirectoryRequest {
    #[must_use]
    /// Sets a window identifier.
    pub fn identifier(mut self, identifier: impl Into<Option<WindowIdentifier>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    /// Sets the token that can be used to activate the chosen application.
    #[must_use]
    pub fn activation_token(
        mut self,
        activation_token: impl Into<Option<ActivationToken>>,
    ) -> Self {
        self.options.activation_token = activation_token.into();
        self
    }

    #[must_use]
    /// Sets a connection to use other than the internal one.
    pub fn connection(mut self, connection: Option<zbus::Connection>) -> Self {
        self.connection = connection;
        self
    }

    /// Send the request.
    pub async fn send(self, directory: &impl AsFd) -> Result<Request<()>, Error> {
        let proxy = if let Some(connection) = self.connection {
            OpenURIProxy::with_connection(connection).await?
        } else {
            OpenURIProxy::new().await?
        };
        proxy
            .open_directory(self.identifier.as_ref(), directory, self.options)
            .await
    }
}
