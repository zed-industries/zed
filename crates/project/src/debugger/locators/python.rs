use anyhow::{Result, bail};
use async_trait::async_trait;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::SharedString;

use task::{DebugScenario, SpawnInTerminal, TaskTemplate};

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
            .iter()
            .position(|arg| arg == "-m")
            .map(|position| position + 1);
        // Skip the -m and module name, get all that's after.
        let mut rest_of_the_args = module_specifier_position
            .and_then(|position| build_config.args.get(position..))
            .into_iter()
            .flatten()
            .fuse();
        let mod_name = rest_of_the_args.next();
        let args = rest_of_the_args.collect::<Vec<_>>();

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

    async fn run(&self, _: SpawnInTerminal) -> Result<DebugRequest> {
        bail!("Python locator should not require DapLocator::run to be ran");
    }
}
