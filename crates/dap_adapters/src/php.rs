use adapters::latest_github_release;
use anyhow::Context as _;
use anyhow::bail;
use dap::StartDebuggingRequestArguments;
use dap::StartDebuggingRequestArgumentsRequest;
use dap::adapters::{DebugTaskDefinition, TcpArguments};
use gpui::{AsyncApp, SharedString};
use language::LanguageName;
use std::{collections::HashMap, path::PathBuf, sync::OnceLock};
use util::ResultExt;

use crate::*;

#[derive(Default)]
pub(crate) struct PhpDebugAdapter {
    checked: OnceLock<()>,
}

impl PhpDebugAdapter {
    const ADAPTER_NAME: &'static str = "PHP";
    const ADAPTER_PACKAGE_NAME: &'static str = "vscode-php-debug";
    const ADAPTER_PATH: &'static str = "extension/out/phpDebug.js";

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<AdapterVersion> {
        let release = latest_github_release(
            &format!("{}/{}", "xdebug", Self::ADAPTER_PACKAGE_NAME),
            true,
            false,
            delegate.http_client(),
        )
        .await?;

        let asset_name = format!("php-debug-{}.vsix", release.tag_name.replace("v", ""));

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
            .context("Couldn't find PHP dap directory")?
        };

        let tcp_connection = task_definition.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let mut configuration = task_definition.config.clone();
        if let Some(obj) = configuration.as_object_mut() {
            obj.entry("cwd")
                .or_insert_with(|| delegate.worktree_root_path().to_string_lossy().into());
        }

