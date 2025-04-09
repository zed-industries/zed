use std::{path::PathBuf, sync::OnceLock};

use anyhow::{Result, bail};
use async_trait::async_trait;
use dap::adapters::latest_github_release;
use gpui::AsyncApp;
use task::{DebugAdapterConfig, DebugRequestType, DebugTaskDefinition};

use crate::*;

#[derive(Default)]
pub(crate) struct CodeLldbDebugAdapter {
    last_known_version: OnceLock<String>,
}

impl CodeLldbDebugAdapter {
    const ADAPTER_NAME: &'static str = "CodeLLDB";
}

#[async_trait(?Send)]
impl DebugAdapter for CodeLldbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
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

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion> {
        let release =
            latest_github_release("vadimcn/codelldb", true, false, delegate.http_client()).await?;

        let arch = match std::env::consts::ARCH {
            "aarch64" => "arm64",
            "x86_64" => "x64",
            _ => {
                return Err(anyhow!(
                    "unsupported architecture {}",
                    std::env::consts::ARCH
                ));
            }
        };
        let platform = match std::env::consts::OS {
            "macos" => "darwin",
            "linux" => "linux",
            "windows" => "win32",
            _ => {
                return Err(anyhow!(
                    "unsupported operating system {}",
                    std::env::consts::OS
                ));
            }
        };
        let asset_name = format!("codelldb-{platform}-{arch}.vsix");
        let _ = self.last_known_version.set(release.tag_name.clone());
        let ret = AdapterVersion {
            tag_name: release.tag_name,
            url: release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?
                .browser_download_url
                .clone(),
        };

        Ok(ret)
    }

    async fn get_installed_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugAdapterConfig,
        _: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let Some(version) = self.last_known_version.get() else {
            bail!("Could not determine latest CodeLLDB version");
        };
        let adapter_path = paths::debug_adapters_dir().join(&Self::ADAPTER_NAME);
        let version_path = adapter_path.join(format!("{}_{}", Self::ADAPTER_NAME, version));

        let adapter_dir = version_path.join("extension").join("adapter");
        let command = adapter_dir.join("codelldb");
        let command = command
            .to_str()
            .map(ToOwned::to_owned)
            .ok_or_else(|| anyhow!("Adapter path is expected to be valid UTF-8"))?;
        Ok(DebugAdapterBinary {
            command,
            cwd: Some(adapter_dir),
            ..Default::default()
        })
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
            }
            DebugRequestType::Launch(launch) => {
                map.insert("program".into(), launch.program.clone().into());

                if !launch.args.is_empty() {
                    map.insert("args".into(), launch.args.clone().into());
                }

                if let Some(stop_on_entry) = config.stop_on_entry {
                    map.insert("stopOnEntry".into(), stop_on_entry.into());
                }
                if let Some(cwd) = launch.cwd.as_ref() {
                    map.insert("cwd".into(), cwd.to_string_lossy().into_owned().into());
                }
            }
        }
        args
    }
}
