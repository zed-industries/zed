mod go_locator {
    use collections::HashMap;
    use dap::{DapLocator, adapters::DebugAdapterName};
    use gpui::TestAppContext;
    use project::debugger::locators::go::{DelveLaunchRequest, GoLocator};
    use task::{HideStrategy, RevealStrategy, RevealTarget, Shell, TaskTemplate};
    #[gpui::test]
    async fn test_create_scenario_for_go_build(_: &mut TestAppContext) {
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

        let scenario = locator
            .create_scenario(&task, "test label", &DebugAdapterName("Delve".into()))
            .await;

        assert!(scenario.is_none());
    }

    #[gpui::test]
    async fn test_skip_non_go_commands_with_non_delve_adapter(_: &mut TestAppContext) {
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

        let scenario = locator
            .create_scenario(
                &task,
                "test label",
                &DebugAdapterName("SomeOtherAdapter".into()),
            )
            .await;
        assert!(scenario.is_none());

        let scenario = locator
            .create_scenario(&task, "test label", &DebugAdapterName("Delve".into()))
            .await;
        assert!(scenario.is_none());
    }
    #[gpui::test]
    async fn test_go_locator_run(_: &mut TestAppContext) {
        let locator = GoLocator;
        let delve = DebugAdapterName("Delve".into());

        let task = TaskTemplate {
            label: "go run with flags".into(),
            command: "go".into(),
            args: vec![
                "run".to_string(),
                "-race".to_string(),
                "-ldflags".to_string(),
                "-X main.version=1.0".to_string(),
                "./cmd/myapp".to_string(),
                "--config".to_string(),
                "production.yaml".to_string(),
                "--verbose".to_string(),
            ],
            env: {
                let mut env = HashMap::default();
                env.insert("GO_ENV".to_string(), "production".to_string());
                env
            },
            cwd: Some("/project/root".into()),
            ..Default::default()
        };

        let scenario = locator
            .create_scenario(&task, "test run label", &delve)
            .await
            .unwrap();

        let config: DelveLaunchRequest = serde_json::from_value(scenario.config).unwrap();

        assert_eq!(
            config,
            DelveLaunchRequest {
                request: "launch".to_string(),
                mode: "debug".to_string(),
                program: "./cmd/myapp".to_string(),
                build_flags: vec![
                    "-race".to_string(),
                    "-ldflags".to_string(),
                    "-X main.version=1.0".to_string()
                ],
                args: vec![
                    "--config".to_string(),
                    "production.yaml".to_string(),
                    "--verbose".to_string(),
                ],
                env: {
                    let mut env = HashMap::default();
                    env.insert("GO_ENV".to_string(), "production".to_string());
                    env
                },
                cwd: Some("/project/root".to_string()),
            }
        );
    }

    #[gpui::test]
    async fn test_go_locator_test(_: &mut TestAppContext) {
        let locator = GoLocator;
        let delve = DebugAdapterName("Delve".into());

        // Test with tags and run flag
        let task_with_tags = TaskTemplate {
            label: "test".into(),
            command: "go".into(),
            args: vec![
                "test".to_string(),
                "-tags".to_string(),
                "integration,unit".to_string(),
                "-run".to_string(),
                "Foo".to_string(),
                ".".to_string(),
            ],
            ..Default::default()
        };
        let result = locator
            .create_scenario(&task_with_tags, "", &delve)
            .await
            .unwrap();

        let config: DelveLaunchRequest = serde_json::from_value(result.config).unwrap();

        assert_eq!(
            config,
            DelveLaunchRequest {
                request: "launch".to_string(),
                mode: "test".to_string(),
                program: ".".to_string(),
                build_flags: vec!["-tags".to_string(), "integration,unit".to_string(),],
                args: vec![
                    "-test.run".to_string(),
                    "Foo".to_string(),
                    "-test.v".to_string()
                ],
                env: Default::default(),
                cwd: None,
            }
        );
    }

    #[gpui::test]
    async fn test_skip_unsupported_go_commands(_: &mut TestAppContext) {
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

        let scenario = locator
            .create_scenario(&task, "test label", &DebugAdapterName("Delve".into()))
            .await;
        assert!(scenario.is_none());
    }
}

mod python_locator {
    use dap::{DapLocator, adapters::DebugAdapterName};
    use serde_json::json;

    use project::debugger::locators::python::*;
    use task::{DebugScenario, TaskTemplate};

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

mod memory {
    use project::debugger::{
        MemoryCell,
        memory::{MemoryIterator, PageAddress, PageContents},
    };

    #[test]
    fn iterate_over_unmapped_memory() {
        let empty_iterator = MemoryIterator::new(0..=127, Default::default());
        let actual = empty_iterator.collect::<Vec<_>>();
        let expected = vec![MemoryCell(None); 128];
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }

    #[test]
    fn iterate_over_partially_mapped_memory() {
        let it = MemoryIterator::new(
            0..=127,
            vec![(PageAddress(5), PageContents::mapped(vec![1]))].into_iter(),
        );
        let actual = it.collect::<Vec<_>>();
        let expected = std::iter::repeat_n(MemoryCell(None), 5)
            .chain(std::iter::once(MemoryCell(Some(1))))
            .chain(std::iter::repeat_n(MemoryCell(None), 122))
            .collect::<Vec<_>>();
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }

    #[test]
    fn reads_from_the_middle_of_a_page() {
        let partial_iter = MemoryIterator::new(
            20..=30,
            vec![(PageAddress(0), PageContents::mapped((0..255).collect()))].into_iter(),
        );
        let actual = partial_iter.collect::<Vec<_>>();
        let expected = (20..=30)
            .map(|val| MemoryCell(Some(val)))
            .collect::<Vec<_>>();
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }
}
