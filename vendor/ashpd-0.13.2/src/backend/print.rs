use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    AppID, WindowIdentifierType,
    backend::{
        Result,
        request::{Request, RequestImpl},
    },
    desktop::{
        HandleToken,
        print::{PageSetup, PreparePrint, PreparePrintOptions, PrintOptions, Settings},
        request::Response,
    },
    zvariant::{self, Optional, OwnedObjectPath},
};

#[async_trait]
pub trait PrintImpl: RequestImpl {
    #[allow(clippy::too_many_arguments)]
    #[doc(alias = "PreparePrint")]
    async fn prepare_print(
        &self,
        token: HandleToken,
        app_id: Option<AppID>,
        parent_window: Option<WindowIdentifierType>,
        title: String,
        settings: Settings,
        page_setup: PageSetup,
        options: PreparePrintOptions,
    ) -> Result<PreparePrint>;

    #[doc(alias = "Print")]
    async fn print(
        &self,
        token: HandleToken,
        app_id: Option<AppID>,
        parent_window: Option<WindowIdentifierType>,
        title: String,
        fd: zvariant::OwnedFd,
        options: PrintOptions,
    ) -> Result<()>;
}

pub(crate) struct PrintInterface {
    imp: Arc<dyn PrintImpl>,
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    cnx: zbus::Connection,
}

impl PrintInterface {
    pub fn new(
        imp: Arc<dyn PrintImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    ) -> Self {
        Self { imp, cnx, spawn }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Print")]
impl PrintInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        3
    }

    #[allow(clippy::too_many_arguments)]
    #[zbus(out_args("response", "results"))]
    async fn prepare_print(
        &self,
        handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        window_identifier: Optional<WindowIdentifierType>,
        title: String,
        settings: Settings,
        page_setup: PageSetup,
        options: PreparePrintOptions,
    ) -> Result<Response<PreparePrint>> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "Print::PreparePrint",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.prepare_print(
                    HandleToken::try_from(&handle).unwrap(),
                    app_id.into(),
                    window_identifier.into(),
                    title,
                    settings,
                    page_setup,
                    options,
                )
                .await
            },
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    #[zbus(out_args("response", "results"))]
    async fn print(
        &self,
        handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        window_identifier: Optional<WindowIdentifierType>,
        title: String,
        fd: zvariant::OwnedFd,
        options: PrintOptions,
    ) -> Result<Response<()>> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "Print::Print",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.print(
                    HandleToken::try_from(&handle).unwrap(),
                    app_id.into(),
                    window_identifier.into(),
                    title,
                    fd,
                    options,
                )
                .await
            },
        )
        .await
    }
}
