//! S1's last exit (ARCHITECTURE §22): an agent breakpoint round-trips
//! through Zed's own debugger. The real panel and Debug panel in one
//! workspace, Zed's fake debug adapter, and fake-omp replaying
//! `debug_round_trip.jsonl` (a hand-made turn: OMP asks the person before
//! each `cedian_debug` call, as `cedian_debug: prompt` makes it). Checked:
//!
//! 1. the agent's breakpoint is in Zed's `BreakpointStore` (the gutter's)
//!    and reaches the adapter's SetBreakpoints;
//! 2. `start` (approved) launches the debug.json scenario in the Debug
//!    panel, and its reply is the stop: `src/main.rs:3`, `x = 42`;
//! 3. the Debug panel's active session is the agent's session;
//! 4. the agent's `continue` reaches the adapter;
//! 5. the person continues from the Debug panel, and the agent reads that
//!    the session runs;
//! 6. refused: an unknown scenario (no adapter starts), `../x.c`, and a
//!    `continue` on a person-started session the person denies (no
//!    Continue reaches its adapter).
//!
//! fake-omp checks each reply's text whole and exits on any difference.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

// Zed's fake adapter handlers return `dap::ErrorResponse`, as its own tests do.
#![expect(clippy::result_large_err)]

use cedian_panel::{CedianPanel, Connection, Turn};
use dap::client::DebugAdapterClient;
use debugger_ui::debugger_panel::DebugPanel;
use gpui::{AppContext as _, Entity, Modifiers, TestAppContext, VisualTestContext, WindowHandle};
use project::Project;
use project::debugger::session::Session;
use settings::SettingsStore;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use workspace::Workspace;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/debug_round_trip.jsonl"
);
const MAIN: &str = "fn main() {\n    let x = 42;\n    println!(\"{x}\");\n}\n";
const DEBUG_JSON: &str =
    r#"[{"label": "main", "adapter": "fake-adapter", "request": "launch", "program": "main"}]"#;
/// The dialog the person denies: the agent's `continue` on their session.
const DENIED_DIALOG: usize = 7;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        _ => {}
    }
    let root = std::env::temp_dir().join(format!("cedian-s1-dap-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("ws/src")).unwrap();
    std::fs::create_dir_all(root.join("ws/.zed")).unwrap();
    std::fs::write(root.join("ws/src/main.rs"), MAIN).unwrap();
    std::fs::write(root.join("ws/.zed/debug.json"), DEBUG_JSON).unwrap();
    std::fs::write(root.join("x.c"), "int main(void) { return 0; }\n").unwrap();
    std::fs::write(root.join("cedian.toml"), "schema = 1\n").unwrap();
    let root = root.canonicalize().unwrap();
    // SAFETY: single-threaded here; nothing else reads the environment yet.
    unsafe {
        std::env::set_var("CEDIAN_CONFIG", root.join("cedian.toml"));
        std::env::set_var("CEDIAN_STATE_DIR", root.join("state"));
        std::env::set_var("HOME", root.join("home"));
        std::env::set_var("CEDIAN_OMP_BINARY", std::env::current_exe().unwrap());
    }
    print!("test agent_breakpoint_round_trips_through_zeds_debugger ... ");
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("debug_round_trip"));
            gpui::ForegroundExecutor::new(exec).block_test(scenario(&mut cx, &root));
            cx.run_until_parked();
            cx.update(|cx| cx.quit());
            cx.run_until_parked();
            dispatcher.drain_tasks();
            let _ = std::fs::remove_dir_all(&root);
        }),
    );
    println!("ok");
}

/// What the fake adapters saw, across every session.
#[derive(Default)]
struct Seen {
    /// Lines (1-based) the last SetBreakpoints for `src/main.rs` carried.
    breakpoints: parking_lot::Mutex<Vec<u64>>,
    /// Continue requests, by session order (0 = the agent's, 1 = the person's).
    continues: [AtomicUsize; 2],
    sessions: AtomicUsize,
}

