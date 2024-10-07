use std::str::FromStr;

use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct PhpDebugAdapter {
    program: String,
    adapter_path: Option<String>,
}

impl PhpDebugAdapter {
    const ADAPTER_NAME: &'static str = "vscode-php-debug";

    pub(crate) fn new(adapter_config: &DebugAdapterConfig) -> Self {
        PhpDebugAdapter {
            program: adapter_config.program.clone(),
            adapter_path: adapter_config.adapter_path.clone(),
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PhpDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn connect(
        &self,
        adapter_binary: DebugAdapterBinary,
        cx: &mut AsyncAppContext,
    ) -> Result<TransportParams> {
        let host = TCPHost {
            port: Some(8132),
            host: None,
            delay: Some(1000),
        };

        create_tcp_client(host, adapter_binary, cx).await
    }

    async fn install_or_fetch_binary(
        &self,
        delegate: Box<dyn DapDelegate>,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = paths::debug_adapters_dir().join(self.name());
        let fs = delegate.fs();

        if let Some(adapter_path) = self.adapter_path.as_ref() {
            return Ok(DebugAdapterBinary {
                start_command: Some("bun".into()),
                path: std::path::PathBuf::from_str(adapter_path)?,
                arguments: vec!["--server=8132".into()],
                env: None,
            });
        }

        if fs.is_dir(adapter_path.as_path()).await {
            return Ok(DebugAdapterBinary {
                start_command: Some("bun".into()),
                path: adapter_path.join("out/phpDebug.js"),
                arguments: vec!["--server=8132".into()],
                env: None,
            });
        } else if let Some(http_client) = delegate.http_client() {
            if !adapter_path.exists() {
                fs.create_dir(&adapter_path.as_path()).await?;
            }

            let release =
                latest_github_release("xdebug/vscode-php-debug", false, false, http_client.clone())
                    .await?;

            let asset_name = format!("{}-{}", self.name(), release.tag_name);
            let zip_path = adapter_path.join(asset_name);

            if fs::metadata(&zip_path).await.is_err() {
                let mut response = http_client
                    .get(&release.zipball_url, Default::default(), true)
                    .await
                    .context("Error downloading release")?;

                let mut file = File::create(&zip_path).await?;
                futures::io::copy(response.body_mut(), &mut file).await?;

                let _unzip_status = process::Command::new("unzip")
                    .current_dir(&adapter_path)
                    .arg(&zip_path)
                    .output()
                    .await?
                    .status;

                let mut ls = process::Command::new("ls")
                    .current_dir(&adapter_path)
                    .stdout(Stdio::piped())
                    .spawn()?;

                let std = ls
                    .stdout
                    .take()
                    .ok_or(anyhow!("Failed to list directories"))?
                    .into_stdio()
                    .await?;

                let file_name = String::from_utf8(
                    process::Command::new("grep")
                        .arg("xdebug-vscode-php-debug")
                        .stdin(std)
                        .output()
                        .await?
                        .stdout,
                )?;

                let file_name = file_name.trim_end();

                process::Command::new("sh")
                    .current_dir(&adapter_path)
                    .arg("-c")
                    .arg(format!("mv {file_name}/* ."))
                    .output()
                    .await?;

                process::Command::new("rm")
                    .current_dir(&adapter_path)
                    .arg("-rf")
                    .arg(file_name)
                    .arg(zip_path)
                    .output()
                    .await?;

                let _npm = delegate
                    .node_runtime()
                    .ok_or(anyhow!("Couldn't get npm runtime"))?
                    .run_npm_subcommand(&adapter_path, "run", &["build"])
                    .await
                    .is_ok();

                return Ok(DebugAdapterBinary {
                    start_command: Some("bun".into()),
                    path: adapter_path.join("out/phpDebug.js"),
                    arguments: vec!["--server=8132".into()],
                    env: None,
                });
            }
        }

        bail!("Install or fetch not implemented for Php debug adapter (yet)");
    }

    fn request_args(&self) -> Value {
        json!({"program": format!("{}", &self.program)})
    }
}
