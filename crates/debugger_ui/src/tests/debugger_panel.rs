#![expect(clippy::result_large_err)]
use crate::{
    persistence::DebuggerPaneItem,
    tests::{start_debug_session, start_debug_session_with},
    *,
};
use dap::{
    ErrorResponse, Message, RunInTerminalRequestArguments, SourceBreakpoint,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest,
    adapters::DebugTaskDefinition,
    client::SessionId,
    requests::{
        Continue, Disconnect, Launch, Next, RunInTerminal, SetBreakpoints, StackTrace,
        StartDebugging, StepBack, StepIn, StepOut, Terminate, Threads,
    },
};
use editor::{
    ActiveDebugLine, Editor, EditorMode, MultiBuffer,
    actions::{self},
};
use gpui::{
    App, AppContext, BackgroundExecutor, BorrowAppContext, Entity, TestAppContext, UpdateGlobal,
    VisualTestContext, px,
};
use project::{
    FakeFs, Project,
    debugger::session::{ThreadId, ThreadStatus},
};
use serde_json::json;
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};
use terminal_view::terminal_panel::TerminalPanel;
use tests::{active_debug_session_panel, init_test, init_test_workspace};
use util::{path, rel_path::rel_path};
use workspace::item::SaveOptions;
use workspace::pane_group::SplitDirection;
use workspace::{
    Item, Pane,
    dock::{Panel, test::TestPanel},
    move_active_item, move_item,
};

#[gpui::test]
async fn test_auto_close_prevents_restoring_debugger_visibility(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());

    for auto_close in [false, true] {
        for restore_before_panel in [false, true] {
            cx.update(|cx| {
                cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                    store.update_user_settings(cx, |settings| {
                        settings
                            .debugger
                            .get_or_insert_default()
                            .auto_close_on_session_end = Some(auto_close);
                    });
                });
            });
            let project = Project::test(fs.clone(), [], cx).await;
            let window = cx
                .add_window(|window, cx| workspace::MultiWorkspace::test_new(project, window, cx));
            let workspace = window
                .read_with(cx, |multi, _| multi.workspace().clone())
                .expect("workspace exists");
            let cx = &mut VisualTestContext::from_window(*window, cx);
            let mut saved = workspace.update_in(cx, |workspace, window, cx| {
                workspace.capture_dock_state(window, cx)
            });
            saved.bottom.visible = true;
            saved.bottom.active_panel = Some(DebugPanel::persistent_name().into());
            saved.bottom.zoom = true;
            if restore_before_panel {
                workspace.update_in(cx, |workspace, window, cx| {
                    workspace.set_dock_structure(saved.clone(), window, cx)
                });
            }
            let panel = workspace
                .update_in(cx, |_, window, cx| {
                    cx.spawn_in(window, async move |workspace, cx| {
                        DebugPanel::load(workspace, cx).await
                    })
                })
                .await
                .expect("debugger panel loads");
            workspace.update_in(cx, |workspace, window, cx| {
                workspace.add_panel(panel.clone(), window, cx);
                if !restore_before_panel {
                    workspace.set_dock_structure(saved, window, cx);
                }
            });
            cx.run_until_parked();
            workspace.update_in(cx, |workspace, window, cx| {
                assert_eq!(workspace.bottom_dock().read(cx).is_open(), !auto_close);
                assert_eq!(panel.read(cx).is_zoomed(window, cx), !auto_close);
                workspace.open_panel::<DebugPanel>(window, cx);
                assert!(workspace.bottom_dock().read(cx).is_open());
            });
        }
    }
}

#[gpui::test]
async fn test_auto_close_on_session_end(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"main.rs": "fn main() {}"}))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    for auto_close in [false, true] {
        cx.update(|_, cx| {
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .debugger
                        .get_or_insert_default()
                        .auto_close_on_session_end = Some(auto_close);
                });
            });
        });
        let first_session =
            start_debug_session(&workspace, cx, |_| {}).expect("first session should start");
        let second_session =
            start_debug_session(&workspace, cx, |_| {}).expect("second session should start");
        for (session, expected_open) in [(first_session, true), (second_session, !auto_close)] {
            let client = session.update(cx, |session, _| {
                session.adapter_client().expect("running adapter")
            });
            client
                .fake_event(dap::messages::Events::Terminated(None))
                .await;
            cx.run_until_parked();
            workspace
                .read_with(cx, |multi, cx| {
                    let workspace = multi.workspace().read(cx);
                    let panel = workspace
                        .panel::<DebugPanel>(cx)
                        .expect("debugger panel exists");
                    assert_eq!(workspace.bottom_dock().read(cx).is_open(), expected_open);
                    assert!(panel.read(cx).active_session().is_some());
                })
                .expect("workspace exists");
        }
    }

    start_debug_session(&workspace, cx, |_| {}).expect("session should start");
    // A test panel stands in for the terminal panel, which would spawn a real shell when opened.
    workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                let other_panel =
                    cx.new(|cx| TestPanel::new(workspace::dock::DockPosition::Bottom, 100, cx));
                workspace.add_panel(other_panel, window, cx);
                workspace.open_panel::<TestPanel>(window, cx);
            });
        })
        .expect("workspace exists");
    let session = project
        .read_with(cx, |project, cx| {
            project.dap_store().read(cx).sessions().next().cloned()
        })
        .expect("session exists");
    session.update(cx, |session, cx| session.shutdown(cx)).await;
    cx.run_until_parked();
    workspace
        .read_with(cx, |multi, cx| {
            let workspace = multi.workspace().read(cx);
            assert!(workspace.bottom_dock().read(cx).is_open());
            assert_eq!(
                workspace
                    .bottom_dock()
                    .read(cx)
                    .visible_panel()
                    .map(|panel| panel.persistent_name()),
                Some(TestPanel::persistent_name())
            );
        })
        .expect("workspace exists");
}

#[gpui::test]
async fn test_console_stays_in_terminal_across_debug_restarts(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"main.rs": "fn main() {}"}))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    cx.update(|_, cx| {
        cx.update_global::<settings::SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings
                    .debugger
                    .get_or_insert_default()
                    .auto_close_on_session_end = Some(true);
            });
        });
    });

    let first_session = start_debug_session(&workspace, cx, |_| {}).expect("session starts");
    let (source_pane, console_item_id, terminal_pane) = workspace
        .update(cx, |multi, _window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                let running = workspace
                    .panel::<DebugPanel>(cx)
                    .expect("debug panel")
                    .update(cx, |panel, cx| {
                        panel.running_state(cx).expect("running state")
                    });
                let source_pane = running
                    .read(cx)
                    .debugger_panes()
                    .into_iter()
                    .find(|pane| pane_console_count(pane, cx) > 0)
                    .expect("console starts in the debugger");
                let console_item_id = source_pane
                    .read(cx)
                    .items()
                    .find(|item| is_console_item(item.as_ref(), cx))
                    .map(|item| item.item_id())
                    .expect("console item");
                let terminal_pane = workspace
                    .panel::<TerminalPanel>(cx)
                    .expect("terminal panel")
                    .read(cx)
                    .panes()[0]
                    .clone();
                (source_pane, console_item_id, terminal_pane)
            })
        })
        .expect("workspace");

    workspace
        .update(cx, |_multi, window, cx| {
            move_item(
                &source_pane,
                &terminal_pane,
                console_item_id,
                0,
                false,
                window,
                cx,
            );
        })
        .expect("move console");
    cx.run_until_parked();

    workspace
        .update(cx, |multi, _window, cx| {
            let running = multi.workspace().update(cx, |workspace, cx| {
                workspace
                    .panel::<DebugPanel>(cx)
                    .unwrap()
                    .update(cx, |panel, cx| panel.running_state(cx).unwrap())
            });
            assert!(matches!(
                running.read(cx).console_anchor(),
                crate::persistence::ConsoleAnchor::Terminal { .. }
            ));
            assert_eq!(pane_console_count(&terminal_pane, cx), 1);
            assert!(
                running
                    .read(cx)
                    .debugger_panes()
                    .iter()
                    .all(|pane| pane_console_count(pane, cx) == 0)
            );
        })
        .expect("console moved");

    let client = first_session.update(cx, |session, _| session.adapter_client().unwrap());
    client
        .fake_event(dap::messages::Events::Terminated(None))
        .await;
    cx.run_until_parked();

    workspace
        .update(cx, |multi, _window, cx| {
            assert_eq!(pane_console_count(&terminal_pane, cx), 0);
            let running = multi.workspace().update(cx, |workspace, cx| {
                workspace
                    .panel::<DebugPanel>(cx)
                    .unwrap()
                    .update(cx, |panel, cx| panel.running_state(cx).unwrap())
            });
            assert!(matches!(
                running.read(cx).console_anchor(),
                crate::persistence::ConsoleAnchor::Terminal { .. }
            ));
        })
        .expect("console hidden");

    workspace
        .update(cx, |_multi, window, cx| {
            let editor = cx.new(|cx| Editor::single_line(window, cx));
            terminal_pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(editor), true, false, None, window, cx);
            });
        })
        .expect("terminal tab");

    let second_session = start_debug_session(&workspace, cx, |_| {}).expect("session restarts");
    let second_session_id = second_session.read_with(cx, |session, _| session.session_id());
    cx.run_until_parked();

    workspace
        .update(cx, |multi, _window, cx| {
            let consoles = console_views(&terminal_pane, cx);
            assert_eq!(consoles.len(), 1);
            assert_eq!(
                consoles[0].read(cx).running_session_id(cx),
                Some(second_session_id)
            );
            assert!(
                terminal_pane
                    .read(cx)
                    .active_item()
                    .is_some_and(|item| is_console_item(item.as_ref(), cx))
            );
            let running = multi.workspace().update(cx, |workspace, cx| {
                workspace
                    .panel::<DebugPanel>(cx)
                    .unwrap()
                    .update(cx, |panel, cx| panel.running_state(cx).unwrap())
            });
            assert!(
                running
                    .read(cx)
                    .debugger_panes()
                    .iter()
                    .all(|pane| pane_console_count(pane, cx) == 0)
            );
        })
        .expect("console restored");
}

