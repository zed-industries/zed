use std::ffi::OsStr;

use anyhow::Result;
use async_trait::async_trait;
use gpui::AsyncApp;
use task::DebugAdapterConfig;

use crate::*;

pub(crate) struct GdbDebugAdapter {}

impl GdbDebugAdapter {
    const ADAPTER_NAME: &'static str = "gdb";

    pub(crate) fn new() -> Self {
        GdbDebugAdapter {}
    }
}

#[async_trait(?Send)]
impl DebugAdapter for GdbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
        user_installed_path: Option<std::path::PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let user_setting_path = user_installed_path
            .filter(|p| p.exists())
            .and_then(|p| p.to_str().map(|s| s.to_string()));

        /* GDB implements DAP natively so just need to  */
        let gdb_path = delegate
            .which(OsStr::new("gdb"))
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .ok_or(anyhow!("Could not find gdb in path"));

        if gdb_path.is_err() && user_setting_path.is_none() {
            bail!("Could not find gdb path or it's not installed");
        }

        let gdb_path = user_setting_path.unwrap_or(gdb_path?);

        Ok(DebugAdapterBinary {
            command: gdb_path,
            arguments: Some(vec!["-i=dap".into()]),
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
        unimplemented!("GDB debug adapter cannot be installed by Zed (yet)")
    }

    async fn fetch_latest_adapter_version(&self, _: &dyn DapDelegate) -> Result<AdapterVersion> {
        unimplemented!("Fetch latest GDB version not implemented (yet)")
    }

    async fn get_installed_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugAdapterConfig,
        _: Option<std::path::PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        unimplemented!("GDB cannot be installed by Zed (yet)")
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program, "cwd": config.cwd})
    }
}
