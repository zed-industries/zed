use crate::{
    debugger_panel::DebugPanel,
    session::ThreadItem,
    tests::{active_debug_session_panel, init_test, init_test_workspace},
};
use dap::{
    requests::{Modules, StackTrace, Threads},
    StoppedEvent,
};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{FakeFs, Project};
use std::sync::{
    atomic::{AtomicBool, AtomicI32, Ordering},
    Arc,
};
use task::LaunchConfig;

#[gpui::test]
async fn test_module_list(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx);
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.fake_debug_session(
            dap::DebugRequestType::Launch(LaunchConfig::default()),
            Some(dap::Capabilities {
                supports_modules_request: Some(true),
                ..Default::default()
            }),
            false,
            cx,
        )
    });

    let session = task.await.unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .on_request::<StackTrace, _>(move |_, args| {
            assert!(args.thread_id == 1);
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::default(),
                total_frames: None,
            })
        })
        .await;

    let called_modules = Arc::new(AtomicBool::new(false));
    let modules = vec![
        dap::Module {
            id: dap::ModuleId::Number(1),
            name: "First Module".into(),
            address_range: None,
            date_time_stamp: None,
            path: None,
            symbol_file_path: None,
            symbol_status: None,
            version: None,
            is_optimized: None,
            is_user_code: None,
        },
        dap::Module {
            id: dap::ModuleId::Number(2),
            name: "Second Module".into(),
            address_range: None,
            date_time_stamp: None,
            path: None,
            symbol_file_path: None,
            symbol_status: None,
            version: None,
            is_optimized: None,
            is_user_code: None,
        },
    ];

    client
        .on_request::<Threads, _>(move |_, _| {
            Ok(dap::ThreadsResponse {
                threads: vec![dap::Thread {
                    id: 1,
                    name: "Thread 1".into(),
                }],
            })
        })
        .await;

    client
        .on_request::<Modules, _>({
            let called_modules = called_modules.clone();
            let modules_request_count = AtomicI32::new(0);
            let modules = modules.clone();
            move |_, _| {
                modules_request_count.fetch_add(1, Ordering::SeqCst);
                assert_eq!(
                    1,
                    modules_request_count.load(Ordering::SeqCst),
                    "This request should only be called once from the host"
                );
                called_modules.store(true, Ordering::SeqCst);

                Ok(dap::ModulesResponse {
                    modules: modules.clone(),
                    total_modules: Some(2u64),
                })
            }
        })
        .await;

    client
        .fake_event(dap::messages::Events::Stopped(StoppedEvent {
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

    let running_state =
        active_debug_session_panel(workspace, cx).update_in(cx, |item, window, cx| {
            cx.focus_self(window);
            item.mode()
                .as_running()
                .expect("Session should be running by this point")
                .clone()
        });

    assert!(
        !called_modules.load(std::sync::atomic::Ordering::SeqCst),
        "Request Modules shouldn't be called before it's needed"
    );

    running_state.update(cx, |state, cx| {
        state.set_thread_item(ThreadItem::Modules, cx);
        cx.refresh_windows();
    });

    cx.run_until_parked();

    assert!(
        called_modules.load(std::sync::atomic::Ordering::SeqCst),
        "Request Modules should be called because a user clicked on the module list"
    );

    active_debug_session_panel(workspace, cx).update(cx, |_, cx| {
        running_state.update(cx, |state, cx| {
            state.set_thread_item(ThreadItem::Modules, cx)
        });
        let actual_modules = running_state.update(cx, |state, cx| {
            state.module_list().update(cx, |list, cx| list.modules(cx))
        });

        assert_eq!(modules, actual_modules);
    });

    // Test all module events now
    // New Module
    // Changed
    // Removed

    let new_module = dap::Module {
        id: dap::ModuleId::Number(3),
        name: "Third Module".into(),
        address_range: None,
        date_time_stamp: None,
        path: None,
        symbol_file_path: None,
        symbol_status: None,
        version: None,
        is_optimized: None,
        is_user_code: None,
    };

    client
        .fake_event(dap::messages::Events::Module(dap::ModuleEvent {
            reason: dap::ModuleEventReason::New,
            module: new_module.clone(),
        }))
        .await;

    cx.run_until_parked();

    active_debug_session_panel(workspace, cx).update(cx, |_, cx| {
        let actual_modules = running_state.update(cx, |state, cx| {
            state.module_list().update(cx, |list, cx| list.modules(cx))
        });
        assert_eq!(actual_modules.len(), 3);
        assert!(actual_modules.contains(&new_module));
    });

    let changed_module = dap::Module {
        id: dap::ModuleId::Number(2),
        name: "Modified Second Module".into(),
        address_range: None,
        date_time_stamp: None,
        path: None,
        symbol_file_path: None,
        symbol_status: None,
        version: None,
        is_optimized: None,
        is_user_code: None,
    };

    client
        .fake_event(dap::messages::Events::Module(dap::ModuleEvent {
            reason: dap::ModuleEventReason::Changed,
            module: changed_module.clone(),
        }))
        .await;

    cx.run_until_parked();

    active_debug_session_panel(workspace, cx).update(cx, |_, cx| {
        let actual_modules = running_state.update(cx, |state, cx| {
            state.module_list().update(cx, |list, cx| list.modules(cx))
        });

        assert_eq!(actual_modules.len(), 3);
        assert!(actual_modules.contains(&changed_module));
    });

    client
        .fake_event(dap::messages::Events::Module(dap::ModuleEvent {
            reason: dap::ModuleEventReason::Removed,
            module: changed_module.clone(),
        }))
        .await;

    cx.run_until_parked();

    active_debug_session_panel(workspace, cx).update(cx, |_, cx| {
        let actual_modules = running_state.update(cx, |state, cx| {
            state.module_list().update(cx, |list, cx| list.modules(cx))
        });

        assert_eq!(actual_modules.len(), 2);
        assert!(!actual_modules.contains(&changed_module));
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}
