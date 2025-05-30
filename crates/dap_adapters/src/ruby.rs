use anyhow::Result;
use async_trait::async_trait;
use dap::{
    DebugRequest, StartDebuggingRequestArguments,
    adapters::{
        DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName, DebugTaskDefinition,
    },
};
use gpui::{AsyncApp, SharedString};
use language::LanguageName;
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use task::{DebugScenario, ZedDebugConfig};
use util::command::new_smol_command;

#[derive(Default)]
pub(crate) struct RubyDebugAdapter;

impl RubyDebugAdapter {
    const ADAPTER_NAME: &'static str = "Ruby";
}

#[async_trait(?Send)]
impl DebugAdapter for RubyDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(SharedString::new_static("Ruby").into())
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
                                "showProtocolLog": {
                                    "type": "boolean",
                                    "description": "Show a log of DAP requests, events, and responses",
                                    "default": false
                                },
                                "useBundler": {
                                    "type": "boolean",
                                    "description": "Execute Ruby programs with `bundle exec` instead of directly",
                                    "default": false
                                },
                                "bundlePath": {
                                    "type": "string",
                                    "description": "Location of the bundle executable"
                                },
                                "rdbgPath": {
                                    "type": "string",
                                    "description": "Location of the rdbg executable"
                                },
                                "askParameters": {
                                    "type": "boolean",
                                    "description": "Ask parameters at first."
                                },
                                "debugPort": {
                                    "type": "string",
                                    "description": "UNIX domain socket name or TPC/IP host:port"
                                },
                                "waitLaunchTime": {
                                    "type": "number",
                                    "description": "Wait time before connection in milliseconds"
                                },
                                "localfs": {
                                    "type": "boolean",
                                    "description": "true if the VSCode and debugger run on a same machine",
                                    "default": false
                                },
                                "useTerminal": {
                                    "type": "boolean",
                                    "description": "Create a new terminal and then execute commands there",
                                    "default": false
                                }
                            }
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
                            "properties": {
                                "rdbgPath": {
                                    "type": "string",
                                    "description": "Location of the rdbg executable"
                                },
                                "debugPort": {
                                    "type": "string",
                                    "description": "UNIX domain socket name or TPC/IP host:port"
                                },
                                "showProtocolLog": {
                                    "type": "boolean",
                                    "description": "Show a log of DAP requests, events, and responses",
                                    "default": false
                                },
                                "localfs": {
                                    "type": "boolean",
                                    "description": "true if the VSCode and debugger run on a same machine",
                                    "default": false
                                },
                                "localfsMap": {
                                    "type": "string",
                                    "description": "Specify pairs of remote root path and local root path like `/remote_dir:/local_dir`. You can specify multiple pairs like `/rem1:/loc1,/rem2:/loc2` by concatenating with `,`."
                                },
                                "env": {
                                    "type": "object",
                                    "description": "Additional environment variables to pass to the rdbg process",
                                    "default": {}
                                }
                            }
                        }
                    ]
                }
            ]
        })
    }

    fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        let mut config = serde_json::Map::new();

        match &zed_scenario.request {
            DebugRequest::Launch(launch) => {
                config.insert("request".to_string(), json!("launch"));
                config.insert("script".to_string(), json!(launch.program));
                config.insert("command".to_string(), json!("ruby"));

                if !launch.args.is_empty() {
                    config.insert("args".to_string(), json!(launch.args));
                }

                if !launch.env.is_empty() {
                    config.insert("env".to_string(), json!(launch.env));
                }

                if let Some(cwd) = &launch.cwd {
                    config.insert("cwd".to_string(), json!(cwd));
                }

                // Ruby stops on entry so there's no need to handle that case
            }
            DebugRequest::Attach(attach) => {
                config.insert("request".to_string(), json!("attach"));

                config.insert("processId".to_string(), json!(attach.process_id));
            }
        }

        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            config: serde_json::Value::Object(config),
            tcp_connection: None,
            build: None,
        })
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

        let arguments = vec![
            "--open".to_string(),
            format!("--port={}", port),
            format!("--host={}", host),
        ];

        Ok(DebugAdapterBinary {
            command: rdbg_path.to_string_lossy().to_string(),
            arguments,
            connection: Some(dap::adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
            cwd: None,
            envs: std::collections::HashMap::default(),
            request_args: StartDebuggingRequestArguments {
                request: self.validate_config(&definition.config)?,
                configuration: definition.config.clone(),
            },
        })
    }
}
