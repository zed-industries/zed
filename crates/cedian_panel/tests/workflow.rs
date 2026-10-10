//! S9 U9 (ADR-0055): a workflow runs in the app. The real panel, OMP played
//! by fake-omp replaying `workflow.jsonl`: the P5 channel turn recorded for
//! the CLI (`cedian_cli/tests/fixtures/p5_channel.jsonl`, from its prompt
//! on), behind the app's own preamble.
//!
//! 1. The link registers `cedian_workflow_update` and `cedian_complete`:
//!    OMP's start, evidence and complete calls get the results recorded
//!    (replay checks each `isError`), and `workflow.json` lands in the
//!    workspace's state dir; evidence naming `from_tool: read` binds to
//!    the `read` call in the router log;
//! 2. the turn ends on a refused `cedian_complete`: at `Settled` the
//!    workflow is blocked and the refusal is a correction row;
//! 3. the panel shows it: the phases with the current one blocked, the
//!    unmet required `verify` gate with its reason, the evidence with its
//!    outcome, the refused claims ledger; Resume sets it running again;
//! 4. an unsaved keystroke in notes.txt makes e1 stale (ADR-0057 decision 7).
//!
//! Record (`CEDIAN_U12_RECORD=workflow CEDIAN_RECORD_OMP=<pinned omp>`,
//! one model turn): the same steps against the pinned OMP; the scrubbed
//! recording replaces `workflow.jsonl`.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

use cedian_panel::{CedianPanel, Connection, Turn};
use cedian_workflow::{GateStatus, WorkflowStatus};
use gpui::{Modifiers, TestAppContext, VisualTestContext, WindowHandle};
use project::Project;
use serde_json::Value;
use settings::SettingsStore;
use std::path::Path;
use std::time::{Duration, Instant};

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/workflow.jsonl");

