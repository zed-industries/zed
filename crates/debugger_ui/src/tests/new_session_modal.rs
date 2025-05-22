use anyhow::{Context as _, Result};
use dap::adapters::DebugTaskDefinition;
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext, WindowHandle};
use project::{FakeFs, Project, WorktreeId};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use task::{DebugScenario, TaskContext, VariableName};
use util::path;
use workspace::Workspace;

use crate::debugger_panel::DebugPanel;
use crate::session::running::RunningState;
use crate::tests::{active_debug_session_panel, init_test, init_test_workspace};

#[gpui::test]
async fn test_debug_session_substitutes_variables_and_relativizes_paths(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) -> Result<()> {
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

    // Test cases for different path formats
    let test_cases = vec![
        // Absolute path - should not be relativized
        ("/absolute/path/to/program", "/absolute/path/to/program"),
        // Path with $ZED_WORKTREE_ROOT - should be substituted
        (
            "$ZED_WORKTREE_ROOT/src/program",
            "/test/worktree/path/src/program",
        ),
        // Relative path - should be prefixed with worktree root
        ("./src/program", "/test/worktree/path/src/program"),
        // Implicit relative path - should be prefixed with worktree root
        ("src/program", "/test/worktree/path/src/program"),
        // Home directory path - should be prefixed with worktree root
        ("~/src/program", "/test/worktree/path/src/program"),
    ];

    let called_launch = Arc::new(AtomicBool::new(false));

    for (input_path, expected_path) in test_cases {
        let _subscription = project::debugger::test::intercept_debug_sessions(cx, {
            let called_launch = called_launch.clone();
            move |client| {
                client.on_request::<dap::requests::Launch, _>({
                    let called_launch = called_launch.clone();
                    move |_, args| {
                        let config = args.raw.as_object().unwrap();

                        // Verify the program path was substituted correctly
                        assert_eq!(
                            config["program"].as_str().unwrap(),
                            expected_path,
                            "Program path was not correctly substituted for input: {}",
                            input_path
                        );

                        // Verify the cwd path was substituted correctly
                        assert_eq!(
                            config["cwd"].as_str().unwrap(),
                            expected_path,
                            "CWD path was not correctly substituted for input: {}",
                            input_path
                        );

                        // Verify that otherField was substituted but not relativized
                        // It should still have $ZED_WORKTREE_ROOT substituted if present
                        let expected_other_field = if input_path.contains("$ZED_WORKTREE_ROOT") {
                            input_path.replace("$ZED_WORKTREE_ROOT", "/test/worktree/path")
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

        // Create a debug scenario with the input path
        let scenario = DebugScenario {
            adapter: "fake-adapter".into(),
            label: "test-debug-session".into(),
            build: None,
            config: json!({
                "request": "launch",
                "program": input_path,
                "cwd": input_path,
                "otherField": input_path // This field should not be relativized
            }),
            tcp_connection: None,
        };

        // Start the debug session
        workspace
            .update(cx, |workspace, window, cx| {
                workspace.start_debug_session(scenario, task_context.clone(), None, window, cx)
            })
            .unwrap();

        cx.run_until_parked();

        assert!(called_launch.load(Ordering::SeqCst));
        called_launch.store(false, Ordering::SeqCst);
    }

    Ok(())
}
