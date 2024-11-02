use dap::transport::{TcpTransport, Transport};
use regex::Regex;
use std::{collections::HashMap, net::Ipv4Addr};
use sysinfo::{Pid, Process};
use task::DebugRequestType;
use util::maybe;

use crate::*;

pub(crate) struct JsDebugAdapter {
    port: u16,
    host: Ipv4Addr,
    timeout: Option<u64>,
}

impl JsDebugAdapter {
    const ADAPTER_NAME: &'static str = "vscode-js-debug";
    const ADAPTER_PATH: &'static str = "src/dapDebugServer.js";

    pub(crate) async fn new(host: TCPHost) -> Result<Self> {
        Ok(JsDebugAdapter {
            host: host.host(),
            timeout: host.timeout,
            port: TcpTransport::port(&host).await?,
        })
    }
}

#[async_trait(?Send)]
impl DebugAdapter for JsDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn transport(&self) -> Box<dyn Transport> {
        Box::new(TcpTransport::new(self.host, self.port, self.timeout))
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
        config: &DebugAdapterConfig,
        user_installed_path: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        let node_runtime = delegate
            .node_runtime()
            .ok_or(anyhow!("Couldn't get npm runtime"))?;

        let adapter_path = paths::debug_adapters_dir().join(self.name());
        let file_name_prefix = format!("{}_", self.name());

        let adapter_info: Result<_> = maybe!(async {
            let adapter_path =
                util::fs::find_file_name_in_dir(adapter_path.as_path(), |file_name| {
                    file_name.starts_with(&file_name_prefix)
                })
                .await
                .ok_or_else(|| anyhow!("Couldn't find Php dap directory"))?;

            let version = adapter_path
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .and_then(|file_name| file_name.strip_prefix(&file_name_prefix))
                .ok_or_else(|| anyhow!("PHP debug adapter has invalid file name"))?
                .to_string();

            Ok((adapter_path, version))
        })
        .await;

        let (adapter_path, version) = match user_installed_path {
            Some(path) => (path, "N/A".into()),
            None => adapter_info?,
        };

        Ok(DebugAdapterBinary {
            command: node_runtime
                .binary_path()
                .await?
                .to_string_lossy()
                .into_owned(),
            arguments: Some(vec![
                adapter_path.join(Self::ADAPTER_PATH).into(),
                self.port.to_string().into(),
            ]),
            cwd: config.cwd.clone(),
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
        let pid = if let DebugRequestType::Attach(attach_config) = &config.request {
            attach_config.process_id
        } else {
            None
        };

        json!({
            "program": config.program,
            "type": "pwa-node",
            "request": match config.request {
                DebugRequestType::Launch => "launch",
                DebugRequestType::Attach(_) => "attach",
            },
            "processId": pid,
            "cwd": config.cwd,
        })
    }

    fn supports_attach(&self) -> bool {
        true
    }

    fn attach_processes<'a>(
        &self,
        processes: &'a HashMap<Pid, Process>,
    ) -> Option<Vec<(&'a Pid, &'a Process)>> {
        let regex = Regex::new(r"(?i)^(?:node|bun|iojs)(?:$|\b)").unwrap();

        Some(
            processes
                .iter()
                .filter(|(_, process)| regex.is_match(&process.name().to_string_lossy()))
                .collect::<Vec<_>>(),
        )
    }
}
