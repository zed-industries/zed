use adapters::latest_github_release;
use dap::adapters::{DebugTaskDefinition, TcpArguments};
use gpui::{AsyncApp, SharedString};
use language::LanguageName;
use std::{collections::HashMap, path::PathBuf, sync::OnceLock};
use util::ResultExt;

use crate::*;

#[derive(Default)]
pub(crate) struct PhpDebugAdapter {
    checked: OnceLock<()>,
}

impl PhpDebugAdapter {
    const ADAPTER_NAME: &'static str = "PHP";
    const ADAPTER_PACKAGE_NAME: &'static str = "vscode-php-debug";
    const ADAPTER_PATH: &'static str = "extension/out/phpDebug.js";

    fn request_args(
        &self,
        config: &DebugTaskDefinition,
    ) -> Result<dap::StartDebuggingRequestArguments> {
        match &config.request {
            dap::DebugRequest::Attach(_) => {
                anyhow::bail!("php adapter does not support attaching")
            }
            dap::DebugRequest::Launch(launch_config) => Ok(dap::StartDebuggingRequestArguments {
                configuration: json!({
                    "program": launch_config.program,
                    "cwd": launch_config.cwd,
                    "args": launch_config.args,
                    "env": launch_config.env_json(),
                    "stopOnEntry": config.stop_on_entry.unwrap_or_default(),
                }),
                request: config.request.to_dap(),
            }),
        }
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
            .ok_or_else(|| anyhow!("Couldn't find PHP dap directory"))?
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
                format!("--server={}", port),
            ],
            connection: Some(TcpArguments {
                port,
                host,
                timeout,
            }),
            cwd: None,
            envs: HashMap::default(),
            request_args: self.request_args(config)?,
        })
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PhpDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(SharedString::new_static("PHP").into())
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
                    adapters::DownloadedFileType::Vsix,
                    delegate,
                )
                .await?;
            }
        }

        self.get_installed_binary(delegate, &config, user_installed_path, cx)
            .await
    }
}
