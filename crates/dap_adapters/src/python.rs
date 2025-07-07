use crate::*;
use anyhow::Context as _;
use dap::adapters::latest_github_release;
use dap::{DebugRequest, StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use gpui::{AppContext, AsyncApp, SharedString};
use json_dotpath::DotPaths;
use language::{LanguageName, Toolchain};
use serde_json::Value;
use std::borrow::Cow;
use std::net::Ipv4Addr;
use std::sync::LazyLock;
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::OnceLock,
};
#[cfg(feature = "update-schemas")]
use tempfile::TempDir;
use util::ResultExt;

#[derive(Default)]
pub struct PythonDebugAdapter {
    checked: OnceLock<()>,
}

impl PythonDebugAdapter {
    pub const ADAPTER_NAME: &'static str = "Debugpy";
    const DEBUG_ADAPTER_NAME: DebugAdapterName =
        DebugAdapterName(SharedString::new_static(Self::ADAPTER_NAME));
    const ADAPTER_PACKAGE_NAME: &'static str = "debugpy";
    const ADAPTER_PATH: &'static str = "src/debugpy/adapter";
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
            vec![
                user_installed_path
                    .join(Self::ADAPTER_PATH)
                    .to_string_lossy()
                    .to_string(),
            ]
        } else if installed_in_venv {
            log::debug!("Using venv-installed debugpy");
            vec!["-m".to_string(), "debugpy.adapter".to_string()]
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
            vec![
                debugpy_dir
                    .join(Self::ADAPTER_PATH)
                    .to_string_lossy()
                    .to_string(),
            ]
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
            adapter_name.as_ref(),
            version,
            adapters::DownloadedFileType::GzipTar,
            paths::debug_adapters_dir(),
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
        user_args: Option<Vec<String>>,
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

    fn dap_schema(&self) -> Cow<'static, serde_json::Value> {
        static SCHEMA: LazyLock<serde_json::Value> = LazyLock::new(|| {
            const RAW_SCHEMA: &str = include_str!("../schemas/Debugpy.json");
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
                            user_args,
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

        self.get_installed_binary(delegate, &config, None, user_args, toolchain, false)
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

#[cfg(feature = "update-schemas")]
impl PythonDebugAdapter {
    pub fn get_schema(
        temp_dir: &TempDir,
        delegate: UpdateSchemasDapDelegate,
    ) -> anyhow::Result<serde_json::Value> {
        use fs::Fs as _;

        let temp_dir = std::fs::canonicalize(temp_dir.path())?;
        let fs = delegate.fs.clone();
        let executor = delegate.executor.clone();

        let (package_json, package_nls_json) = executor.block(async move {
            let version = fetch_latest_adapter_version_from_github(
                GithubRepo {
                    repo_name: "vscode-python-debugger".into(),
                    repo_owner: "microsoft".into(),
                },
                &delegate,
            )
            .await?;

            let path = adapters::download_adapter_from_github(
                "schemas",
                version,
                adapters::DownloadedFileType::GzipTar,
                &temp_dir,
                &delegate,
            )
            .await?;

            let path = util::fs::find_file_name_in_dir(path.as_path(), |file_name| {
                file_name.starts_with("microsoft-vscode-python-debugger-")
            })
            .await
            .context("find python debugger extension in download")?;

            let package_json = fs.load(&path.join("package.json")).await?;
            let package_nls_json = fs.load(&path.join("package.nls.json")).await.ok();

            anyhow::Ok((package_json, package_nls_json))
        })?;

        let package_json = parse_package_json(package_json, package_nls_json)?;

        let [debugger] =
            <[_; 1]>::try_from(package_json.contributes.debuggers).map_err(|debuggers| {
                anyhow::anyhow!("unexpected number of python debuggers: {}", debuggers.len())
            })?;

        Ok(schema_for_configuration_attributes(
            debugger.configuration_attributes,
        ))
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

        assert!(user_args[0].ends_with("src/debugpy/adapter"));
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
