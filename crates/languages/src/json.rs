use anyhow::{anyhow, Result};
use async_trait::async_trait;
use collections::HashMap;
use feature_flags::FeatureFlagAppExt;
use futures::StreamExt;
use gpui::AppContext;
use language::{LanguageRegistry, LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use serde_json::{json, Value};
use settings::{KeymapFile, SettingsJsonSchemaParams, SettingsStore};
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};
use util::{async_maybe, paths, ResultExt};

const SERVER_PATH: &str = "node_modules/vscode-json-languageserver/bin/vscode-json-languageserver";

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
        let tasks_schema = task::static_source::DefinitionProvider::generate_json_schema();
        serde_json::json!({
            "json": {
                "format": {
                    "enable": true,
                },
                "schemas": [
                    {
                        "fileMatch": [
                            schema_file_match(&paths::SETTINGS),
                            &*paths::LOCAL_SETTINGS_RELATIVE_PATH,
                        ],
                        "schema": settings_schema,
                    },
                    {
                        "fileMatch": [schema_file_match(&paths::KEYMAP)],
                        "schema": KeymapFile::generate_json_schema(&action_names),
                    },
                    {
                        "fileMatch": [
                            schema_file_match(&paths::TASKS),
                            &*paths::LOCAL_TASKS_RELATIVE_PATH,
                        ],
                        "schema": tasks_schema,
                    }
                ]
            }
        })
    }
}

#[async_trait]
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
                .npm_package_latest_version("vscode-json-languageserver")
                .await?,
        ) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<String>().unwrap();
        let server_path = container_dir.join(SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[("vscode-json-languageserver", version.as_str())],
                )
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

    fn initialization_options(&self) -> Option<serde_json::Value> {
        Some(json!({
            "provideFormatter": true
        }))
    }

    fn workspace_configuration(&self, _workspace_root: &Path, cx: &mut AppContext) -> Value {
        self.workspace_config
            .get_or_init(|| Self::get_workspace_config(self.languages.language_names(), cx))
            .clone()
    }

    fn language_ids(&self) -> HashMap<String, String> {
        [("JSON".into(), "jsonc".into())].into_iter().collect()
    }
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &dyn NodeRuntime,
) -> Option<LanguageServerBinary> {
    async_maybe!({
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

fn schema_file_match(path: &Path) -> &Path {
    path.strip_prefix(path.parent().unwrap().parent().unwrap())
        .unwrap()
}
