use dap::{StartDebuggingRequestArguments, adapters::DebugScenario};
use gpui::AsyncApp;
use std::{collections::HashMap, ffi::OsStr, path::PathBuf};
use task::DebugTaskDefinition;

use crate::*;

#[derive(Default, Debug)]
pub(crate) struct GoDebugAdapter;

impl GoDebugAdapter {
    const ADAPTER_NAME: &'static str = "Delve";
    fn request_args(&self, config: &DebugScenario) -> StartDebuggingRequestArguments {
        let mut args = match &config.request {
            dap::DebugRequest::Attach(attach_config) => {
                json!({
                    "processId": attach_config.process_id,
                })
            }
            dap::DebugRequest::Launch(launch_config) => json!({
                "program": launch_config.program,
                "cwd": launch_config.cwd,
                "args": launch_config.args
            }),
        };

        let map = args.as_object_mut().unwrap();

        if let Some(stop_on_entry) = config.stop_on_entry {
            map.insert("stopOnEntry".into(), stop_on_entry.into());
        }

        StartDebuggingRequestArguments {
            configuration: args,
            request: config.request.to_dap(),
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for GoDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugScenario,
        user_installed_path: Option<PathBuf>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        self.get_installed_binary(delegate, config, user_installed_path, cx)
            .await
    }

    async fn fetch_latest_adapter_version(
        &self,
        _delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion> {
        unimplemented!("This adapter is used from path for now");
    }

    async fn install_binary(
        &self,
        version: AdapterVersion,
        delegate: &dyn DapDelegate,
    ) -> Result<()> {
        adapters::download_adapter_from_github(
            self.name(),
            version,
            adapters::DownloadedFileType::Zip,
            delegate,
        )
        .await?;
        Ok(())
    }

    async fn get_installed_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugScenario,
        _: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let delve_path = delegate
            .which(OsStr::new("dlv"))
            .and_then(|p| p.to_str().map(|p| p.to_string()))
            .ok_or(anyhow!("Dlv not found in path"))?;

        let tcp_connection = config.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        Ok(DebugAdapterBinary {
            command: delve_path,
            arguments: vec![
                "dap".into(),
                "--listen".into(),
                format!("{}:{}", host, port),
            ],
            cwd: None,
            envs: HashMap::default(),
            connection: Some(adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
            request_args: self.request_args(config),
        })
    }
}
