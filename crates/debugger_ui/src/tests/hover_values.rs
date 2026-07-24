use dap::{
    StackFrame, Variable,
    requests::{Evaluate, Scopes, StackTrace, Threads, Variables},
};
use editor::{
    Editor, EditorMode, MultiBuffer, MultiBufferOffset, SelectionEffects,
    actions::{
        Cancel as CancelAction, DebuggerHoverExpandSelected, DebuggerHoverSelectNext,
        Hover as HoverAction,
    },
};
use gpui::{BackgroundExecutor, Focusable as _, Modifiers, TestAppContext, VisualTestContext};
use language::rust_lang;
use project::{FakeFs, Project};
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use util::path;

use crate::{
    debugger_panel::DebugPanel,
    tests::{init_test, init_test_workspace, start_debug_session},
};

const SOURCE: &str =
    "fn main() {\n    let value = 42;\n    let x = value + 1;\n    println!(\"{}\", x);\n}\n";

fn trigger_hover(editor: &gpui::Entity<Editor>, cx: &mut VisualTestContext, offset: usize) {
    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selection| {
            selection.select_ranges([MultiBufferOffset(offset)..MultiBufferOffset(offset)])
        });
        editor::hover_popover::hover(editor, &HoverAction, window, cx);
    });
}

fn focus_editor(editor: &gpui::Entity<Editor>, cx: &mut VisualTestContext) {
    editor.update_in(cx, |editor, window, cx| {
        window.focus(&editor.focus_handle(cx), cx);
    });
}

fn editor_has_focus(editor: &gpui::Entity<Editor>, cx: &mut VisualTestContext) -> bool {
    cx.update(|window, app| {
        editor.read_with(app, |editor, cx| editor.focus_handle(cx).is_focused(window))
    })
}

fn hover_uses_keyboard_grace<C: gpui::AppContext>(editor: &gpui::Entity<Editor>, cx: &C) -> bool {
    editor.read_with(cx, |editor, _| {
        editor
            .hover_state
            .info_popovers
            .first()
            .is_some_and(|popover| *popover.keyboard_grace.borrow())
    })
}

fn hover_is_visible<C: gpui::AppContext>(editor: &gpui::Entity<Editor>, cx: &C) -> bool {
    editor.read_with(cx, |editor, _| editor.hover_state.visible())
}

fn hover_has_markdown_content<C: gpui::AppContext>(editor: &gpui::Entity<Editor>, cx: &C) -> bool {
    editor.read_with(cx, |editor, _| {
        editor
            .hover_state
            .info_popovers
            .first()
            .is_some_and(|popover| popover.parsed_content.is_some())
    })
}

#[gpui::test]
async fn test_hover_values_after_debugger_stop_when_hover_is_retriggered(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(executor);
    fs.insert_tree(path!("/project"), json!({ "main.rs": SOURCE }))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx)
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

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
    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![StackFrame {
                id: 1,
                name: "Stack Frame 1".into(),
                source: Some(dap::Source {
                    name: Some("main.rs".into()),
                    path: Some(path!("/project/main.rs").into()),
                    source_reference: None,
                    presentation_hint: None,
                    origin: None,
                    sources: None,
                    adapter_data: None,
                    checksums: None,
                }),
                line: 3,
                column: 1,
                end_line: None,
                end_column: None,
                can_restart: None,
                instruction_pointer_reference: None,
                module_id: None,
                presentation_hint: None,
            }],
            total_frames: None,
        })
    });
    client.on_request::<Scopes, _>(move |_, _| Ok(dap::ScopesResponse { scopes: vec![] }));
    client.on_request::<Evaluate, _>(move |_, args| {
        assert_eq!(args.expression, "value");
        assert_eq!(args.context, Some(dap::EvaluateArgumentsContext::Hover));
        Ok(dap::EvaluateResponse {
            result: "42".into(),
            type_: Some("i32".into()),
            presentation_hint: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            value_location_reference: None,
        })
    });

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/project/main.rs"), cx)
        })
        .await
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.set_language(Some(rust_lang()), cx));
    cx.run_until_parked();

    let (editor, editor_cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer.clone(), cx),
            Some(project.clone()),
            window,
            cx,
        )
    });
    editor_cx.run_until_parked();

    let hover_offset = SOURCE.find("value + 1").unwrap();
    trigger_hover(&editor, editor_cx, hover_offset);
    editor_cx.run_until_parked();
    assert!(editor_cx.debug_bounds("debugger-hover-node-root").is_none());

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

    editor_cx.run_until_parked();
    trigger_hover(&editor, editor_cx, hover_offset);
    editor_cx.run_until_parked();

    assert!(
        editor_cx.debug_bounds("debugger-hover-node-root").is_some(),
        "expected visible debugger hover tree after stop"
    );
    assert!(
        editor_cx
            .debug_bounds("debugger-hover-toggle-root")
            .is_none(),
        "expected scalar hover to render without a disclosure toggle"
    );
    assert!(
        !hover_has_markdown_content(&editor, editor_cx),
        "expected debugger hover to render without the old markdown value block"
    );
}

