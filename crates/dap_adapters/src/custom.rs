use std::ffi::OsString;

use dap::transport::{StdioTransport, TcpTransport, Transport};
use serde_json::Value;
use task::DebugAdapterConfig;

use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct CustomDebugAdapter {
    custom_args: CustomArgs,
}

impl CustomDebugAdapter {
    const ADAPTER_NAME: &'static str = "custom_dap";

    pub(crate) fn new(custom_args: CustomArgs) -> Self {
        CustomDebugAdapter { custom_args }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for CustomDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn transport(&self) -> Box<dyn Transport> {
        match &self.custom_args.connection {
            DebugConnectionType::STDIO => Box::new(StdioTransport::new()),
            DebugConnectionType::TCP(tcp_host) => Box::new(TcpTransport::new(tcp_host.clone())),
        }
    }

    async fn install_binary(&self, _: &dyn DapDelegate) -> Result<()> {
        Ok(())
    }

    async fn fetch_binary(
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
        })
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program})
    }
}
