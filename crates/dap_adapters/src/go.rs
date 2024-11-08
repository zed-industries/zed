use dap::transport::{TcpTransport, Transport};
use std::{ffi::OsStr, net::Ipv4Addr, path::PathBuf};

use crate::*;

pub(crate) struct GoDebugAdapter {
    port: u16,
    host: Ipv4Addr,
    timeout: Option<u64>,
}

impl GoDebugAdapter {
    const _ADAPTER_NAME: &'static str = "delve";
    // const ADAPTER_PATH: &'static str = "src/debugpy/adapter";

    pub(crate) async fn new(host: &TCPHost) -> Result<Self> {
        Ok(GoDebugAdapter {
            port: TcpTransport::port(host).await?,
            host: host.host(),
            timeout: host.timeout,
        })
    }
}

#[async_trait(?Send)]
impl DebugAdapter for GoDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::_ADAPTER_NAME.into())
    }

    fn transport(&self) -> Box<dyn Transport> {
        Box::new(TcpTransport::new(self.host, self.port, self.timeout))
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
        adapter_path: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        self.get_installed_binary(delegate, config, adapter_path)
            .await
    }

    async fn fetch_latest_adapter_version(
        &self,
        _delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion> {
        // let github_repo = GithubRepo {
        //     repo_name: Self::ADAPTER_NAME.into(),
        //     repo_owner: "go-delve".into(),
        // };

        // adapters::fetch_latest_adapter_version_from_github(github_repo, delegate).await
        unimplemented!("This adapter is used from path for now");
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
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
        _user_installed_path: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        let delve_path = delegate
            .which(OsStr::new("dlv"))
            .and_then(|p| p.to_str().map(|p| p.to_string()))
            .ok_or(anyhow!("Dlv not found in path"))?;

        let ip_address = format!("{}:{}", self.host, self.port);
        let version = "N/A".into();

        Ok(DebugAdapterBinary {
            command: delve_path,
            arguments: Some(vec!["dap".into(), "--listen".into(), ip_address.into()]),
            cwd: config.cwd.clone(),
            envs: None,
            version,
        })
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program, "subProcess": true})
    }
}
