use crate::*;
use anyhow::Context as _;
use dap::{DebugRequest, StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use fs::RemoveOptions;
use futures::{StreamExt, TryStreamExt};
use gpui::http_client::AsyncBody;
use gpui::{AsyncApp, SharedString};
use json_dotpath::DotPaths;
use language::LanguageName;
use paths::debug_adapters_dir;
use serde_json::Value;
use smol::fs::File;
use smol::io::AsyncReadExt;
use smol::lock::OnceCell;
use std::ffi::OsString;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};
use util::{ResultExt, maybe, paths::PathStyle, rel_path::RelPath};

#[derive(Default)]
pub(crate) struct PythonDebugAdapter {
    base_venv_path: OnceCell<Result<Arc<Path>, String>>,
    debugpy_whl_base_path: OnceCell<Result<Arc<Path>, String>>,
}

impl PythonDebugAdapter {
    const ADAPTER_NAME: &'static str = "Debugpy";
    const DEBUG_ADAPTER_NAME: DebugAdapterName =
        DebugAdapterName(SharedString::new_static(Self::ADAPTER_NAME));

    const LANGUAGE_NAME: &'static str = "Python";

    async fn generate_debugpy_arguments(
        host: &Ipv4Addr,
        port: u16,
        user_installed_path: Option<&Path>,
        user_args: Option<Vec<String>>,
    ) -> Result<Vec<String>> {
        let mut args = if let Some(user_installed_path) = user_installed_path {
            log::debug!(
                "Using user-installed debugpy adapter from: {}",
                user_installed_path.display()
            );
            vec![user_installed_path.to_string_lossy().to_string()]
        } else {
            let adapter_path = paths::debug_adapters_dir().join(Self::DEBUG_ADAPTER_NAME.as_ref());
            let path = adapter_path
                .join("debugpy")
                .join("adapter")
                .to_string_lossy()
                .into_owned();
            log::debug!("Using pip debugpy adapter from: {path}");
            vec![path]
        };

        args.extend(if let Some(args) = user_args {
            args
        } else {
            vec![format!("--host={}", host), format!("--port={}", port)]
        });
        Ok(args)
    }

    async fn request_args(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        task_definition: &DebugTaskDefinition,
    ) -> Result<StartDebuggingRequestArguments> {
        let request = self.request_kind(&task_definition.config).await?;

        let mut configuration = task_definition.config.clone();
        if let Ok(console) = configuration.dot_get_mut("console") {
            // Use built-in Zed terminal if user did not explicitly provide a setting for console.
            if console.is_null() {
                *console = Value::String("integratedTerminal".into());
            }
        }

        if let Some(obj) = configuration.as_object_mut() {
            obj.entry("cwd")
                .or_insert(delegate.worktree_root_path().to_string_lossy().into());
        }

        Ok(StartDebuggingRequestArguments {
            configuration,
            request,
        })
    }

    async fn fetch_wheel(&self, delegate: &Arc<dyn DapDelegate>) -> Result<Arc<Path>, String> {
        let download_dir = debug_adapters_dir().join(Self::ADAPTER_NAME).join("wheels");
        std::fs::create_dir_all(&download_dir).map_err(|e| e.to_string())?;
        let system_python = self.base_venv_path(delegate).await?;

        let installation_succeeded = util::command::new_smol_command(system_python.as_ref())
            .args([
                "-m",
                "pip",
                "download",
                "debugpy",
                "--only-binary=:all:",
                "-d",
                download_dir.to_string_lossy().as_ref(),
            ])
            .output()
            .await
            .map_err(|e| format!("{e}"))?
            .status
            .success();
        if !installation_succeeded {
            return Err("debugpy installation failed (could not fetch Debugpy's wheel)".into());
        }

        let wheel_path = std::fs::read_dir(&download_dir)
            .map_err(|e| e.to_string())?
            .find_map(|entry| {
                entry.ok().filter(|e| {
                    e.file_type().is_ok_and(|typ| typ.is_file())
                        && Path::new(&e.file_name()).extension() == Some("whl".as_ref())
                })
            })
            .ok_or_else(|| String::from("Did not find a .whl in {download_dir}"))?;

        util::archive::extract_zip(
            &debug_adapters_dir().join(Self::ADAPTER_NAME),
            File::open(&wheel_path.path())
                .await
                .map_err(|e| e.to_string())?,
        )
        .await
        .map_err(|e| e.to_string())?;

        Ok(Arc::from(wheel_path.path()))
    }

