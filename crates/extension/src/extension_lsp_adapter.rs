use crate::wasm_host::{wit::LanguageServerConfig, WasmExtension, WasmHost};
use anyhow::{anyhow, Result};
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
use wasmtime_wasi::preview2::WasiView as _;

pub struct ExtensionLspAdapter {
    pub(crate) extension: WasmExtension,
    pub(crate) config: LanguageServerConfig,
    pub(crate) host: Arc<WasmHost>,
}

#[async_trait]
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
}
