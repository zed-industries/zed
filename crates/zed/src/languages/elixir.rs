use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use gpui::{AsyncAppContext, Task};
pub use language::*;
use lsp::{CompletionItemKind, LanguageServerBinary, SymbolKind};
use smol::fs::{self, File};
use std::{
    any::Any,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
use util::{
    fs::remove_matching,
    github::{latest_github_release, GitHubLspBinaryVersion},
    ResultExt,
};

pub struct ElixirLspAdapter;

#[async_trait]
impl LspAdapter for ElixirLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("elixir-ls".into())
    }

    fn will_start_server(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Option<Task<Result<()>>> {
        static DID_SHOW_NOTIFICATION: AtomicBool = AtomicBool::new(false);

        const NOTIFICATION_MESSAGE: &str = "Could not run the elixir language server, `elixir-ls`, because `elixir` was not found.";

        let delegate = delegate.clone();
        Some(cx.spawn(|mut cx| async move {
            let elixir_output = smol::process::Command::new("elixir")
                .args(["--version"])
                .output()
                .await;
            if elixir_output.is_err() {
                if DID_SHOW_NOTIFICATION
                    .compare_exchange(false, true, SeqCst, SeqCst)
                    .is_ok()
                {
                    cx.update(|cx| {
                        delegate.show_notification(NOTIFICATION_MESSAGE, cx);
                    })
                }
                return Err(anyhow!("cannot run elixir-ls"));
            }

            Ok(())
        }))
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let http = delegate.http_client();
        let release = latest_github_release("elixir-lsp/elixir-ls", false, http).await?;
        let version_name = release
            .name
            .strip_prefix("Release ")
            .context("Elixir-ls release name does not start with prefix")?
            .to_owned();

        let asset_name = format!("elixir-ls-{}.zip", &version_name);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;

        let version = GitHubLspBinaryVersion {
            name: version_name,
            url: asset.browser_download_url.clone(),
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
        let zip_path = container_dir.join(format!("elixir-ls_{}.zip", version.name));
        let version_dir = container_dir.join(format!("elixir-ls_{}", version.name));
        let binary_path = version_dir.join("language_server.sh");

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = delegate
                .http_client()
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
                Err(anyhow!("failed to unzip elixir-ls archive"))?;
            }

            remove_matching(&container_dir, |entry| entry != version_dir).await;
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            arguments: vec![],
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir).await
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

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
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
