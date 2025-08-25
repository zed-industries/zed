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
    env::consts,
    ffi::OsStr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::OnceLock,
};

use crate::*;

#[derive(Default, Debug)]
pub(crate) struct GoDebugAdapter {
    shim_path: OnceLock<PathBuf>,
}

impl GoDebugAdapter {
    const ADAPTER_NAME: &'static str = "Delve";
    async fn fetch_latest_adapter_version(
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<AdapterVersion> {
        let release = latest_github_release(
            "zed-industries/delve-shim-dap",
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

#[async_trait(?Send)]
impl DebugAdapter for GoDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(SharedString::new_static("Go").into())
    }

    fn dap_schema(&self) -> serde_json::Value {
        // Create common properties shared between launch and attach
        let common_properties = json!({
            "debugAdapter": {
                "enum": ["legacy", "dlv-dap"],
                "description": "Select which debug adapter to use with this configuration.",
                "default": "dlv-dap"
            },
            "stopOnEntry": {
                "type": "boolean",
                "description": "Automatically stop program after launch or attach.",
                "default": false
            },
            "showLog": {
                "type": "boolean",
                "description": "Show log output from the delve debugger. Maps to dlv's `--log` flag.",
                "default": false
            },
            "cwd": {
                "type": "string",
                "description": "Workspace relative or absolute path to the working directory of the program being debugged.",
                "default": "${ZED_WORKTREE_ROOT}"
            },
            "dlvFlags": {
                "type": "array",
                "description": "Extra flags for `dlv`. See `dlv help` for the full list of supported flags.",
                "items": {
                    "type": "string"
                },
                "default": []
            },
            "port": {
                "type": "number",
                "description": "Debug server port. For remote configurations, this is where to connect.",
                "default": 2345
            },
            "host": {
                "type": "string",
                "description": "Debug server host. For remote configurations, this is where to connect.",
                "default": "127.0.0.1"
            },
            "substitutePath": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "from": {
                            "type": "string",
                            "description": "The absolute local path to be replaced."
                        },
                        "to": {
                            "type": "string",
                            "description": "The absolute remote path to replace with."
                        }
                    }
                },
                "description": "Mappings from local to remote paths for debugging.",
                "default": []
            },
            "trace": {
                "type": "string",
                "enum": ["verbose", "trace", "log", "info", "warn", "error"],
                "default": "error",
                "description": "Debug logging level."
            },
            "backend": {
                "type": "string",
                "enum": ["default", "native", "lldb", "rr"],
                "description": "Backend used by delve. Maps to `dlv`'s `--backend` flag."
            },
            "logOutput": {
                "type": "string",
                "enum": ["debugger", "gdbwire", "lldbout", "debuglineerr", "rpc", "dap"],
                "description": "Components that should produce debug output.",
                "default": "debugger"
            },
            "logDest": {
                "type": "string",
                "description": "Log destination for delve."
            },
            "stackTraceDepth": {
                "type": "number",
                "description": "Maximum depth of stack traces.",
                "default": 50
            },
            "showGlobalVariables": {
                "type": "boolean",
                "default": false,
                "description": "Show global package variables in variables pane."
            },
            "showRegisters": {
                "type": "boolean",
                "default": false,
                "description": "Show register variables in variables pane."
            },
            "hideSystemGoroutines": {
                "type": "boolean",
                "default": false,
                "description": "Hide system goroutines from call stack view."
            },
            "console": {
                "default": "internalConsole",
                "description": "Where to launch the debugger.",
                "enum": ["internalConsole", "integratedTerminal"]
            },
            "asRoot": {
                "default": false,
                "description": "Debug with elevated permissions (on Unix).",
                "type": "boolean"
            }
        });

