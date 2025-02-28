use adapters::latest_github_release;
use dap::{adapters::TcpArguments, transport::TcpTransport};
use gpui::AsyncApp;
use std::{net::Ipv4Addr, path::PathBuf};

use crate::*;

pub(crate) struct PhpDebugAdapter {
    port: u16,
    host: Ipv4Addr,
    timeout: Option<u64>,
}

impl PhpDebugAdapter {
    const ADAPTER_NAME: &'static str = "vscode-php-debug";
    const ADAPTER_PATH: &'static str = "extension/out/phpDebug.js";

    pub(crate) async fn new(host: TCPHost) -> Result<Self> {
        Ok(PhpDebugAdapter {
            port: TcpTransport::port(&host).await?,
            host: host.host(),
            timeout: host.timeout,
        })
    }
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
            &format!("{}/{}", "xdebug", Self::ADAPTER_NAME),
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
            let adapter_path = paths::debug_adapters_dir().join(self.name());

            let file_name_prefix = format!("{}_", self.name());

            util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
                file_name.starts_with(&file_name_prefix)
            })
            .await
            .ok_or_else(|| anyhow!("Couldn't find PHP dap directory"))?
        };

        Ok(DebugAdapterBinary {
            command: delegate
                .node_runtime()
                .binary_path()
                .await?
                .to_string_lossy()
                .into_owned(),
            arguments: Some(vec![
                adapter_path.join(Self::ADAPTER_PATH).into(),
                format!("--server={}", self.port).into(),
            ]),
            connection: Some(TcpArguments {
                port: self.port,
                host: self.host,
                timeout: self.timeout,
            }),
            cwd: config.cwd.clone(),
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

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({
            "program": config.program,
            "cwd": config.cwd,
        })
    }
}
