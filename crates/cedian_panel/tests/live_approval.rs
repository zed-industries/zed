//! S9 U4 exit check: a real OMP 18.6.1 turn, recorded once and replayed in
//! the app. The model runs `echo cedian` with bash; OMP raises its own
//! `Allow tool: bash` approval; the person clicks Approve in the panel; the
//! tool output and the final text show; the audit has allow/user/bash.
//!
//! Replay (default) plays `live_approval.jsonl` through fake-omp. Record
//! (`CEDIAN_U4_RECORD=1 CEDIAN_RECORD_OMP=<pinned omp>`, one model turn,
//! the person's HOME for OMP's auth) proxies to the pinned OMP through a
//! wrapper that adds `--session-dir` in the run's temp dir, so the session
//! stays out of the person's OMP store, and writes the scrubbed fixture.
//!
//! Two request/response pairs are synthetic, added after the recording
//! when the link began sending them: `set_host_tools` (`req_tools_1`) and
//! `set_subagent_subscription` (`req_sub_1`). Their responses follow OMP's
//! documented shapes; a re-record replaces them.
//!
//! The benchmark's instruments ride along (ADR-0058 decision 3): the app
//! writes one spawn and one turn row in the CLI's `CEDIAN_TIMING` schema,
//! whose parts add up, and one `CEDIAN_FRAMES` row for the run's draws.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

#![allow(
    clippy::disallowed_methods,
    reason = "test setup writes a wrapper script with std::fs"
)]

use cedian_panel::{CedianPanel, Connection, Turn};
use gpui::{Modifiers, TestAppContext, VisualTestContext, WindowHandle};
use omp_rpc::ExtensionUiRequest;
use project::Project;
use serde_json::Value;
use settings::SettingsStore;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/live_approval.jsonl"
);
const PROMPT: &str = "Run the shell command `echo cedian` with your bash tool. \
                      Then reply with exactly: done";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        _ => {}
    }
    let record = std::env::var_os("CEDIAN_U4_RECORD").is_some();
    // U12e (ADR-0058 decision 3): `CEDIAN_U12_APP_TURNS` runs the app live
    // without writing a fixture, sends that many warm turns after the approval
    // one, and copies the instruments' rows into `CEDIAN_U12_APP_OUT` (this
    // run's temp root is removed at the end).
    let warm: usize = std::env::var("CEDIAN_U12_APP_TURNS")
        .ok()
        .and_then(|turns| turns.parse().ok())
        .unwrap_or(0);
    // U12e's release measurement cannot drive the UI: GPUI's debug bounds are
    // a debug-build feature, so no test click reaches a dialog. With
    // `CEDIAN_U12_APP_POLICY=omp` the run takes OMP's own config (the person's
    // approval mode), which asks for nothing, and the scenario skips the
    // dialog. The frame budget is about streaming either way.
    let dialog = std::env::var("CEDIAN_U12_APP_POLICY").as_deref() != Ok("omp");
    let live = record || warm > 0;
    let root = std::env::temp_dir().join(format!("cedian-u4-live-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("ws")).unwrap();
    let mut config = "schema = 1\n".to_string();
    if !dialog {
        config.push_str(&format!(
            "[projects.\"{}\"]\npolicy = \"omp\"\n",
            root.canonicalize().unwrap().join("ws").display()
        ));
    }
    std::fs::write(root.join("cedian.toml"), config).unwrap();
    let root = root.canonicalize().unwrap();
    // SAFETY: single-threaded here; nothing else reads the environment yet.
    unsafe {
        std::env::set_var("CEDIAN_CONFIG", root.join("cedian.toml"));
        std::env::set_var("CEDIAN_STATE_DIR", root.join("state"));
        if !live {
            std::env::set_var("HOME", root.join("home"));
        }
        // U12e spawns the pinned OMP itself (see `scenario`): the record proxy
        // would leave the session file held by its own child, which the panel's
        // one-driver check reads as another driver (ADR-0046).
        if warm == 0 {
            std::env::set_var("CEDIAN_OMP_BINARY", std::env::current_exe().unwrap());
        }
        std::env::set_var("CEDIAN_TIMING", root.join("timing.jsonl"));
        std::env::set_var("CEDIAN_FRAMES", root.join("frames.jsonl"));
    }
    print!("test live_approval_replays_in_the_app ... ");
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("live_approval"));
            gpui::ForegroundExecutor::new(exec)
                .block_test(scenario(&mut cx, &root, record, warm, dialog));
            cx.run_until_parked();
            cx.update(|cx| cx.quit());
            cx.run_until_parked();
            dispatcher.drain_tasks();
            let _ = std::fs::remove_dir_all(&root);
        }),
    );
    println!("ok");
}

