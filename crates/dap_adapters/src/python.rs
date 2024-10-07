use std::str::FromStr;

use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct PythonDebugAdapter {
    program: String,
    adapter_path: Option<String>,
}

impl PythonDebugAdapter {
    const ADAPTER_NAME: &'static str = "debugpy";

    pub(crate) fn new(adapter_config: &DebugAdapterConfig) -> Self {
        PythonDebugAdapter {
            program: adapter_config.program.clone(),
            adapter_path: adapter_config.adapter_path.clone(),
        }
    }
}

#[async_trait(?Send)]
impl DebugAdapter for PythonDebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn connect(
        &self,
        adapter_binary: DebugAdapterBinary,
        _cx: &mut AsyncAppContext,
    ) -> Result<TransportParams> {
        create_stdio_client(adapter_binary)
    }

    async fn install_or_fetch_binary(
        &self,
        delegate: Box<dyn DapDelegate>,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = paths::debug_adapters_dir().join("debugpy/src/debugpy/adapter");
        let fs = delegate.fs();

        if let Some(adapter_path) = self.adapter_path.as_ref() {
            return Ok(DebugAdapterBinary {
                start_command: Some("python3".to_string()),
                path: std::path::PathBuf::from_str(&adapter_path)?,
                arguments: vec![],
                env: None,
            });
        }

        if fs.is_dir(adapter_path.as_path()).await {
            return Ok(DebugAdapterBinary {
                start_command: Some("python3".to_string()),
                path: adapter_path,
                arguments: vec![],
                env: None,
            });
        } else if let Some(http_client) = delegate.http_client() {
            let debugpy_dir = paths::debug_adapters_dir().join("debugpy");

            if !debugpy_dir.exists() {
                fs.create_dir(&debugpy_dir.as_path()).await?;
            }

            let release =
                latest_github_release("microsoft/debugpy", false, false, http_client.clone())
                    .await?;
            let asset_name = format!("{}.zip", release.tag_name);

            let zip_path = debugpy_dir.join(asset_name);

            if fs::metadata(&zip_path).await.is_err() {
                let mut response = http_client
                    .get(&release.zipball_url, Default::default(), true)
                    .await
                    .context("Error downloading release")?;

                let mut file = File::create(&zip_path).await?;
                futures::io::copy(response.body_mut(), &mut file).await?;

                let _unzip_status = process::Command::new("unzip")
                    .current_dir(&debugpy_dir)
                    .arg(&zip_path)
                    .output()
                    .await?
                    .status;

                let mut ls = process::Command::new("ls")
                    .current_dir(&debugpy_dir)
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
                        .arg("microsoft-debugpy")
                        .stdin(std)
                        .output()
                        .await?
                        .stdout,
                )?;

                let file_name = file_name.trim_end();
                process::Command::new("sh")
                    .current_dir(&debugpy_dir)
                    .arg("-c")
                    .arg(format!("mv {file_name}/* ."))
                    .output()
                    .await?;

                process::Command::new("rm")
                    .current_dir(&debugpy_dir)
                    .arg("-rf")
                    .arg(file_name)
                    .arg(zip_path)
                    .output()
                    .await?;

                return Ok(DebugAdapterBinary {
                    start_command: Some("python3".to_string()),
                    path: adapter_path,
                    arguments: vec![],
                    env: None,
                });
            }
            return Err(anyhow!("Failed to download debugpy"));
        } else {
            return Err(anyhow!(
                "Could not find debugpy in paths or connect to http"
            ));
        }
    }

    fn request_args(&self) -> Value {
        json!({"program": format!("{}", &self.program)})
    }
}
