//! S9 U9i (ADR-0055): the reviewer runs from the app. The real panel; the
//! implementing OMP is fake-omp replaying `review.jsonl` (the app's
//! preamble from `workflow.jsonl`, then OMP's recorded `edit` of notes.txt
//! from the CLI's `s2_blocked.jsonl`); the reviewer is a second fake-omp
//! replaying the CLI's `shell_review_reviewer.jsonl` unchanged, so the
//! reviewer must be shown exactly the diff the CLI showed it. That reviewer
//! turn was asked for by a person (`review --agent`), and no recording has
//! OMP asking with its focus, so the review here is the person's ("Ask for
//! review"); OMP's `cedian_review_request` runs the same function, and its
//! registration is checked in the spawn overlay.
//!
//! 1. OMP's edit is a hunk in the task's review (Zed buffers);
//! 2. a review asked for in the app reads its diff from those buffers: the
//!    reviewer runs under its own spawn profile, reports a blocker through
//!    `cedian_review_finding`, and the finding shows on its hunk in Review
//!    Changes; its `review` evidence is `fail` (an independent reviewer);
//! 3. the blocker refuses `cedian_complete`, naming the app's Dismiss;
//! 4. Dismiss takes a reason (an empty one is refused), writes a
//!    `finding_dismissed` row and an audit row, and completion no longer
//!    names the blocker.
//!
//! U9j: with notes.txt open in an editor of the workspace, the open finding
//! is a block under its hunk (not a diagnostic: `cedian://diagnostics` does
//! not carry it); the block goes when the hunk's text changes, comes back
//! with it, and goes when the finding is dismissed.
//!
//! Fix round: a second review asked for while one runs is refused; the
//! review uses the launch's settings (cedian.toml is broken after spawn);
//! notes.txt is opened only after the review and gets its block at once;
//! a blocker whose hunk changed keeps a Dismiss in Review Changes.
//!
//! Record (`CEDIAN_U12_RECORD=review CEDIAN_RECORD_OMP=<pinned omp>`, one
//! model turn): the implementing OMP is the pinned one and its scrubbed
//! recording replaces `review.jsonl`; the reviewer still replays the CLI's
//! recording, so the app must show it the diff the CLI did.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

