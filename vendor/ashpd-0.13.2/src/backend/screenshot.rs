use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    AppID, WindowIdentifierType,
    backend::{
        Result,
        request::{Request, RequestImpl},
    },
    desktop::{
        Color, HandleToken,
        request::Response,
        screenshot::{ColorOptions, Screenshot as ScreenshotResponse, ScreenshotOptions},
    },
    zvariant::{Optional, OwnedObjectPath},
};

#[async_trait]
pub trait ScreenshotImpl: RequestImpl {
    #[doc(alias = "Screenshot")]
    async fn screenshot(
        &self,
        token: HandleToken,
        app_id: Option<AppID>,
        window_identifier: Option<WindowIdentifierType>,
        options: ScreenshotOptions,
    ) -> Result<ScreenshotResponse>;

    #[doc(alias = "PickColor")]
    async fn pick_color(
        &self,
        token: HandleToken,
        app_id: Option<AppID>,
        window_identifier: Option<WindowIdentifierType>,
        options: ColorOptions,
    ) -> Result<Color>;
}

pub(crate) struct ScreenshotInterface {
    imp: Arc<dyn ScreenshotImpl>,
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    cnx: zbus::Connection,
}

impl ScreenshotInterface {
    pub fn new(
        imp: Arc<dyn ScreenshotImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    ) -> Self {
        Self { imp, cnx, spawn }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.Screenshot")]
impl ScreenshotInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        2
    }

    #[zbus(name = "Screenshot")]
    #[zbus(out_args("response", "results"))]
    async fn screenshot(
        &self,
        handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        window_identifier: Optional<WindowIdentifierType>,
        options: ScreenshotOptions,
    ) -> Result<Response<ScreenshotResponse>> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "Screenshot::Screenshot",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.screenshot(
                    HandleToken::try_from(&handle).unwrap(),
                    app_id.into(),
                    window_identifier.into(),
                    options,
                )
                .await
            },
        )
        .await
    }

    #[zbus(name = "PickColor")]
    #[zbus(out_args("response", "results"))]
    async fn pick_color(
        &self,
        handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        window_identifier: Optional<WindowIdentifierType>,
        options: ColorOptions,
    ) -> Result<Response<Color>> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "Screenshot::PickColor",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.pick_color(
                    HandleToken::try_from(&handle).unwrap(),
                    app_id.into(),
                    window_identifier.into(),
                    options,
                )
                .await
            },
        )
        .await
    }
}
