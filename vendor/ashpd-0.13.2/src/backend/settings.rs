use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    PortalError,
    desktop::{
        Color,
        settings::{
            ACCENT_COLOR_SCHEME_KEY, APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY, CONTRAST_KEY,
            ColorScheme, Contrast, Namespace,
        },
    },
    zbus::object_server::SignalEmitter,
    zvariant::{OwnedValue, Value},
};

#[async_trait]
pub trait SettingsSignalEmitter: Send + Sync {
    #[doc(alias = "SettingChanged")]
    async fn emit_changed(&self, namespace: &str, key: &str, value: Value<'_>) -> zbus::Result<()>;
    async fn emit_contrast_changed(&self, contrast: Contrast) -> zbus::Result<()>;
    async fn emit_accent_color_changed(&self, color: Color) -> zbus::Result<()>;
    async fn emit_color_scheme_changed(&self, scheme: ColorScheme) -> zbus::Result<()>;
}

#[async_trait]
pub trait SettingsImpl: Send + Sync {
    #[doc(alias = "ReadAll")]
    async fn read_all(
        &self,
        namespaces: Vec<String>,
    ) -> Result<HashMap<String, Namespace>, PortalError>;

    #[doc(alias = "Read")]
    async fn read(&self, namespace: &str, key: &str) -> Result<OwnedValue, PortalError>;

    // Set the signal emitter, allowing to notify of changes.
    fn set_signal_emitter(&mut self, signal_emitter: Arc<dyn SettingsSignalEmitter>);
}

pub(crate) struct SettingsInterface {
    imp: Arc<dyn SettingsImpl>,
    cnx: zbus::Connection,
    #[allow(dead_code)]
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
}

impl SettingsInterface {
    pub fn new(
        imp: Arc<dyn SettingsImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    ) -> Self {
        Self { imp, cnx, spawn }
    }

    pub async fn changed(&self, namespace: &str, key: &str, value: Value<'_>) -> zbus::Result<()> {
        let object_server = self.cnx.object_server();
        let iface_ref = object_server
            .interface::<_, Self>(crate::proxy::DESKTOP_PATH)
            .await?;
        Self::setting_changed(iface_ref.signal_emitter(), namespace, key, value).await
    }

    pub async fn contrast_changed(&self, contrast: Contrast) -> zbus::Result<()> {
        self.changed(
            APPEARANCE_NAMESPACE,
            CONTRAST_KEY,
            OwnedValue::from(contrast).into(),
        )
        .await
    }

    pub async fn accent_color_changed(&self, color: Color) -> zbus::Result<()> {
        self.changed(
            APPEARANCE_NAMESPACE,
            ACCENT_COLOR_SCHEME_KEY,
            OwnedValue::try_from(color).unwrap().into(),
        )
        .await
    }

    pub async fn color_scheme_changed(&self, scheme: ColorScheme) -> zbus::Result<()> {
        self.changed(
            APPEARANCE_NAMESPACE,
            COLOR_SCHEME_KEY,
            OwnedValue::from(scheme).into(),
        )
        .await
    }
}

#[async_trait]
impl SettingsSignalEmitter for SettingsInterface {
    async fn emit_changed(&self, namespace: &str, key: &str, value: Value<'_>) -> zbus::Result<()> {
        self.changed(namespace, key, value).await
    }

    async fn emit_contrast_changed(&self, contrast: Contrast) -> zbus::Result<()> {
        self.contrast_changed(contrast).await
    }

    async fn emit_accent_color_changed(&self, color: Color) -> zbus::Result<()> {
        self.accent_color_changed(color).await
    }

    async fn emit_color_scheme_changed(&self, scheme: ColorScheme) -> zbus::Result<()> {
        self.color_scheme_changed(scheme).await
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Settings")]
impl SettingsInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        2
    }

    #[zbus(out_args("value"))]
    async fn read_all(
        &self,
        namespaces: Vec<String>,
    ) -> Result<HashMap<String, Namespace>, PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Settings::ReadAll");

        let response = self.imp.read_all(namespaces).await;

        #[cfg(feature = "tracing")]
        tracing::debug!("Settings::ReadAll returned {:#?}", response);
        response
    }

    #[zbus(out_args("value"))]
    async fn read(&self, namespace: &str, key: &str) -> Result<OwnedValue, PortalError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Settings::Read");

        let response = self.imp.read(namespace, key).await;

        #[cfg(feature = "tracing")]
        tracing::debug!("Settings::Read returned {:#?}", response);
        response
    }

    #[zbus(signal)]
    async fn setting_changed(
        signal_ctxt: &SignalEmitter<'_>,
        namespace: &str,
        key: &str,
        value: Value<'_>,
    ) -> zbus::Result<()>;
}
