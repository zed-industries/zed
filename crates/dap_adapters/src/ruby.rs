use anyhow::{Result, anyhow};
use async_trait::async_trait;
use dap::{
    DebugRequest, StartDebuggingRequestArguments,
    adapters::{
        self, DapDelegate, DebugAdapter, DebugAdapterBinary, DebugAdapterName, DebugTaskDefinition,
    },
};
use gpui::AsyncApp;
use std::path::PathBuf;
use util::command::new_smol_command;

use crate::ToDap;

#[derive(Default)]
pub(crate) struct RubyDebugAdapter;

impl RubyDebugAdapter {
    const ADAPTER_NAME: &'static str = "Ruby";
}

#[async_trait(?Send)]
impl DebugAdapter for RubyDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        definition: &DebugTaskDefinition,
        _user_installed_path: Option<PathBuf>,
        _cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = paths::debug_adapters_dir().join(self.name().as_ref());
        let mut rdbg_path = adapter_path.join("rdbg");
        if !delegate.fs().is_file(&rdbg_path).await {
            match delegate.which("rdbg".as_ref()) {
                Some(path) => rdbg_path = path,
                None => {
                    delegate.output_to_console(
                        "rdbg not found on path, trying `gem install debug`".to_string(),
                    );
                    let output = new_smol_command("gem")
                        .arg("install")
                        .arg("--no-document")
                        .arg("--bindir")
                        .arg(adapter_path)
                        .arg("debug")
                        .output()
                        .await?;
                    if !output.status.success() {
                        return Err(anyhow!(
                            "Failed to install rdbg:\n{}",
                            String::from_utf8_lossy(&output.stderr).to_string()
                        ));
                    }
                }
            }
        }

        let tcp_connection = definition.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let DebugRequest::Launch(mut launch) = definition.request.clone() else {
            anyhow::bail!("rdbg does not yet support attaching");
        };

        let mut arguments = vec![
            "--open".to_string(),
            format!("--port={}", port),
            format!("--host={}", host),
        ];
        if delegate.which(launch.program.as_ref()).is_some() {
            arguments.push("--command".to_string())
        }
        arguments.push(launch.program);
        arguments.extend(launch.args);

        Ok(DebugAdapterBinary {
            command: rdbg_path.to_string_lossy().to_string(),
            arguments,
            connection: Some(adapters::TcpArguments {
                host,
                port,
                timeout,
            }),
            cwd: launch.cwd,
            envs: launch.env.into_iter().collect(),
            request_args: StartDebuggingRequestArguments {
                configuration: serde_json::Value::Object(Default::default()),
                request: definition.request.to_dap(),
            },
        })
    }
}
