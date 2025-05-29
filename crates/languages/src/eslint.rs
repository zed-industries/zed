use anyhow::{Context as _, Result};

use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use gpui::{AsyncApp, SharedString};
use http_client::github::{AssetKind, GitHubLspBinaryVersion, build_asset_url};
use language::{
    Attach, LanguageToolchainStore, LspAdapter, LspAdapterDelegate, ManifestName, ManifestProvider,
    ManifestQuery,
};
use lsp::{CodeActionKind, LanguageServerBinary, LanguageServerName};
use node_runtime::NodeRuntime;
use project::{Fs, lsp_store::language_server_settings};
use serde_json::{Value, json};
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::archive::extract_zip;
use util::{fs::remove_matching, merge_json_value_into};

fn eslint_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![
        "--max-old-space-size=8192".into(),
        server_path.into(),
        "--stdio".into(),
    ]
}

pub struct EsLintLspAdapter {
    node: NodeRuntime,
}

impl EsLintLspAdapter {
    const CURRENT_VERSION: &'static str = "2.4.4";
    const CURRENT_VERSION_TAG_NAME: &'static str = "release/2.4.4";

    #[cfg(not(windows))]
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::TarGz;
    #[cfg(windows)]
    const GITHUB_ASSET_KIND: AssetKind = AssetKind::Zip;

    const SERVER_PATH: &'static str = "vscode-eslint/server/out/eslintServer.js";
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("eslint");

    const FLAT_CONFIG_FILE_NAMES: &'static [&'static str] = &[
        "eslint.config.js",
        "eslint.config.mjs",
        "eslint.config.cjs",
        "eslint.config.ts",
        "eslint.config.cts",
        "eslint.config.mts",
    ];

    const LEGACY_CONFIG_FILE_NAMES: &'static [&'static str] = &[
        ".eslintrc.js",
        ".eslintrc.cjs",
        ".eslintrc.mjs",
        ".eslintrc.yaml",
        ".eslintrc.yml",
        ".eslintrc.json",
        ".eslintrc",
    ];

    pub fn new(node: NodeRuntime) -> Self {
        EsLintLspAdapter { node }
    }

    fn build_destination_path(container_dir: &Path) -> PathBuf {
        container_dir.join(format!("vscode-eslint-{}", Self::CURRENT_VERSION))
    }
}

pub(crate) struct EsLintConfigProvider;

impl ManifestProvider for EsLintConfigProvider {
    fn name(&self) -> ManifestName {
        SharedString::new_static("eslint").into()
    }

    fn search(
        &self,
        ManifestQuery {
            path,
            depth,
            delegate,
        }: ManifestQuery,
    ) -> Option<Arc<Path>> {
        for path in path.ancestors().take(depth) {
            for config_file in EsLintLspAdapter::FLAT_CONFIG_FILE_NAMES {
                let config_path = path.join(config_file);
                if delegate.exists(&config_path, Some(false)) {
                    return Some(path.into());
                }
            }
            for config_file in EsLintLspAdapter::LEGACY_CONFIG_FILE_NAMES {
                let config_path = path.join(config_file);
                if delegate.exists(&config_path, Some(false)) {
                    return Some(path.into());
                }
            }
        }
        None
    }
}

