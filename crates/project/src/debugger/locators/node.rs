use std::{borrow::Cow, path::PathBuf};

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
    async fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: &DebugAdapterName,
    ) -> Option<DebugScenario> {
        if adapter.0.as_ref() != "JavaScript" {
            return None;
        }
        if build_config.command != TYPESCRIPT_RUNNER_VARIABLE.template_value() {
            return None;
        }

        let test_binary_base = match build_config.args.first()?.as_str() {
            "jest" => Some("${ZED_CUSTOM_TYPESCRIPT_JEST_PACKAGE_PATH}".to_owned()),
            "mocha" => Some("${ZED_CUSTOM_TYPESCRIPT_MOCHA_PACKAGE_PATH}".to_owned()),
            "vitest" => Some("${ZED_CUSTOM_TYPESCRIPT_VITEST_PACKAGE_PATH}".to_owned()),
            "jasmine" => Some("${ZED_CUSTOM_TYPESCRIPT_JASMINE_PACKAGE_PATH}".to_owned()),
            _ => None,
        };

        let (runtime_executable, args) = if let Some(test_binary_base) = test_binary_base {
            (
                PathBuf::from(test_binary_base)
                    .join("node_modules")
                    .join(".bin")
                    .join(build_config.args.first()?.as_str())
                    .to_string_lossy()
                    .to_string(),
                build_config.args[1..].iter().cloned().collect::<Vec<_>>(),
            )
        } else {
            (build_config.command.clone(), build_config.args.clone())
        };

        let config = serde_json::json!({
            "request": "launch",
            "type": "pwa-node",
            "args": args,
            "cwd": build_config.cwd.clone(),
            "runtimeExecutable": runtime_executable,
            "runtimeArgs": ["--inspect-brk"],
            "console": "integratedTerminal",
        });

        Some(DebugScenario {
            adapter: adapter.0.clone(),
            label: resolved_label.to_string().into(),
            build: None,
            config,
            tcp_connection: None,
        })
    }

    async fn run(&self, _: SpawnInTerminal) -> Result<DebugRequest> {
        bail!("JavaScript locator should not require DapLocator::run to be ran");
    }
}