fn is_console_item(item: &dyn workspace::ItemHandle, cx: &App) -> bool {
    item.downcast::<crate::session::running::SubView>()
        .is_some_and(|view| view.read(cx).view_kind() == DebuggerPaneItem::Console)
}

fn pane_console_count(pane: &Entity<Pane>, cx: &App) -> usize {
    console_views(pane, cx).len()
}

fn console_views(pane: &Entity<Pane>, cx: &App) -> Vec<Entity<crate::session::running::SubView>> {
    pane.read(cx)
        .items()
        .filter_map(|item| item.downcast::<crate::session::running::SubView>())
        .filter(|view| view.read(cx).view_kind() == DebuggerPaneItem::Console)
        .collect()
}

#[gpui::test]
async fn test_auto_close_on_session_end_reveals_project_panel(cx: &mut TestAppContext) {
    init_test(cx);
    cx.update(project_panel::init);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"main.rs": "fn main() {}"}))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let window = init_test_workspace(&project, cx).await;
    let workspace = window
        .read_with(cx, |multi, _| multi.workspace().clone())
        .expect("workspace exists");
    let cx = &mut VisualTestContext::from_window(*window, cx);
    let panel = workspace
        .update_in(cx, |_, window, cx| {
            cx.spawn_in(window, async move |workspace, cx| {
                project_panel::ProjectPanel::load(workspace, cx.clone()).await
            })
        })
        .await
        .expect("project panel loads");
    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_panel(panel.clone(), window, cx)
    });

    for (auto_open, auto_close, initially_visible, same_dock) in [
        (false, true, false, false),
        (true, false, false, false),
        (true, true, false, false),
        (true, true, true, false),
        (true, true, false, true),
    ] {
        cx.update(|_, cx| {
            cx.update_global::<settings::SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    let project_panel = settings.project_panel.get_or_insert_default();
                    project_panel.auto_open_on_project_open = Some(auto_open);
                    project_panel.dock = Some(settings::DockSide::Left);
                    let debugger = settings.debugger.get_or_insert_default();
                    debugger.auto_close_on_session_end = Some(auto_close);
                    debugger.dock = Some(if same_dock {
                        settings::DockPosition::Left
                    } else {
                        settings::DockPosition::Bottom
                    });
                });
            });
        });
        cx.run_until_parked();
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.close_all_docks(window, cx);
            if initially_visible {
                workspace.open_panel::<project_panel::ProjectPanel>(window, cx);
            }
        });
        let first_session = start_debug_session(&window, cx, |_| {}).expect("first session starts");
        let second_session =
            start_debug_session(&window, cx, |_| {}).expect("second session starts");
        for (session, last_session) in [(first_session, false), (second_session, true)] {
            let client = session.read_with(cx, |session, _| {
                session.adapter_client().expect("running adapter")
            });
            client
                .fake_event(dap::messages::Events::Terminated(None))
                .await;
            cx.run_until_parked();
            workspace.update_in(cx, |workspace, window, cx| {
                let project_visible = workspace
                    .left_dock()
                    .read(cx)
                    .visible_panel()
                    .is_some_and(|visible_panel| visible_panel.panel_id() == panel.entity_id());
                assert_eq!(
                    project_visible,
                    initially_visible || (last_session && auto_close && auto_open)
                );
                let debugger = workspace.panel::<DebugPanel>(cx).expect("debugger exists");
                let debugger_visible = workspace
                    .dock_at_position(debugger.read(cx).position(window, cx))
                    .read(cx)
                    .visible_panel()
                    .is_some_and(|visible_panel| visible_panel.panel_id() == debugger.entity_id());
                assert_eq!(debugger_visible, !(last_session && auto_close));
            });
        }
    }
}

#[gpui::test]
async fn test_browser_navigates_after_child_configuration(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"main.js": "console.log(42);"}))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let url = "http://localhost:3000/?name=\"test\"";
    let session = start_debug_session_with(
        &workspace,
        cx,
        DebugTaskDefinition {
            adapter: "fake-adapter".into(),
            label: "Browser".into(),
            config: json!({
                "type": "pwa-chrome", "request": "launch", "url": "about:blank",
                dap::adapters::ZED_BROWSER_LAUNCH_URL: url,
            }),
            tcp_connection: None,
        },
        |_| {},
    )
    .expect("parent session should start");
    let client = session.update(cx, |session, _| {
        session.adapter_client().expect("parent adapter")
    });
    let evaluations = Arc::new(AtomicUsize::new(0));
    let _subscription = project::debugger::test::intercept_debug_sessions(cx, {
        let evaluations = evaluations.clone();
        move |client| {
            let configured = Arc::new(AtomicBool::new(false));
            client.on_request::<dap::requests::Initialize, _>(|_, _| {
                Ok(dap::Capabilities {
                    supports_configuration_done_request: Some(true),
                    ..Default::default()
                })
            });
            client.on_request::<dap::requests::ConfigurationDone, _>({
                let configured = configured.clone();
                move |_, _| {
                    configured.store(true, Ordering::SeqCst);
                    Ok(())
                }
            });
            client.on_request::<dap::requests::Evaluate, _>({
                let evaluations = evaluations.clone();
                move |_, arguments| {
                    assert!(configured.load(Ordering::SeqCst));
                    assert_eq!(arguments.frame_id, None);
                    assert_eq!(arguments.context, Some(dap::EvaluateArgumentsContext::Repl));
                    assert_eq!(
                        arguments.expression,
                        format!("globalThis.location.href = {}", json!(url))
                    );
                    evaluations.fetch_add(1, Ordering::SeqCst);
                    Ok(dap::EvaluateResponse {
                        result: url.into(),
                        variables_reference: 0,
                        type_: None,
                        presentation_hint: None,
                        named_variables: None,
                        indexed_variables: None,
                        memory_reference: None,
                        value_location_reference: None,
                    })
                }
            });
        }
    });
    for (task_type, name, expected_evaluations) in [
        ("pwa-node", "Node", 0),
        ("pwa-chrome", "Restored page", 1),
        ("pwa-chrome", "about:blank", 1),
    ] {
        client
            .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
                request: StartDebuggingRequestArgumentsRequest::Launch,
                configuration: json!({
                    "type": task_type, "name": name, "__pendingTargetId": name,
                }),
            })
            .await;
        cx.run_until_parked();
        assert_eq!(evaluations.load(Ordering::SeqCst), expected_evaluations);
    }
}

#[gpui::test]
async fn test_breakpoint_activates_session_window(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"main.rs": "fn main() {}"}))
        .await;
    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let other_project = Project::test(fs, [], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let other_workspace = init_test_workspace(&other_project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let session = start_debug_session(&workspace, cx, |client| {
        client.on_request::<Threads, _>(|_, _| {
            Ok(dap::ThreadsResponse {
                threads: vec![dap::Thread {
                    id: 1,
                    name: "Main thread".into(),
                }],
            })
        });
        client.on_request::<StackTrace, _>(|_, _| {
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::new(),
                total_frames: None,
            })
        });
    })
    .expect("debug session should start");
    let client = session.update(cx, |session, _| {
        session.adapter_client().expect("running debug adapter")
    });

    for (reason, thread_id, preserve_focus_hint, breakpoint_hit) in [
        (dap::StoppedEventReason::Pause, Some(1), None, false),
        (dap::StoppedEventReason::Step, Some(1), None, false),
        (dap::StoppedEventReason::Entry, Some(1), None, false),
        (dap::StoppedEventReason::Breakpoint, Some(1), None, true),
        (
            dap::StoppedEventReason::FunctionBreakpoint,
            Some(1),
            None,
            true,
        ),
        (dap::StoppedEventReason::DataBreakpoint, Some(1), None, true),
        (
            dap::StoppedEventReason::InstructionBreakpoint,
            Some(1),
            None,
            true,
        ),
        (dap::StoppedEventReason::Breakpoint, None, None, true),
        (
            dap::StoppedEventReason::Breakpoint,
            Some(1),
            Some(true),
            true,
        ),
    ] {
        other_workspace
            .update(cx, |_, window, _| window.activate_window())
            .expect("other workspace window exists");
        cx.run_until_parked();
        assert!(!cx.update(|window, _| window.is_window_active()));

        client
            .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
                reason,
                description: None,
                thread_id,
                preserve_focus_hint,
                text: None,
                all_threads_stopped: Some(true),
                hit_breakpoint_ids: None,
            }))
            .await;
        cx.run_until_parked();

        assert_eq!(
            cx.update(|window, _| window.is_window_active()),
            breakpoint_hit
        );
        assert_eq!(
            other_workspace
                .update(cx, |_, window, _| window.is_window_active())
                .expect("other workspace window exists"),
            !breakpoint_hit,
        );
    }
}

#[gpui::test]
async fn test_debug_panel_default_size(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;
    let workspace = init_test_workspace(&project, cx).await;

    workspace
        .update(cx, |workspace, window, cx| {
            let panel = workspace.panel::<DebugPanel>(cx).unwrap();
            assert_eq!(panel.read(cx).default_size(window, cx), px(300.));

            for (dock, expected_size) in [
                (settings::DockPosition::Left, px(495.)),
                (settings::DockPosition::Right, px(495.)),
                (settings::DockPosition::Bottom, px(275.)),
            ] {
                settings::SettingsStore::update_global(cx, |store, cx| {
                    store.update_user_settings(cx, |settings| {
                        let debugger = settings.debugger.get_or_insert_default();
                        debugger.dock = Some(dock);
                        debugger.default_width = Some(495.0.into());
                        debugger.default_height = Some(275.0.into());
                    });
                });

                assert_eq!(panel.read(cx).default_size(window, cx), expected_size);
            }
        })
        .unwrap();
}

#[gpui::test]
async fn test_basic_show_debug_panel(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
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
            stack_frames: Vec::default(),
            total_frames: None,
        })
    });

    cx.run_until_parked();

    // assert we have a debug panel item before the session has stopped
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session =
                debug_panel.update(cx, |debug_panel, _| debug_panel.active_session().unwrap());

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_session().is_some());
                assert!(running_state.read(cx).selected_thread_id().is_none());
            });
        })
        .unwrap();

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

    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session = debug_panel
                .update(cx, |this, _| this.active_session())
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            assert_eq!(client.id(), running_state.read(cx).session_id());
            assert_eq!(
                ThreadId(1),
                running_state.read(cx).selected_thread_id().unwrap()
            );
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();

    // assert we still have a debug panel item after the client shutdown
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            let active_session = debug_panel
                .update(cx, |this, _| this.active_session())
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_session().is_some());
                assert_eq!(
                    ThreadId(1),
                    running_state.read(cx).selected_thread_id().unwrap()
                );
            });
        })
        .unwrap();
}