async fn scenario(cx: &mut TestAppContext, root: &Path) {
    cx.executor().allow_parking();
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
        command_palette_hooks::init(cx);
        terminal_view::init(cx);
        debugger_ui::init(cx);
        dap_adapters::init(cx);
    });
    let ws = root.join("ws");
    let main_rs = ws.join("src/main.rs");
    let state = cedian_shell::state::dir(&ws).unwrap();
    cedian_fake_omp::install_replay(&state.join("omp"), Path::new(FIXTURE)).unwrap();

    let seen = Arc::new(Seen::default());
    let _adapters = project::debugger::test::intercept_debug_sessions(cx, {
        let seen = seen.clone();
        let main_rs = main_rs.clone();
        move |client| fake_adapter(client, &seen, &main_rs)
    });

    let project = Project::test(fs::RealFs::new(None, cx.executor()), [ws.as_path()], cx).await;
    let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let debug_panel = window
        .update(cx, |_, window, cx| {
            cx.spawn_in(window, async move |this, cx| {
                DebugPanel::load(this, cx).await
            })
        })
        .unwrap()
        .await
        .unwrap();
    let panel = window
        .update(cx, |workspace, window, cx| {
            workspace.add_panel(debug_panel.clone(), window, cx);
            let handle = cx.entity().downgrade();
            let panel = cx.new(|cx| {
                let mut panel = CedianPanel::new(project.clone(), window, cx);
                panel.set_workspace(handle, cx);
                panel
            });
            workspace.add_panel(panel.clone(), window, cx);
            workspace.open_panel::<CedianPanel>(window, cx);
            panel
        })
        .unwrap();
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    wait(cx, &panel, "OMP ready", |p| {
        matches!(p.connection(), Connection::Ready { .. })
    });
    cx.update_window(window.into(), |_, window, cx| {
        panel.update(cx, |panel, cx| {
            panel.set_prompt("Debug main", window, cx);
            panel.submit(window, cx);
        })
    })
    .unwrap();

    let mut answered = 0;
    let mut stops = 0;
    let mut person: Option<Entity<Session>> = None;
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        cx.run_until_parked();
        let (idle, dialogs) = panel.read_with(cx, |p, _| (p.turn() == &Turn::Idle, p.dialog_ids()));
        if idle && answered == DENIED_DIALOG {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "stuck after {answered} dialogs, {stops} stops: {}",
            panel.read_with(cx, |p, _| format!(
                "{:?} · {:?}",
                p.connection(),
                p.notice()
            ))
        );
        let agent = sessions(&project, cx).first().cloned();

        // The adapter stops at the agent's breakpoint after launch, and
        // again after the agent's continue.
        if let Some(session) = &agent {
            let want = match stops {
                0 => seen.breakpoints.lock().contains(&3),
                1 => seen.continues[0].load(Ordering::SeqCst) == 1,
                _ => false,
            };
            if want {
                stop(session, cx).await;
                stops += 1;
                continue;
            }
        }

        if let Some(id) = dialogs.first().cloned() {
            let n = answered + 1;
            if n == 2 {
                assert!(agent.is_none(), "no session before the approved start");
            }
            if n == 3 {
                let session = agent.clone().expect("the agent's session");
                // 2. The breakpoint is the editor's, and the adapter has it.
                let store = project.read_with(cx, |p, _| p.breakpoint_store());
                let rows: Vec<u32> = store.read_with(cx, |s, cx| {
                    s.all_source_breakpoints(cx)
                        .get(main_rs.as_path())
                        .map(|bps| bps.iter().map(|b| b.row).collect())
                        .unwrap_or_default()
                });
                assert_eq!(rows, [2], "the agent's breakpoint is in Zed's store");
                assert_eq!(*seen.breakpoints.lock(), [3], "and reached SetBreakpoints");
                // 3. The Debug panel shows the agent's session.
                let shown = debug_panel.read_with(cx, |panel, cx| {
                    panel
                        .active_session()
                        .map(|s| s.read(cx).running_state().read(cx).session().clone())
                });
                assert_eq!(shown.as_ref(), Some(&session), "the Debug panel's session");
            }
            if n == 4 {
                // 4. The agent's continue reached the adapter. 5. The person
                // continues from the Debug panel before the agent reads.
                assert_eq!(seen.continues[0].load(Ordering::SeqCst), 1);
                debug_panel.update(cx, |panel, cx| {
                    let running = panel
                        .active_session()
                        .unwrap()
                        .read(cx)
                        .running_state()
                        .clone();
                    running.update(cx, |r, cx| r.continue_thread(cx));
                });
                cx.run_until_parked();
                let session = agent.clone().unwrap();
                assert!(!session.read_with(cx, |s, _| s.any_stopped_thread()));
            }
            if n == 6 {
                assert_eq!(
                    sessions(&project, cx).len(),
                    1,
                    "the unknown scenario started no adapter"
                );
            }
            if n == DENIED_DIALOG {
                person = Some(person_session(&window, cx).await);
            }
            let choice = if n == DENIED_DIALOG {
                "Deny"
            } else {
                "Approve"
            };
            let button: &'static str = format!("cedian-dialog-{id}-{choice}").leak();
            click(&mut vcx, button);
            answered = n;
            continue;
        }
        std::thread::sleep(Duration::from_millis(20));
    }

    let (connection, notice) = panel.read_with(cx, |p, _| {
        (p.connection().clone(), p.notice().map(str::to_string))
    });
    assert!(
        matches!(connection, Connection::Ready { .. }) && notice.is_none(),
        "fake-omp replayed to the end (no divergence): {connection:?} · {notice:?}"
    );
    assert!(person.is_some());
    assert_eq!(
        seen.continues[0].load(Ordering::SeqCst),
        2,
        "the agent's continue and the person's"
    );
    assert_eq!(
        seen.continues[1].load(Ordering::SeqCst),
        0,
        "the denied continue never reached the person's session"
    );
}

