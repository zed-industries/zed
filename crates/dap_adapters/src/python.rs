use crate::*;

use dap::{DebugRequest, StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use gpui::{AsyncApp, SharedString};
use language::LanguageName;
use serde_json::Value;

use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
enum PackageManager {
    Uv,
    Poetry,
    Pdm,
    Pip,
}

async fn detect_package_manager(
    delegate: &dyn DapDelegate,
    worktree_root: &Path,
) -> PackageManager {
    if delegate.fs().is_file(&worktree_root.join("uv.lock")).await {
        log::info!("Found 'uv.lock', setting package manager to Uv.");
        return PackageManager::Uv;
    }
    if delegate
        .fs()
        .is_file(&worktree_root.join("poetry.lock"))
        .await
    {
        log::info!("Found 'poetry.lock', setting package manager to Poetry.");
        return PackageManager::Poetry;
    }
    if delegate.fs().is_file(&worktree_root.join("pdm.lock")).await {
        log::info!("Found 'pdm.lock', setting package manager to Pdm.");
        return PackageManager::Pdm;
    }
    // fallback to Pip if no specific lock file is found
    log::info!("No specific lock file found, falling back to Pip.");
    PackageManager::Pip
}

#[derive(Default)]
pub(crate) struct PythonDebugAdapter;

impl PythonDebugAdapter {
    const ADAPTER_NAME: &'static str = "Debugpy";
    const DEBUG_ADAPTER_NAME: DebugAdapterName =
        DebugAdapterName(SharedString::new_static(Self::ADAPTER_NAME));

    const LANGUAGE_NAME: &'static str = "Python";

    async fn request_args(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        task_definition: &DebugTaskDefinition,
    ) -> Result<StartDebuggingRequestArguments> {
        let request = self.request_kind(&task_definition.config).await?;

        let mut configuration = task_definition.config.clone();
        if let Some(obj) = configuration.as_object_mut() {
            // Use built-in Zed terminal if user did not explicitly provide a setting for console.
            if let Some(console) = obj.get_mut("console") {
                if console.is_null() {
                    *console = "integratedTerminal".into();
                }
            } else {
                obj.insert("console".to_string(), "integratedTerminal".into());
            }

            // Set the working directory to the project root if not specified.
            obj.entry("cwd").or_insert_with(|| {
                Value::String(delegate.worktree_root_path().to_string_lossy().into_owned())
            });
        }

        Ok(StartDebuggingRequestArguments {
            configuration,
            request,
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
        // --- Python Path Discovery ---
        let mut python_path: Option<PathBuf> = None;
        let worktree_root = delegate.worktree_root_path();

        // Priority 1: User-selected toolchain in Zed
        log::info!("Searching for active Python toolchain...");
        let toolchain = delegate
            .toolchain_store()
            .active_toolchain(
                delegate.worktree_id(),
                Arc::from(worktree_root),
                language::LanguageName::new(Self::LANGUAGE_NAME),
                cx,
            )
            .await;

        if let Some(toolchain) = toolchain {
            log::info!("Found active toolchain: '{}'", toolchain.path);
            python_path = Some(PathBuf::from(toolchain.path.as_ref()));
        } else {
            log::info!("No active toolchain found.");
        }

        // Priority 2: `.venv` directory in project root
        if python_path.is_none() {
            log::info!("Checking for '.venv' directory in project root...");
            let venv_path = worktree_root.join(".venv");
            let python_in_venv = venv_path.join(if cfg!(target_os = "windows") {
                "Scripts/python.exe"
            } else {
                "bin/python"
            });
            if delegate.fs().is_file(&python_in_venv).await {
                log::info!("Found Python in '.venv': {}", python_in_venv.display());
                python_path = Some(python_in_venv);
            }
        }

        // Priority 3: Check python environment.
        let python_path = python_path.ok_or_else(|| {
            anyhow::anyhow!("Could not find a Python environment. Please select a Python interpreter in the Zed status bar, or ensure your project has a `.venv` directory.")
        })?;

        // --- Debug Adapter Logic ---
        let (host, port, timeout) =
            crate::configure_tcp_connection(config.tcp_connection.clone().unwrap_or_default())
                .await?;

        // If user provided a path to a specific adapter script, run it with the discovered python.
        if let Some(user_adapter_path) = user_installed_path {
            log::info!(
                "Using user-configured debugpy adapter path: {}",
                user_adapter_path.display()
            );

            let mut arguments = vec![user_adapter_path.to_string_lossy().to_string()];
            arguments.extend(if let Some(args) = user_args {
                args
            } else {
                vec![format!("--host={host}"), format!("--port={port}")]
            });

            return Ok(DebugAdapterBinary {
                command: Some(python_path.to_string_lossy().into_owned()),
                arguments,
                envs: Default::default(),
                connection: Some(adapters::TcpArguments {
                    host,
                    port,
                    timeout,
                }),
                cwd: Some(worktree_root.to_path_buf()),
                request_args: self.request_args(delegate, &config).await?,
            });
        }

        // --- Package Management ---
        // Detect the package manager and ensure debugpy is installed.
        let package_manager = detect_package_manager(&**delegate, worktree_root).await;
        log::info!("Detected package manager: {:?}", package_manager);

        match package_manager {
            PackageManager::Uv => {
                if let Some(uv_path) = delegate.which("uv".as_ref()).await {
                    log::info!("Found 'uv', using it to check for and install 'debugpy'.");
                    let mut check_command = util::command::new_smol_command(&uv_path);
                    check_command
                        .args(["pip", "show", "debugpy"])
                        .current_dir(worktree_root);

                    log::info!(
                        "Checking for debugpy installation with: {:?}",
                        check_command
                    );
                    let check_output = check_command.output().await?;
                    if !check_output.status.success() {
                        log::info!("'debugpy' not found, attempting to install it with 'uv'...");
                        let mut install_command = util::command::new_smol_command(uv_path);
                        install_command
                            .args(["pip", "install", "debugpy"])
                            .current_dir(worktree_root);

                        log::info!("Running command: {:?}", install_command);
                        let install_output = install_command.output().await?;
                        if !install_output.status.success() {
                            anyhow::bail!(
                                "Failed to install 'debugpy' with 'uv'.\nStdout: {}\nStderr: {}",
                                String::from_utf8_lossy(&install_output.stdout),
                                String::from_utf8_lossy(&install_output.stderr)
                            );
                        }
                        log::info!("'debugpy' installed successfully with 'uv'.");
                    } else {
                        log::info!("'debugpy' is already installed in the selected environment.");
                    }
                } else {
                    anyhow::bail!(
                        "Project is managed by `uv` (uv.lock found), but `uv` command is not in your PATH."
                    );
                }
            }
            PackageManager::Poetry => {
                if let Some(poetry_path) = delegate.which("poetry".as_ref()).await {
                    log::info!("Found 'poetry', using it to run python commands.");
                    let mut check_command = util::command::new_smol_command(&poetry_path);
                    check_command
                        .args(["run", "python", "-m", "pip", "show", "debugpy"])
                        .current_dir(worktree_root);
                    log::info!("Checking for debugpy with: {:?}", check_command);
                    let check_output = check_command.output().await?;

                    if !check_output.status.success() {
                        log::info!("'debugpy' not found, installing with `poetry run pip`...");
                        let mut install_command = util::command::new_smol_command(&poetry_path);
                        install_command
                            .args(["run", "python", "-m", "pip", "install", "debugpy", "-U"])
                            .current_dir(worktree_root);
                        let install_output = install_command.output().await?;
                        if !install_output.status.success() {
                            anyhow::bail!(
                                "Failed to install 'debugpy' with `poetry`.\nStdout: {}\nStderr: {}",
                                String::from_utf8_lossy(&install_output.stdout),
                                String::from_utf8_lossy(&install_output.stderr)
                            );
                        }
                    } else {
                        log::info!("'debugpy' is already installed in the poetry environment.");
                    }
                } else {
                    anyhow::bail!(
                        "Project is managed by `poetry` (poetry.lock found), but `poetry` command is not in your PATH."
                    );
                }
            }
            PackageManager::Pdm => {
                if let Some(pdm_path) = delegate.which("pdm".as_ref()).await {
                    log::info!("Found 'pdm', using it to run python commands.");
                    let mut check_command = util::command::new_smol_command(&pdm_path);
                    check_command
                        .args(["run", "python", "-m", "pip", "show", "debugpy"])
                        .current_dir(worktree_root);
                    log::info!("Checking for debugpy with: {:?}", check_command);
                    let check_output = check_command.output().await?;

                    if !check_output.status.success() {
                        log::info!("'debugpy' not found, installing with `pdm run pip`...");
                        let mut install_command = util::command::new_smol_command(&pdm_path);
                        install_command
                            .args(["run", "python", "-m", "pip", "install", "debugpy", "-U"])
                            .current_dir(worktree_root);
                        let install_output = install_command.output().await?;
                        if !install_output.status.success() {
                            anyhow::bail!(
                                "Failed to install 'debugpy' with `pdm`.\nStdout: {}\nStderr: {}",
                                String::from_utf8_lossy(&install_output.stdout),
                                String::from_utf8_lossy(&install_output.stderr)
                            );
                        }
                    } else {
                        log::info!("'debugpy' is already installed in the pdm environment.");
                    }
                } else {
                    anyhow::bail!(
                        "Project is managed by `pdm` (pdm.lock found), but `pdm` command is not in your PATH."
                    );
                }
            }
            PackageManager::Pip => {
                log::info!("Falling back to 'python -m pip'.");
                // Check if debugpy is installed by running `python -m pip show debugpy`
                let mut check_command = util::command::new_smol_command(&python_path);
                check_command.args(["-m", "pip", "show", "debugpy"]);
                log::info!(
                    "Checking for debugpy installation with: {:?}",
                    check_command
                );
                let check_output = check_command.output().await?;
                if !check_output.status.success() {
                    log::info!("'debugpy' not found, attempting to install it with pip...");
                    let mut install_command = util::command::new_smol_command(&python_path);
                    install_command.args(["-m", "pip", "install", "debugpy", "-U"]);
                    log::info!("Running command: {:?}", install_command);
                    let install_output = install_command.output().await?;
                    if !install_output.status.success() {
                        anyhow::bail!(
                            "Failed to install 'debugpy'.\nStdout: {}\nStderr: {}",
                            String::from_utf8_lossy(&install_output.stdout),
                            String::from_utf8_lossy(&install_output.stderr)
                        );
                    }
                    log::info!("'debugpy' installed successfully.");
                } else {
                    log::info!("'debugpy' is already installed in the selected environment.");
                }
            }
        }

        let mut arguments = vec!["-m".to_string(), "debugpy.adapter".to_string()];
        arguments.extend(if let Some(args) = user_args {
            args
        } else {
            vec![format!("--host={host}"), format!("--port={port}")]
        });

        Ok(DebugAdapterBinary {
            command: Some(python_path.to_string_lossy().into_owned()),
            arguments,
            envs: Default::default(),
            connection: Some(adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
            cwd: Some(worktree_root.to_path_buf()),
            request_args: self.request_args(delegate, &config).await?,
        })
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
