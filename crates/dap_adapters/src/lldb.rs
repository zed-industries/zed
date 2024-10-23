use anyhow::Result;
use async_trait::async_trait;
use dap::transport::{StdioTransport, Transport};
use task::DebugAdapterConfig;

use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct LldbDebugAdapter {}

impl LldbDebugAdapter {
    const ADAPTER_NAME: &'static str = "lldb";

    pub(crate) fn new() -> Self {
        LldbDebugAdapter {}
    }
}

#[async_trait(?Send)]
impl DebugAdapter for LldbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn transport(&self) -> Box<dyn Transport> {
        Box::new(StdioTransport::new())
    }

    async fn install_binary(
        &self,
        _version: AdapterVersion,
        _delegate: &dyn DapDelegate,
    ) -> Result<()> {
        bail!("Install binary is not support for install_binary (yet)")
    }

    async fn fetch_latest_adapter_version(&self, _: &dyn DapDelegate) -> Result<AdapterVersion> {
        bail!("Fetch latest adapter version not implemented for lldb (yet)")
    }

    async fn get_installed_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary> {
        #[cfg(target_os = "macos")]
        {
            let output = std::process::Command::new("xcrun")
                .args(&["-f", "lldb-dap"])
                .output()?;
            let lldb_dap_path = String::from_utf8(output.stdout)?.trim().to_string();

            Ok(DebugAdapterBinary {
                command: lldb_dap_path,
                arguments: None,
                envs: None,
            })
        }
        #[cfg(not(target_os = "macos"))]
        {
            Err(anyhow::anyhow!(
                "LLDB-DAP is only supported on macOS (Right now)"
            ))
        }
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program})
    }
}
