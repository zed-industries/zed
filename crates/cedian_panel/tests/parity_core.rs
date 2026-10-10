//! S9 U11 (ADR-0057 decision 3): the parity core in the app. The real
//! panel, OMP played by fake-omp replaying `parity_core.jsonl` through the
//! real spawn profile; frames shaped as OMP 18.6.1 sends them (`wire.rs`).
//!
//! 1. One toast surface: `notice`, `extension_error`, the `notify` UI
//!    request, `config_warnings_changed` and `ttsr_triggered` each show as
//!    a toast with its level; Dismiss removes one.
//! 2. The model and thinking-level picker: connecting and opening it read `get_state`,
//!    `get_available_models` and `get_available_thinking_levels`; picking a
//!    model sends `set_model`, Next model `cycle_model`, a level
//!    `set_thinking_level`, Next level `cycle_thinking_level` (replay checks
//!    each request field by field); `thinking_level_changed` shows OMP's
//!    level and `model_changed` re-reads the open picker.
//! 3. Queue modes, compaction and retry: each setting button sends its
//!    command (checked whole) and is noted in the thread; a turn's
//!    compaction, retry and fallback events are notes too; Stop retry sends
//!    `abort_retry`; Steer now on a queued follow-up sends
//!    `promote_queued_message`, and the follow-up joins the running turn
//!    with no review turn of its own.
//! 4. Under `policy = "omp"` the panel shows the OMP policy badge.
//!
//! Step 0 runs first: OMP has no model, so the panel onboards through
//! `get_login_providers` and `login` (ADR-0057 decision 3). Its login
//! frames are hand-made and stay so (ADR-0058 decision 6): recording them
//! takes a real OAuth login in the person's browser, which no record lane
//! makes for them.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