#[gpui::test]
async fn test_hover_values_load_children_on_expand(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(executor);
    let source = format!(
        "fn main() {{\n{}{indent}let value = 42;\n{indent}let x = value + 1;\n}}\n",
        "\n".repeat(20),
        indent = "    "
    );
    fs.insert_tree(path!("/project"), json!({ "main.rs": source }))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx)
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

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
    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![StackFrame {
                id: 1,
                name: "Stack Frame 1".into(),
                source: Some(dap::Source {
                    name: Some("main.rs".into()),
                    path: Some(path!("/project/main.rs").into()),
                    source_reference: None,
                    presentation_hint: None,
                    origin: None,
                    sources: None,
                    adapter_data: None,
                    checksums: None,
                }),
                line: 3,
                column: 1,
                end_line: None,
                end_column: None,
                can_restart: None,
                instruction_pointer_reference: None,
                module_id: None,
                presentation_hint: None,
            }],
            total_frames: None,
        })
    });
    client.on_request::<Scopes, _>(move |_, _| Ok(dap::ScopesResponse { scopes: vec![] }));
    client.on_request::<Evaluate, _>(move |_, args| {
        assert_eq!(args.expression, "value");
        assert_eq!(args.context, Some(dap::EvaluateArgumentsContext::Hover));
        Ok(dap::EvaluateResponse {
            result: "Point { x: 42 }".into(),
            type_: Some("Point".into()),
            presentation_hint: None,
            variables_reference: 1,
            named_variables: Some(1),
            indexed_variables: None,
            memory_reference: None,
            value_location_reference: None,
        })
    });

    let variables_request_count = Arc::new(AtomicUsize::new(0));
    client.on_request::<Variables, _>({
        let variables_request_count = variables_request_count.clone();
        move |_, args| {
            variables_request_count.fetch_add(1, Ordering::SeqCst);
            assert_eq!(args.variables_reference, 1);
            Ok(dap::VariablesResponse {
                variables: vec![Variable {
                    name: "x".into(),
                    value: "42".into(),
                    type_: Some("i32".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                }],
            })
        }
    });

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/project/main.rs"), cx)
        })
        .await
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.set_language(Some(rust_lang()), cx));
    cx.run_until_parked();

    let (editor, editor_cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer.clone(), cx),
            Some(project.clone()),
            window,
            cx,
        )
    });
    editor_cx.run_until_parked();
    focus_editor(&editor, editor_cx);

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

    editor_cx.run_until_parked();
    editor_cx.simulate_resize(gpui::size(gpui::px(320.), gpui::px(110.)));

    let hover_offset = source.find("value + 1").unwrap();
    trigger_hover(&editor, editor_cx, hover_offset);
    editor_cx.run_until_parked();

    assert!(editor_cx.debug_bounds("debugger-hover-node-root").is_some());
    assert!(editor_cx.debug_bounds("debugger-hover-node-0").is_none());
    assert_eq!(variables_request_count.load(Ordering::SeqCst), 0);
    assert!(editor_has_focus(&editor, editor_cx));
    assert!(hover_uses_keyboard_grace(&editor, editor_cx));

    let root_bounds = editor_cx
        .debug_bounds("debugger-hover-node-root")
        .expect("expected root hover row bounds");
    editor_cx.simulate_click(root_bounds.center(), Modifiers::none());
    editor_cx.refresh().unwrap();
    editor_cx.run_until_parked();

    let expanded_root_bounds = editor_cx
        .debug_bounds("debugger-hover-node-root")
        .expect("expected root hover row bounds after expanding");

    assert_eq!(variables_request_count.load(Ordering::SeqCst), 1);
    assert!(editor_cx.debug_bounds("debugger-hover-node-0").is_some());
    assert_eq!(
        root_bounds.origin, expanded_root_bounds.origin,
        "expected debugger hover root row to stay in place when expanding"
    );
    assert!(editor_has_focus(&editor, editor_cx));
    assert!(!hover_uses_keyboard_grace(&editor, editor_cx));

    editor.update_in(editor_cx, |editor, window, cx| {
        editor.cancel(&CancelAction, window, cx);
    });
    editor_cx.refresh().unwrap();
    editor_cx.run_until_parked();

    assert!(!hover_is_visible(&editor, editor_cx));
}

