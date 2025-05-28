use anyhow::{Context as _, anyhow, bail};
use dap::{
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest,
    adapters::DebugTaskDefinition,
};

use gpui::{AsyncApp, SharedString};
use language::LanguageName;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};
use util;

use crate::*;

#[derive(Default, Debug)]
pub(crate) struct GoDebugAdapter;

impl GoDebugAdapter {
    const ADAPTER_NAME: &'static str = "Delve";
    const DEFAULT_TIMEOUT_MS: u64 = 60000;
}

#[async_trait(?Send)]
impl DebugAdapter for GoDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(SharedString::new_static("Go").into())
    }

    async fn dap_schema(&self) -> serde_json::Value {
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
                            "required": ["processId", "mode"],
                            "properties": attach_properties
                        }
                    ]
                }
            ]
        })
    }

    fn validate_config(
        &self,
        config: &serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest> {
        let map = config.as_object().context("Config isn't an object")?;

        let request_variant = map
            .get("request")
            .and_then(|val| val.as_str())
            .context("request argument is not found or invalid")?;

        match request_variant {
            "launch" => Ok(StartDebuggingRequestArgumentsRequest::Launch),
            "attach" => Ok(StartDebuggingRequestArgumentsRequest::Attach),
            _ => Err(anyhow!("request must be either 'launch' or 'attach'")),
        }
    }

    fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
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
        _user_installed_path: Option<PathBuf>,
        _cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = paths::debug_adapters_dir().join(&Self::ADAPTER_NAME);
        let dlv_path = adapter_path.join("dlv");

        let delve_path = if let Some(path) = delegate.which(OsStr::new("dlv")).await {
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

        let mut tcp_connection = task_definition.tcp_connection.clone().unwrap_or_default();

        if tcp_connection.timeout.is_none()
            || tcp_connection.timeout.unwrap_or(0) < Self::DEFAULT_TIMEOUT_MS
        {
            tcp_connection.timeout = Some(Self::DEFAULT_TIMEOUT_MS);
        }

        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let cwd = task_definition
            .config
            .get("cwd")
            .and_then(|s| s.as_str())
            .map(PathBuf::from)
            .unwrap_or_else(|| delegate.worktree_root_path().to_path_buf());

        let arguments = if cfg!(windows) {
            vec![
                "dap".into(),
                "--listen".into(),
                format!("{}:{}", host, port),
                "--headless".into(),
            ]
        } else {
            vec![
                "dap".into(),
                "--listen".into(),
                format!("{}:{}", host, port),
            ]
        };

        Ok(DebugAdapterBinary {
            command: delve_path,
            arguments,
            cwd: Some(cwd),
            envs: HashMap::default(),
            connection: Some(adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
            request_args: StartDebuggingRequestArguments {
                configuration: task_definition.config.clone(),
                request: self.validate_config(&task_definition.config)?,
            },
        })
    }
}
