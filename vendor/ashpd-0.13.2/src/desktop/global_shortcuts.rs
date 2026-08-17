//! Register global shortcuts

use std::{collections::HashMap, fmt::Debug, time::Duration};

use futures_util::Stream;
use serde::{Deserialize, Serialize};
use zbus::zvariant::{
    ObjectPath, Optional, OwnedObjectPath, OwnedValue, Type,
    as_value::{self, optional},
};

use super::{HandleToken, Request, Session, session::SessionPortal};
use crate::{
    ActivationToken, Error, WindowIdentifier,
    desktop::session::{CreateSessionOptions, CreateSessionResponse},
    proxy::Proxy,
};

#[derive(Clone, Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
struct NewShortcutInfo {
    /// User-readable text describing what the shortcut does.
    #[serde(with = "as_value")]
    description: String,
    /// The preferred shortcut trigger, defined as described by the "shortcuts"
    /// XDG specification. Optional.
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    preferred_trigger: Option<String>,
}

/// Shortcut descriptor used to bind new shortcuts in
/// [`GlobalShortcuts::bind_shortcuts`]
#[derive(Clone, Serialize, Type, Debug)]
pub struct NewShortcut(String, NewShortcutInfo);

impl NewShortcut {
    /// Construct new shortcut
    pub fn new(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self(
            id.into(),
            NewShortcutInfo {
                description: description.into(),
                preferred_trigger: None,
            },
        )
    }

    /// Sets the preferred shortcut trigger, defined as described by the
    /// "shortcuts" XDG specification.
    #[must_use]
    pub fn preferred_trigger<'a>(mut self, preferred_trigger: impl Into<Option<&'a str>>) -> Self {
        self.1.preferred_trigger = preferred_trigger.into().map(ToOwned::to_owned);
        self
    }
}

#[derive(Clone, Deserialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
struct ShortcutInfo {
    /// User-readable text describing what the shortcut does.
    #[serde(with = "as_value")]
    description: String,
    /// User-readable text describing how to trigger the shortcut for the client
    /// to render.
    #[serde(with = "as_value")]
    trigger_description: String,
}

/// Struct that contains information about existing binded shortcut.
///
/// If you need to create a new shortcuts, take a look at [`NewShortcut`]
/// instead.
#[derive(Clone, Deserialize, Type, Debug)]
pub struct Shortcut(String, ShortcutInfo);

impl Shortcut {
    /// Shortcut id
    pub fn id(&self) -> &str {
        &self.0
    }

    /// User-readable text describing what the shortcut does.
    pub fn description(&self) -> &str {
        &self.1.description
    }

    /// User-readable text describing how to trigger the shortcut for the client
    /// to render.
    pub fn trigger_description(&self) -> &str {
        &self.1.trigger_description
    }
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`GlobalShortcuts::bind_shortcuts`] request.
pub struct BindShortcutsOptions {
    /// A string that will be used as the last element of the handle.
    #[serde(with = "as_value")]
    handle_token: HandleToken,
}

/// A response to a [`GlobalShortcuts::bind_shortcuts`] request.
#[derive(Deserialize, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct BindShortcuts {
    #[serde(default, with = "as_value")]
    shortcuts: Vec<Shortcut>,
}

impl BindShortcuts {
    /// A list of shortcuts.
    pub fn shortcuts(&self) -> &[Shortcut] {
        &self.shortcuts
    }
}

#[derive(Serialize, Type, Debug)]
#[zvariant(signature = "dict")]
/// Specified options for a [`GlobalShortcuts::configure_shortcuts`] request.
pub struct ConfigureShortcutsOptions {
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    activation_token: Option<ActivationToken>,
}

impl ConfigureShortcutsOptions {
    /// Sets the token that can be used to activate the configuration dialog.
    #[must_use]
    pub fn set_activation_token(
        mut self,
        activation_token: impl Into<Option<ActivationToken>>,
    ) -> Self {
        self.activation_token = activation_token.into();
        self
    }
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`GlobalShortcuts::list_shortcuts`] request.
pub struct ListShortcutsOptions {
    /// A string that will be used as the last element of the handle.
    #[serde(with = "as_value")]
    handle_token: HandleToken,
}

/// A response to a [`GlobalShortcuts::list_shortcuts`] request.
#[derive(Deserialize, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct ListShortcuts {
    /// A list of shortcuts.
    #[serde(default, with = "as_value")]
    shortcuts: Vec<Shortcut>,
}

impl ListShortcuts {
    /// A list of shortcuts.
    pub fn shortcuts(&self) -> &[Shortcut] {
        &self.shortcuts
    }
}

/// Notifies about a shortcut becoming active.
#[derive(Debug, Deserialize, Type)]
pub struct Activated(OwnedObjectPath, String, u64, HashMap<String, OwnedValue>);

impl Activated {
    /// Session that requested the shortcut.
    pub fn session_handle(&self) -> ObjectPath<'_> {
        self.0.as_ref()
    }

    /// The application-provided ID for the shortcut.
    pub fn shortcut_id(&self) -> &str {
        &self.1
    }

    /// The timestamp, as seconds and microseconds since the Unix epoch.
    pub fn timestamp(&self) -> Duration {
        Duration::from_millis(self.2)
    }

    /// Optional information
    pub fn options(&self) -> &HashMap<String, OwnedValue> {
        &self.3
    }
}

/// Notifies that a shortcut is not active anymore.
#[derive(Debug, Deserialize, Type)]
pub struct Deactivated(OwnedObjectPath, String, u64, HashMap<String, OwnedValue>);

