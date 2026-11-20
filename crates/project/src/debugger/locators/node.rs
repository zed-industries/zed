use std::borrow::Cow;

use anyhow::{Result, bail};
use async_trait::async_trait;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::{BackgroundExecutor, SharedString};

use task::{DebugScenario, SpawnInTerminal, TaskTemplate, VariableName};

pub(crate) struct NodeLocator;

const TYPESCRIPT_RUNNER_VARIABLE: VariableName =
    VariableName::Custom(Cow::Borrowed("TYPESCRIPT_RUNNER"));

/// Commands js-debug can launch directly. Package managers are included
/// because they spawn the test runner as a child process, which js-debug
/// attaches to on its own.
const SUPPORTED_COMMANDS: &[&str] = &["node", "npm", "pnpm", "yarn", "npx"];

fn is_supported_command(command: &str) -> bool {
    if command == TYPESCRIPT_RUNNER_VARIABLE.template_value() {
        return true;
    }
    // Tasks may spell the command out as a path, such as `/usr/local/bin/node`,
    // and Windows installs these as `node.exe` or `npm.cmd`.
    let name = command.rsplit(['/', '\\']).next().unwrap_or(command);
    let name = name
        .strip_suffix(".exe")
        .or_else(|| name.strip_suffix(".cmd"))
        .unwrap_or(name);
    SUPPORTED_COMMANDS.contains(&name)
}

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
        if !is_supported_command(&build_config.command) {
            return None;
        }

        // Everything after the command is passed as runtime arguments: js-debug
        // launches `runtimeExecutable` with them and attaches to whatever child
        // process it spawns, so the inspector flag must not be injected here.
        // Adding it would break the command for package managers, which reject
        // unknown options, and would stop a direct `node` invocation on the
        // runner rather than on the test.
        let config = serde_json::json!({
            "request": "launch",
            "type": "pwa-node",
            "cwd": build_config.cwd.clone(),
            "env": build_config.env.clone(),
            "runtimeExecutable": build_config.command.clone(),
            "runtimeArgs": build_config.args.clone(),
            "console": "integratedTerminal",
            "autoAttachChildProcesses": true,
            "skipFiles": ["<node_internals>/**"],
        });

        Some(DebugScenario {
            adapter: adapter.0.clone(),
            label: resolved_label.to_string().into(),
            build: None,
            config,
            tcp_connection: None,
        })
    }

    async fn run(&self, _: SpawnInTerminal, _executor: BackgroundExecutor) -> Result<DebugRequest> {
        bail!("JavaScript locator should not require DapLocator::run to be ran");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(command: &str, args: &[&str]) -> TaskTemplate {
        TaskTemplate {
            label: "test".to_owned(),
            command: command.to_owned(),
            args: args.iter().map(|arg| (*arg).to_owned()).collect(),
            cwd: Some("/project".to_owned()),
            ..TaskTemplate::default()
        }
    }

    fn scenario(command: &str, args: &[&str]) -> Option<DebugScenario> {
        scenario_for_adapter(command, args, "JavaScript")
    }

    fn scenario_for_adapter(command: &str, args: &[&str], adapter: &str) -> Option<DebugScenario> {
        futures::executor::block_on(NodeLocator.create_scenario(
            &task(command, args),
            "node test example",
            &DebugAdapterName(adapter.to_owned().into()),
        ))
    }

    #[test]
    fn test_supported_commands() {
        for command in [
            "node",
            "npm",
            "pnpm",
            "yarn",
            "npx",
            "/usr/local/bin/node",
            "node.exe",
            "npm.cmd",
            TYPESCRIPT_RUNNER_VARIABLE.template_value().as_str(),
        ] {
            assert!(is_supported_command(command), "{command} must be supported");
        }
        for command in ["bun", "deno", "cargo", "python", "nodemon"] {
            assert!(
                !is_supported_command(command),
                "{command} is not launched by js-debug"
            );
        }
    }

    #[test]
    fn test_node_test_runner_scenario() {
        let scenario = scenario(
            "node",
            &["--test", "--test-name-pattern", "\"adds\"", "a.test.ts"],
        )
        .expect("the built-in Node test runner must be debuggable");

        assert_eq!(scenario.config["type"], "pwa-node");
        assert_eq!(scenario.config["runtimeExecutable"], "node");
        assert_eq!(
            scenario.config["runtimeArgs"],
            serde_json::json!(["--test", "--test-name-pattern", "\"adds\"", "a.test.ts"])
        );
        assert_eq!(scenario.config["cwd"], "/project");
        assert!(
            scenario.config.get("args").is_none(),
            "arguments belong to the runtime invocation, not to a program"
        );
    }

    #[test]
    fn test_package_manager_scenario_does_not_inspect_the_manager() {
        let scenario = scenario("npm", &["exec", "--", "vitest", "run", "a.test.ts"])
            .expect("package manager tasks must be debuggable");

        assert_eq!(scenario.config["runtimeExecutable"], "npm");
        assert_eq!(
            scenario.config["runtimeArgs"],
            serde_json::json!(["exec", "--", "vitest", "run", "a.test.ts"]),
            "the inspector flag must not be inserted before the manager's own arguments"
        );
        assert_eq!(scenario.config["autoAttachChildProcesses"], true);
    }

    #[test]
    fn test_unrelated_commands_and_adapters_are_ignored() {
        assert!(scenario("cargo", &["test"]).is_none());
        assert!(
            scenario_for_adapter("node", &["--test"], "Debugpy").is_none(),
            "another language's adapter must not be handed a Node configuration"
        );
    }
}
