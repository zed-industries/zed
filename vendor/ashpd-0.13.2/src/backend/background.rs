use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use enumflags2::{BitFlags, bitflags};
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    AppID, PortalError,
    backend::request::{Request, RequestImpl},
    desktop::{HandleToken, Response},
    zbus::object_server::SignalEmitter,
    zvariant::{OwnedObjectPath, Type, as_value},
};

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, PartialEq, Eq, Debug, Type)]
#[repr(u32)]
pub enum Activity {
    Forbid = 0,
    Allow = 1,
    AllowInstance = 2,
}

#[derive(Debug, Serialize, Type)]
#[zvariant(signature = "dict")]
pub struct Background {
    #[serde(with = "as_value")]
    result: Activity,
}

impl Background {
    pub fn new(activity: Activity) -> Self {
        Self { result: activity }
    }
}

#[derive(Serialize_repr, Copy, Clone, PartialEq, Eq, Debug, Type)]
#[repr(u32)]
pub enum AppState {
    Background = 0,
    Running = 1,
    Active = 2,
}

#[bitflags]
#[derive(Deserialize_repr, PartialEq, Eq, Copy, Clone, Debug, Type)]
#[repr(u32)]
pub enum AutoStartFlags {
    DBusActivation = 1,
}

#[async_trait]
pub trait BackgroundSignalEmitter: Send + Sync {
    #[doc(alias = "RunningApplicationsChanged")]
    async fn emit_changed(&self) -> zbus::Result<()>;
}

#[async_trait]
pub trait BackgroundImpl: RequestImpl {
    #[doc(alias = "GetAppState")]
    async fn get_app_state(&self) -> Result<HashMap<AppID, AppState>, PortalError>;

    #[doc(alias = "NotifyBackground")]
    async fn notify_background(
        &self,
        token: HandleToken,
        app_id: AppID,
        name: &str,
    ) -> Result<Background, PortalError>;

    #[doc(alias = "EnableAutostart")]
    async fn enable_autostart(
        &self,
        app_id: AppID,
        enable: bool,
        commandline: Vec<String>,
        flags: BitFlags<AutoStartFlags>,
    ) -> Result<bool, PortalError>;

    // Set the signal emitter, allowing to notify of changes.
    fn set_signal_emitter(&mut self, signal_emitter: Arc<dyn BackgroundSignalEmitter>);
}

pub(crate) struct BackgroundInterface {
    imp: Arc<dyn BackgroundImpl>,
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    cnx: zbus::Connection,
}

impl BackgroundInterface {
    pub fn new(
        imp: Arc<dyn BackgroundImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    ) -> Self {
        Self { imp, cnx, spawn }
    }

    pub async fn changed(&self) -> zbus::Result<()> {
        let object_server = self.cnx.object_server();
        let iface_ref = object_server
            .interface::<_, Self>(crate::proxy::DESKTOP_PATH)
            .await?;
        Self::running_applications_changed(iface_ref.signal_emitter()).await
    }
}

#[async_trait]
impl BackgroundSignalEmitter for BackgroundInterface {
    async fn emit_changed(&self) -> zbus::Result<()> {
        self.changed().await
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Background")]
impl BackgroundInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        2
    }

    #[zbus(out_args("apps"))]
    async fn get_app_state(&self) -> Result<HashMap<AppID, AppState>, PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Background::GetAppState");

        let response = self.imp.get_app_state().await;

        #[cfg(feature = "tracing")]
        tracing::debug!("Background::GetAppState returned {:#?}", response);
        response
    }

    #[zbus(out_args("response", "results"))]
    async fn notify_background(
        &self,
        handle: OwnedObjectPath,
        app_id: AppID,
        name: String,
    ) -> Result<Response<Background>, PortalError> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "Background::NotifyBackground",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.notify_background(HandleToken::try_from(&handle).unwrap(), app_id, &name)
                    .await
            },
        )
        .await
    }

    #[zbus(out_args("result"))]
    async fn enable_autostart(
        &self,
        app_id: AppID,
        enable: bool,
        commandline: Vec<String>,
        flags: BitFlags<AutoStartFlags>,
    ) -> Result<bool, PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Background::EnableAutostart");

        let response = self
            .imp
            .enable_autostart(app_id, enable, commandline, flags)
            .await;

        #[cfg(feature = "tracing")]
        tracing::debug!("Background::EnableAutostart returned {:#?}", response);
        response
    }

    #[zbus(signal)]
    async fn running_applications_changed(signal_ctxt: &SignalEmitter<'_>) -> zbus::Result<()>;
}
