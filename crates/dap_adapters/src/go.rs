use dap::{
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest,
    adapters::DebugTaskDefinition,
};
use gpui::{AsyncApp, SharedString};
use language::LanguageName;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};

use crate::*;

#[derive(Default, Debug)]
pub(crate) struct GoDebugAdapter;

impl GoDebugAdapter {
    const ADAPTER_NAME: &'static str = "Delve";
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
                "default": "${workspaceFolder}"
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
                "enum": ["internalConsole", "integratedTerminal", "externalTerminal"]
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
                "default": "${workspaceFolder}"
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
                "enum": ["auto", "debug", "test", "exec", "replay", "core"],
                "description": "Debug mode for launch configuration.",
                "default": "auto"
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
        let map = config
            .as_object()
            .ok_or_else(|| anyhow!("Config isn't an object"))?;

        let request_variant = map["request"]
            .as_str()
            .ok_or_else(|| anyhow!("request is not valid"))?;

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
                    "processId": attach_config.process_id,
                })
            }
            dap::DebugRequest::Launch(launch_config) => json!({
                "program": launch_config.program,
                "cwd": launch_config.cwd,
                "args": launch_config.args,
                "env": launch_config.env_json()
            }),
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
        delegate: &dyn DapDelegate,
        task_definition: DebugTaskDefinition,
        _user_installed_path: Option<PathBuf>,
        _cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let delve_path = delegate
            .which(OsStr::new("dlv"))
            .and_then(|p| p.to_str().map(|p| p.to_string()))
            .ok_or(anyhow!("Dlv not found in path"))?;

        let tcp_connection = task_definition.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        Ok(DebugAdapterBinary {
            command: delve_path,
            arguments: vec![
                "dap".into(),
                "--listen".into(),
                format!("{}:{}", host, port),
            ],
            cwd: None,
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
