//! S5 exit: the worker worktrees in the app (ADR-0033/§43). The real panel on
//! a real repo, OMP played by fake-omp replaying `subagents.jsonl` — whose
//! `cedian_worktree_request` turn makes the tree the surface shows.
//!
//! 1. The surface reads at panel start and renders its (empty) list;
//! 2. the agent's request makes `.worktrees/w1`; a refresh shows it with its
//!    class — a fresh tree is `merged`, its branch level with the base;
//! 3. a tracked edit in the tree makes the row `wip`;
//! 4. a backdated commit plus a `running` registry row makes it `unmerged`
//!    and puts the `stuck` badge on the row (`worker_trees::read` too);
//! 5. Remove on that unmerged tree is refused, and the tree and row survive;
//! 6. Merge back lands the branch's commit on the checked-out base and the
//!    row's status becomes `done`;
//! 7. Remove is two-step on a clean, merged tree: the first click arms the
//!    confirm, Keep disarms it, and the confirm removes tree and row;
//! 8. a tree with no registry row still shows, and Remove on it is refused
//!    naming the id; a second edit plus Cleanup then removes the `wip` tree,
//!    and — the sweep being the crate's — the clean trees beside it, each
//!    removed row with them;
//! 9. a remove on a gone row is harmless — its control is no longer rendered,
//!    and the action only refuses, naming the id.
//!
//! Harness off: invoked with `--mode` (or `config`) this binary is fake-omp.

#![allow(
    clippy::disallowed_methods,
    reason = "test setup runs real git and drives the panel's own actions"
)]

