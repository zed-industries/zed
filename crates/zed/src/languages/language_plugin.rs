use anyhow::{anyhow, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::lock::Mutex;
use gpui::executor::Background;
use language::{LanguageServerBinary, LanguageServerName, LspAdapter};
use plugin_runtime::{Plugin, PluginBinary, PluginBuilder, WasiFn};
use std::{any::Any, path::PathBuf, sync::Arc};
use util::http::HttpClient;
use util::ResultExt;

#[allow(dead_code)]
pub async fn new_json(executor: Arc<Background>) -> Result<PluginLspAdapter> {
    let plugin = PluginBuilder::new_default()?
        .host_function_async("command", |command: String| async move {
            let mut args = command.split(' ');
            let command = args.next().unwrap();
            smol::process::Command::new(command)
                .args(args)
                .output()
                .await
                .log_err()
                .map(|output| output.stdout)
        })?
        .init(PluginBinary::Precompiled(include_bytes!(
            "../../../../plugins/bin/json_language.wasm.pre",
        )))
        .await?;

    PluginLspAdapter::new(plugin, executor).await
}

pub struct PluginLspAdapter {
    name: WasiFn<(), String>,
    fetch_latest_server_version: WasiFn<(), Option<String>>,
    fetch_server_binary: WasiFn<(PathBuf, String), Result<LanguageServerBinary, String>>,
    cached_server_binary: WasiFn<PathBuf, Option<LanguageServerBinary>>,
    initialization_options: WasiFn<(), String>,
    language_ids: WasiFn<(), Vec<(String, String)>>,
    executor: Arc<Background>,
    runtime: Arc<Mutex<Plugin>>,
}

impl PluginLspAdapter {
    #[allow(unused)]
    pub async fn new(mut plugin: Plugin, executor: Arc<Background>) -> Result<Self> {
        Ok(Self {
            name: plugin.function("name")?,
            fetch_latest_server_version: plugin.function("fetch_latest_server_version")?,
            fetch_server_binary: plugin.function("fetch_server_binary")?,
            cached_server_binary: plugin.function("cached_server_binary")?,
            initialization_options: plugin.function("initialization_options")?,
            language_ids: plugin.function("language_ids")?,
            executor,
            runtime: Arc::new(Mutex::new(plugin)),
        })
    }
}

#[async_trait]
impl LspAdapter for PluginLspAdapter {
    async fn name(&self) -> LanguageServerName {
        let name: String = self
            .runtime
            .lock()
            .await
            .call(&self.name, ())
            .await
            .unwrap();
        LanguageServerName(name.into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let runtime = self.runtime.clone();
        let function = self.fetch_latest_server_version;
        self.executor
            .spawn(async move {
                let mut runtime = runtime.lock().await;
                let versions: Result<Option<String>> =
                    runtime.call::<_, Option<String>>(&function, ()).await;
                versions
                    .map_err(|e| anyhow!("{}", e))?
                    .ok_or_else(|| anyhow!("Could not fetch latest server version"))
                    .map(|v| Box::new(v) as Box<_>)
            })
            .await
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<LanguageServerBinary> {
        let version = *version.downcast::<String>().unwrap();
        let runtime = self.runtime.clone();
        let function = self.fetch_server_binary;
        self.executor
            .spawn(async move {
                let mut runtime = runtime.lock().await;
                let handle = runtime.attach_path(&container_dir)?;
                let result: Result<LanguageServerBinary, String> =
                    runtime.call(&function, (container_dir, version)).await?;
                runtime.remove_resource(handle)?;
                result.map_err(|e| anyhow!("{}", e))
            })
            .await
    }

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<LanguageServerBinary> {
        let runtime = self.runtime.clone();
        let function = self.cached_server_binary;

        self.executor
            .spawn(async move {
                let mut runtime = runtime.lock().await;
                let handle = runtime.attach_path(&container_dir).ok()?;
                let result: Option<LanguageServerBinary> =
                    runtime.call(&function, container_dir).await.ok()?;
                runtime.remove_resource(handle).ok()?;
                result
            })
            .await
    }

    async fn initialization_options(&self) -> Option<serde_json::Value> {
        let string: String = self
            .runtime
            .lock()
            .await
            .call(&self.initialization_options, ())
            .await
            .log_err()?;

        serde_json::from_str(&string).ok()
    }

    async fn language_ids(&self) -> HashMap<String, String> {
        self.runtime
            .lock()
            .await
            .call(&self.language_ids, ())
            .await
            .log_err()
            .unwrap_or_default()
            .into_iter()
            .collect()
    }
}