    async fn maybe_fetch_new_wheel(&self, delegate: &Arc<dyn DapDelegate>) {
        let latest_release = delegate
            .http_client()
            .get(
                "https://pypi.org/pypi/debugpy/json",
                AsyncBody::empty(),
                false,
            )
            .await
            .log_err();
        maybe!(async move {
            let response = latest_release.filter(|response| response.status().is_success())?;

            let download_dir = debug_adapters_dir().join(Self::ADAPTER_NAME);
            std::fs::create_dir_all(&download_dir).ok()?;

            let mut output = String::new();
            response
                .into_body()
                .read_to_string(&mut output)
                .await
                .ok()?;
            let as_json = serde_json::Value::from_str(&output).ok()?;
            let latest_version = as_json.get("info").and_then(|info| {
                info.get("version")
                    .and_then(|version| version.as_str())
                    .map(ToOwned::to_owned)
            })?;
            let dist_info_dirname: OsString = format!("debugpy-{latest_version}.dist-info").into();
            let is_up_to_date = delegate
                .fs()
                .read_dir(&debug_adapters_dir().join(Self::ADAPTER_NAME))
                .await
                .ok()?
                .into_stream()
                .any(async |entry| {
                    entry.is_ok_and(|e| e.file_name().is_some_and(|name| name == dist_info_dirname))
                })
                .await;

            if !is_up_to_date {
                delegate
                    .fs()
                    .remove_dir(
                        &debug_adapters_dir().join(Self::ADAPTER_NAME),
                        RemoveOptions {
                            recursive: true,
                            ignore_if_not_exists: true,
                        },
                    )
                    .await
                    .ok()?;
                self.fetch_wheel(delegate).await.ok()?;
            }
            Some(())
        })
        .await;
    }

