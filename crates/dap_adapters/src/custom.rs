use serde_json::Value;
use task::DebugAdapterConfig;

use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct CustomDebugAdapter {
    custom_args: CustomArgs,
}

impl CustomDebugAdapter {
    const _ADAPTER_NAME: &'static str = "custom_dap";

    pub(crate) fn new(custom_args: CustomArgs) -> Self {
        CustomDebugAdapter { custom_args }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for CustomDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::_ADAPTER_NAME.into())
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

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program})
    }

    async fn install_binary(&self, _: &dyn DapDelegate) -> Result<()> {
        bail!("Install or fetch not implemented for custom debug adapter (yet)")
    }

    async fn fetch_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary> {
        bail!("Install or fetch not implemented for custom debug adapter (yet)")
    }
}
