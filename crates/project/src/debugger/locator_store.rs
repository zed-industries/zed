use anyhow::{anyhow, Result};
use async_trait::async_trait;
use collections::HashMap;
use dap::DebugAdapterConfig;
use gpui::SharedString;

pub(super) struct LocatorStore {
    locators: HashMap<SharedString, Box<dyn DapLocator>>,
}

impl LocatorStore {
    pub(super) fn new() -> Self {
        let locators = HashMap::from_iter([
            (
                SharedString::new("cargo"), 
                Box::new(CargoLocator {}) as Box<dyn DapLocator>
            )
        ]);
        Self { locators }
    }

    pub(super) async fn resolve_debug_config(
        &self,
        debug_config: &mut DebugAdapterConfig,
    ) -> Result<()> {
        let Some(ref locator_name) = &debug_config.locator else {
            log::debug!("Attempted to resolve debug config without a locator field");
            return Ok(());
        };

        if let Some(locator) = self.locators.get(locator_name as &str) {
            locator.run_locator(debug_config).await
        } else {
            Err(anyhow!("Couldn't find locator {}", locator_name))
        }
    }
}

#[async_trait]
trait DapLocator {
    async fn run_locator(&self, debug_config: &mut DebugAdapterConfig) -> Result<()>;
}

struct CargoLocator {}

#[async_trait]
impl DapLocator for CargoLocator {
    async fn run_locator(&self, debug_config: &mut DebugAdapterConfig) -> Result<()> {
        use serde_json::Value;
        use smol::{
            io::AsyncReadExt,
            process::{Command, Stdio},
        };

        let mut child = Command::new("cargo")
            .args(&debug_config.args)
            .arg("--message-format=json")
            .current_dir(debug_config.cwd.as_ref().unwrap())
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

        let executable = output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| serde_json::from_str(line).ok())
            .find_map(|json: Value| {
                json.get("executable")
                    .and_then(Value::as_str)
                    .map(String::from)
            });

        debug_config.program = executable;
        debug_config.args.clear();
        Ok(())
    }
}
