use dap::transport::TcpTransport;
use gpui::AsyncApp;
use serde_json::Value;
use std::{ffi::OsString, path::PathBuf};
use task::DebugAdapterConfig;

use crate::*;

pub(crate) struct CustomDebugAdapter {
    custom_args: CustomArgs,
}

impl CustomDebugAdapter {
    const ADAPTER_NAME: &'static str = "custom_dap";

    pub(crate) async fn new(custom_args: CustomArgs) -> Result<Self> {
        Ok(CustomDebugAdapter { custom_args })
    }
}

#[async_trait(?Send)]
impl DebugAdapter for CustomDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn get_binary(
        &self,
        _: &dyn DapDelegate,
        config: &DebugAdapterConfig,
        _: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let connection = if let DebugConnectionType::TCP(connection) = &self.custom_args.connection
        {
            Some(adapters::TcpArguments {
                host: connection.host(),
                port: TcpTransport::port(&connection).await?,
                timeout: connection.timeout,
            })
        } else {
            None
        };
        let ret = DebugAdapterBinary {
            command: self.custom_args.command.clone(),
            arguments: self
                .custom_args
                .args
                .clone()
                .map(|args| args.iter().map(OsString::from).collect()),
            cwd: config.cwd.clone(),
            envs: self.custom_args.envs.clone(),
            #[cfg(any(test, feature = "test-support"))]
            is_fake: false,
            connection,
        };
        Ok(ret)
    }

    async fn fetch_latest_adapter_version(&self, _: &dyn DapDelegate) -> Result<AdapterVersion> {
        bail!("Custom debug adapters don't have latest versions")
    }

    async fn install_binary(&self, _: AdapterVersion, _: &dyn DapDelegate) -> Result<()> {
        bail!("Custom debug adapters cannot be installed")
    }

    async fn get_installed_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugAdapterConfig,
        _: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        bail!("Custom debug adapters cannot be installed")
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program})
    }
}
