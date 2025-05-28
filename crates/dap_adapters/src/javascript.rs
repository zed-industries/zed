use adapters::latest_github_release;
use anyhow::{Context as _, anyhow};
use dap::{
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest,
    adapters::DebugTaskDefinition,
};
use gpui::AsyncApp;
use std::{collections::HashMap, path::PathBuf, sync::OnceLock};
use task::DebugRequest;
use util::ResultExt;

use crate::*;

#[derive(Debug, Default)]
pub(crate) struct JsDebugAdapter {
    checked: OnceLock<()>,
}

impl JsDebugAdapter {
    const ADAPTER_NAME: &'static str = "JavaScript";
    const ADAPTER_NPM_NAME: &'static str = "vscode-js-debug";
    const ADAPTER_PATH: &'static str = "js-debug/src/dapDebugServer.js";

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<AdapterVersion> {
        let release = latest_github_release(
            &format!("{}/{}", "microsoft", Self::ADAPTER_NPM_NAME),
            true,
            false,
            delegate.http_client(),
        )
        .await?;

        let asset_name = format!("js-debug-dap-{}.tar.gz", release.tag_name);

        Ok(AdapterVersion {
            tag_name: release.tag_name,
            url: release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .with_context(|| format!("no asset found matching {asset_name:?}"))?
                .browser_download_url
                .clone(),
        })
    }

    async fn get_installed_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        task_definition: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = if let Some(user_installed_path) = user_installed_path {
            user_installed_path
        } else {
            let adapter_path = paths::debug_adapters_dir().join(self.name().as_ref());

            let file_name_prefix = format!("{}_", self.name());

            util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
                file_name.starts_with(&file_name_prefix)
            })
            .await
            .context("Couldn't find JavaScript dap directory")?
        };

        let tcp_connection = task_definition.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let ret = DebugAdapterBinary {
            command: delegate
                .node_runtime()
                .binary_path()
                .await?
                .to_string_lossy()
                .into_owned(),
            arguments: vec![
                adapter_path
                    .join(Self::ADAPTER_PATH)
                    .to_string_lossy()
                    .to_string(),
                port.to_string(),
                host.to_string(),
            ],
            cwd: Some(delegate.worktree_root_path().to_path_buf()),
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
        };
        Ok(dbg!(ret))
    }
}

