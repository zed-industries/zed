//! S9 U4: one prompt at a time, and every way a prompt ends reaches the
//! panel. The real panel, OMP played by fake-omp replaying four fixtures,
//! one per OMP process (`turn_lifecycle_{1,2,3,4}.jsonl`):
//!
//! 1. Stop while the prompt is still queued: it never reaches OMP (the
//!    fixture's first prompt is "one", and replay checks the text);
//! 2. while a turn runs, the Steer button sends `steer` and Send queues
//!    `follow_up` (ADR-0050 decision 3), both shown as OMP's queue chip; a
//!    steer OMP refuses keeps its text in the composer; Stop takes each
//!    queued message back out of OMP's queue (`remove_queued_message`,
//!    since OMP's `abort` keeps them and runs a kept steer), puts them back
//!    in the composer, and aborts the turn once, also when Stop comes while
//!    a follow-up's call is still in flight (OMP answers it a second late);
//! 3. a prompt OMP rejects returns the panel to idle with the reason;
//! 4. an answer OMP got but the audit lost fails the turn with exactly one
//!    abort (a second would meet the next recorded prompt and end the
//!    replay);
//! 5. Restart with a prompt still queued: the old OMP never sends it (the
//!    fixture would write `old-omp-got-four`), though it was alive and held
//!    the prompt in its queue at the restart (its thread parked by the
//!    test until the restart has closed the old link);
//! 6. Stop, then Restart while the old OMP is slow to exit and still holds
//!    the session file: one abort, and the new OMP waits for the old and
//!    opens the session, not Taken;
//! 7. Restart with a follow-up queued, while the old OMP never answers the
//!    abort and a process it started holds the session file: the follow-up
//!    goes back to the composer and its chip goes, the old OMP's group is
//!    killed and the new OMP opens the session, not Taken;
//! 8. a turn of ours never shows as a run OMP started on its own, not even
//!    between its `prompt_result` and `session_settled`;
//! 9. a steer OMP takes after our turn's last step starts a run of its
//!    own (OMP drains a queued steer); a Stop pressed before that run
//!    starts leaves it shown as running, and a second Stop aborts it.
//!
//! The frames are OMP 18.6.1's, but fixtures 1, 3 and 4 are not recorded:
//! their order (a `queue_update` before its command's response, `abort`'s
//! response with no `queue_update`, the runs OMP starts on its own) is
//! hand-ordered from OMP's RPC handlers, and the `fs_sleep`s only stretch
//! a gap real OMP leaves short.
//!
//! Their queue order is checked against OMP's own frames: in every fixture,
//! and in `turn_lifecycle_live.jsonl` once recorded
//! (`CEDIAN_U12_RECORD=turn_lifecycle CEDIAN_RECORD_OMP=<pinned omp>`, one
//! model turn: a long reply, a refused and a taken steer, a follow-up, then
//! Stop, against the pinned OMP). The record lane does not replace the four
//! fixtures: their restarts, hangs and refusals are no model's to make.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

#![allow(
    clippy::disallowed_methods,
    reason = "probing a real OS process id; no async spawn helper applies"
)]

use cedian_omp::UserAnswer;
use cedian_panel::{CedianPanel, Connection, Turn};
use gpui::{TestAppContext, VisualTestContext, WindowHandle};
use project::Project;
use settings::SettingsStore;
use std::path::Path;
use std::time::{Duration, Instant};

fn fixture(n: u8) -> String {
    format!(
        "{}/tests/fixtures/turn_lifecycle_{n}.jsonl",
        env!("CARGO_MANIFEST_DIR")
    )
}

