//! S1's live turn (ADR-0070): one model turn on the pinned OMP in which
//! OMP is asked to debug a C program on the real `lldb-dap`, in the app's
//! panel beside Zed's Debug panel, without being told which tool to use
//! (ADR-0073 decision 4); the run prints which it picked. Every dialog is
//! approved. The recording becomes
//! `cedian_panel/tests/fixtures/debug_live.jsonl`, which
//! `cedian_panel/tests/debug_live_replay.rs` replays hermetically.
//!
//! Record only (`CEDIAN_U12_RECORD=debug_live CEDIAN_RECORD_OMP=<pinned omp>`,
//! through `script/cedian-bench/run.sh record debug_live`); otherwise it
//! skips.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

mod support;

use cedian_panel::{CedianPanel, Connection, Turn};
use gpui::{AppContext as _, Entity, Modifiers, TestAppContext, VisualTestContext};
use std::path::Path;
use std::time::{Duration, Instant};

const LANE: &str = "debug_live";
const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../cedian_panel/tests/fixtures/debug_live.jsonl"
);
const PROMPT: &str = "This workspace is hosted by the cedian IDE, and the person is watching \
its debugger. main.c is built with debug info as ./main, and the IDE has a debug.json scenario \
labelled 'main' that launches it. Find the value of x at the printf line of main.c by debugging \
the program in the IDE's debugger, then let it run to its end. Report the value.";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        // Any other call is not a test run: Zed's environment loader runs
        // this binary as `--printenv` in a login shell, and running the
        // scenario there would start another OMP (in a record lane, another
        // model turn) for every launch.
        Some(other) => {
            eprintln!("{other}: not a test run");
            std::process::exit(2)
        }
        None => {}
    }
    if !cedian_fake_omp::recording::armed(LANE) {
        println!("test live_turn ... skipped (record lane {LANE} only)");
        return;
    }
    // Refused before any model turn: the turn would only time out.
    assert!(
        cedian_debug_live::debugger_authorized(),
        "Developer mode is off (`DevToolsSecurity -status`): lldb-dap cannot launch unattended; \
         `sudo DevToolsSecurity -enable` first"
    );
    let Some(lldb_dap) = cedian_debug_live::lldb_dap() else {
        panic!("the {LANE} lane needs lldb-dap (xcrun -f lldb-dap)");
    };
    let root = support::scratch("live-turn");
    let ws = support::c_workspace(&root);
    std::fs::write(root.join("cedian.toml"), "schema = 1\n").unwrap();
    // SAFETY: single-threaded here; nothing else reads the environment yet.
    // A recording keeps the person's HOME: the pinned OMP's auth.
    unsafe {
        std::env::set_var("CEDIAN_CONFIG", root.join("cedian.toml"));
        std::env::set_var("CEDIAN_STATE_DIR", root.join("state"));
        std::env::set_var("CEDIAN_OMP_BINARY", std::env::current_exe().unwrap());
    }
    print!("test live_turn ... ");
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("live_turn"));
            gpui::ForegroundExecutor::new(exec).block_test(scenario(&mut cx, &root, &ws, lldb_dap));
            cx.run_until_parked();
            cx.update(|cx| cx.quit());
            cx.run_until_parked();
            dispatcher.drain_tasks();
        }),
    );
    println!("ok");
}

async fn scenario(cx: &mut TestAppContext, root: &Path, ws: &Path, lldb_dap: std::path::PathBuf) {
    cx.executor().allow_parking();
    support::init(cx);
    cx.update(|cx| cedian_debug_live::register(lldb_dap, cx));
    let state = cedian_shell::state::dir(ws).unwrap();
    cedian_fake_omp::recording::arm_app(&state.join("omp"), root);

    let project = support::project(ws, cx).await;
    let window = support::workspace(&project, cx);
    let debug_panel = window
        .update(cx, |_, window, cx| {
            cx.spawn_in(window, async move |this, cx| {
                debugger_ui::debugger_panel::DebugPanel::load(this, cx).await
            })
        })
        .unwrap()
        .await
        .unwrap();
    let panel = window
        .update(cx, |workspace, window, cx| {
            workspace.add_panel(debug_panel, window, cx);
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
            panel.set_prompt(PROMPT, window, cx);
            panel.submit(window, cx);
        })
    })
    .unwrap();

    // The turn ends when it has run and gone idle with no dialog open,
    // whichever tool the model picked; the recording is written either way.
    let deadline = Instant::now() + cedian_fake_omp::recording::wait_limit(Duration::from_secs(60));
    let mut ran = false;
    let ended = loop {
        cx.run_until_parked();
        let (idle, dialogs) = panel.read_with(cx, |p, _| (p.turn() == &Turn::Idle, p.dialog_ids()));
        ran |= !idle;
        if ran && idle && dialogs.is_empty() {
            break true;
        }
        if Instant::now() >= deadline {
            break false;
        }
        if let Some(id) = dialogs.first() {
            let button: &'static str = format!("cedian-dialog-{id}-Approve").leak();
            vcx.run_until_parked();
            if let Some(bounds) = vcx.debug_bounds(button) {
                vcx.simulate_click(bounds.center(), Modifiers::none());
                continue;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let recorded = state.join("omp").join(cedian_fake_omp::RECORDED_FILE);
    let text = std::fs::read_to_string(&recorded).unwrap_or_default();
    let tools = |name: &str| {
        text.lines()
            .filter(|l| {
                l.contains("\"tool_execution_start\"")
                    && l.contains(&format!("\"toolName\":\"{name}\""))
            })
            .count()
    };
    // ADR-0073 decision 4: which tool the model picks is measured here.
    println!(
        "tool picked: cedian_debug calls {}, OMP's own debug {}",
        tools("cedian_debug"),
        tools("debug")
    );
    cedian_fake_omp::recording::write_fixture(&recorded, Path::new(FIXTURE));
    assert!(ended, "the live turn did not end");
}

fn wait(
    cx: &mut TestAppContext,
    panel: &Entity<CedianPanel>,
    what: &str,
    done: impl Fn(&CedianPanel) -> bool,
) {
    let deadline = Instant::now() + cedian_fake_omp::recording::wait_limit(Duration::from_secs(30));
    loop {
        cx.run_until_parked();
        if panel.read_with(cx, |p, _| done(p)) {
            return;
        }
        assert!(Instant::now() < deadline, "timed out waiting for {what}");
        std::thread::sleep(Duration::from_millis(50));
    }
}
