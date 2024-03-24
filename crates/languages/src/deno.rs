use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::StreamExt;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::{CodeActionKind, LanguageServerBinary};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use settings::Settings;
use smol::{fs, fs::File};
use std::{any::Any, env::consts, ffi::OsString, path::PathBuf, sync::Arc};
use util::{
    fs::remove_matching,
    github::{latest_github_release, GitHubLspBinaryVersion},
    maybe, ResultExt,
};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct DenoSettings {
    pub enable: bool,
}

#[derive(Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct DenoSettingsContent {
    enable: Option<bool>,
}

impl Settings for DenoSettings {
    const KEY: Option<&'static str> = Some("deno");

    type FileContent = DenoSettingsContent;

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

fn deno_server_binary_arguments() -> Vec<OsString> {
    vec!["lsp".into()]
}

pub struct DenoLspAdapter {}

impl DenoLspAdapter {
    pub fn new() -> Self {
        DenoLspAdapter {}
    }
}

#[async_trait(?Send)]
impl LspAdapter for DenoLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("deno-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release =
            latest_github_release("denoland/deno", true, false, delegate.http_client()).await?;
        let os = match consts::OS {
            "macos" => "apple-darwin",
            "linux" => "unknown-linux-gnu",
            "windows" => "pc-windows-msvc",
            other => bail!("Running on unsupported os: {other}"),
        };
        let asset_name = format!("deno-{}-{os}.zip", consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
        let version = GitHubLspBinaryVersion {
            name: release.tag_name,
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
        let zip_path = container_dir.join(format!("deno_{}.zip", version.name));
        let version_dir = container_dir.join(format!("deno_{}", version.name));
        let binary_path = version_dir.join("deno");

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
                .arg("-d")
                .arg(&version_dir)
                .output()
                .await?
                .status;
            if !unzip_status.success() {
                Err(anyhow!("failed to unzip deno archive"))?;
            }

            remove_matching(&container_dir, |entry| entry != version_dir).await;
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            env: None,
            arguments: deno_server_binary_arguments(),
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

    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        Some(vec![
            CodeActionKind::QUICKFIX,
            CodeActionKind::REFACTOR,
            CodeActionKind::REFACTOR_EXTRACT,
            CodeActionKind::SOURCE,
        ])
    }

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        use lsp::CompletionItemKind as Kind;
        let len = item.label.len();
        let grammar = language.grammar()?;
        let highlight_id = match item.kind? {
            Kind::CLASS | Kind::INTERFACE => grammar.highlight_id_for_name("type"),
            Kind::CONSTRUCTOR => grammar.highlight_id_for_name("type"),
            Kind::CONSTANT => grammar.highlight_id_for_name("constant"),
            Kind::FUNCTION | Kind::METHOD => grammar.highlight_id_for_name("function"),
            Kind::PROPERTY | Kind::FIELD => grammar.highlight_id_for_name("property"),
            _ => None,
        }?;

        let text = match &item.detail {
            Some(detail) => format!("{} {}", item.label, detail),
            None => item.label.clone(),
        };

        Some(language::CodeLabel {
            text,
            runs: vec![(0..len, highlight_id)],
            filter_range: 0..len,
        })
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(json!({
            "provideFormatter": true,
        })))
    }

    fn language_ids(&self) -> HashMap<String, String> {
        HashMap::from_iter([
            ("TypeScript".into(), "typescript".into()),
            ("JavaScript".into(), "javascript".into()),
            ("TSX".into(), "typescriptreact".into()),
        ])
    }
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    maybe!(async {
        let mut last = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            last = Some(entry?.path());
        }

        match last {
            Some(path) if path.is_dir() => {
                let binary = path.join("deno");
                if fs::metadata(&binary).await.is_ok() {
                    return Ok(LanguageServerBinary {
                        path: binary,
                        env: None,
                        arguments: deno_server_binary_arguments(),
                    });
                }
            }
            _ => {}
        }

        Err(anyhow!("no cached binary"))
    })
    .await
    .log_err()
}
