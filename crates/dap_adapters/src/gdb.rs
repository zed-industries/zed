use std::{collections::HashMap, ffi::OsStr};

use anyhow::{Result, bail};
use async_trait::async_trait;
use dap::{StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use gpui::AsyncApp;
use task::DebugRequest;

use crate::*;

#[derive(Default)]
pub(crate) struct GdbDebugAdapter;

impl GdbDebugAdapter {
    const ADAPTER_NAME: &'static str = "GDB";

    fn request_args(&self, config: &DebugTaskDefinition) -> StartDebuggingRequestArguments {
        let mut args = json!({
            "request": match config.request {
                DebugRequest::Launch(_) => "launch",
                DebugRequest::Attach(_) => "attach",
            },
        });

        let map = args.as_object_mut().unwrap();
        match &config.request {
            DebugRequest::Attach(attach) => {
                map.insert("pid".into(), attach.process_id.into());
            }

            DebugRequest::Launch(launch) => {
                map.insert("program".into(), launch.program.clone().into());

                if !launch.args.is_empty() {
                    map.insert("args".into(), launch.args.clone().into());
                }

                if let Some(stop_on_entry) = config.stop_on_entry {
                    map.insert(
                        "stopAtBeginningOfMainSubprogram".into(),
                        stop_on_entry.into(),
                    );
                }
                if let Some(cwd) = launch.cwd.as_ref() {
                    map.insert("cwd".into(), cwd.to_string_lossy().into_owned().into());
                }
            }
        }
        StartDebuggingRequestArguments {
            configuration: args,
            request: config.request.to_dap(),
        }
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
        config: &DebugTaskDefinition,
        user_installed_path: Option<std::path::PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let user_setting_path = user_installed_path
            .filter(|p| p.exists())
            .and_then(|p| p.to_str().map(|s| s.to_string()));

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
            arguments: vec!["-i=dap".into()],
            envs: HashMap::default(),
            cwd: None,
            connection: None,
            request_args: self.request_args(config),
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
        _: &DebugTaskDefinition,
        _: Option<std::path::PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        unimplemented!("GDB cannot be installed by Zed (yet)")
    }
}
