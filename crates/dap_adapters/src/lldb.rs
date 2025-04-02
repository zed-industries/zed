use std::{ffi::OsStr, path::PathBuf};

use anyhow::Result;
use async_trait::async_trait;
use gpui::AsyncApp;
use task::{DebugAdapterConfig, DebugRequestType, DebugTaskDefinition};

use crate::*;

#[derive(Default)]
pub(crate) struct LldbDebugAdapter;

impl LldbDebugAdapter {
    const ADAPTER_NAME: &'static str = "LLDB";
}

#[async_trait(?Send)]
impl DebugAdapter for LldbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        _: &DebugAdapterConfig,
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
            cwd: None,
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

    fn request_args(&self, config: &DebugTaskDefinition) -> Value {
        let mut args = json!({
            "request": match config.request {
                DebugRequestType::Launch(_) => "launch",
                DebugRequestType::Attach(_) => "attach",
            },
        });
        let map = args.as_object_mut().unwrap();
        match &config.request {
            DebugRequestType::Attach(attach) => {
                map.insert("pid".into(), attach.process_id.into());
                map.insert("stopOnEntry".into(), config.stop_on_entry.into());
            }
            DebugRequestType::Launch(launch) => {
                map.insert("program".into(), launch.program.clone().into());
                map.insert("args".into(), launch.args.clone().into());
                map.insert(
                    "cwd".into(),
                    launch
                        .cwd
                        .as_ref()
                        .map(|s| s.to_string_lossy().into_owned())
                        .into(),
                );
            }
        }
        args
    }
}
