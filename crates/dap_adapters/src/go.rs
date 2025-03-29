use anyhow::bail;
use gpui::AsyncApp;
use std::{ffi::OsStr, path::PathBuf};
use task::DebugTaskDefinition;

use crate::*;

#[derive(Default, Debug)]
pub(crate) struct GoDebugAdapter;

impl GoDebugAdapter {
    const ADAPTER_NAME: &'static str = "Delve";
}

#[async_trait(?Send)]
impl DebugAdapter for GoDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
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
        config: &DebugAdapterConfig,
        _: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let delve_path = delegate
            .which(OsStr::new("dlv"))
            .and_then(|p| p.to_str().map(|p| p.to_string()))
            .ok_or(anyhow!("Dlv not found in path"))?;

        let Some(tcp_connection) = config.tcp_connection.clone() else {
            bail!("Go Debug Adapter expects tcp connection arguments to be provided");
        };
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        Ok(DebugAdapterBinary {
            command: delve_path,
            arguments: Some(vec![
                "dap".into(),
                "--listen".into(),
                format!("{}:{}", host, port).into(),
            ]),
            cwd: None,
            envs: None,
            connection: Some(adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
        })
    }

    fn request_args(&self, config: &DebugTaskDefinition) -> Value {
        match &config.request {
            dap::DebugRequestType::Attach(attach_config) => {
                json!({
                    "processId": attach_config.process_id
                })
            }
            dap::DebugRequestType::Launch(launch_config) => json!({
                "program": launch_config.program,
                "cwd": launch_config.cwd,
            }),
        }
    }
}
