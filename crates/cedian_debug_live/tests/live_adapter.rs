//! S1's live evidence (ARCHITECTURE §22): the `cedian_debug` round trip on
//! the real Command Line Tools `lldb-dap` and a `clang -g` C program, no
//! model. Ignored by default (it needs the Command Line Tools); run with
//! `cargo test -p cedian_debug_live --test live_adapter -- --ignored`.

mod support;

use cedian_panel::debug_tool::{self, Debugger, Op, Reply};
use gpui::TestAppContext;
use std::time::Duration;

#[test]
#[ignore = "needs the Command Line Tools lldb-dap and clang"]
fn agent_breakpoint_round_trips_on_lldb_dap() {
    let Some(lldb_dap) = cedian_debug_live::lldb_dap() else {
        println!("skipped: no lldb-dap (xcrun -f lldb-dap)");
        return;
    };
    zlog::init_test();
    let root = support::scratch("live-adapter");
    let ws = support::c_workspace(&root);
    // A stuck adapter fails the run rather than hanging it.
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(120));
        eprintln!("live_adapter: no answer in 120 s");
        std::process::exit(1);
    });
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("live_adapter"));
            gpui::ForegroundExecutor::new(exec).block_test(scenario(&mut cx, &ws, lldb_dap));
            cx.run_until_parked();
            cx.update(|cx| cx.quit());
            cx.run_until_parked();
            dispatcher.drain_tasks();
        }),
    );
    let _ = std::fs::remove_dir_all(&root);
}

async fn scenario(cx: &mut TestAppContext, ws: &std::path::Path, lldb_dap: std::path::PathBuf) {
    cx.executor().allow_parking();
    support::init(cx);
    cx.update(|cx| cedian_debug_live::register(lldb_dap, cx));
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
    window
        .update(cx, |workspace, window, cx| {
            workspace.add_panel(debug_panel.clone(), window, cx)
        })
        .unwrap();
    let debugger = Debugger {
        project: project.clone(),
        workspace: Some(
            window
                .update(cx, |_, _, cx| cx.entity().downgrade())
                .unwrap(),
        ),
        window: Some(window.into()),
    };
    let call = async |op: Op| {
        let task = cx.update(|cx| debug_tool::answer(&op, &debugger, cx));
        let reply = task.await;
        println!("{op:?} -> {reply:?}");
        reply
    };

    let set = call(Op::SetBreakpoint {
        path: "main.c".into(),
        line: support::BREAK_LINE,
        condition: None,
    })
    .await;
    assert_eq!(
        set,
        Ok(Reply::Breakpoint {
            path: "main.c".into(),
            line: support::BREAK_LINE,
            set: true
        })
    );
    let stop = call(Op::Start {
        scenario: "main".into(),
        wait_stopped_ms: Some(15_000),
    })
    .await
    .unwrap();
    let Reply::Stopped {
        top,
        locals,
        session,
        ..
    } = stop
    else {
        panic!(
            "start stops at the breakpoint: {stop:?}. If lldb-dap never answered `launch`, \
             macOS is waiting to authorize the debugger: `DevToolsSecurity -status` must say \
             enabled (`sudo DevToolsSecurity -enable`)"
        );
    };
    let top = top.expect("a top frame");
    assert_eq!(
        (top.path.as_deref(), top.line),
        (Some("main.c"), u64::from(support::BREAK_LINE)),
        "{top:?}"
    );
    assert!(
        locals.iter().any(|v| v.name == "x" && v.value == "42"),
        "x = 42 in {locals:?}"
    );
    let shown = debug_panel.read_with(cx, |panel, cx| {
        panel.active_session().map(|s| {
            s.read(cx)
                .running_state()
                .read(cx)
                .session()
                .read(cx)
                .session_id()
                .0
        })
    });
    assert_eq!(
        shown,
        Some(session),
        "the Debug panel shows the agent's session"
    );
    let end = call(Op::Continue {
        at: Default::default(),
        wait_stopped_ms: Some(15_000),
    })
    .await
    .unwrap();
    assert_eq!(end, Reply::Exited { session }, "the program ran to its end");
}
