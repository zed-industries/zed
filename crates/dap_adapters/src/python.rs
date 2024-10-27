use dap::transport::{StdioTransport, Transport};

use crate::*;

pub(crate) struct PythonDebugAdapter {}

impl PythonDebugAdapter {
    const ADAPTER_NAME: &'static str = "debugpy";
    const ADAPTER_PATH: &'static str = "src/debugpy/adapter";

    pub(crate) fn new() -> Self {
        PythonDebugAdapter {}
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PythonDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn transport(&self) -> Box<dyn Transport> {
        Box::new(StdioTransport::new())
    }

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion> {
        let github_repo = GithubRepo {
            repo_name: Self::ADAPTER_NAME.into(),
            repo_owner: "microsoft".into(),
        };

        adapters::fetch_latest_adapter_version_from_github(github_repo, delegate).await
    }

    async fn install_binary(
        &self,
        version: AdapterVersion,
        delegate: &dyn DapDelegate,
    ) -> Result<()> {
        adapters::download_adapter_from_github(self.name(), version, delegate).await?;
        Ok(())
    }

    async fn get_installed_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = paths::debug_adapters_dir().join(self.name());
        let file_name_prefix = format!("{}_", self.name());

        let debugpy_dir = util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
            file_name.starts_with(&file_name_prefix)
        })
        .await
        .ok_or_else(|| anyhow!("Debugpy directory not found"))?;

        let version = debugpy_dir
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .and_then(|file_name| file_name.strip_prefix(&file_name_prefix))
            .ok_or_else(|| anyhow!("Python debug adapter has invalid file name"))?
            .to_string();

        Ok(DebugAdapterBinary {
            command: "python3".to_string(),
            arguments: Some(vec![debugpy_dir.join(Self::ADAPTER_PATH).into()]),
            envs: None,
            version,
        })
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program, "subProcess": true})
    }
}