#[async_trait(?Send)]
impl DebugAdapter for JsDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn validate_config(
        &self,
        config: &serde_json::Value,
    ) -> Result<dap::StartDebuggingRequestArgumentsRequest> {
        match config.get("request") {
            Some(val) if val == "launch" => {
                if config.get("program").is_none() && config.get("url").is_none() {
                    return Err(anyhow!(
                        "either program or url is required for launch request"
                    ));
                }
                Ok(StartDebuggingRequestArgumentsRequest::Launch)
            }
            Some(val) if val == "attach" => {
                if !config.get("processId").is_some_and(|val| val.is_u64()) {
                    return Err(anyhow!("processId must be a number"));
                }
                Ok(StartDebuggingRequestArgumentsRequest::Attach)
            }
            _ => Err(anyhow!("missing or invalid request field in config")),
        }
    }

    fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        let mut args = json!({
            "type": "pwa-node",
            "request": match zed_scenario.request {
                DebugRequest::Launch(_) => "launch",
                DebugRequest::Attach(_) => "attach",
            },
        });

        let map = args.as_object_mut().unwrap();
        match &zed_scenario.request {
            DebugRequest::Attach(attach) => {
                map.insert("processId".into(), attach.process_id.into());
            }
            DebugRequest::Launch(launch) => {
                if launch.program.starts_with("http://") {
                    map.insert("url".into(), launch.program.clone().into());
                } else {
                    map.insert("program".into(), launch.program.clone().into());
                }

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
        };

        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            build: None,
            config: args,
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
                                "type": {
                                    "type": "string",
                                    "enum": ["pwa-node", "node", "chrome", "pwa-chrome", "edge", "pwa-edge"],
                                    "description": "The type of debug session",
                                    "default": "pwa-node"
                                },
                                "program": {
                                    "type": "string",
                                    "description": "Path to the program or file to debug"
                                },
                                "cwd": {
                                    "type": "string",
                                    "description": "Absolute path to the working directory of the program being debugged"
                                },
                                "args": {
                                    "type": ["array", "string"],
                                    "description": "Command line arguments passed to the program",
                                    "items": {
                                        "type": "string"
                                    },
                                    "default": []
                                },
                                "env": {
                                    "type": "object",
                                    "description": "Environment variables passed to the program",
                                    "default": {}
                                },
                                "envFile": {
                                    "type": ["string", "array"],
                                    "description": "Path to a file containing environment variable definitions",
                                    "items": {
                                        "type": "string"
                                    }
                                },
                                "stopOnEntry": {
                                    "type": "boolean",
                                    "description": "Automatically stop program after launch",
                                    "default": false
                                },
                                "runtimeExecutable": {
                                    "type": ["string", "null"],
                                    "description": "Runtime to use, an absolute path or the name of a runtime available on PATH",
                                    "default": "node"
                                },
                                "runtimeArgs": {
                                    "type": ["array", "null"],
                                    "description": "Arguments passed to the runtime executable",
                                    "items": {
                                        "type": "string"
                                    },
                                    "default": []
                                },
                                "outFiles": {
                                    "type": "array",
                                    "description": "Glob patterns for locating generated JavaScript files",
                                    "items": {
                                        "type": "string"
                                    },
                                    "default": ["${ZED_WORKTREE_ROOT}/**/*.js", "!**/node_modules/**"]
                                },
                                "sourceMaps": {
                                    "type": "boolean",
                                    "description": "Use JavaScript source maps if they exist",
                                    "default": true
                                },
                                "sourceMapPathOverrides": {
                                    "type": "object",
                                    "description": "Rewrites the locations of source files from what the sourcemap says to their locations on disk",
                                    "default": {}
                                },
                                "restart": {
                                    "type": ["boolean", "object"],
                                    "description": "Restart session after Node.js has terminated",
                                    "default": false
                                },
                                "trace": {
                                    "type": ["boolean", "object"],
                                    "description": "Enables logging of the Debug Adapter",
                                    "default": false
                                },
                                "console": {
                                    "type": "string",
                                    "enum": ["internalConsole", "integratedTerminal"],
                                    "description": "Where to launch the debug target",
                                    "default": "internalConsole"
                                },
                                // Browser-specific
                                "url": {
                                    "type": ["string", "null"],
                                    "description": "Will navigate to this URL and attach to it (browser debugging)"
                                },
                                "webRoot": {
                                    "type": "string",
                                    "description": "Workspace absolute path to the webserver root",
                                    "default": "${ZED_WORKTREE_ROOT}"
                                },
                                "userDataDir": {
                                    "type": ["string", "boolean"],
                                    "description": "Path to a custom Chrome user profile (browser debugging)",
                                    "default": true
                                },
                                "skipFiles": {
                                    "type": "array",
                                    "description": "An array of glob patterns for files to skip when debugging",
                                    "items": {
                                        "type": "string"
                                    },
                                    "default": ["<node_internals>/**"]
                                },
                                "timeout": {
                                    "type": "number",
                                    "description": "Retry for this number of milliseconds to connect to the debug adapter",
                                    "default": 10000
                                },
                                "resolveSourceMapLocations": {
                                    "type": ["array", "null"],
                                    "description": "A list of minimatch patterns for source map resolution",
                                    "items": {
                                        "type": "string"
                                    }
                                }
                            },
                            "oneOf": [
                                { "required": ["program"] },
                                { "required": ["url"] }
                            ]
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
                                "type": {
                                    "type": "string",
                                    "enum": ["pwa-node", "node", "chrome", "pwa-chrome", "edge", "pwa-edge"],
                                    "description": "The type of debug session",
                                    "default": "pwa-node"
                                },
                                "processId": {
                                    "type": ["string", "number"],
                                    "description": "ID of process to attach to (Node.js debugging)"
                                },
                                "port": {
                                    "type": "number",
                                    "description": "Debug port to attach to",
                                    "default": 9229
                                },
                                "address": {
                                    "type": "string",
                                    "description": "TCP/IP address of the process to be debugged",
                                    "default": "localhost"
                                },
                                "restart": {
                                    "type": ["boolean", "object"],
                                    "description": "Restart session after Node.js has terminated",
                                    "default": false
                                },
                                "sourceMaps": {
                                    "type": "boolean",
                                    "description": "Use JavaScript source maps if they exist",
                                    "default": true
                                },
                                "sourceMapPathOverrides": {
                                    "type": "object",
                                    "description": "Rewrites the locations of source files from what the sourcemap says to their locations on disk",
                                    "default": {}
                                },
                                "outFiles": {
                                    "type": "array",
                                    "description": "Glob patterns for locating generated JavaScript files",
                                    "items": {
                                        "type": "string"
                                    },
                                    "default": ["${ZED_WORKTREE_ROOT}/**/*.js", "!**/node_modules/**"]
                                },
                                "url": {
                                    "type": "string",
                                    "description": "Will search for a page with this URL and attach to it (browser debugging)"
                                },
                                "webRoot": {
                                    "type": "string",
                                    "description": "Workspace absolute path to the webserver root",
                                    "default": "${ZED_WORKTREE_ROOT}"
                                },
                                "skipFiles": {
                                    "type": "array",
                                    "description": "An array of glob patterns for files to skip when debugging",
                                    "items": {
                                        "type": "string"
                                    },
                                    "default": ["<node_internals>/**"]
                                },
                                "timeout": {
                                    "type": "number",
                                    "description": "Retry for this number of milliseconds to connect to the debug adapter",
                                    "default": 10000
                                },
                                "resolveSourceMapLocations": {
                                    "type": ["array", "null"],
                                    "description": "A list of minimatch patterns for source map resolution",
                                    "items": {
                                        "type": "string"
                                    }
                                },
                                "remoteRoot": {
                                    "type": ["string", "null"],
                                    "description": "Path to the remote directory containing the program"
                                },
                                "localRoot": {
                                    "type": ["string", "null"],
                                    "description": "Path to the local directory containing the program"
                                }
                            },
                            "oneOf": [
                                { "required": ["processId"] },
                                { "required": ["port"] }
                            ]
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
        user_installed_path: Option<PathBuf>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        if self.checked.set(()).is_ok() {
            delegate.output_to_console(format!("Checking latest version of {}...", self.name()));
            if let Some(version) = self.fetch_latest_adapter_version(delegate).await.log_err() {
                adapters::download_adapter_from_github(
                    self.name(),
                    version,
                    adapters::DownloadedFileType::GzipTar,
                    delegate.as_ref(),
                )
                .await?;
            } else {
                delegate.output_to_console(format!("{} debug adapter is up to date", self.name()));
            }
        }

        self.get_installed_binary(delegate, &config, user_installed_path, cx)
            .await
    }
}
