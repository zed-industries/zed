//! # Examples
//!
//! How to inhibit logout/user switch
//!
//! ```rust,no_run
//! use std::{thread, time};
//!
//! use ashpd::desktop::inhibit::{InhibitFlags, InhibitOptions, InhibitProxy, SessionState};
//! use futures_util::StreamExt;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = InhibitProxy::new().await?;
//!     let session = proxy.create_monitor(None, Default::default()).await?;
//!
//!     let state = proxy.receive_state_changed().await?.next().await.unwrap();
//!     match state.session_state() {
//!         SessionState::Running => (),
//!         SessionState::QueryEnd => {
//!             proxy
//!                 .inhibit(
//!                     None,
//!                     InhibitFlags::Logout | InhibitFlags::UserSwitch,
//!                     InhibitOptions::default()
//!                         .set_reason("please save the opened project first"),
//!                 )
//!                 .await?;
//!             thread::sleep(time::Duration::from_secs(1));
//!             proxy.query_end_response(&session).await?;
//!         }
//!         SessionState::Ending => {
//!             println!("ending the session");
//!         }
//!     }
//!     Ok(())
//! }
//! ```

use enumflags2::{BitFlags, bitflags};
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::zvariant::{
    ObjectPath, Optional, OwnedObjectPath, Type,
    as_value::{self, optional},
};

use super::{HandleToken, Request, Session, session::SessionPortal};
use crate::{Error, WindowIdentifier, desktop::session::CreateSessionResponse, proxy::Proxy};

#[derive(Serialize, Type, Debug, Default)]
/// Specified options for a [`InhibitProxy::create_monitor`] request.
#[zvariant(signature = "dict")]
pub struct CreateMonitorOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
    #[serde(with = "as_value")]
    session_handle_token: HandleToken,
}

#[derive(Serialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for a [`InhibitProxy::inhibit`] request.
pub struct InhibitOptions {
    #[serde(with = "as_value")]
    handle_token: HandleToken,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

impl InhibitOptions {
    /// Sets a user-visible reason for the request.
    #[must_use]
    pub fn set_reason<'a>(mut self, reason: impl Into<Option<&'a str>>) -> Self {
        self.reason = reason.into().map(ToOwned::to_owned);
        self
    }
}

#[bitflags]
#[derive(Serialize_repr, PartialEq, Eq, Debug, Clone, Copy, Type)]
#[repr(u32)]
#[doc(alias = "XdpInhibitFlags")]
/// The actions to inhibit that can end the user's session
pub enum InhibitFlags {
    #[doc(alias = "XDP_INHIBIT_FLAG_LOGOUT")]
    /// Logout.
    Logout,
    #[doc(alias = "XDP_INHIBIT_FLAG_USER_SWITCH")]
    /// User switch.
    UserSwitch,
    #[doc(alias = "XDP_INHIBIT_FLAG_SUSPEND")]
    /// Suspend.
    Suspend,
    #[doc(alias = "XDP_INHIBIT_FLAG_IDLE")]
    /// Idle.
    Idle,
}

#[derive(Debug, Deserialize, Type)]
#[zvariant(signature = "dict")]
#[serde(rename_all = "kebab-case")]
struct State {
    #[serde(with = "as_value")]
    screensaver_active: bool,
    #[serde(with = "as_value")]
    session_state: SessionState,
}

#[derive(Debug, Deserialize, Type)]
/// A response received when the `state_changed` signal is received.
pub struct InhibitState(OwnedObjectPath, State);

impl InhibitState {
    /// The session triggered the state change
    pub fn session_handle(&self) -> ObjectPath<'_> {
        self.0.as_ref()
    }

    /// Whether screensaver is active or not.
    pub fn screensaver_active(&self) -> bool {
        self.1.screensaver_active
    }

    /// The session state.
    pub fn session_state(&self) -> SessionState {
        self.1.session_state
    }
}

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdSessionState"))]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Clone, Copy, Type)]
#[doc(alias = "XdpLoginSessionState")]
#[repr(u32)]
/// The current state of the user's session.
pub enum SessionState {
    #[doc(alias = "XDP_LOGIN_SESSION_RUNNING")]
    /// Running.
    Running = 1,
    #[doc(alias = "XDP_LOGIN_SESSION_QUERY_END")]
    /// The user asked to end the session e.g logout.
    QueryEnd = 2,
    #[doc(alias = "XDP_LOGIN_SESSION_ENDING")]
    /// The session is ending.
    Ending = 3,
}

