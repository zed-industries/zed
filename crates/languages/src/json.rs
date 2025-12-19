use anyhow::{Context as _, Result, bail};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use collections::HashMap;
use futures::StreamExt;
use gpui::{App, AsyncApp, Task};
use http_client::github::{GitHubLspBinaryVersion, latest_github_release};
use language::{
    ContextProvider, LanguageName, LanguageRegistry, LocalFile as _, LspAdapter,
    LspAdapterDelegate, LspInstaller, Toolchain,
};
use lsp::{LanguageServerBinary, LanguageServerName, Uri};
use node_runtime::{NodeRuntime, VersionStrategy};
use project::lsp_store::language_server_settings;
use semver::Version;
use serde_json::{Value, json};
use smol::{
    fs::{self},
    io::BufReader,
};
use std::{
    env::consts,
    ffi::OsString,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::{
    ResultExt, archive::extract_zip, fs::remove_matching, maybe, merge_json_value_into,
    rel_path::RelPath,
};

use crate::PackageJsonData;

const SERVER_PATH: &str =
    "node_modules/vscode-langservers-extracted/bin/vscode-json-language-server";

pub(crate) struct JsonTaskProvider;

impl ContextProvider for JsonTaskProvider {
    fn associated_tasks(
        &self,
        file: Option<Arc<dyn language::File>>,
        cx: &App,
    ) -> gpui::Task<Option<TaskTemplates>> {
        let Some(file) = project::File::from_dyn(file.as_ref()).cloned() else {
            return Task::ready(None);
        };
        let is_package_json = file.path.ends_with(RelPath::unix("package.json").unwrap());
        let is_composer_json = file.path.ends_with(RelPath::unix("composer.json").unwrap());
        if !is_package_json && !is_composer_json {
            return Task::ready(None);
        }

        cx.spawn(async move |cx| {
            let contents = file
                .worktree
                .update(cx, |this, cx| this.load_file(&file.path, cx))
                .ok()?
                .await
                .ok()?;
            let path = cx.update(|cx| file.abs_path(cx)).ok()?.as_path().into();

            let task_templates = if is_package_json {
                let package_json = serde_json_lenient::from_str::<
                    HashMap<String, serde_json_lenient::Value>,
                >(&contents.text)
                .ok()?;
                let package_json = PackageJsonData::new(path, package_json);
                let command = package_json.package_manager.unwrap_or("npm").to_owned();
                package_json
                    .scripts
                    .into_iter()
                    .map(|(_, key)| TaskTemplate {
                        label: format!("run {key}"),
                        command: command.clone(),
                        args: vec!["run".into(), key],
                        cwd: Some(VariableName::Dirname.template_value()),
                        ..TaskTemplate::default()
                    })
                    .chain([TaskTemplate {
                        label: "package script $ZED_CUSTOM_script".to_owned(),
                        command: command.clone(),
                        args: vec![
                            "run".into(),
                            VariableName::Custom("script".into()).template_value(),
                        ],
                        cwd: Some(VariableName::Dirname.template_value()),
                        tags: vec!["package-script".into()],
                        ..TaskTemplate::default()
                    }])
                    .collect()
            } else if is_composer_json {
                serde_json_lenient::Value::from_str(&contents.text)
                    .ok()?
                    .get("scripts")?
                    .as_object()?
                    .keys()
                    .map(|key| TaskTemplate {
                        label: format!("run {key}"),
                        command: "composer".to_owned(),
                        args: vec!["-d".into(), "$ZED_DIRNAME".into(), key.into()],
                        ..TaskTemplate::default()
                    })
                    .chain([TaskTemplate {
                        label: "composer script $ZED_CUSTOM_script".to_owned(),
                        command: "composer".to_owned(),
                        args: vec![
                            "-d".into(),
                            "$ZED_DIRNAME".into(),
                            VariableName::Custom("script".into()).template_value(),
                        ],
                        tags: vec!["composer-script".into()],
                        ..TaskTemplate::default()
                    }])
                    .collect()
            } else {
                vec![]
            };

            Some(TaskTemplates(task_templates))
        })
    }
}

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct JsonLspAdapter {
    languages: Arc<LanguageRegistry>,
    node: NodeRuntime,
}

impl JsonLspAdapter {
    const PACKAGE_NAME: &str = "vscode-langservers-extracted";

    pub fn new(languages: Arc<LanguageRegistry>, node: NodeRuntime) -> Self {
        Self { languages, node }
    }
}

impl LspInstaller for JsonLspAdapter {
    type BinaryVersion = Version;

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<Self::BinaryVersion> {
        self.node
            .npm_package_latest_version(Self::PACKAGE_NAME)
            .await
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let path = delegate
            .which("vscode-json-language-server".as_ref())
            .await?;
        let env = delegate.shell_env().await;

        Some(LanguageServerBinary {
            path,
            env: Some(env),
            arguments: vec!["--stdio".into()],
        })
    }

    async fn check_if_version_installed(
        &self,
        version: &Self::BinaryVersion,
        container_dir: &PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let server_path = container_dir.join(SERVER_PATH);

        let should_install_language_server = self
            .node
            .should_install_npm_package(
                Self::PACKAGE_NAME,
                &server_path,
                container_dir,
                VersionStrategy::Latest(version),
            )
            .await;

        if should_install_language_server {
            None
        } else {
            Some(LanguageServerBinary {
                path: self.node.binary_path().await.ok()?,
                env: None,
                arguments: server_binary_arguments(&server_path),
            })
        }
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Self::BinaryVersion,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let server_path = container_dir.join(SERVER_PATH);
        let latest_version = latest_version.to_string();

        self.node
            .npm_install_packages(
                &container_dir,
                &[(Self::PACKAGE_NAME, latest_version.as_str())],
            )
            .await?;

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &self.node).await
    }
}

