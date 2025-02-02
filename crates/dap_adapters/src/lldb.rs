use std::{collections::HashMap, ffi::OsStr, path::PathBuf, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use dap::transport::{StdioTransport, Transport};
use gpui::AsyncApp;
use sysinfo::{Pid, Process};
use task::{DebugAdapterConfig, DebugRequestType};

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

    fn transport(&self) -> Arc<dyn Transport> {
        Arc::new(StdioTransport::new())
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
        user_installed_path: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let lldb_dap_path = if let Some(user_installed_path) = user_installed_path {
            user_installed_path.to_string_lossy().into()
        } else if cfg!(target_os = "macos") {
            util::command::new_smol_command("xcrun")
                .args(&["-f", "lldb-dap"])
                .output()
                .await
                .ok()
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .map(|path| path.trim().to_string())
                .ok_or(anyhow!("Failed to find lldb-dap in user's path"))?
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
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        unimplemented!("LLDB debug adapter cannot be installed by Zed (yet)")
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        let pid = if let DebugRequestType::Attach(attach_config) = &config.request {
            attach_config.process_id
        } else {
            None
        };

        json!({
            "program": config.program,
            "request": match config.request {
                DebugRequestType::Launch => "launch",
                DebugRequestType::Attach(_) => "attach",
            },
            "pid": pid,
            "cwd": config.cwd,
        })
    }

    fn supports_attach(&self) -> bool {
        true
    }

    fn attach_processes<'a>(
        &self,
        processes: &'a HashMap<Pid, Process>,
    ) -> Option<Vec<(&'a Pid, &'a Process)>> {
        // let regex = Regex::new(r"(?i)^(?:node|bun|iojs)(?:$|\b)").unwrap();

        Some(processes.iter().collect::<Vec<_>>())
    }
}
