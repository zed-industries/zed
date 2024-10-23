use dap::transport::{TcpTransport, Transport};

use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct PhpDebugAdapter {}

impl PhpDebugAdapter {
    const ADAPTER_NAME: &'static str = "vscode-php-debug";
    const ADAPTER_PATH: &'static str = "out/phpDebug.js";

    pub(crate) fn new() -> Self {
        PhpDebugAdapter {}
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PhpDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn transport(&self) -> Box<dyn Transport> {
        Box::new(TcpTransport::new(TCPHost {
            port: Some(8132),
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
            file_name.starts_with("vscode-php-debug_")
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
                "--server=8132".into(),
            ]),
            envs: None,
        })
    }

    async fn install_binary(&self, delegate: &dyn DapDelegate) -> Result<()> {
        let github_repo = GithubRepo {
            repo_name: "vscode-php-debug".to_string(),
            repo_owner: "xdebug".to_string(),
        };

        let adapter_path =
            adapters::download_adapter_from_github(self.name(), github_repo, delegate).await?;

        let _ = delegate
            .node_runtime()
            .ok_or(anyhow!("Couldn't get npm runtime"))?
            .run_npm_subcommand(&adapter_path, "install", &[])
            .await
            .is_ok();

        let _ = delegate
            .node_runtime()
            .ok_or(anyhow!("Couldn't get npm runtime"))?
            .run_npm_subcommand(&adapter_path, "run", &["build"])
            .await
            .is_ok();

        Ok(())
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program})
    }
}
