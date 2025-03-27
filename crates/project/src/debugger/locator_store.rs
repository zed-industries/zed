use anyhow::Result;
use dap::DebugAdapterConfig;

pub(super) struct LocatorStore {
    locators: Vec<CargoLocator>,
}

impl LocatorStore {
    pub(super) fn new() -> Self {
        Self {
            locators: vec![CargoLocator {}],
        }
    }

    pub(super) async fn resolve_debug_config(
        &self,
        debug_config: &mut DebugAdapterConfig,
    ) -> Result<()> {
        let Some(ref locator_name) = &debug_config.locator else {
            log::debug!("Attempted to resolve debug config without a locator field");
            return Ok(());
        };

        match locator_name.as_str() {
            "cargo" => self.locators[0].run_cargo_build_json(debug_config).await,
            _ => Err(anyhow::anyhow!("Unsupported locator: {}", locator_name)),
        }
    }
}

struct CargoLocator {}

impl CargoLocator {
    async fn run_cargo_build_json(&self, debug_config: &mut DebugAdapterConfig) -> Result<()> {
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
