use std::{path::Path, sync::Arc};

use dap::{Scope, StackFrame, Variable, requests::Variables};
use editor::{Editor, EditorMode, MultiBuffer};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use language::{Language, LanguageConfig, LanguageMatcher, tree_sitter_python, tree_sitter_rust};
use project::{FakeFs, Project};
use serde_json::json;
use unindent::Unindent as _;
use util::{path, rel_path::rel_path};

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
            project.open_buffer((worktree_id, rel_path("main.rs")), cx)
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
    static mut GLOBAL: usize = 1;

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
    static mut GLOBAL: usize = 1;

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
    static mut GLOBAL: usize = 1;

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
    static mut GLOBAL: usize = 1;

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
    static mut GLOBAL: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y: 4 = 4;
        let tester = {
            let y: 4 = 10;
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
    static mut GLOBAL: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y: 10 = 4;
        let tester = {
            let y: 10 = 10;
            let y: 10 = 5;
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
    static mut GLOBAL: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y: 5 = 4;
        let tester = {
            let y: 5 = 10;
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
    static mut GLOBAL: usize = 1;

    fn main() {
        let x: 10 = 10;
        let value: 42 = 42;
        let y: 5 = 4;
        let tester = {
            let y: 5 = 10;
            let y: 5 = 5;
            let b: 3 = 3;
            vec![y: 5, 20, 30]
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
    static mut GLOBAL: usize = 1;

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
    static mut GLOBAL: usize = 1;

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
    static mut GLOBAL: usize = 1;

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
            let x: 10 = 3;
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
    static mut GLOBAL: usize = 1;

    fn main() {
        let x: 3 = 10;
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
            println!("x={}", x: 3);
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
    static mut GLOBAL: usize = 1;

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

        let result = value: 42 * 2 * x: 10;
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
    static mut GLOBAL: usize = 1;

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

        let result: 840 = value: 42 * 2 * x: 10;
        println!("Simple test executed: value={}, result={}", value: 42, result: 840);
        assert!(true);
    }
    "#
            .unindent(),
            editor.snapshot(window, cx).text()
        );
    });
}

fn rust_lang() -> Language {
    let debug_variables_query = include_str!("../../../languages/src/rust/debugger.scm");
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
    .with_debug_variables_query(debug_variables_query)
    .unwrap()
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
            project.open_buffer((worktree_id, rel_path("main.py")), cx)
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
            result: 84 = typed_param: 42 * 2
            text: Hello, world = "Hello, " + another_typed: world

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
    let debug_variables_query = include_str!("../../../languages/src/python/debugger.scm");
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
    .with_debug_variables_query(debug_variables_query)
    .unwrap()
}

fn go_lang() -> Language {
    let debug_variables_query = include_str!("../../../languages/src/go/debugger.scm");
    Language::new(
        LanguageConfig {
            name: "Go".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["go".to_string()],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_go::LANGUAGE.into()),
    )
    .with_debug_variables_query(debug_variables_query)
    .unwrap()
}

/// Test utility function for inline values testing
///
/// # Arguments
/// * `variables` - List of tuples containing (variable_name, variable_value)
/// * `before` - Source code before inline values are applied
/// * `after` - Expected source code after inline values are applied
/// * `language` - Language configuration to use for parsing
/// * `executor` - Background executor for async operations
/// * `cx` - Test app context
async fn test_inline_values_util(
    local_variables: &[(&str, &str)],
    global_variables: &[(&str, &str)],
    before: &str,
    after: &str,
    active_debug_line: Option<usize>,
    language: Language,
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let lines_count = before.lines().count();
    let stop_line =
        active_debug_line.unwrap_or_else(|| if lines_count > 6 { 6 } else { lines_count - 1 });

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(path!("/project"), json!({ "main.rs": before.to_string() }))
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

    client.on_request::<dap::requests::Threads, _>(|_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "main".into(),
            }],
        })
    });

    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![dap::StackFrame {
                id: 1,
                name: "main".into(),
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
                line: stop_line as u64,
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

    let local_vars: Vec<Variable> = local_variables
        .iter()
        .map(|(name, value)| Variable {
            name: (*name).into(),
            value: (*value).into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        })
        .collect();

    let global_vars: Vec<Variable> = global_variables
        .iter()
        .map(|(name, value)| Variable {
            name: (*name).into(),
            value: (*value).into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        })
        .collect();

    client.on_request::<Variables, _>({
        let local_vars = Arc::new(local_vars.clone());
        let global_vars = Arc::new(global_vars.clone());
        move |_, args| {
            let variables = match args.variables_reference {
                2 => (*local_vars).clone(),
                3 => (*global_vars).clone(),
                _ => vec![],
            };
            Ok(dap::VariablesResponse { variables })
        }
    });

    client.on_request::<dap::requests::Scopes, _>(move |_, _| {
        Ok(dap::ScopesResponse {
            scopes: vec![
                Scope {
                    name: "Local".into(),
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
                Scope {
                    name: "Global".into(),
                    presentation_hint: None,
                    variables_reference: 3,
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

    if !global_variables.is_empty() {
        let global_evaluate_map: std::collections::HashMap<String, String> = global_variables
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();

        client.on_request::<dap::requests::Evaluate, _>(move |_, args| {
            let value = global_evaluate_map
                .get(&args.expression)
                .unwrap_or(&"undefined".to_string())
                .clone();

            Ok(dap::EvaluateResponse {
                result: value,
                type_: None,
                presentation_hint: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
                value_location_reference: None,
            })
        });
    }

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
            project.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();

    buffer.update(cx, |buffer, cx| {
        buffer.set_language(Some(Arc::new(language)), cx);
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
        pretty_assertions::assert_eq!(after, editor.snapshot(window, cx).text());
    });
}

#[gpui::test]
async fn test_inline_values_example(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    let variables = [("x", "10"), ("y", "20"), ("result", "30")];

    let before = r#"
fn main() {
    let x = 10;
    let y = 20;
    let result = x + y;
    println!("Result: {}", result);
}
"#
    .unindent();

    let after = r#"
fn main() {
    let x: 10 = 10;
    let y: 20 = 20;
    let result: 30 = x: 10 + y: 20;
    println!("Result: {}", result: 30);
}
"#
    .unindent();

    test_inline_values_util(
        &variables,
        &[],
        &before,
        &after,
        None,
        rust_lang(),
        executor,
        cx,
    )
    .await;
}

#[gpui::test]
async fn test_inline_values_with_globals(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    let variables = [("x", "5"), ("y", "10")];

    let before = r#"
static mut GLOBAL_COUNTER: usize = 42;

fn main() {
    let x = 5;
    let y = 10;
    unsafe {
        GLOBAL_COUNTER += 1;
    }
    println!("x={}, y={}, global={}", x, y, unsafe { GLOBAL_COUNTER });
}
"#
    .unindent();

    let after = r#"
static mut GLOBAL_COUNTER: 42: usize = 42;

fn main() {
    let x: 5 = 5;
    let y: 10 = 10;
    unsafe {
        GLOBAL_COUNTER += 1;
    }
    println!("x={}, y={}, global={}", x, y, unsafe { GLOBAL_COUNTER });
}
"#
    .unindent();

    test_inline_values_util(
        &variables,
        &[("GLOBAL_COUNTER", "42")],
        &before,
        &after,
        None,
        rust_lang(),
        executor,
        cx,
    )
    .await;
}

#[gpui::test]
async fn test_go_inline_values(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    let variables = [("x", "42"), ("y", "hello")];

    let before = r#"
package main

var globalCounter int = 100

func main() {
    x := 42
    y := "hello"
    z := x + 10
    println(x, y, z)
}
"#
    .unindent();

    let after = r#"
package main

var globalCounter: 100 int = 100

func main() {
    x: 42 := 42
    y := "hello"
    z := x + 10
    println(x, y, z)
}
"#
    .unindent();

    test_inline_values_util(
        &variables,
        &[("globalCounter", "100")],
        &before,
        &after,
        None,
        go_lang(),
        executor,
        cx,
    )
    .await;
}

#[gpui::test]
async fn test_trim_multi_line_inline_value(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    let variables = [("y", "hello\n world")];

    let before = r#"
fn main() {
    let y = "hello\n world";
}
"#
    .unindent();

    let after = r#"
fn main() {
    let y: hello… = "hello\n world";
}
"#
    .unindent();

    test_inline_values_util(
        &variables,
        &[],
        &before,
        &after,
        None,
        rust_lang(),
        executor,
        cx,
    )
    .await;
}
