use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use dap::adapters::{DebugTaskDefinition, latest_github_release};
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

    fn dap_schema(&self) -> serde_json::Value {
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
        user_args: Option<Vec<String>>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let mut command = user_installed_path
            .map(|p| p.to_string_lossy().into_owned())
            .or(self.path_to_codelldb.get().cloned());

        if command.is_none() {
            delegate.output_to_console(format!("Checking latest version of {}...", self.name()));
            let adapter_path = paths::debug_adapters_dir().join(&Self::ADAPTER_NAME);
            let version_path = match self.fetch_latest_adapter_version(delegate).await {
                Ok(version) => {
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
                }
                Err(e) => {
                    delegate.output_to_console("Unable to fetch latest version".to_string());
                    log::error!("Error fetching latest version of {}: {}", self.name(), e);
                    delegate.output_to_console(format!(
                        "Searching for adapters in: {}",
                        adapter_path.display()
                    ));
                    let mut paths = delegate
                        .fs()
                        .read_dir(&adapter_path)
                        .await
                        .context("No cached adapter directory")?;
                    paths
                        .next()
                        .await
                        .context("No cached adapter found")?
                        .context("No cached adapter found")?
                }
            };
            let adapter_dir = version_path.join("extension").join("adapter");
            let path = adapter_dir.join("codelldb").to_string_lossy().into_owned();
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
                            .is_some_and(|array| array.iter().all(Value::is_string))
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
