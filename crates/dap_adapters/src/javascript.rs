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

    async fn fetch_binary(
        &self,
        delegate: &dyn DapDelegate,
        _: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary> {
        let node_runtime = delegate
            .node_runtime()
            .ok_or(anyhow!("Couldn't get npm runtime"))?;

        let adapter_path = paths::debug_adapters_dir().join(self.name());
        let adapter_path = util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
            file_name.starts_with("vscode-js-debug_")
        })
        .await
        .ok_or_else(|| anyhow!("Couldn't find javascript dap directory"))?;

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
        })
    }

    async fn install_binary(&self, delegate: &dyn DapDelegate) -> Result<()> {
        let github_repo = GithubRepo {
            repo_name: "vscode-js-debug".to_string(),
            repo_owner: "microsoft".to_string(),
        };

        let adapter_path =
            adapters::download_adapter_from_github(self.name(), github_repo, delegate).await?;

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
