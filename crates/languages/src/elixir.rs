use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use gpui::{AppContext, AsyncAppContext, Task};
pub use language::*;
use lsp::{CompletionItemKind, LanguageServerBinary, SymbolKind};
use project::project_settings::ProjectSettings;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use settings::Settings;
use smol::fs::{self, File};
use std::{
    any::Any,
    env::consts,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
use task::static_source::{Definition, TaskDefinitions};
use util::{
    fs::remove_matching,
    github::{latest_github_release, GitHubLspBinaryVersion},
    maybe, ResultExt,
};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct ElixirSettings {
    pub lsp: ElixirLspSetting,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ElixirLspSetting {
    ElixirLs,
    NextLs,
    Local {
        path: String,
        arguments: Vec<String>,
    },
}

#[derive(Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct ElixirSettingsContent {
    lsp: Option<ElixirLspSetting>,
}

impl Settings for ElixirSettings {
    const KEY: Option<&'static str> = Some("elixir");

    type FileContent = ElixirSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Self::load_via_json_merge(default_value, user_values)
    }
}

pub struct ElixirLspAdapter;

#[async_trait(?Send)]
impl LspAdapter for ElixirLspAdapter {
    fn name(&self) -> LanguageServerName {
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
        Some(cx.spawn(|cx| async move {
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
                    })?
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
        let release = latest_github_release("elixir-lsp/elixir-ls", true, false, http).await?;

        let asset_name = format!("elixir-ls-{}.zip", &release.tag_name);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {asset_name:?}"))?;

        let version = GitHubLspBinaryVersion {
            name: release.tag_name.clone(),
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
        let folder_path = container_dir.join("elixir-ls");
        let binary_path = folder_path.join("language_server.sh");

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

            fs::create_dir_all(&folder_path)
                .await
                .with_context(|| format!("failed to create directory {}", folder_path.display()))?;
            let unzip_status = smol::process::Command::new("unzip")
                .arg(&zip_path)
                .arg("-d")
                .arg(&folder_path)
                .output()
                .await?
                .status;
            if !unzip_status.success() {
                Err(anyhow!("failed to unzip elixir-ls archive"))?;
            }

            remove_matching(&container_dir, |entry| entry != folder_path).await;
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            env: None,
            arguments: vec![],
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary_elixir_ls(container_dir).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary_elixir_ls(container_dir).await
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

    fn workspace_configuration(&self, _workspace_root: &Path, cx: &mut AppContext) -> Value {
        let settings = ProjectSettings::get_global(cx)
            .lsp
            .get("elixir-ls")
            .and_then(|s| s.settings.clone())
            .unwrap_or_default();

        serde_json::json!({
            "elixirLS": settings
        })
    }
}

async fn get_cached_server_binary_elixir_ls(
    container_dir: PathBuf,
) -> Option<LanguageServerBinary> {
    let server_path = container_dir.join("elixir-ls/language_server.sh");
    if server_path.exists() {
        Some(LanguageServerBinary {
            path: server_path,
            env: None,
            arguments: vec![],
        })
    } else {
        log::error!("missing executable in directory {:?}", server_path);
        None
    }
}

pub struct NextLspAdapter;

#[async_trait(?Send)]
impl LspAdapter for NextLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("next-ls".into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let platform = match consts::ARCH {
            "x86_64" => "darwin_amd64",
            "aarch64" => "darwin_arm64",
            other => bail!("Running on unsupported platform: {other}"),
        };
        let release =
            latest_github_release("elixir-tools/next-ls", true, false, delegate.http_client())
                .await?;
        let version = release.tag_name;
        let asset_name = format!("next_ls_{platform}");
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching {asset_name:?}"))?;
        let version = GitHubLspBinaryVersion {
            name: version,
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

        let binary_path = container_dir.join("next-ls");

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;

            let mut file = smol::fs::File::create(&binary_path).await?;
            if !response.status().is_success() {
                Err(anyhow!(
                    "download failed with status {}",
                    response.status().to_string()
                ))?;
            }
            futures::io::copy(response.body_mut(), &mut file).await?;

            // todo("windows")
            #[cfg(not(windows))]
            {
                fs::set_permissions(
                    &binary_path,
                    <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
                )
                .await?;
            }
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            env: None,
            arguments: vec!["--stdio".into()],
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary_next(container_dir)
            .await
            .map(|mut binary| {
                binary.arguments = vec!["--stdio".into()];
                binary
            })
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary_next(container_dir)
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
        label_for_completion_elixir(completion, language)
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        symbol_kind: SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        label_for_symbol_elixir(name, symbol_kind, language)
    }
}

async fn get_cached_server_binary_next(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    maybe!(async {
        let mut last_binary_path = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map_or(false, |name| name == "next-ls")
            {
                last_binary_path = Some(entry.path());
            }
        }

        if let Some(path) = last_binary_path {
            Ok(LanguageServerBinary {
                path,
                env: None,
                arguments: Vec::new(),
            })
        } else {
            Err(anyhow!("no cached binary"))
        }
    })
    .await
    .log_err()
}

pub struct LocalLspAdapter {
    pub path: String,
    pub arguments: Vec<String>,
}

#[async_trait(?Send)]
impl LspAdapter for LocalLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("local-ls".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let path = shellexpand::full(&self.path)?;
        Ok(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            env: None,
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let path = shellexpand::full(&self.path).ok()?;
        Some(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            env: None,
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        let path = shellexpand::full(&self.path).ok()?;
        Some(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            env: None,
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        label_for_completion_elixir(completion, language)
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        symbol: SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        label_for_symbol_elixir(name, symbol, language)
    }
}

fn label_for_completion_elixir(
    completion: &lsp::CompletionItem,
    language: &Arc<Language>,
) -> Option<CodeLabel> {
    return Some(CodeLabel {
        runs: language.highlight_text(&completion.label.clone().into(), 0..completion.label.len()),
        text: completion.label.clone(),
        filter_range: 0..completion.label.len(),
    });
}

fn label_for_symbol_elixir(
    name: &str,
    _: SymbolKind,
    language: &Arc<Language>,
) -> Option<CodeLabel> {
    Some(CodeLabel {
        runs: language.highlight_text(&name.into(), 0..name.len()),
        text: name.to_string(),
        filter_range: 0..name.len(),
    })
}

pub(super) fn elixir_task_context() -> ContextProviderWithTasks {
    // Taken from https://gist.github.com/josevalim/2e4f60a14ccd52728e3256571259d493#gistcomment-4995881
    ContextProviderWithTasks::new(TaskDefinitions(vec![
        Definition {
            label: "Elixir: test suite".to_owned(),
            command: "mix".to_owned(),
            args: vec!["test".to_owned()],
            ..Default::default()
        },
        Definition {
            label: "Elixir: failed tests suite".to_owned(),
            command: "mix".to_owned(),
            args: vec!["test".to_owned(), "--failed".to_owned()],
            ..Default::default()
        },
        Definition {
            label: "Elixir: test file".to_owned(),
            command: "mix".to_owned(),
            args: vec!["test".to_owned(), "$ZED_FILE".to_owned()],
            ..Default::default()
        },
        Definition {
            label: "Elixir: test at current line".to_owned(),
            command: "mix".to_owned(),
            args: vec!["test".to_owned(), "$ZED_FILE:$ZED_ROW".to_owned()],
            ..Default::default()
        },
        Definition {
            label: "Elixir: break line".to_owned(),
            command: "iex".to_owned(),
            args: vec![
                "-S".to_owned(),
                "mix".to_owned(),
                "test".to_owned(),
                "-b".to_owned(),
                "$ZED_FILE:$ZED_ROW".to_owned(),
            ],
            ..Default::default()
        },
    ]))
}
