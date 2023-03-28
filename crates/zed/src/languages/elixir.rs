use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
pub use language::*;
use lsp::{CompletionItemKind, SymbolKind};
use smol::fs::{self, File};
use std::{any::Any, path::PathBuf, sync::Arc};
use util::fs::remove_matching;
use util::github::latest_github_release;
use util::http::HttpClient;
use util::ResultExt;

use util::github::GitHubLspBinaryVersion;

pub struct ElixirLspAdapter;

#[async_trait]
impl LspAdapter for ElixirLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("elixir-ls".into())
    }

    async fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release("elixir-lsp/elixir-ls", http).await?;
        let asset_name = "elixir-ls.zip";
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
        let version = GitHubLspBinaryVersion {
            name: release.name,
            url: asset.browser_download_url.clone(),
        };
        Ok(Box::new(version) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        http: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let zip_path = container_dir.join(format!("elixir-ls_{}.zip", version.name));
        let version_dir = container_dir.join(format!("elixir-ls_{}", version.name));
        let binary_path = version_dir.join("language_server.sh");

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = http
                .get(&version.url, Default::default(), true)
                .await
                .context("error downloading release")?;
            let mut file = File::create(&zip_path)
                .await
                .with_context(|| format!("failed to create file {}", zip_path.display()))?;
            if !response.status().is_success() {
                Err(anyhow!(
                    "download failed with status {}",
                    response.status().to_string()
                ))?;
            }
            futures::io::copy(response.body_mut(), &mut file).await?;

            fs::create_dir_all(&version_dir)
                .await
                .with_context(|| format!("failed to create directory {}", version_dir.display()))?;
            let unzip_status = smol::process::Command::new("unzip")
                .arg(&zip_path)
                .arg("-d")
                .arg(&version_dir)
                .output()
                .await?
                .status;
            if !unzip_status.success() {
                Err(anyhow!("failed to unzip clangd archive"))?;
            }

            remove_matching(&container_dir, |entry| entry != version_dir).await;
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            arguments: vec![],
        })
    }

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<LanguageServerBinary> {
        (|| async move {
            let mut last = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                last = Some(entry?.path());
            }
            last.map(|path| LanguageServerBinary {
                path,
                arguments: vec![],
            })
            .ok_or_else(|| anyhow!("no cached binary"))
        })()
        .await
        .log_err()
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        match completion.kind.zip(completion.detail.as_ref()) {
            Some((_, detail)) if detail.starts_with("(function)") => {
                let text = detail.strip_prefix("(function) ")?;
                let filter_range = 0..text.find('(').unwrap_or(text.len());
                let source = Rope::from(format!("def {text}").as_str());
                let runs = language.highlight_text(&source, 4..4 + text.len());
                return Some(CodeLabel {
                    text: text.to_string(),
                    runs,
                    filter_range,
                });
            }
            Some((_, detail)) if detail.starts_with("(macro)") => {
                let text = detail.strip_prefix("(macro) ")?;
                let filter_range = 0..text.find('(').unwrap_or(text.len());
                let source = Rope::from(format!("defmacro {text}").as_str());
                let runs = language.highlight_text(&source, 9..9 + text.len());
                return Some(CodeLabel {
                    text: text.to_string(),
                    runs,
                    filter_range,
                });
            }
            Some((
                CompletionItemKind::CLASS
                | CompletionItemKind::MODULE
                | CompletionItemKind::INTERFACE
                | CompletionItemKind::STRUCT,
                _,
            )) => {
                let filter_range = 0..completion
                    .label
                    .find(" (")
                    .unwrap_or(completion.label.len());
                let text = &completion.label[filter_range.clone()];
                let source = Rope::from(format!("defmodule {text}").as_str());
                let runs = language.highlight_text(&source, 10..10 + text.len());
                return Some(CodeLabel {
                    text: completion.label.clone(),
                    runs,
                    filter_range,
                });
            }
            _ => {}
        }

        None
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            SymbolKind::METHOD | SymbolKind::FUNCTION => {
                let text = format!("def {}", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            SymbolKind::CLASS | SymbolKind::MODULE | SymbolKind::INTERFACE | SymbolKind::STRUCT => {
                let text = format!("defmodule {}", name);
                let filter_range = 10..10 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            runs: language.highlight_text(&text.as_str().into(), display_range.clone()),
            text: text[display_range].to_string(),
            filter_range,
        })
    }
}
