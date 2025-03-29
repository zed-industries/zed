use adapters::latest_github_release;
use anyhow::bail;
use dap::adapters::TcpArguments;
use gpui::AsyncApp;
use std::path::PathBuf;
use task::DebugTaskDefinition;

use crate::*;

#[derive(Default)]
pub(crate) struct PhpDebugAdapter;

impl PhpDebugAdapter {
    const ADAPTER_NAME: &'static str = "PHP";
    const ADAPTER_PACKAGE_NAME: &'static str = "vscode-php-debug";
    const ADAPTER_PATH: &'static str = "extension/out/phpDebug.js";
}

#[async_trait(?Send)]
impl DebugAdapter for PhpDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion> {
        let release = latest_github_release(
            &format!("{}/{}", "xdebug", Self::ADAPTER_PACKAGE_NAME),
            true,
            false,
            delegate.http_client(),
        )
        .await?;

        let asset_name = format!("php-debug-{}.vsix", release.tag_name.replace("v", ""));

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
            .ok_or_else(|| anyhow!("Couldn't find PHP dap directory"))?
        };

        let Some(tcp_connection) = config.tcp_connection.clone() else {
            bail!("PHP Debug Adapter expects tcp connection arguments to be provided");
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
                format!("--server={}", port).into(),
            ]),
            connection: Some(TcpArguments {
                port,
                host,
                timeout,
            }),
            cwd: None,
            envs: None,
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
            adapters::DownloadedFileType::Vsix,
            delegate,
        )
        .await?;

        Ok(())
    }

    fn request_args(&self, config: &DebugTaskDefinition) -> Value {
        match &config.request {
            dap::DebugRequestType::Attach(_) => {
                // php adapter does not support attaching
                json!({})
            }
            dap::DebugRequestType::Launch(launch_config) => {
                json!({
                    "program": launch_config.program,
                    "cwd": launch_config.cwd,
                })
            }
        }
    }
}