const LIVE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/turn_lifecycle_live.jsonl"
);

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        _ => {}
    }
    let record = cedian_fake_omp::recording::armed("turn_lifecycle");
    queue_order_matches_omps();
    let root = std::env::temp_dir().join(format!("cedian-u4-turns-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("ws")).unwrap();
    std::fs::write(root.join("cedian.toml"), "schema = 1\n").unwrap();
    let root = root.canonicalize().unwrap();
    // SAFETY: single-threaded here; nothing else reads the environment yet.
    unsafe {
        std::env::set_var("CEDIAN_CONFIG", root.join("cedian.toml"));
        std::env::set_var("CEDIAN_STATE_DIR", root.join("state"));
        // A recording keeps the person's HOME: the pinned OMP's auth.
        if !record {
            std::env::set_var("HOME", root.join("home"));
        }
        std::env::set_var("CEDIAN_OMP_BINARY", std::env::current_exe().unwrap());
    }
    print!(
        "test turn_lifecycle ({}) ... ",
        if record { "record" } else { "replay" }
    );
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("turn_lifecycle"));
            let executor = gpui::ForegroundExecutor::new(exec);
            if record {
                executor.block_test(record_scenario(&mut cx, &root));
            } else {
                executor.block_test(scenario(&mut cx, &root));
            }
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
    let install = |n| cedian_fake_omp::install_replay(&sessions, Path::new(&fixture(n))).unwrap();
    install(1);

    let project = Project::test(fs::RealFs::new(None, cx.executor()), [ws.as_path()], cx).await;
    let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
    let mut vcx = VisualTestContext::from_window(window.into(), cx);

    // 1. Stop while queued, before OMP is even ready.
    let turn = window
        .update(cx, |panel, window, cx| {
            panel.set_prompt("never", window, cx);
            panel.submit(window, cx);
            assert_eq!(panel.turn(), &Turn::Queued);
            panel.stop_turn(cx);
            panel.turn().clone()
        })
        .unwrap();
    assert_eq!(turn, Turn::Stopping);
    wait(cx, &window, "the cancelled prompt to end", |p| {
        ready(p) && p.turn() == &Turn::Idle
    });
    let transcript = window.update(cx, |p, _, _| p.transcript()).unwrap();
    assert!(
        !transcript.iter().any(|l| l.contains("never")),
        "OMP never ran it: {transcript:?}"
    );

    // 2. While a turn runs, the Steer button sends `steer` and Send queues
    // a `follow_up` (replay checks each text); OMP's queue shows as a chip.
    // Stop takes both back out of OMP's queue first (replay checks each
    // text and queue), then aborts once; they go back to the composer.
    assert_eq!(submit(cx, &window, "one").0, Turn::Queued);
    wait(cx, &window, "turn one to stream", |p| {
        p.turn() == &Turn::Streaming
    });
    set_prompt(cx, &window, "/plan");
    click(&mut vcx, "cedian-steer");
    wait(cx, &window, "the refused steer", |p| {
        p.notice().is_some_and(|n| n.contains("cannot be queued"))
    });
    assert_eq!(
        prompt(cx, &window),
        "/plan",
        "a refused steer keeps its text"
    );
    set_prompt(cx, &window, "faster");
    click(&mut vcx, "cedian-steer");
    wait(cx, &window, "the steer chip", |p| {
        p.transcript()
            .iter()
            .any(|l| l.ends_with("queued: faster / "))
    });
    assert_eq!(prompt(cx, &window), "", "OMP took the steer");
    let (turn, notice) = submit(cx, &window, "two");
    assert_eq!(
        (turn, notice.as_str()),
        (Turn::Streaming, ""),
        "turn one still runs"
    );
    wait(cx, &window, "OMP to have the follow-up", |_| {
        ws.join("omp-got-two").exists()
    });
    click(&mut vcx, "cedian-stop");
    assert_eq!(
        window.update(cx, |p, _, _| p.turn().clone()).unwrap(),
        Turn::Stopping,
        "Stopping at once, though OMP has not answered the follow-up yet"
    );
    set_prompt(cx, &window, "typed meanwhile");
    wait(cx, &window, "turn one to stop", |p| p.turn() == &Turn::Idle);
    wait(cx, &window, "the queue back in the composer", |p| {
        !p.transcript().iter().any(|l| l.contains("queued:"))
    });
    assert_eq!(prompt(cx, &window), "faster\ntwo\ntyped meanwhile");
    set_prompt(cx, &window, "");
    assert_ready(cx, &window);

    // 3. OMP rejects the prompt.
    submit(cx, &window, "three");
    wait(cx, &window, "the rejected prompt", |p| {
        p.turn() == &Turn::Idle
            && p.notice()
                .is_some_and(|n| n.contains("rejected by the test"))
    });

    // 4. Unaudited answer: the turn fails, one abort.
    submit(cx, &window, "audit");
    wait(cx, &window, "the dialog", |p| {
        p.dialog_ids() == ["u4-audit"]
    });
    let turn = window
        .update(cx, |p, _, cx| {
            p.break_audit();
            p.answer("u4-audit", UserAnswer::Choice("Approve".into()), cx);
            p.turn().clone()
        })
        .unwrap();
    assert!(
        matches!(turn, Turn::Failed(ref r) if r.starts_with("audit:")),
        "{turn:?}"
    );
    wait(cx, &window, "the failed turn to settle", |p| {
        p.turn() == &Turn::Idle
    });
    let notice = window
        .update(cx, |p, _, _| p.notice().map(str::to_string))
        .unwrap();
    assert!(notice.is_some_and(|n| n.starts_with("turn failed: audit")));
    assert_ready(cx, &window);

    // 5. Restart with a prompt queued behind a parked OMP thread. The
    // thread is released only once Restart has dropped the old link, so
    // the prompt is in the queue at the restart whatever the machine's
    // speed (a driver check as the lever lost that race on CI).
    install(2);
    let old = window
        .update(cx, |panel, window, cx| {
            let hold = panel.hold_omp();
            panel.set_prompt("four", window, cx);
            panel.submit(window, cx);
            assert!(panel.prompt_queued(), "the old OMP holds four in its queue");
            let old = panel.omp_pid().unwrap();
            assert!(alive(old), "the old OMP runs");
            panel.restart(window, cx);
            drop(hold);
            old
        })
        .unwrap();
    wait(cx, &window, "the restarted OMP", ready);
    assert!(
        !ws.join("old-omp-got-four").exists(),
        "the old OMP sent the queued prompt"
    );

    assert!(!alive(old), "the old OMP ended");

    // 6. Stop, then Restart while the old OMP winds down holding the
    // session file: one abort (fixture 2 records a second as a file).
    submit(cx, &window, "slow");
    wait(cx, &window, "the slow turn", |p| {
        p.turn() == &Turn::Streaming
    });
    install(3);
    window
        .update(cx, |panel, window, cx| {
            panel.stop_turn(cx);
            panel.restart(window, cx);
        })
        .unwrap();
    wait(cx, &window, "the session after the slow OMP", |p| {
        !matches!(p.connection(), Connection::Starting)
    });
    assert_ready(cx, &window);
    assert!(
        !ws.join("old-omp-got-two-aborts").exists(),
        "Stop then Restart sent a second abort"
    );
    submit(cx, &window, "five");
    wait(cx, &window, "turn five", |p| {
        p.turn() == &Turn::Idle && p.notice().is_none()
    });
    assert_ready(cx, &window);

    // 7. Restart while the old OMP never answers the abort and a process
    // it started holds the session file: past the wait, its whole group is
    // killed and the new OMP opens the session.
    submit(cx, &window, "hang");
    wait(cx, &window, "the hanging turn", |p| {
        p.turn() == &Turn::Streaming
    });
    submit(cx, &window, "later");
    wait(cx, &window, "the follow-up chip", |p| {
        p.transcript()
            .iter()
            .any(|l| l.ends_with("queued:  / later"))
    });
    install(4);
    cedian_panel::omp_link::set_previous_exit(Duration::from_millis(500));
    window
        .update(cx, |panel, window, cx| panel.restart(window, cx))
        .unwrap();
    let transcript = window.update(cx, |p, _, _| p.transcript()).unwrap();
    assert!(
        !transcript.iter().any(|l| l.contains("queued:")),
        "the old OMP's queue died with it: {transcript:?}"
    );
    assert_eq!(prompt(cx, &window), "later");
    wait(cx, &window, "the session after the hung OMP", |p| {
        !matches!(p.connection(), Connection::Starting)
    });
    assert_ready(cx, &window);
    set_prompt(cx, &window, "");

    // 8. Our own turn is never OMP's own run, also between its result and
    // the session settling (the fixture pauses there).
    submit(cx, &window, "own");
    wait(cx, &window, "the own turn to end", |p| {
        assert!(!p.omp_runs_unprompted(), "our own turn shows as OMP's");
        p.turn() == &Turn::Idle && p.transcript().iter().any(|l| l == "User: own")
    });
    assert_ready(cx, &window);

    // 9. A steer OMP takes once our turn's last step is done starts a run
    // of its own. Stop before that run starts sends nothing (no run, no
    // queue the panel knows of); the run then shows as running, and the
    // second Stop takes the steer back (OMP has delivered it) and aborts.
    submit(cx, &window, "last");
    wait(cx, &window, "turn last's step to end", |p| {
        p.turn() == &Turn::Streaming && !p.prompt_open()
    });
    set_prompt(cx, &window, "kept");
    click(&mut vcx, "cedian-steer");
    wait(cx, &window, "the kept steer's chip", |p| {
        p.transcript()
            .iter()
            .any(|l| l.ends_with("queued: kept / "))
    });
    click(&mut vcx, "cedian-stop");
    assert_eq!(
        window.update(cx, |p, _, _| p.turn().clone()).unwrap(),
        Turn::Stopping
    );
    wait(cx, &window, "OMP's own run, shown as running", |p| {
        p.turn() == &Turn::Streaming && p.omp_runs_unprompted()
    });
    click(&mut vcx, "cedian-stop");
    wait(cx, &window, "OMP's own run to be aborted", |p| {
        ws.join("own-run-aborted").exists() && p.turn() == &Turn::Idle
    });
    let notice = window
        .update(cx, |p, _, _| p.notice().map(str::to_string))
        .unwrap()
        .unwrap_or_default();
    assert!(
        notice.contains("already taken these into a run") && notice.contains("kept"),
        "the drained steer is named, not lost silently: {notice:?}"
    );
    assert_ready(cx, &window);
}

