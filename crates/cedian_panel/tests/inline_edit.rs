//! S9 U10d (ADR-0056): the P6 replay through the app. The real panel; OMP
//! is fake-omp replaying `inline_edit.jsonl`, recorded against the pinned
//! OMP 18.8.6 in the live pass u12b (ADR-0065): the inline edit's turn and
//! the person's prompt turn, scrubbed.
//!
//! P6's checks, through the app:
//! 1. the inline edit of line 2, asked for with ctrl-enter in the editor
//!    (U10f), is a turn of its own kind, named in Review
//!    Changes, and its change is one attributed hunk, one transaction;
//! 2. the next prompt turn changes lines 1 and 4; the person edits line 4;
//! 3. revert turn from the editor's key puts line 1 back, keeps the STALE
//!    line 4, and the revert of that revert redoes it;
//! 4. reverting the inline-edit turn restores its start (line 2), and one
//!    undo redoes it.
//!
//! Record (`CEDIAN_U12_RECORD=inline_edit CEDIAN_RECORD_OMP=<pinned omp>`,
//! two model turns): the same two turns against the pinned OMP; the
//! scrubbed recording replaces `inline_edit.jsonl`.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

use cedian_panel::{CedianPanel, Connection, Turn};
use editor::Editor;
use gpui::AppContext as _;
use gpui::{Focusable as _, TestAppContext, VisualTestContext, WindowHandle};
use language::Point;
use project::Project;
use settings::SettingsStore;
use std::path::Path;
use std::time::{Duration, Instant};
use workspace::MultiWorkspace;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/inline_edit.jsonl"
);
const PROMPT: &str = "In notes.txt change line 1 'alpha' to 'ALPHA' and line 4 'delta' to 'DELTA' with your edit tool. Change nothing else. Reply with only: t2-done";
#[cfg(target_os = "macos")]
const REVERT_TURN_KEY: &str = "cmd-alt-shift-z";
#[cfg(not(target_os = "macos"))]
const REVERT_TURN_KEY: &str = "ctrl-alt-shift-z";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        _ => {}
    }
    let root = std::env::temp_dir().join(format!("cedian-u10-inline-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("ws")).unwrap();
    std::fs::write(root.join("cedian.toml"), "schema = 1\n").unwrap();
    let root = root.canonicalize().unwrap();
    // SAFETY: single-threaded here; nothing else reads the environment yet.
    unsafe {
        std::env::set_var("CEDIAN_CONFIG", root.join("cedian.toml"));
        std::env::set_var("CEDIAN_STATE_DIR", root.join("state"));
        // A recording keeps the person's HOME: the pinned OMP's auth.
        if !cedian_fake_omp::recording::armed("inline_edit") {
            std::env::set_var("HOME", root.join("home"));
        }
        std::env::set_var("CEDIAN_OMP_BINARY", std::env::current_exe().unwrap());
    }
    print!("test p6_replay_through_the_app ... ");
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("inline_edit"));
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
        cedian_panel::init(cx);
    });
    bind_default_keymap(cx);
    let ws = root.join("ws");
    std::fs::write(ws.join("notes.txt"), "alpha\nbeta\ngamma\ndelta\n").unwrap();
    let state = cedian_shell::state::dir(&ws).unwrap();
    let record = cedian_fake_omp::recording::armed("inline_edit");
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
    let buffer = project
        .update(cx, |p, cx| p.open_local_buffer(ws.join("notes.txt"), cx))
        .await
        .unwrap();

    // The person's workspace: notes.txt open and focused, the panel docked.
    let panel = window.root(cx).unwrap();
    let workspace_window =
        cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    workspace_window
        .update(cx, |multi, window, cx| {
            multi.workspace().clone().update(cx, |workspace, cx| {
                workspace.add_panel(panel.clone(), window, cx);
                let editor = cx.new(|cx| {
                    Editor::for_buffer(buffer.clone(), Some(project.clone()), window, cx)
                });
                workspace.add_item_to_active_pane(Box::new(editor.clone()), None, true, window, cx);
                editor.focus_handle(cx).focus(window, cx);
            })
        })
        .unwrap();
    let handle = workspace_window
        .read_with(cx, |multi, _| multi.workspace().downgrade())
        .unwrap();
    window
        .update(cx, |p, _, cx| p.set_workspace(handle, cx))
        .unwrap();

    // 1. The inline edit of line 2, asked for as the person does: select
    // the line, ctrl-enter, type the instruction, Enter.
    let editor = workspace_window
        .read_with(cx, |multi, cx| {
            multi
                .workspace()
                .read(cx)
                .active_item_as::<Editor>(cx)
                .unwrap()
        })
        .unwrap();
    let mut wcx = VisualTestContext::from_window(workspace_window.into(), cx);
    editor.update_in(&mut wcx, |editor, window, cx| {
        editor.change_selections(Default::default(), window, cx, |s| {
            s.select_ranges([Point::new(1, 0)..Point::new(2, 0)])
        });
    });
    wcx.update(|window, _| window.refresh());
    wcx.run_until_parked();
    wcx.simulate_keystrokes("ctrl-enter");
    wcx.update(|window, _| window.refresh());
    wcx.run_until_parked();
    assert!(
        wcx.debug_bounds("cedian-inline-edit").is_some(),
        "ctrl-enter opens the instruction block"
    );
    wcx.simulate_input("make this line uppercase");
    wcx.simulate_keystrokes("enter");
    wcx.run_until_parked();
    // The task's own end, not a reply word: the inline edit's message asks
    // for an edit, not for "done", so a live take need not say it.
    wait(cx, &window, "the inline edit to settle", |p| {
        p.turn() == &Turn::Idle
            && p.review()
                .files()
                .first()
                .is_some_and(|f| !f.agent_txns().is_empty())
    });
    assert_connected(cx, &window);
    let text = |cx: &mut TestAppContext| buffer.read_with(cx, |b, _| b.text());
    assert_eq!(text(cx), "alpha\nBETA\ngamma\ndelta\n");
    let (label, hunks, txns) = window
        .update(cx, |p, _, _| {
            let file = &p.review().files()[0];
            (
                p.review().turn_label(1).map(str::to_string),
                file.hunks()
                    .iter()
                    .map(|h| h.new_text.clone())
                    .collect::<Vec<_>>(),
                file.agent_txns().iter().map(|t| t.turn).collect::<Vec<_>>(),
            )
        })
        .unwrap();
    assert_eq!(
        label.as_deref(),
        Some("notes.txt:2-2 make this line uppercase")
    );
    assert_eq!(hunks, vec!["BETA\n".to_string()], "one attributed hunk");
    assert_eq!(txns, vec![1], "one transaction, the inline-edit turn's");
    let notice = window
        .update(cx, |p, _, _| p.notice().map(str::to_string))
        .unwrap();
    assert_eq!(
        notice, None,
        "an edit inside its selection warns of nothing"
    );
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    let bounds = {
        vcx.update(|window, _| window.refresh());
        vcx.run_until_parked();
        vcx.debug_bounds("cedian-review-toggle")
            .expect("Review Changes toggle")
    };
    vcx.simulate_click(bounds.center(), gpui::Modifiers::none());
    vcx.run_until_parked();
    assert_drawn(
        &mut vcx,
        "Revert turn 1: inline edit notes.txt:2-2 make this line uppercase",
    );
    // 2. The prompt turn, then the person's edit over line 4.
    let mut wcx = VisualTestContext::from_window(workspace_window.into(), cx);
    editor.update_in(&mut wcx, |editor, window, cx| {
        editor.change_selections(Default::default(), window, cx, |s| {
            s.select_ranges([Point::new(0, 0)..Point::new(0, 0)])
        });
    });
    window
        .update(cx, |p, window, cx| {
            p.set_prompt(PROMPT, window, cx);
            p.submit(window, cx);
        })
        .unwrap();
    wait(cx, &window, "the prompt turn to settle", |p| {
        p.turn() == &Turn::Idle && p.transcript().iter().any(|l| l.contains("t2-done"))
    });
    if record {
        cedian_fake_omp::recording::write_fixture(
            &state.join("omp").join(cedian_fake_omp::RECORDED_FILE),
            Path::new(FIXTURE),
        );
    }
    assert_connected(cx, &window);
    assert_eq!(text(cx), "ALPHA\nBETA\ngamma\nDELTA\n");
    buffer.update(cx, |b, cx| b.edit([(22..22, "!")], None, cx));
    assert_eq!(text(cx), "ALPHA\nBETA\ngamma\nDELTA!\n");

    // 3. Revert turn from the editor: STALE line 4 kept; again redoes it.
    let mut wcx = VisualTestContext::from_window(workspace_window.into(), cx);
    wcx.simulate_keystrokes(REVERT_TURN_KEY);
    wcx.run_until_parked();
    assert_eq!(text(cx), "alpha\nBETA\ngamma\nDELTA!\n");
    let notice = window
        .update(cx, |p, _, _| p.notice().map(str::to_string))
        .unwrap();
    assert_eq!(
        notice.as_deref(),
        Some(
            "turn 2 reverted: 1 hunk(s) put back, 1 STALE kept, \
             0 changed again by a later turn, kept, 0 accepted, kept; kept as STALE, \
             edited after the agent wrote it (an undo then redo counts as an edit): \
             /notes.txt line 4"
        )
    );
    let mut wcx = VisualTestContext::from_window(workspace_window.into(), cx);
    wcx.simulate_keystrokes(REVERT_TURN_KEY);
    wcx.run_until_parked();
    assert_eq!(
        text(cx),
        "ALPHA\nBETA\ngamma\nDELTA!\n",
        "the revert of the revert"
    );

    // 4. Reverting the inline-edit turn restores its start.
    window.update(cx, |p, _, cx| p.revert_turn(1, cx)).unwrap();
    assert_eq!(text(cx), "ALPHA\nbeta\ngamma\nDELTA!\n");
    buffer.update(cx, |b, cx| {
        b.undo(cx);
    });
    assert_eq!(
        text(cx),
        "ALPHA\nBETA\ngamma\nDELTA!\n",
        "one undo redoes it"
    );
    assert_connected(cx, &window);
}