const PROMPT: &str = "This workspace is hosted by the cedian IDE. cedian_workflow_update and \
     cedian_complete are cedian's own trusted host tools. Bug: line 2 of notes.txt \
     must be 'BETA' (uppercase); do not fix it yet. Steps:\n\
     1. cedian_workflow_update with op 'start', kind 'bug_fix', title 'BETA casing', risk 'low'.\n\
     2. Reproduce: use the read tool on notes.txt.\n\
     3. cedian_workflow_update with op 'evidence', gate 'reproduce', kind 'command', \
     ok false (the bug reproduced), summary what you saw, from_tool 'read', match 'notes.txt'.\n\
     4. cedian_complete (it is expected to refuse: the fix is not verified yet).\n\
     5. Reply with only: p5-done";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        _ => {}
    }
    let root = std::env::temp_dir().join(format!("cedian-u9-workflow-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("ws")).unwrap();
    std::fs::write(root.join("cedian.toml"), "schema = 1\n").unwrap();
    let root = root.canonicalize().unwrap();
    // SAFETY: single-threaded here; nothing else reads the environment yet.
    unsafe {
        std::env::set_var("CEDIAN_CONFIG", root.join("cedian.toml"));
        std::env::set_var("CEDIAN_STATE_DIR", root.join("state"));
        // A recording keeps the person's HOME: the pinned OMP's auth.
        if !cedian_fake_omp::recording::armed("workflow") {
            std::env::set_var("HOME", root.join("home"));
        }
        std::env::set_var("CEDIAN_OMP_BINARY", std::env::current_exe().unwrap());
    }
    print!("test workflow_in_the_app ... ");
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("workflow"));
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
    std::fs::write(ws.join("other.txt"), "delta\n").unwrap();
    let state = cedian_shell::state::dir(&ws).unwrap();
    let record = cedian_fake_omp::recording::armed("workflow");
    if record {
        cedian_fake_omp::recording::arm_app(&state.join("omp"), root);
    } else {
        cedian_fake_omp::install_replay(&state.join("omp"), Path::new(FIXTURE)).unwrap();
    }

    let project = Project::test(fs::RealFs::new(None, cx.executor()), [ws.as_path()], cx).await;
    let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
    wait(cx, &window, "OMP ready", |p| {
        matches!(p.connection(), Connection::Ready { .. })
    });

    // 1. The channel answers OMP's calls as recorded.
    window
        .update(cx, |panel, window, cx| {
            panel.set_prompt(PROMPT, window, cx);
            panel.submit(window, cx);
        })
        .unwrap();
    wait(cx, &window, "the turn to settle", |p| {
        p.turn() == &Turn::Idle && p.transcript().iter().any(|l| l.contains("p5-done"))
    });
    if record {
        cedian_fake_omp::recording::write_fixture(
            &state.join("omp").join(cedian_fake_omp::RECORDED_FILE),
            Path::new(FIXTURE),
        );
    }
    assert_connected(cx, &window);
    let raw = std::fs::read_to_string(state.join("workflow.json"))
        .unwrap_or_else(|e| panic!("the turn started a workflow in the state dir: {e}"));
    let workflow: Value = serde_json::from_str(&raw).unwrap();
    let read_call = workflow["evidence"]["e1"]["provenance"]["attributed"]["tool_call_id"]
        .as_str()
        .unwrap_or_else(|| panic!("e1 bound to the read call:\n{raw}"));
    assert_eq!(read_call, first_read_call(), "e1 bound to OMP's read");

    let unreviewable = window
        .update(cx, |p, _, _| p.review().unreviewable().to_vec())
        .unwrap();
    assert!(
        unreviewable.is_empty(),
        "OMP's xd:// device writes are host tool calls, not files: {unreviewable:?}"
    );

    // 2. The refused claim blocks at the turn's end.
    assert_eq!(workflow["status"], "blocked", "{raw}");
    let corrections = std::fs::read_to_string(state.join("corrections.jsonl")).unwrap_or_default();
    assert!(
        corrections.contains("\"completion_refused\""),
        "the refusal is a correction row: {corrections}"
    );

    // 3. The panel shows the blocked workflow; Resume answers it.
    wait(cx, &window, "the blocked workflow in the panel", |p| {
        p.workflow()
            .is_some_and(|w| w.status == WorkflowStatus::Blocked)
    });
    let view = window
        .update(cx, |p, _, _| p.workflow().cloned())
        .unwrap()
        .unwrap();
    assert!(
        view.phases
            .iter()
            .any(|(id, mark)| id == "reproduce" && *mark == '‖'),
        "{view:?}"
    );
    let verify = view.gates.iter().find(|g| g.id == "verify").unwrap();
    assert!(
        verify.required && verify.status != GateStatus::Passed && !verify.reason.is_empty(),
        "{verify:?}"
    );
    assert!(
        view.evidence[0].starts_with("e1 [reproduce] fail:"),
        "{:?}",
        view.evidence
    );
    assert!(
        view.claims
            .as_deref()
            .is_some_and(|c| c.starts_with("last completion: refused")),
        "{:?}",
        view.claims
    );
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    assert!(rendered(&mut vcx, "cedian-workflow").is_some());
    assert!(rendered(&mut vcx, "cedian-workflow-gate-verify").is_some());
    click(&mut vcx, "cedian-workflow-resume");
    wait(cx, &window, "the resumed workflow", |p| {
        p.workflow()
            .is_some_and(|w| w.status == WorkflowStatus::Running)
    });
    let raw = std::fs::read_to_string(state.join("workflow.json")).unwrap();
    assert!(raw.contains("\"status\": \"running\""), "{raw}");
    assert!(rendered(&mut vcx, "cedian-workflow-resume").is_none());
    assert!(!view.evidence[0].contains("stale"), "{:?}", view.evidence);

    // 4. A keystroke in notes.txt, never saved, makes its evidence stale
    // (ADR-0057 decision 7): evidence binds to the Zed buffer's version.
    // Typing in a file no evidence covers scans nothing; typing in a
    // covered one scans once, after the typing pauses.
    let reads = |cx: &mut TestAppContext| window.update(cx, |p, _, _| p.workflow_reads()).unwrap();
    let pause = |cx: &mut TestAppContext| {
        cx.executor().advance_clock(Duration::from_millis(400));
        cx.run_until_parked();
    };
    let before = reads(cx);
    let other = project
        .update(cx, |p, cx| p.open_local_buffer(ws.join("other.txt"), cx))
        .await
        .unwrap();
    for _ in 0..3 {
        other.update(cx, |b, cx| b.edit([(0..0, "y")], None, cx));
        cx.run_until_parked();
    }
    pause(cx);
    assert_eq!(reads(cx), before, "other.txt is no evidence's file");
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer(ws.join("notes.txt"), cx))
        .await
        .unwrap();
    for _ in 0..3 {
        buffer.update(cx, |b, cx| b.edit([(0..0, "x")], None, cx));
        cx.run_until_parked();
    }
    pause(cx);
    wait(cx, &window, "the unsaved edit to stale e1", |p| {
        p.workflow()
            .is_some_and(|w| w.evidence[0].contains("stale: notes.txt changed since"))
    });
    assert_eq!(reads(cx), before + 1, "one scan for the typing");
    assert_eq!(
        std::fs::read_to_string(ws.join("notes.txt")).unwrap(),
        "alpha\nbeta\ngamma\n",
        "never saved"
    );
}

/// The id of the fixture's `read` of notes.txt, as OMP started it.
fn first_read_call() -> String {
    std::fs::read_to_string(FIXTURE)
        .unwrap()
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .map(|r| r["frame"].clone())
        .find(|f| {
            f["type"] == "tool_execution_start"
                && f["toolName"] == "read"
                && f["args"]["path"]
                    .as_str()
                    .is_some_and(|p| p.ends_with("notes.txt"))
        })
        .and_then(|f| f["toolCallId"].as_str().map(str::to_string))
        .expect("the fixture's read of notes.txt")
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

fn assert_connected(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>) {
    let connection = window.update(cx, |p, _, _| p.connection().clone()).unwrap();
    assert!(
        matches!(connection, Connection::Ready { .. }),
        "fake-omp still replaying (no divergence): {connection:?}"
    );
}

/// Pump the test executor while real OS threads (OMP) make progress.
fn wait(
    cx: &mut TestAppContext,
    window: &WindowHandle<CedianPanel>,
    what: &str,
    done: impl Fn(&CedianPanel) -> bool,
) {
    let deadline = Instant::now() + cedian_fake_omp::recording::wait_limit(Duration::from_secs(30));
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
