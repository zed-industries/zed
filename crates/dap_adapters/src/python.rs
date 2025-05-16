use crate::*;
use dap::{
    DebugRequest, StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest,
    adapters::DebugTaskDefinition,
};
use gpui::{AsyncApp, SharedString};
use language::LanguageName;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf, sync::OnceLock};
use util::ResultExt;

#[derive(Default)]
pub(crate) struct PythonDebugAdapter {
    checked: OnceLock<()>,
}

impl PythonDebugAdapter {
    const ADAPTER_NAME: &'static str = "Debugpy";
    const ADAPTER_PACKAGE_NAME: &'static str = "debugpy";
    const ADAPTER_PATH: &'static str = "src/debugpy/adapter";
    const LANGUAGE_NAME: &'static str = "Python";

    fn request_args(&self, config: &DebugTaskDefinition) -> StartDebuggingRequestArguments {
        // let args = json!({
        //     "request": match config.request {
        //         DebugRequest::Launch(_) => "launch",
        //         DebugRequest::Attach(_) => "attach",
        //     },
        //     "subProcess": true,
        //     "redirectOutput": true,
        // });

        // let map = args.as_object_mut().unwrap();
        // match &config.request {
        //     DebugRequest::Attach(attach) => {
        //         map.insert("processId".into(), attach.process_id.into());
        //     }
        //     DebugRequest::Launch(launch) => {
        //         map.insert("program".into(), launch.program.clone().into());
        //         map.insert("args".into(), launch.args.clone().into());
        //         if !launch.env.is_empty() {
        //             map.insert("env".into(), launch.env_json());
        //         }

        //         if let Some(stop_on_entry) = config.stop_on_entry {
        //             map.insert("stopOnEntry".into(), stop_on_entry.into());
        //         }
        //         if let Some(cwd) = launch.cwd.as_ref() {
        //             map.insert("cwd".into(), cwd.to_string_lossy().into_owned().into());
        //         }
        //     }
        // }
        //
        // let mut adapter_config = config.config.clone();
        // util::merge_json_value_into(args, &mut adapter_config);
        let request = match config.request {
            task::Request::Launch => StartDebuggingRequestArgumentsRequest::Launch,
            task::Request::Attach => StartDebuggingRequestArgumentsRequest::Attach,
        };

        StartDebuggingRequestArguments {
            configuration: config.config.clone(),
            request,
        }
    }
    async fn fetch_latest_adapter_version(
        &self,
        delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion> {
        let github_repo = GithubRepo {
            repo_name: Self::ADAPTER_PACKAGE_NAME.into(),
            repo_owner: "microsoft".into(),
        };

        adapters::fetch_latest_adapter_version_from_github(github_repo, delegate).await
    }

    async fn install_binary(
        &self,
        version: AdapterVersion,
        delegate: &dyn DapDelegate,
    ) -> Result<()> {
        let version_path = adapters::download_adapter_from_github(
            self.name(),
            version,
            adapters::DownloadedFileType::Zip,
            delegate,
        )
        .await?;

        // only needed when you install the latest version for the first time
        if let Some(debugpy_dir) =
            util::fs::find_file_name_in_dir(version_path.as_path(), |file_name| {
                file_name.starts_with("microsoft-debugpy-")
            })
            .await
        {
            // TODO Debugger: Rename folder instead of moving all files to another folder
            // We're doing unnecessary IO work right now
            util::fs::move_folder_files_to_folder(debugpy_dir.as_path(), version_path.as_path())
                .await?;
        }

        Ok(())
    }

    async fn get_installed_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        const BINARY_NAMES: [&str; 3] = ["python3", "python", "py"];
        let tcp_connection = config.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let debugpy_dir = if let Some(user_installed_path) = user_installed_path {
            user_installed_path
        } else {
            let adapter_path = paths::debug_adapters_dir().join(self.name().as_ref());
            let file_name_prefix = format!("{}_", Self::ADAPTER_NAME);

            util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
                file_name.starts_with(&file_name_prefix)
            })
            .await
            .ok_or_else(|| anyhow!("Debugpy directory not found"))?
        };

        let toolchain = delegate
            .toolchain_store()
            .active_toolchain(
                delegate.worktree_id(),
                Arc::from("".as_ref()),
                language::LanguageName::new(Self::LANGUAGE_NAME),
                cx,
            )
            .await;

        let python_path = if let Some(toolchain) = toolchain {
            Some(toolchain.path.to_string())
        } else {
            BINARY_NAMES
                .iter()
                .filter_map(|cmd| {
                    delegate
                        .which(OsStr::new(cmd))
                        .map(|path| path.to_string_lossy().to_string())
                })
                .find(|_| true)
        };

        Ok(DebugAdapterBinary {
            command: python_path.ok_or(anyhow!("failed to find binary path for python"))?,
            arguments: vec![
                debugpy_dir
                    .join(Self::ADAPTER_PATH)
                    .to_string_lossy()
                    .to_string(),
                format!("--port={}", port),
                format!("--host={}", host),
            ],
            connection: Some(adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
            cwd: None,
            envs: HashMap::default(),
            request_args: self.request_args(config),
        })
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PythonDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(SharedString::new_static("Python").into())
    }

    fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> DebugScenario {
        let mut args = json!({
            "request": match zed_scenario.request {
                DebugRequest::Launch(_) => "launch",
                DebugRequest::Attach(_) => "attach",
            },
            "subProcess": true,
            "redirectOutput": true,
        });

        let map = args.as_object_mut().unwrap();
        match &zed_scenario.request {
            DebugRequest::Attach(attach) => {
                map.insert("processId".into(), attach.process_id.into());
            }
            DebugRequest::Launch(launch) => {
                map.insert("program".into(), launch.program.clone().into());
                map.insert("args".into(), launch.args.clone().into());
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

        DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            config: args,
            build: None,
            request: Some(match zed_scenario.request {
                DebugRequest::Launch(_) => task::Request::Launch,
                DebugRequest::Attach(_) => task::Request::Attach,
            }),
            tcp_connection: None,
            stop_on_entry: zed_scenario.stop_on_entry,
        }
    }

    fn dap_schema(&self) -> serde_json::Value {
        json!({
            "properties": {
                "module": {
                    "type": "string",
                    "description": "Name of the module to be debugged."
                },
                "program": {
                    "type": "string",
                    "description": "Absolute path to the program."
                },
                "code": {
                    "type": "string",
                    "description": "Code to execute in string form. Example: \"import debugpy;print(debugpy.__version__)\""
                },
                "python": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "Path python executable and interpreter arguments. Example: [\"/usr/bin/python\", \"-E\"]"
                },
                "args": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "Command line arguments passed to the program."
                },
                "console": {
                    "type": "string",
                    "enum": ["internalConsole", "integratedTerminal", "externalTerminal"],
                    "default": "integratedTerminal",
                    "description": "Sets where to launch the debug target. Default is \"integratedTerminal\"."
                },
                "cwd": {
                    "type": "string",
                    "description": "Absolute path to the working directory of the program being debugged."
                },
                "env": {
                    "type": "object",
                    "additionalProperties": {
                        "type": "string"
                    },
                    "description": "Environment variables defined as a key value pair."
                },

                "django": {
                    "type": "boolean",
                    "default": false,
                    "description": "When true enables Django templates. Default is false."
                },
                "gevent": {
                    "type": "boolean",
                    "default": false,
                    "description": "When true enables debugging of gevent monkey-patched code. Default is false."
                },
                "jinja": {
                    "type": "boolean",
                    "default": false,
                    "description": "When true enables Jinja2 template debugging (e.g. Flask). Default is false."
                },
                "justMyCode": {
                    "type": "boolean",
                    "default": true,
                    "description": "When true debug only user-written code. To debug standard library or anything outside of \"cwd\" use false. Default is true."
                },
                "logToFile": {
                    "type": "boolean",
                    "default": false,
                    "description": "When true enables logging of debugger events to a log file(s). Default is false."
                },
                "pathMappings": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "required": ["localRoot", "remoteRoot"],
                        "properties": {
                            "localRoot": {
                                "type": "string",
                                "description": "Local path"
                            },
                            "remoteRoot": {
                                "type": "string",
                                "description": "Remote path"
                            }
                        }
                    },
                    "description": "Map of local and remote paths. Example: [{\"localRoot\": \"local path\", \"remoteRoot\": \"remote path\"}, ...]"
                },
                "pyramid": {
                    "type": "boolean",
                    "default": false,
                    "description": "When true enables debugging Pyramid applications. Default is false."
                },
                "redirectOutput": {
                    "type": "boolean",
                    "default": false,
                    "description": "When true redirects output to debug console. Default is false."
                },
                "showReturnValue": {
                    "type": "boolean",
                    "description": "Shows return value of functions when stepping. The return value is added to the response to Variables Request"
                },
                "stopOnEntry": {
                    "type": "boolean",
                    "description": "When true debugger stops at first line of user code. When false debugger does not stop until breakpoint, exception or pause."
                },
                "subProcess": {
                    "type": "boolean",
                    "default": true,
                    "description": "When true enables debugging multiprocess applications. Default is true."
                },
                "sudo": {
                    "type": "boolean",
                    "default": false,
                    "description": "When true runs program under elevated permissions (on Unix). Default is false."
                },

                "label": {
                    "type": "string",
                    "description": "The name of the debug configuration"
                },
                "build": {
                    "oneOf": [
                        { "type": "string" },
                        { "type": "object" }
                    ],
                    "description": "A task to run prior to spawning the debuggee"
                },
                "tcp_connection": {
                    "type": "object",
                    "properties": {
                        "port": {
                            "type": "integer",
                            "description": "The port that the debug adapter is listening on"
                        },
                        "host": {
                            "type": "string",
                            "description": "The host that the debug adapter is listening to (e.g. 127.0.0.1)"
                        },
                        "timeout": {
                            "type": "integer",
                            "description": "Timeout in milliseconds to connect to the debug adapter"
                        }
                    },
                    "description": "TCP connection information for connecting to an externally started debug adapter"
                }
            },
            "allOf": [
                {
                    "oneOf": [
                        {
                            "required": ["module"],
                            "title": "Debug Python Module"
                        },
                        {
                            "required": ["program"],
                            "title": "Debug Python Program"
                        },
                        {
                            "required": ["code"],
                            "title": "Debug Python Code"
                        }
                    ]
                }
            ]
        })
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        if self.checked.set(()).is_ok() {
            delegate.output_to_console(format!("Checking latest version of {}...", self.name()));
            if let Some(version) = self.fetch_latest_adapter_version(delegate).await.log_err() {
                self.install_binary(version, delegate).await?;
            }
        }

        self.get_installed_binary(delegate, &config, user_installed_path, cx)
            .await
    }
}