impl Deactivated {
    /// Session that requested the shortcut.
    pub fn session_handle(&self) -> ObjectPath<'_> {
        self.0.as_ref()
    }

    /// The application-provided ID for the shortcut.
    pub fn shortcut_id(&self) -> &str {
        &self.1
    }

    /// The timestamp, as seconds and microseconds since the Unix epoch.
    pub fn timestamp(&self) -> Duration {
        Duration::from_millis(self.2)
    }

    /// Optional information
    pub fn options(&self) -> &HashMap<String, OwnedValue> {
        &self.3
    }
}

/// Indicates that the information associated with some of the shortcuts has
/// changed.
#[derive(Debug, Deserialize, Type)]
pub struct ShortcutsChanged(OwnedObjectPath, Vec<Shortcut>);

impl ShortcutsChanged {
    /// Session that requested the shortcut.
    pub fn session_handle(&self) -> ObjectPath<'_> {
        self.0.as_ref()
    }

    /// Shortcuts that have been registered.
    pub fn shortcuts(&self) -> &[Shortcut] {
        &self.1
    }
}

/// Wrapper of the DBus interface: [`org.freedesktop.portal.GlobalShortcuts`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.GlobalShortcuts")]
pub struct GlobalShortcuts(Proxy<'static>);

impl GlobalShortcuts {
    /// Create a new instance of [`GlobalShortcuts`].
    pub async fn new() -> Result<GlobalShortcuts, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.GlobalShortcuts").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`GlobalShortcuts`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<GlobalShortcuts, Error> {
        let proxy = Proxy::new_desktop_with_connection(
            connection,
            "org.freedesktop.portal.GlobalShortcuts",
        )
        .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Create a global shortcuts session.
    ///
    /// # Specifications
    ///
    /// See also [`CreateSession`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html#org-freedesktop-portal-globalshortcuts-createsession).
    #[doc(alias = "CreateSession")]
    pub async fn create_session(
        &self,
        options: CreateSessionOptions,
    ) -> Result<Session<Self>, Error> {
        let (request, proxy) = futures_util::try_join!(
            self.0.request::<CreateSessionResponse>(
                &options.handle_token,
                "CreateSession",
                &options
            ),
            Session::from_unique_name(self.0.connection().clone(), &options.session_handle_token),
        )?;
        assert_eq!(proxy.path(), &request.response()?.session_handle.as_ref());
        Ok(proxy)
    }

    /// Bind the shortcuts.
    ///
    /// # Specifications
    ///
    /// See also [`BindShortcuts`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html#org-freedesktop-portal-globalshortcuts-bindshortcuts).
    #[doc(alias = "BindShortcuts")]
    pub async fn bind_shortcuts(
        &self,
        session: &Session<Self>,
        shortcuts: &[NewShortcut],
        identifier: Option<&WindowIdentifier>,
        options: BindShortcutsOptions,
    ) -> Result<Request<BindShortcuts>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .request(
                &options.handle_token,
                "BindShortcuts",
                &(session, shortcuts, identifier, &options),
            )
            .await
    }

    /// Lists all shortcuts.
    ///
    /// # Specifications
    ///
    /// See also [`ListShortcuts`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html#org-freedesktop-portal-globalshortcuts-listshortcuts).
    #[doc(alias = "ListShortcuts")]
    pub async fn list_shortcuts(
        &self,
        session: &Session<Self>,
        options: ListShortcutsOptions,
    ) -> Result<Request<ListShortcuts>, Error> {
        self.0
            .request(&options.handle_token, "ListShortcuts", &(session, &options))
            .await
    }

    /// Request showing a configuration UI so the user is able to configure all
    /// shortcuts of this session.
    ///
    /// # Specifications
    ///
    /// See also [`ConfigureShortcuts`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html#org-freedesktop-portal-globalshortcuts-configureshortcuts).
    #[doc(alias = "ConfigureShortcuts")]
    pub async fn configure_shortcuts(
        &self,
        session: &Session<Self>,
        identifier: Option<&WindowIdentifier>,
        options: ConfigureShortcutsOptions,
    ) -> Result<(), Error> {
        let identifier = Optional::from(identifier);

        self.0
            .call_versioned::<()>("ConfigureShortcuts", &(session, identifier, options), 2)
            .await
    }

    /// Signal emitted when shortcut becomes active.
    ///
    /// # Specifications
    ///
    /// See also [`Activated`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html#org-freedesktop-portal-globalshortcuts-activated).
    #[doc(alias = "Activated")]
    pub async fn receive_activated(
        &self,
    ) -> Result<impl Stream<Item = Activated> + use<'_>, Error> {
        self.0.signal("Activated").await
    }

    /// Signal emitted when shortcut is not active anymore.
    ///
    /// # Specifications
    ///
    /// See also [`Deactivated`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html#org-freedesktop-portal-globalshortcuts-deactivated).
    #[doc(alias = "Deactivated")]
    pub async fn receive_deactivated(
        &self,
    ) -> Result<impl Stream<Item = Deactivated> + use<'_>, Error> {
        self.0.signal("Deactivated").await
    }

    /// Signal emitted when information associated with some of the shortcuts
    /// has changed.
    ///
    /// # Specifications
    ///
    /// See also [`ShortcutsChanged`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html#org-freedesktop-portal-globalshortcuts-shortcutschanged).
    #[doc(alias = "ShortcutsChanged")]
    pub async fn receive_shortcuts_changed(
        &self,
    ) -> Result<impl Stream<Item = ShortcutsChanged> + use<'_>, Error> {
        self.0.signal("ShortcutsChanged").await
    }
}

impl std::ops::Deref for GlobalShortcuts {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl crate::Sealed for GlobalShortcuts {}
impl SessionPortal for GlobalShortcuts {}
