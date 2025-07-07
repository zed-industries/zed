use std::{
    borrow::Cow,
    collections::HashMap,
    path::PathBuf,
    sync::{LazyLock, OnceLock},
};

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use dap::adapters::{DapDelegate, DebugTaskDefinition, latest_github_release};
use futures::StreamExt;
use gpui::AsyncApp;
use serde_json::Value;
use task::{DebugRequest, DebugScenario, ZedDebugConfig};
use util::fs::remove_matching;

use crate::*;

#[derive(Default)]
pub struct CodeLldbDebugAdapter {
    path_to_codelldb: OnceLock<String>,
}

impl CodeLldbDebugAdapter {
    pub const ADAPTER_NAME: &'static str = "CodeLLDB";

    async fn request_args(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        mut configuration: Value,
        label: &str,
    ) -> Result<dap::StartDebuggingRequestArguments> {
        let obj = configuration
            .as_object_mut()
            .context("CodeLLDB is not a valid json object")?;

        // CodeLLDB uses `name` for a terminal label.
        obj.entry("name")
            .or_insert(Value::String(String::from(label)));

        obj.entry("cwd")
            .or_insert(delegate.worktree_root_path().to_string_lossy().into());

        let request = self.request_kind(&configuration).await?;

        Ok(dap::StartDebuggingRequestArguments {
            request,
            configuration,
        })
    }

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<AdapterVersion> {
        let release =
            latest_github_release("vadimcn/codelldb", true, false, delegate.http_client()).await?;

        let arch = match std::env::consts::ARCH {
            "aarch64" => "arm64",
            "x86_64" => "x64",
            unsupported => {
                anyhow::bail!("unsupported architecture {unsupported}");
            }
        };
        let platform = match std::env::consts::OS {
            "macos" => "darwin",
            "linux" => "linux",
            "windows" => "win32",
            unsupported => {
                anyhow::bail!("unsupported operating system {unsupported}");
            }
        };
        let asset_name = format!("codelldb-{platform}-{arch}.vsix");
        let ret = AdapterVersion {
            tag_name: release.tag_name,
            url: release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .with_context(|| format!("no asset found matching {asset_name:?}"))?
                .browser_download_url
                .clone(),
        };

        Ok(ret)
    }
}

#[cfg(feature = "update-schemas")]
impl CodeLldbDebugAdapter {
    pub fn get_schema(
        temp_dir: &tempfile::TempDir,
        delegate: UpdateSchemasDapDelegate,
    ) -> anyhow::Result<serde_json::Value> {
        let (package_json, package_nls_json) = get_vsix_package_json(
            temp_dir,
            "vadimcn/codelldb",
            |_| Ok("codelldb-bootstrap.vsix".into()),
            delegate,
        )?;
        let package_json = parse_package_json(package_json, package_nls_json)?;

        let [debugger] =
            <[_; 1]>::try_from(package_json.contributes.debuggers).map_err(|debuggers| {
                anyhow::anyhow!(
                    "unexpected number of codelldb debuggers: {}",
                    debuggers.len()
                )
            })?;

        Ok(schema_for_configuration_attributes(
            debugger.configuration_attributes,
        ))
    }
}

#[async_trait(?Send)]
impl DebugAdapter for CodeLldbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        let mut configuration = json!({
            "request": match zed_scenario.request {
                DebugRequest::Launch(_) => "launch",
                DebugRequest::Attach(_) => "attach",
            },
        });
        let map = configuration.as_object_mut().unwrap();
        // CodeLLDB uses `name` for a terminal label.
        map.insert(
            "name".into(),
            Value::String(String::from(zed_scenario.label.as_ref())),
        );
        match &zed_scenario.request {
            DebugRequest::Attach(attach) => {
                map.insert("pid".into(), attach.process_id.into());
            }
            DebugRequest::Launch(launch) => {
                map.insert("program".into(), launch.program.clone().into());

                if !launch.args.is_empty() {
                    map.insert("args".into(), launch.args.clone().into());
                }
                if !launch.env.is_empty() {
                    map.insert("env".into(), launch.env_json());
                }
                if let Some(stop_on_entry) = zed_scenario.stop_on_entry {
                    map.insert("stopOnEntry".into(), stop_on_entry.into());
                }
                if let Some(cwd) = launch.cwd.as_ref() {
                    map.insert("cwd".into(), cwd.to_string_lossy().into_owned().into());
                }
            }
        }

        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            config: configuration,
            build: None,
            tcp_connection: None,
        })
    }

    fn dap_schema(&self) -> Cow<'static, serde_json::Value> {
        static SCHEMA: LazyLock<serde_json::Value> = LazyLock::new(|| {
            const RAW_SCHEMA: &str = include_str!("../schemas/CodeLLDB.json");
            serde_json::from_str(RAW_SCHEMA).unwrap()
        });
        Cow::Borrowed(&*SCHEMA)
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        user_args: Option<Vec<String>>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let mut command = user_installed_path
            .map(|p| p.to_string_lossy().to_string())
            .or(self.path_to_codelldb.get().cloned());

        if command.is_none() {
            delegate.output_to_console(format!("Checking latest version of {}...", self.name()));
            let adapter_path = paths::debug_adapters_dir().join(&Self::ADAPTER_NAME);
            let version_path =
                if let Ok(version) = self.fetch_latest_adapter_version(delegate).await {
                    adapters::download_adapter_from_github(
                        Self::ADAPTER_NAME,
                        version.clone(),
                        adapters::DownloadedFileType::Vsix,
                        paths::debug_adapters_dir(),
                        delegate.as_ref(),
                    )
                    .await?;
                    let version_path =
                        adapter_path.join(format!("{}_{}", Self::ADAPTER_NAME, version.tag_name));
                    remove_matching(&adapter_path, |entry| entry != version_path).await;
                    version_path
                } else {
                    let mut paths = delegate.fs().read_dir(&adapter_path).await?;
                    paths.next().await.context("No adapter found")??
                };
            let adapter_dir = version_path.join("extension").join("adapter");
            let path = adapter_dir.join("codelldb").to_string_lossy().to_string();
            self.path_to_codelldb.set(path.clone()).ok();
            command = Some(path);
        };
        let mut json_config = config.config.clone();
        Ok(DebugAdapterBinary {
            command: Some(command.unwrap()),
            cwd: Some(delegate.worktree_root_path().to_path_buf()),
            arguments: user_args.unwrap_or_else(|| {
                if let Some(config) = json_config.as_object_mut()
                    && let Some(source_languages) = config.get("sourceLanguages").filter(|value| {
                        value
                            .as_array()
                            .map_or(false, |array| array.iter().all(Value::is_string))
                    })
                {
                    let ret = vec![
                        "--settings".into(),
                        json!({"sourceLanguages": source_languages}).to_string(),
                    ];
                    config.remove("sourceLanguages");
                    ret
                } else {
                    vec![]
                }
            }),
            request_args: self
                .request_args(delegate, json_config, &config.label)
                .await?,
            envs: HashMap::default(),
            connection: None,
        })
    }
}
