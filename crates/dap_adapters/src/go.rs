use anyhow::{Context as _, bail};
use collections::HashMap;
use dap::{
    StartDebuggingRequestArguments,
    adapters::{
        DebugTaskDefinition, DownloadedFileType, TcpArguments, download_adapter_from_github,
        latest_github_release,
    },
};
use fs::Fs;
use gpui::{AsyncApp, SharedString};
use language::LanguageName;
use log::warn;
use serde_json::{Map, Value};
use task::TcpArgumentsTemplate;
use util;

use std::{
    borrow::Cow,
    env::consts,
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{LazyLock, OnceLock},
};

use crate::*;

#[derive(Default, Debug)]
pub struct GoDebugAdapter {
    shim_path: OnceLock<PathBuf>,
}

impl GoDebugAdapter {
    pub const ADAPTER_NAME: &'static str = "Delve";
    async fn fetch_latest_adapter_version(
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<AdapterVersion> {
        let release = latest_github_release(
            &"zed-industries/delve-shim-dap",
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
        let asset_name = format!("delve-shim-dap-{}-{os}{suffix}", consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching `{asset_name:?}`"))?;

        Ok(AdapterVersion {
            tag_name: release.tag_name,
            url: asset.browser_download_url.clone(),
        })
    }
    async fn install_shim(&self, delegate: &Arc<dyn DapDelegate>) -> anyhow::Result<PathBuf> {
        if let Some(path) = self.shim_path.get().cloned() {
            return Ok(path);
        }

        let asset = Self::fetch_latest_adapter_version(delegate).await?;
        let ty = if consts::OS == "windows" {
            DownloadedFileType::Zip
        } else {
            DownloadedFileType::GzipTar
        };
        download_adapter_from_github(
            "delve-shim-dap".into(),
            asset.clone(),
            ty,
            paths::debug_adapters_dir(),
            delegate.as_ref(),
        )
        .await?;

        let path = paths::debug_adapters_dir()
            .join("delve-shim-dap")
            .join(format!("delve-shim-dap_{}", asset.tag_name))
            .join(format!("delve-shim-dap{}", std::env::consts::EXE_SUFFIX));
        self.shim_path.set(path.clone()).ok();

        Ok(path)
    }
}

#[cfg(feature = "update-schemas")]
impl GoDebugAdapter {
    pub fn get_schema(
        temp_dir: &TempDir,
        delegate: UpdateSchemasDapDelegate,
    ) -> anyhow::Result<serde_json::Value> {
        let (package_json, package_nls_json) = get_vsix_package_json(
            temp_dir,
            "golang/vscode-go",
            |version| {
                let version = version
                    .tag_name
                    .strip_prefix("v")
                    .context("parse tag name")?;
                Ok(format!("go-{version}.vsix"))
            },
            delegate,
        )?;
        let package_json = parse_package_json(package_json, package_nls_json)?;

        let [debugger] =
            <[_; 1]>::try_from(package_json.contributes.debuggers).map_err(|debuggers| {
                anyhow::anyhow!("unexpected number of go debuggers: {}", debuggers.len())
            })?;

        let configuration_attributes = debugger.configuration_attributes;
        let conjuncts = configuration_attributes
            .launch
            .map(|schema| ("launch", schema))
            .into_iter()
            .chain(
                configuration_attributes
                    .attach
                    .map(|schema| ("attach", schema)),
            )
            .map(|(request, schema)| {
                json!({
                    "if": {
                        "properties": {
                            "request": {
                                "const": request
                            }
                        },
                        "required": ["request"]
                    },
                    "then": schema
                })
            })
            .collect::<Vec<_>>();

        let schema = json!({
            "allOf": conjuncts
        });
        Ok(schema)
    }
}

#[async_trait(?Send)]
impl DebugAdapter for GoDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(SharedString::new_static("Go").into())
    }

    fn dap_schema(&self) -> Cow<'static, serde_json::Value> {
        static SCHEMA: LazyLock<serde_json::Value> = LazyLock::new(|| {
            const RAW_SCHEMA: &str = include_str!("../schemas/Delve.json");
            serde_json::from_str(RAW_SCHEMA).unwrap()
        });
        Cow::Borrowed(&*SCHEMA)
    }

    async fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        let mut args = match &zed_scenario.request {
            dap::DebugRequest::Attach(attach_config) => {
                json!({
                    "request": "attach",
                    "mode": "debug",
                    "processId": attach_config.process_id,
                })
            }
            dap::DebugRequest::Launch(launch_config) => {
                let mode = if launch_config.program != "." {
                    "exec"
                } else {
                    "debug"
                };

                json!({
                    "request": "launch",
                    "mode": mode,
                    "program": launch_config.program,
                    "cwd": launch_config.cwd,
                    "args": launch_config.args,
                    "env": launch_config.env_json()
                })
            }
        };

        let map = args.as_object_mut().unwrap();

