use anyhow::{anyhow, bail, Result};

use async_trait::async_trait;
pub use language::*;
use lsp::{LanguageServerBinary, SymbolKind};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Setting;
use smol::{fs, stream::StreamExt};
use std::{any::Any, env::consts, ops::Deref, path::PathBuf, sync::Arc};
use util::{
    async_iife,
    github::{latest_github_release, GitHubLspBinaryVersion},
    ResultExt,
};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct ElixirSettings {
    pub next: ElixirNextSetting,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ElixirNextSetting {
    Off,
    On,
    Local {
        path: String,
        arguments: Vec<String>,
    },
}

#[derive(Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct ElixirSettingsContent {
    next: Option<ElixirNextSetting>,
}

impl Setting for ElixirSettings {
    const KEY: Option<&'static str> = Some("elixir");

    type FileContent = ElixirSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Self::load_via_json_merge(default_value, user_values)
    }
}

pub struct NextLspAdapter;

#[async_trait]
impl LspAdapter for NextLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("next-ls".into())
    }

    fn short_name(&self) -> &'static str {
        "next-ls"
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release =
            latest_github_release("elixir-tools/next-ls", false, delegate.http_client()).await?;
        let version = release.name.clone();
        let platform = match consts::ARCH {
            "x86_64" => "darwin_arm64",
            "aarch64" => "darwin_amd64",
            other => bail!("Running on unsupported platform: {other}"),
        };
        let asset_name = format!("next_ls_{}", platform);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
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

            fs::set_permissions(
                &binary_path,
                <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
            )
            .await?;
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            arguments: vec!["--stdio".into()],
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir)
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
        get_cached_server_binary(container_dir)
            .await
            .map(|mut binary| {
                binary.arguments = vec!["--help".into()];
                binary
            })
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        symbol_kind: SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        label_for_symbol_next(name, symbol_kind, language)
    }
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    async_iife!({
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
                arguments: Vec::new(),
            })
        } else {
            Err(anyhow!("no cached binary"))
        }
    })
    .await
    .log_err()
}

pub struct LocalNextLspAdapter {
    pub path: String,
    pub arguments: Vec<String>,
}

#[async_trait]
impl LspAdapter for LocalNextLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("local-next-ls".into())
    }

    fn short_name(&self) -> &'static str {
        "next-ls"
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
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        let path = shellexpand::full(&self.path).ok()?;
        Some(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        symbol: SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        label_for_symbol_next(name, symbol, language)
    }
}

fn label_for_symbol_next(name: &str, _: SymbolKind, language: &Arc<Language>) -> Option<CodeLabel> {
    Some(CodeLabel {
        runs: language.highlight_text(&name.into(), 0..name.len()),
        text: name.to_string(),
        filter_range: 0..name.len(),
    })
}
