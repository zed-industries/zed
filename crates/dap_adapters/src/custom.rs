use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct CustomDebugAdapter {
    start_command: String,
    initialize_args: Option<Vec<String>>,
    program: String,
    connection: DebugConnectionType,
}

impl CustomDebugAdapter {
    const _ADAPTER_NAME: &'static str = "custom_dap";

    pub(crate) fn new(adapter_config: &DebugAdapterConfig, custom_args: CustomArgs) -> Self {
        CustomDebugAdapter {
            start_command: custom_args.start_command,
            program: adapter_config.program.clone(),
            connection: custom_args.connection,
            initialize_args: adapter_config.initialize_args.clone(),
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for CustomDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::_ADAPTER_NAME.into())
    }

    async fn connect(
        &self,
        adapter_binary: DebugAdapterBinary,
        cx: &mut AsyncAppContext,
    ) -> Result<TransportParams> {
        match &self.connection {
            DebugConnectionType::STDIO => create_stdio_client(adapter_binary),
            DebugConnectionType::TCP(tcp_host) => {
                create_tcp_client(tcp_host.clone(), adapter_binary, cx).await
            }
        }
    }

    async fn install_or_fetch_binary(
        &self,
        _delegate: Box<dyn DapDelegate>,
    ) -> Result<DebugAdapterBinary> {
        bail!("Install or fetch not implemented for custom debug adapter (yet)");
    }

    fn request_args(&self) -> Value {
        let base_args = json!({
            "program": format!("{}", &self.program)
        });

        // TODO Debugger: Figure out a way to combine this with base args
        // if let Some(args) = &self.initialize_args {
        //     let args = json!(args.clone()).as_object().into_iter();
        //     base_args.as_object_mut().unwrap().extend(args);
        // }

        base_args
    }
}