#[gpui::test]
async fn test_we_can_only_have_one_panel_per_debug_session(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
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
            stack_frames: Vec::default(),
            total_frames: None,
        })
    });

    cx.run_until_parked();

    // assert we have a debug panel item before the session has stopped
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            debug_panel.update(cx, |this, _| {
                assert!(this.active_session().is_some());
            });
        })
        .unwrap();

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

    // assert we added a debug panel item
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session = debug_panel
                .update(cx, |this, _| this.active_session())
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            assert_eq!(client.id(), active_session.read(cx).session_id(cx));
            assert_eq!(
                ThreadId(1),
                running_state.read(cx).selected_thread_id().unwrap()
            );
        })
        .unwrap();

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Pause,
            description: None,
            thread_id: Some(2),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session = debug_panel
                .update(cx, |this, _| this.active_session())
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            assert_eq!(client.id(), active_session.read(cx).session_id(cx));
            assert_eq!(
                ThreadId(1),
                running_state.read(cx).selected_thread_id().unwrap()
            );
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();

    // assert we still have a debug panel item after the client shutdown
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session = debug_panel
                .update(cx, |this, _| this.active_session())
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_session().is_some());
                assert_eq!(
                    ThreadId(1),
                    running_state.read(cx).selected_thread_id().unwrap()
                );
            });
        })
        .unwrap();
}

#[gpui::test]
async fn test_handle_successful_run_in_terminal_reverse_request(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    // needed because the debugger launches a terminal which starts a background PTY
    cx.executor().allow_parking();
    init_test(cx);

    let send_response = Arc::new(AtomicBool::new(false));

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .on_response::<RunInTerminal, _>({
            let send_response = send_response.clone();
            move |response| {
                send_response.store(true, Ordering::SeqCst);

                assert!(response.success);
                assert!(response.body.is_some());
            }
        })
        .await;

    client
        .fake_reverse_request::<RunInTerminal>(RunInTerminalRequestArguments {
            kind: None,
            title: None,
            cwd: std::env::temp_dir().to_string_lossy().into_owned(),
            args: vec![],
            env: None,
            args_can_be_interpreted_by_shell: None,
        })
        .await;

    cx.run_until_parked();

    assert!(
        send_response.load(std::sync::atomic::Ordering::SeqCst),
        "Expected to receive response from reverse request"
    );

    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let session = debug_panel.read(cx).active_session().unwrap();
            let running = session.read(cx).running_state();
            assert_eq!(
                running
                    .read(cx)
                    .pane_items_status(cx)
                    .get(&DebuggerPaneItem::Terminal),
                Some(&true)
            );
            assert!(running.read(cx).debug_terminal.read(cx).terminal.is_some());
        })
        .unwrap();
}

#[gpui::test]
async fn test_handle_start_debugging_request(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let fake_config = json!({"one": "two"});
    let launched_with = Arc::new(parking_lot::Mutex::new(None));

    let _subscription = project::debugger::test::intercept_debug_sessions(cx, {
        let launched_with = launched_with.clone();
        move |client| {
            let launched_with = launched_with.clone();
            client.on_request::<dap::requests::Launch, _>(move |_, args| {
                launched_with.lock().replace(args.raw);
                Ok(())
            });
            client.on_request::<dap::requests::Attach, _>(move |_, _| {
                assert!(false, "should not get attach request");
                Ok(())
            });
        }
    });

    let sessions = workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            debug_panel.read(cx).sessions().collect::<Vec<_>>()
        })
        .unwrap();
    assert_eq!(sessions.len(), 1);
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            request: StartDebuggingRequestArgumentsRequest::Launch,
            configuration: fake_config.clone(),
        })
        .await;

    cx.run_until_parked();

    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            // Active session changes on spawn, as the parent has never stopped.
            let active_session = debug_panel
                .read(cx)
                .active_session()
                .unwrap()
                .read(cx)
                .session(cx);
            let current_sessions = debug_panel.read(cx).sessions().collect::<Vec<_>>();
            assert_eq!(active_session, current_sessions[1].read(cx).session(cx));
            assert_eq!(
                active_session.read(cx).parent_session(),
                Some(&current_sessions[0].read(cx).session(cx))
            );

            assert_eq!(current_sessions.len(), 2);
            assert_eq!(current_sessions[0], sessions[0]);

            let parent_session = current_sessions[1]
                .read(cx)
                .session(cx)
                .read(cx)
                .parent_session()
                .unwrap();
            assert_eq!(parent_session, &sessions[0].read(cx).session(cx));

            // We should preserve the original binary (params to spawn process etc.) except for launch params
            // (as they come from reverse spawn request).
            let mut original_binary = parent_session.read(cx).binary().cloned().unwrap();
            original_binary.request_args = StartDebuggingRequestArguments {
                request: StartDebuggingRequestArgumentsRequest::Launch,
                configuration: fake_config.clone(),
            };

            assert_eq!(
                current_sessions[1]
                    .read(cx)
                    .session(cx)
                    .read(cx)
                    .binary()
                    .unwrap(),
                &original_binary
            );
        })
        .unwrap();

    assert_eq!(&fake_config, launched_with.lock().as_ref().unwrap());
}

// // covers that we always send a response back, if something when wrong,
// // while spawning the terminal
#[gpui::test]
async fn test_handle_error_run_in_terminal_reverse_request(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let send_response = Arc::new(AtomicBool::new(false));

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .on_response::<RunInTerminal, _>({
            let send_response = send_response.clone();
            move |response| {
                send_response.store(true, Ordering::SeqCst);

                assert!(!response.success);
                assert!(response.body.is_some());
            }
        })
        .await;

    client
        .fake_reverse_request::<RunInTerminal>(RunInTerminalRequestArguments {
            kind: None,
            title: None,
            cwd: "".into(),
            args: vec!["oops".into(), "oops".into()],
            env: None,
            args_can_be_interpreted_by_shell: None,
        })
        .await;

    cx.run_until_parked();

    assert!(
        send_response.load(std::sync::atomic::Ordering::SeqCst),
        "Expected to receive response from reverse request"
    );

    workspace
        .update(cx, |workspace, _window, cx| {
            let terminal_panel = workspace.panel::<TerminalPanel>(cx).unwrap();

            assert_eq!(
                0,
                terminal_panel.read(cx).pane().unwrap().read(cx).items_len()
            );
        })
        .unwrap();
}

#[gpui::test]
async fn test_handle_start_debugging_reverse_request(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    cx.executor().allow_parking();
    init_test(cx);

    let send_response = Arc::new(AtomicBool::new(false));

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
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

    client
        .on_response::<StartDebugging, _>({
            let send_response = send_response.clone();
            move |response| {
                send_response.store(true, Ordering::SeqCst);

                assert!(response.success);
                assert!(response.body.is_some());
            }
        })
        .await;
    // Set up handlers for sessions spawned with reverse request too.
    let _reverse_request_subscription =
        project::debugger::test::intercept_debug_sessions(cx, |_| {});
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    let child_session = project.update(cx, |project, cx| {
        project
            .dap_store()
            .read(cx)
            .session_by_id(SessionId(1))
            .unwrap()
    });
    let child_client = child_session.update(cx, |session, _| session.adapter_client().unwrap());

    child_client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    child_client.on_request::<Disconnect, _>(move |_, _| Ok(()));

    child_client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Pause,
            description: None,
            thread_id: Some(2),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    assert!(
        send_response.load(std::sync::atomic::Ordering::SeqCst),
        "Expected to receive response from reverse request"
    );
}

#[gpui::test]
async fn test_shutdown_children_when_parent_session_shutdown(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let dap_store = project.update(cx, |project, _| project.dap_store());
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let parent_session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = parent_session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_response::<StartDebugging, _>(move |_| {}).await;
    // Set up handlers for sessions spawned with reverse request too.
    let _reverse_request_subscription =
        project::debugger::test::intercept_debug_sessions(cx, |_| {});
    // start first child session
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    // start second child session
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    // configure first child session
    let first_child_session = dap_store.read_with(cx, |dap_store, _| {
        dap_store.session_by_id(SessionId(1)).unwrap()
    });
    let first_child_client =
        first_child_session.update(cx, |session, _| session.adapter_client().unwrap());

    first_child_client.on_request::<Disconnect, _>(move |_, _| Ok(()));

    // configure second child session
    let second_child_session = dap_store.read_with(cx, |dap_store, _| {
        dap_store.session_by_id(SessionId(2)).unwrap()
    });
    let second_child_client =
        second_child_session.update(cx, |session, _| session.adapter_client().unwrap());

    second_child_client.on_request::<Disconnect, _>(move |_, _| Ok(()));

    cx.run_until_parked();

    // shutdown parent session
    dap_store
        .update(cx, |dap_store, cx| {
            dap_store.shutdown_session(parent_session.read(cx).session_id(), cx)
        })
        .await
        .unwrap();

    // assert parent session and all children sessions are shutdown
    dap_store.update(cx, |dap_store, cx| {
        assert!(
            dap_store
                .session_by_id(parent_session.read(cx).session_id())
                .is_none()
        );
        assert!(
            dap_store
                .session_by_id(first_child_session.read(cx).session_id())
                .is_none()
        );
        assert!(
            dap_store
                .session_by_id(second_child_session.read(cx).session_id())
                .is_none()
        );
    });
}

