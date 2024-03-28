use crate::wasm_host::{wit::LanguageServerConfig, WasmExtension, WasmHost};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::{Future, FutureExt};
use gpui::AsyncAppContext;
use language::{Language, LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use std::{
    any::Any,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use wasmtime_wasi::WasiView as _;

pub struct ExtensionLspAdapter {
    pub(crate) extension: WasmExtension,
    pub(crate) config: LanguageServerConfig,
    pub(crate) host: Arc<WasmHost>,
}

#[async_trait(?Send)]
impl LspAdapter for ExtensionLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(self.config.name.clone().into())
    }

    fn get_language_server_command<'a>(
        self: Arc<Self>,
        _: Arc<Language>,
        _: Arc<Path>,
        delegate: Arc<dyn LspAdapterDelegate>,
        _: futures::lock::MutexGuard<'a, Option<LanguageServerBinary>>,
        _: &'a mut AsyncAppContext,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<LanguageServerBinary>>>> {
        async move {
            let command = self
                .extension
                .call({
                    let this = self.clone();
                    |extension, store| {
                        async move {
                            let resource = store.data_mut().table().push(delegate)?;
                            let command = extension
                                .call_language_server_command(store, &this.config, resource)
                                .await?
                                .map_err(|e| anyhow!("{}", e))?;
                            anyhow::Ok(command)
                        }
                        .boxed()
                    }
                })
                .await?;

            let path = self
                .host
                .path_from_extension(&self.extension.manifest.id, command.command.as_ref());

            // TODO: Eventually we'll want to expose an extension API for doing this, but for
            // now we just manually set the file permissions for extensions that we know need it.
            if self.extension.manifest.id.as_ref() == "zig" {
                #[cfg(not(windows))]
                {
                    use std::fs::{self, Permissions};
                    use std::os::unix::fs::PermissionsExt;

                    fs::set_permissions(&path, Permissions::from_mode(0o755))
                        .context("failed to set file permissions")?;
                }
            }

            Ok(LanguageServerBinary {
                path,
                arguments: command.args.into_iter().map(|arg| arg.into()).collect(),
                env: Some(command.env.into_iter().collect()),
            })
        }
        .boxed_local()
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        unreachable!("get_language_server_command is overridden")
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        unreachable!("get_language_server_command is overridden")
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        unreachable!("get_language_server_command is overridden")
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        None
    }

    async fn initialization_options(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let delegate = delegate.clone();
        let json_options = self
            .extension
            .call({
                let this = self.clone();
                |extension, store| {
                    async move {
                        let resource = store.data_mut().table().push(delegate)?;
                        let options = extension
                            .call_language_server_initialization_options(
                                store,
                                &this.config,
                                resource,
                            )
                            .await?
                            .map_err(|e| anyhow!("{}", e))?;
                        anyhow::Ok(options)
                    }
                    .boxed()
                }
            })
            .await?;
        Ok(if let Some(json_options) = json_options {
            serde_json::from_str(&json_options).with_context(|| {
                format!("failed to parse initialization_options from extension: {json_options}")
            })?
        } else {
            None
        })
    }
}
