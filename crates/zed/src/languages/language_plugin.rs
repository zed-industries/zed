use anyhow::{anyhow, Result};
use client::http::HttpClient;
use futures::lock::Mutex;
use futures::{future::BoxFuture, FutureExt};
use gpui::executor::Background;
use language::{LanguageServerName, LspAdapter};
use plugin_runtime::{Wasi, WasiFn, WasiPluginBuilder};
use std::{any::Any, path::PathBuf, sync::Arc};
use util::ResultExt;

pub async fn new_json(executor: Arc<Background>) -> Result<PluginLspAdapter> {
    let plugin = WasiPluginBuilder::new_with_default_ctx()?
        .host_function("command", |command: String| {
            // TODO: actual thing
            dbg!(&command);
            let mut args = command.split(' ');
            let command = args.next().unwrap();
            std::process::Command::new(command)
                .args(args)
                .output()
                .log_err()
                .map(|output| dbg!(output.stdout))
        })?
        .init(include_bytes!("../../../../plugins/bin/json_language.wasm"))
        .await?;
    PluginLspAdapter::new(plugin, executor).await
}

pub struct PluginLspAdapter {
    name: WasiFn<(), String>,
    server_args: WasiFn<(), Vec<String>>,
    fetch_latest_server_version: WasiFn<(), Option<String>>,
    fetch_server_binary: WasiFn<(PathBuf, String), Result<PathBuf, String>>,
    cached_server_binary: WasiFn<PathBuf, Option<PathBuf>>,
    label_for_completion: WasiFn<String, Option<String>>,
    initialization_options: WasiFn<(), String>,
    executor: Arc<Background>,
    runtime: Arc<Mutex<Wasi>>,
}

impl PluginLspAdapter {
    pub async fn new(mut plugin: Wasi, executor: Arc<Background>) -> Result<Self> {
        Ok(Self {
            name: plugin.function("name")?,
            server_args: plugin.function("server_args")?,
            fetch_latest_server_version: plugin.function("fetch_latest_server_version")?,
            fetch_server_binary: plugin.function("fetch_server_binary")?,
            cached_server_binary: plugin.function("cached_server_binary")?,
            label_for_completion: plugin.function("label_for_completion")?,
            initialization_options: plugin.function("initialization_options")?,
            executor,
            runtime: Arc::new(Mutex::new(plugin)),
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
        let name: String = call_block!(self, &self.name, ()).unwrap();
        LanguageServerName(name.into())
    }

    fn server_args<'a>(&'a self) -> Vec<String> {
        call_block!(self, &self.server_args, ()).unwrap()
    }

    fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<Box<dyn 'static + Send + Any>>> {
        // let versions: Result<Option<String>> = call_block!(self, "fetch_latest_server_version", ());
        let runtime = self.runtime.clone();
        let function = self.fetch_latest_server_version;
        async move {
            let mut runtime = runtime.lock().await;
            let versions: Result<Option<String>> =
                runtime.call::<_, Option<String>>(&function, ()).await;
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
        let version = *version.downcast::<String>().unwrap();
        let runtime = self.runtime.clone();
        let function = self.fetch_server_binary;
        async move {
            let mut runtime = runtime.lock().await;
            let handle = runtime.attach_path(&container_dir)?;
            let result: Result<PathBuf, String> =
                runtime.call(&function, (container_dir, version)).await?;
            runtime.remove_resource(handle)?;
            result.map_err(|e| anyhow!("{}", e))
        }
        .boxed()
    }

    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>> {
        let runtime = self.runtime.clone();
        let function = self.cached_server_binary;

        async move {
            let mut runtime = runtime.lock().await;
            let handle = runtime.attach_path(&container_dir).ok()?;
            let result: Option<PathBuf> = runtime.call(&function, container_dir).await.ok()?;
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
        // TODO: Push more of this method down into the plugin.
        use lsp::CompletionItemKind as Kind;
        let len = item.label.len();
        let grammar = language.grammar()?;
        let kind = format!("{:?}", item.kind?);
        let name: String = call_block!(self, &self.label_for_completion, kind).log_err()??;
        let highlight_id = grammar.highlight_id_for_name(&name)?;
        Some(language::CodeLabel {
            text: item.label.clone(),
            runs: vec![(0..len, highlight_id)],
            filter_range: 0..len,
        })
    }

    fn initialization_options(&self) -> Option<serde_json::Value> {
        let string: String = call_block!(self, &self.initialization_options, ()).log_err()?;

        serde_json::from_str(&string).ok()
    }
}
