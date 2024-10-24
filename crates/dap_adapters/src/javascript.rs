use dap::transport::{TcpTransport, Transport};

use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct JsDebugAdapter {}

impl JsDebugAdapter {
    const ADAPTER_NAME: &'static str = "vscode-js-debug";
    const ADAPTER_PATH: &'static str = "src/dapDebugServer.js";

    pub(crate) fn new() -> Self {
        JsDebugAdapter {}
    }
}

#[async_trait(?Send)]
impl DebugAdapter for JsDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn transport(&self) -> Box<dyn Transport> {
        Box::new(TcpTransport::new(TCPHost {
            port: Some(8133),
            host: None,
            timeout: None,
        }))
    }

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion> {
        let github_repo = GithubRepo {
            repo_name: Self::ADAPTER_NAME.into(),
            repo_owner: "microsoft".to_string(),
        };

        adapters::fetch_latest_adapter_version_from_github(github_repo, delegate).await
    }

    async fn get_installed_binary(
        &self,
        delegate: &dyn DapDelegate,
        _: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary> {
        let node_runtime = delegate
            .node_runtime()
            .ok_or(anyhow!("Couldn't get npm runtime"))?;

        let adapter_path = paths::debug_adapters_dir().join(self.name());
        let file_name_prefix = format!("{}_", self.name());

        let adapter_path = util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
            file_name.starts_with(&file_name_prefix)
        })
        .await
        .ok_or_else(|| anyhow!("Couldn't find Javascript dap directory"))?;

        let version = adapter_path
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .and_then(|file_name| file_name.strip_prefix(&file_name_prefix))
            .ok_or_else(|| anyhow!("Javascript debug adapter has invalid file name"))?
            .to_string();

        Ok(DebugAdapterBinary {
            command: node_runtime
                .binary_path()
                .await?
                .to_string_lossy()
                .into_owned(),
            arguments: Some(vec![
                adapter_path.join(Self::ADAPTER_PATH).into(),
                "8133".into(),
            ]),
            envs: None,
            version,
        })
    }

    async fn install_binary(
        &self,
        version: AdapterVersion,
        delegate: &dyn DapDelegate,
    ) -> Result<()> {
        let adapter_path =
            adapters::download_adapter_from_github(self.name(), version, delegate).await?;

        let _ = delegate
            .node_runtime()
            .ok_or(anyhow!("Couldn't get npm runtime"))?
            .run_npm_subcommand(&adapter_path, "install", &[])
            .await
            .ok();

        let _ = delegate
            .node_runtime()
            .ok_or(anyhow!("Couldn't get npm runtime"))?
            .run_npm_subcommand(&adapter_path, "run", &["compile"])
            .await
            .ok();

        return Ok(());
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({
            "program": config.program,
            "type": "pwa-node",
        })
    }
}