#[async_trait(?Send)]
impl LspAdapter for JsonLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("json-language-server".into())
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(json!({
            "provideFormatter": true
        })))
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: Option<Uri>,
        cx: &mut AsyncApp,
    ) -> Result<Value> {
        let mut config = cx.update(|cx| {
            let schemas = json_schema_store::all_schema_file_associations(&self.languages, cx);

            // This can be viewed via `dev: open language server logs` -> `json-language-server` ->
            // `Server Info`
            serde_json::json!({
                "json": {
                    "format": {
                        "enable": true,
                    },
                    "validate": {
                        "enable": true,
                    },
                    "schemas": schemas
                }
            })
        })?;
        let project_options = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &self.name(), cx)
                .and_then(|s| s.settings.clone())
        })?;

        if let Some(override_options) = project_options {
            merge_json_value_into(override_options, &mut config);
        }

        Ok(config)
    }

    fn language_ids(&self) -> HashMap<LanguageName, String> {
        [
            (LanguageName::new_static("JSON"), "json".into()),
            (LanguageName::new_static("JSONC"), "jsonc".into()),
        ]
        .into_iter()
        .collect()
    }

    fn is_primary_zed_json_schema_adapter(&self) -> bool {
        true
    }
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &NodeRuntime,
) -> Option<LanguageServerBinary> {
    maybe!(async {
        let server_path = container_dir.join(SERVER_PATH);
        anyhow::ensure!(
            server_path.exists(),
            "missing executable in directory {server_path:?}"
        );
        Ok(LanguageServerBinary {
            path: node.binary_path().await?,
            env: None,
            arguments: server_binary_arguments(&server_path),
        })
    })
    .await
    .log_err()
}

pub struct NodeVersionAdapter;

impl NodeVersionAdapter {
    const SERVER_NAME: LanguageServerName =
        LanguageServerName::new_static("package-version-server");
}

impl LspInstaller for NodeVersionAdapter {
    type BinaryVersion = GitHubLspBinaryVersion;

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<GitHubLspBinaryVersion> {
        let release = latest_github_release(
            "zed-industries/package-version-server",
            true,
            false,
            delegate.http_client(),
        )
        .await?;
        let os = match consts::OS {
            "macos" => "apple-darwin",
            "linux" => "unknown-linux-gnu",
            "windows" => "pc-windows-msvc",
            other => bail!("Running on unsupported os: {other}"),
        };
        let suffix = if consts::OS == "windows" {
            ".zip"
        } else {
            ".tar.gz"
        };
        let asset_name = format!("{}-{}-{os}{suffix}", Self::SERVER_NAME, consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching `{asset_name:?}`"))?;
        Ok(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
            digest: asset.digest.clone(),
        })
    }

    async fn check_if_user_installed(
        &self,
        delegate: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let path = delegate.which(Self::SERVER_NAME.as_ref()).await?;
        Some(LanguageServerBinary {
            path,
            env: None,
            arguments: Default::default(),
        })
    }

    async fn fetch_server_binary(
        &self,
        latest_version: GitHubLspBinaryVersion,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = &latest_version;
        let destination_path = container_dir.join(format!(
            "{}-{}{}",
            Self::SERVER_NAME,
            version.name,
            std::env::consts::EXE_SUFFIX
        ));
        let destination_container_path =
            container_dir.join(format!("{}-{}-tmp", Self::SERVER_NAME, version.name));
        if fs::metadata(&destination_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .context("downloading release")?;
            if version.url.ends_with(".zip") {
                extract_zip(&destination_container_path, response.body_mut()).await?;
            } else if version.url.ends_with(".tar.gz") {
                let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
                let archive = Archive::new(decompressed_bytes);
                archive.unpack(&destination_container_path).await?;
            }

            fs::copy(
                destination_container_path.join(format!(
                    "{}{}",
                    Self::SERVER_NAME,
                    std::env::consts::EXE_SUFFIX
                )),
                &destination_path,
            )
            .await?;
            remove_matching(&container_dir, |entry| entry != destination_path).await;
        }
        Ok(LanguageServerBinary {
            path: destination_path,
            env: None,
            arguments: Default::default(),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_version_server_binary(container_dir).await
    }
}

#[async_trait(?Send)]
impl LspAdapter for NodeVersionAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }
}

async fn get_cached_version_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    maybe!(async {
        let mut last = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            last = Some(entry?.path());
        }

        anyhow::Ok(LanguageServerBinary {
            path: last.context("no cached binary")?,
            env: None,
            arguments: Default::default(),
        })
    })
    .await
    .log_err()
}
