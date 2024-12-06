use std::{ffi::OsStr, path::PathBuf};

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

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
        user_installed_path: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        let lldb_dap_path = if cfg!(target_os = "macos") {
            std::process::Command::new("xcrun")
                .args(&["-f", "lldb-dap"])
                .output()
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|path| path.trim().to_string())
                .ok_or(anyhow!("Failed to find lldb-dap in user's path"))?
        } else if let Some(user_installed_path) = user_installed_path {
            user_installed_path.to_string_lossy().into()
        } else {
            delegate
                .which(OsStr::new("lldb-dap"))
                .and_then(|p| p.to_str().map(|s| s.to_string()))
                .ok_or(anyhow!("Could not find lldb-dap in path"))?
        };

        Ok(DebugAdapterBinary {
            command: lldb_dap_path,
            arguments: None,
            envs: None,
            cwd: config.cwd.clone(),
        })
    }

    async fn install_binary(
        &self,
        _version: AdapterVersion,
        _delegate: &dyn DapDelegate,
    ) -> Result<()> {
        unimplemented!("LLDB debug adapter cannot be installed by Zed (yet)")
    }

    async fn fetch_latest_adapter_version(&self, _: &dyn DapDelegate) -> Result<AdapterVersion> {
        unimplemented!("Fetch latest adapter version not implemented for lldb (yet)")
    }

    async fn get_installed_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugAdapterConfig,
        _: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        unimplemented!("LLDB debug adapter cannot be installed by Zed (yet)")
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({
            "program": config.program,
            "cwd": config.cwd,
        })
    }
}
