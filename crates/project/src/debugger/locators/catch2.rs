use anyhow::{Result, bail};
use async_trait::async_trait;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::{BackgroundExecutor, SharedString};
use task::{BuildTaskDefinition, DebugScenario, SpawnInTerminal, TaskTemplate};

pub(crate) struct Catch2Locator;

const CATCH2_TEST_BUILD_TASK: &str = "CATCH2_TEST_BUILD_TASK";

#[async_trait]
impl DapLocator for Catch2Locator {
    fn name(&self) -> SharedString {
        SharedString::new_static("catch2-locator")
    }

    async fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: &DebugAdapterName,
    ) -> Option<DebugScenario> {
        let is_catch2 = build_config
            .tags
            .iter()
            .any(|tag| tag == "catch2-test" || tag == "catch2-section");
        if !is_catch2 {
            return None;
        }

        let config = serde_json::json!({
            "request": "launch",
            "program": build_config.command,
            "args": build_config.args,
            "cwd": build_config.cwd,
        });

        let build = build_config
            .env
            .get(CATCH2_TEST_BUILD_TASK)
            .map(|name| BuildTaskDefinition::ByName(name.clone().into()));

        Some(DebugScenario {
            adapter: adapter.0.clone(),
            label: resolved_label.to_string().into(),
            build,
            config,
            tcp_connection: None,
        })
    }

    async fn run(&self, _: SpawnInTerminal, _executor: BackgroundExecutor) -> Result<DebugRequest> {
        bail!("Catch2 locator does not require DapLocator::run to be called");
    }
}
