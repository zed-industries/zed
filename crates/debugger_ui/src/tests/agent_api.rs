#![expect(clippy::result_large_err)]
use crate::tests::{init_test, init_test_workspace, start_debug_session};
use dap::{
    ErrorResponse, Message, Scope, StackFrame, Variable,
    requests::{Continue, Scopes, SetBreakpoints, StackTrace, Threads, Variables},
};
use gpui::{BackgroundExecutor, TestAppContext};
use project::debugger::agent_api::{
    AgentDebuggerApi, AgentDebuggerSessionStatus, AgentDebuggerSnapshotLimits,
    AgentDebuggerThreadStatus, AgentSourceBreakpointInput,
};
use project::{FakeFs, Project, WorktreeId};
use serde_json::json;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use task::SharedTaskContext;
use unindent::Unindent as _;
use util::path;

fn agent_api(project: &gpui::Entity<Project>, cx: &mut TestAppContext) -> AgentDebuggerApi {
    project.read_with(cx, |project, _| {
        AgentDebuggerApi::new(project.dap_store(), project.breakpoint_store())
    })
}

fn error_response(message: &str) -> ErrorResponse {
    ErrorResponse {
        error: Some(Message {
            id: 1,
            format: message.into(),
            variables: None,
            send_telemetry: None,
            show_user: None,
            url: None,
            url_label: None,
        }),
    }
}