#[gpui::test]
async fn test_parent_session_survives_all_children_shutting_down(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let dap_store = project.update(cx, |project, _| project.dap_store());
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let parent_session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = parent_session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_response::<StartDebugging, _>(move |_| {}).await;
    // Set up handlers for sessions spawned with reverse request too.
    let _reverse_request_subscription =
        project::debugger::test::intercept_debug_sessions(cx, |_| {});
    // start first child session
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    // start second child session
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    // configure first child session
    let first_child_session = dap_store.read_with(cx, |dap_store, _| {
        dap_store.session_by_id(SessionId(1)).unwrap()
    });
    let first_child_client =
        first_child_session.update(cx, |session, _| session.adapter_client().unwrap());

    first_child_client.on_request::<Disconnect, _>(move |_, _| Ok(()));

    // configure second child session
    let second_child_session = dap_store.read_with(cx, |dap_store, _| {
        dap_store.session_by_id(SessionId(2)).unwrap()
    });
    let second_child_client =
        second_child_session.update(cx, |session, _| session.adapter_client().unwrap());

    second_child_client.on_request::<Disconnect, _>(move |_, _| Ok(()));

    cx.run_until_parked();

    // shutdown first child session
    dap_store
        .update(cx, |dap_store, cx| {
            dap_store.shutdown_session(first_child_session.read(cx).session_id(), cx)
        })
        .await
        .unwrap();

    // assert parent session and second child session still exist
    dap_store.update(cx, |dap_store, cx| {
        assert!(
            dap_store
                .session_by_id(parent_session.read(cx).session_id())
                .is_some()
        );
        assert!(
            dap_store
                .session_by_id(first_child_session.read(cx).session_id())
                .is_none()
        );
        assert!(
            dap_store
                .session_by_id(second_child_session.read(cx).session_id())
                .is_some()
        );
    });

    dap_store
        .update(cx, |dap_store, cx| {
            dap_store.shutdown_session(second_child_session.read(cx).session_id(), cx)
        })
        .await
        .unwrap();

    dap_store.update(cx, |dap_store, cx| {
        assert!(
            dap_store
                .session_by_id(parent_session.read(cx).session_id())
                .is_some()
        );
        assert!(
            dap_store
                .session_by_id(second_child_session.read(cx).session_id())
                .is_none()
        );
        assert!(!parent_session.read(cx).is_terminated());
        assert!(parent_session.read(cx).child_session_ids().is_empty());
    });

    dap_store
        .update(cx, |dap_store, cx| {
            dap_store.shutdown_session(parent_session.read(cx).session_id(), cx)
        })
        .await
        .expect("explicit parent shutdown should succeed");
}

#[gpui::test]
async fn test_titlebar_debug_controls_follow_session_with_panel_closed(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(executor);
    fs.insert_tree(path!("/project"), json!({ "main.rs": "" }))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                let controls = workspace
                    .debugger_provider()
                    .and_then(|provider| provider.titlebar_controls())
                    .expect("debugger provides titlebar controls");
                workspace.set_titlebar_item(controls, window, cx);
                workspace.close_panel::<DebugPanel>(window, cx);
            });
        })
        .expect("workspace exists");
    cx.refresh().expect("window refreshes");
    assert!(cx.debug_bounds("debug-titlebar-controls").is_none());

    let step_requests = Arc::new(AtomicUsize::new(0));
    let pause_requests = Arc::new(AtomicUsize::new(0));
    let session = start_debug_session(&workspace, cx, {
        let step_requests = step_requests.clone();
        let pause_requests = pause_requests.clone();
        move |client| {
            client.on_request::<Threads, _>(|_, _| {
                Ok(dap::ThreadsResponse {
                    threads: vec![dap::Thread {
                        id: 1,
                        name: "Main thread".into(),
                    }],
                })
            });
            client.on_request::<StackTrace, _>(|_, _| {
                Ok(dap::StackTraceResponse {
                    stack_frames: Vec::new(),
                    total_frames: None,
                })
            });
            client.on_request::<Next, _>({
                let step_requests = step_requests.clone();
                move |_, _| {
                    step_requests.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            });
            client.on_request::<dap::requests::Pause, _>({
                let pause_requests = pause_requests.clone();
                move |_, _| {
                    pause_requests.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            });
        }
    })
    .expect("debug session starts");
    session.update(cx, |session, cx| {
        session.threads(cx);
    });
    cx.run_until_parked();
    workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.close_panel::<DebugPanel>(window, cx);
            });
        })
        .expect("workspace exists");
    cx.refresh().expect("window refreshes");
    assert!(cx.debug_bounds("debug-titlebar-controls").is_some());
    let step_button = cx
        .debug_bounds("ICON-DebugStepOver")
        .expect("step button exists");
    cx.simulate_click(step_button.center(), gpui::Modifiers::none());
    assert_eq!(step_requests.load(Ordering::SeqCst), 0);
    let pause_button = cx
        .debug_bounds("ICON-DebugPause")
        .expect("pause button exists");
    cx.simulate_click(pause_button.center(), gpui::Modifiers::none());
    assert_eq!(pause_requests.load(Ordering::SeqCst), 1);

    let client = session.read_with(cx, |session, _| {
        session.adapter_client().expect("adapter is connected")
    });
    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: Some(true),
            hit_breakpoint_ids: None,
        }))
        .await;
    cx.run_until_parked();
    workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.close_panel::<DebugPanel>(window, cx);
            });
        })
        .expect("workspace exists");
    cx.refresh().expect("window refreshes");
    assert!(cx.debug_bounds("ICON-DebugContinue").is_some());
    let step_button = cx
        .debug_bounds("ICON-DebugStepOver")
        .expect("step button exists");
    cx.simulate_click(step_button.center(), gpui::Modifiers::none());
    assert_eq!(step_requests.load(Ordering::SeqCst), 1);

    let stop_button = cx.debug_bounds("ICON-Stop").expect("stop button exists");
    cx.simulate_click(stop_button.center(), gpui::Modifiers::none());
    cx.run_until_parked();
    assert!(session.read_with(cx, |session, _| session.is_terminated()));
    cx.refresh().expect("window refreshes");
    assert!(cx.debug_bounds("debug-titlebar-controls").is_none());
}

#[gpui::test]
async fn test_running_session_binds_continue_before_first_stop(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(path!("/project"), json!({ "main.rs": "" }))
        .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |client| {
        client.on_request::<Threads, _>(|_, _| {
            Ok(dap::ThreadsResponse {
                threads: vec![dap::Thread {
                    id: 1,
                    name: "Main thread".into(),
                }],
            })
        });
        client.on_request::<StackTrace, _>(|_, _| {
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::new(),
                total_frames: None,
            })
        });
        client.on_request::<Continue, _>(|_, _| {
            Ok(dap::ContinueResponse {
                all_threads_continued: None,
            })
        });
    })
    .unwrap();

    session.update(cx, |session, cx| {
        session.threads(cx);
    });
    cx.run_until_parked();

    session.update(cx, |session, _| {
        assert!(session.thread_state(ThreadId(1)).is_none());
    });
    workspace
        .update(cx, |multi, _, cx| {
            let context = multi.workspace().read(cx).key_context(cx);
            assert!(context.contains("debugger_running"));
            assert!(context.contains("debugger_session"));
            assert!(!context.contains("debugger_stopped"));
        })
        .unwrap();

    let client = session.update(cx, |session, _| session.adapter_client().unwrap());
    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: Some(true),
            hit_breakpoint_ids: None,
        }))
        .await;
    cx.run_until_parked();

    workspace
        .update(cx, |multi, _, cx| {
            let context = multi.workspace().read(cx).key_context(cx);
            assert!(context.contains("debugger_stopped"));
            assert!(!context.contains("debugger_running"));
        })
        .unwrap();

    session.update(cx, |session, cx| {
        session.continue_program(ThreadId(1), cx);
    });
    cx.run_until_parked();

    workspace
        .update(cx, |multi, _, cx| {
            let context = multi.workspace().read(cx).key_context(cx);
            assert!(context.contains("debugger_running"));
            assert!(!context.contains("debugger_stopped"));
        })
        .unwrap();
}

#[gpui::test]
async fn test_continue_response_updates_thread_statuses(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(path!("/project"), json!({ "main.rs": "" }))
        .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |client| {
        client.on_request::<Continue, _>(move |_, args| {
            assert_eq!(args.single_thread, None);
            Ok(dap::ContinueResponse {
                all_threads_continued: None,
            })
        });
    })
    .unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: Some(true),
            hit_breakpoint_ids: None,
        }))
        .await;
    cx.run_until_parked();

    session.update(cx, |session, cx| {
        assert_eq!(session.thread_status(ThreadId(1)), ThreadStatus::Stopped);
        assert_eq!(session.thread_status(ThreadId(2)), ThreadStatus::Stopped);
        session.continue_program(ThreadId(1), cx);
    });
    cx.run_until_parked();

    session.update(cx, |session, _| {
        assert_eq!(session.thread_status(ThreadId(1)), ThreadStatus::Running);
        assert_eq!(session.thread_status(ThreadId(2)), ThreadStatus::Running);
    });

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: Some(true),
            hit_breakpoint_ids: None,
        }))
        .await;
    cx.run_until_parked();

    cx.dispatch_action(ContinueThread);
    cx.run_until_parked();

    session.update(cx, |session, _| {
        assert_eq!(session.thread_status(ThreadId(1)), ThreadStatus::Stopped);
        assert_eq!(session.thread_status(ThreadId(2)), ThreadStatus::Stopped);
    });
}

#[gpui::test]
async fn test_continue_thread_action_only_resumes_selected_thread(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(path!("/project"), json!({ "main.rs": "" }))
        .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let continue_requested = Arc::new(AtomicBool::new(false));

    let session = start_debug_session(&workspace, cx, {
        let continue_requested = continue_requested.clone();
        move |client| {
            client.on_request::<dap::requests::Initialize, _>(move |_, _| {
                Ok(dap::Capabilities {
                    supports_single_thread_execution_requests: Some(true),
                    ..Default::default()
                })
            });
            client.on_request::<Continue, _>({
                let continue_requested = continue_requested.clone();
                move |_, args| {
                    assert_eq!(args.single_thread, Some(true));
                    continue_requested.store(true, Ordering::SeqCst);
                    Ok(dap::ContinueResponse {
                        all_threads_continued: Some(false),
                    })
                }
            });
        }
    })
    .unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: Some(true),
            hit_breakpoint_ids: None,
        }))
        .await;
    cx.run_until_parked();

    cx.dispatch_action(ContinueThread);
    cx.run_until_parked();

    assert!(continue_requested.load(Ordering::SeqCst));
    session.update(cx, |session, _| {
        assert_eq!(session.thread_status(ThreadId(1)), ThreadStatus::Running);
        assert_eq!(session.thread_status(ThreadId(2)), ThreadStatus::Stopped);
    });
}

