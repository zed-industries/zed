use super::installation::{npm_install_packages, npm_package_latest_version};
use anyhow::{anyhow, Context, Result};
use client::http::HttpClient;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use isahc::http::version;
use language::{LanguageServerName, LspAdapter};
use parking_lot::{Mutex, RwLock};
use plugin_runtime::{Wasi, WasiPlugin};
use serde_json::json;
use std::fs;
use std::{any::Any, path::PathBuf, sync::Arc};
use util::{ResultExt, TryFutureExt};

pub fn new_json() -> LanguagePluginLspAdapter {
    let plugin = WasiPlugin {
        module: include_bytes!("../../../../plugins/bin/json_language.wasm").to_vec(),
        wasi_ctx: Wasi::default_ctx(),
    };
    LanguagePluginLspAdapter::new(plugin)
}

pub struct LanguagePluginLspAdapter {
    runtime: Mutex<Wasi>,
}

impl LanguagePluginLspAdapter {
    pub fn new(plugin: WasiPlugin) -> Self {
        Self {
            runtime: Mutex::new(Wasi::init(plugin).unwrap()),
        }
    }
}

struct Versions {
    language_version: String,
    server_version: String,
}

impl LspAdapter for LanguagePluginLspAdapter {
    fn name(&self) -> LanguageServerName {
        let name: String = self.runtime.lock().call("name", ()).unwrap();
        LanguageServerName(name.into())
    }

    fn server_args<'a>(&'a self) -> Vec<String> {
        self.runtime.lock().call("server_args", ()).unwrap()
    }

    fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<Box<dyn 'static + Send + Any>>> {
        let versions: Result<(String, String)> =
            self.runtime.lock().call("fetch_latest_server_version", ());

        async move {
            versions.map(|(language_version, server_version)| {
                Box::new(Versions {
                    language_version,
                    server_version,
                }) as Box<_>
            })
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
        let mut runtime = self.runtime.lock();

        let result: Result<PathBuf, _> = (|| {
            let handle = runtime.attach_path(&container_dir)?;
            let result = runtime
                .call::<_, Option<PathBuf>>("fetch_server_binary", container_dir)?
                .ok_or_else(|| anyhow!("Could not load cached server binary"));
            runtime.remove_resource(handle)?;
            result
        })();

        async move { result }.boxed()
    }

    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>> {
        let mut runtime = self.runtime.lock();

        let result: Option<PathBuf> = (|| {
            let handle = runtime.attach_path(&container_dir).ok()?;
            let result = runtime
                .call::<_, Option<PathBuf>>("cached_server_binary", container_dir)
                .ok()?;
            runtime.remove_resource(handle).ok()?;
            result
        })();

        async move { result }.boxed()
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
        let name: String = self
            .runtime
            .lock()
            .call("label_for_completion", kind)
            .ok()?;
        let highlight_id = grammar.highlight_id_for_name(&name)?;
        Some(language::CodeLabel {
            text: item.label.clone(),
            runs: vec![(0..len, highlight_id)],
            filter_range: 0..len,
        })
    }

    fn initialization_options(&self) -> Option<serde_json::Value> {
        let string = self
            .runtime
            .lock()
            .call::<_, Option<String>>("initialization_options", ())
            .unwrap()?;

        serde_json::from_str(&string).ok()
    }
}
