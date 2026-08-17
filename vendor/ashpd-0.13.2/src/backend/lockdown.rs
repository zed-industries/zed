use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait LockdownImpl: Send + Sync {
    #[doc(alias = "disable-printing")]
    async fn disable_printing(&self) -> bool;
    #[doc(alias = "disable-printing")]
    async fn set_disable_printing(&self, disable_printing: bool) -> zbus::Result<()>;

    #[doc(alias = "disable-save-to-disk")]
    async fn disable_save_to_disk(&self) -> bool;
    #[doc(alias = "disable-save-to-disk")]
    async fn set_disable_save_to_disk(&self, disable_save_to_disk: bool) -> zbus::Result<()>;

    #[doc(alias = "disable-application-handlers")]
    async fn disable_application_handlers(&self) -> bool;
    #[doc(alias = "disable-application-handlers")]
    async fn set_disable_application_handlers(
        &self,
        disable_application_handlers: bool,
    ) -> zbus::Result<()>;

    #[doc(alias = "disable-location")]
    async fn disable_location(&self) -> bool;
    #[doc(alias = "disable-location")]
    async fn set_disable_location(&self, disable_location: bool) -> zbus::Result<()>;

    #[doc(alias = "disable-camera")]
    async fn disable_camera(&self) -> bool;
    #[doc(alias = "disable-camera")]
    async fn set_disable_camera(&self, disable_camera: bool) -> zbus::Result<()>;

    #[doc(alias = "disable-microphone")]
    async fn disable_microphone(&self) -> bool;
    #[doc(alias = "disable-microphone")]
    async fn set_disable_microphone(&self, disable_microphone: bool) -> zbus::Result<()>;

    #[doc(alias = "disable-sound-output")]
    async fn disable_sound_output(&self) -> bool;
    #[doc(alias = "disable-sound-output")]
    async fn set_disable_sound_output(&self, disable_sound_output: bool) -> zbus::Result<()>;
}

pub(crate) struct LockdownInterface {
    imp: Arc<dyn LockdownImpl>,
    #[allow(dead_code)]
    cnx: zbus::Connection,
    #[allow(dead_code)]
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
}

impl LockdownInterface {
    pub fn new(
        imp: Arc<dyn LockdownImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    ) -> Self {
        Self { imp, cnx, spawn }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Lockdown")]
impl LockdownInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        1
    }

    #[zbus(property, name = "disable-printing")]
    async fn disable_printing(&self) -> bool {
        self.imp.disable_printing().await
    }

    #[zbus(property, name = "disable-printing")]
    async fn set_disable_printing(&self, disable_printing: bool) -> zbus::Result<()> {
        let object_server = self.cnx.object_server();
        let iface_ref = object_server
            .interface::<_, Self>(crate::proxy::DESKTOP_PATH)
            .await?;
        let ctxt = iface_ref.signal_emitter();

        self.imp.set_disable_printing(disable_printing).await?;
        self.disable_printing_changed(ctxt).await?;
        Ok(())
    }

    #[zbus(property, name = "disable-save-to-disk")]
    async fn disable_save_to_disk(&self) -> bool {
        self.imp.disable_save_to_disk().await
    }

    #[zbus(property, name = "disable-save-to-disk")]
    async fn set_disable_save_to_disk(&self, disable_save_to_disk: bool) -> zbus::Result<()> {
        let object_server = self.cnx.object_server();
        let iface_ref = object_server
            .interface::<_, Self>(crate::proxy::DESKTOP_PATH)
            .await?;
        let ctxt = iface_ref.signal_emitter();

        self.imp
            .set_disable_save_to_disk(disable_save_to_disk)
            .await?;
        self.disable_save_to_disk_changed(ctxt).await?;
        Ok(())
    }

    #[zbus(property, name = "disable-application-handlers")]
    async fn disable_application_handlers(&self) -> bool {
        self.imp.disable_application_handlers().await
    }

    #[zbus(property, name = "disable-application-handlers")]
    async fn set_disable_application_handlers(
        &self,
        disable_application_handlers: bool,
    ) -> zbus::Result<()> {
        let object_server = self.cnx.object_server();
        let iface_ref = object_server
            .interface::<_, Self>(crate::proxy::DESKTOP_PATH)
            .await?;
        let ctxt = iface_ref.signal_emitter();

        self.imp
            .set_disable_application_handlers(disable_application_handlers)
            .await?;
        self.disable_application_handlers_changed(ctxt).await?;
        Ok(())
    }

    #[zbus(property, name = "disable-location")]
    async fn disable_location(&self) -> bool {
        self.imp.disable_location().await
    }

    #[zbus(property, name = "disable-location")]
    async fn set_disable_location(&self, disable_location: bool) -> zbus::Result<()> {
        let object_server = self.cnx.object_server();
        let iface_ref = object_server
            .interface::<_, Self>(crate::proxy::DESKTOP_PATH)
            .await?;
        let ctxt = iface_ref.signal_emitter();

        self.imp.set_disable_location(disable_location).await?;
        self.disable_location_changed(ctxt).await?;
        Ok(())
    }

    #[zbus(property, name = "disable-camera")]
    async fn disable_camera(&self) -> bool {
        self.imp.disable_camera().await
    }

    #[zbus(property, name = "disable-camera")]
    async fn set_disable_camera(&self, disable_camera: bool) -> zbus::Result<()> {
        let object_server = self.cnx.object_server();
        let iface_ref = object_server
            .interface::<_, Self>(crate::proxy::DESKTOP_PATH)
            .await?;
        let ctxt = iface_ref.signal_emitter();

        self.imp.set_disable_camera(disable_camera).await?;
        self.disable_camera_changed(ctxt).await?;
        Ok(())
    }

    #[zbus(property, name = "disable-microphone")]
    async fn disable_microphone(&self) -> bool {
        self.imp.disable_microphone().await
    }

    #[zbus(property, name = "disable-microphone")]
    async fn set_disable_microphone(&self, disable_microphone: bool) -> zbus::Result<()> {
        let object_server = self.cnx.object_server();
        let iface_ref = object_server
            .interface::<_, Self>(crate::proxy::DESKTOP_PATH)
            .await?;
        let ctxt = iface_ref.signal_emitter();

        self.imp.set_disable_microphone(disable_microphone).await?;
        self.disable_microphone_changed(ctxt).await?;
        Ok(())
    }

    #[zbus(property, name = "disable-sound-output")]
    async fn disable_sound_output(&self) -> bool {
        self.imp.disable_sound_output().await
    }

    #[zbus(property, name = "disable-sound-output")]
    async fn set_disable_sound_output(&self, disable_sound_output: bool) -> zbus::Result<()> {
        let object_server = self.cnx.object_server();
        let iface_ref = object_server
            .interface::<_, Self>(crate::proxy::DESKTOP_PATH)
            .await?;
        let ctxt = iface_ref.signal_emitter();

        self.imp
            .set_disable_sound_output(disable_sound_output)
            .await?;
        self.disable_sound_output_changed(ctxt).await?;
        Ok(())
    }
}
