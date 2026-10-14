//! S10 U1: the session manager and history. The real panel; OMP is
//! fake-omp replaying `sessions.jsonl`: the preamble and recorded `edit` turn
//! of `review.jsonl`, then frames recorded from the pinned OMP 18.8.6 over
//! two session files it wrote (`fixtures/sessions/`), with the frames that
//! need a model turn hand-made and marked `handMade` (OMP's own rename, and
//! handoff, whose summary the model writes).
//!
//! 1. OMP's edit is a pending hunk in the task's review;
//! 2. the session list shows OMP's sessions; Open switches to one: the
//!    thread of the old session goes, its hunk stays pending, and the
//!    header names the new session;
//! 3. its history comes page by page (`get_messages_page`), then at once
//!    (`get_messages`), with its timeline (`get_entries`) and OMP's last
//!    reply (`get_last_assistant_text`, none here);
//! 4. Rename sends `set_session_name`; OMP's own rename
//!    (`session_info_update`) shows too;
//! 5. Export HTML writes outside the project, where OMP answers;
//! 6. Hand off: the history then opens with OMP's summary;
//! 7. switching to a session another process holds is refused like a
//!    taken resume (ADR-0040 decision 5) until Retry finds it free; the hunk
//!    is still pending.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

#![allow(
    clippy::disallowed_methods,
    reason = "the stand-in driver is a real OS process holding a file; no async spawn helper applies"
)]

