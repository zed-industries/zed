use dap::DapRegistry;
use editor::Editor;
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{FakeFs, Fs as _, Project};
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use task::{DebugRequest, DebugScenario, LaunchRequest, TaskContext, VariableName, ZedDebugConfig};
use text::Point;
use util::path;

use crate::NewProcessMode;
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

    let test_variables = vec![(
        VariableName::WorktreeRoot,
        path!("/test/worktree/path").to_string(),
    )]
    .into_iter()
    .collect();

    let task_context = TaskContext {
        cwd: None,
        task_variables: test_variables,
        project_env: Default::default(),
    };

    let home_dir = paths::home_dir();

    let test_cases: Vec<(&'static str, &'static str)> = vec![
        // Absolute path - should not be relativized
        (
            path!("/absolute/path/to/program"),
            path!("/absolute/path/to/program"),
        ),
        // Relative path - should be prefixed with worktree root
        (
            format!(".{0}src{0}program", std::path::MAIN_SEPARATOR).leak(),
            path!("/test/worktree/path/src/program"),
        ),
        // Home directory path - should be expanded to full home directory path
        (
            format!("~{0}src{0}program", std::path::MAIN_SEPARATOR).leak(),
            home_dir
                .join("src")
                .join("program")
                .to_string_lossy()
                .to_string()
                .leak(),
        ),
        // Path with $ZED_WORKTREE_ROOT - should be substituted without double appending
        (
            format!(
                "$ZED_WORKTREE_ROOT{0}src{0}program",
                std::path::MAIN_SEPARATOR
            )
            .leak(),
            path!("/test/worktree/path/src/program"),
        ),
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

                        assert_eq!(
                            config["program"].as_str().unwrap(),
                            expected_path,
                            "Program path was not correctly substituted for input: {}",
                            input_path
                        );

                        assert_eq!(
                            config["cwd"].as_str().unwrap(),
                            expected_path,
                            "CWD path was not correctly substituted for input: {}",
                            input_path
                        );

                        let expected_other_field = if input_path.contains("$ZED_WORKTREE_ROOT") {
                            input_path.replace("$ZED_WORKTREE_ROOT", path!("/test/worktree/path"))
                        } else {
                            input_path.to_string()
                        };

                        assert_eq!(
                            config["otherField"].as_str().unwrap(),
                            &expected_other_field,
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
                workspace.start_debug_session(
                    scenario,
                    task_context.clone(),
                    None,
                    None,
                    window,
                    cx,
                )
            })
            .unwrap();

        cx.run_until_parked();

        assert!(called_launch.load(Ordering::SeqCst));
        called_launch.store(false, Ordering::SeqCst);
    }
}

#[gpui::test]
async fn test_save_debug_scenario_to_file(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "fn main() {}"
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    workspace
        .update(cx, |workspace, window, cx| {
            crate::new_process_modal::NewProcessModal::show(
                workspace,
                window,
                NewProcessMode::Debug,
                None,
                cx,
            );
        })
        .unwrap();

    cx.run_until_parked();

    let modal = workspace
        .update(cx, |workspace, _, cx| {
            workspace.active_modal::<crate::new_process_modal::NewProcessModal>(cx)
        })
        .unwrap()
        .expect("Modal should be active");

    modal.update_in(cx, |modal, window, cx| {
        modal.set_configure("/project/main", "/project", false, window, cx);
        modal.save_debug_scenario(window, cx);
    });

    cx.executor().run_until_parked();

    let editor = workspace
        .update(cx, |workspace, _window, cx| {
            workspace.active_item_as::<Editor>(cx).unwrap()
        })
        .unwrap();

    let debug_json_content = fs
        .load(path!("/project/.zed/debug.json").as_ref())
        .await
        .expect("debug.json should exist")
        .lines()
        .filter(|line| !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    let expected_content = indoc::indoc! {r#"
        [
          {
            "adapter": "fake-adapter",
            "label": "main (fake-adapter)",
            "request": "launch",
            "program": "/project/main",
            "cwd": "/project",
            "args": [],
            "env": {}
          }
        ]"#};

    pretty_assertions::assert_eq!(expected_content, debug_json_content);

    editor.update(cx, |editor, cx| {
        assert_eq!(
            editor.selections.newest::<Point>(cx).head(),
            Point::new(5, 2)
        )
    });

    modal.update_in(cx, |modal, window, cx| {
        modal.set_configure("/project/other", "/project", true, window, cx);
        modal.save_debug_scenario(window, cx);
    });

    cx.executor().run_until_parked();

    let expected_content = indoc::indoc! {r#"
        [
          {
            "adapter": "fake-adapter",
            "label": "main (fake-adapter)",
            "request": "launch",
            "program": "/project/main",
            "cwd": "/project",
            "args": [],
            "env": {}
          },
          {
            "adapter": "fake-adapter",
            "label": "other (fake-adapter)",
            "request": "launch",
            "program": "/project/other",
            "cwd": "/project",
            "args": [],
            "env": {}
          }
        ]"#};

    let debug_json_content = fs
        .load(path!("/project/.zed/debug.json").as_ref())
        .await
        .expect("debug.json should exist")
        .lines()
        .filter(|line| !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    pretty_assertions::assert_eq!(expected_content, debug_json_content);
}

#[gpui::test]
async fn test_dap_adapter_config_conversion_and_validation(cx: &mut TestAppContext) {
    init_test(cx);

    let mut expected_adapters = vec![
        "CodeLLDB",
        "Debugpy",
        "JavaScript",
        "Delve",
        "GDB",
        "fake-adapter",
    ];

    let adapter_names = cx.update(|cx| {
        let registry = DapRegistry::global(cx);
        registry.enumerate_adapters::<Vec<_>>()
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
            .await
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
            .request_kind(&debug_scenario.config)
            .await
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