#[gpui::test]
async fn test_agent_api_breakpoint_editing(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(
        path!("/project"),
        json!({
            "src": {
                "main.js": "let a = 1;\nlet b = 2;\nlet c = 3;\n",
            }
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let api = agent_api(&project, cx);
    let path = PathBuf::from(path!("/project/src/main.js"));

    let input = AgentSourceBreakpointInput {
        path: path.clone(),
        line: 2,
        enabled: true,
        condition: None,
        hit_condition: None,
        log_message: None,
    };

    // Adding a new breakpoint reports a change.
    let result = cx
        .update(|cx| api.set_source_breakpoint(input.clone(), cx))
        .await
        .unwrap();
    assert!(result.changed);
    assert_eq!(result.line, 2);

    // Setting an identical breakpoint is idempotent.
    let result = cx
        .update(|cx| api.set_source_breakpoint(input.clone(), cx))
        .await
        .unwrap();
    assert!(!result.changed);

    let breakpoints = cx.update(|cx| api.list_breakpoints(cx));
    assert_eq!(breakpoints.len(), 1);
    assert_eq!(breakpoints[0].path, path);
    assert_eq!(breakpoints[0].line, 2);
    assert!(breakpoints[0].enabled);
    assert_eq!(breakpoints[0].condition, None);

    // Setting the same line with a condition updates the breakpoint in place
    // rather than toggling it off.
    let mut conditional = input.clone();
    conditional.condition = Some("a > 0".to_string());
    let result = cx
        .update(|cx| api.set_source_breakpoint(conditional, cx))
        .await
        .unwrap();
    assert!(result.changed);

    let breakpoints = cx.update(|cx| api.list_breakpoints(cx));
    assert_eq!(breakpoints.len(), 1);
    assert_eq!(breakpoints[0].condition.as_deref(), Some("a > 0"));

    // Lines outside the file are rejected instead of being clipped.
    let mut out_of_range = input.clone();
    out_of_range.line = 1000;
    let result = cx
        .update(|cx| api.set_source_breakpoint(out_of_range, cx))
        .await;
    assert!(result.is_err());

    // Removing reports a change the first time and is a no-op afterwards.
    let result = cx
        .update(|cx| api.remove_source_breakpoint(path.clone(), 2, cx))
        .await
        .unwrap();
    assert!(result.changed);
    let result = cx
        .update(|cx| api.remove_source_breakpoint(path.clone(), 2, cx))
        .await
        .unwrap();
    assert!(!result.changed);
    assert!(cx.update(|cx| api.list_breakpoints(cx)).is_empty());
}

#[gpui::test]
async fn test_agent_api_snapshot_is_bounded(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let test_file_content = r#"
        const variable1 = "Value 1";
        const variable2 = "Value 2";
        const variable3 = "Value 3";
        console.log(variable1);
    "#
    .unindent();

    fs.insert_tree(
        path!("/project"),
        json!({
            "src": {
                "test.js": test_file_content,
            }
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    let stack_frames = (1..=3)
        .map(|id| StackFrame {
            id,
            name: format!("Stack Frame {id}"),
            source: Some(dap::Source {
                name: Some("test.js".into()),
                path: Some(path!("/project/src/test.js").into()),
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data: None,
                checksums: None,
            }),
            line: id,
            column: 1,
            end_line: None,
            end_column: None,
            can_restart: None,
            instruction_pointer_reference: None,
            module_id: None,
            presentation_hint: None,
        })
        .collect::<Vec<_>>();

    client.on_request::<StackTrace, _>({
        let stack_frames = Arc::new(stack_frames);
        move |_, args| {
            assert_eq!(1, args.thread_id);
            Ok(dap::StackTraceResponse {
                stack_frames: (*stack_frames).clone(),
                total_frames: None,
            })
        }
    });

    client.on_request::<Scopes, _>(move |_, _| {
        Ok(dap::ScopesResponse {
            scopes: vec![Scope {
                name: "Locals".into(),
                presentation_hint: None,
                variables_reference: 2,
                named_variables: None,
                indexed_variables: None,
                expensive: false,
                source: None,
                line: None,
                column: None,
                end_line: None,
                end_column: None,
            }],
        })
    });

    client.on_request::<Variables, _>(move |_, args| {
        assert_eq!(2, args.variables_reference);
        let variable = |name: &str, value: &str| Variable {
            name: name.into(),
            value: value.into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        };
        Ok(dap::VariablesResponse {
            variables: vec![
                variable("variable1", "this value is too long to fit"),
                variable("variable2", "v2"),
                variable("variable3", "v3"),
            ],
        })
    });

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Pause,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    let api = agent_api(&project, cx);
    let session_id = session.read_with(cx, |session, _| session.session_id());

    let limits = AgentDebuggerSnapshotLimits {
        max_frames: 2,
        max_variables_per_scope: 2,
        max_variable_value_length: 8,
        max_output_events: 10,
        max_output_bytes: 1024,
        max_source_context_lines: 3,
    };
    let snapshot = cx
        .update(|cx| api.snapshot(session_id, limits, cx))
        .await
        .unwrap();

    assert_eq!(snapshot.session.session_id, session_id);
    assert_eq!(snapshot.session.status, AgentDebuggerSessionStatus::Stopped);
    assert!(snapshot.session.has_ever_stopped);

    assert_eq!(snapshot.threads.len(), 1);
    let thread = &snapshot.threads[0];
    assert_eq!(thread.name, "Thread 1");
    assert_eq!(thread.status, AgentDebuggerThreadStatus::Stopped);

    // Stack frames are bounded by max_frames.
    assert_eq!(thread.frames.len(), 2);
    assert!(
        snapshot
            .notes
            .iter()
            .any(|note| note.contains("Stack frames truncated")),
        "expected a stack frame truncation note, got {:?}",
        snapshot.notes
    );

    let frame = &thread.frames[0];
    assert_eq!(frame.name, "Stack Frame 1");
    assert_eq!(
        frame.source_path.as_deref(),
        Some(std::path::Path::new(path!("/project/src/test.js")))
    );

    // Source context is bounded by max_source_context_lines and centered on
    // the frame's line.
    let source_context = frame.source_context.as_ref().unwrap();
    assert!(source_context.lines.len() <= 3);
    assert!(
        source_context
            .lines
            .iter()
            .any(|line| line.line == frame.line as u32)
    );

    // Variables are bounded by max_variables_per_scope and their values by
    // max_variable_value_length.
    assert_eq!(frame.scopes.len(), 1);
    let scope = &frame.scopes[0];
    assert_eq!(scope.name, "Locals");
    assert_eq!(scope.variables.len(), 2);
    assert!(scope.variables_truncated);
    let long_variable = &scope.variables[0];
    assert_eq!(long_variable.name, "variable1");
    assert_eq!(long_variable.value, "this val");
    assert!(long_variable.value_truncated);
    assert!(!scope.variables[1].value_truncated);
}

#[gpui::test]
async fn test_agent_api_snapshot_skips_dap_requests_for_terminated_session(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(path!("/project"), json!({ "main.js": "" }))
        .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let session_id = session.read_with(cx, |session, _| session.session_id());
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());
    let thread_requests = Arc::new(Mutex::new(0));

    client.on_request::<Threads, _>({
        let thread_requests = thread_requests.clone();
        move |_, _| {
            *thread_requests.lock().unwrap() += 1;
            Err(error_response(
                "threads should not be requested for terminated sessions",
            ))
        }
    });

    client
        .fake_event(dap::messages::Events::Output(dap::OutputEvent {
            category: Some(dap::OutputEventCategory::Stdout),
            output: "session finished\n".to_string(),
            data: None,
            variables_reference: None,
            source: None,
            line: None,
            column: None,
            group: None,
            location_reference: None,
        }))
        .await;
    client
        .fake_event(dap::messages::Events::Terminated(Some(
            dap::TerminatedEvent { restart: None },
        )))
        .await;

    for _ in 0..100 {
        if session.read_with(cx, |session, _| session.is_terminated()) {
            break;
        }
        assert!(cx.dispatcher.tick(false));
    }
    assert!(session.read_with(cx, |session, _| session.is_terminated()));

    let api = agent_api(&project, cx);
    let snapshot = cx
        .update(|cx| {
            api.snapshot_session_for_test(
                session.clone(),
                AgentDebuggerSnapshotLimits::default(),
                cx,
            )
        })
        .await
        .unwrap();

    assert_eq!(snapshot.session.session_id, session_id);
    assert_eq!(
        snapshot.session.status,
        AgentDebuggerSessionStatus::Terminated
    );
    assert!(snapshot.threads.is_empty());
    assert_eq!(snapshot.output.len(), 1);
    assert!(
        snapshot
            .notes
            .iter()
            .any(|note| note == "Session has ended; threads were not requested")
    );
    assert_eq!(*thread_requests.lock().unwrap(), 0);
}

#[gpui::test]
async fn test_agent_api_snapshot_returns_partial_data_when_nested_requests_fail(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(path!("/project"), json!({ "main.js": "debugger;\n" }))
        .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![
                dap::Thread {
                    id: 1,
                    name: "Thread with stack error".into(),
                },
                dap::Thread {
                    id: 2,
                    name: "Thread with partial frames".into(),
                },
            ],
        })
    });

    client.on_request::<StackTrace, _>(move |_, args| match args.thread_id {
        1 => Err(error_response("stack trace failed")),
        2 => Ok(dap::StackTraceResponse {
            stack_frames: vec![
                StackFrame {
                    id: 20,
                    name: "Frame with scopes error".into(),
                    source: None,
                    line: 1,
                    column: 1,
                    end_line: None,
                    end_column: None,
                    can_restart: None,
                    instruction_pointer_reference: None,
                    module_id: None,
                    presentation_hint: None,
                },
                StackFrame {
                    id: 21,
                    name: "Frame with variables error".into(),
                    source: None,
                    line: 1,
                    column: 1,
                    end_line: None,
                    end_column: None,
                    can_restart: None,
                    instruction_pointer_reference: None,
                    module_id: None,
                    presentation_hint: None,
                },
            ],
            total_frames: None,
        }),
        thread_id => panic!("unexpected thread id {thread_id}"),
    });

    client.on_request::<Scopes, _>(move |_, args| match args.frame_id {
        20 => Err(error_response("scopes failed")),
        21 => Ok(dap::ScopesResponse {
            scopes: vec![Scope {
                name: "Locals".into(),
                presentation_hint: None,
                variables_reference: 200,
                named_variables: Some(1),
                indexed_variables: None,
                expensive: false,
                source: None,
                line: None,
                column: None,
                end_line: None,
                end_column: None,
            }],
        }),
        frame_id => panic!("unexpected frame id {frame_id}"),
    });

    client.on_request::<Variables, _>(move |_, args| match args.variables_reference {
        200 => Err(error_response("variables failed")),
        variables_reference => panic!("unexpected variables reference {variables_reference}"),
    });

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Pause,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: Some(true),
            hit_breakpoint_ids: None,
        }))
        .await;
    cx.run_until_parked();

    let api = agent_api(&project, cx);
    let session_id = session.read_with(cx, |session, _| session.session_id());
    let snapshot = cx
        .update(|cx| {
            api.snapshot(
                session_id,
                AgentDebuggerSnapshotLimits {
                    max_frames: 10,
                    max_variables_per_scope: 10,
                    max_variable_value_length: 1024,
                    max_output_events: 10,
                    max_output_bytes: 1024,
                    max_source_context_lines: 0,
                },
                cx,
            )
        })
        .await
        .unwrap();

    assert_eq!(snapshot.threads.len(), 2);
    assert_eq!(snapshot.threads[0].thread_id.0, 1);
    assert!(snapshot.threads[0].frames.is_empty());
    assert_eq!(snapshot.threads[1].thread_id.0, 2);
    assert_eq!(snapshot.threads[1].frames.len(), 2);
    assert!(snapshot.threads[1].frames[0].scopes.is_empty());
    let scope = &snapshot.threads[1].frames[1].scopes[0];
    assert_eq!(scope.name, "Locals");
    assert!(scope.variables.is_empty());
    assert!(scope.variables_truncated);

    assert!(
        snapshot
            .notes
            .iter()
            .any(|note| note.contains("Stack frames for thread")),
        "expected a stack frame failure note, got {:?}",
        snapshot.notes
    );
    assert!(
        snapshot
            .notes
            .iter()
            .any(|note| note.contains("Scopes for frame")),
        "expected a scopes failure note, got {:?}",
        snapshot.notes
    );
    assert!(
        snapshot
            .notes
            .iter()
            .any(|note| note.contains("Variables for scope")),
        "expected a variables failure note, got {:?}",
        snapshot.notes
    );
}

