use crate::*;
use anyhow::Context as _;
use dap::{DebugRequest, StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use gpui::{AsyncApp, SharedString};
use json_dotpath::DotPaths;
use language::LanguageName;
use paths::debug_adapters_dir;
use serde_json::Value;
use smol::lock::OnceCell;
use std::net::Ipv4Addr;
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[derive(Default)]
pub(crate) struct PythonDebugAdapter {
    python_venv_base: OnceCell<Result<Arc<Path>, String>>,
}

impl PythonDebugAdapter {
    const ADAPTER_NAME: &'static str = "Debugpy";
    const DEBUG_ADAPTER_NAME: DebugAdapterName =
        DebugAdapterName(SharedString::new_static(Self::ADAPTER_NAME));
    const PYTHON_ADAPTER_IN_VENV: &'static str = if cfg!(target_os = "windows") {
        "Scripts/python3"
    } else {
        "bin/python3"
    };
    const ADAPTER_PATH: &'static str = if cfg!(target_os = "windows") {
        "debugpy-venv/Scripts/debugpy-adapter"
    } else {
        "debugpy-venv/bin/debugpy-adapter"
    };

    const LANGUAGE_NAME: &'static str = "Python";

    async fn generate_debugpy_arguments(
        host: &Ipv4Addr,
        port: u16,
        user_installed_path: Option<&Path>,
        user_args: Option<Vec<String>>,
        installed_in_venv: bool,
    ) -> Result<Vec<String>> {
        let mut args = if let Some(user_installed_path) = user_installed_path {
            log::debug!(
                "Using user-installed debugpy adapter from: {}",
                user_installed_path.display()
            );
            vec![user_installed_path.to_string_lossy().to_string()]
        } else if installed_in_venv {
            log::debug!("Using venv-installed debugpy");
            vec!["-m".to_string(), "debugpy.adapter".to_string()]
        } else {
            let adapter_path = paths::debug_adapters_dir().join(Self::DEBUG_ADAPTER_NAME.as_ref());
            let path = adapter_path
                .join(Self::ADAPTER_PATH)
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

    async fn ensure_venv(delegate: &dyn DapDelegate) -> Result<Arc<Path>> {
        let python_path = Self::find_base_python(delegate)
            .await
            .context("Could not find Python installation for DebugPy")?;
        let work_dir = debug_adapters_dir().join(Self::ADAPTER_NAME);
        if !work_dir.exists() {
            std::fs::create_dir_all(&work_dir)?;
        }
        let mut path = work_dir.clone();
        path.push("debugpy-venv");
        if !path.exists() {
            util::command::new_smol_command(python_path)
                .arg("-m")
                .arg("venv")
                .arg("debugpy-venv")
                .current_dir(work_dir)
                .spawn()?
                .output()
                .await?;
        }

        Ok(path.into())
    }

    // Find "baseline", user python version from which we'll create our own venv.
    async fn find_base_python(delegate: &dyn DapDelegate) -> Option<PathBuf> {
        for path in ["python3", "python"] {
            if let Some(path) = delegate.which(path.as_ref()).await {
                return Some(path);
            }
        }
        None
    }
    const BINARY_DIR: &str = if cfg!(target_os = "windows") {
        "Scripts"
    } else {
        "bin"
    };
    async fn base_venv(&self, delegate: &dyn DapDelegate) -> Result<Arc<Path>, String> {
        self.python_venv_base
            .get_or_init(move || async move {
                let venv_base = Self::ensure_venv(delegate)
                    .await
                    .map_err(|e| format!("{e}"))?;
                Self::install_debugpy_into_venv(&venv_base).await?;
                Ok(venv_base)
            })
            .await
            .clone()
    }

    async fn install_debugpy_into_venv(venv_path: &Path) -> Result<(), String> {
        let pip_path = venv_path.join(Self::BINARY_DIR).join("pip3");
        let installation_succeeded = util::command::new_smol_command(pip_path.as_path())
            .arg("install")
            .arg("debugpy")
            .arg("-U")
            .output()
            .await
            .map_err(|e| format!("{e}"))?
            .status
            .success();
        if !installation_succeeded {
            return Err("debugpy installation failed".into());
        }

        Ok(())
    }

    async fn get_installed_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        user_args: Option<Vec<String>>,
        python_from_toolchain: Option<String>,
        installed_in_venv: bool,
    ) -> Result<DebugAdapterBinary> {
        const BINARY_NAMES: [&str; 3] = ["python3", "python", "py"];
        let tcp_connection = config.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let python_path = if let Some(toolchain) = python_from_toolchain {
            Some(toolchain)
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
            user_args,
            installed_in_venv,
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
                .get_installed_binary(
                    delegate,
                    &config,
                    Some(local_path.clone()),
                    user_args,
                    None,
                    false,
                )
                .await;
        }

        let base_path = config
            .config
            .get("cwd")
            .and_then(|cwd| {
                cwd.as_str()
                    .map(Path::new)?
                    .strip_prefix(delegate.worktree_root_path())
                    .ok()
            })
            .unwrap_or_else(|| "".as_ref())
            .into();
        let toolchain = delegate
            .toolchain_store()
            .active_toolchain(
                delegate.worktree_id(),
                base_path,
                language::LanguageName::new(Self::LANGUAGE_NAME),
                cx,
            )
            .await;

        if let Some(toolchain) = &toolchain {
            if let Some(path) = Path::new(&toolchain.path.to_string()).parent() {
                if let Some(parent) = path.parent() {
                    Self::install_debugpy_into_venv(parent).await.ok();
                }

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
                            user_args,
                            Some(toolchain.path.to_string()),
                            true,
                        )
                        .await;
                }
            }
        }
        let toolchain = self
            .base_venv(&**delegate)
            .await
            .map_err(|e| anyhow::anyhow!(e))?
            .join(Self::PYTHON_ADAPTER_IN_VENV);

        self.get_installed_binary(
            delegate,
            &config,
            None,
            user_args,
            Some(toolchain.to_string_lossy().into_owned()),
            false,
        )
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
    use super::*;
    use std::{net::Ipv4Addr, path::PathBuf};

    #[gpui::test]
    async fn test_debugpy_install_path_cases() {
        let host = Ipv4Addr::new(127, 0, 0, 1);
        let port = 5678;

        // Case 1: User-defined debugpy path (highest precedence)
        let user_path = PathBuf::from("/custom/path/to/debugpy/src/debugpy/adapter");
        let user_args = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            Some(&user_path),
            None,
            false,
        )
        .await
        .unwrap();

        // Case 2: Venv-installed debugpy (uses -m debugpy.adapter)
        let venv_args =
            PythonDebugAdapter::generate_debugpy_arguments(&host, port, None, None, true)
                .await
                .unwrap();

        assert_eq!(user_args[0], "/custom/path/to/debugpy/src/debugpy/adapter");
        assert_eq!(user_args[1], "--host=127.0.0.1");
        assert_eq!(user_args[2], "--port=5678");

        assert_eq!(venv_args[0], "-m");
        assert_eq!(venv_args[1], "debugpy.adapter");
        assert_eq!(venv_args[2], "--host=127.0.0.1");
        assert_eq!(venv_args[3], "--port=5678");

        // The same cases, with arguments overridden by the user
        let user_args = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            Some(&user_path),
            Some(vec!["foo".into()]),
            false,
        )
        .await
        .unwrap();
        let venv_args = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            None,
            Some(vec!["foo".into()]),
            true,
        )
        .await
        .unwrap();

        assert!(user_args[0].ends_with("src/debugpy/adapter"));
        assert_eq!(user_args[1], "foo");

        assert_eq!(venv_args[0], "-m");
        assert_eq!(venv_args[1], "debugpy.adapter");
        assert_eq!(venv_args[2], "foo");

        // Note: Case 3 (GitHub-downloaded debugpy) is not tested since this requires mocking the Github API.
    }
}
