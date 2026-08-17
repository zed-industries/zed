use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use futures_util::Stream;
use serde::{Deserialize, Serialize, Serializer};
use zbus::zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Type, as_value};

use crate::{Error, desktop::HandleToken, proxy::Proxy};

/// Shared by all portal interfaces that involve long lived sessions.
///
/// When a method that creates a session is called, if successful, the reply
/// will include a session handle (i.e. object path) for a Session object, which
/// will stay alive for the duration of the session.
///
/// The duration of the session is defined by the interface that creates it.
/// For convenience, the interface contains a method [`Session::close`],
/// and a signal [`Session::receive_closed`]. Whether it is allowed to
/// directly call [`Session::close`] depends on the interface.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Session`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Session.html).
#[derive(Type)]
#[doc(alias = "org.freedesktop.portal.Session")]
#[zvariant(signature = "o")]
pub struct Session<T>(Proxy<'static>, PhantomData<T>)
where
    T: SessionPortal;

impl<T> Session<T>
where
    T: SessionPortal,
{
    /// Create a new instance of [`Session`].
    ///
    /// **Note** A [`Session`] is not supposed to be created manually.
    pub(crate) async fn with_connection<P>(
        connection: zbus::Connection,
        path: P,
    ) -> Result<Self, Error>
    where
        P: TryInto<ObjectPath<'static>>,
        P::Error: Into<zbus::Error>,
    {
        let proxy =
            Proxy::new_desktop_with_path(connection, "org.freedesktop.portal.Session", path)
                .await?;
        Ok(Self(proxy, PhantomData))
    }

    pub(crate) async fn from_unique_name(
        connection: zbus::Connection,
        handle_token: &HandleToken,
    ) -> Result<Self, crate::Error> {
        let path = Proxy::unique_name(
            &connection,
            "/org/freedesktop/portal/desktop/session",
            handle_token,
        )
        .await?;
        #[cfg(feature = "tracing")]
        tracing::info!("Creating a org.freedesktop.portal.Session {}", path);
        Self::with_connection(connection, path).await
    }

    /// Emitted when a session is closed.
    ///
    /// # Specifications
    ///
    /// See also [`Closed`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Session.html#org-freedesktop-portal-session-closed).
    #[doc(alias = "Closed")]
    pub async fn receive_closed(
        &self,
    ) -> Result<impl Stream<Item = HashMap<String, OwnedValue>>, Error> {
        self.0.signal("Closed").await
    }

    /// Closes the portal session to which this object refers and ends all
    /// related user interaction (dialogs, etc).
    ///
    /// # Specifications
    ///
    /// See also [`Close`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Session.html#org-freedesktop-portal-session-close).
    #[doc(alias = "Close")]
    pub async fn close(&self) -> Result<(), Error> {
        self.0.call("Close", &()).await
    }

    pub(crate) fn path(&self) -> &ObjectPath<'_> {
        self.0.path()
    }
}

impl<T> Serialize for Session<T>
where
    T: SessionPortal,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ObjectPath::serialize(self.path(), serializer)
    }
}

impl<T> Debug for Session<T>
where
    T: SessionPortal,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Session")
            .field(&self.path().as_str())
            .finish()
    }
}

/// Portals that have a long-lived interaction
pub trait SessionPortal: crate::Sealed {}

/// A response to a `create_session` request.
#[derive(Type, Debug)]
#[zvariant(signature = "dict")]
pub(crate) struct CreateSessionResponse {
    pub(crate) session_handle: OwnedObjectPath,
}

// Context: Various portal were expected to actually return an OwnedObjectPath
// but unfortunately this wasn't the case when the portals were implemented in
// xdp. Fixing that would be an API break as well...
// See <https://github.com/flatpak/xdg-desktop-portal/pull/609>
// The Location, ScreenCast, Remote Desktop, Global Shortcuts and Inhibit
// portals `CreateSession` calls are all affected.
//
// So in order to be future proof, we try to deserialize the `session_handle`
// key as a string and fallback to an object path in case the situation gets
// resolved in the future.
impl<'de> Deserialize<'de> for CreateSessionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map: HashMap<String, OwnedValue> = HashMap::deserialize(deserializer)?;
        let session_handle = map.get("session_handle").ok_or_else(|| {
            serde::de::Error::custom(
                "CreateSessionResponse failed to deserialize. Couldn't find a session_handle",
            )
        })?;

        let path = if let Ok(object_path_str) = session_handle.downcast_ref::<&str>() {
            ObjectPath::try_from(object_path_str).unwrap()
        } else if let Ok(object_path) = session_handle.downcast_ref::<ObjectPath<'_>>() {
            object_path
        } else {
            return Err(serde::de::Error::custom(
                "Wrong session_handle type. Expected `s` or `o`.",
            ));
        };

        Ok(Self {
            session_handle: path.into(),
        })
    }
}

#[derive(Serialize, Deserialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
/// Specified options for creating a session.
pub struct CreateSessionOptions {
    #[serde(with = "as_value")]
    pub(crate) handle_token: HandleToken,
    #[serde(with = "as_value")]
    pub(crate) session_handle_token: HandleToken,
}
