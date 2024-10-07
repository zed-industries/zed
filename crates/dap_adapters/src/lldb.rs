use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct LldbDebugAdapter {
    program: String,
    adapter_path: Option<String>,
}

impl LldbDebugAdapter {
    const _ADAPTER_NAME: &'static str = "lldb";

    pub(crate) fn new(adapter_config: &DebugAdapterConfig) -> Self {
        LldbDebugAdapter {
            program: adapter_config.program.clone(),
            adapter_path: adapter_config.adapter_path.clone(),
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for LldbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::_ADAPTER_NAME.into())
    }

    async fn connect(
        &self,
        adapter_binary: DebugAdapterBinary,
        _: &mut AsyncAppContext,
    ) -> Result<TransportParams> {
        create_stdio_client(adapter_binary)
    }

    async fn install_or_fetch_binary(
        &self,
        _delegate: Box<dyn DapDelegate>,
    ) -> Result<DebugAdapterBinary> {
        bail!("Install or fetch binary not implemented for lldb debug adapter (yet)");
    }

    fn request_args(&self) -> Value {
        json!({"program": format!("{}", &self.program)})
    }
}
