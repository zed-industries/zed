use std::path::Path;

use anyhow::{Result, bail};
use async_trait::async_trait;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::SharedString;

use task::{DebugScenario, SpawnInTerminal, TaskTemplate, VariableName};

pub(crate) struct PythonLocator;

#[async_trait]
impl DapLocator for PythonLocator {
    fn name(&self) -> SharedString {
        SharedString::new_static("Python")
    }

    /// Determines whether this locator can generate debug target for given task.
    async fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: &DebugAdapterName,
    ) -> Option<DebugScenario> {
        if adapter.0.as_ref() != "Debugpy" {
            return None;
        }
        let valid_program = build_config.command.starts_with("$ZED_")
            || Path::new(&build_config.command)
                .file_name()
                .is_some_and(|name| name.to_str().is_some_and(|path| path.starts_with("python")));
        if !valid_program || build_config.args.iter().any(|arg| arg == "-c") {
            // We cannot debug selections.
            return None;
        }
        let command = build_config.command.clone();
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

        let program_position = mod_name
            .is_none()
            .then(|| {
                let zed_file = VariableName::File.template_value_with_whitespace();
                build_config.args.iter().position(|arg| *arg == zed_file)
            })
            .flatten();
        let args = if let Some(position) = program_position {
            args.into_iter().skip(position).collect::<Vec<_>>()
        } else {
            args
        };
        if program_position.is_none() && mod_name.is_none() {
            return None;
        }
        let mut config = serde_json::json!({
            "request": "launch",
            "python": command,
            "args": args,
            "cwd": build_config.cwd.clone()
        });
        if let Some(config_obj) = config.as_object_mut() {
            if let Some(module) = mod_name {
                config_obj.insert("module".to_string(), module.clone().into());
            }
            if let Some(program) = program_position {
                config_obj.insert(
                    "program".to_string(),
                    build_config.args[program].clone().into(),
                );
            }
        }

        Some(DebugScenario {
            adapter: adapter.0.clone(),
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

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[gpui::test]
    async fn test_python_locator() {
        let adapter = DebugAdapterName("Debugpy".into());
        let build_task = TaskTemplate {
            label: "run module '$ZED_FILE'".into(),
            command: "$ZED_CUSTOM_PYTHON_ACTIVE_ZED_TOOLCHAIN".into(),
            args: vec!["-m".into(), "$ZED_CUSTOM_PYTHON_MODULE_NAME".into()],
            env: Default::default(),
            cwd: Some("$ZED_WORKTREE_ROOT".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: task::RevealStrategy::Always,
            reveal_target: task::RevealTarget::Dock,
            hide: task::HideStrategy::Never,
            tags: vec!["python-module-main-method".into()],
            shell: task::Shell::System,
            show_summary: false,
            show_command: false,
        };

        let expected_scenario = DebugScenario {
            adapter: "Debugpy".into(),
            label: "run module 'main.py'".into(),
            build: None,
            config: json!({
                "request": "launch",
                "python": "$ZED_CUSTOM_PYTHON_ACTIVE_ZED_TOOLCHAIN",
                "args": [],
                "cwd": "$ZED_WORKTREE_ROOT",
                "module": "$ZED_CUSTOM_PYTHON_MODULE_NAME",
            }),
            tcp_connection: None,
        };

        assert_eq!(
            PythonLocator
                .create_scenario(&build_task, "run module 'main.py'", &adapter)
                .await
                .expect("Failed to create a scenario"),
            expected_scenario
        );
    }
}