#[async_trait(?Send)]
impl LspAdapter for EsLintLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME.clone()
    }

    fn manifest_name(&self) -> Option<ManifestName> {
        Some(SharedString::new_static("eslint").into())
    }

    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        Some(vec![
            CodeActionKind::QUICKFIX,
            CodeActionKind::new("source.fixAll.eslint"),
        ])
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        _: &dyn Fs,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Arc<dyn LanguageToolchainStore>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        let workspace_root = delegate.worktree_root_path();

        dbg!(&delegate.worktree_root_path());
        dbg!(&delegate.subproject_root_path());

        let use_flat_config = Self::FLAT_CONFIG_FILE_NAMES
            .iter()
            .any(|file| workspace_root.join(file).is_file());

        let mut default_workspace_configuration = json!({
            "validate": "on",
            "rulesCustomizations": [],
            "run": "onType",
            "nodePath": null,
            "workingDirectory": {
                "mode": "auto"
            },
            "workspaceFolder": {
                "uri": workspace_root,
                "name": workspace_root.file_name()
                    .unwrap_or(workspace_root.as_os_str())
                    .to_string_lossy(),
            },
            "problems": {},
            "codeActionOnSave": {
                // We enable this, but without also configuring code_actions_on_format
                // in the Zed configuration, it doesn't have an effect.
                "enable": true,
            },
            "codeAction": {
                "disableRuleComment": {
                    "enable": true,
                    "location": "separateLine",
                },
                "showDocumentation": {
                    "enable": true
                }
            },
            "experimental": {
                "useFlatConfig": use_flat_config,
            },
        });

        let override_options = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &Self::SERVER_NAME, cx)
                .and_then(|s| s.settings.clone())
        })?;

        if let Some(override_options) = override_options {
            merge_json_value_into(override_options, &mut default_workspace_configuration);
        }

        Ok(json!({
            "": default_workspace_configuration
        }))
    }

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let url = build_asset_url(
            "zed-industries/vscode-eslint",
            Self::CURRENT_VERSION_TAG_NAME,
            Self::GITHUB_ASSET_KIND,
        )?;

        Ok(Box::new(GitHubLspBinaryVersion {
            name: Self::CURRENT_VERSION.into(),
            url,
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let destination_path = Self::build_destination_path(&container_dir);
        let server_path = destination_path.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            remove_matching(&container_dir, |entry| entry != destination_path).await;

            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .context("downloading release")?;
            match Self::GITHUB_ASSET_KIND {
                AssetKind::TarGz => {
                    let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
                    let archive = Archive::new(decompressed_bytes);
                    archive.unpack(&destination_path).await.with_context(|| {
                        format!("extracting {} to {:?}", version.url, destination_path)
                    })?;
                }
                AssetKind::Gz => {
                    let mut decompressed_bytes =
                        GzipDecoder::new(BufReader::new(response.body_mut()));
                    let mut file =
                        fs::File::create(&destination_path).await.with_context(|| {
                            format!(
                                "creating a file {:?} for a download from {}",
                                destination_path, version.url,
                            )
                        })?;
                    futures::io::copy(&mut decompressed_bytes, &mut file)
                        .await
                        .with_context(|| {
                            format!("extracting {} to {:?}", version.url, destination_path)
                        })?;
                }
                AssetKind::Zip => {
                    extract_zip(&destination_path, response.body_mut())
                        .await
                        .with_context(|| {
                            format!("unzipping {} to {:?}", version.url, destination_path)
                        })?;
                }
            }

            let mut dir = fs::read_dir(&destination_path).await?;
            let first = dir.next().await.context("missing first file")??;
            let repo_root = destination_path.join("vscode-eslint");
            fs::rename(first.path(), &repo_root).await?;

            #[cfg(target_os = "windows")]
            {
                handle_symlink(
                    repo_root.join("$shared"),
                    repo_root.join("client").join("src").join("shared"),
                )
                .await?;
                handle_symlink(
                    repo_root.join("$shared"),
                    repo_root.join("server").join("src").join("shared"),
                )
                .await?;
            }

            self.node
                .run_npm_subcommand(&repo_root, "install", &[])
                .await?;

            self.node
                .run_npm_subcommand(&repo_root, "run-script", &["compile"])
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: eslint_server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let server_path =
            Self::build_destination_path(&container_dir).join(EsLintLspAdapter::SERVER_PATH);
        Some(LanguageServerBinary {
            path: self.node.binary_path().await.ok()?,
            env: None,
            arguments: eslint_server_binary_arguments(&server_path),
        })
    }
}

#[cfg(target_os = "windows")]
async fn handle_symlink(src_dir: PathBuf, dest_dir: PathBuf) -> Result<()> {
    anyhow::ensure!(
        fs::metadata(&src_dir).await.is_ok(),
        "Directory {src_dir:?} is not present"
    );
    if fs::metadata(&dest_dir).await.is_ok() {
        fs::remove_file(&dest_dir).await?;
    }
    fs::create_dir_all(&dest_dir).await?;
    let mut entries = fs::read_dir(&src_dir).await?;
    while let Some(entry) = entries.try_next().await? {
        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let dest_path = dest_dir.join(&entry_name);
        fs::copy(&entry_path, &dest_path).await?;
    }
    Ok(())
}