        // Create launch-specific properties
        let launch_properties = json!({
            "program": {
                "type": "string",
                "description": "Path to the program folder or file to debug.",
                "default": "${ZED_WORKTREE_ROOT}"
            },
            "args": {
                "type": ["array", "string"],
                "description": "Command line arguments for the program.",
                "items": {
                    "type": "string"
                },
                "default": []
            },
            "env": {
                "type": "object",
                "description": "Environment variables for the debugged program.",
                "default": {}
            },
            "envFile": {
                "type": ["string", "array"],
                "items": {
                    "type": "string"
                },
                "description": "Path(s) to files with environment variables.",
                "default": ""
            },
            "buildFlags": {
                "type": ["string", "array"],
                "items": {
                    "type": "string"
                },
                "description": "Flags for the Go compiler.",
                "default": []
            },
            "output": {
                "type": "string",
                "description": "Output path for the binary.",
                "default": "debug"
            },
            "mode": {
                "enum": [ "debug", "test", "exec", "replay", "core"],
                "description": "Debug mode for launch configuration.",
            },
            "traceDirPath": {
                "type": "string",
                "description": "Directory for record trace (for 'replay' mode).",
                "default": ""
            },
            "coreFilePath": {
                "type": "string",
                "description": "Path to core dump file (for 'core' mode).",
                "default": ""
            }
        });

        // Create attach-specific properties
        let attach_properties = json!({
            "processId": {
                "anyOf": [
                    {
                        "enum": ["${command:pickProcess}", "${command:pickGoProcess}"],
                        "description": "Use process picker to select a process."
                    },
                    {
                        "type": "string",
                        "description": "Process name to attach to."
                    },
                    {
                        "type": "number",
                        "description": "Process ID to attach to."
                    }
                ],
                "default": 0
            },
            "mode": {
                "enum": ["local", "remote"],
                "description": "Local or remote debugging.",
                "default": "local"
            },
            "remotePath": {
                "type": "string",
                "description": "Path to source on remote machine.",
                "markdownDeprecationMessage": "Use `substitutePath` instead.",
                "default": ""
            }
        });

        // Create the final schema
        json!({
            "oneOf": [
                {
                    "allOf": [
                        {
                            "type": "object",
                            "required": ["request"],
                            "properties": {
                                "request": {
                                    "type": "string",
                                    "enum": ["launch"],
                                    "description": "Request to launch a new process"
                                }
                            }
                        },
                        {
                            "type": "object",
                            "properties": common_properties
                        },
                        {
                            "type": "object",
                            "required": ["program", "mode"],
                            "properties": launch_properties
                        }
                    ]
                },
                {
                    "allOf": [
                        {
                            "type": "object",
                            "required": ["request"],
                            "properties": {
                                "request": {
                                    "type": "string",
                                    "enum": ["attach"],
                                    "description": "Request to attach to an existing process"
                                }
                            }
                        },
                        {
                            "type": "object",
                            "properties": common_properties
                        },
                        {
                            "type": "object",
                            "required": ["mode"],
                            "properties": attach_properties
                        }
                    ]
                }
            ]
        })
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

    let mut env_vars = HashMap::default();
    for path in env_files {
        let Some(path) = path
            .and_then(|s| PathBuf::from_str(s).ok())
            .and_then(rebase_path)
        else {
            continue;
        };

        if let Ok(file) = fs.open_sync(&path).await {
            let file_envs: HashMap<String, String> = dotenvy::from_read_iter(file)
                .filter_map(Result::ok)
                .collect();
            envs.extend(file_envs.iter().map(|(k, v)| (k.clone(), v.clone())));
            env_vars.extend(file_envs);
        } else {
            warn!("While starting Go debug session: failed to read env file {path:?}");
        };
    }

    let mut env_obj: serde_json::Map<String, Value> = serde_json::Map::new();

    for (k, v) in env_vars {
        env_obj.insert(k, Value::String(v));
    }

    if let Some(existing_env) = config.get("env").and_then(|v| v.as_object()) {
        for (k, v) in existing_env {
            env_obj.insert(k.clone(), v.clone());
        }
    }

    if !env_obj.is_empty() {
        config.insert("env".to_string(), Value::Object(env_obj));
    }

    // remove envFile now that it's been handled
    config.remove("envFile");
    Some(())
}