#[gpui::test]
async fn test_agent_run_to_line_removes_temporary_breakpoint_when_continue_fails(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(
        path!("/project"),
        json!({ "src": { "main.js": "let a = 1;\nlet b = 2;\n" } }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());
    let breakpoint_requests = Arc::new(Mutex::new(Vec::<Vec<u64>>::new()));

    client.on_request::<SetBreakpoints, _>({
        let breakpoint_requests = breakpoint_requests.clone();
        move |_, args| {
            let lines = args
                .breakpoints
                .unwrap_or_default()
                .into_iter()
                .map(|breakpoint| breakpoint.line)
                .collect::<Vec<_>>();
            breakpoint_requests.lock().unwrap().push(lines);
            Ok(dap::SetBreakpointsResponse {
                breakpoints: Vec::default(),
            })
        }
    });

    client.on_request::<Continue, _>(move |_, _| Err(error_response("continue failed")));

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Pause,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;
    cx.run_until_parked();

    let api = agent_api(&project, cx);
    let session_id = session.read_with(cx, |session, _| session.session_id());
    let result = cx
        .update(|cx| {
            api.run_to_line(
                session_id,
                project::debugger::session::ThreadId(1),
                PathBuf::from(path!("/project/src/main.js")),
                2,
                Duration::from_millis(100),
                cx,
            )
        })
        .await;

    assert!(result.is_err());
    let breakpoint_requests = breakpoint_requests.lock().unwrap();
    assert_eq!(&*breakpoint_requests, &[vec![2], Vec::<u64>::new()]);
}

#[gpui::test]
async fn test_start_debug_session_rejects_invalid_explicit_worktree_id(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(path!("/project"), json!({ "main.js": "" }))
        .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let invalid_worktree_id = WorktreeId::from_usize(usize::MAX);

    let error = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.start_debug_session(
                    task::DebugScenario {
                        adapter: "fake-adapter".into(),
                        label: "agent session".into(),
                        build: None,
                        config: json!({ "request": "launch" }),
                        tcp_connection: None,
                    },
                    SharedTaskContext::default(),
                    None,
                    Some(invalid_worktree_id),
                    window,
                    cx,
                )
            })
        })
        .unwrap()
        .unwrap_err();

    assert!(
        error
            .to_string()
            .contains(&format!("Could not find worktree {invalid_worktree_id}")),
        "unexpected error: {error:#}"
    );
}

#[gpui::test]
async fn test_start_debug_session_returns_session_info(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(path!("/project"), json!({ "main.js": "" }))
        .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;

    let _subscription = project::debugger::test::intercept_debug_sessions(cx, |_| {});
    let info = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.start_debug_session(
                    task::DebugScenario {
                        adapter: "fake-adapter".into(),
                        label: "agent session".into(),
                        build: None,
                        config: json!({ "request": "launch" }),
                        tcp_connection: None,
                    },
                    SharedTaskContext::default(),
                    None,
                    None,
                    window,
                    cx,
                )
            })
        })
        .unwrap()
        .unwrap();

    assert_eq!(info.label, "agent session");
    assert_eq!(info.adapter, "fake-adapter");

    cx.run_until_parked();

    // The reported session id refers to a real session in the DAP store.
    let api = agent_api(&project, cx);
    let sessions = cx.update(|cx| api.list_sessions(cx));
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].session_id.to_proto(), info.session_id);
    assert_eq!(sessions[0].label.as_deref(), Some("agent session"));
    assert_eq!(sessions[0].adapter, "fake-adapter");
}
