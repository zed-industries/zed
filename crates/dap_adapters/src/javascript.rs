use adapters::latest_github_release;
use dap::{StartDebuggingRequestArguments, adapters::DebugTaskDefinition};
use gpui::AsyncApp;
use std::{collections::HashMap, path::PathBuf, sync::OnceLock};
use task::DebugRequest;
use util::ResultExt;

use crate::*;

#[derive(Debug, Default)]
pub(crate) struct JsDebugAdapter {
    checked: OnceLock<()>,
}

impl JsDebugAdapter {
    const ADAPTER_NAME: &'static str = "JavaScript";
    const ADAPTER_NPM_NAME: &'static str = "vscode-js-debug";
    const ADAPTER_PATH: &'static str = "js-debug/src/dapDebugServer.js";

    fn request_args(&self, config: &DebugTaskDefinition) -> StartDebuggingRequestArguments {
        // let mut args = json!({
        //     "type": "pwa-node",
        //     "request": match config.request {
        //         DebugRequest::Launch(_) => "launch",
        //         DebugRequest::Attach(_) => "attach",
        //     },
        // });
        // let map = args.as_object_mut().unwrap();
        // match &config.request {
        //     DebugRequest::Attach(attach) => {
        //         map.insert("processId".into(), attach.process_id.into());
        //     }
        //     DebugRequest::Launch(launch) => {
        //         map.insert("program".into(), launch.program.clone().into());

        //         if !launch.args.is_empty() {
        //             map.insert("args".into(), launch.args.clone().into());
        //         }
        //         if !launch.env.is_empty() {
        //             map.insert("env".into(), launch.env_json());
        //         }

        //         if let Some(stop_on_entry) = config.stop_on_entry {
        //             map.insert("stopOnEntry".into(), stop_on_entry.into());
        //         }
        //         if let Some(cwd) = launch.cwd.as_ref() {
        //             map.insert("cwd".into(), cwd.to_string_lossy().into_owned().into());
        //         }
        //     }
        // }
        // StartDebuggingRequestArguments {
        //     configuration: args,
        //     request: config.request.to_dap(),
        // }
        todo!()
    }

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion> {
        let release = latest_github_release(
            &format!("{}/{}", "microsoft", Self::ADAPTER_NPM_NAME),
            true,
            false,
            delegate.http_client(),
        )
        .await?;

        let asset_name = format!("js-debug-dap-{}.tar.gz", release.tag_name);

        Ok(AdapterVersion {
            tag_name: release.tag_name,
            url: release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?
                .browser_download_url
                .clone(),
        })
    }

    async fn get_installed_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = if let Some(user_installed_path) = user_installed_path {
            user_installed_path
        } else {
            let adapter_path = paths::debug_adapters_dir().join(self.name().as_ref());

            let file_name_prefix = format!("{}_", self.name());

            util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
                file_name.starts_with(&file_name_prefix)
            })
            .await
            .ok_or_else(|| anyhow!("Couldn't find JavaScript dap directory"))?
        };

        let tcp_connection = config.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        Ok(DebugAdapterBinary {
            command: delegate
                .node_runtime()
                .binary_path()
                .await?
                .to_string_lossy()
                .into_owned(),
            arguments: vec![
                adapter_path
                    .join(Self::ADAPTER_PATH)
                    .to_string_lossy()
                    .to_string(),
                port.to_string(),
                host.to_string(),
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

#[async_trait(?Send)]
impl DebugAdapter for JsDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn config_from_zed_format(&self, zed_scenario: ZedDebugScenario) -> DebugScenario {
        todo!()
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        if self.checked.set(()).is_ok() {
            delegate.output_to_console(format!("Checking latest version of {}...", self.name()));
            if let Some(version) = self.fetch_latest_adapter_version(delegate).await.log_err() {
                adapters::download_adapter_from_github(
                    self.name(),
                    version,
                    adapters::DownloadedFileType::GzipTar,
                    delegate,
                )
                .await?;
            }
        }

        self.get_installed_binary(delegate, &config, user_installed_path, cx)
            .await
    }
}
