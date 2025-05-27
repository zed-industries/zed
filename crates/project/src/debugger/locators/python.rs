use anyhow::{Context as _, Result, bail};
use async_trait::async_trait;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::SharedString;
use serde_json::Value;
use smol::{
    io::AsyncReadExt,
    process::{Command, Stdio},
};
use task::{BuildTaskDefinition, DebugScenario, ShellBuilder, SpawnInTerminal, TaskTemplate};

pub(crate) struct PythonLocator;

#[async_trait]
impl DapLocator for PythonLocator {
    fn name(&self) -> SharedString {
        SharedString::new_static("Python")
    }

    /// Determines whether this locator can generate debug target for given task.
    fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: DebugAdapterName,
    ) -> Option<DebugScenario> {
        if adapter.as_ref() != "Debugpy" {
            return None;
        }
        let module_specifier_position = build_config
            .args
            .windows(2)
            .position(|tuple| tuple[0] == "-m");
        let mod_name = module_specifier_position.and_then(|pos| build_config.args.get(pos + 1));
        let args_start = module_specifier_position
            .map(|pos| pos + 1)
            .unwrap_or_default();
        let args = build_config.args[args_start..].to_owned();
        Some(DebugScenario {
            adapter: adapter.0,
            label: resolved_label.to_string().into(),
            build: None,
            config: serde_json::json!({
                "request": "launch",
                "python": build_config.command,
                "args": args,
                "module": mod_name,
                "cwd": build_config.cwd.clone()
            }),
            tcp_connection: None,
        })
    }

    async fn run(&self, build_config: SpawnInTerminal) -> Result<DebugRequest> {
        bail!("nope");
    }
}