use cedian_panel::{CedianPanel, Connection, Turn};
use cedian_review::FindingSeverity;
use editor::Editor;
use gpui::AppContext as _;
use gpui::{Modifiers, TestAppContext, VisualTestContext, WindowHandle};
use project::Project;
use serde_json::{Value, json};
use settings::SettingsStore;
use std::path::Path;
use std::time::{Duration, Instant};
use workspace::Workspace;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/review.jsonl");
const REVIEWER_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../cedian_cli/tests/fixtures/shell_review_reviewer.jsonl"
);
const PROMPT: &str = "Line 2 of notes.txt must be 'BETA' (uppercase). Fix it with the edit tool, then reply with only: p5-done";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        _ => {}
    }
    let root = std::env::temp_dir().join(format!("cedian-u9-review-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("ws")).unwrap();
    std::fs::write(root.join("cedian.toml"), "schema = 1\n").unwrap();
    let root = root.canonicalize().unwrap();
    // SAFETY: single-threaded here; nothing else reads the environment yet.
    unsafe {
        std::env::set_var("CEDIAN_CONFIG", root.join("cedian.toml"));
        std::env::set_var("CEDIAN_STATE_DIR", root.join("state"));
        // A recording keeps the person's HOME: the pinned OMP's auth.
        if !cedian_fake_omp::recording::armed("review") {
            std::env::set_var("HOME", root.join("home"));
        }
        std::env::set_var("CEDIAN_OMP_BINARY", std::env::current_exe().unwrap());
    }
    print!("test reviewer_from_the_app ... ");
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("review"));
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
    let state = cedian_shell::state::dir(&ws).unwrap();
    let record = cedian_fake_omp::recording::armed("review");
    if record {
        cedian_fake_omp::recording::arm_app(&state.join("omp"), root);
    } else {
        cedian_fake_omp::install_replay(&state.join("omp"), Path::new(FIXTURE)).unwrap();
    }
    cedian_fake_omp::install_replay(&state.join("reviewer"), Path::new(REVIEWER_FIXTURE)).unwrap();
    // The role the reviewer reads, from the directory cedian owns.
    let roles = state.join("roles/.omp");
    std::fs::create_dir_all(&roles).unwrap();
    std::fs::write(
        roles.join("config.yml"),
        "modelRoles:\n  default: opencode-go/muse-spark-1.3-contributor\n  review: opencode-go/glm-5.3\n",
    )
    .unwrap();

    let project = Project::test(fs::RealFs::new(None, cx.executor()), [ws.as_path()], cx).await;
    let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
    wait(cx, &window, "OMP ready", |p| {
        matches!(p.connection(), Connection::Ready { .. })
    });

    // OMP may call the reviewer itself: the host tool is registered.
    let overlay: Value = serde_json::from_str(
        &std::fs::read_to_string(state.join("omp/cedian-overlay.yml")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        overlay["tools"]["approval"]["cedian_review_request"], "allow",
        "{overlay}"
    );

    // 1. OMP's edit is a hunk on the buffer.
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
    let hunks = window
        .update(cx, |p, _, _| {
            p.review()
                .files()
                .iter()
                .flat_map(|f| f.hunks().iter().map(|h| h.new_text.clone()))
                .collect::<Vec<_>>()
        })
        .unwrap();
    assert_eq!(hunks, vec!["BETA\n".to_string()]);

    // U9j: the person's workspace; notes.txt is opened in it only after
    // the review.
    let buffer = window
        .update(cx, |p, _, _| p.review().files()[0].buffer().clone())
        .unwrap();
    let workspace_window =
        cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
    let workspace_handle = workspace_window
        .update(cx, |_, _, cx| cx.entity().downgrade())
        .unwrap();
    window
        .update(cx, |p, _, cx| p.set_workspace(workspace_handle, cx))
        .unwrap();
    // A review uses the settings OMP was launched with, not cedian.toml now.
    std::fs::write(root.join("cedian.toml"), "schema = [unreadable\n").unwrap();

    let channel = window
        .update(cx, |p, _, _| p.workflow_channel())
        .unwrap()
        .expect("the panel keeps the workflow channel");
    let args = |v: Value| v.as_object().unwrap().clone();
    channel
        .update(&args(
            json!({"op": "start", "kind": "feature", "title": "casing", "risk": "low"}),
        ))
        .unwrap();

    // 2. The review reads the buffers; the finding shows on its hunk.
    window
        .update(cx, |p, _, cx| {
            p.request_review("is the uppercase BETA intended".to_string(), cx);
            p.request_review("a second, at once".to_string(), cx);
            assert_eq!(
                p.notice(),
                Some("a review is already running"),
                "one review at a time"
            );
        })
        .unwrap();
    wait(cx, &window, "the reviewer's finding", |p| {
        !p.findings().is_empty() && p.notice().is_some_and(|n| n.contains("reported 1 finding"))
    });
    let finding = window
        .update(cx, |p, _, _| p.findings()[0].clone())
        .unwrap();
    assert_eq!(finding.finding.path, "/notes.txt");
    assert_eq!(finding.finding.severity, FindingSeverity::Blocker);
    assert_eq!(finding.hunk_text, "BETA");
    assert_eq!(
        finding.reviewer_model.as_deref(),
        Some("opencode-go/glm-5.3")
    );

    let audit = std::fs::read_to_string(state.join("audit.jsonl")).unwrap();
    let review_row: Value = audit
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .find(|r| r["item"]["kind"] == "review" && r["item"]["independent"].is_boolean())
        .unwrap_or_else(|| panic!("the review is audited: {audit}"));
    assert_eq!(review_row["item"]["independent"], true, "{review_row}");
    assert!(
        audit.contains("\"actor\":\"reviewer\"") || audit.contains("\"actor\": \"reviewer\""),
        "the reviewer's calls are audited as the reviewer's: {audit}"
    );
    let workflow: Value =
        serde_json::from_str(&std::fs::read_to_string(state.join("workflow.json")).unwrap())
            .unwrap();
    let evidence = workflow["evidence"]
        .as_object()
        .unwrap()
        .values()
        .find(|e| e["for_gates"][0] == "review")
        .unwrap_or_else(|| panic!("review evidence: {workflow}"));
    assert_eq!(evidence["outcome"], "fail", "{evidence}");

    // U9j: the person opens notes.txt now; the finding is a block under
    // its hunk at once, never a diagnostic: the context OMP reads does not
    // carry it.
    workspace_window
        .update(cx, |workspace, window, cx| {
            let editor =
                cx.new(|cx| Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx));
            workspace.add_item_to_active_pane(Box::new(editor), None, true, window, cx);
        })
        .unwrap();
    cx.run_until_parked();
    let blocks = window.update(cx, |p, _, _| p.finding_blocks()).unwrap();
    assert_eq!(blocks, vec![("f1".to_string(), 1..2)]);
    {
        let mut wcx = VisualTestContext::from_window(workspace_window.into(), cx);
        assert!(
            rendered(&mut wcx, "cedian-finding-block-f1").is_some(),
            "the block renders in the editor"
        );
    }
    let diagnostics = window
        .update(cx, |p, _, cx| p.read_context("cedian://diagnostics", cx))
        .unwrap();
    let diagnostics = diagnostics.await.unwrap().content;
    assert!(
        !diagnostics.contains("f1") && !diagnostics.contains("Uppercase"),
        "the finding is not a diagnostic: {diagnostics}"
    );
    let count = buffer.read_with(cx, |b, _| {
        b.snapshot()
            .diagnostics_in_range::<_, usize>(0..b.len(), false)
            .count()
    });
    assert_eq!(count, 0, "no Zed diagnostic on the buffer");
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    click(&mut vcx, "cedian-review-toggle");
    // The hunk changes: the block goes; it comes back with the hunk.
    buffer.update(cx, |b, cx| b.edit([(6..10, "BETAX")], None, cx));
    cx.run_until_parked();
    assert!(
        window
            .update(cx, |p, _, _| p.finding_blocks())
            .unwrap()
            .is_empty(),
        "a changed hunk has no block"
    );
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    assert!(
        rendered(&mut vcx, "cedian-dismiss-f1").is_some(),
        "a blocker whose change left the review can still be dismissed"
    );
    buffer.update(cx, |b, cx| {
        b.undo(cx);
    });
    cx.run_until_parked();
    assert_eq!(
        window.update(cx, |p, _, _| p.finding_blocks()).unwrap(),
        vec![("f1".to_string(), 1..2)]
    );

    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    assert!(
        rendered(&mut vcx, "cedian-finding-f1").is_some(),
        "the finding shows on its hunk"
    );

    // 3. The blocker refuses completion and names the app's Dismiss.
    let refused = complete(cx, &channel).unwrap_err();
    assert!(refused.contains("review: blocker f1"), "{refused}");
    assert!(refused.contains("in Review Changes"), "{refused}");
    assert!(!refused.contains("cedian review dismiss"), "{refused}");

    // 4. Dismiss takes a reason.
    click(&mut vcx, "cedian-dismiss-f1");
    click(&mut vcx, "cedian-dismiss-confirm-f1");
    let (dismissed, notice) = window
        .update(cx, |p, _, _| {
            (
                p.findings()[0].dismissed.clone(),
                p.notice().map(str::to_string),
            )
        })
        .unwrap();
    assert_eq!(dismissed, None, "an empty reason is refused");
    assert!(
        notice.is_some_and(|n| n.contains("needs a reason")),
        "the refusal is shown"
    );
    window
        .update(cx, |p, window, cx| {
            p.dismiss_input()
                .expect("the reason box is open")
                .update(cx, |e, cx| e.set_text("uppercase is intended", window, cx))
        })
        .unwrap();
    click(&mut vcx, "cedian-dismiss-confirm-f1");
    let dismissed = window
        .update(cx, |p, _, _| p.findings()[0].dismissed.clone())
        .unwrap();
    assert_eq!(dismissed.as_deref(), Some("uppercase is intended"));
    assert!(
        window
            .update(cx, |p, _, _| p.finding_blocks())
            .unwrap()
            .is_empty(),
        "a dismissed finding has no block"
    );
    let corrections = std::fs::read_to_string(state.join("corrections.jsonl")).unwrap_or_default();
    assert!(
        corrections.contains("\"finding_dismissed\""),
        "the dismissal is a correction row: {corrections}"
    );
    let audit = std::fs::read_to_string(state.join("audit.jsonl")).unwrap();
    assert!(
        audit.contains("dismiss f1: uppercase is intended"),
        "the dismissal is audited: {audit}"
    );
    let refused = complete(cx, &channel).unwrap_err();
    assert!(!refused.contains("blocker f1"), "{refused}");
    assert_connected(cx, &window);
}

/// `cedian_complete` as OMP calls it: off the app thread, whose review
/// it reads.
fn complete(
    cx: &mut TestAppContext,
    channel: &std::sync::Arc<cedian_workflow::WorkflowChannel>,
) -> Result<String, String> {
    let channel = std::sync::Arc::clone(channel);
    let call = std::thread::spawn(move || channel.complete(&serde_json::Map::new()));
    while !call.is_finished() {
        cx.run_until_parked();
        std::thread::sleep(Duration::from_millis(20));
    }
    call.join().unwrap()
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
    vcx.run_until_parked();
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
    let deadline = Instant::now() + cedian_fake_omp::recording::wait_limit(Duration::from_secs(60));
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