use cedian_panel::{CedianPanel, Connection, ToastLevel, Turn};
use gpui::{TestAppContext, VisualTestContext, WindowHandle};
use project::Project;
use settings::SettingsStore;
use std::path::Path;
use std::time::{Duration, Instant};

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/parity_core.jsonl"
);

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        _ => {}
    }
    let root = std::env::temp_dir().join(format!("cedian-u11-core-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("ws")).unwrap();
    std::fs::write(root.join("cedian.toml"), "schema = 1\n").unwrap();
    let root = root.canonicalize().unwrap();
    // SAFETY: single-threaded here; nothing else reads the environment yet.
    unsafe {
        std::env::set_var("CEDIAN_CONFIG", root.join("cedian.toml"));
        std::env::set_var("CEDIAN_STATE_DIR", root.join("state"));
        std::env::set_var("HOME", root.join("home"));
        std::env::set_var("CEDIAN_OMP_BINARY", slow_version(&root));
    }
    print!("test parity_core ... ");
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("parity_core"));
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
    let state = cedian_shell::state::dir(&ws).unwrap();
    cedian_fake_omp::install_replay(&state.join("omp"), Path::new(FIXTURE)).unwrap();
    let project = Project::test(fs::RealFs::new(None, cx.executor()), [ws.as_path()], cx).await;
    let opened = Instant::now();
    let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
    assert!(
        opened.elapsed() < Duration::from_secs(1),
        "the panel opens while `omp --version` is still running ({:?})",
        opened.elapsed()
    );
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    wait(cx, &window, "OMP ready", |p| {
        matches!(p.connection(), Connection::Ready { .. })
    });
    // 0. OMP has no model: the panel onboards. It lists OMP's login
    // providers, Log in runs `login`, the sign-in page opens in the system
    // browser, OMP's code prompt is a dialog, and the model OMP then has is
    // read for the picker.
    wait(cx, &window, "the login providers", |p| {
        p.onboarding().is_some_and(|o| o.providers.len() == 2)
    });
    assert!(rendered(&mut vcx, "cedian-onboarding").is_some());
    assert!(rendered(&mut vcx, "cedian-login-openai-codex").is_some());
    click(&mut vcx, "cedian-login-anthropic");
    wait(cx, &window, "OMP's code prompt", |p| {
        p.dialog_ids() == ["login-code"]
    });
    assert_eq!(
        cx.opened_url().as_deref(),
        Some("https://claude.ai/oauth/authorize?code=true&client_id=x"),
        "the sign-in page opens in the system browser"
    );
    assert!(
        rendered(&mut vcx, "cedian-login-url").is_some(),
        "its link stays shown"
    );
    window
        .update(cx, |panel, window, cx| {
            let text_box = panel.dialog("login-code").unwrap().text_box().unwrap();
            text_box.update(cx, |e, cx| e.set_text("abc#123", window, cx));
        })
        .unwrap();
    click(&mut vcx, "cedian-dialog-login-code-Submit");
    wait(cx, &window, "the login to finish", |p| {
        p.onboarding().is_none() && p.picker().model.is_some()
    });
    assert_picker(cx, &window, "anthropic/claude-x", "high");
    let progress = window.update(cx, |p, _, _| p.toasts().to_vec()).unwrap();
    assert!(
        progress.len() == 1
            && progress[0]
                .text
                .contains("Waiting for the authorization code"),
        "{progress:?}"
    );
    click(&mut vcx, "cedian-toast-0-dismiss");
    assert!(rendered(&mut vcx, "cedian-onboarding").is_none());

    // 1. Every OMP notice kind is a toast, with its level.
    submit(cx, &window, "hello");
    wait(cx, &window, "the turn's end", |p| {
        p.turn() == &Turn::Idle && p.toasts().len() == 5
    });
    let toasts = window.update(cx, |p, _, _| p.toasts().to_vec()).unwrap();
    let expect = [
        (ToastLevel::Warning, "compaction: Context is 90% full"),
        (ToastLevel::Error, "/ext/lint.ts"),
        (ToastLevel::Info, "Lint extension loaded"),
        (ToastLevel::Warning, "config warnings changed"),
        (ToastLevel::Info, "no-console"),
    ];
    for ((level, text), toast) in expect.iter().zip(&toasts) {
        assert_eq!(toast.level, *level, "{toast:?}");
        assert!(toast.text.contains(text), "{toast:?} names {text:?}");
    }
    assert!(
        toasts[1].text.contains("TypeError: x is undefined")
            && toasts[1].text.contains("tool_call")
    );
    assert!(
        rendered(&mut vcx, "cedian-toast-0").is_some(),
        "toasts render"
    );
    click(&mut vcx, "cedian-toast-0-dismiss");
    wait(cx, &window, "one toast dismissed", |p| {
        p.toasts().len() == 4
    });

    // 2. The model and thinking-level picker.
    click(&mut vcx, "cedian-picker-toggle");
    wait(cx, &window, "the picker's choices", |p| {
        p.picker().models.len() == 2 && p.picker().levels.len() == 3
    });
    assert_picker(cx, &window, "anthropic/claude-x", "high");
    click(&mut vcx, "cedian-model-openai/gpt-y");
    wait(cx, &window, "set_model", |p| {
        p.picker().model.as_deref() == Some("openai/gpt-y")
    });
    click(&mut vcx, "cedian-cycle-model");
    wait(cx, &window, "cycle_model", |p| {
        p.picker().model.as_deref() == Some("anthropic/claude-x") && level(p) == Some("low")
    });
    click(&mut vcx, "cedian-thinking-off");
    wait(cx, &window, "set_thinking_level", |p| {
        level(p) == Some("off")
    });
    click(&mut vcx, "cedian-cycle-thinking");
    wait(
        cx,
        &window,
        "cycle_thinking_level, then OMP's events",
        |p| p.picker().model.as_deref() == Some("openai/gpt-y") && level(p) == Some("medium"),
    );
    assert!(rendered(&mut vcx, "cedian-model-anthropic/claude-x").is_some());

    // 3. Queue modes, compaction and retry.
    for (button, note) in [
        ("cedian-steering-one", "steering mode: one-at-a-time"),
        ("cedian-follow-up-all", "follow-up mode: all"),
        ("cedian-interrupt-wait", "interrupt mode: wait"),
        ("cedian-auto-compaction-off", "auto-compaction off"),
        ("cedian-auto-retry-off", "auto-retry off"),
        ("cedian-compact", "compacted 1200 tokens: short"),
    ] {
        click(&mut vcx, button);
        wait(cx, &window, note, |p| {
            p.session_notes().last().map(String::as_str) == Some(note)
        });
    }
    let turn = window
        .update(cx, |p, _, _| p.review().current_turn())
        .unwrap();
    submit(cx, &window, "work");
    wait(cx, &window, "OMP retrying", |p| p.retrying());
    assert!(
        rendered(&mut vcx, "cedian-session-note-6").is_some(),
        "notes render"
    );
    click(&mut vcx, "cedian-abort-retry");
    wait(cx, &window, "the fallback", |p| {
        !p.retrying()
            && p.session_notes()
                .last()
                .is_some_and(|n| n == "default answered on fallback openai/gpt-y")
    });
    submit(cx, &window, "later");
    wait(cx, &window, "the queued follow-up", |p| {
        p.queued_follow_ups() == ["later"]
    });
    click(&mut vcx, "cedian-promote-0");
    wait(cx, &window, "the turn's end", |p| p.turn() == &Turn::Idle);
    let (notes, queued, notice) = window
        .update(cx, |p, _, _| {
            (
                p.session_notes().to_vec(),
                p.queued_follow_ups().len(),
                p.notice().map(str::to_string),
            )
        })
        .unwrap();
    for note in [
        "compacting (threshold, context-full)",
        "compacted 90000 tokens",
        "retry 1/3 in 2000 ms: overloaded",
        "retry gave up after 1: overloaded",
        "default falls back from anthropic/claude-x to openai/gpt-y: overloaded",
        "steering now: later",
    ] {
        assert!(notes.iter().any(|n| n == note), "{note:?} in {notes:?}");
    }
    assert_eq!(queued, 0);
    let after = window
        .update(cx, |p, _, _| p.review().current_turn())
        .unwrap();
    assert_eq!(
        after,
        turn + 1,
        "one review turn for the prompt and its steer"
    );
    assert_eq!(
        notice, None,
        "the promoted follow-up opened no turn of its own"
    );
    assert_connected(cx, &window);

    // 4. Under `policy = "omp"` the panel shows the badge, with what OMP's
    //    config says (ADR-0035 decision 4).
    assert!(
        window
            .update(cx, |p, _, _| p.omp_policy_badge().is_none())
            .unwrap(),
        "no badge under cedian's policy"
    );
    std::fs::write(
        root.join("cedian.toml"),
        format!(
            "schema = 1\n[projects.{:?}]\npolicy = \"omp\"\n",
            ws.display().to_string()
        ),
    )
    .unwrap();
    window
        .update(cx, |p, window, cx| p.restart(window, cx))
        .unwrap();
    wait(cx, &window, "the OMP policy badge", |p| {
        p.omp_policy_badge()
            .is_some_and(|b| b.contains("OMP policy") && b.contains("approvalMode: write"))
    });
    assert!(
        rendered(&mut vcx, "cedian-omp-policy").is_some(),
        "the badge renders"
    );
}

