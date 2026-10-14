use adapters::latest_github_release;
use anyhow::Context as _;
use collections::HashMap;
use dap::{StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use gpui::AsyncApp;
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};
use task::DebugRequest;
use util::{ResultExt, maybe, shell::Shell, shell_builder::ShellBuilder};

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
            &format!("microsoft/{}", Self::ADAPTER_NPM_NAME),
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
        user_args: Option<Vec<String>>,
        user_env: Option<HashMap<String, String>>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let tcp_connection = task_definition.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let mut envs = user_env.unwrap_or_default();

        let mut configuration = task_definition.config.clone();
        if let Some(configuration) = configuration.as_object_mut() {
            // Read before the type is normalized to `pwa-node` below.
            let is_node_terminal =
                configuration.get("type").and_then(Value::as_str) == Some("node-terminal");
            maybe!({
                configuration
                    .get("type")
                    .filter(|value| value == &"node-terminal")?;
                let command = configuration.get("command")?.as_str()?.to_owned();
                // Wrap the whole command string as one real shell invocation
                // (`cmd /C "..."` / `sh -c "..."`) instead of naively
                // whitespace-splitting it into program+args — the latter
                // treats shell operators (`&&`, `||`, pipes, ...) as literal
                // argument tokens to the first program, breaking any chained
                // command (`gulp clean && gulp build && gulp dev` would run
                // `gulp` with `&&`/`gulp`/`build`/... as its literal args).
                // Running it as a real shell lets each step in the chain
                // spawn as its own genuine child process, which js-debug's
                // `console: externalTerminal` auto-attach then picks up in
                // turn as the shell works through the chain.
                let (program, args) =
                    ShellBuilder::new(&Shell::System, cfg!(target_os = "windows"))
                        .build(Some(command), &[]);
                configuration.insert("runtimeExecutable".to_owned(), program.into());
                configuration.insert(
                    "runtimeArgs".to_owned(),
                    args.into_iter().map(Value::from).collect::<Vec<_>>().into(),
                );
                configuration.insert("console".to_owned(), "externalTerminal".into());
                Some(())
            });

            configuration.entry("type").and_modify(normalize_task_type);

            if let Some(program) = configuration
                .get("program")
                .cloned()
                .and_then(|value| value.as_str().map(str::to_owned))
            {
                match program.as_str() {
                    "npm" | "pnpm" | "yarn" | "bun"
                        if !configuration.contains_key("runtimeExecutable")
                            && !configuration.contains_key("runtimeArgs") =>
                    {
                        configuration.remove("program");
                        configuration.insert("runtimeExecutable".to_owned(), program.into());
                        if let Some(args) = configuration.remove("args") {
                            configuration.insert("runtimeArgs".to_owned(), args);
                        }
                    }
                    _ => {}
                }
            }

            if let Some(env) = configuration.get("env").cloned()
                && let Ok(env) = serde_json::from_value::<HashMap<String, String>>(env)
            {
                envs.extend(env);
            }

            configuration
                .entry("cwd")
                .or_insert(delegate.worktree_root_path().to_string_lossy().into());

            if is_node_terminal
                && let Some(cwd) = configuration
                    .get("cwd")
                    .and_then(Value::as_str)
                    .map(PathBuf::from)
            {
                add_project_bins_to_path(configuration, &cwd);
            }

            let is_attach = configuration.get("request").and_then(Value::as_str) == Some("attach");
            if !is_attach {
                configuration
                    .entry("console")
                    .or_insert("externalTerminal".into());
            }

            configuration.entry("sourceMaps").or_insert(true.into());
            configuration
                .entry("pauseForSourceMap")
                .or_insert(true.into());
            configuration
                .entry("sourceMapRenames")
                .or_insert(true.into());

            // Set up remote browser debugging
            if delegate.is_headless() {
                configuration
                    .entry("browserLaunchLocation")
                    .or_insert("ui".into());
            }
        }

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
            .join(Self::ADAPTER_PATH)
        };

        let arguments = if let Some(mut args) = user_args {
            args.insert(0, adapter_path.to_string_lossy().into_owned());
            args
        } else {
            vec![
                adapter_path.to_string_lossy().into_owned(),
                port.to_string(),
                host.to_string(),
            ]
        };

        Ok(DebugAdapterBinary {
            command: Some(
                delegate
                    .node_runtime()
                    .binary_path()
                    .await?
                    .to_string_lossy()
                    .into_owned(),
            ),
            arguments,
            cwd: Some(delegate.worktree_root_path().to_path_buf()),
            envs,
            connection: Some(adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
            request_args: StartDebuggingRequestArguments {
                configuration,
                request: self.request_kind(&task_definition.config).await?,
            },
        })
    }
}

