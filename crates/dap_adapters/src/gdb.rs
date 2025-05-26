use std::{collections::HashMap, ffi::OsStr};

use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use dap::{StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use gpui::AsyncApp;
use task::{DebugScenario, ZedDebugConfig};

use crate::*;

#[derive(Default)]
pub(crate) struct GdbDebugAdapter;

impl GdbDebugAdapter {
    const ADAPTER_NAME: &'static str = "GDB";
}

#[async_trait(?Send)]
impl DebugAdapter for GdbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        let mut obj = serde_json::Map::default();

        match &zed_scenario.request {
            dap::DebugRequest::Attach(attach) => {
                obj.insert("pid".into(), attach.process_id.into());
            }

            dap::DebugRequest::Launch(launch) => {
                obj.insert("program".into(), launch.program.clone().into());

                if !launch.args.is_empty() {
                    obj.insert("args".into(), launch.args.clone().into());
                }

                if !launch.env.is_empty() {
                    obj.insert("env".into(), launch.env_json());
                }

                if let Some(stop_on_entry) = zed_scenario.stop_on_entry {
                    obj.insert(
                        "stopAtBeginningOfMainSubprogram".into(),
                        stop_on_entry.into(),
                    );
                }
                if let Some(cwd) = launch.cwd.as_ref() {
                    obj.insert("cwd".into(), cwd.to_string_lossy().into_owned().into());
                }
            }
        }

        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            build: None,
            config: serde_json::Value::Object(obj),
            tcp_connection: None,
        })
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
                            "properties": {
                                "program": {
                                    "type": "string",
                                    "description": "The program to debug. This corresponds to the GDB 'file' command."
                                },
                                "args": {
                                    "type": "array",
                                    "items": {
                                        "type": "string"
                                    },
                                    "description": "Command line arguments passed to the program. These strings are provided as command-line arguments to the inferior.",
                                    "default": []
                                },
                                "cwd": {
                                    "type": "string",
                                    "description": "Working directory for the debugged program. GDB will change its working directory to this directory."
                                },
                                "env": {
                                    "type": "object",
                                    "description": "Environment variables for the debugged program. Each key is the name of an environment variable; each value is the value of that variable."
                                },
                                "stopAtBeginningOfMainSubprogram": {
                                    "type": "boolean",
                                    "description": "When true, GDB will set a temporary breakpoint at the program's main procedure, like the 'start' command.",
                                    "default": false
                                },
                                "stopOnEntry": {
                                    "type": "boolean",
                                    "description": "When true, GDB will set a temporary breakpoint at the program's first instruction, like the 'starti' command.",
                                    "default": false
                                }
                            },
                            "required": ["program"]
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
                                "pid": {
                                    "type": "number",
                                    "description": "The process ID to which GDB should attach."
                                },
                                "program": {
                                    "type": "string",
                                    "description": "The program to debug (optional). This corresponds to the GDB 'file' command. In many cases, GDB can determine which program is running automatically."
                                },
                                "target": {
                                    "type": "string",
                                    "description": "The target to which GDB should connect. This is passed to the 'target remote' command."
                                }
                            },
                            "required": ["pid"]
                        }
                    ]
                }
            ]
        })
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<std::path::PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let user_setting_path = user_installed_path
            .filter(|p| p.exists())
            .and_then(|p| p.to_str().map(|s| s.to_string()));

        let gdb_path = delegate
            .which(OsStr::new("gdb"))
            .await
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .context("Could not find gdb in path");

        if gdb_path.is_err() && user_setting_path.is_none() {
            bail!("Could not find gdb path or it's not installed");
        }

        let gdb_path = user_setting_path.unwrap_or(gdb_path?);

        let request_args = StartDebuggingRequestArguments {
            request: self.validate_config(&config.config)?,
            configuration: config.config.clone(),
        };

        Ok(DebugAdapterBinary {
            command: gdb_path,
            arguments: vec!["-i=dap".into()],
            envs: HashMap::default(),
            cwd: Some(delegate.worktree_root_path().to_path_buf()),
            connection: None,
            request_args,
        })
    }
}
