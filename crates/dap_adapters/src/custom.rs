use std::ffi::OsString;

use dap::transport::{StdioTransport, TcpTransport, Transport};
use serde_json::Value;
use task::DebugAdapterConfig;

use crate::*;

pub(crate) struct CustomDebugAdapter {
    custom_args: CustomArgs,
    transport: Box<dyn Transport>,
}

impl CustomDebugAdapter {
    const ADAPTER_NAME: &'static str = "custom_dap";

    pub(crate) async fn new(custom_args: CustomArgs) -> Result<Self> {
        Ok(CustomDebugAdapter {
            transport: match &custom_args.connection {
                DebugConnectionType::TCP(host) => Box::new(TcpTransport::new(
                    host.host(),
                    TcpTransport::port(&host).await?,
                    host.timeout,
                )),
                DebugConnectionType::STDIO => Box::new(StdioTransport::new()),
            },
            custom_args,
        })
    }
}

#[async_trait(?Send)]
impl DebugAdapter for CustomDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn transport(&self) -> Box<dyn Transport> {
        self.transport.clone_box()
    }

    async fn fetch_latest_adapter_version(&self, _: &dyn DapDelegate) -> Result<AdapterVersion> {
        bail!("Custom debug adapters don't have latest versions")
    }

    async fn install_binary(&self, _: AdapterVersion, _: &dyn DapDelegate) -> Result<()> {
        Ok(())
    }

    async fn get_installed_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary> {
        Ok(DebugAdapterBinary {
            command: self.custom_args.command.clone(),
            arguments: self
                .custom_args
                .args
                .clone()
                .map(|args| args.iter().map(OsString::from).collect()),
            envs: self.custom_args.envs.clone(),
            version: "Custom daps".to_string(),
        })
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program})
    }
}
