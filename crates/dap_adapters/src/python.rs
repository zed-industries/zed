use crate::*;
use anyhow::{Context as _, bail};
use collections::HashMap;
use dap::{DebugRequest, StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use fs::{RemoveOptions, RenameOptions};
use futures::StreamExt as _;
use gpui::http_client::AsyncBody;
use gpui::{AsyncApp, BackgroundExecutor, SharedString};
use json_dotpath::DotPaths;
use language::{LanguageName, Toolchain};
use paths::debug_adapters_dir;
use serde_json::Value;
use smol::fs::File;
use smol::io::AsyncReadExt;
use smol::lock::OnceCell;
use std::net::IpAddr;
use std::str::FromStr;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use util::command::new_command;
use util::{ResultExt, paths::PathStyle, rel_path::RelPath};

enum DebugpyLaunchMode<'a> {
    Normal,
    AttachWithConnect { host: Option<&'a str> },
}

#[derive(Default)]
pub(crate) struct PythonDebugAdapter {
    base_venv_path: OnceCell<Arc<Path>>,
    debugpy_whl_base_path: OnceCell<PathBuf>,
}

impl PythonDebugAdapter {
    const ADAPTER_NAME: &'static str = "Debugpy";
    const DEBUG_ADAPTER_NAME: DebugAdapterName =
        DebugAdapterName(SharedString::new_static(Self::ADAPTER_NAME));

    const LANGUAGE_NAME: &'static str = "Python";

    async fn generate_debugpy_arguments<'a>(
        host: &'a IpAddr,
        port: u16,
        launch_mode: DebugpyLaunchMode<'a>,
        user_installed_path: Option<&'a Path>,
        user_args: Option<Vec<String>>,
    ) -> Result<Vec<String>> {
        let mut args = if let Some(user_installed_path) = user_installed_path {
            log::debug!(
                "Using user-installed debugpy adapter from: {}",
                user_installed_path.display()
            );
            vec![user_installed_path.to_string_lossy().into_owned()]
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
            match launch_mode {
                DebugpyLaunchMode::Normal => {
                    vec![format!("--host={}", host), format!("--port={}", port)]
                }
                DebugpyLaunchMode::AttachWithConnect { host } => {
                    let mut args = vec!["connect".to_string()];

                    if let Some(host) = host {
                        args.push(format!("{host}:"));
                    }
                    args.push(format!("{port}"));
                    args
                }
            }
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

    async fn fetch_wheel(
        &self,
        toolchain: Option<Toolchain>,
        delegate: &Arc<dyn DapDelegate>,
        version: &str,
        version_path: &Path,
    ) -> Result<PathBuf> {
        anyhow::ensure!(
            delegate.request_binary_download_approval("debugpy").await,
            "{}",
            util::downloads_disabled_error_with_retry("debugpy", "start the debug session again")
        );

        let adapter_path = debug_adapters_dir().join(Self::ADAPTER_NAME);
        let fs = delegate.fs();
        fs.create_dir(&adapter_path).await?;
        let installed_version = adapters::latest_installed_version_path(
            Self::ADAPTER_NAME,
            Path::new("debugpy/adapter/__main__.py"),
            delegate.as_ref(),
        )
        .await;
        let mut retained_paths = vec![adapter_path.join("zed_base_venv")];
        if let Some(installed_version) = installed_version {
            retained_paths.push(installed_version);
        } else if fs
            .is_file(&adapter_path.join("debugpy/adapter/__main__.py"))
            .await
        {
            let mut entries = fs.read_dir(&adapter_path).await?;
            let version_prefix = format!("{}_", Self::ADAPTER_NAME);
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_name() != Some(OsStr::new(".download"))
                    && !entry
                        .file_name()
                        .and_then(OsStr::to_str)
                        .is_some_and(|name| name.starts_with(&version_prefix))
                {
                    retained_paths.push(entry);
                }
            }
        }
        adapters::remove_other_adapter_versions(
            &adapter_path,
            &retained_paths
                .iter()
                .map(PathBuf::as_path)
                .collect::<Vec<_>>(),
            fs.as_ref(),
        )
        .await?;
        let staging_path = adapter_path.join(".download");
        let download_dir = staging_path.join("wheels");
        fs.create_dir(&download_dir).await?;
        let venv_python = self.base_venv_path(toolchain, delegate).await?;
        anyhow::ensure!(
            delegate.request_binary_download_approval("debugpy").await,
            "{}",
            util::downloads_disabled_error_with_retry("debugpy", "start the debug session again")
        );

        let installation_succeeded = util::command::new_command(venv_python.as_ref())
            .kill_on_drop(true)
            .args([
                "-m",
                "pip",
                "download",
                &format!("debugpy=={version}"),
                "--only-binary=:all:",
                "-d",
                download_dir.to_string_lossy().as_ref(),
            ])
            .output()
            .await
            .context("spawn system python")?
            .status
            .success();
        if !installation_succeeded {
            bail!("debugpy installation failed (could not fetch Debugpy's wheel)");
        }

        let wheel_path = std::fs::read_dir(&download_dir)?
            .find_map(|entry| {
                entry.ok().filter(|e| {
                    e.file_type().is_ok_and(|typ| typ.is_file())
                        && Path::new(&e.file_name()).extension() == Some("whl".as_ref())
                })
            })
            .with_context(|| format!("Did not find a .whl in {download_dir:?}"))?;

        util::archive::extract_zip(&staging_path, File::open(&wheel_path.path()).await?).await?;
        anyhow::ensure!(
            fs.is_file(&staging_path.join("debugpy/adapter/__main__.py"))
                .await,
            "downloaded debugpy wheel is missing its adapter"
        );
        fs.remove_dir(
            &download_dir,
            RemoveOptions {
                recursive: true,
                ignore_if_not_exists: false,
            },
        )
        .await?;
        fs.rename(&staging_path, version_path, RenameOptions::default())
            .await?;
        Ok(version_path.join("debugpy/adapter"))
    }

    async fn maybe_fetch_new_wheel(
        &self,
        toolchain: Option<Toolchain>,
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<PathBuf> {
        anyhow::ensure!(
            delegate.request_binary_download_approval("debugpy").await,
            "{}",
            util::downloads_disabled_error_with_retry("debugpy", "start the debug session again")
        );
        let latest_release = delegate
            .http_client()
            .get(
                "https://pypi.org/pypi/debugpy/json",
                AsyncBody::empty(),
                false,
            )
            .await
            .log_err();
        let response = latest_release
            .filter(|response| response.status().is_success())
            .context("getting latest release")?;

        let mut output = String::new();
        response.into_body().read_to_string(&mut output).await?;
        let as_json = serde_json::Value::from_str(&output)?;
        let latest_version = as_json
            .get("info")
            .and_then(|info| {
                info.get("version")
                    .and_then(|version| version.as_str())
                    .map(ToOwned::to_owned)
            })
            .context("parsing latest release information")?;
        let adapter_path = debug_adapters_dir().join(Self::ADAPTER_NAME);
        let version_path = adapter_path.join(format!("{}_{latest_version}", Self::ADAPTER_NAME));
        let binary_path = version_path.join("debugpy/adapter");
        if delegate
            .fs()
            .is_file(&binary_path.join("__main__.py"))
            .await
        {
            return Ok(binary_path);
        }
        let legacy_path = adapter_path.join("debugpy/adapter");
        if delegate
            .fs()
            .is_dir(&adapter_path.join(format!("debugpy-{latest_version}.dist-info")))
            .await
            && delegate
                .fs()
                .is_file(&legacy_path.join("__main__.py"))
                .await
        {
            return Ok(legacy_path);
        }
        self.fetch_wheel(toolchain, delegate, &latest_version, &version_path)
            .await
    }

    async fn fetch_debugpy_whl(
        &self,
        toolchain: Option<Toolchain>,
        delegate: &Arc<dyn DapDelegate>,
        executor: &BackgroundExecutor,
    ) -> Result<PathBuf> {
        adapters::get_or_download_adapter(
            "debugpy",
            delegate,
            async {
                if let Some(version_path) = adapters::latest_installed_version_path(
                    Self::ADAPTER_NAME,
                    Path::new("debugpy/adapter/__main__.py"),
                    delegate.as_ref(),
                )
                .await
                {
                    return Some(version_path.join("debugpy/adapter"));
                }
                let adapter_path = debug_adapters_dir()
                    .join(Self::ADAPTER_NAME)
                    .join("debugpy/adapter");
                delegate
                    .fs()
                    .is_file(&adapter_path.join("__main__.py"))
                    .await
                    .then_some(adapter_path)
            },
            self.maybe_fetch_new_wheel(toolchain, delegate),
            &self.debugpy_whl_base_path,
            executor,
        )
        .await
    }

    async fn base_venv_path(
        &self,
        toolchain: Option<Toolchain>,
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<Arc<Path>> {
        self.base_venv_path
            .get_or_try_init(|| async {
                let base_python = if let Some(toolchain) = toolchain {
                    toolchain.path.to_string()
                } else {
                    Self::system_python_name(delegate).await?.ok_or_else(|| {
                        let mut message = "Could not find a Python installation".to_owned();
                        if cfg!(windows){
                            message.push_str(". Install Python from the Microsoft Store, or manually from https://www.python.org/downloads/windows.")
                        }
                        anyhow::anyhow!(message)
                    })?
                };

                let debug_adapter_path = paths::debug_adapters_dir().join(Self::DEBUG_ADAPTER_NAME.as_ref());
                Self::require_python_execution(delegate).await?;
                let output = util::command::new_command(&base_python)
                    .kill_on_drop(true)
                    .args(["-m", "venv", "zed_base_venv"])
                    .current_dir(
                        &debug_adapter_path,
                    )
                    .spawn()?
                    .output()
                    .await?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let debug_adapter_path = debug_adapter_path.display();
                    bail!("Failed to create base virtual environment with {base_python} in:\n{debug_adapter_path}\nstderr:\n{stderr}\nstdout:\n{stdout}\n");
                }

                const PYTHON_PATH: &str = if cfg!(target_os = "windows") {
                    "Scripts/python.exe"
                } else {
                    "bin/python3"
                };
                Ok(Arc::from(
                    paths::debug_adapters_dir()
                        .join(Self::DEBUG_ADAPTER_NAME.as_ref())
                        .join("zed_base_venv")
                        .join(PYTHON_PATH)
                        .as_ref(),
                ))
            })
            .await
            .cloned()
    }
    async fn require_python_execution(delegate: &Arc<dyn DapDelegate>) -> Result<()> {
        let tool = adapters::execution_tool(Self::ADAPTER_NAME);
        anyhow::ensure!(
            delegate.request_binary_download_approval(&tool).await,
            "{}",
            util::downloads_disabled_error_with_retry(&tool, "start the debug session again")
        );
        Ok(())
    }

    async fn system_python_name(delegate: &Arc<dyn DapDelegate>) -> Result<Option<String>> {
        const BINARY_NAMES: [&str; 3] = ["python3", "python", "py"];
        let mut name = None;

        for cmd in BINARY_NAMES {
            let Some(path) = delegate.which(OsStr::new(cmd)).await else {
                continue;
            };
            // Try to detect situations where `python3` exists but is not a real Python interpreter.
            // Notably, on fresh Windows installs, `python3` is a shim that opens the Microsoft Store app
            // when run with no arguments, and just fails otherwise.
            Self::require_python_execution(delegate).await?;
            let Some(output) = new_command(&path)
                .kill_on_drop(true)
                .args(["-c", "import sys; print(sys.executable)"])
                .output()
                .await
                .ok()
            else {
                continue;
            };
            if !output.status.success() {
                continue;
            }
            let Ok(python) = std::str::from_utf8(output.stdout.trim_ascii()) else {
                continue;
            };
            let python_path = Path::new(python);
            if !python_path.is_absolute() || !delegate.fs().is_file(python_path).await {
                continue;
            }
            name = Some(python.to_owned());
            break;
        }
        Ok(name)
    }

    async fn get_installed_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        user_args: Option<Vec<String>>,
        user_env: Option<HashMap<String, String>>,
        python_from_toolchain: Option<String>,
    ) -> Result<DebugAdapterBinary> {
        let mut tcp_connection = config.tcp_connection.clone().unwrap_or_default();

        let (config_port, config_host) = config
            .config
            .get("connect")
            .map(|value| {
                (
                    value
                        .get("port")
                        .and_then(|val| val.as_u64().map(|p| p as u16)),
                    value.get("host").and_then(|val| val.as_str()),
                )
            })
            .unwrap_or_else(|| {
                (
                    config
                        .config
                        .get("port")
                        .and_then(|port| port.as_u64().map(|p| p as u16)),
                    config.config.get("host").and_then(|host| host.as_str()),
                )
            });

        let is_attach_with_connect = if config
            .config
            .get("request")
            .is_some_and(|val| val.as_str().is_some_and(|request| request == "attach"))
        {
            if tcp_connection.host.is_some() && config_host.is_some() {
                bail!("Cannot have two different hosts in debug configuration")
            } else if tcp_connection.port.is_some() && config_port.is_some() {
                bail!("Cannot have two different ports in debug configuration")
            }

            if let Some(hostname) = config_host {
                tcp_connection.host = Some(hostname.parse().context("invalid IP address")?);
            }
            tcp_connection.port = config_port;
            DebugpyLaunchMode::AttachWithConnect { host: config_host }
        } else {
            DebugpyLaunchMode::Normal
        };

        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let python_path = if let Some(toolchain) = python_from_toolchain {
            Some(toolchain)
        } else {
            Self::system_python_name(delegate).await?
        };

        let python_command = python_path.context("failed to find binary path for Python")?;
        log::debug!("Using Python executable: {}", python_command);

        let arguments = Self::generate_debugpy_arguments(
            &host,
            port,
            is_attach_with_connect,
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
            envs: user_env.unwrap_or_default(),
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
        user_env: Option<HashMap<String, String>>,
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
                    config,
                    Some(local_path.clone()),
                    user_args,
                    user_env,
                    None,
                )
                .await;
        }

        let base_paths = ["cwd", "program", "module"]
            .into_iter()
            .filter_map(|key| {
                config.config.get(key).and_then(|cwd| {
                    RelPath::new(
                        cwd.as_str()
                            .map(Path::new)?
                            .strip_prefix(delegate.worktree_root_path())
                            .ok()?,
                        PathStyle::local(),
                    )
                    .ok()
                })
            })
            .chain(
                // While Debugpy's wiki saids absolute paths are required, but it actually supports relative paths when cwd is passed in.
                // (Which should always be the case because Zed defaults to the cwd worktree root)
                // So we want to check that these relative paths find toolchains as well. Otherwise, they won't be checked
                // because the strip prefix in the iteration above will return an error
                config
                    .config
                    .get("cwd")
                    .map(|_| {
                        ["program", "module"].into_iter().filter_map(|key| {
                            config.config.get(key).and_then(|value| {
                                let path = Path::new(value.as_str()?);
                                RelPath::new(path, PathStyle::local()).ok()
                            })
                        })
                    })
                    .into_iter()
                    .flatten(),
            )
            .chain([RelPath::empty().into()]);

        let mut toolchain = None;

        for base_path in base_paths {
            if let Some(found_toolchain) = delegate
                .toolchain_store()
                .active_toolchain(
                    delegate.worktree_id(),
                    base_path.into_arc(),
                    language::LanguageName::new_static(Self::LANGUAGE_NAME),
                    cx,
                )
                .await
            {
                toolchain = Some(found_toolchain);
                break;
            }
        }

        let adapter_path = self
            .fetch_debugpy_whl(toolchain.clone(), delegate, cx.background_executor())
            .await?;
        if let Some(toolchain) = &toolchain {
            return self
                .get_installed_binary(
                    delegate,
                    config,
                    Some(adapter_path),
                    user_args,
                    user_env,
                    Some(toolchain.path.to_string()),
                )
                .await;
        }

        self.get_installed_binary(
            delegate,
            config,
            Some(adapter_path),
            user_args,
            user_env,
            None,
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
    use fs::{FakeFs, Fs};
    use futures::{FutureExt as _, channel::oneshot};
    use gpui::TestAppContext;
    use http_client::{FakeHttpClient, HttpClient};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::time::Duration;
    use task::TcpArgumentsTemplate;
    use util::{defer, path};

    #[gpui::test]
    async fn test_pip_admission_after_venv_preparation(cx: &mut TestAppContext) {
        let delegate = DownloadDelegate::new(cx);
        delegate.allowed.store(true, Ordering::SeqCst);
        let adapter = Arc::new(PythonDebugAdapter::default());
        let (prepared_tx, prepared_rx) = oneshot::channel::<Arc<Path>>();
        let prepare = cx.spawn({
            let adapter = adapter.clone();
            async move |_| {
                adapter
                    .base_venv_path
                    .get_or_try_init(|| async { anyhow::Ok(prepared_rx.await?) })
                    .await
                    .cloned()
            }
        });
        cx.run_until_parked();
        let fetch = cx.spawn({
            let adapter = adapter.clone();
            let delegate = delegate.clone() as Arc<dyn DapDelegate>;
            async move |_| {
                adapter
                    .fetch_wheel(
                        None,
                        &delegate,
                        "1.0",
                        &debug_adapters_dir().join("Debugpy/Debugpy_1.0"),
                    )
                    .await
            }
        });
        cx.run_until_parked();
        assert_eq!(delegate.approval_requests.load(Ordering::SeqCst), 1);
        delegate.allowed.store(false, Ordering::SeqCst);
        prepared_tx
            .send(Arc::from(Path::new(path!("/missing/consent-test-python"))))
            .unwrap();
        prepare.await.unwrap();
        assert_eq!(
            fetch.await.unwrap_err().to_string(),
            util::downloads_disabled_error_with_retry("debugpy", "start the debug session again")
        );
        assert_eq!(delegate.approval_requests.load(Ordering::SeqCst), 2);
        assert_eq!(delegate.requests.load(Ordering::SeqCst), 0);
    }

    #[gpui::test]
    async fn test_download_denial_can_be_retried(cx: &mut TestAppContext) {
        let delegate = DownloadDelegate::new(cx);
        let delegate_object = delegate.clone() as Arc<dyn DapDelegate>;
        let adapter = PythonDebugAdapter::default();
        assert_eq!(
            adapter
                .fetch_debugpy_whl(None, &delegate_object, &cx.background_executor)
                .await
                .unwrap_err()
                .to_string(),
            util::downloads_disabled_error_with_retry("debugpy", "start the debug session again")
        );
        assert_eq!(delegate.requests.load(Ordering::SeqCst), 0);
        assert!(
            adapter
                .base_venv_path(None, &delegate_object)
                .await
                .is_err()
        );
        assert!(adapter.base_venv_path.get().is_none());

        delegate.allowed.store(true, Ordering::SeqCst);
        assert_eq!(
            adapter
                .fetch_debugpy_whl(None, &delegate_object, &cx.background_executor)
                .await
                .unwrap_err()
                .to_string(),
            "getting latest release"
        );
        assert_eq!(delegate.requests.load(Ordering::SeqCst), 1);

        let adapter_path = debug_adapters_dir()
            .join(PythonDebugAdapter::ADAPTER_NAME)
            .join("debugpy/adapter");
        delegate.fs.create_dir(&adapter_path).await.unwrap();
        delegate
            .fs
            .insert_file(adapter_path.join("__main__.py"), Vec::new())
            .await;
        delegate.allowed.store(false, Ordering::SeqCst);
        assert_eq!(
            adapter
                .fetch_debugpy_whl(None, &delegate_object, &cx.background_executor)
                .await
                .unwrap(),
            adapter_path
        );
        assert_eq!(delegate.requests.load(Ordering::SeqCst), 1);
    }

    #[gpui::test]
    async fn test_download_refresh_timeout_preserves_cache(cx: &mut TestAppContext) {
        let delegate = DownloadDelegate::new(cx);
        delegate.allowed.store(true, Ordering::SeqCst);
        let delegate_object = delegate.clone() as Arc<dyn DapDelegate>;
        let executor = cx.background_executor.clone();
        let cached_path = PathBuf::from(path!("/adapter/cached"));
        delegate
            .fs
            .create_dir(cached_path.parent().unwrap())
            .await
            .unwrap();
        delegate
            .fs
            .insert_file(&cached_path, b"cached".to_vec())
            .await;
        let resolved_path = OnceCell::new();
        let active_refreshes = AtomicUsize::new(0);
        let started = executor.now();
        let resolve = adapters::get_or_download_adapter(
            "test",
            &delegate_object,
            async { Some(cached_path.clone()) },
            async {
                active_refreshes.fetch_add(1, Ordering::SeqCst);
                let _active_refresh = defer(|| {
                    active_refreshes.fetch_sub(1, Ordering::SeqCst);
                });
                executor.timer(Duration::from_secs(20)).await;
                delegate
                    .fs
                    .remove_file(&cached_path, RemoveOptions::default())
                    .await?;
                anyhow::bail!("refresh continued after its deadline")
            },
            &resolved_path,
            &executor,
        );
        futures::pin_mut!(resolve);
        assert!(resolve.as_mut().now_or_never().is_none());
        assert_eq!(active_refreshes.load(Ordering::SeqCst), 1);
        cx.run_until_parked();
        executor.advance_clock(Duration::from_secs(9));
        cx.run_until_parked();
        assert!(resolve.as_mut().now_or_never().is_none());
        executor.advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        let result = resolve
            .as_mut()
            .now_or_never()
            .expect("cached refresh must stop after 10 seconds")
            .unwrap();
        assert_eq!(result, cached_path);
        assert_eq!(
            executor.now().duration_since(started),
            Duration::from_secs(10)
        );
        assert_eq!(active_refreshes.load(Ordering::SeqCst), 0);
        assert_eq!(resolved_path.get(), Some(&cached_path));

        executor.advance_clock(Duration::from_secs(20));
        cx.run_until_parked();
        assert_eq!(delegate.fs.load(&cached_path).await.unwrap(), "cached");
        assert_eq!(active_refreshes.load(Ordering::SeqCst), 0);
        assert_eq!(
            adapters::get_or_download_adapter(
                "test",
                &delegate_object,
                async { panic!("must reuse the cached path after timeout") },
                async { panic!("must not restart a timed-out refresh") },
                &resolved_path,
                &executor,
            )
            .await
            .unwrap(),
            cached_path
        );
    }

    #[gpui::test]
    async fn test_tcp_connection_conflict_with_connect_args() {
        let adapter = PythonDebugAdapter {
            base_venv_path: OnceCell::new(),
            debugpy_whl_base_path: OnceCell::new(),
        };

        let config_with_port_conflict = json!({
            "request": "attach",
            "connect": {
                "port": 5679
            }
        });

        let tcp_connection = TcpArgumentsTemplate {
            host: None,
            port: Some(5678),
            timeout: None,
        };

        let task_def = DebugTaskDefinition {
            label: "test".into(),
            adapter: PythonDebugAdapter::ADAPTER_NAME.into(),
            config: config_with_port_conflict,
            tcp_connection: Some(tcp_connection.clone()),
        };

        let result = adapter
            .get_installed_binary(
                &test_mocks::MockDelegate::new(),
                &task_def,
                None,
                None,
                None,
                Some("python3".to_string()),
            )
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Cannot have two different ports")
        );

        let host = IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
        let config_with_host_conflict = json!({
            "request": "attach",
            "connect": {
                "host": "192.168.1.1",
                "port": 5678
            }
        });

        let tcp_connection_with_host = TcpArgumentsTemplate {
            host: Some(host),
            port: None,
            timeout: None,
        };

        let task_def_host = DebugTaskDefinition {
            label: "test".into(),
            adapter: PythonDebugAdapter::ADAPTER_NAME.into(),
            config: config_with_host_conflict,
            tcp_connection: Some(tcp_connection_with_host),
        };

        let result_host = adapter
            .get_installed_binary(
                &test_mocks::MockDelegate::new(),
                &task_def_host,
                None,
                None,
                None,
                Some("python3".to_string()),
            )
            .await;

        assert!(result_host.is_err());
        assert!(
            result_host
                .unwrap_err()
                .to_string()
                .contains("Cannot have two different hosts")
        );
    }

    #[gpui::test]
    async fn test_attach_with_connect_mode_generates_correct_arguments() {
        let host = IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
        let port = 5678;

        let args_without_host = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            DebugpyLaunchMode::AttachWithConnect { host: None },
            None,
            None,
        )
        .await
        .unwrap();

        let expected_suffix = path!("debug_adapters/Debugpy/debugpy/adapter");
        assert!(args_without_host[0].ends_with(expected_suffix));
        assert_eq!(args_without_host[1], "connect");
        assert_eq!(args_without_host[2], "5678");

        let args_with_host = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            DebugpyLaunchMode::AttachWithConnect {
                host: Some("192.168.1.100"),
            },
            None,
            None,
        )
        .await
        .unwrap();

        assert!(args_with_host[0].ends_with(expected_suffix));
        assert_eq!(args_with_host[1], "connect");
        assert_eq!(args_with_host[2], "192.168.1.100:");
        assert_eq!(args_with_host[3], "5678");

        let args_normal = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            DebugpyLaunchMode::Normal,
            None,
            None,
        )
        .await
        .unwrap();

        assert!(args_normal[0].ends_with(expected_suffix));
        assert_eq!(args_normal[1], "--host=127.0.0.1");
        assert_eq!(args_normal[2], "--port=5678");
        assert!(!args_normal.contains(&"connect".to_string()));
    }

    #[gpui::test]
    async fn test_debugpy_install_path_cases() {
        let host = IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
        let port = 5678;

        // Case 1: User-defined debugpy path (highest precedence)
        let user_path = PathBuf::from("/custom/path/to/debugpy/src/debugpy/adapter");
        let user_args = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            DebugpyLaunchMode::Normal,
            Some(&user_path),
            None,
        )
        .await
        .unwrap();

        let venv_args = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            DebugpyLaunchMode::Normal,
            None,
            None,
        )
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
            DebugpyLaunchMode::Normal,
            Some(&user_path),
            Some(vec!["foo".into()]),
        )
        .await
        .unwrap();
        let venv_args = PythonDebugAdapter::generate_debugpy_arguments(
            &host,
            port,
            DebugpyLaunchMode::Normal,
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

    struct DownloadDelegate {
        fs: Arc<FakeFs>,
        allowed: AtomicBool,
        approval_requests: AtomicUsize,
        requests: Arc<AtomicUsize>,
        python: Option<PathBuf>,
    }

    impl DownloadDelegate {
        fn new(cx: &TestAppContext) -> Arc<Self> {
            Arc::new(Self {
                fs: FakeFs::new(cx.background_executor.clone()),
                allowed: AtomicBool::new(false),
                approval_requests: AtomicUsize::new(0),
                requests: Arc::new(AtomicUsize::new(0)),
                python: None,
            })
        }
    }

    #[async_trait]
    impl DapDelegate for DownloadDelegate {
        fn worktree_id(&self) -> settings::WorktreeId {
            settings::WorktreeId::from_usize(0)
        }

        fn worktree_root_path(&self) -> &Path {
            Path::new(path!("/"))
        }

        fn http_client(&self) -> Arc<dyn HttpClient> {
            let requests = self.requests.clone();
            FakeHttpClient::create(move |_| {
                requests.fetch_add(1, Ordering::SeqCst);
                async { anyhow::bail!("offline") }
            })
        }

        fn node_runtime(&self) -> node_runtime::NodeRuntime {
            panic!("unexpected node runtime request")
        }

        fn toolchain_store(&self) -> Arc<dyn language::LanguageToolchainStore> {
            panic!("unexpected toolchain request")
        }

        fn fs(&self) -> Arc<dyn Fs> {
            self.fs.clone()
        }

        fn output_to_console(&self, _message: String) {}

        async fn which(&self, _command: &OsStr) -> Option<PathBuf> {
            self.python.clone()
        }

        async fn read_text_file(&self, _path: &RelPath) -> Result<String> {
            anyhow::bail!("unexpected text file request")
        }

        async fn shell_env(&self) -> HashMap<String, String> {
            HashMap::default()
        }

        fn is_headless(&self) -> bool {
            false
        }

        async fn request_binary_download_approval(&self, _tool: &str) -> bool {
            self.approval_requests.fetch_add(1, Ordering::SeqCst);
            self.allowed.load(Ordering::SeqCst)
        }

        async fn binary_downloads_allowed(&self, _tool: &str) -> bool {
            self.allowed.load(Ordering::SeqCst)
        }
    }
}
