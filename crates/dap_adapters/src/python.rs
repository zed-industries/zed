use dap::transport::{TcpTransport, Transport};
use std::{ffi::OsStr, net::Ipv4Addr, path::PathBuf};

use crate::*;

pub(crate) struct PythonDebugAdapter {
    port: u16,
    host: Ipv4Addr,
    timeout: Option<u64>,
}

impl PythonDebugAdapter {
    const ADAPTER_NAME: &'static str = "debugpy";
    const ADAPTER_PATH: &'static str = "src/debugpy/adapter";

    pub(crate) async fn new(host: &TCPHost) -> Result<Self> {
        Ok(PythonDebugAdapter {
            port: TcpTransport::port(host).await?,
            host: host.host(),
            timeout: host.timeout,
        })
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PythonDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn transport(&self) -> Box<dyn Transport> {
        Box::new(TcpTransport::new(self.host, self.port, self.timeout))
    }

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion> {
        let github_repo = GithubRepo {
            repo_name: Self::ADAPTER_NAME.into(),
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
            util::fs::move_folder_files_to_folder(debugpy_dir.as_path(), version_path.as_path())
                .await?;
        }

        Ok(())
    }

    async fn get_installed_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
        user_installed_path: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        let debugpy_dir = if let Some(user_installed_path) = user_installed_path {
            user_installed_path
        } else {
            let adapter_path = paths::debug_adapters_dir().join(self.name());
            let file_name_prefix = format!("{}_", self.name());

            util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
                file_name.starts_with(&file_name_prefix)
            })
            .await
            .ok_or_else(|| anyhow!("Debugpy directory not found"))?
        };

        let python_cmds = [
            OsStr::new("python3"),
            OsStr::new("python"),
            OsStr::new("py"),
        ];
        let python_path = python_cmds
            .iter()
            .filter_map(|cmd| {
                delegate
                    .which(cmd)
                    .and_then(|path| path.to_str().map(|str| str.to_string()))
            })
            .find(|_| true);

        let python_path = python_path.ok_or(anyhow!(
            "Failed to start debugger because python couldn't be found in PATH"
        ))?;

        Ok(DebugAdapterBinary {
            command: python_path,
            arguments: Some(vec![
                debugpy_dir.join(Self::ADAPTER_PATH).into(),
                format!("--port={}", self.port).into(),
                format!("--host={}", self.host).into(),
            ]),
            cwd: config.cwd.clone(),
            envs: None,
        })
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({
            "program": config.program,
            "subProcess": true,
            "cwd": config.cwd,
        })
    }
}
