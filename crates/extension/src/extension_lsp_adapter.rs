use crate::wasm_host::{wit::LanguageServerConfig, WasmExtension, WasmHost};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use collections::HashMap;
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

            // TODO: This should now be done via the `zed::make_file_executable` function in
            // Zed extension API, but we're leaving these existing usages in place temporarily
            // to avoid any compatibility issues between Zed and the extension versions.
            //
            // We can remove once the following extension versions no longer see any use:
            // - toml@0.0.2
            // - zig@0.0.1
            if ["toml", "zig"].contains(&self.extension.manifest.id.as_ref())
                && path.starts_with(&self.host.work_dir)
            {
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

    fn language_ids(&self) -> HashMap<String, String> {
        // TODO: The language IDs can be provided via the language server options
        // in `extension.toml now but we're leaving these existing usages in place temporarily
        // to avoid any compatibility issues between Zed and the extension versions.
        //
        // We can remove once the following extension versions no longer see any use:
        // - php@0.0.1
        if self.extension.manifest.id.as_ref() == "php" {
            return HashMap::from_iter([("PHP".into(), "php".into())]);
        }

        self.extension
            .manifest
            .language_servers
            .get(&LanguageServerName(self.config.name.clone().into()))
            .map(|server| server.language_ids.clone())
            .unwrap_or_default()
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