    async fn fetch_debugpy_whl(
        &self,
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<Arc<Path>, String> {
        self.debugpy_whl_base_path
            .get_or_init(|| async move {
                self.maybe_fetch_new_wheel(delegate).await;
                Ok(Arc::from(
                    debug_adapters_dir()
                        .join(Self::ADAPTER_NAME)
                        .join("debugpy")
                        .join("adapter")
                        .as_ref(),
                ))
            })
            .await
            .clone()
    }

    async fn base_venv_path(&self, delegate: &Arc<dyn DapDelegate>) -> Result<Arc<Path>, String> {
        self.base_venv_path
            .get_or_init(|| async {
                let base_python = Self::system_python_name(delegate)
                    .await
                    .ok_or_else(|| String::from("Could not find a Python installation"))?;

                let did_succeed = util::command::new_smol_command(base_python)
                    .args(["-m", "venv", "zed_base_venv"])
                    .current_dir(
                        paths::debug_adapters_dir().join(Self::DEBUG_ADAPTER_NAME.as_ref()),
                    )
                    .spawn()
                    .map_err(|e| format!("{e:#?}"))?
                    .status()
                    .await
                    .map_err(|e| format!("{e:#?}"))?
                    .success();

                if !did_succeed {
                    return Err("Failed to create base virtual environment".into());
                }

                const DIR: &str = if cfg!(target_os = "windows") {
                    "Scripts"
                } else {
                    "bin"
                };
                Ok(Arc::from(
                    paths::debug_adapters_dir()
                        .join(Self::DEBUG_ADAPTER_NAME.as_ref())
                        .join("zed_base_venv")
                        .join(DIR)
                        .join("python3")
                        .as_ref(),
                ))
            })
            .await
            .clone()
    }
    async fn system_python_name(delegate: &Arc<dyn DapDelegate>) -> Option<String> {
        const BINARY_NAMES: [&str; 3] = ["python3", "python", "py"];
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
    }

    async fn get_installed_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        user_args: Option<Vec<String>>,
        python_from_toolchain: Option<String>,
    ) -> Result<DebugAdapterBinary> {
        let tcp_connection = config.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let python_path = if let Some(toolchain) = python_from_toolchain {
            Some(toolchain)
        } else {
            Self::system_python_name(delegate).await
        };

        let python_command = python_path.context("failed to find binary path for Python")?;
        log::debug!("Using Python executable: {}", python_command);

        let arguments = Self::generate_debugpy_arguments(
            &host,
            port,
            user_installed_path.as_deref(),
            user_args,
        )
        .await?;

        log::debug!(
            "Starting debugpy adapter with command: {} {}",
            python_command,
            arguments.join(" ")
        );

        Ok(DebugAdapterBinary {
            command: Some(python_command),
            arguments,
            connection: Some(adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
            cwd: Some(delegate.worktree_root_path().to_path_buf()),
            envs: HashMap::default(),
            request_args: self.request_args(delegate, config).await?,
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

    async fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
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

    fn dap_schema(&self) -> serde_json::Value {
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
        user_args: Option<Vec<String>>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        if let Some(local_path) = &user_installed_path {
            log::debug!(
                "Using user-installed debugpy adapter from: {}",
                local_path.display()
            );
            return self
                .get_installed_binary(delegate, config, Some(local_path.clone()), user_args, None)
                .await;
        }

        let base_path = config
            .config
            .get("cwd")
            .and_then(|cwd| {
                RelPath::from_std_path(
                    cwd.as_str()
                        .map(Path::new)?
                        .strip_prefix(delegate.worktree_root_path())
                        .ok()?,
                    PathStyle::local(),
                )
                .ok()
            })
            .unwrap_or_else(|| RelPath::empty().into());
        let toolchain = delegate
            .toolchain_store()
            .active_toolchain(
                delegate.worktree_id(),
                base_path,
                language::LanguageName::new(Self::LANGUAGE_NAME),
                cx,
            )
            .await;

        let debugpy_path = self
            .fetch_debugpy_whl(delegate)
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        if let Some(toolchain) = &toolchain {
            log::debug!(
                "Found debugpy in toolchain environment: {}",
                debugpy_path.display()
            );
            return self
                .get_installed_binary(
                    delegate,
                    config,
                    None,
                    user_args,
                    Some(toolchain.path.to_string()),
                )
                .await;
        }

        self.get_installed_binary(delegate, config, None, user_args, None)
            .await
    }

    fn label_for_child_session(&self, args: &StartDebuggingRequestArguments) -> Option<String> {
        let label = args
            .configuration
            .get("name")?
            .as_str()
            .filter(|label| !label.is_empty())?;
        Some(label.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use util::path;

    use super::*;
    use std::{net::Ipv4Addr, path::PathBuf};

    #[gpui::test]
    async fn test_debugpy_install_path_cases() {
        let host = Ipv4Addr::new(127, 0, 0, 1);
        let port = 5678;

        // Case 1: User-defined debugpy path (highest precedence)
        let user_path = PathBuf::from("/custom/path/to/debugpy/src/debugpy/adapter");
        let user_args =
            PythonDebugAdapter::generate_debugpy_arguments(&host, port, Some(&user_path), None)
                .await
                .unwrap();

        // Case 2: Venv-installed debugpy (uses -m debugpy.adapter)
        let venv_args = PythonDebugAdapter::generate_debugpy_arguments(&host, port, None, None)
            .await
            .unwrap();

        assert_eq!(user_args[0], "/custom/path/to/debugpy/src/debugpy/adapter");
        assert_eq!(user_args[1], "--host=127.0.0.1");
        assert_eq!(user_args[2], "--port=5678");

        let expected_suffix = path!("debug_adapters/Debugpy/debugpy/adapter");
        assert!(venv_args[0].ends_with(expected_suffix));
        assert_eq!(venv_args[1], "--host=127.0.0.1");
        assert_eq!(venv_args[2], "--port=5678");

        // The same cases, with arguments overridden by the user
        let user_args = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            Some(&user_path),
            Some(vec!["foo".into()]),
        )
        .await
        .unwrap();
        let venv_args = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            None,
            Some(vec!["foo".into()]),
        )
        .await
        .unwrap();

        assert!(user_args[0].ends_with("src/debugpy/adapter"));
        assert_eq!(user_args[1], "foo");

        assert!(venv_args[0].ends_with(expected_suffix));
        assert_eq!(venv_args[1], "foo");

        // Note: Case 3 (GitHub-downloaded debugpy) is not tested since this requires mocking the Github API.
    }
}
