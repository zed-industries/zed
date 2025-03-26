use std::{collections::HashMap, ffi::OsStr, path::PathBuf};

use anyhow::Result;
use async_trait::async_trait;
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

    pub fn attach_processes(processes: &HashMap<Pid, Process>) -> Vec<(&Pid, &Process)> {
        processes.iter().collect::<Vec<_>>()
    }
}

#[async_trait(?Send)]
impl DebugAdapter for LldbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
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
            connection: None,
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
}
