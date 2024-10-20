use dap::transport::{StdioTransport, Transport};

use crate::*;

#[derive(Debug, Eq, PartialEq, Clone)]
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

    async fn fetch_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary> {
        let adapter_path = paths::debug_adapters_dir().join(self.name());

        Ok(DebugAdapterBinary {
            command: "python3".to_string(),
            arguments: Some(vec![adapter_path.join(Self::ADAPTER_PATH).into()]),
            envs: None,
        })
    }

    async fn install_binary(&self, delegate: &dyn DapDelegate) -> Result<()> {
        let adapter_path = paths::debug_adapters_dir().join(self.name());
        let fs = delegate.fs();

        if fs.is_dir(adapter_path.as_path()).await {
            return Ok(());
        }

        if let Some(http_client) = delegate.http_client() {
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

                return Ok(());
            }
        }

        bail!("Install or fetch not implemented for Python debug adapter (yet)");
    }

    fn request_args(&self, config: &DebugAdapterConfig) -> Value {
        json!({"program": config.program, "subProcess": true})
    }
}
