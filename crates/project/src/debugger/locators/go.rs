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
use uuid::Uuid;

pub(crate) struct GoLocator;

impl GoLocator {
    fn get_build_tags(&self, build_config: &TaskTemplate) -> Vec<String> {
        let mut tags = Vec::new();
        let mut i = 0;
        let args = &build_config.args;
        while i < args.len() {
            if args[i] == "-tags" && i + 1 < args.len() {
                tags.push("-tags".to_string());
                tags.push(args[i + 1].clone());
                i += 2;
            } else {
                i += 1;
            }
        }
        tags
    }
}

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
        if build_config.command != "go" {
            return None;
        }

        let go_action = build_config.args.first()?;

        match go_action.as_str() {
            "test" => {
                let binary_path = format!("__debug_{}", Uuid::new_v4().simple());
                let build_tags = self.get_build_tags(build_config);

                let mut args = vec!["test".into(), "-c".into()];
                args.extend(build_tags);
                args.extend(vec![
                    "-gcflags \"all=-N -l\"".into(),
                    "-o".into(),
                    binary_path,
                ]);

                let build_task = TaskTemplate {
                    label: "go test debug".into(),
                    command: "go".into(),
                    args,
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
            "run" => {
                let program = build_config
                    .args
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| ".".to_string());
                let build_tags = self.get_build_tags(build_config);

                let mut args = vec!["build".into()];
                args.extend(build_tags);
                args.extend(vec!["-gcflags \"all=-N -l\"".into(), program.clone()]);

                let build_task = TaskTemplate {
                    label: "go build debug".into(),
                    command: "go".into(),
                    args,
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
            "test" => {
                // Find the binary path after the -o flag
                let binary_arg = build_config
                    .args
                    .windows(2)
                    .find(|window| window[0] == "-o")
                    .map(|window| &window[1])
                    .ok_or_else(|| anyhow::anyhow!("can't locate debug binary"))?;

                let program = PathBuf::from(&cwd)
                    .join(binary_arg)
                    .to_string_lossy()
                    .into_owned();

                Ok(DebugRequest::Launch(task::LaunchRequest {
                    program,
                    cwd: Some(PathBuf::from(&cwd)),
                    args: vec!["-test.v".into(), "-test.run=${ZED_SYMBOL}".into()],
                    env,
                }))
            }
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
    use task::{HideStrategy, RevealStrategy, RevealTarget, Shell, TaskId, TaskTemplate};

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
    fn test_create_scenario_for_go_test() {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go test".into(),
            command: "go".into(),
            args: vec!["test".into(), ".".into()],
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
            assert!(task_template.args.contains(&"test".into()));
            assert!(task_template.args.contains(&"-c".into()));
            assert!(
                task_template
                    .args
                    .contains(&"-gcflags \"all=-N -l\"".into())
            );
            assert!(task_template.args.contains(&"-o".into()));
            assert!(
                task_template
                    .args
                    .iter()
                    .any(|arg| arg.starts_with("__debug_"))
            );
        } else {
            panic!("Expected BuildTaskDefinition::Template");
        }

        assert!(
            scenario.config.is_null(),
            "Initial config should be null to ensure it's invalid"
        );
    }

    #[test]
    fn test_create_scenario_for_go_test_with_tags() {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go test with tags".into(),
            command: "go".into(),
            args: vec![
                "test".into(),
                "-tags".into(),
                "integration,e2e".into(),
                ".".into(),
            ],
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

        if let Some(BuildTaskDefinition::Template { task_template, .. }) = &scenario.build {
            assert!(task_template.args.contains(&"test".into()));
            assert!(task_template.args.contains(&"-c".into()));
            assert!(task_template.args.contains(&"-tags".into()));
            assert!(task_template.args.contains(&"integration,e2e".into()));
            assert!(
                task_template
                    .args
                    .contains(&"-gcflags \"all=-N -l\"".into())
            );
        } else {
            panic!("Expected BuildTaskDefinition::Template");
        }
    }

    #[test]
    fn test_get_build_tags() {
        let locator = GoLocator;

        // Test with tags
        let task_with_tags = TaskTemplate {
            label: "test".into(),
            command: "go".into(),
            args: vec![
                "test".to_string(),
                "-tags".to_string(),
                "integration,unit".to_string(),
                ".".to_string(),
            ],
            env: FxHashMap::default(),
            cwd: None,
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
        let tags = locator.get_build_tags(&task_with_tags);
        assert_eq!(
            tags,
            vec!["-tags".to_string(), "integration,unit".to_string()]
        );

        // Test without tags
        let task_without_tags = TaskTemplate {
            label: "test".into(),
            command: "go".into(),
            args: vec!["test".to_string(), ".".to_string()],
            env: FxHashMap::default(),
            cwd: None,
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
        let tags = locator.get_build_tags(&task_without_tags);
        assert!(tags.is_empty());

        let task_multiple_tags = TaskTemplate {
            label: "test".into(),
            command: "go".into(),
            args: vec![
                "test".to_string(),
                "-tags".to_string(),
                "unit".to_string(),
                "-tags".to_string(),
                "integration".to_string(),
            ],
            env: FxHashMap::default(),
            cwd: None,
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
        let tags = locator.get_build_tags(&task_multiple_tags);
        assert_eq!(
            tags,
            vec![
                "-tags".to_string(),
                "unit".to_string(),
                "-tags".to_string(),
                "integration".to_string()
            ]
        );
    }

    #[test]
    fn test_create_scenario_for_go_test_with_cwd_binary() {
        let locator = GoLocator;

        let task = TaskTemplate {
            label: "go test".into(),
            command: "go".into(),
            args: vec!["test".into(), ".".into()],
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

        if let Some(BuildTaskDefinition::Template { task_template, .. }) = &scenario.build {
            assert!(
                task_template
                    .args
                    .iter()
                    .any(|arg| arg.starts_with("__debug_"))
            );
        } else {
            panic!("Expected BuildTaskDefinition::Template");
        }
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

    #[test]
    fn test_run_go_test_missing_binary_path() {
        let locator = GoLocator;
        let build_config = SpawnInTerminal {
            id: TaskId("test_task".to_string()),
            full_label: "go test".to_string(),
            label: "go test".to_string(),
            command: "go".into(),
            args: vec![
                "test".into(),
                "-c".into(),
                "-gcflags \"all=-N -l\"".into(),
                "-o".into(),
            ], // Missing the binary path after -o
            command_label: "go test -c -gcflags \"all=-N -l\" -o".to_string(),
            env: Default::default(),
            cwd: Some(PathBuf::from("/test/path")),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            show_summary: true,
            show_command: true,
            show_rerun: true,
        };

        let result = futures::executor::block_on(locator.run(build_config));
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("can't locate debug binary")
        );
    }

    #[test]
    fn test_run_go_test_with_tags() {
        let locator = GoLocator;
        let build_config = SpawnInTerminal {
            id: TaskId("test_task".to_string()),
            full_label: "go test".to_string(),
            label: "go test".to_string(),
            command: "go".into(),
            args: vec![
                "test".into(),
                "-c".into(),
                "-tags".into(),
                "integration".into(),
                "-gcflags \"all=-N -l\"".into(),
                "-o".into(),
                "__debug_binary".into(),
            ],
            command_label: "go test -c -tags integration -gcflags \"all=-N -l\" -o __debug_binary"
                .to_string(),
            env: Default::default(),
            cwd: Some(PathBuf::from("/test/path")),
            use_new_terminal: false,
            allow_concurrent_runs: false,
            reveal: RevealStrategy::Always,
            reveal_target: RevealTarget::Dock,
            hide: HideStrategy::Never,
            shell: Shell::System,
            show_summary: true,
            show_command: true,
            show_rerun: true,
        };

        let result = futures::executor::block_on(locator.run(build_config));
        assert!(result.is_ok());

        if let Ok(DebugRequest::Launch(launch_request)) = result {
            assert!(launch_request.program.ends_with("__debug_binary"));
            assert_eq!(
                launch_request.args,
                vec!["-test.v".to_string(), "-test.run=${ZED_SYMBOL}".to_string()]
            );
        } else {
            panic!("Expected LaunchRequest");
        }
    }
}