        if let Some(stop_on_entry) = zed_scenario.stop_on_entry {
            map.insert("stopOnEntry".into(), stop_on_entry.into());
        }

        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            build: None,
            config: args,
            tcp_connection: None,
        })
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        task_definition: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        user_args: Option<Vec<String>>,
        _cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = paths::debug_adapters_dir().join(&Self::ADAPTER_NAME);
        let dlv_path = adapter_path.join("dlv");

        let delve_path = if let Some(path) = user_installed_path {
            path.to_string_lossy().to_string()
        } else if let Some(path) = delegate.which(OsStr::new("dlv")).await {
            path.to_string_lossy().to_string()
        } else if delegate.fs().is_file(&dlv_path).await {
            dlv_path.to_string_lossy().to_string()
        } else {
            let go = delegate
                .which(OsStr::new("go"))
                .await
                .context("Go not found in path. Please install Go first, then Dlv will be installed automatically.")?;

            let adapter_path = paths::debug_adapters_dir().join(&Self::ADAPTER_NAME);

            let install_output = util::command::new_smol_command(&go)
                .env("GO111MODULE", "on")
                .env("GOBIN", &adapter_path)
                .args(&["install", "github.com/go-delve/delve/cmd/dlv@latest"])
                .output()
                .await?;

            if !install_output.status.success() {
                bail!(
                    "failed to install dlv via `go install`. stdout: {:?}, stderr: {:?}\n Please try installing it manually using 'go install github.com/go-delve/delve/cmd/dlv@latest'",
                    String::from_utf8_lossy(&install_output.stdout),
                    String::from_utf8_lossy(&install_output.stderr)
                );
            }

            adapter_path.join("dlv").to_string_lossy().to_string()
        };

        let cwd = Some(
            task_definition
                .config
                .get("cwd")
                .and_then(|s| s.as_str())
                .map(PathBuf::from)
                .unwrap_or_else(|| delegate.worktree_root_path().to_path_buf()),
        );

        let arguments;
        let command;
        let connection;

        let mut configuration = task_definition.config.clone();
        let mut envs = HashMap::default();

        if let Some(configuration) = configuration.as_object_mut() {
            configuration
                .entry("cwd")
                .or_insert_with(|| delegate.worktree_root_path().to_string_lossy().into());

            handle_envs(
                configuration,
                &mut envs,
                cwd.as_deref(),
                delegate.fs().clone(),
            )
            .await;
        }

        if let Some(connection_options) = &task_definition.tcp_connection {
            command = None;
            arguments = vec![];
            let (host, port, timeout) =
                crate::configure_tcp_connection(connection_options.clone()).await?;
            connection = Some(TcpArguments {
                host,
                port,
                timeout,
            });
        } else {
            let minidelve_path = self.install_shim(delegate).await?;
            let (host, port, _) =
                crate::configure_tcp_connection(TcpArgumentsTemplate::default()).await?;
            command = Some(minidelve_path.to_string_lossy().into_owned());
            connection = None;
            arguments = if let Some(mut args) = user_args {
                args.insert(0, delve_path);
                args
            } else if cfg!(windows) {
                vec![
                    delve_path,
                    "dap".into(),
                    "--listen".into(),
                    format!("{}:{}", host, port),
                    "--headless".into(),
                ]
            } else {
                vec![
                    delve_path,
                    "dap".into(),
                    "--listen".into(),
                    format!("{}:{}", host, port),
                ]
            };
        }
        Ok(DebugAdapterBinary {
            command,
            arguments,
            cwd,
            envs,
            connection,
            request_args: StartDebuggingRequestArguments {
                configuration,
                request: self.request_kind(&task_definition.config).await?,
            },
        })
    }
}

// delve doesn't do anything with the envFile setting, so we intercept it
async fn handle_envs(
    config: &mut Map<String, Value>,
    envs: &mut HashMap<String, String>,
    cwd: Option<&Path>,
    fs: Arc<dyn Fs>,
) -> Option<()> {
    let env_files = match config.get("envFile")? {
        Value::Array(arr) => arr.iter().map(|v| v.as_str()).collect::<Vec<_>>(),
        Value::String(s) => vec![Some(s.as_str())],
        _ => return None,
    };

    let rebase_path = |path: PathBuf| {
        if path.is_absolute() {
            Some(path)
        } else {
            cwd.map(|p| p.join(path))
        }
    };

    for path in env_files {
        let Some(path) = path
            .and_then(|s| PathBuf::from_str(s).ok())
            .and_then(rebase_path)
        else {
            continue;
        };

        if let Ok(file) = fs.open_sync(&path).await {
            envs.extend(dotenvy::from_read_iter(file).filter_map(Result::ok))
        } else {
            warn!("While starting Go debug session: failed to read env file {path:?}");
        };
    }

    // remove envFile now that it's been handled
    config.remove("entry");
    Some(())
}