/// fake-omp behind a launcher whose `--version` takes 2 s, so a probe on
/// the UI thread shows as a slow panel open.
fn slow_version(root: &Path) -> std::path::PathBuf {
    use std::os::unix::fs::PermissionsExt as _;
    let path = root.join("slow-omp");
    std::fs::write(
        &path,
        format!(
            "#!/bin/sh\nif [ \"$1\" = --version ]; then /bin/sleep 2; fi\nexec {:?} \"$@\"\n",
            std::env::current_exe().unwrap()
        ),
    )
    .unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    path
}

fn level(p: &CedianPanel) -> Option<&'static str> {
    p.picker().thinking.map(|l| l.as_str())
}

fn assert_picker(
    cx: &mut TestAppContext,
    window: &WindowHandle<CedianPanel>,
    model: &str,
    thinking: &str,
) {
    let (m, t) = window
        .update(cx, |p, _, _| (p.picker().model.clone(), level(p)))
        .unwrap();
    assert_eq!((m.as_deref(), t), (Some(model), Some(thinking)));
}

fn submit(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>, text: &str) {
    window
        .update(cx, |panel, window, cx| {
            panel.set_prompt(text, window, cx);
            panel.submit(window, cx);
        })
        .unwrap();
}

fn click(vcx: &mut VisualTestContext, selector: &'static str) {
    let bounds = rendered(vcx, selector).unwrap_or_else(|| panic!("{selector} is not on screen"));
    vcx.simulate_click(bounds.center(), gpui::Modifiers::none());
}

fn rendered(
    vcx: &mut VisualTestContext,
    selector: &'static str,
) -> Option<gpui::Bounds<gpui::Pixels>> {
    vcx.update(|window, _| window.refresh());
    vcx.run_until_parked();
    vcx.debug_bounds(selector)
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
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        cx.run_until_parked();
        let (finished, state) = window
            .update(cx, |p, _, _| {
                (
                    done(p),
                    format!(
                        "{:?} · {:?} · {:?} · {:?}",
                        p.connection(),
                        p.turn(),
                        p.notice(),
                        p.toasts()
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
        std::thread::sleep(Duration::from_millis(50));
    }
}
