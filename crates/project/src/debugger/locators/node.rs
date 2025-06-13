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
        let test_library = build_config.args.first()?;
        let program_path_base: PathBuf = match test_library.as_str() {
            "jest" => "${ZED_CUSTOM_TYPESCRIPT_JEST_PACKAGE_PATH}".to_owned(),
            "mocha" => "${ZED_CUSTOM_TYPESCRIPT_MOCHA_PACKAGE_PATH}".to_owned(),
            "vitest" => "${ZED_CUSTOM_TYPESCRIPT_VITEST_PACKAGE_PATH}".to_owned(),
            "jasmine" => "${ZED_CUSTOM_TYPESCRIPT_JASMINE_PACKAGE_PATH}".to_owned(),
            _ => VariableName::WorktreeRoot.template_value(),
        }
        .into();

        let program_path = program_path_base
            .join("node_modules")
            .join(".bin")
            .join(test_library);

        let mut args = if test_library == "jest" {
            vec!["--runInBand".to_owned()]
        } else {
            vec![]
        };
        args.extend(build_config.args[1..].iter().cloned());

        let config = serde_json::json!({
            "request": "launch",
            "type": "pwa-node",
            "runtimeExecutable": program_path,
            "args": args,
            "cwd": build_config.cwd.clone(),
            "env": {
                "VITEST_MIN_FORKS": "0",
                "VITEST_MAX_FORKS": "1"
            },
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
