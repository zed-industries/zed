//! ADR-0064: Stop and send. The real panel, OMP played by fake-omp
//! replaying `stop_and_send.jsonl`, whose prompts must arrive in order
//! with these texts (replay checks each message):
//!
//! 1. a turn with a queued follow-up: Stop and send aborts once, takes the
//!    follow-up back to the composer, and sends the composer's text only
//!    after the stopped run settled; the stopped turn's review turn ended
//!    before the new one began;
//! 2. OMP settles the stopped run before the stopped prompt's call returns:
//!    the next prompt still waits for the call, so the link's worker is free
//!    and the late return does not end the new turn;
//! 3. Stop and send while the prompt is still queued in the link: it never
//!    reaches OMP, and only the new text does.
//!
//! The frames are OMP 18.8.6's, hand-ordered as `turn_lifecycle_{1,3,4}`
//! are; the `fs_sleep`s only stretch a gap real OMP leaves short.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

use cedian_panel::{CedianPanel, Connection, Turn};
use gpui::{TestAppContext, VisualTestContext, WindowHandle};
use project::Project;
use settings::SettingsStore;
use std::path::Path;
use std::time::{Duration, Instant};

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/stop_and_send.jsonl"
);

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        _ => {}
    }
    let root = std::env::temp_dir().join(format!("cedian-stop-and-send-{}", std::process::id()));
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
    print!("test stop_and_send ... ");
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("stop_and_send"));
            let executor = gpui::ForegroundExecutor::new(exec);
            executor.block_test(scenario(&mut cx, &root));
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
    std::fs::write(ws.join("session.jsonl"), "").unwrap();
    let sessions = cedian_shell::state::dir(&ws).unwrap().join("omp");
    cedian_fake_omp::install_replay(&sessions, Path::new(FIXTURE)).unwrap();

    let project = Project::test(fs::RealFs::new(None, cx.executor()), [ws.as_path()], cx).await;
    let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
    let mut vcx = VisualTestContext::from_window(window.into(), cx);

    // 1. Stop and send with a follow-up queued.
    assert_eq!(submit(cx, &window, "one"), Turn::Queued);
    wait(cx, &window, "turn one to stream", |p| {
        ready(p) && p.turn() == &Turn::Streaming
    });
    assert_eq!(submit(cx, &window, "two"), Turn::Streaming);
    wait(cx, &window, "OMP to have the follow-up", |_| {
        ws.join("omp-got-two").exists()
    });
    let turn_one = review_turn(cx, &window);
    set_prompt(cx, &window, "three");
    click(&mut vcx, "cedian-stop-and-send");
    let (turn, next, typed) = window
        .update(cx, |p, _, cx| {
            (
                p.turn().clone(),
                p.next_prompt().map(str::to_string),
                p.prompt_text(cx),
            )
        })
        .unwrap();
    assert_eq!(
        (turn, next.as_deref()),
        (Turn::Stopping, Some("three")),
        "Stopping at once, the composer's text held"
    );
    assert!(
        !typed.contains("three"),
        "held, not left in the composer: {typed:?}"
    );
    wait(cx, &window, "turn three to run and end", |p| {
        p.turn() == &Turn::Idle
            && p.next_prompt().is_none()
            && p.transcript().iter().any(|l| l.contains("three"))
    });
    assert_eq!(
        prompt(cx, &window),
        "two",
        "the queued follow-up came back, not resent"
    );
    assert_eq!(
        review_turn(cx, &window),
        (turn_one.0 + 1, false),
        "turn three had a review turn of its own, after turn one's"
    );
    set_prompt(cx, &window, "");

    // 2. OMP settles before the stopped prompt's call returns.
    assert_eq!(submit(cx, &window, "four"), Turn::Queued);
    wait(cx, &window, "turn four to stream", |p| {
        p.turn() == &Turn::Streaming && ws.join("omp-runs-four").exists()
    });
    set_prompt(cx, &window, "five");
    window
        .update(cx, |p, window, cx| p.stop_and_send(window, cx))
        .unwrap();
    wait(cx, &window, "turn five to run and end", |p| {
        p.turn() == &Turn::Idle
            && p.next_prompt().is_none()
            && p.transcript().iter().any(|l| l.contains("five"))
    });
    let notice = window
        .update(cx, |p, _, _| p.notice().map(str::to_string))
        .unwrap();
    assert_eq!(notice, None, "the late return did not end turn five");

    // 3. Stop and send while the prompt waits in the link: "six" never
    // reaches OMP (replay expects "seven" next).
    window
        .update(cx, |panel, window, cx| {
            let hold = panel.hold_omp();
            panel.set_prompt("six", window, cx);
            panel.submit(window, cx);
            assert!(panel.prompt_queued(), "six waits in the link");
            panel.set_prompt("seven", window, cx);
            panel.stop_and_send(window, cx);
            assert_eq!(panel.next_prompt(), Some("seven"));
            drop(hold);
        })
        .unwrap();
    wait(cx, &window, "turn seven to run and end", |p| {
        p.turn() == &Turn::Idle
            && p.next_prompt().is_none()
            && p.transcript().iter().any(|l| l.contains("seven"))
    });
    let transcript = window.update(cx, |p, _, _| p.transcript()).unwrap();
    assert!(
        !transcript.iter().any(|l| l.contains("six")),
        "six never ran: {transcript:?}"
    );
    assert_ready(cx, &window);
}

fn click(vcx: &mut VisualTestContext, selector: &'static str) {
    vcx.update(|window, _| window.refresh());
    vcx.run_until_parked();
    let bounds = vcx
        .debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} is not on screen"));
    vcx.simulate_click(bounds.center(), gpui::Modifiers::none());
}

fn ready(p: &CedianPanel) -> bool {
    matches!(p.connection(), Connection::Ready { .. })
}

fn assert_ready(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>) {
    let connection = window.update(cx, |p, _, _| p.connection().clone()).unwrap();
    assert!(
        matches!(connection, Connection::Ready { .. }),
        "{connection:?}"
    );
}

fn review_turn(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>) -> (u32, bool) {
    window.update(cx, |p, _, _| p.review_turn()).unwrap()
}

fn set_prompt(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>, text: &str) {
    window
        .update(cx, |p, window, cx| p.set_prompt(text, window, cx))
        .unwrap();
}

fn prompt(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>) -> String {
    window.update(cx, |p, _, cx| p.prompt_text(cx)).unwrap()
}

fn submit(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>, text: &str) -> Turn {
    window
        .update(cx, |panel, window, cx| {
            panel.set_prompt(text, window, cx);
            panel.submit(window, cx);
            panel.turn().clone()
        })
        .unwrap()
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