/// The interface lets sandboxed applications inhibit the user session from
/// ending, suspending, idling or getting switched away.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Inhibit`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Inhibit.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Inhibit")]
pub struct InhibitProxy(Proxy<'static>);

impl InhibitProxy {
    /// Create a new instance of [`InhibitProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Inhibit").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`InhibitProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Inhibit")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Creates a monitoring session.
    /// While this session is active, the caller will receive `state_changed`
    /// signals with updates on the session state.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The application window identifier.
    ///
    /// # Specifications
    ///
    /// See also [`CreateMonitor`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Inhibit.html#org-freedesktop-portal-inhibit-createmonitor).
    #[doc(alias = "CreateMonitor")]
    #[doc(alias = "xdp_portal_session_monitor_start")]
    pub async fn create_monitor(
        &self,
        identifier: Option<&WindowIdentifier>,
        options: CreateMonitorOptions,
    ) -> Result<Session<Self>, Error> {
        let identifier = Optional::from(identifier);
        let body = &(identifier, &options);
        let (monitor, proxy) = futures_util::try_join!(
            self.0
                .request::<CreateSessionResponse>(&options.handle_token, "CreateMonitor", body),
            Session::from_unique_name(self.0.connection().clone(), &options.session_handle_token),
        )?;
        assert_eq!(proxy.path(), &monitor.response()?.session_handle.as_ref());
        Ok(proxy)
    }

    /// Inhibits a session status changes.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The application window identifier.
    /// * `flags` - The flags determine what changes are inhibited.
    ///
    /// # Specifications
    ///
    /// See also [`Inhibit`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Inhibit.html#org-freedesktop-portal-inhibit-inhibit).
    #[doc(alias = "Inhibit")]
    #[doc(alias = "xdp_portal_session_inhibit")]
    pub async fn inhibit(
        &self,
        identifier: Option<&WindowIdentifier>,
        flags: BitFlags<InhibitFlags>,
        options: InhibitOptions,
    ) -> Result<Request<()>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .empty_request(
                &options.handle_token,
                "Inhibit",
                &(identifier, flags, &options),
            )
            .await
    }

    /// Signal emitted when the session state changes.
    ///
    /// # Specifications
    ///
    /// See also [`StateChanged`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Inhibit.html#org-freedesktop-portal-inhibit-statechanged).
    #[doc(alias = "StateChanged")]
    #[doc(alias = "XdpPortal::session-state-changed")]
    pub async fn receive_state_changed(
        &self,
    ) -> Result<impl Stream<Item = InhibitState> + use<'_>, Error> {
        self.0.signal("StateChanged").await
    }

    /// Acknowledges that the caller received the "state_changed" signal.
    /// This method should be called within one second after receiving a
    /// [`receive_state_changed()`][`InhibitProxy::receive_state_changed`]
    /// signal with the [`SessionState::QueryEnd`] state.
    ///
    /// # Arguments
    ///
    /// * `session` - A [`Session`], created with
    ///   [`create_monitor()`][`InhibitProxy::create_monitor`].
    ///
    /// # Specifications
    ///
    /// See also [`QueryEndResponse`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Inhibit.html#org-freedesktop-portal-inhibit-queryendresponse).
    #[doc(alias = "QueryEndResponse")]
    #[doc(alias = "xdp_portal_session_monitor_query_end_response")]
    pub async fn query_end_response(&self, session: &Session<Self>) -> Result<(), Error> {
        self.0.call("QueryEndResponse", &(session)).await
    }
}

impl std::ops::Deref for InhibitProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl crate::Sealed for InhibitProxy {}
impl SessionPortal for InhibitProxy {}