/// The live lane: the queue and Stop of step 2 against the pinned OMP.
async fn record_scenario(cx: &mut TestAppContext, root: &Path) {
    cx.executor().allow_parking();
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });
    let ws = root.join("ws");
    let sessions = cedian_shell::state::dir(&ws).unwrap().join("omp");
    cedian_fake_omp::recording::arm_app(&sessions, root);
    let project = Project::test(fs::RealFs::new(None, cx.executor()), [ws.as_path()], cx).await;
    let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    wait(cx, &window, "OMP ready", ready);

    submit(
        cx,
        &window,
        "Write the whole numbers from 1 to 300 in English words, one per line, and nothing else.",
    );
    wait(cx, &window, "the long reply to stream", |p| {
        p.turn() == &Turn::Streaming
    });
    set_prompt(cx, &window, "/plan");
    click(&mut vcx, "cedian-steer");
    wait(cx, &window, "the refused steer", |p| {
        p.notice().is_some_and(|n| n.contains("cannot be queued"))
    });
    set_prompt(cx, &window, "faster");
    click(&mut vcx, "cedian-steer");
    wait(cx, &window, "the steer chip", |p| {
        p.transcript()
            .iter()
            .any(|l| l.ends_with("queued: faster / "))
    });
    submit(cx, &window, "two");
    wait(cx, &window, "the follow-up chip", |p| {
        p.transcript()
            .iter()
            .any(|l| l.ends_with("queued: faster / two"))
    });
    click(&mut vcx, "cedian-stop");
    wait(cx, &window, "the stopped turn", |p| {
        p.turn() == &Turn::Idle && !p.transcript().iter().any(|l| l.contains("queued:"))
    });
    assert_eq!(prompt(cx, &window), "faster\ntwo");
    cedian_fake_omp::recording::write_fixture(
        &sessions.join(cedian_fake_omp::RECORDED_FILE),
        Path::new(LIVE),
    );
    let broken = queue_order(&std::fs::read_to_string(LIVE).unwrap());
    assert!(broken.is_empty(), "OMP's own order: {broken:#?}");
}