#[gpui::test]
async fn test_debug_panel_item_thread_status_reset_on_failure(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |client| {
        client.on_request::<dap::requests::Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_step_back: Some(true),
                ..Default::default()
            })
        });
    })
    .unwrap();

    let client = session.update(cx, |session, _| session.adapter_client().unwrap());
    const THREAD_ID_NUM: i64 = 1;

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: THREAD_ID_NUM,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<Launch, _>(move |_, _| Ok(()));

    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: Vec::default(),
            total_frames: None,
        })
    });

    client.on_request::<Next, _>(move |_, _| {
        Err(ErrorResponse {
            error: Some(dap::Message {
                id: 1,
                format: "error".into(),
                variables: None,
                send_telemetry: None,
                show_user: None,
                url: None,
                url_label: None,
            }),
        })
    });

    client.on_request::<StepOut, _>(move |_, _| {
        Err(ErrorResponse {
            error: Some(dap::Message {
                id: 1,
                format: "error".into(),
                variables: None,
                send_telemetry: None,
                show_user: None,
                url: None,
                url_label: None,
            }),
        })
    });

    client.on_request::<StepIn, _>(move |_, _| {
        Err(ErrorResponse {
            error: Some(dap::Message {
                id: 1,
                format: "error".into(),
                variables: None,
                send_telemetry: None,
                show_user: None,
                url: None,
                url_label: None,
            }),
        })
    });

    client.on_request::<StepBack, _>(move |_, _| {
        Err(ErrorResponse {
            error: Some(dap::Message {
                id: 1,
                format: "error".into(),
                variables: None,
                send_telemetry: None,
                show_user: None,
                url: None,
                url_label: None,
            }),
        })
    });

    client.on_request::<Continue, _>(move |_, _| {
        Err(ErrorResponse {
            error: Some(dap::Message {
                id: 1,
                format: "error".into(),
                variables: None,
                send_telemetry: None,
                show_user: None,
                url: None,
                url_label: None,
            }),
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

    let running_state = active_debug_session_panel(workspace, cx)
        .read_with(cx, |item, _| item.running_state().clone());

    cx.run_until_parked();
    let thread_id = ThreadId(1);

    for operation in &[
        "step_over",
        "continue_program",
        "step_back",
        "step_in",
        "step_out",
    ] {
        running_state.update(cx, |running_state, cx| match *operation {
            "step_over" => running_state.step_over(cx),
            "continue_program" => running_state.continue_program(cx),
            "step_back" => running_state.step_back(cx),
            "step_in" => running_state.step_in(cx),
            "step_out" => running_state.step_out(cx),
            _ => unreachable!(),
        });

        // Check that we step the thread status to the correct intermediate state
        running_state.update(cx, |running_state, cx| {
            assert_eq!(
                running_state
                    .thread_status(cx)
                    .expect("There should be an active thread selected"),
                match *operation {
                    "continue_program" => ThreadStatus::Running,
                    _ => ThreadStatus::Stepping,
                },
                "Thread status was not set to correct intermediate state after {} request",
                operation
            );
        });

        cx.run_until_parked();

        running_state.update(cx, |running_state, cx| {
            assert_eq!(
                running_state
                    .thread_status(cx)
                    .expect("There should be an active thread selected"),
                ThreadStatus::Stopped,
                "Thread status not reset to Stopped after failed {}",
                operation
            );

            // update state to running, so we can test it actually changes the status back to stopped
            running_state
                .session()
                .update(cx, |session, cx| session.continue_program(thread_id, cx));
        });
    }
}

#[gpui::test]
async fn test_send_breakpoints_when_editor_has_been_saved(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let project_path = Path::new(path!("/project"));
    let worktree = project
        .update(cx, |project, cx| project.find_worktree(project_path, cx))
        .expect("This worktree should exist in project")
        .0;

    let worktree_id = workspace
        .update(cx, |_, _, cx| worktree.read(cx).id())
        .unwrap();

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();

    let (editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    client.on_request::<Launch, _>(move |_, _| Ok(()));

    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: Vec::default(),
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

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client.on_request::<SetBreakpoints, _>({
        let called_set_breakpoints = called_set_breakpoints.clone();
        move |_, args| {
            assert_eq!(path!("/project/main.rs"), args.source.path.unwrap());
            assert_eq!(
                vec![SourceBreakpoint {
                    line: 2,
                    column: None,
                    condition: None,
                    hit_condition: None,
                    log_message: None,
                    mode: None
                }],
                args.breakpoints.unwrap()
            );
            assert!(!args.source_modified.unwrap());

            called_set_breakpoints.store(true, Ordering::SeqCst);

            Ok(dap::SetBreakpointsResponse {
                breakpoints: Vec::default(),
            })
        }
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    cx.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called"
    );

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client.on_request::<SetBreakpoints, _>({
        let called_set_breakpoints = called_set_breakpoints.clone();
        move |_, args| {
            assert_eq!(path!("/project/main.rs"), args.source.path.unwrap());
            assert_eq!(
                vec![SourceBreakpoint {
                    line: 3,
                    column: None,
                    condition: None,
                    hit_condition: None,
                    log_message: None,
                    mode: None
                }],
                args.breakpoints.unwrap()
            );
            assert!(args.source_modified.unwrap());

            called_set_breakpoints.store(true, Ordering::SeqCst);

            Ok(dap::SetBreakpointsResponse {
                breakpoints: Vec::default(),
            })
        }
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.move_up(&zed_actions::editor::MoveUp, window, cx);
        editor.insert("new text\n", window, cx);
    });

    editor
        .update_in(cx, |editor, window, cx| {
            editor.save(
                SaveOptions {
                    format: true,
                    force_format: false,
                    autosave: false,
                },
                project.clone(),
                window,
                cx,
            )
        })
        .await
        .unwrap();

    cx.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called after editor is saved"
    );
}

#[gpui::test]
async fn test_unsetting_breakpoints_on_clear_breakpoint_action(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
            "second.rs": "First line\nSecond line\nThird line\nFourth line",
            "no_breakpoints.rs": "Used to ensure that we don't unset breakpoint in files with no breakpoints"
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let project_path = Path::new(path!("/project"));
    let worktree = project
        .update(cx, |project, cx| project.find_worktree(project_path, cx))
        .expect("This worktree should exist in project")
        .0;

    let worktree_id = workspace
        .update(cx, |_, _, cx| worktree.read(cx).id())
        .unwrap();

    let first = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();

    let second = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("second.rs")), cx)
        })
        .await
        .unwrap();

    let (first_editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(first, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    let (second_editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(second, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    first_editor.update_in(cx, |editor, window, cx| {
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    second_editor.update_in(cx, |editor, window, cx| {
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));

    client.on_request::<SetBreakpoints, _>({
        move |_, args| {
            assert!(
                args.breakpoints.is_none_or(|bps| bps.is_empty()),
                "Send empty breakpoint sets to clear them from DAP servers"
            );

            match args
                .source
                .path
                .expect("We should always send a breakpoint's path")
                .as_str()
            {
                path!("/project/main.rs") | path!("/project/second.rs") => {}
                _ => {
                    panic!("Unset breakpoints for path that doesn't have any")
                }
            }

            called_set_breakpoints.store(true, Ordering::SeqCst);

            Ok(dap::SetBreakpointsResponse {
                breakpoints: Vec::default(),
            })
        }
    });

    cx.dispatch_action(crate::ClearAllBreakpoints);
    cx.run_until_parked();
}

#[gpui::test]
async fn test_debug_session_is_shutdown_when_attach_and_launch_request_fails(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let disconnected = Arc::new(AtomicBool::new(false));
    let result = start_debug_session(&workspace, cx, {
        let disconnected = disconnected.clone();
        move |client| {
            let disconnected = disconnected.clone();
            client.on_request::<Disconnect, _>(move |_, arguments| {
                assert_eq!(arguments.terminate_debuggee, Some(false));
                disconnected.store(true, Ordering::SeqCst);
                Ok(())
            });
            client.on_request::<dap::requests::Initialize, _>(|_, _| {
                Err(ErrorResponse {
                    error: Some(Message {
                        format: "failed to launch".to_string(),
                        id: 1,
                        variables: None,
                        send_telemetry: None,
                        show_user: None,
                        url: None,
                        url_label: None,
                    }),
                })
            });
        }
    });
    if let Ok(session) = result {
        assert!(session.read_with(cx, |session, _| session.is_terminated()));
    }

    cx.run_until_parked();
    assert!(disconnected.load(Ordering::SeqCst));

    project.update(cx, |project, cx| {
        assert!(
            project.dap_store().read(cx).sessions().count() == 0,
            "Session wouldn't exist if it was shutdown"
        );
    });
}

#[gpui::test]
async fn test_we_send_arguments_from_user_config(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let debug_definition = DebugTaskDefinition {
        adapter: "fake-adapter".into(),
        config: json!({
            "request": "launch",
            "program": "main.rs".to_owned(),
            "args": vec!["arg1".to_owned(), "arg2".to_owned()],
            "cwd": path!("/Random_path"),
            "env": json!({ "KEY": "VALUE" }),
        }),
        label: "test".into(),
        tcp_connection: None,
    };

    let launch_handler_called = Arc::new(AtomicBool::new(false));

    start_debug_session_with(&workspace, cx, debug_definition.clone(), {
        let launch_handler_called = launch_handler_called.clone();

        move |client| {
            let debug_definition = debug_definition.clone();
            let launch_handler_called = launch_handler_called.clone();

            client.on_request::<dap::requests::Launch, _>(move |_, args| {
                launch_handler_called.store(true, Ordering::SeqCst);

                assert_eq!(args.raw, debug_definition.config);

                Ok(())
            });
        }
    })
    .ok();

    cx.run_until_parked();

    assert!(
        launch_handler_called.load(Ordering::SeqCst),
        "Launch request handler was not called"
    );
}

#[gpui::test]
async fn test_active_debug_line_setting(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
            "second.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let project_path = Path::new(path!("/project"));
    let worktree = project
        .update(cx, |project, cx| project.find_worktree(project_path, cx))
        .expect("This worktree should exist in project")
        .0;

    let worktree_id = workspace
        .update(cx, |_, _, cx| worktree.read(cx).id())
        .unwrap();

    let main_buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();

    let second_buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("second.rs")), cx)
        })
        .await
        .unwrap();

    let (main_editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(main_buffer, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    let (second_editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(second_buffer, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

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

    client.on_request::<dap::requests::Scopes, _>(move |_, _| {
        Ok(dap::ScopesResponse {
            scopes: Vec::default(),
        })
    });

    client.on_request::<StackTrace, _>(move |_, args| {
        assert_eq!(args.thread_id, 1);

        Ok(dap::StackTraceResponse {
            stack_frames: vec![dap::StackFrame {
                id: 1,
                name: "frame 1".into(),
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
                line: 2,
                column: 0,
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

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    main_editor.update_in(cx, |editor, window, cx| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>(cx).collect();

        assert_eq!(
            active_debug_lines.len(),
            1,
            "There should be only one active debug line"
        );

        let point = editor
            .snapshot(window, cx)
            .buffer_snapshot()
            .summary_for_anchor::<language::Point>(&active_debug_lines.first().unwrap().0.start);

        assert_eq!(point.row, 1);
    });

    second_editor.update(cx, |editor, cx| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>(cx).collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines"
        );
    });

    let handled_second_stacktrace = Arc::new(AtomicBool::new(false));
    client.on_request::<StackTrace, _>({
        let handled_second_stacktrace = handled_second_stacktrace.clone();
        move |_, args| {
            handled_second_stacktrace.store(true, Ordering::SeqCst);
            assert_eq!(args.thread_id, 1);

            Ok(dap::StackTraceResponse {
                stack_frames: vec![dap::StackFrame {
                    id: 2,
                    name: "frame 2".into(),
                    source: Some(dap::Source {
                        name: Some("second.rs".into()),
                        path: Some(path!("/project/second.rs").into()),
                        source_reference: None,
                        presentation_hint: None,
                        origin: None,
                        sources: None,
                        adapter_data: None,
                        checksums: None,
                    }),
                    line: 3,
                    column: 0,
                    end_line: None,
                    end_column: None,
                    can_restart: None,
                    instruction_pointer_reference: None,
                    module_id: None,
                    presentation_hint: None,
                }],
                total_frames: None,
            })
        }
    });

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    second_editor.update_in(cx, |editor, window, cx| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>(cx).collect();

        assert_eq!(
            active_debug_lines.len(),
            1,
            "There should be only one active debug line"
        );

        let point = editor
            .snapshot(window, cx)
            .buffer_snapshot()
            .summary_for_anchor::<language::Point>(&active_debug_lines.first().unwrap().0.start);

        assert_eq!(point.row, 2);
    });

    main_editor.update(cx, |editor, cx| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>(cx).collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines"
        );
    });

    assert!(
        handled_second_stacktrace.load(Ordering::SeqCst),
        "Second stacktrace request handler was not called"
    );

    client
        .fake_event(dap::messages::Events::Continued(dap::ContinuedEvent {
            thread_id: 0,
            all_threads_continued: Some(true),
        }))
        .await;

    cx.run_until_parked();

    second_editor.update(cx, |editor, cx| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>(cx).collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines"
        );
    });

    main_editor.update(cx, |editor, cx| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>(cx).collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines"
        );
    });

    // Clean up
    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();

    main_editor.update(cx, |editor, cx| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>(cx).collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines after session shutdown"
        );
    });

    second_editor.update(cx, |editor, cx| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>(cx).collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines after session shutdown"
        );
    });
}

