//! Compose an email.
//!
//! Wrapper of the DBus interface: [`org.freedesktop.portal.Email`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Email.html).
//!
//! # Examples
//!
//! Compose an email
//!
//! ```rust,no_run
//! use std::{fs::File, os::fd::OwnedFd};
//!
//! use ashpd::desktop::email::EmailRequest;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let file = File::open("/home/bilelmoussaoui/Downloads/adwaita-night.jpg").unwrap();
//!     EmailRequest::default()
//!         .address("test@gmail.com")
//!         .subject("email subject")
//!         .body("the pre-filled email body")
//!         .attach(OwnedFd::from(file))
//!         .send()
//!         .await;
//!     Ok(())
//! }
//! ```

use std::os::fd::OwnedFd;

use serde::{Deserialize, Serialize};
use zbus::zvariant::{
    self, Optional, Type,
    as_value::{self, optional},
};

use super::{HandleToken, Request};
use crate::{ActivationToken, Error, Uri, WindowIdentifier, proxy::Proxy};

/// Options for composing an email.
#[derive(Serialize, Deserialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct EmailOptions {
    #[serde(with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    addresses: Vec<String>,
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    cc: Vec<String>,
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    bcc: Vec<String>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    subject: Option<String>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(
        with = "as_value",
        skip_serializing_if = "Vec::is_empty",
        skip_deserializing
    )]
    attachment_fds: Vec<zvariant::OwnedFd>,
    #[serde(default, with = "as_value", skip_serializing)]
    #[cfg_attr(not(feature = "backend"), allow(dead_code))]
    attachments: Vec<Uri>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    activation_token: Option<ActivationToken>,
}

impl EmailOptions {
    /// Gets the email address.
    #[cfg(feature = "backend")]
    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }

    /// Gets the list of email addresses.
    #[cfg(feature = "backend")]
    pub fn addresses(&self) -> &[String] {
        &self.addresses
    }

    /// Gets the list of CC email addresses.
    #[cfg(feature = "backend")]
    pub fn cc(&self) -> &[String] {
        &self.cc
    }

    /// Gets the list of BCC email addresses.
    #[cfg(feature = "backend")]
    pub fn bcc(&self) -> &[String] {
        &self.bcc
    }

    /// Gets the email subject.
    #[cfg(feature = "backend")]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Gets the email body.
    #[cfg(feature = "backend")]
    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    /// Gets the list of attachment URIs.
    #[cfg(feature = "backend")]
    pub fn attachments(&self) -> &[Uri] {
        &self.attachments
    }

    /// Gets the activation token.
    #[cfg(feature = "backend")]
    pub fn activation_token(&self) -> Option<&ActivationToken> {
        self.activation_token.as_ref()
    }
}

/// Wrapper of the DBus interface: [`org.freedesktop.portal.Email`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Email.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Email")]
pub struct EmailProxy(Proxy<'static>);

impl EmailProxy {
    /// Create a new instance of [`EmailProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Email").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`EmailProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Email").await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Presents a window that lets the user compose an email.
    ///
    /// **Note** the default email client for the host will need to support
    /// `mailto:` URIs following RFC 2368.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Identifier for the application window.
    /// * `options` - An [`EmailOptions`].
    ///
    /// # Specifications
    ///
    /// See also [`ComposeEmail`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Email.html#org-freedesktop-portal-email-composeemail).
    #[doc(alias = "ComposeEmail")]
    pub async fn compose(
        &self,
        identifier: Option<&WindowIdentifier>,
        options: EmailOptions,
    ) -> Result<Request<()>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .empty_request(
                &options.handle_token,
                "ComposeEmail",
                &(identifier, &options),
            )
            .await
    }
}

impl std::ops::Deref for EmailProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
#[doc(alias = "xdp_portal_compose_email")]
/// A [builder-pattern] type to compose an email.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct EmailRequest {
    identifier: Option<WindowIdentifier>,
    options: EmailOptions,
    connection: Option<zbus::Connection>,
}

impl EmailRequest {
    /// Sets a window identifier.
    #[must_use]
    pub fn identifier(mut self, identifier: impl Into<Option<WindowIdentifier>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    /// Sets the email address to send the email to.
    #[must_use]
    pub fn address<'a>(mut self, address: impl Into<Option<&'a str>>) -> Self {
        self.options.address = address.into().map(ToOwned::to_owned);
        self
    }

    /// Sets a list of email addresses to send the email to.
    #[must_use]
    pub fn addresses<P: IntoIterator<Item = I>, I: AsRef<str> + Type + Serialize>(
        mut self,
        addresses: impl Into<Option<P>>,
    ) -> Self {
        if let Some(a) = addresses.into() {
            self.options.addresses = a.into_iter().map(|s| s.as_ref().to_owned()).collect();
        }
        self
    }

    /// Sets a list of email addresses to BCC.
    #[must_use]
    pub fn bcc<P: IntoIterator<Item = I>, I: AsRef<str> + Type + Serialize>(
        mut self,
        bcc: impl Into<Option<P>>,
    ) -> Self {
        if let Some(a) = bcc.into() {
            self.options.bcc = a.into_iter().map(|s| s.as_ref().to_owned()).collect();
        }
        self
    }

    /// Sets a list of email addresses to CC.
    #[must_use]
    pub fn cc<P: IntoIterator<Item = I>, I: AsRef<str> + Type + Serialize>(
        mut self,
        cc: impl Into<Option<P>>,
    ) -> Self {
        if let Some(a) = cc.into() {
            self.options.cc = a.into_iter().map(|s| s.as_ref().to_owned()).collect();
        }
        self
    }

    /// Sets the email subject.
    #[must_use]
    pub fn subject<'a>(mut self, subject: impl Into<Option<&'a str>>) -> Self {
        self.options.subject = subject.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the email body.
    #[must_use]
    pub fn body<'a>(mut self, body: impl Into<Option<&'a str>>) -> Self {
        self.options.body = body.into().map(ToOwned::to_owned);
        self
    }

    /// Attaches a file to the email.
    #[must_use]
    pub fn attach(mut self, attachment: OwnedFd) -> Self {
        self.add_attachment(attachment);
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

    /// A different variant of [`Self::attach`].
    pub fn add_attachment(&mut self, attachment: OwnedFd) {
        let attachment = zvariant::OwnedFd::from(attachment);
        self.options.attachment_fds.push(attachment);
    }

    #[must_use]
    /// Sets a connection to use other than the internal one.
    pub fn connection(mut self, connection: Option<zbus::Connection>) -> Self {
        self.connection = connection;
        self
    }

    /// Send the request.
    pub async fn send(mut self) -> Result<Request<()>, Error> {
        let proxy = if let Some(connection) = self.connection {
            EmailProxy::with_connection(connection).await?
        } else {
            EmailProxy::new().await?
        };

        // activation_token requires version 4
        if proxy.version() < 4 {
            self.options.activation_token = None;
        }

        proxy.compose(self.identifier.as_ref(), self.options).await
    }
}
