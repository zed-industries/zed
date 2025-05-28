use anyhow::Result;
use async_trait::async_trait;
use collections::FxHashMap;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::SharedString;
use std::path::PathBuf;
use task::{
    BuildTaskDefinition, DebugScenario, RevealStrategy, RevealTarget, Shell, SpawnInTerminal,
    TaskTemplate,
};

pub(crate) struct GoLocator;

#[async_trait]
impl DapLocator for GoLocator {
    fn name(&self) -> SharedString {
        SharedString::new_static("go-debug-locator")
    }

    fn create_scenario(
        &self,
        build_config: &TaskTemplate,
        resolved_label: &str,
        adapter: DebugAdapterName,
    ) -> Option<DebugScenario> {
        let go_action = build_config.args.first()?;

        match go_action.as_str() {
            "run" => {
                let program = build_config
                    .args
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| ".".to_string());

                let build_task = TaskTemplate {
                    label: "go build debug".into(),
                    command: "go".into(),
                    args: vec![
                        "build".into(),
                        "-gcflags \"all=-N -l\"".into(),
                        program.clone(),
                    ],
                    env: build_config.env.clone(),
                    cwd: build_config.cwd.clone(),
                    use_new_terminal: false,
                    allow_concurrent_runs: false,
                    reveal: RevealStrategy::Always,
                    reveal_target: RevealTarget::Dock,
                    hide: task::HideStrategy::Never,
                    shell: Shell::System,
                    tags: vec![],
                    show_summary: true,
                    show_command: true,
                };

                Some(DebugScenario {
                    label: resolved_label.to_string().into(),
                    adapter: adapter.0,
                    build: Some(BuildTaskDefinition::Template {
                        task_template: build_task,
                        locator_name: Some(self.name()),
                    }),
                    config: serde_json::Value::Null,
                    tcp_connection: None,
                })
            }
            _ => None,
        }
    }

    async fn run(&self, build_config: SpawnInTerminal) -> Result<DebugRequest> {
        if build_config.args.is_empty() {
            return Err(anyhow::anyhow!("Invalid Go command"));
        }

        let go_action = &build_config.args[0];
        let cwd = build_config
            .cwd
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());

        let mut env = FxHashMap::default();
        for (key, value) in &build_config.env {
            env.insert(key.clone(), value.clone());
        }

        match go_action.as_str() {
            "build" => {
                let package = build_config
                    .args
                    .get(2)
                    .cloned()
                    .unwrap_or_else(|| ".".to_string());

                Ok(DebugRequest::Launch(task::LaunchRequest {
                    program: package,
                    cwd: Some(PathBuf::from(&cwd)),
                    args: vec![],
                    env,
                }))
            }
            _ => Err(anyhow::anyhow!("Unsupported Go command: {}", go_action)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use task::{HideStrategy, RevealStrategy, RevealTarget, Shell, TaskTemplate};

    #[test]
    fn test_create_scenario_for_go_run() {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go run main.go".into(),
            command: "go".into(),
            args: vec!["run".into(), "main.go".into()],
            env: Default::default(),
            cwd: Some("${ZED_WORKTREE_ROOT}".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            tags: vec![],
            show_summary: true,
            show_command: true,
        };

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));

        assert!(scenario.is_some());
        let scenario = scenario.unwrap();
        assert_eq!(scenario.adapter, "Delve");
        assert_eq!(scenario.label, "test label");
        assert!(scenario.build.is_some());

        if let Some(BuildTaskDefinition::Template { task_template, .. }) = &scenario.build {
            assert_eq!(task_template.command, "go");
            assert!(task_template.args.contains(&"build".into()));
            assert!(
                task_template
                    .args
                    .contains(&"-gcflags \"all=-N -l\"".into())
            );
            assert!(task_template.args.contains(&"main.go".into()));
        } else {
            panic!("Expected BuildTaskDefinition::Template");
        }

        assert!(
            scenario.config.is_null(),
            "Initial config should be null to ensure it's invalid"
        );
    }

    #[test]
    fn test_create_scenario_for_go_build() {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go build".into(),
            command: "go".into(),
            args: vec!["build".into(), ".".into()],
            env: Default::default(),
            cwd: Some("${ZED_WORKTREE_ROOT}".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            tags: vec![],
            show_summary: true,
            show_command: true,
        };

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));

        assert!(scenario.is_none());
    }

    #[test]
    fn test_skip_non_go_commands_with_non_delve_adapter() {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "cargo build".into(),
            command: "cargo".into(),
            args: vec!["build".into()],
            env: Default::default(),
            cwd: Some("${ZED_WORKTREE_ROOT}".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            tags: vec![],
            show_summary: true,
            show_command: true,
        };

        let scenario = locator.create_scenario(
            &task,
            "test label",
            DebugAdapterName("SomeOtherAdapter".into()),
        );
        assert!(scenario.is_none());

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));
        assert!(scenario.is_none());
    }

    #[test]
    fn test_skip_unsupported_go_commands() {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go clean".into(),
            command: "go".into(),
            args: vec!["clean".into()],
            env: Default::default(),
            cwd: Some("${ZED_WORKTREE_ROOT}".into()),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            tags: vec![],
            show_summary: true,
            show_command: true,
        };

        let scenario =
            locator.create_scenario(&task, "test label", DebugAdapterName("Delve".into()));
        assert!(scenario.is_none());
    }
}