/// Zed's default keymap for this platform, as the app loads it; actions
/// this binary does not link fail to load and are skipped.
fn bind_default_keymap(cx: &mut TestAppContext) {
    #[cfg(target_os = "macos")]
    let keymap = include_str!("../../../assets/keymaps/default-macos.json");
    #[cfg(target_os = "windows")]
    let keymap = include_str!("../../../assets/keymaps/default-windows.json");
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let keymap = include_str!("../../../assets/keymaps/default-linux.json");
    cx.update(|cx| {
        let bindings = match settings::KeymapFile::load(keymap, cx) {
            settings::KeymapFileLoadResult::Success { key_bindings, .. } => key_bindings,
            settings::KeymapFileLoadResult::SomeFailedToLoad { key_bindings, .. } => key_bindings,
            settings::KeymapFileLoadResult::JsonParseFailure { error } => {
                panic!("the default keymap parses: {error}")
            }
        };
        cx.bind_keys(bindings);
    });
}

/// The Revert turn button is drawn with `text`.
fn assert_drawn(vcx: &mut VisualTestContext, text: &str) {
    vcx.update(|window, _| window.refresh());
    vcx.run_until_parked();
    let selector = format!("cedian-revert-turn-text:{text}").leak();
    assert!(
        vcx.debug_bounds(selector).is_some(),
        "Revert turn is drawn as {text:?}"
    );
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
