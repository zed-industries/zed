use super::installation::{npm_install_packages, npm_package_latest_version};
use anyhow::{anyhow, Context, Result};
use client::http::HttpClient;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use language::{LanguageServerName, LspAdapter};
use parking_lot::{Mutex, RwLock};
use plugin_runtime::{Runtime, Wasm, WasmPlugin};
use serde_json::json;
use smol::fs;
use std::{any::Any, path::PathBuf, sync::Arc};
use util::{ResultExt, TryFutureExt};

pub fn new_json() {}

pub struct LanguagePluginLspAdapter {
    runtime: Mutex<Wasm<()>>,
}

impl LanguagePluginLspAdapter {
    pub fn new(plugin: WasmPlugin<()>) -> Self {
        Self {
            runtime: Mutex::new(Wasm::init(plugin).unwrap()),
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
        self.runtime.lock().call("args", ()).unwrap()
    }

    fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<Box<dyn 'static + Send + Any>>> {
        let versions: Result<(String, String)> =
            self.runtime.lock().call("fetch_latest_server_version", ());

        async move {
            if let Ok((language_version, server_version)) = versions {
                Ok(Box::new(Versions {
                    language_version,
                    server_version,
                }) as Box<_>)
            } else {
                panic!()
            }
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        versions: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        // TODO: async runtime
        let result = self
            .runtime
            .lock()
            .call("fetch_server_binary", container_dir);
        async move { result }.boxed()
    }

    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>> {
        let result = self
            .runtime
            .lock()
            .call("cached_server_binary", container_dir);
        async move { result }.log_err().boxed()
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
        let result = self
            .runtime
            .lock()
            .call("initialization_options", ())
            .unwrap();
        Some(result)
    }
}
