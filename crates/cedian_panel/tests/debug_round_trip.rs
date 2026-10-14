//! S1's last exit (ARCHITECTURE §22): an agent breakpoint round-trips
//! through Zed's own debugger. The real panel and Debug panel in one
//! workspace, Zed's fake debug adapter, and fake-omp replaying
//! `debug_round_trip.jsonl` (a hand-made turn: OMP's own approval select
//! before each call is guessed, and the test approves every one). cedian's
//! own gate (ADR-0073 decision 5) runs on every call. Checked:
//!
//! 1. the agent's breakpoint is in Zed's `BreakpointStore` (the gutter's)
//!    and reaches the adapter's SetBreakpoints; set again with a condition,
//!    the condition changes;
//! 2. `start` opens cedian's own approval dialog, and no adapter starts
//!    before the person approves; then the debug.json scenario launches in
//!    the Debug panel, and its reply is the stop: `src/main.rs:3`, `x = 42`;
//! 3. the Debug panel's active session is the agent's session;
//! 4. the agent's `continue` on its own session is not asked, and reaches
//!    the adapter;
//! 5. the person continues from the Debug panel, and the agent reads that
//!    the session runs;
//! 6. denied by cedian with no dialog: an unknown scenario (no adapter
//!    starts) and `../x.c` (the reason checked);
//! 7. a scenario whose adapter is missing: `start` answers that it did not
//!    start, and binds no other session;
//! 8. the person denies a `start` (no adapter starts) and a `continue` on
//!    their own session (no Continue reaches its adapter);
//! 9. every call has one gate row in the audit log.
//!
//! fake-omp checks each reply's text whole and the host tools registered,
//! and exits on any difference.
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
const DEBUG_JSON: &str = r#"[
  {"label": "main", "adapter": "fake-adapter", "request": "launch", "program": "main"},
  {"label": "ghost", "adapter": "no-such-adapter", "request": "launch", "program": "main"}
]"#;
/// OMP's approval selects in the fixture, one per call.
const OMP_DIALOGS: usize = 10;
/// cedian's own Asks: start main (approved), start ghost (approved), start
/// main again (denied), continue on the person's session (denied).
const CEDIAN_DIALOGS: usize = 4;
/// The OMP select on the person's session: the test starts it then.
const PERSON_CONTINUE: usize = 10;

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

    let mut omp = 0;
    let mut asked = 0;
    let mut stops = 0;
    let mut launched_before_denied_start = None;
    let mut person: Option<Entity<Session>> = None;
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        cx.run_until_parked();
        let (idle, dialogs) = panel.read_with(cx, |p, _| (p.turn() == &Turn::Idle, p.dialog_ids()));
        if idle && omp == OMP_DIALOGS && asked == CEDIAN_DIALOGS {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "stuck after {omp} OMP dialogs, {asked} cedian dialogs, {stops} stops: {}",
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

        let Some(id) = dialogs.first().cloned() else {
            std::thread::sleep(Duration::from_millis(20));
            continue;
        };
        let choice = if id.starts_with("cedian-debug-") {
            // cedian's own Ask.
            asked += 1;
            match asked {
                1 => {
                    assert!(
                        agent.is_none(),
                        "no session before the person approves start"
                    );
                    assert_eq!(seen.sessions.load(Ordering::SeqCst), 0, "no adapter either");
                    "Approve"
                }
                2 => "Approve",
                3 => {
                    launched_before_denied_start = Some(seen.sessions.load(Ordering::SeqCst));
                    "Deny"
                }
                _ => "Deny",
            }
        } else {
            omp += 1;
            if omp == 4 {
                let session = agent.clone().expect("the agent's session");
                // 1. The breakpoint is the editor's, and the adapter has it.
                let store = project.read_with(cx, |p, _| p.breakpoint_store());
                let rows: Vec<(u32, Option<String>)> = store.read_with(cx, |s, cx| {
                    s.all_source_breakpoints(cx)
                        .get(main_rs.as_path())
                        .map(|bps| {
                            bps.iter()
                                .map(|b| (b.row, b.condition.as_deref().map(str::to_string)))
                                .collect()
                        })
                        .unwrap_or_default()
                });
                assert_eq!(
                    rows,
                    [(2, Some("x > 1".to_string()))],
                    "the agent's breakpoint, with its new condition, is in Zed's store"
                );
                assert_eq!(*seen.breakpoints.lock(), [3], "and reached SetBreakpoints");
                // 3. The Debug panel shows the agent's session.
                let shown = debug_panel.read_with(cx, |panel, cx| {
                    panel
                        .active_session()
                        .map(|s| s.read(cx).running_state().read(cx).session().clone())
                });
                assert_eq!(shown.as_ref(), Some(&session), "the Debug panel's session");
            }
            if omp == 5 {
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
            if omp == 7 {
                assert_eq!(
                    sessions(&project, cx).len(),
                    1,
                    "the unknown scenario started no adapter"
                );
            }
            if omp == PERSON_CONTINUE {
                person = Some(person_session(&window, cx).await);
            }
            // OMP's own select (hand-made): always approved here, so every
            // refusal below is cedian's.
            "Approve"
        };
        let button: &'static str = format!("cedian-dialog-{id}-{choice}").leak();
        click(&mut vcx, button);
    }

    let (connection, notice) = panel.read_with(cx, |p, _| {
        (p.connection().clone(), p.notice().map(str::to_string))
    });
    assert!(
        matches!(connection, Connection::Ready { .. }) && notice.is_none(),
        "fake-omp replayed to the end (no divergence): {connection:?} · {notice:?}"
    );
    assert!(person.is_some());
    // 8. What the person denied never reached an adapter.
    assert_eq!(
        seen.continues[0].load(Ordering::SeqCst),
        2,
        "the agent's continue and the person's"
    );
    assert_eq!(
        seen.continues[1].load(Ordering::SeqCst),
        0,
        "cedian sent no Continue to the person's session after the Deny"
    );
    assert_eq!(
        launched_before_denied_start,
        Some(1),
        "only the agent's approved launch had started an adapter"
    );
    assert_eq!(
        seen.sessions.load(Ordering::SeqCst),
        2,
        "the denied start launched nothing: the agent's session and the person's"
    );
    // 9. One gate row per call, in order.
    let audit = std::fs::read_to_string(state.join(cedian_shell::audit::AUDIT_FILE)).unwrap();
    let gate: Vec<(String, String)> = audit
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .map(|row| row["item"].clone())
        .filter(|row| {
            row["kind"] == "gate"
                && row["command"]
                    .as_str()
                    .is_some_and(|c| c.starts_with("cedian_debug "))
        })
        .map(|row| {
            (
                row["decision"].as_str().unwrap_or_default().to_string(),
                row["answered_by"].as_str().unwrap_or_default().to_string(),
            )
        })
        .collect();
    let want = [
        ("allow", "cedian"),
        ("allow", "cedian"),
        ("allow", "user"),
        ("allow", "cedian"),
        ("allow", "cedian"),
        ("deny", "cedian"),
        ("deny", "cedian"),
        ("allow", "user"),
        ("deny", "user"),
        ("deny", "user"),
    ]
    .map(|(d, a)| (d.to_string(), a.to_string()));
    assert_eq!(gate, want, "cedian's gate rows in the audit log");
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