/// Zed's fake adapter, answering as a program stopped in `main` at line 3
/// with `x = 42`.
fn fake_adapter(client: &Arc<DebugAdapterClient>, seen: &Arc<Seen>, main_rs: &Path) {
    let order = seen.sessions.fetch_add(1, Ordering::SeqCst).min(1);
    let main_rs: PathBuf = main_rs.to_path_buf();
    client.on_request::<dap::requests::Initialize, _>(|_, _| {
        Ok(dap::Capabilities {
            supports_single_thread_execution_requests: Some(true),
            ..Default::default()
        })
    });
    client.on_request::<dap::requests::SetBreakpoints, _>({
        let seen = seen.clone();
        let main_rs = main_rs.clone();
        move |_, args| {
            if args.source.path.as_deref() == Some(main_rs.to_str().unwrap()) {
                *seen.breakpoints.lock() = args
                    .breakpoints
                    .unwrap_or_default()
                    .iter()
                    .map(|b| b.line)
                    .collect();
            }
            Ok(dap::SetBreakpointsResponse {
                breakpoints: Vec::new(),
            })
        }
    });
    client.on_request::<dap::requests::Threads, _>(|_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "main".into(),
            }],
        })
    });
    client.on_request::<dap::requests::Continue, _>({
        let seen = seen.clone();
        move |_, _| {
            seen.continues[order].fetch_add(1, Ordering::SeqCst);
            Ok(dap::ContinueResponse {
                all_threads_continued: Some(true),
            })
        }
    });
    client.on_request::<dap::requests::StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: vec![dap::StackFrame {
                id: 1,
                name: "main".into(),
                source: Some(dap::Source {
                    name: Some("main.rs".into()),
                    path: Some(main_rs.to_string_lossy().into_owned()),
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
    client.on_request::<dap::requests::Scopes, _>(|_, _| {
        Ok(dap::ScopesResponse {
            scopes: vec![dap::Scope {
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
    client.on_request::<dap::requests::Variables, _>(|_, _| {
        Ok(dap::VariablesResponse {
            variables: vec![dap::Variable {
                name: "x".into(),
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
            }],
        })
    });
}

async fn stop(session: &Entity<Session>, cx: &mut TestAppContext) {
    let client = session.read_with(cx, |s, _| s.adapter_client().unwrap());
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
}

/// The project's debug sessions, oldest first.
fn sessions(project: &Entity<Project>, cx: &mut TestAppContext) -> Vec<Entity<Session>> {
    project.read_with(cx, |p, cx| {
        let mut sessions: Vec<_> = p.dap_store().read(cx).sessions().cloned().collect();
        sessions.sort_by_key(|s| s.read(cx).session_id().0);
        sessions
    })
}

/// The person launches the scenario from the Debug panel themselves.
async fn person_session(
    window: &WindowHandle<Workspace>,
    cx: &mut TestAppContext,
) -> Entity<Session> {
    let scenario = task::DebugScenario {
        adapter: "fake-adapter".into(),
        label: "mine".into(),
        build: None,
        config: serde_json::json!({"request": "launch", "program": "main"}),
        tcp_connection: None,
    };
    window
        .update(cx, |workspace, window, cx| {
            workspace.start_debug_session(
                scenario,
                task::SharedTaskContext::default(),
                None,
                None,
                window,
                cx,
            )
        })
        .unwrap();
    cx.run_until_parked();
    let session = window
        .update(cx, |workspace, _, cx| {
            let mut all: Vec<_> = workspace
                .project()
                .read(cx)
                .dap_store()
                .read(cx)
                .sessions()
                .cloned()
                .collect();
            all.sort_by_key(|s| s.read(cx).session_id().0);
            all.last().cloned()
        })
        .unwrap()
        .expect("the person's session");
    session
}

fn click(vcx: &mut VisualTestContext, selector: &'static str) {
    vcx.run_until_parked();
    let bounds = vcx
        .debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} is not on screen"));
    vcx.simulate_click(bounds.center(), Modifiers::none());
}

/// Pump the test executor while real OS threads (OMP) make progress.
fn wait(
    cx: &mut TestAppContext,
    panel: &Entity<CedianPanel>,
    what: &str,
    done: impl Fn(&CedianPanel) -> bool,
) {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        cx.run_until_parked();
        let (finished, state) = panel.read_with(cx, |p, _| {
            (
                done(p),
                format!("{:?} · {:?} · {:?}", p.connection(), p.turn(), p.notice()),
            )
        });
        if finished {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {what}: {state}"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}