/// The order the hand-ordered fixtures rely on, checked in each fixture and
/// in the live recording when there is one.
fn queue_order_matches_omps() {
    print!("test turn_lifecycle_queue_order ... ");
    let swapped = concat!(
        r#"{"dir":"in","frame":{"id":"s","type":"steer","message":"x"}}"#,
        "\n",
        r#"{"dir":"out","frame":{"id":"s","type":"response","command":"steer","success":true}}"#,
        "\n",
        r#"{"dir":"out","frame":{"type":"queue_update","steering":["x"],"followUp":[]}}"#,
        "\n",
        r#"{"dir":"in","frame":{"id":"a","type":"abort"}}"#,
        "\n",
        r#"{"dir":"out","frame":{"type":"queue_update","steering":[],"followUp":[]}}"#,
        "\n",
        r#"{"dir":"out","frame":{"id":"a","type":"response","command":"abort","success":true}}"#,
        "\n"
    );
    assert_eq!(queue_order(swapped).len(), 2, "the check sees both breaks");
    let mut files: Vec<String> = (1..=4).map(fixture).collect();
    if Path::new(LIVE).exists() {
        files.push(LIVE.to_string());
    }
    for file in files {
        let broken = queue_order(&std::fs::read_to_string(&file).unwrap());
        assert!(broken.is_empty(), "{file}: {broken:#?}");
    }
    println!("ok");
}