#[gpui::test]
async fn test_closing_a_window_shuts_down_only_its_debug_sessions(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"main.js": ""}))
        .await;
    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let other_project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let other_workspace = init_test_workspace(&other_project, cx).await;
    let disconnected = Arc::new(AtomicBool::new(false));
    let session = start_debug_session(&workspace, cx, {
        let disconnected = disconnected.clone();
        move |client| {
            let disconnected = disconnected.clone();
            client.on_request::<Disconnect, _>(move |_, arguments| {
                assert_eq!(arguments.terminate_debuggee, Some(true));
                disconnected.store(true, Ordering::SeqCst);
                Ok(())
            });
        }
    })
    .expect("debug session should start");
    let other_disconnected = Arc::new(AtomicBool::new(false));
    let other_session = start_debug_session(&other_workspace, cx, {
        let other_disconnected = other_disconnected.clone();
        move |client| {
            let other_disconnected = other_disconnected.clone();
            client.on_request::<Disconnect, _>(move |_, arguments| {
                assert_eq!(arguments.terminate_debuggee, Some(true));
                other_disconnected.store(true, Ordering::SeqCst);
                Ok(())
            });
        }
    })
    .expect("other debug session should start");

    let prepare = workspace
        .update(cx, |multi_workspace, window, cx| {
            multi_workspace.workspace().update(cx, |workspace, cx| {
                workspace.prepare_to_close(workspace::CloseIntent::CloseWindow, window, cx)
            })
        })
        .expect("workspace window should exist");
    assert!(prepare.await.expect("preparing the window should succeed"));
    cx.run_until_parked();
    assert!(!disconnected.load(Ordering::SeqCst));
    assert!(!other_disconnected.load(Ordering::SeqCst));

    workspace
        .update(cx, |_, window, _| window.remove_window())
        .expect("workspace window should close");
    cx.run_until_parked();
    assert!(disconnected.load(Ordering::SeqCst));
    assert!(session.read_with(cx, |session, _| session.is_terminated()));
    assert!(!other_disconnected.load(Ordering::SeqCst));
    assert!(!other_session.read_with(cx, |session, _| session.is_terminated()));
    other_workspace
        .update(cx, |_, window, _| window.remove_window())
        .expect("other workspace window should close");
    cx.run_until_parked();
    assert!(other_disconnected.load(Ordering::SeqCst));
}

#[gpui::test]
async fn test_debug_adapters_shutdown_on_app_quit(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let disconnect_request_received = Arc::new(AtomicBool::new(false));
    let disconnect_clone = disconnect_request_received.clone();

    client.on_request::<Disconnect, _>(move |_, _| {
        disconnect_clone.store(true, Ordering::SeqCst);
        Ok(())
    });

    executor.run_until_parked();

    workspace
        .update(cx, |workspace, _, cx| {
            let panel = workspace.panel::<DebugPanel>(cx).unwrap();
            panel.read_with(cx, |panel, _| {
                assert!(
                    panel.sessions().next().is_some(),
                    "Debug session should be active"
                );
            });
        })
        .unwrap();

    cx.update(|_, cx| cx.defer(|cx| cx.shutdown()));

    executor.run_until_parked();

    assert!(
        disconnect_request_received.load(Ordering::SeqCst),
        "Disconnect request should have been sent to the adapter on app shutdown"
    );
}

