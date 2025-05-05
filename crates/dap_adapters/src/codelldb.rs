use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use anyhow::Result;
use async_trait::async_trait;
use dap::adapters::{DebugTaskDefinition, InlineValueProvider, latest_github_release};
use gpui::AsyncApp;
use task::DebugRequest;

use crate::*;

#[derive(Default)]
pub(crate) struct CodeLldbDebugAdapter {
    path_to_codelldb: OnceLock<String>,
}

impl CodeLldbDebugAdapter {
    const ADAPTER_NAME: &'static str = "CodeLLDB";

    fn request_args(&self, config: &DebugTaskDefinition) -> dap::StartDebuggingRequestArguments {
        let mut configuration = json!({
            "request": match config.request {
                DebugRequest::Launch(_) => "launch",
                DebugRequest::Attach(_) => "attach",
            },
        });
        let map = configuration.as_object_mut().unwrap();
        // CodeLLDB uses `name` for a terminal label.
        map.insert(
            "name".into(),
            Value::String(String::from(config.label.as_ref())),
        );
        let request = config.request.to_dap();
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
                    map.insert("stopOnEntry".into(), stop_on_entry.into());
                }
                if let Some(cwd) = launch.cwd.as_ref() {
                    map.insert("cwd".into(), cwd.to_string_lossy().into_owned().into());
                }
            }
        }
        dap::StartDebuggingRequestArguments {
            request,
            configuration,
        }
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
}

#[async_trait(?Send)]
impl DebugAdapter for CodeLldbDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let mut command = user_installed_path
            .map(|p| p.to_string_lossy().to_string())
            .or(self.path_to_codelldb.get().cloned());

        if command.is_none() {
            delegate.output_to_console(format!("Checking latest version of {}...", self.name()));
            let version = self.fetch_latest_adapter_version(delegate).await?;
            adapters::download_adapter_from_github(
                self.name(),
                version.clone(),
                adapters::DownloadedFileType::Vsix,
                delegate,
            )
            .await?;
            let adapter_path = paths::debug_adapters_dir().join(&Self::ADAPTER_NAME);
            let version_path =
                adapter_path.join(format!("{}_{}", Self::ADAPTER_NAME, version.tag_name));

            let adapter_dir = version_path.join("extension").join("adapter");
            let path = adapter_dir.join("codelldb");
            let path = path
                .to_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| anyhow!("Adapter path is expected to be valid UTF-8"))?;
            self.path_to_codelldb.set(path.clone()).ok();
            command = Some(path);
        };

        Ok(DebugAdapterBinary {
            command: command.unwrap(),
            cwd: None,
            arguments: vec![
                "--settings".into(),
                json!({"sourceLanguages": ["cpp", "rust"]}).to_string(),
            ],
            request_args: self.request_args(config),
            envs: HashMap::default(),
            connection: None,
        })
    }

    fn inline_value_provider(&self) -> Option<Box<dyn InlineValueProvider>> {
        Some(Box::new(CodeLldbInlineValueProvider))
    }
}

struct CodeLldbInlineValueProvider;

impl InlineValueProvider for CodeLldbInlineValueProvider {
    fn provide(&self, variables: Vec<(String, lsp_types::Range)>) -> Vec<lsp_types::InlineValue> {
        variables
            .into_iter()
            .map(|(variable, range)| {
                lsp_types::InlineValue::VariableLookup(lsp_types::InlineValueVariableLookup {
                    range,
                    variable_name: Some(variable),
                    case_sensitive_lookup: true,
                })
            })
            .collect()
    }
}
