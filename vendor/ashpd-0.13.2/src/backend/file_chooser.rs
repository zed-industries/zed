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
        file_chooser::{OpenFileOptions, SaveFileOptions, SaveFilesOptions, SelectedFiles},
        request::Response,
    },
    zvariant::{Optional, OwnedObjectPath},
};

#[async_trait]
pub trait FileChooserImpl: RequestImpl {
    #[doc(alias = "OpenFile")]
    async fn open_file(
        &self,
        token: HandleToken,
        app_id: Option<AppID>,
        window_identifier: Option<WindowIdentifierType>,
        title: &str,
        options: OpenFileOptions,
    ) -> Result<SelectedFiles>;

    #[doc(alias = "SaveFile")]
    async fn save_file(
        &self,
        token: HandleToken,
        app_id: Option<AppID>,
        window_identifier: Option<WindowIdentifierType>,
        title: &str,
        options: SaveFileOptions,
    ) -> Result<SelectedFiles>;

    #[doc(alias = "SaveFiles")]
    async fn save_files(
        &self,
        token: HandleToken,
        app_id: Option<AppID>,
        window_identifier: Option<WindowIdentifierType>,
        title: &str,
        options: SaveFilesOptions,
    ) -> Result<SelectedFiles>;
}

pub(crate) struct FileChooserInterface {
    imp: Arc<dyn FileChooserImpl>,
    spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    cnx: zbus::Connection,
}

impl FileChooserInterface {
    pub fn new(
        imp: Arc<dyn FileChooserImpl>,
        cnx: zbus::Connection,
        spawn: Arc<dyn futures_util::task::Spawn + Send + Sync>,
    ) -> Self {
        Self { imp, cnx, spawn }
    }
}

#[zbus::interface(name = "org.freedesktop.impl.portal.FileChooser")]
impl FileChooserInterface {
    #[zbus(property(emits_changed_signal = "const"), name = "version")]
    fn version(&self) -> u32 {
        4
    }

    #[zbus(out_args("response", "results"))]
    async fn open_file(
        &self,
        handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        window_identifier: Optional<WindowIdentifierType>,
        title: String,
        options: OpenFileOptions,
    ) -> Result<Response<SelectedFiles>> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "FileChooser::OpenFile",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.open_file(
                    HandleToken::try_from(&handle).unwrap(),
                    app_id.into(),
                    window_identifier.into(),
                    &title,
                    options,
                )
                .await
            },
        )
        .await
    }

    #[zbus(out_args("response", "results"))]
    async fn save_file(
        &self,
        handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        window_identifier: Optional<WindowIdentifierType>,
        title: String,
        options: SaveFileOptions,
    ) -> Result<Response<SelectedFiles>> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "FileChooser::SaveFile",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.save_file(
                    HandleToken::try_from(&handle).unwrap(),
                    app_id.into(),
                    window_identifier.into(),
                    &title,
                    options,
                )
                .await
            },
        )
        .await
    }

    #[zbus(out_args("response", "results"))]
    async fn save_files(
        &self,
        handle: OwnedObjectPath,
        app_id: Optional<AppID>,
        window_identifier: Optional<WindowIdentifierType>,
        title: String,
        options: SaveFilesOptions,
    ) -> Result<Response<SelectedFiles>> {
        let imp = Arc::clone(&self.imp);

        Request::spawn(
            "FileChooser::SaveFiles",
            &self.cnx,
            handle.clone(),
            Arc::clone(&self.imp),
            Arc::clone(&self.spawn),
            async move {
                imp.save_files(
                    HandleToken::try_from(&handle).unwrap(),
                    app_id.into(),
                    window_identifier.into(),
                    &title,
                    options,
                )
                .await
            },
        )
        .await
    }
}