use cedian_omp::SubagentStatus;
use cedian_panel::worker_trees::read;
use cedian_panel::{CedianPanel, Connection, Turn};
use cedian_worker::{Registry, TreeClass, WorkerStatus};
use gpui::{TestAppContext, VisualTestContext, WindowHandle};
use project::Project;
use settings::SettingsStore;
use std::path::Path;
use std::time::{Duration, Instant};

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/subagents.jsonl"
);

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--mode") => std::process::exit(cedian_fake_omp::run(&args)),
        Some("--version") => std::process::exit(cedian_fake_omp::version()),
        Some("config") => std::process::exit(cedian_fake_omp::config_get(&args)),
        _ => {}
    }
    let root = std::env::temp_dir().join(format!("cedian-s5-workers-{}", std::process::id()));
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
    print!("test worker_trees ... ");
    gpui::run_test_once(
        0,
        Box::new(move |dispatcher| {
            let exec = std::sync::Arc::new(dispatcher.clone());
            let mut cx = TestAppContext::build(dispatcher.clone(), Some("worker_trees"));
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
    std::fs::write(ws.join("session.jsonl"), "").unwrap();
    std::fs::write(ws.join("notes.txt"), "base\n").unwrap();
    git(&ws, &["init", "-q", "-b", "main"]);
    git(&ws, &["config", "user.email", "t@t"]);
    git(&ws, &["config", "user.name", "t"]);
    git(&ws, &["add", "notes.txt"]);
    git(&ws, &["commit", "-q", "-m", "base"]);
    let state = cedian_shell::state::dir(&ws).unwrap();
    cedian_fake_omp::install_replay(&state.join("omp"), Path::new(FIXTURE)).unwrap();

    let project = Project::test(fs::RealFs::new(None, cx.executor()), [ws.as_path()], cx).await;
    let window = cx.add_window(|window, cx| CedianPanel::new(project.clone(), window, cx));
    let mut vcx = VisualTestContext::from_window(window.into(), cx);
    wait(cx, &window, "OMP ready", |p| {
        matches!(p.connection(), Connection::Ready { .. })
    });

    // 1. The surface starts at panel start, with nothing to show yet.
    wait(cx, &window, "the first worker read", |p| {
        p.worker_trees().is_some()
    });
    assert_eq!(
        window
            .update(cx, |p, _, _| p.worker_trees().map(|rows| rows.len()))
            .unwrap(),
        Some(0),
        "a repo with no cedian worktree lists none"
    );
    assert!(
        rendered(&mut vcx, "cedian-workers").is_none(),
        "an empty surface stays out of the way"
    );

    // A tree the test makes itself: a clean, merged one, for the two-step
    // Remove and for showing a stray remove on a gone row harms nothing else.
    spawn_tracked(&ws, &state, "plain");

    // 2. OMP asks the app for worktrees (`subagents.jsonl`'s turn): the
    // earlier subagent exchanges are that fixture's prefix, replayed here so
    // the recording reaches its `cedian_worktree_request` frames.
    submit(cx, &window, "fan out");
    wait(cx, &window, "two running subagents", |p| {
        statuses(p)
            == [
                ("sa-1", SubagentStatus::Running),
                ("sa-2", SubagentStatus::Running),
            ]
    });
    type_steer(cx, &window, "sa-1", "focus on routing");
    click(&mut vcx, "cedian-subagent-sa-1-steer");
    wait(cx, &window, "the steer to land", |p| {
        subagent(p, "sa-1").description == "routing only"
    });
    type_steer(cx, &window, "sa-2", "and the tests");
    click(&mut vcx, "cedian-subagent-sa-2-steer");
    wait(cx, &window, "the refusal on the row", |p| {
        p.subagent_note("sa-2")
            .is_some_and(|n| n.contains("Subagent not running: sa-2"))
    });
    click(&mut vcx, "cedian-subagent-sa-2-cancel");
    wait(cx, &window, "already ended", |p| {
        subagent(p, "sa-2").status == SubagentStatus::Completed
    });
    click(&mut vcx, "cedian-subagent-sa-1-cancel");
    wait(cx, &window, "the turn's end", |p| {
        p.turn() == &Turn::Idle
            && statuses(p)
                == [
                    ("sa-1", SubagentStatus::Aborted),
                    ("sa-2", SubagentStatus::Completed),
                ]
    });
    assert_connected(cx, &window);

    submit(cx, &window, "make trees");
    wait(cx, &window, "the worktree turn", |p| {
        p.turn() == &Turn::Idle && p.transcript().iter().any(|l| l == "User: make trees")
    });
    assert!(
        ws.join(".worktrees/w1/notes.txt").exists(),
        "the request made the tree"
    );
    click(&mut vcx, "cedian-workers-refresh");
    wait(cx, &window, "the w1 row", |p| worker(p, "w1").is_some());
    assert!(
        rendered(&mut vcx, "cedian-worker-w1").is_some(),
        "the tree the agent asked for is a row"
    );
    let row = window
        .update(cx, |p, _, _| worker(p, "w1").cloned())
        .unwrap()
        .unwrap();
    assert_eq!(
        row.state.class,
        TreeClass::Merged,
        "a fresh tree sits at base"
    );
    assert_eq!(row.state.path, ws.join(".worktrees/w1"));
    let head = row.head.expect("the request recorded a registry row");
    assert_eq!(head.status, WorkerStatus::Running, "OMP just started it");
    assert_eq!(head.task_title, "fix the casing");
    assert_eq!(head.timebox_min, 15.0, "the brief's timebox is the row's");
    assert!(!row.stuck);
    assert!(rendered(&mut vcx, "cedian-worker-w1-stuck").is_none());
    assert!(
        rendered(&mut vcx, "cedian-worker-plain-remove").is_some(),
        "a tree with no row of its own still shows"
    );

    // 3. A tracked edit in the tree: the row reads `wip`.
    std::fs::write(ws.join(".worktrees/w1/notes.txt"), "base\nworker edit\n").unwrap();
    click(&mut vcx, "cedian-workers-refresh");
    wait(cx, &window, "the wip row", |p| {
        worker(p, "w1").is_some_and(|row| row.state.class == TreeClass::Wip)
    });

    // 4. A commit the base lacks, and a `running` row whose timebox passed
    // (the commit is backdated): the row is `unmerged` and stuck.
    git(&ws.join(".worktrees/w1"), &["add", "notes.txt"]);
    git_dated(
        &ws.join(".worktrees/w1"),
        &["commit", "-q", "-m", "wip: casing"],
    );
    {
        let lock = Registry::lock(&state).unwrap();
        let (mut registry, _) = Registry::open(&state).unwrap();
        let mut head = registry.get("w1").cloned().unwrap();
        head.status = WorkerStatus::Running;
        head.timebox_min = 1.0;
        head.last_progress = None;
        registry.remove("w1");
        registry.insert(head).unwrap();
        registry.save(&state).unwrap();
        drop(lock);
    }
    click(&mut vcx, "cedian-workers-refresh");
    wait(cx, &window, "the unmerged, stuck row", |p| {
        worker(p, "w1").is_some_and(|row| row.state.class == TreeClass::Unmerged && row.stuck)
    });
    assert!(rendered(&mut vcx, "cedian-worker-w1-stuck").is_some());
    let rows = read(&ws, &state, "HEAD").unwrap();
    let w1 = rows.iter().find(|row| row.state.id == "w1").unwrap();
    assert_eq!(w1.state.class, TreeClass::Unmerged);
    assert!(w1.stuck, "read carries the stuck flag");

    // 5. Remove is two-step, and git refuses an unmerged tree: the tree and
    // its row stay, and the notice says why.
    click(&mut vcx, "cedian-worker-w1-remove");
    assert!(rendered(&mut vcx, "cedian-worker-w1-remove-confirm").is_some());
    assert!(
        rendered(&mut vcx, "cedian-worker-w1-cleanup").is_none(),
        "the armed row offers the confirm, not the actions"
    );
    click(&mut vcx, "cedian-worker-w1-remove-confirm");
    wait(cx, &window, "the refusal", |p| {
        p.notice().is_some_and(|n| n.contains("unmerged commits"))
    });
    assert!(
        ws.join(".worktrees/w1").exists(),
        "a refused remove keeps the tree"
    );
    assert!(
        window
            .update(cx, |p, _, _| worker(p, "w1").is_some())
            .unwrap()
    );
    assert!(
        std::fs::read_to_string(state.join("workers.json"))
            .unwrap()
            .contains("w1"),
        "a refused remove keeps the row"
    );

    // 6. Merge back: the branch's commit lands on the checked-out base and
    // the row's status is `done`.
    click(&mut vcx, "cedian-worker-w1-merge");
    wait(cx, &window, "the merge", |p| {
        worker(p, "w1").is_some_and(|row| {
            row.head
                .as_ref()
                .is_some_and(|head| head.status == WorkerStatus::Done)
        })
    });
    assert_eq!(git(&ws, &["log", "-1", "--format=%s"]), "wip: casing");
    assert!(ws.join(".worktrees/w1").exists());
    let row = window
        .update(cx, |p, _, _| worker(p, "w1").cloned())
        .unwrap()
        .unwrap();
    assert_eq!(
        row.state.class,
        TreeClass::Merged,
        "its work is in the base now"
    );
    assert_connected(cx, &window);

    // 7. Remove through the UI: Keep disarms, the confirm removes the clean,
    // merged tree and drops its row.
    click(&mut vcx, "cedian-worker-plain-remove");
    assert!(rendered(&mut vcx, "cedian-worker-plain-remove-confirm").is_some());
    assert!(
        rendered(&mut vcx, "cedian-worker-plain-merge").is_none(),
        "the armed row offers the confirm, not the actions"
    );
    click(&mut vcx, "cedian-worker-plain-remove-cancel");
    assert!(rendered(&mut vcx, "cedian-worker-plain-remove-cancel").is_none());
    assert!(
        window
            .update(cx, |p, _, _| worker(p, "plain").is_some())
            .unwrap(),
        "Keep changes nothing"
    );
    click(&mut vcx, "cedian-worker-plain-remove");
    click(&mut vcx, "cedian-worker-plain-remove-confirm");
    wait(cx, &window, "the removal", |p| worker(p, "plain").is_none());
    assert!(!ws.join(".worktrees/plain").exists());
    assert!(
        !std::fs::read_to_string(state.join("workers.json"))
            .unwrap()
            .contains("plain"),
        "the removed tree's row is gone"
    );
    assert!(
        window
            .update(cx, |p, _, _| worker(p, "w1").is_some())
            .unwrap(),
        "nothing else was harmed"
    );

    // 8. A tree with no registry row still shows (git is the source of
    // truth), and an action on it is refused naming the id. A second edit
    // then makes w1 `wip`, and Cleanup on that row removes it and nothing
    // else: a person who clicked one row decided about that tree alone.
    spawn_tracked(&ws, &state, "scratch");
    // The head is dropped: `cedian_worker::spawn` alone records nothing.
    let _ = cedian_worker::spawn(&ws, "orphan", "HEAD").unwrap();
    click(&mut vcx, "cedian-workers-refresh");
    wait(cx, &window, "the rowless tree", |p| {
        worker(p, "orphan").is_some_and(|row| row.head.is_none())
    });
    assert!(rendered(&mut vcx, "cedian-worker-orphan").is_some());
    click(&mut vcx, "cedian-worker-orphan-remove");
    click(&mut vcx, "cedian-worker-orphan-remove-confirm");
    wait(cx, &window, "the rowless refusal", |p| {
        p.notice()
            .is_some_and(|n| n.contains("no worker \"orphan\""))
    });
    assert!(
        ws.join(".worktrees/orphan").exists(),
        "a refused remove keeps the rowless tree"
    );
    std::fs::write(
        ws.join(".worktrees/w1/notes.txt"),
        "base\nworker edit\nagain\n",
    )
    .unwrap();
    click(&mut vcx, "cedian-workers-refresh");
    wait(cx, &window, "the wip row again", |p| {
        worker(p, "w1").is_some_and(|row| row.state.class == TreeClass::Wip)
    });
    click(&mut vcx, "cedian-worker-w1-cleanup");
    wait(cx, &window, "the cleanup", |p| {
        worker(p, "w1").is_none() && p.notice().is_some_and(|n| n.contains("removed w1"))
    });
    assert!(
        !ws.join(".worktrees/w1").exists(),
        "cleanup removed the wip tree"
    );
    for kept in ["scratch", "orphan"] {
        assert!(
            ws.join(".worktrees").join(kept).exists(),
            "one row's cleanup leaves {kept} alone"
        );
    }
    let registry = std::fs::read_to_string(state.join("workers.json")).unwrap();
    assert!(
        !registry.contains("w1") && registry.contains("scratch") && !registry.contains("orphan"),
        "w1's row went and the others stayed: {registry}"
    );

    // 9. A remove on the gone row: no control to click, and the action only
    // refuses, naming the id.
    assert!(rendered(&mut vcx, "cedian-worker-w1-remove").is_none());
    window
        .update(cx, |p, _, cx| {
            p.begin_remove_worker("w1", cx);
            p.confirm_remove_worker("w1", cx);
        })
        .unwrap();
    wait(cx, &window, "a stray remove to be refused", |p| {
        p.notice().is_some_and(|n| n.contains("no worker \"w1\""))
    });
    assert!(
        read(&ws, &state, "HEAD")
            .unwrap()
            .iter()
            .any(|row| row.state.id == "scratch"),
        "the refusal changed nothing"
    );

    // 10. Cleanup needs no registry row: it reads git, so the trees nobody
    // recorded are not stuck forever. Each click decides one tree.
    click(&mut vcx, "cedian-worker-scratch-cleanup");
    wait(cx, &window, "the recorded tree to go", |p| {
        worker(p, "scratch").is_none()
    });
    click(&mut vcx, "cedian-worker-orphan-cleanup");
    wait(cx, &window, "the rowless tree to go", |p| {
        worker(p, "orphan").is_none()
    });
    for id in ["scratch", "orphan"] {
        assert!(!ws.join(".worktrees").join(id).exists(), "{id} is gone");
    }
    assert_eq!(
        window
            .update(cx, |p, _, _| p.worker_trees().map(|rows| rows.len()))
            .unwrap(),
        Some(0),
        "nothing came back"
    );
    assert!(read(&ws, &state, "HEAD").unwrap().is_empty());
    assert_connected(cx, &window);
}

/// A cedian tree with a registry row, as `cedian worker spawn` leaves one.
fn spawn_tracked(repo: &Path, state: &Path, id: &str) {
    let head = cedian_worker::spawn(repo, id, "HEAD").unwrap();
    let lock = Registry::lock(state).unwrap();
    let (mut registry, _) = Registry::open(state).unwrap();
    registry.insert(head).unwrap();
    registry.save(state).unwrap();
    drop(lock);
}

fn worker<'a>(
    p: &'a CedianPanel,
    id: &str,
) -> Option<&'a cedian_panel::worker_trees::WorkerTreeRow> {
    p.worker_trees()?.iter().find(|row| row.state.id == id)
}

