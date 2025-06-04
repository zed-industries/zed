use crate::*;
use anyhow::Context as _;
use dap::adapters::latest_github_release;
use dap::{DebugRequest, StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use gpui::{AppContext, AsyncApp, SharedString};
use json_dotpath::DotPaths;
use language::{LanguageName, Toolchain};
use serde_json::Value;
use std::net::Ipv4Addr;
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use util::ResultExt;

#[derive(Default)]
pub(crate) struct PythonDebugAdapter {
    checked: OnceLock<()>,
}

impl PythonDebugAdapter {
    const ADAPTER_NAME: &'static str = "Debugpy";
    const DEBUG_ADAPTER_NAME: DebugAdapterName =
        DebugAdapterName(SharedString::new_static(Self::ADAPTER_NAME));
    const ADAPTER_PACKAGE_NAME: &'static str = "debugpy";
    const ADAPTER_PATH: &'static str = "src/debugpy/adapter";
    const LANGUAGE_NAME: &'static str = "Python";

    async fn generate_debugpy_arguments(
        host: &Ipv4Addr,
        port: u16,
        user_installed_path: Option<&Path>,
        installed_in_venv: bool,
    ) -> Result<Vec<String>> {
        if let Some(user_installed_path) = user_installed_path {
            log::debug!(
                "Using user-installed debugpy adapter from: {}",
                user_installed_path.display()
            );
            Ok(vec![
                user_installed_path
                    .join(Self::ADAPTER_PATH)
                    .to_string_lossy()
                    .to_string(),
                format!("--host={}", host),
                format!("--port={}", port),
            ])
        } else if installed_in_venv {
            log::debug!("Using venv-installed debugpy");
            Ok(vec![
                "-m".to_string(),
                "debugpy.adapter".to_string(),
                format!("--host={}", host),
                format!("--port={}", port),
            ])
        } else {
            let adapter_path = paths::debug_adapters_dir().join(Self::DEBUG_ADAPTER_NAME.as_ref());
            let file_name_prefix = format!("{}_", Self::ADAPTER_NAME);

            let debugpy_dir =
                util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
                    file_name.starts_with(&file_name_prefix)
                })
                .await
                .context("Debugpy directory not found")?;

            log::debug!(
                "Using GitHub-downloaded debugpy adapter from: {}",
                debugpy_dir.display()
            );
            Ok(vec![
                debugpy_dir
                    .join(Self::ADAPTER_PATH)
                    .to_string_lossy()
                    .to_string(),
                format!("--host={}", host),
                format!("--port={}", port),
            ])
        }
    }

    fn request_args(
        &self,
        task_definition: &DebugTaskDefinition,
    ) -> Result<StartDebuggingRequestArguments> {
        let request = self.request_kind(&task_definition.config)?;

        let mut configuration = task_definition.config.clone();
        if let Ok(console) = configuration.dot_get_mut("console") {
            // Use built-in Zed terminal if user did not explicitly provide a setting for console.
            if console.is_null() {
                *console = Value::String("integratedTerminal".into());
            }
        }

        Ok(StartDebuggingRequestArguments {
            configuration,
            request,
        })
    }
    async fn fetch_latest_adapter_version(
        &self,
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<AdapterVersion> {
        let github_repo = GithubRepo {
            repo_name: Self::ADAPTER_PACKAGE_NAME.into(),
            repo_owner: "microsoft".into(),
        };

        fetch_latest_adapter_version_from_github(github_repo, delegate.as_ref()).await
    }

    async fn install_binary(
        adapter_name: DebugAdapterName,
        version: AdapterVersion,
        delegate: Arc<dyn DapDelegate>,
    ) -> Result<()> {
        let version_path = adapters::download_adapter_from_github(
            adapter_name,
            version,
            adapters::DownloadedFileType::GzipTar,
            delegate.as_ref(),
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
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        toolchain: Option<Toolchain>,
        installed_in_venv: bool,
    ) -> Result<DebugAdapterBinary> {
        const BINARY_NAMES: [&str; 3] = ["python3", "python", "py"];
        let tcp_connection = config.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let python_path = if let Some(toolchain) = toolchain {
            Some(toolchain.path.to_string())
        } else {
            let mut name = None;

            for cmd in BINARY_NAMES {
                name = delegate
                    .which(OsStr::new(cmd))
                    .await
                    .map(|path| path.to_string_lossy().to_string());
                if name.is_some() {
                    break;
                }
            }
            name
        };

        let python_command = python_path.context("failed to find binary path for Python")?;
        log::debug!("Using Python executable: {}", python_command);

        let arguments = Self::generate_debugpy_arguments(
            &host,
            port,
            user_installed_path.as_deref(),
            installed_in_venv,
        )
        .await?;

        log::debug!(
            "Starting debugpy adapter with command: {} {}",
            python_command,
            arguments.join(" ")
        );

        Ok(DebugAdapterBinary {
            command: python_command,
            arguments,
            connection: Some(adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
            cwd: Some(delegate.worktree_root_path().to_path_buf()),
            envs: HashMap::default(),
            request_args: self.request_args(config)?,
        })
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PythonDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        Self::DEBUG_ADAPTER_NAME
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(SharedString::new_static("Python").into())
    }

    fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
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

        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            config: args,
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
                "autoReload": {
                    "default": {},
                    "description": "Configures automatic reload of code on edit.",
                    "properties": {
                        "enable": {
                            "default": false,
                            "description": "Automatically reload code on edit.",
                            "type": "boolean"
                        },
                        "exclude": {
                            "default": [
                                "**/.git/**",
                                "**/.metadata/**",
                                "**/__pycache__/**",
                                "**/node_modules/**",
                                "**/site-packages/**"
                            ],
                            "description": "Glob patterns of paths to exclude from auto reload.",
                            "items": {
                                "type": "string"
                            },
                            "type": "array"
                        },
                        "include": {
                            "default": [
                                "**/*.py",
                                "**/*.pyw"
                            ],
                            "description": "Glob patterns of paths to include in auto reload.",
                            "items": {
                                "type": "string"
                            },
                            "type": "array"
                        }
                    },
                    "type": "object"
                },
                "debugAdapterPath": {
                    "description": "Path (fully qualified) to the python debug adapter executable.",
                    "type": "string"
                },
                "django": {
                    "default": false,
                    "description": "Django debugging.",
                    "type": "boolean"
                },
                "jinja": {
                    "default": null,
                    "description": "Jinja template debugging (e.g. Flask).",
                    "enum": [
                        false,
                        null,
                        true
                    ]
                },
                "justMyCode": {
                    "default": true,
                    "description": "If true, show and debug only user-written code. If false, show and debug all code, including library calls.",
                    "type": "boolean"
                },
                "logToFile": {
                    "default": false,
                    "description": "Enable logging of debugger events to a log file. This file can be found in the debugpy extension install folder.",
                    "type": "boolean"
                },
                "pathMappings": {
                    "default": [],
                    "items": {
                        "label": "Path mapping",
                        "properties": {
                            "localRoot": {
                                "default": "${ZED_WORKTREE_ROOT}",
                                "label": "Local source root.",
                                "type": "string"
                            },
                            "remoteRoot": {
                                "default": "",
                                "label": "Remote source root.",
                                "type": "string"
                            }
                        },
                        "required": [
                            "localRoot",
                            "remoteRoot"
                        ],
                        "type": "object"
                    },
                    "label": "Path mappings.",
                    "type": "array"
                },
                "redirectOutput": {
                    "default": true,
                    "description": "Redirect output.",
                    "type": "boolean"
                },
                "showReturnValue": {
                    "default": true,
                    "description": "Show return value of functions when stepping.",
                    "type": "boolean"
                },
                "subProcess": {
                    "default": false,
                    "description": "Whether to enable Sub Process debugging",
                    "type": "boolean"
                },
                "consoleName": {
                    "default": "Python Debug Console",
                    "description": "Display name of the debug console or terminal",
                    "type": "string"
                },
                "clientOS": {
                    "default": null,
                    "description": "OS that VS code is using.",
                    "enum": [
                        "windows",
                        null,
                        "unix"
                    ]
                }
            },
            "required": ["request"],
            "allOf": [
                {
                    "if": {
                        "properties": {
                            "request": {
                                "enum": ["attach"]
                            }
                        }
                    },
                    "then": {
                        "properties": {
                            "connect": {
                                "label": "Attach by connecting to debugpy over a socket.",
                                "properties": {
                                    "host": {
                                        "default": "127.0.0.1",
                                        "description": "Hostname or IP address to connect to.",
                                        "type": "string"
                                    },
                                    "port": {
                                        "description": "Port to connect to.",
                                        "type": [
                                            "number",
                                            "string"
                                        ]
                                    }
                                },
                                "required": [
                                    "port"
                                ],
                                "type": "object"
                            },
                            "listen": {
                                "label": "Attach by listening for incoming socket connection from debugpy",
                                "properties": {
                                    "host": {
                                        "default": "127.0.0.1",
                                        "description": "Hostname or IP address of the interface to listen on.",
                                        "type": "string"
                                    },
                                    "port": {
                                        "description": "Port to listen on.",
                                        "type": [
                                            "number",
                                            "string"
                                        ]
                                    }
                                },
                                "required": [
                                    "port"
                                ],
                                "type": "object"
                            },
                            "processId": {
                                "anyOf": [
                                    {
                                        "default": "${command:pickProcess}",
                                        "description": "Use process picker to select a process to attach, or Process ID as integer.",
                                        "enum": [
                                            "${command:pickProcess}"
                                        ]
                                    },
                                    {
                                        "description": "ID of the local process to attach to.",
                                        "type": "integer"
                                    }
                                ]
                            }
                        }
                    }
                },
                {
                    "if": {
                        "properties": {
                            "request": {
                                "enum": ["launch"]
                            }
                        }
                    },
                    "then": {
                        "properties": {
                            "args": {
                                "default": [],
                                "description": "Command line arguments passed to the program. For string type arguments, it will pass through the shell as is, and therefore all shell variable expansions will apply. But for the array type, the values will be shell-escaped.",
                                "items": {
                                    "type": "string"
                                },
                                "anyOf": [
                                    {
                                        "default": "${command:pickArgs}",
                                        "enum": [
                                            "${command:pickArgs}"
                                        ]
                                    },
                                    {
                                        "type": [
                                            "array",
                                            "string"
                                        ]
                                    }
                                ]
                            },
                            "console": {
                                "default": "integratedTerminal",
                                "description": "Where to launch the debug target: internal console, integrated terminal, or external terminal.",
                                "enum": [
                                    "externalTerminal",
                                    "integratedTerminal",
                                    "internalConsole"
                                ]
                            },
                            "cwd": {
                                "default": "${ZED_WORKTREE_ROOT}",
                                "description": "Absolute path to the working directory of the program being debugged. Default is the root directory of the file (leave empty).",
                                "type": "string"
                            },
                            "autoStartBrowser": {
                                "default": false,
                                "description": "Open external browser to launch the application",
                                "type": "boolean"
                            },
                            "env": {
                                "additionalProperties": {
                                    "type": "string"
                                },
                                "default": {},
                                "description": "Environment variables defined as a key value pair. Property ends up being the Environment Variable and the value of the property ends up being the value of the Env Variable.",
                                "type": "object"
                            },
                            "envFile": {
                                "default": "${ZED_WORKTREE_ROOT}/.env",
                                "description": "Absolute path to a file containing environment variable definitions.",
                                "type": "string"
                            },
                            "gevent": {
                                "default": false,
                                "description": "Enable debugging of gevent monkey-patched code.",
                                "type": "boolean"
                            },
                            "module": {
                                "default": "",
                                "description": "Name of the module to be debugged.",
                                "type": "string"
                            },
                            "program": {
                                "default": "${ZED_FILE}",
                                "description": "Absolute path to the program.",
                                "type": "string"
                            },
                            "purpose": {
                                "default": [],
                                "description": "Tells extension to use this configuration for test debugging, or when using debug-in-terminal command.",
                                "items": {
                                    "enum": [
                                        "debug-test",
                                        "debug-in-terminal"
                                    ],
                                    "enumDescriptions": [
                                        "Use this configuration while debugging tests using test view or test debug commands.",
                                        "Use this configuration while debugging a file using debug in terminal button in the editor."
                                    ]
                                },
                                "type": "array"
                            },
                            "pyramid": {
                                "default": false,
                                "description": "Whether debugging Pyramid applications.",
                                "type": "boolean"
                            },
                            "python": {
                                "default": "${command:python.interpreterPath}",
                                "description": "Absolute path to the Python interpreter executable; overrides workspace configuration if set.",
                                "type": "string"
                            },
                            "pythonArgs": {
                                "default": [],
                                "description": "Command-line arguments passed to the Python interpreter. To pass arguments to the debug target, use \"args\".",
                                "items": {
                                    "type": "string"
                                },
                                "type": "array"
                            },
                            "stopOnEntry": {
                                "default": false,
                                "description": "Automatically stop after launch.",
                                "type": "boolean"
                            },
                            "sudo": {
                                "default": false,
                                "description": "Running debug program under elevated permissions (on Unix).",
                                "type": "boolean"
                            },
                            "guiEventLoop": {
                                "default": "matplotlib",
                                "description": "The GUI event loop that's going to run. Possible values: \"matplotlib\", \"wx\", \"qt\", \"none\", or a custom function that'll be imported and run.",
                                "type": "string"
                            }
                        }
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
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        if let Some(local_path) = &user_installed_path {
            log::debug!(
                "Using user-installed debugpy adapter from: {}",
                local_path.display()
            );
            return self
                .get_installed_binary(delegate, &config, Some(local_path.clone()), None, false)
                .await;
        }

        let toolchain = delegate
            .toolchain_store()
            .active_toolchain(
                delegate.worktree_id(),
                Arc::from("".as_ref()),
                language::LanguageName::new(Self::LANGUAGE_NAME),
                cx,
            )
            .await;

        if let Some(toolchain) = &toolchain {
            if let Some(path) = Path::new(&toolchain.path.to_string()).parent() {
                let debugpy_path = path.join("debugpy");
                if delegate.fs().is_file(&debugpy_path).await {
                    log::debug!(
                        "Found debugpy in toolchain environment: {}",
                        debugpy_path.display()
                    );
                    return self
                        .get_installed_binary(
                            delegate,
                            &config,
                            None,
                            Some(toolchain.clone()),
                            true,
                        )
                        .await;
                }
            }
        }

        if self.checked.set(()).is_ok() {
            delegate.output_to_console(format!("Checking latest version of {}...", self.name()));
            if let Some(version) = self.fetch_latest_adapter_version(delegate).await.log_err() {
                cx.background_spawn(Self::install_binary(self.name(), version, delegate.clone()))
                    .await
                    .context("Failed to install debugpy")?;
            }
        }

        self.get_installed_binary(delegate, &config, None, toolchain, false)
            .await
    }
}

async fn fetch_latest_adapter_version_from_github(
    github_repo: GithubRepo,
    delegate: &dyn DapDelegate,
) -> Result<AdapterVersion> {
    let release = latest_github_release(
        &format!("{}/{}", github_repo.repo_owner, github_repo.repo_name),
        false,
        false,
        delegate.http_client(),
    )
    .await?;

    Ok(AdapterVersion {
        tag_name: release.tag_name,
        url: release.tarball_url,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{net::Ipv4Addr, path::PathBuf};

    #[gpui::test]
    async fn test_debugpy_install_path_cases() {
        let host = Ipv4Addr::new(127, 0, 0, 1);
        let port = 5678;

        // Case 1: User-defined debugpy path (highest precedence)
        let user_path = PathBuf::from("/custom/path/to/debugpy");
        let user_args =
            PythonDebugAdapter::generate_debugpy_arguments(&host, port, Some(&user_path), false)
                .await
                .unwrap();

        // Case 2: Venv-installed debugpy (uses -m debugpy.adapter)
        let venv_args = PythonDebugAdapter::generate_debugpy_arguments(&host, port, None, true)
            .await
            .unwrap();

        assert!(user_args[0].ends_with("src/debugpy/adapter"));
        assert_eq!(user_args[1], "--host=127.0.0.1");
        assert_eq!(user_args[2], "--port=5678");

        assert_eq!(venv_args[0], "-m");
        assert_eq!(venv_args[1], "debugpy.adapter");
        assert_eq!(venv_args[2], "--host=127.0.0.1");
        assert_eq!(venv_args[3], "--port=5678");

        // Note: Case 3 (GitHub-downloaded debugpy) is not tested since this requires mocking the Github API.
    }
}