use cedian_panel::{CedianPanel, Connection, Turn};
use cedian_review::HunkStatus;
use gpui::{Modifiers, TestAppContext, VisualTestContext, WindowHandle};
use project::Project;
use settings::SettingsStore;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/sessions.jsonl");
const SESSION_FILES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/sessions");
const PROMPT: &str = "Line 2 of notes.txt must be 'BETA' (uppercase). Fix it with the edit tool, then reply with only: p5-done";
const A: &str = "01a139e5-ba79-729d-95f7-e2f592ebbd2e";
const B: &str = "01a139e5-e3fa-7590-a571-57102af41fd6";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        _ => {}
    }
    let root = std::env::temp_dir().join(format!("cedian-s10-sessions-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("ws")).unwrap();
    std::fs::write(root.join("cedian.toml"), "schema = 1\n").unwrap();
    let root = root.canonicalize().unwrap();
    // SAFETY: single-threaded here; nothing else reads the environment yet.
    unsafe {
        std::env::set_var("CEDIAN_CONFIG", root.join("cedian.toml"));
        std::env::set_var("CEDIAN_STATE_DIR", root.join("state"));
        std::env::set_var("HOME", root.join("home"));
        std::env::set_var("CEDIAN_OMP_BINARY", std::env::current_exe().unwrap());
    }
    print!("test session_manager_and_history ... ");
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("sessions"));
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

fn session_file(dir: &Path, id: &str) -> PathBuf {
    std::fs::read_dir(dir)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .find(|p| p.to_string_lossy().ends_with(&format!("{id}.jsonl")))
        .unwrap()
}

async fn scenario(cx: &mut TestAppContext, root: &Path) {
    cx.executor().allow_parking();
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });
    let ws = root.join("ws");
    std::fs::write(ws.join("notes.txt"), "alpha\nbeta\ngamma\n").unwrap();
    let state = cedian_shell::state::dir(&ws).unwrap();
    let sessions = state.join("omp");
    cedian_fake_omp::install_replay(&sessions, Path::new(FIXTURE)).unwrap();
    for file in std::fs::read_dir(SESSION_FILES).unwrap().flatten() {
        std::fs::copy(file.path(), sessions.join(file.file_name())).unwrap();
    }

    let project = Project::test(fs::RealFs::new(None, cx.executor()), [ws.as_path()], cx).await;
    let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    wait(cx, &window, "OMP ready", |p| {
        matches!(p.connection(), Connection::Ready { .. })
    });

    // 1. A pending hunk from OMP's edit.
    window
        .update(cx, |panel, window, cx| {
            panel.set_prompt(PROMPT, window, cx);
            panel.submit(window, cx);
        })
        .unwrap();
    wait(cx, &window, "the turn to settle", |p| {
        p.turn() == &Turn::Idle && p.transcript().iter().any(|l| l.contains("p5-done"))
    });
    assert_eq!(pending(cx, &window), vec!["BETA\n".to_string()]);

    // 2. The list, and Open.
    click(&mut vcx, "cedian-sessions-toggle");
    wait(cx, &window, "the session list", |p| {
        p.sessions().entries.len() == 2
    });
    let listed: Vec<_> = window
        .update(cx, |p, _, _| {
            p.sessions()
                .entries
                .iter()
                .map(|e| (e.id.clone(), e.title.clone()))
                .collect()
        })
        .unwrap();
    assert_eq!(
        listed,
        vec![
            (B.to_string(), Some("flaky test hunt".to_string())),
            (A.to_string(), Some("release checklist".to_string())),
        ],
        "newest first, with OMP's titles"
    );
    click(&mut vcx, "cedian-open-session-92ebbd2e");
    wait(
        cx,
        &window,
        "session A",
        |p| matches!(p.connection(), Connection::Ready { session_id, resumed: true, .. } if session_id == A),
    );
    let (transcript, title, notes) = window
        .update(cx, |p, _, _| {
            (
                p.transcript(),
                p.sessions().title.clone(),
                p.session_notes().to_vec(),
            )
        })
        .unwrap();
    assert!(
        !transcript.iter().any(|l| l.contains("p5-done")),
        "the old session's thread went: {transcript:?}"
    );
    assert_eq!(title.as_deref(), Some("release checklist"));
    assert!(
        notes.iter().any(|n| n.contains("1 pending change(s)")),
        "{notes:?}"
    );
    assert_eq!(
        pending(cx, &window),
        vec!["BETA\n".to_string()],
        "a switch accepts and rejects nothing"
    );

    // 3. History.
    click(&mut vcx, "cedian-history-more");
    wait(cx, &window, "the first page", |p| {
        p.sessions().history.lines.len() == 20
    });
    let history = window
        .update(cx, |p, _, _| p.sessions().history.clone())
        .unwrap();
    assert_eq!(history.total, Some(25));
    assert_eq!(history.lines[0], "bash: $ echo a-1 → a-1");
    assert!(history.next.is_some());
    assert!(rendered(&mut vcx, "cedian-history-19").is_some());
    click(&mut vcx, "cedian-history-more");
    wait(cx, &window, "the last page", |p| {
        p.sessions().history.lines.len() == 25
    });
    let history = window
        .update(cx, |p, _, _| p.sessions().history.clone())
        .unwrap();
    assert_eq!(history.lines[24], "bash: $ echo a-25 → a-25");
    assert_eq!(history.next, None, "OMP gave no cursor after the last page");
    click(&mut vcx, "cedian-history-all");
    wait(cx, &window, "every message", |p| p.sessions().history.all);
    assert_eq!(
        window
            .update(cx, |p, _, _| p.sessions().history.lines.len())
            .unwrap(),
        25
    );
    click(&mut vcx, "cedian-timeline");
    wait(cx, &window, "the timeline", |p| {
        !p.sessions().history.timeline.is_empty()
    });
    let timeline = window
        .update(cx, |p, _, _| p.sessions().history.timeline.clone())
        .unwrap();
    assert_eq!(timeline[0], "model_change: anthropic/claude-opus-5-5");
    assert!(timeline.contains(&"message: bashExecution".to_string()));
    assert!(timeline.contains(&"title_change: release checklist".to_string()));
    click(&mut vcx, "cedian-last-reply");
    wait(cx, &window, "the last reply", |p| {
        p.sessions().history.last_reply.is_some()
    });
    assert_eq!(
        window
            .update(cx, |p, _, _| p.sessions().history.last_reply.clone())
            .unwrap(),
        Some(None),
        "OMP answers {{}} when the session has no reply"
    );
    assert!(rendered(&mut vcx, "cedian-last-reply-text").is_some());

    // 4. Rename.
    window
        .update(cx, |p, window, cx| {
            p.rename_input()
                .clone()
                .update(cx, |e, cx| e.set_text("release checklist v2", window, cx));
        })
        .unwrap();
    click(&mut vcx, "cedian-rename-session");
    wait(cx, &window, "the rename", |p| {
        p.sessions().title.as_deref() == Some("release checklist v2")
    });
    let entry_title = window
        .update(cx, |p, _, _| {
            p.sessions()
                .entries
                .iter()
                .find(|e| e.id == A)
                .and_then(|e| e.title.clone())
        })
        .unwrap();
    assert_eq!(entry_title.as_deref(), Some("release checklist v2"));

    // 5. Export, then OMP's own rename.
    click(&mut vcx, "cedian-export");
    let export = state.join("exports").join(format!("{A}.html"));
    wait(cx, &window, "the export", |p| {
        p.session_notes()
            .iter()
            .any(|n| n.starts_with("exported to "))
    });
    let note = window
        .update(cx, |p, _, _| {
            p.session_notes()
                .iter()
                .find(|n| n.starts_with("exported to "))
                .cloned()
        })
        .unwrap()
        .unwrap();
    assert_eq!(note, format!("exported to {}", export.display()));
    assert!(export.exists(), "{export:?}");
    assert!(
        !ws.join(format!("{A}.html")).exists(),
        "nothing lands in the project"
    );
    wait(cx, &window, "OMP's own title", |p| {
        p.sessions().title.as_deref() == Some("Release checklist, as OMP titled it")
    });

    // 6. Hand off.
    click(&mut vcx, "cedian-handoff");
    wait(cx, &window, "the handoff summary", |p| {
        p.sessions().history.lines.len() == 1
    });
    let first = window
        .update(cx, |p, _, _| p.sessions().history.lines[0].clone())
        .unwrap();
    assert_eq!(
        first,
        "handoff summary: Goal: the release checklist. Done: 25 echo checks. Next: tag the release."
    );

    // 7. Switch to a session another process drives.
    let held = session_file(&sessions, B);
    let holder = hold(&held);
    until_held(&held, &holder);
    click(&mut vcx, "cedian-open-session-2af41fd6");
    wait(
        cx,
        &window,
        "the taken session",
        |p| matches!(p.connection(), Connection::Taken { session_id, .. } if session_id == B),
    );
    let connection = window.update(cx, |p, _, _| p.connection().clone()).unwrap();
    assert!(
        matches!(&connection, Connection::Taken { reason, .. } if reason.contains(&holder.id().to_string())),
        "{connection:?}"
    );
    let (turn, notice) = window
        .update(cx, |panel, window, cx| {
            panel.set_prompt("must not reach OMP", window, cx);
            panel.submit(window, cx);
            (
                panel.turn().clone(),
                panel.notice().unwrap_or_default().to_string(),
            )
        })
        .unwrap();
    assert_eq!(turn, Turn::Idle);
    assert!(notice.contains("another process drives"), "{notice}");
    release(holder);
    click(&mut vcx, "cedian-retry");
    wait(
        cx,
        &window,
        "session B",
        |p| matches!(p.connection(), Connection::Ready { session_id, .. } if session_id == B),
    );
    assert_eq!(
        window
            .update(cx, |p, _, _| p.sessions().title.clone())
            .unwrap()
            .as_deref(),
        Some("flaky test hunt")
    );
    assert_eq!(pending(cx, &window), vec!["BETA\n".to_string()]);
}

