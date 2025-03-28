use adapters::latest_github_release;
use gpui::AsyncApp;
use regex::Regex;
use std::path::PathBuf;
use task::{DebugRequestType, DebugTaskDefinition};

use crate::*;

#[derive(Debug)]
pub(crate) struct JsDebugAdapter {
    attach_processes: Regex,
}

impl Default for JsDebugAdapter {
    fn default() -> Self {
        Self {
            attach_processes: Regex::new(r"(?i)^(?:node|bun|iojs)(?:$|\b)")
                .expect("Regex compilation to succeed"),
        }
    }
}
impl JsDebugAdapter {
    const ADAPTER_NAME: &'static str = "JavaScript";
    const ADAPTER_NPM_NAME: &'static str = "vscode-js-debug";
    const ADAPTER_PATH: &'static str = "js-debug/src/dapDebugServer.js";
}

#[async_trait(?Send)]
impl DebugAdapter for JsDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
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
        config: &DebugAdapterConfig,
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

        let Some(tcp_connection) = config.tcp_connection.clone() else {
            anyhow::bail!(
                "Javascript Debug Adapter expects tcp connection arguments to be provided"
            );
        };
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        Ok(DebugAdapterBinary {
            command: delegate
                .node_runtime()
                .binary_path()
                .await?
                .to_string_lossy()
                .into_owned(),
            arguments: Some(vec![
                adapter_path.join(Self::ADAPTER_PATH).into(),
                port.to_string().into(),
                host.to_string().into(),
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

    async fn install_binary(
        &self,
        version: AdapterVersion,
        delegate: &dyn DapDelegate,
    ) -> Result<()> {
        adapters::download_adapter_from_github(
            self.name(),
            version,
            adapters::DownloadedFileType::GzipTar,
            delegate,
        )
        .await?;

        return Ok(());
    }

    fn request_args(&self, config: &DebugTaskDefinition) -> Value {
        let mut args = json!({
            "type": "pwa-node",
            "request": match config.request {
                DebugRequestType::Launch(_) => "launch",
                DebugRequestType::Attach(_) => "attach",
            },
        });
        let map = args.as_object_mut().unwrap();
        match &config.request {
            DebugRequestType::Attach(attach) => {
                map.insert("processId".into(), attach.process_id.into());
            }
            DebugRequestType::Launch(launch) => {
                map.insert("program".into(), launch.program.clone().into());
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

    fn attach_processes_filter(&self) -> Regex {
        self.attach_processes.clone()
    }
}
