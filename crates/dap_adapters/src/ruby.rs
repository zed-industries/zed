use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use collections::{FxHashMap, HashMap};
use dap::{
    DebugRequest, StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest,
    adapters::{
        DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName, DebugTaskDefinition,
    },
};
use gpui::{AsyncApp, SharedString};
use language::LanguageName;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use std::{ffi::OsStr, sync::Arc};
use task::{DebugScenario, ZedDebugConfig};
use util::command::new_smol_command;

#[derive(Default)]
pub(crate) struct RubyDebugAdapter;

impl RubyDebugAdapter {
    const ADAPTER_NAME: &'static str = "Ruby";
}

#[derive(Serialize, Deserialize)]
struct RubyDebugConfig {
    script_or_command: Option<String>,
    script: Option<String>,
    command: Option<String>,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: FxHashMap<String, String>,
    cwd: Option<PathBuf>,
}

#[async_trait(?Send)]
impl DebugAdapter for RubyDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(SharedString::new_static("Ruby").into())
    }

    fn request_kind(&self, _: &serde_json::Value) -> Result<StartDebuggingRequestArgumentsRequest> {
        Ok(StartDebuggingRequestArgumentsRequest::Launch)
    }

    async fn dap_schema(&self) -> serde_json::Value {
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
                            "required": ["script"],
                            "properties": {
                                "command": {
                                    "type": "string",
                                    "description": "Command name (ruby, rake, bin/rails, bundle exec ruby, etc)",
                                    "default": "ruby"
                                },
                                "script": {
                                    "type": "string",
                                    "description": "Absolute path to a Ruby file."
                                },
                                "cwd": {
                                    "type": "string",
                                    "description": "Directory to execute the program in",
                                    "default": "${ZED_WORKTREE_ROOT}"
                                },
                                "args": {
                                    "type": "array",
                                    "description": "Command line arguments passed to the program",
                                    "items": {
                                        "type": "string"
                                    },
                                    "default": []
                                },
                                "env": {
                                    "type": "object",
                                    "description": "Additional environment variables to pass to the debugging (and debugged) process",
                                    "default": {}
                                },
                                // "showProtocolLog": {
                                //     "type": "boolean",
                                //     "description": "Show a log of DAP requests, events, and responses",
                                //     "default": false
                                // },
                                // "useBundler": {
                                //     "type": "boolean",
                                //     "description": "Execute Ruby programs with `bundle exec` instead of directly",
                                //     "default": false
                                // },
                                // "bundlePath": {
                                //     "type": "string",
                                //     "description": "Location of the bundle executable"
                                // },
                                // "rdbgPath": {
                                //     "type": "string",
                                //     "description": "Location of the rdbg executable"
                                // },
                                // "askParameters": {
                                //     "type": "boolean",
                                //     "description": "Ask parameters at first."
                                // },
                                // "debugPort": {
                                //     "type": "string",
                                //     "description": "UNIX domain socket name or TPC/IP host:port"
                                // },
                                // "waitLaunchTime": {
                                //     "type": "number",
                                //     "description": "Wait time before connection in milliseconds"
                                // },
                                // "localfs": {
                                //     "type": "boolean",
                                //     "description": "true if the zed and debugger run on a same machine",
                                //     "default": false
                                // },
                                // "useTerminal": {
                                //     "type": "boolean",
                                //     "description": "Create a new terminal and then execute commands there",
                                //     "default": false
                                // }
                            }
                        }
                    ]
                },
                // {
                //     "allOf": [
                //         {
                //             "type": "object",
                //             "required": ["request"],
                //             "properties": {
                //                 "request": {
                //                     "type": "string",
                //                     "enum": ["attach"],
                //                     "description": "Request to attach to an existing process"
                //                 }
                //             }
                //         },
                //         {
                //             "type": "object",
                //             "properties": {
                //                 "rdbgPath": {
                //                     "type": "string",
                //                     "description": "Location of the rdbg executable"
                //                 },
                //                 "debugPort": {
                //                     "type": "string",
                //                     "description": "UNIX domain socket name or TPC/IP host:port"
                //                 },
                //                 "showProtocolLog": {
                //                     "type": "boolean",
                //                     "description": "Show a log of DAP requests, events, and responses",
                //                     "default": false
                //                 },
                //                 "localfs": {
                //                     "type": "boolean",
                //                     "description": "true if the VSCode and debugger run on a same machine",
                //                     "default": false
                //                 },
                //                 "localfsMap": {
                //                     "type": "string",
                //                     "description": "Specify pairs of remote root path and local root path like `/remote_dir:/local_dir`. You can specify multiple pairs like `/rem1:/loc1,/rem2:/loc2` by concatenating with `,`."
                //                 },
                //                 "env": {
                //                     "type": "object",
                //                     "description": "Additional environment variables to pass to the rdbg process",
                //                     "default": {}
                //                 }
                //             }
                //         }
                //     ]
                // }
            ]
        })
    }

    fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        match zed_scenario.request {
            DebugRequest::Launch(launch) => {
                let config = RubyDebugConfig {
                    script_or_command: Some(launch.program),
                    script: None,
                    command: None,
                    args: launch.args,
                    env: launch.env,
                    cwd: launch.cwd.clone(),
                };

                let config = serde_json::to_value(config)?;

                Ok(DebugScenario {
                    adapter: zed_scenario.adapter,
                    label: zed_scenario.label,
                    config,
                    tcp_connection: None,
                    build: None,
                })
            }
            DebugRequest::Attach(_) => {
                anyhow::bail!("Attach requests are unsupported");
            }
        }
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        definition: &DebugTaskDefinition,
        _user_installed_path: Option<PathBuf>,
        _cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = paths::debug_adapters_dir().join(self.name().as_ref());
        let mut rdbg_path = adapter_path.join("rdbg");
        if !delegate.fs().is_file(&rdbg_path).await {
            match delegate.which("rdbg".as_ref()).await {
                Some(path) => rdbg_path = path,
                None => {
                    delegate.output_to_console(
                        "rdbg not found on path, trying `gem install debug`".to_string(),
                    );
                    let output = new_smol_command("gem")
                        .arg("install")
                        .arg("--no-document")
                        .arg("--bindir")
                        .arg(adapter_path)
                        .arg("debug")
                        .output()
                        .await?;
                    anyhow::ensure!(
                        output.status.success(),
                        "Failed to install rdbg:\n{}",
                        String::from_utf8_lossy(&output.stderr).to_string()
                    );
                }
            }
        }

        let tcp_connection = definition.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;
        let ruby_config = serde_json::from_value::<RubyDebugConfig>(definition.config.clone())?;

        // let DebugRequest::Launch(launch) = definition.request.clone() else {
        //     anyhow::bail!("rdbg does not yet support attaching");
        // };

        let mut arguments = vec![
            "--open".to_string(),
            format!("--port={}", port),
            format!("--host={}", host),
        ];

        if let Some(script) = &ruby_config.script {
            arguments.push(script.clone());
        } else if let Some(command) = &ruby_config.command {
            arguments.push("--command".to_string());
            arguments.push(command.clone());
        } else if let Some(command_or_script) = &ruby_config.script_or_command {
            if delegate
                .which(OsStr::new(&command_or_script))
                .await
                .is_some()
            {
                arguments.push("--command".to_string());
            }
            arguments.push(command_or_script.clone());
        } else {
            bail!("Ruby debug config must have 'script' or 'command' args");
        }

        arguments.extend(ruby_config.args);

        Ok(DebugAdapterBinary {
            command: rdbg_path.to_string_lossy().to_string(),
            arguments,
            connection: Some(dap::adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
            cwd: ruby_config.cwd,
            envs: ruby_config.env.into_iter().collect(),
            request_args: StartDebuggingRequestArguments {
                request: self.request_kind(&definition.config)?,
                configuration: definition.config.clone(),
            },
        })
    }
}
