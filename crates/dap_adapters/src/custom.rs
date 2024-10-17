use std::ffi::OsString;

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

    async fn connect(
        &self,
        adapter_binary: &DebugAdapterBinary,
        cx: &mut AsyncAppContext,
    ) -> Result<TransportParams> {
        match &self.custom_args.connection {
            DebugConnectionType::STDIO => create_stdio_client(adapter_binary),
            DebugConnectionType::TCP(tcp_host) => {
                create_tcp_client(tcp_host.clone(), adapter_binary, cx).await
            }
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
