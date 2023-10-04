use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
pub use language::*;
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use serde_json::Value;
use smol::fs::{self, File};
use std::{
    any::Any,
    cell::OnceCell,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{
    fs::remove_matching,
    github::{latest_github_release, GitHubLspBinaryVersion},
    ResultExt,
};

pub struct VueLspVersion {
    vue_version: String,
    ts_version: String,
}

pub struct VueLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl VueLspAdapter {
    const SERVER_PATH: &'static str =
        "node_modules/@vue/language-server/bin/vue-language-server.js";
    // TODO: this can't be hardcoded, yet we have to figure out how to pass it in initialization_options.
    const TYPESCRIPT_PATH: &'static str = "/Users/hiro/Library/Application Support/Zed/languages/vue-language-server/node_modules/typescript/lib"; //"node_modules/@vue/typescript/lib";
    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        Self { node }
    }
}
#[async_trait]
impl super::LspAdapter for VueLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("vue-language-server".into())
    }

    fn short_name(&self) -> &'static str {
        "vue-language-server"
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(VueLspVersion {
            vue_version: self
                .node
                .npm_package_latest_version("@vue/language-server")
                .await?,
            ts_version: self.node.npm_package_latest_version("typescript").await?,
        }) as Box<_>)
    }
    async fn initialization_options(&self) -> Option<Value> {
        Some(serde_json::json!({
            "typescript": {
                "tsdk": Self::TYPESCRIPT_PATH//
            }
        }))
    }
    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<VueLspVersion>().unwrap();
        let server_path = container_dir.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[("@vue/language-server", version.vue_version.as_str())],
                )
                .await?;
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[("typescript", version.ts_version.as_str())],
                )
                .await?;
        }
        assert!(fs::metadata(&server_path).await.is_ok());
        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            arguments: vue_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, self.node.clone()).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, self.node.clone())
            .await
            .map(|mut binary| {
                binary.arguments = vec!["--help".into()];
                binary
            })
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        None
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        None
    }
}

fn vue_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: Arc<dyn NodeRuntime>,
) -> Option<LanguageServerBinary> {
    (|| async move {
        let mut last_version_dir = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_dir() {
                last_version_dir = Some(entry.path());
            }
        }
        let last_version_dir = last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
        let server_path = last_version_dir.join(VueLspAdapter::SERVER_PATH);
        if server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                arguments: vue_server_binary_arguments(&server_path),
            })
        } else {
            Err(anyhow!(
                "missing executable in directory {:?}",
                last_version_dir
            ))
        }
    })()
    .await
    .log_err()
}