#[gpui::test]
async fn test_breakpoint_jumps_only_in_proper_split_view(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
            "second.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let project_path = Path::new(path!("/project"));
    let worktree = project
        .update(cx, |project, cx| project.find_worktree(project_path, cx))
        .expect("This worktree should exist in project")
        .0;

    let worktree_id = workspace
        .update(cx, |_, _, cx| worktree.read(cx).id())
        .unwrap();

    // Open main.rs in pane A (the initial pane)
    let pane_a = workspace
        .update(cx, |multi, _window, cx| {
            multi.workspace().read(cx).active_pane().clone()
        })
        .unwrap();

    let open_main = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
            })
        })
        .unwrap();
    open_main.await.unwrap();

    cx.run_until_parked();

    // Split pane A to the right, creating pane B
    let pane_b = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.split_pane(pane_a.clone(), SplitDirection::Right, window, cx)
            })
        })
        .unwrap();

    cx.run_until_parked();

    // Open main.rs in pane B
    let weak_pane_b = pane_b.downgrade();
    let open_main_in_b = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("main.rs")),
                    Some(weak_pane_b),
                    true,
                    window,
                    cx,
                )
            })
        })
        .unwrap();
    open_main_in_b.await.unwrap();

    cx.run_until_parked();

    // Also open second.rs in pane B as an inactive tab
    let weak_pane_b = pane_b.downgrade();
    let open_second_in_b = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("second.rs")),
                    Some(weak_pane_b),
                    true,
                    window,
                    cx,
                )
            })
        })
        .unwrap();
    open_second_in_b.await.unwrap();

    cx.run_until_parked();

    // Switch pane B back to main.rs so second.rs is inactive there
    let weak_pane_b = pane_b.downgrade();
    let reactivate_main_in_b = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("main.rs")),
                    Some(weak_pane_b),
                    true,
                    window,
                    cx,
                )
            })
        })
        .unwrap();
    reactivate_main_in_b.await.unwrap();

    cx.run_until_parked();

    // Now open second.rs in pane A, making main.rs an inactive tab there
    let weak_pane_a = pane_a.downgrade();
    let open_second = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("second.rs")),
                    Some(weak_pane_a),
                    true,
                    window,
                    cx,
                )
            })
        })
        .unwrap();
    open_second.await.unwrap();

    cx.run_until_parked();

    // Layout:
    //   Pane A: second.rs (active), main.rs (inactive tab)
    //   Pane B: main.rs (active), second.rs (inactive tab)

    // Verify pane A's active item is second.rs (main.rs is an inactive tab)
    workspace
        .read_with(cx, |_multi, cx| {
            let active = pane_a.read(cx).active_item().unwrap();
            let editor = active.to_any_view().downcast::<Editor>().unwrap();
            let path = editor.read(cx).active_project_path(cx).unwrap();
            assert_eq!(
                path.path.file_name().unwrap(),
                "second.rs",
                "Pane A should have second.rs active",
            );
        })
        .unwrap();

    // Verify pane B's active item is main.rs
    workspace
        .read_with(cx, |_multi, cx| {
            let active = pane_b.read(cx).active_item().unwrap();
            let editor = active.to_any_view().downcast::<Editor>().unwrap();
            let path = editor.read(cx).active_project_path(cx).unwrap();
            assert_eq!(
                path.path.file_name().unwrap(),
                "main.rs",
                "Pane B should have main.rs active",
            );
        })
        .unwrap();

    // Start a debug session and trigger a breakpoint stop on main.rs line 2
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

    client.on_request::<dap::requests::Scopes, _>(move |_, _| {
        Ok(dap::ScopesResponse {
            scopes: Vec::default(),
        })
    });

    client.on_request::<StackTrace, _>(move |_, args| {
        assert_eq!(args.thread_id, 1);

        Ok(dap::StackTraceResponse {
            stack_frames: vec![dap::StackFrame {
                id: 1,
                name: "frame 1".into(),
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
                line: 2,
                column: 0,
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

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    // After first breakpoint stop on main.rs:
    // Pane A should still have second.rs as its active item because
    // main.rs was only an inactive tab there. The debugger should have jumped
    // to main.rs only in pane B where it was already the active tab.
    workspace
        .read_with(cx, |_multi, cx| {
            let pane_a_active = pane_a.read(cx).active_item().unwrap();
            let pane_a_editor = pane_a_active.to_any_view().downcast::<Editor>().unwrap();
            let pane_a_path = pane_a_editor.read(cx).active_project_path(cx).unwrap();
            assert_eq!(
                pane_a_path.path.file_name().unwrap(),
                "second.rs",
                "Pane A should still have second.rs as active item. \
                 The debugger should not switch active tabs in panes where the \
                 breakpoint file is not the active tab (issue #40602)",
            );
        })
        .unwrap();

    // There should be exactly one active debug line across all editors in all panes
    workspace
        .read_with(cx, |_multi, cx| {
            let mut total_active_debug_lines = 0;
            for pane in [&pane_a, &pane_b] {
                for item in pane.read(cx).items() {
                    if let Some(editor) = item.to_any_view().downcast::<Editor>().ok() {
                        total_active_debug_lines += editor
                            .read(cx)
                            .highlighted_rows::<ActiveDebugLine>(cx)
                            .count();
                    }
                }
            }
            assert_eq!(
                total_active_debug_lines, 1,
                "There should be exactly one active debug line across all editors in all panes"
            );
        })
        .unwrap();

    // Pane B should show the debug highlight on main.rs
    workspace
        .read_with(cx, |_multi, cx| {
            let pane_b_active = pane_b.read(cx).active_item().unwrap();
            let pane_b_editor = pane_b_active.to_any_view().downcast::<Editor>().unwrap();

            let active_debug_lines: Vec<_> = pane_b_editor
                .read(cx)
                .highlighted_rows::<ActiveDebugLine>(cx)
                .collect();

            assert_eq!(
                active_debug_lines.len(),
                1,
                "Pane B's main.rs editor should have the active debug line"
            );
        })
        .unwrap();

    // Second breakpoint stop: now on second.rs line 3.
    // Even though pane A has second.rs as its active tab, the debug line
    // should open in pane B (the persistent debug pane) because pane B
    // had the last active debug line.
    client.on_request::<StackTrace, _>(move |_, args| {
        assert_eq!(args.thread_id, 1);

        Ok(dap::StackTraceResponse {
            stack_frames: vec![dap::StackFrame {
                id: 2,
                name: "frame 2".into(),
                source: Some(dap::Source {
                    name: Some("second.rs".into()),
                    path: Some(path!("/project/second.rs").into()),
                    source_reference: None,
                    presentation_hint: None,
                    origin: None,
                    sources: None,
                    adapter_data: None,
                    checksums: None,
                }),
                line: 3,
                column: 0,
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

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    // Pane B should now have second.rs as the active tab with the debug line,
    // because pane B was the last pane that had the debug line (persistent debug pane).
    workspace
        .read_with(cx, |_multi, cx| {
            let pane_b_active = pane_b.read(cx).active_item().unwrap();
            let pane_b_editor = pane_b_active.to_any_view().downcast::<Editor>().unwrap();
            let pane_b_path = pane_b_editor.read(cx).active_project_path(cx).unwrap();
            assert_eq!(
                pane_b_path.path.file_name().unwrap(),
                "second.rs",
                "Pane B should have switched to second.rs because it is the persistent debug pane",
            );

            let active_debug_lines: Vec<_> = pane_b_editor
                .read(cx)
                .highlighted_rows::<ActiveDebugLine>(cx)
                .collect();

            assert_eq!(
                active_debug_lines.len(),
                1,
                "Pane B's second.rs editor should have the active debug line"
            );
        })
        .unwrap();

    // There should still be exactly one active debug line across all editors
    workspace
        .read_with(cx, |_multi, cx| {
            let mut total_active_debug_lines = 0;
            for pane in [&pane_a, &pane_b] {
                for item in pane.read(cx).items() {
                    if let Some(editor) = item.to_any_view().downcast::<Editor>().ok() {
                        total_active_debug_lines += editor
                            .read(cx)
                            .highlighted_rows::<ActiveDebugLine>(cx)
                            .count();
                    }
                }
            }
            assert_eq!(
                total_active_debug_lines, 1,
                "There should be exactly one active debug line across all editors after second stop"
            );
        })
        .unwrap();

    // === New case: Move the debug pane (pane B) active item to a new pane C ===
    // This simulates a user dragging the tab with the active debug line to a new split.
    // The debugger should track that the debug line moved to pane C and use pane C
    // for subsequent debug stops.

    // Split pane B to create pane C
    let pane_c = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.split_pane(pane_b.clone(), SplitDirection::Right, window, cx)
            })
        })
        .unwrap();

    cx.run_until_parked();

    // Move the active item (second.rs with debug line) from pane B to pane C
    workspace
        .update(cx, |_multi, window, cx| {
            move_active_item(&pane_b, &pane_c, true, false, window, cx);
        })
        .unwrap();

    cx.run_until_parked();

    // Verify pane C now has second.rs as active item
    workspace
        .read_with(cx, |_multi, cx| {
            let pane_c_active = pane_c.read(cx).active_item().unwrap();
            let pane_c_editor = pane_c_active.to_any_view().downcast::<Editor>().unwrap();
            let pane_c_path = pane_c_editor.read(cx).active_project_path(cx).unwrap();
            assert_eq!(
                pane_c_path.path.file_name().unwrap(),
                "second.rs",
                "Pane C should have second.rs after moving it from pane B",
            );
        })
        .unwrap();

    // Third breakpoint stop: back on main.rs line 2.
    // The debug line should appear in pane C because that's where the debug line
    // was moved to. The debugger should track pane moves.
    client.on_request::<StackTrace, _>(move |_, args| {
        assert_eq!(args.thread_id, 1);

        Ok(dap::StackTraceResponse {
            stack_frames: vec![dap::StackFrame {
                id: 3,
                name: "frame 3".into(),
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
                line: 2,
                column: 0,
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

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    // Pane C should now have main.rs as the active tab with the debug line,
    // because pane C is where the debug line was moved to from pane B.
    workspace
        .read_with(cx, |_multi, cx| {
            let pane_c_active = pane_c.read(cx).active_item().unwrap();
            let pane_c_editor = pane_c_active.to_any_view().downcast::<Editor>().unwrap();
            let pane_c_path = pane_c_editor.read(cx).active_project_path(cx).unwrap();
            assert_eq!(
                pane_c_path.path.file_name().unwrap(),
                "main.rs",
                "Pane C should have switched to main.rs because it is now the persistent debug pane \
                 (the debug line was moved here from pane B)",
            );

            let active_debug_lines: Vec<_> = pane_c_editor
                .read(cx)
                .highlighted_rows::<ActiveDebugLine>(cx)
                .collect();

            assert_eq!(
                active_debug_lines.len(),
                1,
                "Pane C's main.rs editor should have the active debug line"
            );
        })
        .unwrap();

    // There should still be exactly one active debug line across all editors
    workspace
        .read_with(cx, |_multi, cx| {
            let mut total_active_debug_lines = 0;
            for pane in [&pane_a, &pane_b, &pane_c] {
                for item in pane.read(cx).items() {
                    if let Some(editor) = item.to_any_view().downcast::<Editor>().ok() {
                        total_active_debug_lines += editor
                            .read(cx)
                            .highlighted_rows::<ActiveDebugLine>(cx)
                            .count();
                    }
                }
            }
            assert_eq!(
                total_active_debug_lines, 1,
                "There should be exactly one active debug line across all editors after third stop"
            );
        })
        .unwrap();

    // Clean up
    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_terminated_event_disconnects_without_terminating_debuggee(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"main.js": ""}))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    for supports_terminate_request in [false, true] {
        let disconnected = Arc::new(AtomicBool::new(false));
        let session = start_debug_session(&workspace, cx, {
            let disconnected = disconnected.clone();
            move |client| {
                client.on_request::<dap::requests::Initialize, _>(move |_, _| {
                    Ok(dap::Capabilities {
                        supports_terminate_request: Some(supports_terminate_request),
                        ..Default::default()
                    })
                });
                let disconnected = disconnected.clone();
                client.on_request::<Disconnect, _>(move |_, arguments| {
                    assert_eq!(arguments.terminate_debuggee, Some(false));
                    disconnected.store(true, Ordering::SeqCst);
                    Ok(())
                });
                client.on_request::<Terminate, _>(|_, _| {
                    panic!("A terminated event must not trigger a terminate request")
                });
            }
        })
        .expect("debug session should start");
        let client = session.update(cx, |session, _| {
            session.adapter_client().expect("running debug adapter")
        });

        client
            .fake_event(dap::messages::Events::Terminated(Some(
                dap::TerminatedEvent { restart: None },
            )))
            .await;
        cx.run_until_parked();

        assert!(disconnected.load(Ordering::SeqCst));
        project.read_with(cx, |project, cx| {
            assert!(session.read(cx).is_terminated());
            assert!(
                project
                    .dap_store()
                    .read(cx)
                    .session_by_id(session.read(cx).session_id())
                    .is_none()
            );
        });
    }
}

#[gpui::test]
async fn test_last_child_termination_allows_new_child_session(cx: &mut TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"main.js": ""}))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let parent_session =
        start_debug_session(&workspace, cx, |_| {}).expect("parent debug session should start");
    let parent_session_id = cx.read(|cx| parent_session.read(cx).session_id());
    let parent_client = parent_session.update(cx, |session, _| {
        session.adapter_client().expect("running parent adapter")
    });
    parent_client.on_request::<Disconnect, _>(|_, _| {
        panic!("Ending a child session must not disconnect the parent")
    });
    parent_client.on_response::<StartDebugging, _>(|_| {}).await;
    let _subscription = project::debugger::test::intercept_debug_sessions(cx, |_| {});

    for child_session_id in [SessionId(1), SessionId(2)] {
        parent_client
            .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
                configuration: json!({"type": "pwa-chrome"}),
                request: StartDebuggingRequestArgumentsRequest::Launch,
            })
            .await;
        cx.run_until_parked();

        let child_session = project.read_with(cx, |project, cx| {
            project
                .dap_store()
                .read(cx)
                .session_by_id(child_session_id)
                .expect("new child session should start")
        });
        child_session.read_with(cx, |session, cx| {
            assert!(!session.is_terminated());
            assert_eq!(session.parent_id(cx), Some(parent_session_id));
        });
        let child_client = child_session.update(cx, |session, _| {
            session.adapter_client().expect("running child adapter")
        });
        child_client
            .fake_event(dap::messages::Events::Terminated(None))
            .await;
        cx.run_until_parked();

        project.read_with(cx, |project, cx| {
            let store = project.dap_store().read(cx);
            assert!(store.session_by_id(child_session_id).is_none());
            assert!(store.session_by_id(parent_session_id).is_some());
            assert!(!parent_session.read(cx).is_terminated());
            assert!(parent_session.read(cx).child_session_ids().is_empty());
        });
    }

    parent_client.on_request::<Disconnect, _>(|_, arguments| {
        assert_eq!(arguments.terminate_debuggee, Some(true));
        Ok(())
    });
    project
        .update(cx, |project, cx| {
            project.dap_store().update(cx, |store, cx| {
                store.shutdown_session(parent_session_id, cx)
            })
        })
        .await
        .expect("explicit parent shutdown should succeed");
    assert!(parent_session.read_with(cx, |session, _| session.is_terminated()));
}

#[gpui::test]
async fn test_terminated_parent_disconnects_children_without_terminating_debuggee(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/project"), json!({"main.js": ""}))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let parent_session =
        start_debug_session(&workspace, cx, |_| {}).expect("parent debug session should start");
    let parent_session_id = cx.read(|cx| parent_session.read(cx).session_id());
    let parent_client = parent_session.update(cx, |session, _| {
        session.adapter_client().expect("running parent adapter")
    });
    parent_client.on_request::<Disconnect, _>(|_, _| {
        panic!("Ending a child session must not disconnect the root session")
    });
    parent_client.on_response::<StartDebugging, _>(|_| {}).await;
    let disconnected = Arc::new(AtomicUsize::new(0));
    let _subscription = project::debugger::test::intercept_debug_sessions(cx, {
        let disconnected = disconnected.clone();
        move |client| {
            let disconnected = disconnected.clone();
            client.on_request::<Disconnect, _>(move |_, arguments| {
                assert_eq!(arguments.terminate_debuggee, Some(false));
                disconnected.fetch_add(1, Ordering::SeqCst);
                Ok(())
            });
        }
    });

    parent_client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;
    cx.run_until_parked();
    let child_session = project.read_with(cx, |project, cx| {
        project
            .dap_store()
            .read(cx)
            .session_by_id(SessionId(1))
            .expect("child session should start")
    });
    let child_client = child_session.update(cx, |session, _| {
        session.adapter_client().expect("running child adapter")
    });
    child_client.on_response::<StartDebugging, _>(|_| {}).await;
    child_client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;
    cx.run_until_parked();

    child_client
        .fake_event(dap::messages::Events::Terminated(None))
        .await;
    cx.run_until_parked();

    assert_eq!(disconnected.load(Ordering::SeqCst), 2);
    project.read_with(cx, |project, cx| {
        let store = project.dap_store().read(cx);
        assert!(store.session_by_id(SessionId(1)).is_none());
        assert!(store.session_by_id(SessionId(2)).is_none());
        assert!(store.session_by_id(parent_session_id).is_some());
        assert!(!parent_session.read(cx).is_terminated());
        assert!(parent_session.read(cx).child_session_ids().is_empty());
    });

    parent_client.on_request::<Disconnect, _>(|_, _| Ok(()));
    project
        .update(cx, |project, cx| {
            project.dap_store().update(cx, |store, cx| {
                store.shutdown_session(parent_session_id, cx)
            })
        })
        .await
        .expect("explicit parent shutdown should succeed");
}

#[gpui::test]
async fn test_adapter_shutdown_with_child_sessions_on_app_quit(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let parent_session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let parent_session_id = cx.read(|cx| parent_session.read(cx).session_id());
    let parent_client = parent_session.update(cx, |session, _| session.adapter_client().unwrap());

    let disconnect_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let parent_disconnect_called = Arc::new(AtomicBool::new(false));
    let parent_disconnect_clone = parent_disconnect_called.clone();
    let disconnect_count_clone = disconnect_count.clone();

    parent_client.on_request::<Disconnect, _>(move |_, _| {
        parent_disconnect_clone.store(true, Ordering::SeqCst);
        disconnect_count_clone.fetch_add(1, Ordering::SeqCst);

        for _ in 0..50 {
            if disconnect_count_clone.load(Ordering::SeqCst) >= 2 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        Ok(())
    });

    parent_client
        .on_response::<StartDebugging, _>(move |_| {})
        .await;
    let _subscription = project::debugger::test::intercept_debug_sessions(cx, |_| {});

    parent_client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    let child_session = project.update(cx, |project, cx| {
        project
            .dap_store()
            .read(cx)
            .session_by_id(SessionId(1))
            .unwrap()
    });
    let child_session_id = cx.read(|cx| child_session.read(cx).session_id());
    let child_client = child_session.update(cx, |session, _| session.adapter_client().unwrap());

    let child_disconnect_called = Arc::new(AtomicBool::new(false));
    let child_disconnect_clone = child_disconnect_called.clone();
    let disconnect_count_clone = disconnect_count.clone();

    child_client.on_request::<Disconnect, _>(move |_, _| {
        child_disconnect_clone.store(true, Ordering::SeqCst);
        disconnect_count_clone.fetch_add(1, Ordering::SeqCst);

        for _ in 0..50 {
            if disconnect_count_clone.load(Ordering::SeqCst) >= 2 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        Ok(())
    });

    executor.run_until_parked();

    project.update(cx, |project, cx| {
        let store = project.dap_store().read(cx);
        assert!(store.session_by_id(parent_session_id).is_some());
        assert!(store.session_by_id(child_session_id).is_some());
    });

    cx.update(|_, cx| cx.defer(|cx| cx.shutdown()));

    executor.run_until_parked();

    let parent_disconnect_check = parent_disconnect_called.clone();
    let child_disconnect_check = child_disconnect_called.clone();
    let executor_clone = executor.clone();
    let both_disconnected = executor
        .spawn(async move {
            let parent_disconnect = parent_disconnect_check;
            let child_disconnect = child_disconnect_check;

            // We only have 100ms to shutdown the app
            for _ in 0..100 {
                if parent_disconnect.load(Ordering::SeqCst)
                    && child_disconnect.load(Ordering::SeqCst)
                {
                    return true;
                }

                executor_clone
                    .timer(std::time::Duration::from_millis(1))
                    .await;
            }

            false
        })
        .await;

    assert!(
        both_disconnected,
        "Both parent and child sessions should receive disconnect requests"
    );

    assert!(
        parent_disconnect_called.load(Ordering::SeqCst),
        "Parent session should have received disconnect request"
    );
    assert!(
        child_disconnect_called.load(Ordering::SeqCst),
        "Child session should have received disconnect request"
    );
}

#[gpui::test]
async fn test_restart_request_is_not_sent_more_than_once_until_response(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, move |client| {
        client.on_request::<dap::requests::Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_restart_request: Some(true),
                ..Default::default()
            })
        });
    })
    .unwrap();

    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let restart_count = Arc::new(AtomicUsize::new(0));

    client.on_request::<dap::requests::Restart, _>({
        let restart_count = restart_count.clone();
        move |_, _| {
            restart_count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    });

    // This works because the restart request sender is on the foreground thread
    // so it will start running after the gpui update stack is cleared
    session.update(cx, |session, cx| {
        session.restart(None, cx);
        session.restart(None, cx);
        session.restart(None, cx);
    });

    cx.run_until_parked();

    assert_eq!(
        restart_count.load(Ordering::SeqCst),
        1,
        "Only one restart request should be sent while a restart is in-flight"
    );

    session.update(cx, |session, cx| {
        session.restart(None, cx);
    });

    cx.run_until_parked();

    assert_eq!(
        restart_count.load(Ordering::SeqCst),
        2,
        "A second restart should be allowed after the first one completes"
    );
}

#[gpui::test]
async fn test_debugger_terminal_pane_has_search_bar(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    cx.update(|cx| {
        cx.set_global(workspace::PaneSearchBarCallbacks {
            setup_search_bar: |languages, toolbar, window, cx| {
                let search_bar = cx.new(|cx| search::BufferSearchBar::new(languages, window, cx));
                toolbar.update(cx, |toolbar, cx| {
                    toolbar.add_item(search_bar, window, cx);
                });
            },
            wrap_div_with_search_actions: search::buffer_search::register_pane_search_actions,
        });
    });

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(path!("/project"), json!({"main.rs": ""}))
        .await;
    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "main".into(),
            }],
        })
    });
    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![],
            total_frames: None,
        })
    });

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;
    cx.run_until_parked();

    workspace
        .update(cx, |workspace, _, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let session = debug_panel
                .read(cx)
                .active_session()
                .expect("active session");
            let running_state = session.read(cx).running_state().clone();
            let active_pane = running_state.read(cx).active_pane().clone();
            let search_bar = active_pane
                .read(cx)
                .toolbar()
                .read(cx)
                .item_of_type::<search::BufferSearchBar>();
            assert!(
                search_bar.is_some(),
                "active debugger pane toolbar should have BufferSearchBar for cmd-f support"
            );
        })
        .unwrap();
}
