use dap::{
    StackFrame,
    requests::{Evaluate, Scopes, StackTrace, Threads},
};
use editor::{
    Editor, EditorMode, MultiBuffer, MultiBufferOffset, SelectionEffects,
    actions::Hover as HoverAction,
};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use language::rust_lang;
use project::{FakeFs, Project};
use serde_json::json;
use util::path;

use crate::{
    debugger_panel::DebugPanel,
    tests::{init_test, init_test_workspace, start_debug_session},
};

const SOURCE: &str =
    "fn main() {\n    let value = 42;\n    let x = value + 1;\n    println!(\"{}\", x);\n}\n";

fn hover_source<C: gpui::AppContext>(editor: &gpui::Entity<Editor>, cx: &C) -> Option<String> {
    editor.read_with(cx, |editor, cx| {
        editor
            .hover_state
            .info_popovers
            .first()
            .and_then(|popover| {
                popover.parsed_content.as_ref().map(|markdown| {
                    markdown.read_with(cx, |markdown, _| markdown.source().to_string())
                })
            })
    })
}

fn trigger_hover(editor: &gpui::Entity<Editor>, cx: &mut VisualTestContext, offset: usize) {
    editor.update_in(cx, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selection| {
            selection.select_ranges([MultiBufferOffset(offset)..MultiBufferOffset(offset)])
        });
        editor::hover_popover::hover(editor, &HoverAction, window, cx);
    });
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
    assert!(hover_source(&editor, editor_cx).is_none());

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

    let hover = hover_source(&editor, editor_cx).expect("expected visible hover after stop");
    assert!(
        hover.contains("42"),
        "expected debugger value, got: {hover:?}"
    );
    assert!(
        hover.contains("type: i32"),
        "expected debugger type, got: {hover:?}"
    );
}