/// Where `recorded` breaks OMP's queue order: a queue command that changed
/// the queue (`steer`, `follow_up`, `remove_queued_message`) has its
/// `queue_update` before its response; `abort` answers with no
/// `queue_update`.
fn queue_order(recorded: &str) -> Vec<String> {
    let records: Vec<serde_json::Value> = recorded
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    let mut broken = Vec::new();
    for (at, record) in records.iter().enumerate() {
        let frame = &record["frame"];
        let kind = frame["type"].as_str().unwrap_or_default();
        if record["dir"] != "in"
            || !matches!(
                kind,
                "steer" | "follow_up" | "remove_queued_message" | "abort"
            )
        {
            continue;
        }
        let between: Vec<&serde_json::Value> = records[at + 1..]
            .iter()
            .filter(|r| r["dir"] == "out")
            .map(|r| &r["frame"])
            .take_while(|f| !(f["type"] == "response" && f["id"] == frame["id"]))
            .collect();
        let Some(response) = records[at + 1..]
            .iter()
            .map(|r| &r["frame"])
            .find(|f| f["type"] == "response" && f["id"] == frame["id"])
        else {
            // The OMP that took it was stopped first (a restart).
            continue;
        };
        let updated = between.iter().any(|f| f["type"] == "queue_update");
        match kind {
            "abort" if updated => broken.push(format!("abort {} sent a queue_update", frame["id"])),
            "abort" => {}
            "remove_queued_message" if response["data"]["removed"] != true => {}
            _ if response["success"] == true && !updated => broken.push(format!(
                "{kind} {} answered before its queue_update",
                frame["id"]
            )),
            _ => {}
        }
    }
    broken
}

fn set_prompt(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>, text: &str) {
    window
        .update(cx, |p, window, cx| p.set_prompt(text, window, cx))
        .unwrap();
}

fn prompt(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>) -> String {
    window.update(cx, |p, _, cx| p.prompt_text(cx)).unwrap()
}

fn click(vcx: &mut VisualTestContext, selector: &'static str) {
    vcx.update(|window, _| window.refresh());
    vcx.run_until_parked();
    let bounds = vcx
        .debug_bounds(selector)
        .unwrap_or_else(|| panic!("{selector} is not on screen"));
    vcx.simulate_click(bounds.center(), gpui::Modifiers::none());
}

/// Whether `pid` runs: a zombie, dead but not yet reaped, does not.
fn alive(pid: u32) -> bool {
    std::process::Command::new("/bin/ps")
        .args(["-o", "stat=", "-p", &pid.to_string()])
        .output()
        .is_ok_and(|out| {
            let stat = String::from_utf8_lossy(&out.stdout);
            out.status.success() && !stat.trim().is_empty() && !stat.trim().starts_with('Z')
        })
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

fn submit(
    cx: &mut TestAppContext,
    window: &WindowHandle<CedianPanel>,
    text: &str,
) -> (Turn, String) {
    window
        .update(cx, |panel, window, cx| {
            panel.set_prompt(text, window, cx);
            panel.submit(window, cx);
            (
                panel.turn().clone(),
                panel.notice().unwrap_or_default().to_string(),
            )
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
