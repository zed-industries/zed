use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use serde::Serialize;
use zbus::{
    object_server::SignalEmitter,
    zvariant::{OwnedObjectPath, Type, as_value},
};

use crate::{PortalError, desktop::HandleToken};

pub(crate) struct Session {
    path: OwnedObjectPath,
    manager: Arc<Mutex<SessionManager>>,
    monitor: Option<Arc<dyn SessionImpl>>,
}

impl Session {
    pub(crate) fn new(
        path: OwnedObjectPath,
        manager: Arc<Mutex<SessionManager>>,
        monitor: Option<Arc<dyn SessionImpl>>,
    ) -> Self {
        Self {
            path,
            manager,
            monitor,
        }
    }

    pub fn token(&self) -> HandleToken {
        HandleToken::try_from(&self.path).unwrap()
    }

    pub async fn serve(&self, cnx: zbus::Connection) -> zbus::Result<bool> {
        let interface = SessionInterface::new(
            self.path.clone(),
            Arc::clone(&self.manager),
            self.monitor.clone(),
        );
        cnx.object_server().at(&self.path, interface).await
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session").field("path", &self.path).finish()
    }
}

struct SessionInterface {
    path: OwnedObjectPath,
    manager: Arc<Mutex<SessionManager>>,
    monitor: Option<Arc<dyn SessionImpl>>,
}

impl SessionInterface {
    fn new(
        path: OwnedObjectPath,
        manager: Arc<Mutex<SessionManager>>,
        monitor: Option<Arc<dyn SessionImpl>>,
    ) -> Self {
        Self {
            path,
            manager,
            monitor,
        }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Session")]
impl SessionInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        1
    }

    #[doc(alias = "Close")]
    async fn close(
        &self,
        #[zbus(object_server)] server: &zbus::ObjectServer,
    ) -> zbus::fdo::Result<()> {
        #[cfg(feature = "tracing")]
        tracing::debug!("SessionInterface::Close {}", self.path.as_str());
        let token = HandleToken::try_from(&self.path).unwrap();
        {
            // Let the session manager know so it can update
            // its internal map of tracked sessions.
            let mut manager = self.manager.lock().unwrap();
            let _ = manager.remove(&token);
        }
        if let Some(monitor) = &self.monitor {
            // The backend implements the `SessionImpl` trait,
            // it wants to be notified that the session was closed.
            let _ = monitor.session_closed(token).await;
        }
        // This method intentionally does *not* emit the `Closed` signal.
        server.remove::<Self, _>(&self.path).await?;
        Ok(())
    }

    #[zbus(signal)]
    async fn closed(signal_emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
}

#[async_trait]
/// A trait that backends that create long-lived sessions should implement
/// to be notified when a session has been closed.
pub trait SessionImpl: Send + Sync {
    #[doc(alias = "Closed")]
    async fn session_closed(&self, session_token: HandleToken) -> crate::backend::Result<()>;
}

#[derive(Serialize, Type, Debug)]
#[zvariant(signature = "dict")]
pub struct CreateSessionResponse {
    #[serde(with = "as_value")]
    session_id: HandleToken,
}

impl CreateSessionResponse {
    pub fn new(token: HandleToken) -> Self {
        Self { session_id: token }
    }
}

#[derive(Default)]
// Not thread-safe! If it needs to be accessed from several threads,
// consider wrapping it in a Mutex.
pub(crate) struct SessionManager {
    sessions: HashMap<HandleToken, Session>,
}

impl SessionManager {
    /// Expects to find a tracked session with the given handle.
    /// Returns a suitable portal error if not.
    pub fn try_contains(&self, token: &HandleToken) -> crate::backend::Result<&Session> {
        self.sessions
            .get(token)
            .ok_or(PortalError::NotFound(format!("Unknown session: `{token}`")))
    }

    /// Tests whether a session with the given handle token already exists.
    pub fn contains(&self, token: &HandleToken) -> bool {
        #[cfg(feature = "tracing")]
        tracing::debug!("SessionManager::contains: tracked sessions: {:?}", &self);
        self.try_contains(token).is_ok()
    }

    /// Adds a session to the list of tracked sessions.
    /// Assumes that no session with the same handle token is already tracked
    /// (if there was one, it is silently discarded and backends are not
    /// notified). To avoid such a situation, it is recommended to call
    /// `contains()` prior to instantiating and adding a session.
    pub fn add(&mut self, session: Session) {
        let token = session.token();
        let _ = self.sessions.insert(token.clone(), session);
    }

    /// Removes a session from the list of tracked sessions.
    /// Returns an error if no session with the given handle token was tracked.
    pub fn remove(&mut self, token: &HandleToken) -> crate::backend::Result<()> {
        if self.sessions.remove(token).is_some() {
            Ok(())
        } else {
            let message = format!("Unknown session: `{token}`");
            #[cfg(feature = "tracing")]
            tracing::error!("{}", message.as_str());
            Err(PortalError::NotFound(message))
        }
    }
}

impl std::fmt::Debug for SessionManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.sessions.keys()).finish()
    }
}