#[gpui::test]
async fn test_hover_values_expand_with_keyboard_action(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(executor);
    fs.insert_tree(path!("/project"), json!({ "main.rs": SOURCE }))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx)
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

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
    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![StackFrame {
                id: 1,
                name: "Stack Frame 1".into(),
                source: Some(dap::Source {
                    name: Some("main.rs".into()),
                    path: Some(path!("/project/main.rs").into()),
                    source_reference: None,
                    presentation_hint: None,
                    origin: None,
                    sources: None,
                    adapter_data: None,
                    checksums: None,
                }),
                line: 3,
                column: 1,
                end_line: None,
                end_column: None,
                can_restart: None,
                instruction_pointer_reference: None,
                module_id: None,
                presentation_hint: None,
            }],
            total_frames: None,
        })
    });
    client.on_request::<Scopes, _>(move |_, _| Ok(dap::ScopesResponse { scopes: vec![] }));
    client.on_request::<Evaluate, _>(move |_, args| {
        assert_eq!(args.expression, "value");
        assert_eq!(args.context, Some(dap::EvaluateArgumentsContext::Hover));
        Ok(dap::EvaluateResponse {
            result: "Point { x: 42 }".into(),
            type_: Some("Point".into()),
            presentation_hint: None,
            variables_reference: 1,
            named_variables: Some(1),
            indexed_variables: None,
            memory_reference: None,
            value_location_reference: None,
        })
    });
    client.on_request::<Variables, _>(move |_, args| {
        assert_eq!(args.variables_reference, 1);
        Ok(dap::VariablesResponse {
            variables: vec![Variable {
                name: "x".into(),
                value: "42".into(),
                type_: Some("i32".into()),
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
                declaration_location_reference: None,
                value_location_reference: None,
            }],
        })
    });

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/project/main.rs"), cx)
        })
        .await
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.set_language(Some(rust_lang()), cx));
    cx.run_until_parked();

    let (editor, editor_cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer.clone(), cx),
            Some(project.clone()),
            window,
            cx,
        )
    });
    editor_cx.run_until_parked();
    focus_editor(&editor, editor_cx);

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

    editor_cx.run_until_parked();
    let hover_offset = SOURCE.find("value + 1").unwrap();
    trigger_hover(&editor, editor_cx, hover_offset);
    editor_cx.run_until_parked();

    assert!(hover_uses_keyboard_grace(&editor, editor_cx));
    assert!(editor_cx.debug_bounds("debugger-hover-node-0").is_none());

    editor_cx.dispatch_action(DebuggerHoverExpandSelected);
    editor_cx.run_until_parked();

    assert!(editor_cx.debug_bounds("debugger-hover-node-0").is_some());
}

#[gpui::test]
async fn test_hover_values_keyboard_selection_scrolls_into_view(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(executor);
    fs.insert_tree(path!("/project"), json!({ "main.rs": SOURCE }))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx)
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

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
    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![StackFrame {
                id: 1,
                name: "Stack Frame 1".into(),
                source: Some(dap::Source {
                    name: Some("main.rs".into()),
                    path: Some(path!("/project/main.rs").into()),
                    source_reference: None,
                    presentation_hint: None,
                    origin: None,
                    sources: None,
                    adapter_data: None,
                    checksums: None,
                }),
                line: 3,
                column: 1,
                end_line: None,
                end_column: None,
                can_restart: None,
                instruction_pointer_reference: None,
                module_id: None,
                presentation_hint: None,
            }],
            total_frames: None,
        })
    });
    client.on_request::<Scopes, _>(move |_, _| Ok(dap::ScopesResponse { scopes: vec![] }));
    client.on_request::<Evaluate, _>(move |_, args| {
        assert_eq!(args.expression, "value");
        assert_eq!(args.context, Some(dap::EvaluateArgumentsContext::Hover));
        Ok(dap::EvaluateResponse {
            result: "Point { x: 42 }".into(),
            type_: Some("Point".into()),
            presentation_hint: None,
            variables_reference: 1,
            named_variables: Some(10),
            indexed_variables: None,
            memory_reference: None,
            value_location_reference: None,
        })
    });
    client.on_request::<Variables, _>(move |_, args| {
        assert_eq!(args.variables_reference, 1);
        Ok(dap::VariablesResponse {
            variables: (0..10)
                .map(|index| Variable {
                    name: format!("field_{index}"),
                    value: format!("{index}"),
                    type_: Some("i32".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                })
                .collect(),
        })
    });

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/project/main.rs"), cx)
        })
        .await
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.set_language(Some(rust_lang()), cx));
    cx.run_until_parked();

    let (editor, editor_cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer.clone(), cx),
            Some(project.clone()),
            window,
            cx,
        )
    });
    editor_cx.run_until_parked();
    focus_editor(&editor, editor_cx);
    editor_cx.simulate_resize(gpui::size(gpui::px(320.), gpui::px(110.)));

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

    editor_cx.run_until_parked();
    let hover_offset = SOURCE.find("value + 1").unwrap();
    trigger_hover(&editor, editor_cx, hover_offset);
    editor_cx.run_until_parked();
    editor_cx.dispatch_action(DebuggerHoverExpandSelected);
    editor_cx.run_until_parked();

    let initial_scroll_offset = editor.update(editor_cx, |editor, _| {
        editor.hover_state.info_popovers[0].scroll_handle.offset()
    });

    for _ in 0..8 {
        editor_cx.dispatch_action(DebuggerHoverSelectNext);
        editor_cx.run_until_parked();
    }

    let final_scroll_offset = editor.update(editor_cx, |editor, _| {
        editor.hover_state.info_popovers[0].scroll_handle.offset()
    });

    assert!(
        final_scroll_offset.y < initial_scroll_offset.y,
        "expected debugger hover to scroll downward as selection moves out of view"
    );
}