fn pending(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>) -> Vec<String> {
    window
        .update(cx, |p, _, _| {
            p.review()
                .files()
                .iter()
                .flat_map(|f| f.hunks())
                .filter(|h| !matches!(h.status, HunkStatus::Accepted | HunkStatus::Rejected))
                .map(|h| h.new_text.clone())
                .collect()
        })
        .unwrap()
}

/// Another process with the session file open for writing, as a driving
/// OMP keeps it.
fn hold(file: &Path) -> Child {
    Command::new("/bin/sh")
        .arg("-c")
        .arg("exec 3>>\"$0\"; exec sleep 120")
        .arg(file)
        .spawn()
        .unwrap()
}

fn until_held(file: &Path, holder: &Child) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !cedian_omp::driver::holders(&[file.to_path_buf()])
        .unwrap()
        .contains(&holder.id())
    {
        assert!(
            Instant::now() < deadline,
            "the holder never opened {file:?}"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn release(mut holder: Child) {
    holder.kill().ok();
    holder.wait().ok();
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

/// Pump the test executor while real OS threads (OMP) make progress.
fn wait(
    cx: &mut TestAppContext,
    window: &WindowHandle<CedianPanel>,
    what: &str,
    done: impl Fn(&CedianPanel) -> bool,
) {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        cx.run_until_parked();
        let (finished, state) = window
            .update(cx, |p, _, _| {
                (
                    done(p),
                    format!("{:?} · {:?} · {:?}", p.connection(), p.turn(), p.notice()),
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
        std::thread::sleep(Duration::from_millis(50));
    }
}
