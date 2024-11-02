use anyhow::Result;
use async_trait::async_trait;
use dap::transport::{StdioTransport, Transport};
use task::DebugAdapterConfig;

use crate::*;

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

    #[cfg(target_os = "macos")]
    async fn get_binary(
        &self,
        _: &dyn DapDelegate,
        config: &DebugAdapterConfig,
        _: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        let output = std::process::Command::new("xcrun")
            .args(&["-f", "lldb-dap"])
            .output()?;
        let lldb_dap_path = String::from_utf8(output.stdout)?.trim().to_string();

        Ok(DebugAdapterBinary {
            command: lldb_dap_path,
            arguments: None,
            envs: None,
            cwd: config.cwd.clone(),
            version: "1".into(),
        })
    }

    #[cfg(not(target_os = "macos"))]
    async fn get_binary(
        &self,
        _: &dyn DapDelegate,
        _: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        Err(anyhow::anyhow!(
            "LLDB-DAP is only supported on macOS (Right now)"
        ))
    }

    async fn install_binary(
        &self,
        _version: AdapterVersion,
        _delegate: &dyn DapDelegate,
    ) -> Result<()> {
        bail!("LLDB debug adapter cannot be installed")
    }

    async fn fetch_latest_adapter_version(&self, _: &dyn DapDelegate) -> Result<AdapterVersion> {
        bail!("Fetch latest adapter version not implemented for lldb (yet)")
    }

    async fn get_installed_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugAdapterConfig,
        _: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        bail!("LLDB debug adapter cannot be installed")
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program})
    }
}
