#![expect(clippy::result_large_err)]

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

#[gpui::test]
async fn test_hover_horizontal_scrollbar_drag_is_stable(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    const SOURCE: &str = "fn main() {\n    let a = 0;\n    let b = vec![\"heyy\"];\n    println!(\"{a} {b:?}\");\n}\n";
    const VECTOR_TYPE: &str = "alloc::vec::Vec<&str, alloc::alloc::Global>";
    init_test(cx);
    let fs = FakeFs::new(executor);
    fs.insert_tree(path!("/project"), json!({ "main.rs": SOURCE }))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx);
        })
        .unwrap();
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());
    client.on_request::<Threads, _>(|_, _| {
        Ok(
            serde_json::from_value(json!({"threads": [{"id": 1, "name": "main"}]}))
                .expect("threads fixture"),
        )
    });
    client.on_request::<StackTrace, _>(|_, _| {
        Ok(serde_json::from_value(json!({
            "stackFrames": [{"id": 1, "name": "main", "line": 4, "column": 1,
                "source": {"name": "main.rs", "path": path!("/project/main.rs")}}]
        }))
        .expect("stack fixture"))
    });
    client.on_request::<Scopes, _>(|_, _| Ok(dap::ScopesResponse { scopes: vec![] }));
    client.on_request::<Evaluate, _>(|_, args| {
        assert_eq!(args.expression, "b");
        Ok(serde_json::from_value(json!({
            "result": "size=1", "type": VECTOR_TYPE, "variablesReference": 1
        }))
        .expect("evaluate fixture"))
    });
    client.on_request::<Variables, _>(|_, args| {
        let variables = match args.variables_reference {
            1 => json!([
                {"name": "[0]", "value": "\"heyy\"", "variablesReference": 2},
                {"name": "[raw]", "value": VECTOR_TYPE, "variablesReference": 3}
            ]),
            3 => json!([
                {"name": "buf", "value": "{inner:{ptr:{pointer:\"\\xd0c\\U00000003\"}, _marker:{}}, cap:{0:1}, alloc:{}}", "variablesReference": 4},
                {"name": "len", "value": "1", "variablesReference": 0}
            ]),
            reference => panic!("unexpected variables reference {reference}"),
        };
        Ok(serde_json::from_value(json!({"variables": variables})).expect("variables fixture"))
    });
    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(path!("/project/main.rs"), cx)
        })
        .await
        .unwrap();
    buffer.update(cx, |buffer, cx| buffer.set_language(Some(rust_lang()), cx));
    let (editor, editor_cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer, cx),
            Some(project),
            window,
            cx,
        )
    });
    editor_cx.simulate_resize(gpui::size(gpui::px(900.), gpui::px(582.)));
    focus_editor(&editor, editor_cx);
    client
        .fake_event(dap::messages::Events::Stopped(
            serde_json::from_value(json!({
                "reason": "breakpoint", "threadId": 1, "allThreadsStopped": true
            }))
            .unwrap(),
        ))
        .await;
    editor_cx.run_until_parked();
    trigger_hover(&editor, editor_cx, SOURCE.find("b =").unwrap());
    editor_cx.run_until_parked();
    let toggle = editor_cx
        .debug_bounds("debugger-hover-toggle-1")
        .expect("raw disclosure");
    editor_cx.simulate_click(toggle.center(), Modifiers::none());
    editor_cx.run_until_parked();
    assert!(editor_cx.debug_bounds("debugger-hover-node-1-0").is_some());
    let handle = editor.read_with(editor_cx, |editor, _| {
        editor.hover_state.info_popovers[0].scroll_handle.clone()
    });
    let viewport = handle.bounds();
    let max_offset = handle.max_offset();
    assert!(max_offset.x > gpui::px(100.));
    let start = gpui::point(
        viewport.left() + gpui::px(40.),
        viewport.bottom() - gpui::px(7.),
    );
    editor_cx.simulate_mouse_move(start, None, Modifiers::none());
    editor_cx.simulate_mouse_down(start, gpui::MouseButton::Left, Modifiers::none());
    let mut position = start + gpui::point(gpui::px(60.), gpui::px(0.));
    editor_cx.simulate_mouse_move(position, Some(gpui::MouseButton::Left), Modifiers::none());
    let mut previous_offset = handle.offset();
    assert!(previous_offset.x < gpui::px(0.));
    let pixels_per_pointer_pixel = -previous_offset.x / gpui::px(60.);
    for step in 0..20 {
        position.x += gpui::px(1.);
        editor_cx.simulate_mouse_move(position, Some(gpui::MouseButton::Left), Modifiers::none());
        let offset = handle.offset();
        assert_eq!(handle.bounds(), viewport);
        assert_eq!(handle.max_offset(), max_offset);
        assert!(
            (offset.x - previous_offset.x + gpui::px(pixels_per_pointer_pixel)).abs()
                < gpui::px(0.1),
            "drag step {step}: a one-pixel pointer move must not jump the content: {previous_offset:?} -> {offset:?}, expected {} pixels",
            pixels_per_pointer_pixel
        );
        previous_offset = offset;
    }
    editor_cx.simulate_mouse_up(position, gpui::MouseButton::Left, Modifiers::none());
    handle.set_offset(gpui::point(-max_offset.x, gpui::px(0.)));
    editor_cx.update(|window, cx| window.draw(cx).clear(cx));
    let value_bounds = editor_cx
        .debug_bounds("debugger-hover-node-1-0-value-container")
        .expect("expanded buffer value");
    assert!(
        value_bounds.right() <= viewport.right(),
        "the full value must be reachable at the horizontal scroll limit: {value_bounds:?}, viewport {viewport:?}"
    );
}

