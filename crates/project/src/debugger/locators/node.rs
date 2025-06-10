use std::{borrow::Cow, path::Path};

use anyhow::{Result, bail};
use async_trait::async_trait;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::SharedString;

use task::{DebugScenario, SpawnInTerminal, TaskTemplate, VariableName};

pub(crate) struct NodeLocator;

const TYPESCRIPT_RUNNER_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_RUNNER"));

#[async_trait]
impl DapLocator for NodeLocator {
    fn name(&self) -> SharedString {
        SharedString::new_static("Node")
    }

    /// Determines whether this locator can generate debug target for given task.
    fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: DebugAdapterName,
    ) -> Option<DebugScenario> {
        // TODO(debugger) fix issues with `await` breakpoint step
        if cfg!(not(debug_assertions)) {
            return None;
        }
        if adapter.0.as_ref() != "JavaScript" {
            return None;
        }
        if build_config.command != TYPESCRIPT_RUNNER_VARIABLE.template_value() {
            return None;
        }
        let test_library = build_config.args.first()?;
        let program_path = Path::new("$ZED_WORKTREE_ROOT")
            .join("node_modules")
            .join(".bin")
            .join(test_library);
        let args = build_config.args[1..].to_vec();

        let config = serde_json::json!({
            "request": "launch",
            "type": "pwa-node",
            "program": program_path,
            "args": args,
            "cwd": build_config.cwd.clone(),
            "runtimeArgs": ["--inspect-brk"],
            "console": "integratedTerminal",
        });

        Some(DebugScenario {
            adapter: adapter.0,
            label: resolved_label.to_string().into(),
            build: None,
            config,
            tcp_connection: None,
        })
    }

    async fn run(&self, _: SpawnInTerminal) -> Result<DebugRequest> {
        bail!("Python locator should not require DapLocator::run to be ran");
    }
}
