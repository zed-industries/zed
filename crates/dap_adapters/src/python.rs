use crate::*;
use dap::{DebugRequest, StartDebuggingRequestArguments};
use gpui::AsyncApp;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};
use task::DebugTaskDefinition;

#[derive(Default)]
pub(crate) struct PythonDebugAdapter;

impl PythonDebugAdapter {
    const ADAPTER_NAME: &'static str = "Debugpy";
    const ADAPTER_PACKAGE_NAME: &'static str = "debugpy";
    const ADAPTER_PATH: &'static str = "src/debugpy/adapter";
    const LANGUAGE_NAME: &'static str = "Python";

    fn request_args(&self, config: &DebugTaskDefinition) -> StartDebuggingRequestArguments {
        let mut args = json!({
            "request": match config.request {
                DebugRequest::Launch(_) => "launch",
                DebugRequest::Attach(_) => "attach",
            },
            "subProcess": true,
            "redirectOutput": true,
        });
        let map = args.as_object_mut().unwrap();
        match &config.request {
            DebugRequest::Attach(attach) => {
                map.insert("processId".into(), attach.process_id.into());
            }
            DebugRequest::Launch(launch) => {
                map.insert("program".into(), launch.program.clone().into());
                map.insert("args".into(), launch.args.clone().into());

                if let Some(stop_on_entry) = config.stop_on_entry {
                    map.insert("stopOnEntry".into(), stop_on_entry.into());
                }
                if let Some(cwd) = launch.cwd.as_ref() {
                    map.insert("cwd".into(), cwd.to_string_lossy().into_owned().into());
                }
            }
        }
        StartDebuggingRequestArguments {
            configuration: args,
            request: config.request.to_dap(),
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PythonDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
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