#[async_trait(?Send)]
impl DebugAdapter for JsDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn attach_process_name_filter(&self) -> Option<Vec<&'static str>> {
        Some(vec!["node", "deno", "bun"])
    }

    async fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
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
                if let Some(port) = attach.port {
                    map.insert("port".into(), port.into());
                }
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

    fn dap_schema(&self) -> serde_json::Value {
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
                                    "enum": ["pwa-node", "node", "chrome", "pwa-chrome", "msedge", "pwa-msedge", "node-terminal"],
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
                                "attachSimplePort": {
                                    "type": "number",
                                    "description": "If set, attaches to the process via the given port. This is generally no longer necessary for Node.js programs and loses the ability to debug child processes, but can be useful in more esoteric scenarios such as with Deno and Docker launches. If set to 0, a random port will be chosen and --inspect-brk added to the launch arguments automatically."
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
                                "pauseForSourceMap": {
                                    "type": "boolean",
                                    "description": "Wait for source maps to load before setting breakpoints.",
                                    "default": true
                                },
                                "sourceMapRenames": {
                                    "type": "boolean",
                                    "description": "Whether to use the \"names\" mapping in sourcemaps.",
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
        user_args: Option<Vec<String>>,
        user_env: Option<HashMap<String, String>>,
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

        self.get_installed_binary(
            delegate,
            config,
            user_installed_path,
            user_args,
            user_env,
            cx,
        )
        .await
    }

    fn label_for_child_session(&self, args: &StartDebuggingRequestArguments) -> Option<String> {
        let label = args
            .configuration
            .get("name")?
            .as_str()
            .filter(|name| !name.is_empty())?;
        Some(label.to_owned())
    }

    fn compact_child_session(&self) -> bool {
        true
    }

    fn prefer_thread_name(&self) -> bool {
        true
    }

    // js-debug ends a parent session itself: it is told the debug terminal's
    // shell process id, and ends the terminal's session when that shell exits.
    // Between two steps of a chain run there, the session has no children.
    fn adapter_ends_parent_sessions(&self) -> bool {
        true
    }
}

/// Puts the project's `node_modules/.bin` folders first on the `PATH` of a
/// debug terminal's command, nearest folder first, as `npm run` does for a
/// script. A command written as it would be in `package.json`
/// (`gulp build && fast-serve`) then finds the project's own tools, and so
/// do the tools those start by name (fast-serve runs `gulp bundle`).
///
/// It goes in the configuration's `env`, which js-debug hands on to the
/// terminal it asks the editor for. A `PATH` the user set there is kept and
/// extended; otherwise the terminal's usual one is (the IDE's runtimes
/// first, see `ide_env`, then this process's).
fn add_project_bins_to_path(configuration: &mut serde_json::Map<String, Value>, cwd: &Path) {
    let bins: Vec<PathBuf> = cwd
        .ancestors()
        .map(|dir| dir.join("node_modules").join(".bin"))
        .filter(|dir| dir.is_dir())
        .collect();
    if bins.is_empty() {
        return;
    }

    let env = configuration
        .entry("env")
        .or_insert_with(|| Value::Object(Default::default()));
    let Some(env) = env.as_object_mut() else {
        return;
    };
    // Windows spells it `Path` as often as `PATH`. Two differently spelled
    // variables in one environment would leave it to chance which is read,
    // so the spelling already in use is kept.
    let key = env
        .keys()
        .find(|name| ide_env::is_path_variable(name))
        .cloned()
        .or_else(|| {
            std::env::vars_os().find_map(|(name, _)| {
                name.to_str()
                    .filter(|name| ide_env::is_path_variable(name))
                    .map(str::to_owned)
            })
        })
        .unwrap_or_else(|| "PATH".to_owned());
    let base = env
        .get(&key)
        .and_then(Value::as_str)
        .map(std::ffi::OsString::from)
        .or_else(|| ide_env::path_env(Some(cwd)))
        .or_else(|| std::env::var_os("PATH"));
    if let Some(path) =
        ide_env::prepend(&bins, base.as_deref()).and_then(|path| path.into_string().ok())
    {
        env.insert(key, Value::String(path));
    }
}

fn normalize_task_type(task_type: &mut Value) {
    let Some(task_type_str) = task_type.as_str() else {
        return;
    };

    let new_name = match task_type_str {
        "node" | "pwa-node" | "node-terminal" => "pwa-node",
        "chrome" | "pwa-chrome" => "pwa-chrome",
        "edge" | "msedge" | "pwa-edge" | "pwa-msedge" => "pwa-msedge",
        _ => task_type_str,
    }
    .to_owned();

    *task_type = Value::String(new_name);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path_entries(configuration: &serde_json::Map<String, Value>) -> Vec<PathBuf> {
        let env = configuration["env"].as_object().unwrap();
        let (_, path) = env
            .iter()
            .find(|(name, _)| ide_env::is_path_variable(name))
            .expect("a PATH was set");
        std::env::split_paths(path.as_str().unwrap()).collect()
    }

    #[test]
    fn project_bins_come_first_nearest_first() {
        let root = tempfile::tempdir().unwrap();
        let app = root.path().join("packages").join("app");
        for dir in [root.path(), app.as_path()] {
            std::fs::create_dir_all(dir.join("node_modules").join(".bin")).unwrap();
        }

        let mut configuration = serde_json::Map::new();
        add_project_bins_to_path(&mut configuration, &app);

        let entries = path_entries(&configuration);
        assert_eq!(entries[0], app.join("node_modules").join(".bin"));
        assert_eq!(entries[1], root.path().join("node_modules").join(".bin"));
        // What the terminal would otherwise have follows them.
        assert!(entries.len() > 2);
    }

    #[test]
    fn a_path_the_user_set_is_kept_behind_them() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(root.path().join("node_modules").join(".bin")).unwrap();
        let theirs = std::env::join_paths([root.path().join("tools")]).unwrap();

        let mut configuration = serde_json::Map::new();
        configuration.insert(
            "env".into(),
            serde_json::json!({ "PATH": theirs.to_str().unwrap() }),
        );
        add_project_bins_to_path(&mut configuration, root.path());

        let env = configuration["env"].as_object().unwrap();
        assert_eq!(env.len(), 1, "no second, differently spelled PATH");
        assert_eq!(
            path_entries(&configuration),
            vec![
                root.path().join("node_modules").join(".bin"),
                root.path().join("tools"),
            ]
        );
    }

    #[test]
    fn nothing_changes_without_a_node_modules_folder() {
        let root = tempfile::tempdir().unwrap();
        // Only meaningful where nothing above the temp folder is a Node
        // project of its own.
        if root
            .path()
            .ancestors()
            .any(|dir| dir.join("node_modules").join(".bin").is_dir())
        {
            return;
        }
        let mut configuration = serde_json::Map::new();
        add_project_bins_to_path(&mut configuration, root.path());
        assert!(!configuration.contains_key("env"));
    }
}