async fn scenario(cx: &mut TestAppContext, root: &Path, record: bool, warm: usize, dialog: bool) {
    let live = record || warm > 0;
    cx.executor().allow_parking();
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });
    let ws = root.join("ws");
    let state = cedian_shell::state::dir(&ws).unwrap();
    let sessions = state.join("omp");
    if live && record {
        cedian_fake_omp::recording::arm_app(&sessions, root);
    } else if !live {
        cedian_fake_omp::install_replay(&sessions, Path::new(FIXTURE)).unwrap();
    }
    // U12e (live, no recording) spawns the caller's `CEDIAN_OMP_BINARY`: the
    // harness's wrapper around the pinned OMP, which adds the pass's model to
    // an rpc spawn and changes nothing else. The panel must spawn OMP itself,
    // not through the record proxy, whose own child holds the session file and
    // reads to the panel as another driver (ADR-0046).

    let project = Project::test(fs::RealFs::new(None, cx.executor()), [ws.as_path()], cx).await;
    let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    wait(cx, &window, "OMP ready", |p| {
        matches!(p.connection(), Connection::Ready { .. })
    });

    window
        .update(cx, |panel, window, cx| {
            panel.set_prompt(PROMPT, window, cx);
            panel.submit(window, cx);
        })
        .unwrap();
    if dialog {
        wait(cx, &window, "OMP's approval", |p| {
            !p.dialog_ids().is_empty()
        });
    } else {
        // OMP's own approval mode (policy = omp) asks nothing: the turn runs.
        wait(cx, &window, "the first turn to run", |p| {
            p.turn() != &Turn::Idle || p.dialog_ids().is_empty()
        });
    }
    if dialog {
        let (id, approve) = window
            .update(cx, |p, _, _| {
                let id = p.dialog_ids().remove(0);
                let request = p.dialog(&id).unwrap().request().clone();
                let ExtensionUiRequest::Select(select) = request else {
                    panic!("not an approval select: {request:?}");
                };
                assert!(
                    select.title.starts_with("Allow tool: bash"),
                    "{}",
                    select.title
                );
                let approve = select
                    .options
                    .iter()
                    .find(|o| o.to_ascii_lowercase().starts_with("approve"))
                    .unwrap_or_else(|| panic!("no Approve in {:?}", select.options))
                    .clone();
                (id, approve)
            })
            .unwrap();
        // debug_bounds takes a &'static str; this id is only known at run time.
        let button: &'static str = format!("cedian-dialog-{id}-{approve}").leak();
        click(&mut vcx, button);
    }
    wait(cx, &window, "the approved turn to complete", |p| {
        p.dialog_ids().is_empty() && p.turn() == &Turn::Idle
    });

    if record {
        cedian_fake_omp::recording::write_fixture(
            &sessions.join(cedian_fake_omp::RECORDED_FILE),
            Path::new(FIXTURE),
        );
    }
    let transcript = window.update(cx, |p, _, _| p.transcript()).unwrap();
    if dialog {
        assert!(
            transcript.iter().any(|l| l.contains("[Done]")
                && l.contains("echo cedian")
                && l.contains("→ cedian")),
            "the bash card with its output: {transcript:#?}"
        );
        assert!(
            transcript
                .iter()
                .any(|l| l.starts_with("Assistant") && l.contains("done")),
            "the final text: {transcript:#?}"
        );
    }
    let gates: Vec<Value> = std::fs::read_to_string(state.join("audit.jsonl"))
        .unwrap()
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .map(|row| row["item"].clone())
        .filter(|item| item["kind"] == "gate")
        .collect();
    if dialog {
        assert_eq!(gates.len(), 1, "{gates:?}");
        assert_eq!(
            (
                gates[0]["decision"].as_str(),
                gates[0]["answered_by"].as_str(),
                gates[0]["tool"].as_str()
            ),
            (Some("allow"), Some("user"), Some("bash")),
        );
    } else {
        assert!(
            gates.is_empty(),
            "OMP's own approval mode asks nothing: {gates:?}"
        );
    }
    let connection = window.update(cx, |p, _, _| p.connection().clone()).unwrap();
    assert!(
        matches!(connection, Connection::Ready { .. }),
        "the link after the approval turn: {connection:?}"
    );
    // The warm turns: the same session, one prompt each, so the rows describe
    // a session the person has been using rather than its first turn.
    for turn in 1..=warm {
        let prompt = format!("Reply with exactly: warm-{turn}");
        window
            .update(cx, |panel, window, cx| {
                panel.set_prompt(&prompt, window, cx);
                panel.submit(window, cx);
            })
            .unwrap();
        wait(cx, &window, "the warm turn to settle", |p| {
            p.turn() == &Turn::Idle && p.dialog_ids().is_empty()
        });
    }

    wait(cx, &window, "the turn's timing row", |_| {
        rows(&root.join("timing.jsonl")).len() == 2 + warm
    });
    let timing = rows(&root.join("timing.jsonl"));
    assert_eq!(timing[0]["event"], "spawn", "{timing:?}");
    let turn = &timing[1];
    assert_eq!(turn["event"], "turn", "{timing:?}");
    let part = |k: &str| turn[k].as_u64().unwrap_or_else(|| panic!("{k} in {turn}"));
    assert_eq!(
        part("context_ms") + part("omp_ms") + part("post_ms"),
        part("total_ms"),
        "{turn}"
    );
    assert!(part("omp_ms") > 0, "OMP's time is measured: {turn}");
    let frames = rows(&root.join("frames.jsonl"));
    assert!(!frames.is_empty(), "a frames row for the run: {frames:?}");
    let ms = |k: &str| {
        frames[0][k]
            .as_f64()
            .unwrap_or_else(|| panic!("{k} in {frames:?}"))
    };
    assert!(frames[0]["frames"].as_u64().unwrap() > 0, "{frames:?}");
    assert!(
        ms("draw_p50_ms") <= ms("draw_p95_ms")
            && ms("draw_p95_ms") <= ms("draw_p99_ms")
            && ms("draw_p99_ms") <= ms("draw_max_ms")
            && ms("draw_max_ms") > 0.0,
        "{frames:?}"
    );
    // U12e: hand the rows of this run (the approval turn and the warm ones) to
    // whoever asked for them, and leave the person's OMP store as it was: the
    // run spawned the pinned OMP itself, so its session went to OMP's own store
    // rather than a wrapper's `--session-dir`.
    if let Some(out) = std::env::var_os("CEDIAN_U12_APP_OUT") {
        let out = PathBuf::from(out);
        std::fs::create_dir_all(&out).unwrap();
        for name in ["timing.jsonl", "frames.jsonl"] {
            std::fs::copy(root.join(name), out.join(format!("app-{name}"))).unwrap();
        }
        let session = window
            .update(cx, |p, _, _| match p.connection() {
                Connection::Ready { session_id, .. } => Some(session_id.clone()),
                _ => None,
            })
            .unwrap();
        let binary =
            PathBuf::from(std::env::var("CEDIAN_OMP_BINARY").unwrap_or_else(|_| "omp".to_string()));
        if let (Some(id), Ok(agent)) = (session, cedian_omp::OmpConfig::new(binary).dir(root)) {
            let sessions = agent.join("sessions");
            for slug in std::fs::read_dir(&sessions).into_iter().flatten().flatten() {
                for entry in std::fs::read_dir(slug.path())
                    .into_iter()
                    .flatten()
                    .flatten()
                {
                    if !entry.file_name().to_string_lossy().contains(&id) {
                        continue;
                    }
                    let path = entry.path();
                    let _ = if path.is_dir() {
                        std::fs::remove_dir_all(&path)
                    } else {
                        std::fs::remove_file(&path)
                    };
                }
            }
        }
    }
}

fn rows(path: &Path) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn rendered(
    vcx: &mut VisualTestContext,
    selector: &'static str,
) -> Option<gpui::Bounds<gpui::Pixels>> {
    vcx.update(|window, _| window.refresh());
    vcx.run_until_parked();
    vcx.debug_bounds(selector)
}

fn click(vcx: &mut VisualTestContext, selector: &'static str) {
    let bounds = rendered(vcx, selector).unwrap_or_else(|| panic!("{selector} is not on screen"));
    vcx.simulate_click(bounds.center(), Modifiers::none());
}

fn wait(
    cx: &mut TestAppContext,
    window: &WindowHandle<CedianPanel>,
    what: &str,
    done: impl Fn(&CedianPanel) -> bool,
) {
    let deadline = Instant::now() + Duration::from_secs(240);
    loop {
        cx.run_until_parked();
        let (finished, state) = window
            .update(cx, |p, _, _| {
                (
                    done(p),
                    format!(
                        "{:?} · {:?} · {:?}",
                        p.connection(),
                        p.turn(),
                        p.dialog_ids()
                    ),
                )
            })
            .unwrap();
        if finished {
            return;
        }
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {what}: {state}"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}
