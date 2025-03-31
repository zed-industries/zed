use super::DapLocator;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use dap::DebugAdapterConfig;
use serde_json::Value;
use smol::{
    io::AsyncReadExt,
    process::{Command, Stdio},
};

pub(super) struct CargoLocator {}

#[async_trait]
impl DapLocator for CargoLocator {
    async fn run_locator(&self, debug_config: &mut DebugAdapterConfig) -> Result<()> {
        let Some(launch_config) = (match &mut debug_config.request {
            task::DebugRequestDisposition::UserConfigured(task::DebugRequestType::Launch(
                launch_config,
            )) => Some(launch_config),
            _ => None,
        }) else {
            return Err(anyhow!("Couldn't get launch config in locator"));
        };

        let Some(cwd) = launch_config.cwd.clone() else {
            return Err(anyhow!(
                "Couldn't get cwd from debug config which is needed for locators"
            ));
        };

        let mut child = Command::new("cargo")
            .args(&debug_config.args)
            .arg("--message-format=json")
            .current_dir(cwd)
            .stdout(Stdio::piped())
            .spawn()?;

        let mut output = String::new();
        if let Some(mut stdout) = child.stdout.take() {
            stdout.read_to_string(&mut output).await?;
        }

        let status = child.status().await?;
        if !status.success() {
            return Err(anyhow::anyhow!("Cargo command failed"));
        }

        let Some(executable) = output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| serde_json::from_str(line).ok())
            .find_map(|json: Value| {
                json.get("executable")
                    .and_then(Value::as_str)
                    .map(String::from)
            })
        else {
            return Err(anyhow!("Couldn't get executable in cargo locator"));
        };

        launch_config.program = executable;
        debug_config.args.clear();
        Ok(())
    }
}
