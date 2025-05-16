use std::{path::Path, sync::Arc};

use dap::{Scope, StackFrame, Variable, requests::Variables};
use editor::{Editor, EditorMode, MultiBuffer};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use language::{Language, LanguageConfig, LanguageMatcher, tree_sitter_python, tree_sitter_rust};
use project::{FakeFs, Project};
use serde_json::json;
use unindent::Unindent as _;
use util::path;

use crate::{
    debugger_panel::DebugPanel,
    tests::{active_debug_session_panel, init_test, init_test_workspace, start_debug_session},
};

#[gpui::test]
async fn test_rust_inline_values(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    fn stack_frame_for_line(line: u64) -> dap::StackFrame {
        StackFrame {
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
            line,
            column: 1,
            end_line: None,
            end_column: None,
            can_restart: None,
            instruction_pointer_reference: None,
            module_id: None,
            presentation_hint: None,
        }
    }

    let fs = FakeFs::new(executor.clone());
    let source_code = r#"
static mut GLOBAL: usize = 1;

fn main() {
    let x = 10;
    let value = 42;
    let y = 4;
    let tester = {
        let y = 10;
        let y = 5;
        let b = 3;
        vec![y, 20, 30]
    };

    let caller = || {
        let x = 3;
        println!("x={}", x);
    };

    caller();

    unsafe {
        GLOBAL = 2;
    }

    let result = value * 2 * x;
    println!("Simple test executed: value={}, result={}", value, result);
    assert!(true);
}
"#
    .unindent();
    fs.insert_tree(path!("/project"), json!({ "main.rs": source_code }))
        .await;

    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx);
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(4)],
            total_frames: None,
        })
    });

    client.on_request::<dap::requests::Evaluate, _>(move |_, args| {
        assert_eq!("GLOBAL", args.expression);
        Ok(dap::EvaluateResponse {
            result: "1".into(),
            type_: None,
            presentation_hint: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            value_location_reference: None,
        })
    });

    let local_variables = vec![
        Variable {
            name: "x".into(),
            value: "10".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "y".into(),
            value: "4".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "value".into(),
            value: "42".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];

    client.on_request::<Variables, _>({
        let local_variables = Arc::new(local_variables.clone());
        move |_, _| {
            Ok(dap::VariablesResponse {
                variables: (*local_variables).clone(),
            })
        }
    });

    client.on_request::<dap::requests::Scopes, _>(move |_, _| {
        Ok(dap::ScopesResponse {
            scopes: vec![Scope {
                name: "Locale".into(),
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

    let project_path = Path::new(path!("/project"));
    let worktree = project
        .update(cx, |project, cx| project.find_worktree(project_path, cx))
        .expect("This worktree should exist in project")
        .0;

    let worktree_id = workspace
        .update(cx, |_, _, cx| worktree.read(cx).id())
        .unwrap();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "main.rs"), cx)
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        buffer.set_language(Some(Arc::new(rust_lang())), cx);
    });

    let (editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer, cx),
            Some(project),
            window,
            cx,
        )
    });

    active_debug_session_panel(workspace, cx).update_in(cx, |_, window, cx| {
        cx.focus_self(window);
    });
    cx.run_until_parked();

    editor.update(cx, |editor, cx| editor.refresh_inline_values(cx));

    cx.run_until_parked();

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x = 10;
        let value = 42;
        let y = 4;
        let tester = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(5)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value = 42;
        let y = 4;
        let tester = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(6)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y = 4;
        let tester = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(7)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y: 4 = 4;
        let tester = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(8)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y: 4 = 4;
        let tester = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    let local_variables = vec![
        Variable {
            name: "x".into(),
            value: "10".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "y".into(),
            value: "10".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "value".into(),
            value: "42".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];

    client.on_request::<Variables, _>({
        let local_variables = Arc::new(local_variables.clone());
        move |_, _| {
            Ok(dap::VariablesResponse {
                variables: (*local_variables).clone(),
            })
        }
    });
    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(9)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y = 4;
        let tester = {
            let y: 10 = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    let local_variables = vec![
        Variable {
            name: "x".into(),
            value: "10".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "y".into(),
            value: "5".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "value".into(),
            value: "42".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];

    client.on_request::<Variables, _>({
        let local_variables = Arc::new(local_variables.clone());
        move |_, _| {
            Ok(dap::VariablesResponse {
                variables: (*local_variables).clone(),
            })
        }
    });
    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(10)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y = 4;
        let tester = {
            let y = 10;
            let y: 5 = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    let local_variables = vec![
        Variable {
            name: "x".into(),
            value: "10".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "y".into(),
            value: "5".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "value".into(),
            value: "42".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "b".into(),
            value: "3".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];
    client.on_request::<Variables, _>({
        let local_variables = Arc::new(local_variables.clone());
        move |_, _| {
            Ok(dap::VariablesResponse {
                variables: (*local_variables).clone(),
            })
        }
    });
    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(11)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y = 4;
        let tester = {
            let y = 10;
            let y: 5 = 5;
            let b: 3 = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    let local_variables = vec![
        Variable {
            name: "x".into(),
            value: "10".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "y".into(),
            value: "4".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "value".into(),
            value: "42".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "tester".into(),
            value: "size=3".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];
    client.on_request::<Variables, _>({
        let local_variables = Arc::new(local_variables.clone());
        move |_, _| {
            Ok(dap::VariablesResponse {
                variables: (*local_variables).clone(),
            })
        }
    });
    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(14)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y: 4 = 4;
        let tester: size=3 = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    let local_variables = vec![
        Variable {
            name: "x".into(),
            value: "10".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "y".into(),
            value: "4".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "value".into(),
            value: "42".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "tester".into(),
            value: "size=3".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "caller".into(),
            value: "<not available>".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];
    client.on_request::<Variables, _>({
        let local_variables = Arc::new(local_variables.clone());
        move |_, _| {
            Ok(dap::VariablesResponse {
                variables: (*local_variables).clone(),
            })
        }
    });
    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(19)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y: 4 = 4;
        let tester: size=3 = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller: <not available> = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(15)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x = 10;
        let value = 42;
        let y = 4;
        let tester = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    let local_variables = vec![Variable {
        name: "x".into(),
        value: "3".into(),
        type_: None,
        presentation_hint: None,
        evaluate_name: None,
        variables_reference: 0,
        named_variables: None,
        indexed_variables: None,
        memory_reference: None,
        declaration_location_reference: None,
        value_location_reference: None,
    }];
    client.on_request::<Variables, _>({
        let local_variables = Arc::new(local_variables.clone());
        move |_, _| {
            Ok(dap::VariablesResponse {
                variables: (*local_variables).clone(),
            })
        }
    });
    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(16)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 1: usize = 1;

    fn main() {
        let x = 10;
        let value = 42;
        let y = 4;
        let tester = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller = || {
            let x: 3 = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    let local_variables = vec![
        Variable {
            name: "x".into(),
            value: "10".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "y".into(),
            value: "4".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "value".into(),
            value: "42".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "tester".into(),
            value: "size=3".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "caller".into(),
            value: "<not available>".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];
    client.on_request::<Variables, _>({
        let local_variables = Arc::new(local_variables.clone());
        move |_, _| {
            Ok(dap::VariablesResponse {
                variables: (*local_variables).clone(),
            })
        }
    });
    client.on_request::<dap::requests::Evaluate, _>(move |_, args| {
        assert_eq!("GLOBAL", args.expression);
        Ok(dap::EvaluateResponse {
            result: "2".into(),
            type_: None,
            presentation_hint: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            value_location_reference: None,
        })
    });
    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(25)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 2: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y: 4 = 4;
        let tester: size=3 = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller: <not available> = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });

    let local_variables = vec![
        Variable {
            name: "x".into(),
            value: "10".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "y".into(),
            value: "4".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "value".into(),
            value: "42".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "tester".into(),
            value: "size=3".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "caller".into(),
            value: "<not available>".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "result".into(),
            value: "840".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];
    client.on_request::<Variables, _>({
        let local_variables = Arc::new(local_variables.clone());
        move |_, _| {
            Ok(dap::VariablesResponse {
                variables: (*local_variables).clone(),
            })
        }
    });
    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![stack_frame_for_line(26)],
            total_frames: None,
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
    static mut GLOBAL: 2: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y: 4 = 4;
        let tester: size=3 = {
            let y = 10;
            let y = 5;
            let b = 3;
            vec![y, 20, 30]
        };

        let caller: <not available> = || {
            let x = 3;
            println!("x={}", x);
        };

        caller();

        unsafe {
            GLOBAL = 2;
        }

        let result: 840 = value * 2 * x;
        println!("Simple test executed: value={}, result={}", value, result);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });
}

fn rust_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )
}

#[gpui::test]
async fn test_python_inline_values(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    let source_code = r#"
def process_data(untyped_param, typed_param: int, another_typed: str):
    # Local variables
    x = 10
    result = typed_param * 2
    text = "Hello, " + another_typed

    # For loop with range
    sum_value = 0
    for i in range(5):
        sum_value += i

    # Final result
    final_result = x + result + sum_value
    return final_result
"#
    .unindent();
    fs.insert_tree(path!("/project"), json!({ "main.py": source_code }))
        .await;

    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx);
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let project_path = Path::new(path!("/project"));
    let worktree = project
        .update(cx, |project, cx| project.find_worktree(project_path, cx))
        .expect("This worktree should exist in project")
        .0;

    let worktree_id = workspace
        .update(cx, |_, _, cx| worktree.read(cx).id())
        .unwrap();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "main.py"), cx)
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        buffer.set_language(Some(Arc::new(python_lang())), cx);
    });

    let (editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer, cx),
            Some(project),
            window,
            cx,
        )
    });

    editor.update(cx, |editor, cx| editor.refresh_inline_values(cx));

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<dap::requests::StackTrace, _>(move |_, args| {
        assert_eq!(args.thread_id, 1);
        Ok(dap::StackTraceResponse {
            stack_frames: vec![StackFrame {
                id: 1,
                name: "Stack Frame 1".into(),
                source: Some(dap::Source {
                    name: Some("main.py".into()),
                    path: Some(path!("/project/main.py").into()),
                    source_reference: None,
                    presentation_hint: None,
                    origin: None,
                    sources: None,
                    adapter_data: None,
                    checksums: None,
                }),
                line: 12,
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

    client.on_request::<dap::requests::Scopes, _>(move |_, _| {
        Ok(dap::ScopesResponse {
            scopes: vec![
                Scope {
                    name: "Local".into(),
                    presentation_hint: None,
                    variables_reference: 1,
                    named_variables: None,
                    indexed_variables: None,
                    expensive: false,
                    source: None,
                    line: None,
                    column: None,
                    end_line: None,
                    end_column: None,
                },
                Scope {
                    name: "Global".into(),
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
                },
            ],
        })
    });

    client.on_request::<Variables, _>(move |_, args| match args.variables_reference {
        1 => Ok(dap::VariablesResponse {
            variables: vec![
                Variable {
                    name: "untyped_param".into(),
                    value: "test_value".into(),
                    type_: Some("str".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                },
                Variable {
                    name: "typed_param".into(),
                    value: "42".into(),
                    type_: Some("int".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                },
                Variable {
                    name: "another_typed".into(),
                    value: "world".into(),
                    type_: Some("str".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                },
                Variable {
                    name: "x".into(),
                    value: "10".into(),
                    type_: Some("int".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                },
                Variable {
                    name: "result".into(),
                    value: "84".into(),
                    type_: Some("int".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                },
                Variable {
                    name: "text".into(),
                    value: "Hello, world".into(),
                    type_: Some("str".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                },
                Variable {
                    name: "sum_value".into(),
                    value: "10".into(),
                    type_: Some("int".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                },
                Variable {
                    name: "i".into(),
                    value: "4".into(),
                    type_: Some("int".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                },
                Variable {
                    name: "final_result".into(),
                    value: "104".into(),
                    type_: Some("int".into()),
                    presentation_hint: None,
                    evaluate_name: None,
                    variables_reference: 0,
                    named_variables: None,
                    indexed_variables: None,
                    memory_reference: None,
                    declaration_location_reference: None,
                    value_location_reference: None,
                },
            ],
        }),
        _ => Ok(dap::VariablesResponse { variables: vec![] }),
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

    editor.update_in(cx, |editor, window, cx| {
        pretty_assertions::assert_eq!(
            r#"
        def process_data(untyped_param: test_value, typed_param: 42: int, another_typed: world: str):
            # Local variables
            x: 10 = 10
            result: 84 = typed_param * 2
            text: Hello, world = "Hello, " + another_typed

            # For loop with range
            sum_value: 10 = 0
            for i: 4 in range(5):
                sum_value += i

            # Final result
            final_result = x + result + sum_value
            return final_result
        "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });
}

fn python_lang() -> Language {
    Language::new(
        LanguageConfig {
            name: "Python".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["py".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_python::LANGUAGE.into()),
    )
}
