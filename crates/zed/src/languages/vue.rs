use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
pub use language::*;
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use smol::fs::{self, File};
use std::{any::Any, path::PathBuf, sync::Arc};
use util::{
    fs::remove_matching,
    github::{latest_github_release, GitHubLspBinaryVersion},
    ResultExt,
};

pub struct VueLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl VueLspAdapter {
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
        let release =
            latest_github_release("vuejs/language-tools", false, delegate.http_client()).await?;
        let version = GitHubLspBinaryVersion {
            name: release.name,
            url: release.zipball_url,
        };
        Ok(Box::new(version) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let zip_path = container_dir.join(format!("vuejs-language-tools {}.zip", version.name));
        dbg!(&zip_path);
        let version_dir = container_dir.join(format!("vuejs-language-tools-01a2e3e"));
        dbg!(&version_dir);
        let binary_path =
            version_dir.join("packages/vue-language-server/bin/vue-language-server.js");
        dbg!(&version_dir);

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .context("error downloading release")?;
            let mut file = File::create(&zip_path).await?;
            if !response.status().is_success() {
                Err(anyhow!(
                    "download failed with status {}",
                    response.status().to_string()
                ))?;
            }
            futures::io::copy(response.body_mut(), &mut file).await?;

            let unzip_status = smol::process::Command::new("unzip")
                .current_dir(&container_dir)
                .arg(&zip_path)
                .output()
                .await?
                .status;
            if !unzip_status.success() {
                Err(anyhow!("failed to unzip clangd archive"))?;
            }

            remove_matching(&container_dir, |entry| entry != version_dir).await;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            arguments: vec![binary_path.into_os_string()],
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

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: Arc<dyn NodeRuntime>,
) -> Option<LanguageServerBinary> {
    (|| async move {
        let mut last_clangd_dir = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_dir() {
                last_clangd_dir = Some(entry.path());
            }
        }
        let clangd_dir = last_clangd_dir.ok_or_else(|| anyhow!("no cached binary"))?;
        let clangd_bin = clangd_dir.join("packages/vue-language-server/bin/vue-language-server.js");
        if clangd_bin.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                arguments: vec![clangd_bin.into_os_string()],
            })
        } else {
            Err(anyhow!(
                "missing clangd binary in directory {:?}",
                clangd_dir
            ))
        }
    })()
    .await
    .log_err()
}
