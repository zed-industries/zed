use adapters::latest_github_release;
use anyhow::Context as _;
use collections::HashMap;
use dap::{StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use gpui::AsyncApp;
use serde_json::Value;
use std::{
    borrow::Cow,
    path::PathBuf,
    sync::{LazyLock, OnceLock},
};
use task::DebugRequest;
use util::{ResultExt, maybe};

use crate::*;

#[derive(Debug, Default)]
pub struct JsDebugAdapter {
    checked: OnceLock<()>,
}

impl JsDebugAdapter {
    pub const ADAPTER_NAME: &'static str = "JavaScript";
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

        let mut envs = HashMap::default();

        let mut configuration = task_definition.config.clone();
        if let Some(configuration) = configuration.as_object_mut() {
            maybe!({
                configuration
                    .get("type")
                    .filter(|value| value == &"node-terminal")?;
                let command = configuration.get("command")?.as_str()?.to_owned();
                let mut args = shlex::split(&command)?.into_iter();
                let program = args.next()?;
                configuration.insert("runtimeExecutable".to_owned(), program.into());
                configuration.insert(
                    "runtimeArgs".to_owned(),
                    args.map(Value::from).collect::<Vec<_>>().into(),
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

            if let Some(env) = configuration.get("env").cloned() {
                if let Ok(env) = serde_json::from_value(env) {
                    envs = env;
                }
            }

            configuration
                .entry("cwd")
                .or_insert(delegate.worktree_root_path().to_string_lossy().into());

            configuration
                .entry("console")
                .or_insert("externalTerminal".into());

            configuration.entry("sourceMaps").or_insert(true.into());
            configuration
                .entry("pauseForSourceMap")
                .or_insert(true.into());
            configuration
                .entry("sourceMapRenames")
                .or_insert(true.into());
        }

        let arguments = if let Some(mut args) = user_args {
            args.insert(
                0,
                adapter_path
                    .join(Self::ADAPTER_PATH)
                    .to_string_lossy()
                    .to_string(),
            );
            args
        } else {
            vec![
                adapter_path
                    .join(Self::ADAPTER_PATH)
                    .to_string_lossy()
                    .to_string(),
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

    fn dap_schema(&self) -> Cow<'static, serde_json::Value> {
        static SCHEMA: LazyLock<serde_json::Value> = LazyLock::new(|| {
            const RAW_SCHEMA: &str = include_str!("../schemas/JavaScript.json");
            serde_json::from_str(RAW_SCHEMA).unwrap()
        });
        Cow::Borrowed(&*SCHEMA)
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        user_args: Option<Vec<String>>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        if self.checked.set(()).is_ok() {
            delegate.output_to_console(format!("Checking latest version of {}...", self.name()));
            if let Some(version) = self.fetch_latest_adapter_version(delegate).await.log_err() {
                adapters::download_adapter_from_github(
                    Self::ADAPTER_NAME,
                    version,
                    adapters::DownloadedFileType::GzipTar,
                    paths::debug_adapters_dir(),
                    delegate.as_ref(),
                )
                .await?;
            } else {
                delegate.output_to_console(format!("{} debug adapter is up to date", self.name()));
            }
        }

        self.get_installed_binary(delegate, &config, user_installed_path, user_args, cx)
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
}

#[cfg(feature = "update-schemas")]
impl JsDebugAdapter {
    pub fn get_schema(
        temp_dir: &tempfile::TempDir,
        delegate: UpdateSchemasDapDelegate,
    ) -> anyhow::Result<serde_json::Value> {
        let (package_json, package_nls_json) = get_vsix_package_json(
            temp_dir,
            &format!("microsoft/{}", Self::ADAPTER_NPM_NAME),
            |release| {
                let version = release
                    .tag_name
                    .strip_prefix("v")
                    .context("parse version")?;
                let asset_name = format!("ms-vscode.js-debug.{version}.vsix");
                Ok(asset_name)
            },
            delegate,
        )?;

        let package_json = parse_package_json(package_json, package_nls_json)?;

        let types = package_json
            .contributes
            .debuggers
            .iter()
            .map(|debugger| debugger.r#type.clone())
            .collect::<Vec<_>>();
        let mut conjuncts = package_json
            .contributes
            .debuggers
            .into_iter()
            .flat_map(|debugger| {
                let r#type = debugger.r#type;
                let configuration_attributes = debugger.configuration_attributes;
                configuration_attributes
                    .launch
                    .map(|schema| ("launch", schema))
                    .into_iter()
                    .chain(
                        configuration_attributes
                            .attach
                            .map(|schema| ("attach", schema)),
                    )
                    .map(|(request, schema)| {
                        json!({
                            "if": {
                                "properties": {
                                    "type": {
                                        "const": r#type
                                    },
                                    "request": {
                                        "const": request
                                    }
                                },
                                "required": ["type", "request"]
                            },
                            "then": schema
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        conjuncts.push(json!({
            "properties": {
                "type": {
                    "enum": types
                }
            },
            "required": ["type"]
        }));
        let schema = json!({
            "allOf": conjuncts
        });
        Ok(schema)
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
