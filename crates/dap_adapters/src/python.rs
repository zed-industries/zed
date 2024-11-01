use dap::transport::{TcpTransport, Transport};
use std::{net::Ipv4Addr, path::PathBuf};
use util::maybe;

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
        adapters::download_adapter_from_github(self.name(), version, delegate).await?;
        Ok(())
    }

    async fn get_installed_binary(
        &self,
        _: &dyn DapDelegate,
        config: &DebugAdapterConfig,
        user_installed_path: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = paths::debug_adapters_dir().join(self.name());
        let file_name_prefix = format!("{}_", self.name());

        let adapter_info: Result<_> = maybe!(async {
            let debugpy_dir =
                util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
                    file_name.starts_with(&file_name_prefix)
                })
                .await
                .ok_or_else(|| anyhow!("Debugpy directory not found"))?;

            let version = debugpy_dir
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .and_then(|file_name| file_name.strip_prefix(&file_name_prefix))
                .ok_or_else(|| anyhow!("Python debug adapter has invalid file name"))?
                .to_string();

            Ok((debugpy_dir, version))
        })
        .await;

        let (debugpy_dir, version) = match user_installed_path {
            Some(path) => (path, "N/A".into()),
            None => adapter_info?,
        };

        Ok(DebugAdapterBinary {
            command: "python3".to_string(),
            arguments: Some(vec![
                debugpy_dir.join(Self::ADAPTER_PATH).into(),
                format!("--port={}", self.port).into(),
                format!("--host={}", self.host).into(),
            ]),
            cwd: config.cwd.clone(),
            envs: None,
            version,
        })
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program, "subProcess": true})
    }
}