        Ok(DebugAdapterBinary {
            command: Some(
                delegate
                    .node_runtime()
                    .binary_path()
                    .await?
                    .to_string_lossy()
                    .into_owned(),
            ),
            arguments: vec![
                adapter_path
                    .join(Self::ADAPTER_PATH)
                    .to_string_lossy()
                    .to_string(),
                format!("--server={}", port),
            ],
            connection: Some(TcpArguments {
                port,
                host,
                timeout,
            }),
            cwd: Some(delegate.worktree_root_path().to_path_buf()),
            envs: HashMap::default(),
            request_args: StartDebuggingRequestArguments {
                configuration,
                request: <Self as DebugAdapter>::request_kind(self, &task_definition.config)
                    .await?,
            },
        })
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PhpDebugAdapter {
    fn dap_schema(&self) -> serde_json::Value {
        json!({
            "properties": {
                "request": {
                    "type": "string",
                    "enum": ["launch"],
                    "description": "The request type for the PHP debug adapter, always \"launch\"",
                    "default": "launch"
                },
                "hostname": {
                    "type": "string",
                    "description": "The address to bind to when listening for Xdebug (default: all IPv6 connections if available, else all IPv4 connections) or Unix Domain socket (prefix with unix://) or Windows Pipe (\\\\?\\pipe\\name) - cannot be combined with port"
                },
                "port": {
                    "type": "integer",
                    "description": "The port on which to listen for Xdebug (default: 9003). If port is set to 0 a random port is chosen by the system and a placeholder ${port} is replaced with the chosen port in env and runtimeArgs.",
                    "default": 9003
                },
                "program": {
                    "type": "string",
                    "description": "The PHP script to debug (typically a path to a file)",
                    "default": "${file}"
                },
                "cwd": {
                    "type": "string",
                    "description": "Working directory for the debugged program"
                },
                "args": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "description": "Command line arguments to pass to the program"
                },
                "env": {
                    "type": "object",
                    "description": "Environment variables to pass to the program",
                    "additionalProperties": {
                        "type": "string"
                    }
                },
                "stopOnEntry": {
                    "type": "boolean",
                    "description": "Whether to break at the beginning of the script",
                    "default": false
                },
                "pathMappings": {
                    "type": "object",
                    "description": "A mapping of server paths to local paths.",
                },
                "log": {
                    "type": "boolean",
                    "description": "Whether to log all communication between editor and the adapter to the debug console",
                    "default": false
                },
                "ignore": {
                    "type": "array",
                    "description": "An array of glob patterns that errors should be ignored from (for example **/vendor/**/*.php)",
                    "items": {
                        "type": "string"
                    }
                },
                "ignoreExceptions": {
                    "type": "array",
                    "description": "An array of exception class names that should be ignored (for example BaseException, \\NS1\\Exception, \\*\\Exception or \\**\\Exception*)",
                    "items": {
                        "type": "string"
                    }
                },
                "skipFiles": {
                    "type": "array",
                    "description": "An array of glob patterns to skip when debugging. Star patterns and negations are allowed.",
                    "items": {
                        "type": "string"
                    }
                },
                "skipEntryPaths": {
                    "type": "array",
                    "description": "An array of glob patterns to immediately detach from and ignore for debugging if the entry script matches",
                    "items": {
                        "type": "string"
                    }
                },
                "maxConnections": {
                    "type": "integer",
                    "description": "Accept only this number of parallel debugging sessions. Additional connections will be dropped.",
                    "default": 1
                },
                "proxy": {
                    "type": "object",
                    "description": "DBGp Proxy settings",
                    "properties": {
                        "enable": {
                            "type": "boolean",
                            "description": "To enable proxy registration",
                            "default": false
                        },
                        "host": {
                            "type": "string",
                            "description": "The address of the proxy. Supports host name, IP address, or Unix domain socket.",
                            "default": "127.0.0.1"
                        },
                        "port": {
                            "type": "integer",
                            "description": "The port where the adapter will register with the proxy",
                            "default": 9001
                        },
                        "key": {
                            "type": "string",
                            "description": "A unique key that allows the proxy to match requests to your editor",
                            "default": "vsc"
                        },
                        "timeout": {
                            "type": "integer",
                            "description": "The number of milliseconds to wait before giving up on the connection to proxy",
                            "default": 3000
                        },
                        "allowMultipleSessions": {
                            "type": "boolean",
                            "description": "If the proxy should forward multiple sessions/connections at the same time or not",
                            "default": true
                        }
                    }
                },
                "xdebugSettings": {
                    "type": "object",
                    "description": "Allows you to override Xdebug's remote debugging settings to fine tune Xdebug to your needs",
                    "properties": {
                        "max_children": {
                            "type": "integer",
                            "description": "Max number of array or object children to initially retrieve"
                        },
                        "max_data": {
                            "type": "integer",
                            "description": "Max amount of variable data to initially retrieve"
                        },
                        "max_depth": {
                            "type": "integer",
                            "description": "Maximum depth that the debugger engine may return when sending arrays, hashes or object structures to the IDE"
                        },
                        "show_hidden": {
                            "type": "integer",
                            "description": "Whether to show detailed internal information on properties (e.g. private members of classes). Zero means hidden members are not shown.",
                            "enum": [0, 1]
                        },
                        "breakpoint_include_return_value": {
                            "type": "boolean",
                            "description": "Determines whether to enable an additional \"return from function\" debugging step, allowing inspection of the return value when a function call returns"
                        }
                    }
                },
                "xdebugCloudToken": {
                    "type": "string",
                    "description": "Instead of listening locally, open a connection and register with Xdebug Cloud and accept debugging sessions on that connection"
                },
                "stream": {
                    "type": "object",
                    "description": "Allows to influence DBGp streams. Xdebug only supports stdout",
                    "properties": {
                        "stdout": {
                            "type": "integer",
                            "description": "Redirect stdout stream: 0 (disable), 1 (copy), 2 (redirect)",
                            "enum": [0, 1, 2],
                            "default": 0
                        }
                    }
                }
            },
            "required": ["request", "program"]
        })
    }

    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(SharedString::new_static("PHP").into())
    }

    async fn request_kind(
        &self,
        _: &serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest> {
        Ok(StartDebuggingRequestArgumentsRequest::Launch)
    }

    async fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        let obj = match &zed_scenario.request {
            dap::DebugRequest::Attach(_) => {
                bail!("Php adapter doesn't support attaching")
            }
            dap::DebugRequest::Launch(launch_config) => json!({
                "program": launch_config.program,
                "cwd": launch_config.cwd,
                "args": launch_config.args,
                "env": launch_config.env_json(),
                "stopOnEntry": zed_scenario.stop_on_entry.unwrap_or_default(),
            }),
        };

        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            build: None,
            config: obj,
            tcp_connection: None,
        })
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        task_definition: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        if self.checked.set(()).is_ok() {
            delegate.output_to_console(format!("Checking latest version of {}...", self.name()));
            if let Some(version) = self.fetch_latest_adapter_version(delegate).await.log_err() {
                adapters::download_adapter_from_github(
                    self.name(),
                    version,
                    adapters::DownloadedFileType::Vsix,
                    delegate.as_ref(),
                )
                .await?;
            }
        }

        self.get_installed_binary(delegate, &task_definition, user_installed_path, cx)
            .await
    }
}
