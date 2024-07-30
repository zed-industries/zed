use anyhow::{anyhow, bail, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use collections::HashMap;
use feature_flags::FeatureFlagAppExt;
use futures::StreamExt;
use gpui::{AppContext, AsyncAppContext};
use http_client::github::{latest_github_release, GitHubLspBinaryVersion};
use language::{LanguageRegistry, LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use project::ContextProviderWithTasks;
use serde_json::{json, Value};
use settings::{KeymapFile, SettingsJsonSchemaParams, SettingsStore};
use smol::{
    fs::{self},
    io::BufReader,
};
use std::{
    any::Any,
    env::consts,
    ffi::OsString,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, OnceLock},
};
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::{fs::remove_matching, maybe, ResultExt};

const SERVER_PATH: &str =
    "node_modules/vscode-langservers-extracted/bin/vscode-json-language-server";

// Origin: https://github.com/SchemaStore/schemastore
const TSCONFIG_SCHEMA: &str = include_str!("json/schemas/tsconfig.json");
const PACKAGE_JSON_SCHEMA: &str = include_str!("json/schemas/package.json");

pub(super) fn json_task_context() -> ContextProviderWithTasks {
    ContextProviderWithTasks::new(TaskTemplates(vec![
        TaskTemplate {
            label: "package script $ZED_CUSTOM_script".to_owned(),
            command: "npm --prefix $ZED_DIRNAME run".to_owned(),
            args: vec![VariableName::Custom("script".into()).template_value()],
            tags: vec!["package-script".into()],
            ..TaskTemplate::default()
        },
        TaskTemplate {
            label: "composer script $ZED_CUSTOM_script".to_owned(),
            command: "composer -d $ZED_DIRNAME".to_owned(),
            args: vec![VariableName::Custom("script".into()).template_value()],
            tags: vec!["composer-script".into()],
            ..TaskTemplate::default()
        },
    ]))
}

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct JsonLspAdapter {
    node: Arc<dyn NodeRuntime>,
    languages: Arc<LanguageRegistry>,
    workspace_config: OnceLock<Value>,
}

impl JsonLspAdapter {
    pub fn new(node: Arc<dyn NodeRuntime>, languages: Arc<LanguageRegistry>) -> Self {
        Self {
            node,
            languages,
            workspace_config: Default::default(),
        }
    }

    fn get_workspace_config(language_names: Vec<String>, cx: &mut AppContext) -> Value {
        let action_names = cx.all_action_names();
        let staff_mode = cx.is_staff();

        let font_names = &cx.text_system().all_font_names();
        let settings_schema = cx.global::<SettingsStore>().json_schema(
            &SettingsJsonSchemaParams {
                language_names: &language_names,
                staff_mode,
                font_names,
            },
            cx,
        );
        let tasks_schema = task::TaskTemplates::generate_json_schema();
        let tsconfig_schema = serde_json::Value::from_str(TSCONFIG_SCHEMA).unwrap();
        let package_json_schema = serde_json::Value::from_str(PACKAGE_JSON_SCHEMA).unwrap();

        serde_json::json!({
            "json": {
                "format": {
                    "enable": true,
                },
                "validate":
                {
                    "enable": true,
                },
                "schemas": [
                    {
                        "fileMatch": ["tsconfig.json"],
                        "schema":tsconfig_schema
                    },
                    {
                        "fileMatch": ["package.json"],
                        "schema":package_json_schema
                    },
                    {
                        "fileMatch": [
                            schema_file_match(paths::settings_file()),
                            paths::local_settings_file_relative_path()
                        ],
                        "schema": settings_schema,
                    },
                    {
                        "fileMatch": [schema_file_match(paths::keymap_file())],
                        "schema": KeymapFile::generate_json_schema(&action_names),
                    },
                    {
                        "fileMatch": [
                            schema_file_match(paths::tasks_file()),
                            paths::local_tasks_file_relative_path()
                        ],
                        "schema": tasks_schema,
                    }

                ]
            }
        })
    }
}

#[async_trait(?Send)]
impl LspAdapter for JsonLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("json-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(
            self.node
                .npm_package_latest_version("vscode-langservers-extracted")
                .await?,
        ) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<String>().unwrap();
        let server_path = container_dir.join(SERVER_PATH);
        let package_name = "vscode-langservers-extracted";

        let should_install_language_server = self
            .node
            .should_install_npm_package(package_name, &server_path, &container_dir, &latest_version)
            .await;

        if should_install_language_server {
            self.node
                .npm_install_packages(&container_dir, &[(package_name, latest_version.as_str())])
                .await?;
        }

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
        get_cached_server_binary(container_dir, &*self.node).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &*self.node).await
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
        _: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        cx.update(|cx| {
            self.workspace_config
                .get_or_init(|| Self::get_workspace_config(self.languages.language_names(), cx))
                .clone()
        })
    }

    fn language_ids(&self) -> HashMap<String, String> {
        [
            ("JSON".into(), "json".into()),
            ("JSONC".into(), "jsonc".into()),
        ]
        .into_iter()
        .collect()
    }
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &dyn NodeRuntime,
) -> Option<LanguageServerBinary> {
    maybe!(async {
        let mut last_version_dir = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_dir() {
                last_version_dir = Some(entry.path());
            }
        }

        let last_version_dir = last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
        let server_path = last_version_dir.join(SERVER_PATH);
        if server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: server_binary_arguments(&server_path),
            })
        } else {
            Err(anyhow!(
                "missing executable in directory {:?}",
                last_version_dir
            ))
        }
    })
    .await
    .log_err()
}

#[inline]
fn schema_file_match(path: &Path) -> String {
    path.strip_prefix(path.parent().unwrap().parent().unwrap())
        .unwrap()
        .display()
        .to_string()
        .replace('\\', "/")
}

pub(super) struct NodeVersionAdapter;

#[async_trait(?Send)]
impl LspAdapter for NodeVersionAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("package-version-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
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
        let asset_name = format!("package-version-server-{}-{os}{suffix}", consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching `{asset_name:?}`"))?;
        Ok(Box::new(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
        }))
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = latest_version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let destination_path = container_dir.join(format!(
            "package-version-server-{}{}",
            version.name,
            std::env::consts::EXE_SUFFIX
        ));
        let destination_container_path =
            container_dir.join(format!("package-version-server-{}-tmp", version.name));
        if fs::metadata(&destination_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;
            if version.url.ends_with(".zip") {
                node_runtime::extract_zip(
                    &destination_container_path,
                    BufReader::new(response.body_mut()),
                )
                .await?;
            } else if version.url.ends_with(".tar.gz") {
                let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
                let archive = Archive::new(decompressed_bytes);
                archive.unpack(&destination_container_path).await?;
            }

            fs::copy(
                destination_container_path.join(format!(
                    "package-version-server{}",
                    std::env::consts::EXE_SUFFIX
                )),
                &destination_path,
            )
            .await?;
            // todo("windows")
            #[cfg(not(windows))]
            {
                fs::set_permissions(
                    &destination_path,
                    <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
                )
                .await?;
            }
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

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_version_server_binary(container_dir)
            .await
            .map(|mut binary| {
                binary.arguments = vec!["--version".into()];
                binary
            })
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
            path: last.ok_or_else(|| anyhow!("no cached binary"))?,
            env: None,
            arguments: Default::default(),
        })
    })
    .await
    .log_err()
}
