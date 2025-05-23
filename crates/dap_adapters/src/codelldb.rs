use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use dap::{
    StartDebuggingRequestArgumentsRequest,
    adapters::{DebugTaskDefinition, latest_github_release},
};
use futures::StreamExt;
use gpui::AsyncApp;
use serde_json::Value;
use task::{DebugRequest, DebugScenario, ZedDebugConfig};
use util::fs::remove_matching;

use crate::*;

#[derive(Default)]
pub(crate) struct CodeLldbDebugAdapter {
    path_to_codelldb: OnceLock<String>,
}

impl CodeLldbDebugAdapter {
    const ADAPTER_NAME: &'static str = "CodeLLDB";

    fn request_args(
        &self,
        task_definition: &DebugTaskDefinition,
    ) -> Result<dap::StartDebuggingRequestArguments> {
        // CodeLLDB uses `name` for a terminal label.
        let mut configuration = task_definition.config.clone();

        configuration
            .as_object_mut()
            .context("CodeLLDB is not a valid json object")?
            .insert(
                "name".into(),
                Value::String(String::from(task_definition.label.as_ref())),
            );

        let request = self.validate_config(&configuration)?;

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

#[async_trait(?Send)]
impl DebugAdapter for CodeLldbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn validate_config(
        &self,
        config: &serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest> {
        let map = config
            .as_object()
            .ok_or_else(|| anyhow!("Config isn't an object"))?;

        let request_variant = map
            .get("request")
            .and_then(|r| r.as_str())
            .ok_or_else(|| anyhow!("request field is required and must be a string"))?;

        match request_variant {
            "launch" => {
                // For launch, verify that one of the required configs exists
                if !(map.contains_key("program")
                    || map.contains_key("targetCreateCommands")
                    || map.contains_key("cargo"))
                {
                    return Err(anyhow!(
                        "launch request requires either 'program', 'targetCreateCommands', or 'cargo' field"
                    ));
                }
                Ok(StartDebuggingRequestArgumentsRequest::Launch)
            }
            "attach" => {
                // For attach, verify that either pid or program exists
                if !(map.contains_key("pid") || map.contains_key("program")) {
                    return Err(anyhow!(
                        "attach request requires either 'pid' or 'program' field"
                    ));
                }
                Ok(StartDebuggingRequestArgumentsRequest::Attach)
            }
            _ => Err(anyhow!(
                "request must be either 'launch' or 'attach', got '{}'",
                request_variant
            )),
        }
    }

    fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
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

    async fn dap_schema(&self) -> serde_json::Value {
        json!({
            "properties": {
                "request": {
                    "type": "string",
                    "enum": ["attach", "launch"],
                    "description": "Debug adapter request type"
                },
                "program": {
                    "type": "string",
                    "description": "Path to the program to debug or attach to"
                },
                "args": {
                    "type": ["array", "string"],
                    "description": "Program arguments"
                },
                "cwd": {
                    "type": "string",
                    "description": "Program working directory"
                },
                "env": {
                    "type": "object",
                    "description": "Additional environment variables",
                    "patternProperties": {
                        ".*": {
                            "type": "string"
                        }
                    }
                },
                "envFile": {
                    "type": "string",
                    "description": "File to read the environment variables from"
                },
                "stdio": {
                    "type": ["null", "string", "array", "object"],
                    "description": "Destination for stdio streams: null = send to debugger console or a terminal, \"<path>\" = attach to a file/tty/fifo"
                },
                "terminal": {
                    "type": "string",
                    "enum": ["integrated", "console"],
                    "description": "Terminal type to use",
                    "default": "integrated"
                },
                "console": {
                    "type": "string",
                    "enum": ["integratedTerminal", "internalConsole"],
                    "description": "Terminal type to use (compatibility alias of 'terminal')"
                },
                "stopOnEntry": {
                    "type": "boolean",
                    "description": "Automatically stop debuggee after launch",
                    "default": false
                },
                "initCommands": {
                    "type": "array",
                    "description": "Initialization commands executed upon debugger startup",
                    "items": {
                        "type": "string"
                    }
                },
                "targetCreateCommands": {
                    "type": "array",
                    "description": "Commands that create the debug target",
                    "items": {
                        "type": "string"
                    }
                },
                "preRunCommands": {
                    "type": "array",
                    "description": "Commands executed just before the program is launched",
                    "items": {
                        "type": "string"
                    }
                },
                "processCreateCommands": {
                    "type": "array",
                    "description": "Commands that create the debuggee process",
                    "items": {
                        "type": "string"
                    }
                },
                "postRunCommands": {
                    "type": "array",
                    "description": "Commands executed just after the program has been launched",
                    "items": {
                        "type": "string"
                    }
                },
                "preTerminateCommands": {
                    "type": "array",
                    "description": "Commands executed just before the debuggee is terminated or disconnected from",
                    "items": {
                        "type": "string"
                    }
                },
                "exitCommands": {
                    "type": "array",
                    "description": "Commands executed at the end of debugging session",
                    "items": {
                        "type": "string"
                    }
                },
                "expressions": {
                    "type": "string",
                    "enum": ["simple", "python", "native"],
                    "description": "The default evaluator type used for expressions"
                },
                "sourceMap": {
                    "type": "object",
                    "description": "Source path remapping between the build machine and the local machine",
                    "patternProperties": {
                        ".*": {
                            "type": ["string", "null"]
                        }
                    }
                },
                "relativePathBase": {
                    "type": "string",
                    "description": "Base directory used for resolution of relative source paths. Defaults to the workspace folder"
                },
                "sourceLanguages": {
                    "type": "array",
                    "description": "A list of source languages to enable language-specific features for",
                    "items": {
                        "type": "string"
                    }
                },
                "reverseDebugging": {
                    "type": "boolean",
                    "description": "Enable reverse debugging",
                    "default": false
                },
                "breakpointMode": {
                    "type": "string",
                    "enum": ["path", "file"],
                    "description": "Specifies how source breakpoints should be set"
                },
                "pid": {
                    "type": ["integer", "string"],
                    "description": "Process id to attach to"
                },
                "waitFor": {
                    "type": "boolean",
                    "description": "Wait for the process to launch (MacOS only)",
                    "default": false
                }
            },
            "required": ["request"],
            "allOf": [
                {
                    "if": {
                        "properties": {
                            "request": {
                                "enum": ["launch"]
                            }
                        }
                    },
                    "then": {
                        "oneOf": [
                            {
                                "required": ["program"]
                            },
                            {
                                "required": ["targetCreateCommands"]
                            },
                            {
                                "required": ["cargo"]
                            }
                        ]
                    }
                },
                {
                    "if": {
                        "properties": {
                            "request": {
                                "enum": ["attach"]
                            }
                        }
                    },
                    "then": {
                        "oneOf": [
                            {
                                "required": ["pid"]
                            },
                            {
                                "required": ["program"]
                            }
                        ]
                    }
                }
            ]
        })
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
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
                        self.name(),
                        version.clone(),
                        adapters::DownloadedFileType::Vsix,
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
            // todo("windows")
            #[cfg(not(windows))]
            {
                use smol::fs;

                fs::set_permissions(
                    &path,
                    <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
                )
                .await
                .with_context(|| format!("Settings executable permissions to {path:?}"))?;

                let lldb_binaries_dir = version_path.join("extension").join("lldb").join("bin");
                let mut lldb_binaries =
                    fs::read_dir(&lldb_binaries_dir).await.with_context(|| {
                        format!("reading lldb binaries dir contents {lldb_binaries_dir:?}")
                    })?;
                while let Some(binary) = lldb_binaries.next().await {
                    let binary_entry = binary?;
                    let path = binary_entry.path();
                    fs::set_permissions(
                        &path,
                        <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
                    )
                    .await
                    .with_context(|| format!("Settings executable permissions to {path:?}"))?;
                }
            }
            self.path_to_codelldb.set(path.clone()).ok();
            command = Some(path);
        };

        Ok(DebugAdapterBinary {
            command: command.unwrap(),
            cwd: Some(delegate.worktree_root_path().to_path_buf()),
            arguments: vec![
                "--settings".into(),
                json!({"sourceLanguages": ["cpp", "rust"]}).to_string(),
            ],
            request_args: self.request_args(&config)?,
            envs: HashMap::default(),
            connection: None,
        })
    }
}