fn open_variable_menu(editor_cx: &mut VisualTestContext, selector: &'static str) {
    let bounds = editor_cx
        .debug_bounds(selector)
        .expect("expected variable row");
    editor_cx.simulate_mouse_down(bounds.center(), gpui::MouseButton::Right, Modifiers::none());
    editor_cx.simulate_mouse_up(bounds.center(), gpui::MouseButton::Right, Modifiers::none());
    for _ in 0..3 {
        editor_cx.refresh().unwrap();
        editor_cx.update(|window, cx| {
            window.simulate_next_frame(cx);
        });
        editor_cx.run_until_parked();
    }
    assert!(
        editor_cx.debug_bounds("MENU_ITEM-Copy Value").is_some(),
        "expected visible variable menu"
    );
}

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
    const VALUE: &str = "first line\nsecond line\nthird line with a long value that must remain fully readable and copyable without truncation";
    let watch_requests = Arc::new(AtomicUsize::new(0));
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
    client.on_request::<Evaluate, _>({
        let watch_requests = watch_requests.clone();
        move |_, args| {
            assert_eq!(args.expression, "value");
            if args.context == Some(dap::EvaluateArgumentsContext::Watch) {
                watch_requests.fetch_add(1, Ordering::SeqCst);
            } else {
                assert_eq!(args.context, Some(dap::EvaluateArgumentsContext::Hover));
            }
            Ok(dap::EvaluateResponse {
                result: VALUE.into(),
                type_: Some("i32".into()),
                presentation_hint: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
                value_location_reference: None,
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
    let root_bounds = editor_cx
        .debug_bounds("debugger-hover-node-root")
        .expect("expected debugger hover root bounds");
    assert!(
        root_bounds.size.width >= gpui::px(280.),
        "expected a readable hover width after reserving its scrollbar track, got {:?}",
        root_bounds.size.width
    );
    let value_bounds = editor_cx
        .debug_bounds("debugger-hover-node-root-value-container")
        .expect("expected debugger hover value bounds");
    assert!(
        value_bounds.size.width >= gpui::px(48.),
        "expected debugger hover to keep short values visible"
    );
    assert!(
        root_bounds.size.height > gpui::px(40.),
        "multiline values must not be flattened to one line"
    );
    focus_editor(&editor, editor_cx);
    editor_cx.update(|_, cx| {
        let bindings = settings::KeymapFile::load_asset_allow_partial_failure(
            settings::DEFAULT_KEYMAP_PATH,
            cx,
        )
        .unwrap();
        cx.bind_keys(bindings);
    });

    open_variable_menu(editor_cx, "debugger-hover-node-root");
    assert!(
        hover_is_visible(&editor, editor_cx),
        "focusing the menu must preserve the hover"
    );
    assert!(
        !editor_has_focus(&editor, editor_cx),
        "menu must receive keyboard focus"
    );
    editor_cx.simulate_keystrokes("enter");
    editor_cx.run_until_parked();
    assert_eq!(
        editor_cx.read_from_clipboard().unwrap().text().as_deref(),
        Some(VALUE)
    );

    open_variable_menu(editor_cx, "debugger-hover-node-root");
    editor_cx.simulate_keystrokes("down enter");
    editor_cx.run_until_parked();
    assert_eq!(
        editor_cx.read_from_clipboard().unwrap().text().as_deref(),
        Some("value")
    );

    open_variable_menu(editor_cx, "debugger-hover-node-root");
    editor_cx.simulate_keystrokes("down down enter");
    editor_cx.run_until_parked();
    assert_eq!(watch_requests.load(Ordering::SeqCst), 1);
    session.read_with(editor_cx, |session, _| {
        assert_eq!(
            session.watchers().get("value").unwrap().value.as_ref(),
            VALUE
        );
    });
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
            assert!(matches!(args.variables_reference, 1 | 2));
            Ok(dap::VariablesResponse {
                variables: vec![Variable {
                    name: if args.variables_reference == 1 {
                        "x"
                    } else {
                        "y"
                    }
                    .into(),
                    value: "42".into(),
                    type_: Some("i32".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: if args.variables_reference == 1 { 2 } else { 0 },
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
    assert!(editor_cx.debug_bounds("debugger-hover-node-0").is_some());
    assert_eq!(variables_request_count.load(Ordering::SeqCst), 1);
    assert!(editor_has_focus(&editor, editor_cx));
    assert!(hover_uses_keyboard_grace(&editor, editor_cx));

    let root_bounds = editor_cx
        .debug_bounds("debugger-hover-node-root")
        .expect("expected root hover row bounds");
    editor_cx.simulate_click(root_bounds.center(), Modifiers::none());
    editor_cx.refresh().unwrap();
    editor_cx.run_until_parked();
    assert!(editor_cx.debug_bounds("debugger-hover-node-0").is_none());
    let collapsed_bounds = editor_cx.debug_bounds("debugger-hover-node-root").unwrap();
    editor_cx.simulate_click(collapsed_bounds.center(), Modifiers::none());
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

    assert!(editor_cx.debug_bounds("debugger-hover-node-0-0").is_none());
    let toggle = editor_cx.debug_bounds("debugger-hover-toggle-0").unwrap();
    editor_cx.simulate_click(toggle.center(), Modifiers::none());
    editor_cx.run_until_parked();
    assert!(editor_cx.debug_bounds("debugger-hover-node-0-0").is_some());
    assert_eq!(
        variables_request_count.load(Ordering::SeqCst),
        2,
        "only the first layer should load automatically"
    );

    editor.update_in(editor_cx, |editor, window, cx| {
        editor.cancel(&CancelAction, window, cx);
    });
    editor_cx.refresh().unwrap();
    editor_cx.run_until_parked();

    assert!(!hover_is_visible(&editor, editor_cx));
}

#[gpui::test]
async fn test_hover_values_default_keymaps(executor: BackgroundExecutor, cx: &mut TestAppContext) {
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
    for keymap in [
        "keymaps/default-macos.json",
        "keymaps/default-linux.json",
        "keymaps/default-windows.json",
    ] {
        editor_cx.update(|_, cx| {
            cx.clear_key_bindings();
            let bindings = settings::KeymapFile::load_asset_allow_partial_failure(keymap, cx)
                .expect("expected default keymap");
            cx.bind_keys(bindings);
        });
        trigger_hover(&editor, editor_cx, hover_offset);
        editor_cx.run_until_parked();
        let selection_head = editor.read_with(editor_cx, |editor, _| {
            editor.selections.newest_anchor().head()
        });

        assert!(hover_uses_keyboard_grace(&editor, editor_cx));
        assert!(editor_cx.debug_bounds("debugger-hover-node-0").is_some());
        editor_cx.simulate_keystrokes("left");
        editor_cx.run_until_parked();
        assert!(editor_cx.debug_bounds("debugger-hover-node-0").is_none());

        editor_cx.simulate_keystrokes("right");
        editor_cx.run_until_parked();
        assert!(
            editor_cx.debug_bounds("debugger-hover-node-0").is_some(),
            "right should expand the hover with {keymap}"
        );

        editor_cx.simulate_keystrokes("down left left");
        editor_cx.run_until_parked();
        assert!(editor_cx.debug_bounds("debugger-hover-node-0").is_none());

        editor_cx.simulate_keystrokes("right down up left");
        editor_cx.run_until_parked();
        assert!(editor_cx.debug_bounds("debugger-hover-node-0").is_none());
        editor.read_with(editor_cx, |editor, _| {
            assert_eq!(
                editor.selections.newest_anchor().head(),
                selection_head,
                "hover navigation must not move the editor cursor"
            );
        });

        editor_cx.simulate_keystrokes("escape");
        editor_cx.run_until_parked();
        assert!(!hover_is_visible(&editor, editor_cx));
        assert!(editor_has_focus(&editor, editor_cx));
        editor_cx.simulate_keystrokes("right");
        editor.read_with(editor_cx, |editor, _| {
            assert_ne!(
                editor.selections.newest_anchor().head(),
                selection_head,
                "closing the hover must restore normal cursor navigation"
            );
        });
    }
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
                    value: format!("{index}: {}", "long value ".repeat(30)),
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

    for viewport_size in [
        gpui::size(gpui::px(320.), gpui::px(110.)),
        gpui::size(gpui::px(180.), gpui::px(90.)),
    ] {
        editor_cx.simulate_resize(viewport_size);
        editor_cx.run_until_parked();
        editor.read_with(editor_cx, |editor, _| {
            let bounds = editor.hover_state.info_popovers[0]
                .last_bounds
                .get()
                .expect("expected hover bounds");
            assert!(
                bounds.is_contained_within(&gpui::Bounds::new(
                    gpui::point(gpui::px(0.), gpui::px(0.)),
                    viewport_size
                )),
                "expanded hover {bounds:?} must fit inside {viewport_size:?}"
            );
        });
    }

    let scroll_handle = editor.read_with(editor_cx, |editor, _| {
        editor.hover_state.info_popovers[0].scroll_handle.clone()
    });
    let viewport = scroll_handle.bounds();
    let max_offset = scroll_handle.max_offset();
    let position = viewport.center();
    for delta in [
        gpui::point(gpui::px(-30.), gpui::px(0.)),
        gpui::point(gpui::px(0.), gpui::px(-15.)),
        gpui::point(gpui::px(20.), gpui::px(0.)),
        gpui::point(gpui::px(0.), gpui::px(10.)),
    ] {
        let before = scroll_handle.offset();
        editor_cx.simulate_event(gpui::ScrollEvent {
            position,
            delta: gpui::ScrollDelta::Pixels(delta),
            modifiers: Modifiers::none(),
            touch_phase: gpui::TouchPhase::Moved,
            source: gpui::ScrollSource::Pointer,
        });
        editor_cx.run_until_parked();
        assert_eq!(scroll_handle.bounds(), viewport);
        assert_eq!(scroll_handle.max_offset(), max_offset);
        assert_eq!(scroll_handle.offset(), before + delta);
    }
    scroll_handle.set_offset(gpui::Point::default());
    editor_cx.refresh().unwrap();
    editor_cx.run_until_parked();

    let initial_scroll_offset = editor.update(editor_cx, |editor, _| {
        assert!(
            editor.hover_state.info_popovers[0]
                .scroll_handle
                .max_offset()
                .x
                > gpui::px(0.),
            "long child values must be available through horizontal scrolling"
        );
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