fn subagent<'a>(p: &'a CedianPanel, id: &str) -> &'a cedian_agent_ui::SubagentRow {
    p.subagents()
        .rows()
        .iter()
        .find(|row| row.id == id)
        .unwrap_or_else(|| panic!("no subagent {id}"))
}

fn statuses(p: &CedianPanel) -> Vec<(&str, SubagentStatus)> {
    p.subagents()
        .rows()
        .iter()
        .map(|r| (r.id.as_str(), r.status))
        .collect()
}

/// `git -C dir args` with the environment a `git commit` hook exports
/// scrubbed, so a tree's own repo is the one that is touched.
fn git(dir: &Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// A commit whose committer time is long past, so a one-minute worker
/// timebox has elapsed without waiting for it.
fn git_dated(dir: &Path, args: &[&str]) {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .env("GIT_AUTHOR_DATE", "2001-01-01T00:00:00+0000")
        .env("GIT_COMMITTER_DATE", "2001-01-01T00:00:00+0000")
        .env_remove("GIT_DIR")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_WORK_TREE")
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}");
}

fn type_steer(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>, id: &str, text: &str) {
    window
        .update(cx, |p, window, cx| {
            let input = p
                .subagent_steer_box(id)
                .expect("a running row has a steer box");
            input.update(cx, |editor, cx| editor.set_text(text, window, cx));
        })
        .unwrap();
}

fn click(vcx: &mut VisualTestContext, selector: &'static str) {
    let bounds = rendered(vcx, selector).unwrap_or_else(|| panic!("{selector} is not on screen"));
    vcx.simulate_click(bounds.center(), gpui::Modifiers::none());
}

fn submit(cx: &mut TestAppContext, window: &WindowHandle<CedianPanel>, text: &str) {
    window
        .update(cx, |panel, window, cx| {
            panel.set_prompt(text, window, cx);
            panel.submit(window, cx);
        })
        .unwrap();
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

/// Pump the test executor while real OS threads (OMP, git) make progress.
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
                    format!(
                        "{:?} · {:?} · {:?} · {:?}",
                        p.connection(),
                        p.notice(),
                        p.turn(),
                        p.worker_trees().map(|rows| rows
                            .iter()
                            .map(|row| (row.state.id.clone(), row.state.class, row.stuck))
                            .collect::<Vec<_>>()),
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
