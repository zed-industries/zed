use anyhow::Result;
use async_trait::async_trait;
use collections::FxHashMap;
use dap::{DapLocator, DebugRequest, adapters::DebugAdapterName};
use gpui::SharedString;
use std::path::PathBuf;
use task::{DebugScenario, SpawnInTerminal, TaskTemplate};

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
        if build_config.command != "go" {
            return None;
        }

        let go_action = build_config.args.first()?;

        match go_action.as_ref() {
            "run" => {
                let program = build_config
                    .args
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| ".".to_string());
                let args = build_config.args.get(2..).unwrap_or(&[]).to_vec();

                let config = serde_json::json!({
                    "request": "launch",
                    "mode": "debug",
                    "program": program,
                    "args": args,
                    "cwd": build_config.cwd.as_deref().unwrap_or("${ZED_WORKTREE_ROOT}"),
                    "env": build_config.env,
                    "console": "integratedTerminal",
                    "outputMode": "remote"
                });

                Some(DebugScenario {
                    label: resolved_label.to_string().into(),
                    adapter: adapter.0,
                    build: None, // No build task - Delve handles everything
                    config,
                    tcp_connection: None,
                })
            }
            "test" => {
                let package = build_config
                    .args
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| ".".to_string());
                let args = build_config.args.get(2..).unwrap_or(&[]).to_vec();

                let config = serde_json::json!({
                    "request": "launch",
                    "mode": "test",
                    "program": package,
                    "args": args,
                    "cwd": build_config.cwd.as_deref().unwrap_or("${ZED_WORKTREE_ROOT}"),
                    "env": build_config.env,
                    "console": "integratedTerminal",
                    "outputMode": "remote"
                });

                Some(DebugScenario {
                    label: resolved_label.to_string().into(),
                    adapter: adapter.0,
                    build: None, // No build task - Delve handles everything
                    config,
                    tcp_connection: None,
                })
            }
            "build" => {
                let package = build_config
                    .args
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| ".".to_string());

                let config = serde_json::json!({
                    "request": "launch",
                    "mode": "debug",
                    "program": package,
                    "cwd": build_config.cwd.as_deref().unwrap_or("${ZED_WORKTREE_ROOT}"),
                    "env": build_config.env,
                    "console": "integratedTerminal",
                    "outputMode": "remote"
                });

                Some(DebugScenario {
                    label: resolved_label.to_string().into(),
                    adapter: adapter.0,
                    build: None, // No build task - Delve handles everything
                    config,
                    tcp_connection: None,
                })
            }
            _ => None, // Unsupported Go commands
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
            "run" => {
                let program = build_config
                    .args
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| ".".to_string());

                Ok(DebugRequest::Launch(task::LaunchRequest {
                    program,
                    cwd: Some(PathBuf::from(&cwd)),
                    args: build_config.args.get(2..).unwrap_or(&[]).to_vec(),
                    env,
                }))
            }
            "test" => {
                let package = build_config
                    .args
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| ".".to_string());

                Ok(DebugRequest::Launch(task::LaunchRequest {
                    program: package,
                    cwd: Some(PathBuf::from(&cwd)),
                    args: build_config.args.get(2..).unwrap_or(&[]).to_vec(),
                    env,
                }))
            }
            "build" => {
                let package = build_config
                    .args
                    .get(1)
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
        assert!(scenario.build.is_none());
    }

    #[test]
    fn test_create_scenario_for_go_test() {
        let locator = GoLocator;
        let task = TaskTemplate {
            label: "go test".into(),
            command: "go".into(),
            args: vec!["test".into(), "./...".into()],
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
        assert!(scenario.build.is_none());
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

        assert!(scenario.is_some());
        let scenario = scenario.unwrap();
        assert_eq!(scenario.adapter, "Delve");
        assert_eq!(scenario.label, "test label");
        assert!(scenario.build.is_none());
    }

    #[test]
    fn test_skip_non_go_commands() {
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
