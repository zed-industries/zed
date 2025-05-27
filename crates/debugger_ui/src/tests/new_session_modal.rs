use dap::DapRegistry;
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{FakeFs, Project};
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use task::{DebugRequest, DebugScenario, LaunchRequest, TaskContext, VariableName, ZedDebugConfig};
use util::path;

use crate::tests::{init_test, init_test_workspace};

#[gpui::test]
async fn test_debug_session_substitutes_variables_and_relativizes_paths(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "fn main() {}"
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    // Set up task variables to simulate a real environment
    let test_variables = vec![(
        VariableName::WorktreeRoot,
        "/test/worktree/path".to_string(),
    )]
    .into_iter()
    .collect();

    let task_context = TaskContext {
        cwd: None,
        task_variables: test_variables,
        project_env: Default::default(),
    };

    let home_dir = paths::home_dir();

    let sep = std::path::MAIN_SEPARATOR;

    // Test cases for different path formats
    let test_cases: Vec<(Arc<String>, Arc<String>)> = vec![
        // Absolute path - should not be relativized
        (
            Arc::from(format!("{0}absolute{0}path{0}to{0}program", sep)),
            Arc::from(format!("{0}absolute{0}path{0}to{0}program", sep)),
        ),
        // Relative path - should be prefixed with worktree root
        (
            Arc::from(format!(".{0}src{0}program", sep)),
            Arc::from(format!("{0}test{0}worktree{0}path{0}src{0}program", sep)),
        ),
        // Home directory path - should be prefixed with worktree root
        (
            Arc::from(format!("~{0}src{0}program", sep)),
            Arc::from(format!(
                "{1}{0}src{0}program",
                sep,
                home_dir.to_string_lossy()
            )),
        ),
        // Path with $ZED_WORKTREE_ROOT - should be substituted without double appending
        (
            Arc::from(format!("$ZED_WORKTREE_ROOT{0}src{0}program", sep)),
            Arc::from(format!("{0}test{0}worktree{0}path{0}src{0}program", sep)),
        ),
    ];

    let called_launch = Arc::new(AtomicBool::new(false));

    for (input_path, expected_path) in test_cases {
        let _subscription = project::debugger::test::intercept_debug_sessions(cx, {
            let called_launch = called_launch.clone();
            let input_path = input_path.clone();
            let expected_path = expected_path.clone();
            move |client| {
                client.on_request::<dap::requests::Launch, _>({
                    let called_launch = called_launch.clone();
                    let input_path = input_path.clone();
                    let expected_path = expected_path.clone();

                    move |_, args| {
                        let config = args.raw.as_object().unwrap();

                        // Verify the program path was substituted correctly
                        assert_eq!(
                            config["program"].as_str().unwrap(),
                            expected_path.as_str(),
                            "Program path was not correctly substituted for input: {}",
                            input_path.as_str()
                        );

                        // Verify the cwd path was substituted correctly
                        assert_eq!(
                            config["cwd"].as_str().unwrap(),
                            expected_path.as_str(),
                            "CWD path was not correctly substituted for input: {}",
                            input_path.as_str()
                        );

                        // Verify that otherField was substituted but not relativized
                        // It should still have $ZED_WORKTREE_ROOT substituted if present
                        let expected_other_field = if input_path.contains("$ZED_WORKTREE_ROOT") {
                            input_path.replace("$ZED_WORKTREE_ROOT", path!("/test/worktree/path"))
                        } else {
                            input_path.to_string()
                        };

                        assert_eq!(
                            config["otherField"].as_str().unwrap(),
                            expected_other_field,
                            "Other field was incorrectly modified for input: {}",
                            input_path
                        );

                        called_launch.store(true, Ordering::SeqCst);

                        Ok(())
                    }
                });
            }
        });

        let scenario = DebugScenario {
            adapter: "fake-adapter".into(),
            label: "test-debug-session".into(),
            build: None,
            config: json!({
                "request": "launch",
                "program": input_path,
                "cwd": input_path,
                "otherField": input_path
            }),
            tcp_connection: None,
        };

        workspace
            .update(cx, |workspace, window, cx| {
                workspace.start_debug_session(scenario, task_context.clone(), None, window, cx)
            })
            .unwrap();

        cx.run_until_parked();

        assert!(called_launch.load(Ordering::SeqCst));
        called_launch.store(false, Ordering::SeqCst);
    }
}

#[gpui::test]
async fn test_dap_adapter_config_conversion_and_validation(cx: &mut TestAppContext) {
    init_test(cx);

    let mut expected_adapters = vec![
        "CodeLLDB",
        "Debugpy",
        "PHP",
        "JavaScript",
        "Ruby",
        "Delve",
        "GDB",
        "fake-adapter",
    ];

    let adapter_names = cx.update(|cx| {
        let registry = DapRegistry::global(cx);
        registry.enumerate_adapters()
    });

    let zed_config = ZedDebugConfig {
        label: "test_debug_session".into(),
        adapter: "test_adapter".into(),
        request: DebugRequest::Launch(LaunchRequest {
            program: "test_program".into(),
            cwd: None,
            args: vec![],
            env: Default::default(),
        }),
        stop_on_entry: Some(true),
    };

    for adapter_name in adapter_names {
        let adapter_str = adapter_name.to_string();
        if let Some(pos) = expected_adapters.iter().position(|&x| x == adapter_str) {
            expected_adapters.remove(pos);
        }

        let adapter = cx
            .update(|cx| {
                let registry = DapRegistry::global(cx);
                registry.adapter(adapter_name.as_ref())
            })
            .unwrap_or_else(|| panic!("Adapter {} should exist", adapter_name));

        let mut adapter_specific_config = zed_config.clone();
        adapter_specific_config.adapter = adapter_name.to_string().into();

        let debug_scenario = adapter
            .config_from_zed_format(adapter_specific_config)
            .unwrap_or_else(|_| {
                panic!(
                    "Adapter {} should successfully convert from Zed format",
                    adapter_name
                )
            });

        assert!(
            debug_scenario.config.is_object(),
            "Adapter {} should produce a JSON object for config",
            adapter_name
        );

        let request_type = adapter
            .validate_config(&debug_scenario.config)
            .unwrap_or_else(|_| {
                panic!(
                    "Adapter {} should validate the config successfully",
                    adapter_name
                )
            });

        match request_type {
            dap::StartDebuggingRequestArgumentsRequest::Launch => {}
            dap::StartDebuggingRequestArgumentsRequest::Attach => {
                panic!(
                    "Expected Launch request but got Attach for adapter {}",
                    adapter_name
                );
            }
        }
    }

    assert!(
        expected_adapters.is_empty(),
        "The following expected adapters were not found in the registry: {:?}",
        expected_adapters
    );
}
