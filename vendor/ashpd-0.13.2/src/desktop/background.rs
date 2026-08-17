//! Request to run in the background or started automatically when the user
//! logs in.
//!
//! **Note** This portal only works for sandboxed applications.
//!
//! Wrapper of the DBus interface: [`org.freedesktop.portal.Background`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Background.html).
//!
//! ### Examples
//!
//! ```rust,no_run
//! use ashpd::desktop::background::Background;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let response = Background::request()
//!         .reason("Automatically fetch your latest mails")
//!         .auto_start(true)
//!         .command(&["geary"])
//!         .dbus_activatable(false)
//!         .send()
//!         .await?
//!         .response()?;
//!
//!     println!("{}", response.auto_start());
//!     println!("{}", response.run_in_background());
//!
//!     Ok(())
//! }
//! ```
//!
//! If no `command` is provided, the [`Exec`](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#exec-variables) line from the [desktop
//! file](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#introduction) will be used.

use serde::{Deserialize, Serialize};
use zbus::zvariant::{
    Optional, Type,
    as_value::{self, optional},
};

use super::{HandleToken, Request};
use crate::{Error, WindowIdentifier, proxy::Proxy};

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`BackgroundProxy::request_background`] request.
pub struct BackgroundRequestOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    autostart: Option<bool>,
    #[serde(
        with = "optional",
        rename = "dbus-activatable",
        skip_serializing_if = "Option::is_none"
    )]
    dbus_activatable: Option<bool>,
    #[serde(
        with = "as_value",
        rename = "commandline",
        skip_serializing_if = "Vec::is_empty"
    )]
    command: Vec<String>,
}

impl BackgroundRequestOptions {
    #[must_use]
    /// Sets whether to auto start the application or not.
    pub fn set_auto_start(mut self, auto_start: impl Into<Option<bool>>) -> Self {
        self.autostart = auto_start.into();
        self
    }

    #[must_use]
    /// Sets whether the application is dbus activatable.
    pub fn set_dbus_activatable(mut self, dbus_activatable: impl Into<Option<bool>>) -> Self {
        self.dbus_activatable = dbus_activatable.into();
        self
    }

    #[must_use]
    /// Specifies the command line to execute.
    /// If this is not specified, the [`Exec`](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#exec-variables) line from the [desktop
    /// file](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#introduction)
    pub fn set_command<P: IntoIterator<Item = I>, I: AsRef<str> + Type + Serialize>(
        mut self,
        command: P,
    ) -> Self {
        self.command = command.into_iter().map(|s| s.as_ref().to_owned()).collect();
        self
    }

    #[must_use]
    /// Sets a user-visible reason for the request.
    pub fn set_reason<'a>(mut self, reason: impl Into<Option<&'a str>>) -> Self {
        self.reason = reason.into().map(ToOwned::to_owned);
        self
    }
}

#[derive(Deserialize, Type, Debug)]
/// The response of a [`BackgroundRequest`] request.
#[zvariant(signature = "dict")]
pub struct Background {
    #[serde(with = "as_value")]
    background: bool,
    #[serde(with = "as_value")]
    autostart: bool,
}

impl Background {
    /// Creates a new builder-pattern struct instance to construct
    /// [`Background`].
    ///
    /// This method returns an instance of [`BackgroundRequest`].
    pub fn request() -> BackgroundRequest {
        BackgroundRequest::default()
    }

    /// If the application is allowed to run in the background.
    pub fn run_in_background(&self) -> bool {
        self.background
    }

    /// If the application will be auto-started.
    pub fn auto_start(&self) -> bool {
        self.autostart
    }
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`BackgroundProxy::set_status`] request.
pub struct SetStatusOptions {
    #[serde(with = "as_value")]
    message: String,
}

impl SetStatusOptions {
    /// Sets the message to be displayed to the user.
    pub fn set_message(mut self, message: &str) -> Self {
        self.message = message.to_owned();
        self
    }
}

/// The interface lets sandboxed applications request that the application
/// is allowed to run in the background or started automatically when the user
/// logs in.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Background`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Background.html).
#[doc(alias = "org.freedesktop.portal.Background")]
pub struct BackgroundProxy(Proxy<'static>);

impl BackgroundProxy {
    /// Create a new instance of [`BackgroundProxy`].
    pub async fn new() -> Result<BackgroundProxy, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Background").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`BackgroundProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<BackgroundProxy, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Background")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    ///  Sets the status of the application running in background.
    ///
    /// # Arguments
    ///
    /// * `message` - A string that will be used as the status message of the
    ///   application.
    ///
    /// # Required version
    ///
    /// The method requires the 2nd version implementation of the portal and
    /// would fail with [`Error::RequiresVersion`] otherwise.
    ///
    /// # Specifications
    ///
    /// See also [`SetStatus`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Background.html#org-freedesktop-portal-background-setstatus).
    #[doc(alias = "SetStatus")]
    pub async fn set_status(&self, options: SetStatusOptions) -> Result<(), Error> {
        self.0.call_versioned("SetStatus", &(options), 2).await
    }

    ///  Request background access.
    ///
    /// # Specifications
    ///
    /// See also [`RequestBackground`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Background.html#org-freedesktop-portal-background-requestbackground).
    #[doc(alias = "RequestBackground")]
    pub async fn request_background(
        &self,
        identifier: Option<&WindowIdentifier>,
        options: BackgroundRequestOptions,
    ) -> Result<Request<Background>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .request(
                &options.handle_token,
                "RequestBackground",
                (identifier, &options),
            )
            .await
    }
}

impl std::ops::Deref for BackgroundProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[doc(alias = "xdp_portal_request_background")]
/// A [builder-pattern] type to construct [`Background`].
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[derive(Debug, Default)]
pub struct BackgroundRequest {
    identifier: Option<WindowIdentifier>,
    options: BackgroundRequestOptions,
}

impl BackgroundRequest {
    #[must_use]
    /// Sets a window identifier.
    pub fn identifier(mut self, identifier: impl Into<Option<WindowIdentifier>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    #[must_use]
    /// Sets whether to auto start the application or not.
    pub fn auto_start(mut self, auto_start: impl Into<Option<bool>>) -> Self {
        self.options.autostart = auto_start.into();
        self
    }

    #[must_use]
    /// Sets whether the application is dbus activatable.
    pub fn dbus_activatable(mut self, dbus_activatable: impl Into<Option<bool>>) -> Self {
        self.options.dbus_activatable = dbus_activatable.into();
        self
    }

    #[must_use]
    /// Specifies the command line to execute.
    /// If this is not specified, the [`Exec`](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#exec-variables) line from the [desktop
    /// file](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#introduction)
    pub fn command<P: IntoIterator<Item = I>, I: AsRef<str> + Type + Serialize>(
        mut self,
        command: P,
    ) -> Self {
        self.options.command = command.into_iter().map(|s| s.as_ref().to_owned()).collect();
        self
    }

    #[must_use]
    /// Sets a user-visible reason for the request.
    pub fn reason<'a>(mut self, reason: impl Into<Option<&'a str>>) -> Self {
        self.options.reason = reason.into().map(ToOwned::to_owned);
        self
    }

    /// Build the [`Background`].
    pub async fn send(self) -> Result<Request<Background>, Error> {
        let proxy = BackgroundProxy::new().await?;
        proxy
            .request_background(self.identifier.as_ref(), self.options)
            .await
    }
}
