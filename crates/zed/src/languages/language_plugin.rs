use super::installation::{npm_install_packages, npm_package_latest_version};
use anyhow::{anyhow, Context, Result};
use client::http::HttpClient;
use futures::lock::Mutex;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use gpui::executor::{self, Background};
use isahc::http::version;
use language::{LanguageServerName, LspAdapter};
use plugin_runtime::{Wasi, WasiPlugin};
use serde_json::json;
use std::fs;
use std::{any::Any, path::PathBuf, sync::Arc};
use util::{ResultExt, TryFutureExt};

pub async fn new_json(executor: Arc<Background>) -> Result<PluginLspAdapter> {
    let plugin = WasiPlugin {
        module: include_bytes!("../../../../plugins/bin/json_language.wasm").to_vec(),
        wasi_ctx: Wasi::default_ctx(),
    };
    PluginLspAdapter::new(plugin, executor).await
}

pub struct PluginLspAdapter {
    runtime: Arc<Mutex<Wasi>>,
    executor: Arc<Background>,
}

impl PluginLspAdapter {
    pub async fn new(plugin: WasiPlugin, executor: Arc<Background>) -> Result<Self> {
        Ok(Self {
            runtime: Arc::new(Mutex::new(Wasi::init(plugin).await?)),
            executor,
        })
    }
}

struct Versions {
    language_version: String,
    server_version: String,
}

macro_rules! call_block {
    ($self:ident, $name:expr, $arg:expr) => {
        $self
            .executor
            .block(async { $self.runtime.lock().await.call($name, $arg).await })
    };
}

impl LspAdapter for PluginLspAdapter {
    fn name(&self) -> LanguageServerName {
        let name: String = call_block!(self, "name", ()).unwrap();
        LanguageServerName(name.into())
    }

    fn server_args<'a>(&'a self) -> Vec<String> {
        call_block!(self, "server_args", ()).unwrap()
    }

    fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<Box<dyn 'static + Send + Any>>> {
        // let versions: Result<Option<String>> = call_block!(self, "fetch_latest_server_version", ());
        let runtime = self.runtime.clone();
        async move {
            let mut runtime = runtime.lock().await;
            let versions: Result<Option<String>> = runtime
                .call::<_, Option<String>>("fetch_latest_server_version", ())
                .await;
            versions
                .map_err(|e| anyhow!("{}", e))?
                .ok_or_else(|| anyhow!("Could not fetch latest server version"))
                .map(|v| Box::new(v) as Box<_>)
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        let version = version.downcast::<String>().unwrap();
        let runtime = self.runtime.clone();

        async move {
            let mut runtime = runtime.lock().await;
            let handle = runtime.attach_path(&container_dir)?;
            let result: Option<PathBuf> = runtime
                .call("fetch_server_binary", (container_dir, version))
                .await?;
            runtime.remove_resource(handle)?;
            result.ok_or_else(|| anyhow!("Could not load cached server binary"))
        }
        .boxed()
    }

    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>> {
        let runtime = self.runtime.clone();

        async move {
            let mut runtime = runtime.lock().await;
            let handle = runtime.attach_path(&container_dir).ok()?;
            let result: Option<PathBuf> = runtime
                .call("cached_server_binary", container_dir)
                .await
                .ok()?;
            runtime.remove_resource(handle).ok()?;
            result
        }
        .boxed()
    }

    fn process_diagnostics(&self, _: &mut lsp::PublishDiagnosticsParams) {}

    fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &language::Language,
    ) -> Option<language::CodeLabel> {
        use lsp::CompletionItemKind as Kind;
        let len = item.label.len();
        let grammar = language.grammar()?;
        let kind = format!("{:?}", item.kind?);
        let name: String = call_block!(self, "label_for_completion", kind).log_err()?;
        let highlight_id = grammar.highlight_id_for_name(&name)?;
        Some(language::CodeLabel {
            text: item.label.clone(),
            runs: vec![(0..len, highlight_id)],
            filter_range: 0..len,
        })
    }

    fn initialization_options(&self) -> Option<serde_json::Value> {
        let string: String = call_block!(self, "initialization_options", ()).log_err()?;

        serde_json::from_str(&string).ok()
    }
}
