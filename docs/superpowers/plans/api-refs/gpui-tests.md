# Writing GPUI tests in the Zed repo

## Writing GPUI tests in the Zed repo

### 1. The `#[gpui::test]` macro

Any test function annotated with `#[gpui::test]` gets a deterministic scheduler. Sync tests take `cx: &mut gpui::App` (or `TestAppContext`); async tests take `cx: &mut gpui::TestAppContext`. Multiple contexts (`cx_a`, `cx_b`) are supported for collab tests. Docs: `/Users/user/zed/crates/gpui/src/test.rs:1-27`.

Macro options (parsed in `/Users/user/zed/crates/gpui_macros/src/test.rs:14-90`): `#[gpui::test(iterations = N)]`, `#[gpui::test(retries = N)]`, `#[gpui::test(seed = N)]`, `#[gpui::test(seeds(1, 2, 3))]`, `#[gpui::test(on_failure = "path::to::fn")]`. Randomized scheduling is controlled by the `SEED` env var.

`TestAppContext` (`/Users/user/zed/crates/gpui/src/app/test_context.rs:20`) has public fields `background_executor: BackgroundExecutor`, `foreground_executor: ForegroundExecutor`, `dispatcher: TestDispatcher`, and implements `AppContext`, so `cx.new(...)`, `entity.update(cx, ...)`, `entity.read_with(cx, ...)` work directly on it (`test_context.rs:36-77`).

### 2. Minimal real UI-crate test: project_panel

`csv_preview` has only plain `#[test]` unit tests (`/Users/user/zed/crates/csv_preview/src/parser.rs:315`); `project_panel` is the canonical UI test suite.

**init_test boilerplate** — `/Users/user/zed/crates/project_panel/src/project_panel_tests.rs:10805-10822`:

```rust
pub(crate) fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        crate::init(cx);

        cx.update_global::<SettingsStore, _>(|store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project_panel.get_or_insert_default().auto_fold_dirs = Some(false);
                settings.project.worktree.file_scan_exclusions = Some(Vec::new());
            });
        });
    });
}
```

The variant that also opens editors (`init_test_with_editor`, lines 10824-10842) adds `let app_state = AppState::test(cx);` (which itself installs a test `SettingsStore`), `editor::init(cx);`, `workspace::init(app_state, cx);`. Key helpers: `SettingsStore::test(cx: &mut App)` (`/Users/user/zed/crates/settings/src/settings_store.rs:514`), `AppState::test(cx: &mut App) -> Arc<Self>` (`/Users/user/zed/crates/workspace/src/workspace.rs:1185`), `theme_settings::init(themes_to_load: LoadThemes, cx: &mut App)` (`/Users/user/zed/crates/theme_settings/src/theme_settings.rs:71`).

**A real test** — `/Users/user/zed/crates/project_panel/src/project_panel_tests.rs:24-86` (trimmed):

```rust
use gpui::{Entity, TestAppContext, VisualTestContext};
use project::FakeFs;
use serde_json::json;
use workspace::{AppState, MultiWorkspace, Workspace};

#[gpui::test]
async fn test_visible_list(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root1",
        json!({
            ".dockerignore": "",
            "a": { "0": { "q": "" } },
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/root1".as_ref()], cx).await;
    let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    let workspace = window
        .read_with(cx, |mw, _| mw.workspace().clone())
        .unwrap();
    let cx = &mut VisualTestContext::from_window(window.into(), cx);
    let panel = workspace.update_in(cx, ProjectPanel::new);
    cx.run_until_parked();
    assert_eq!(visible_entries_as_strings(&panel, 0..50, cx), &["v root1", ...]);
}
```

Signatures: `FakeFs::new(executor: gpui::BackgroundExecutor) -> Arc<Self>` (`/Users/user/zed/crates/fs/src/fs.rs:1625`); `Project::test(fs: Arc<dyn Fs>, root_paths: impl IntoIterator<Item = &Path>, cx: &mut gpui::TestAppContext) -> Entity<Project>` — async, `#[cfg(feature = "test-support")]` (`/Users/user/zed/crates/project/src/project.rs:2055-2061`); `Workspace::test_new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self` (`/Users/user/zed/crates/workspace/src/workspace.rs:7832`); `VisualTestContext::from_window(window: AnyWindowHandle, cx: &TestAppContext) -> Self` (`/Users/user/zed/crates/gpui/src/app/test_context.rs:730`).

**Cargo.toml requirement**: test-only deps go under `[dev-dependencies]` with `features = ["test-support"]`, e.g. `/Users/user/zed/crates/project_panel/Cargo.toml:56-66`:
```toml
[dev-dependencies]
gpui = { workspace = true, features = ["test-support"] }
editor = { workspace = true, features = ["test-support"] }
workspace = { workspace = true, features = ["test-support"] }
```

Optional logger init used by some suites (`/Users/user/zed/crates/language/src/buffer_tests.rs:40-44`):
```rust
#[cfg(test)]
#[ctor::ctor(unsafe)]
fn init_logger() {
    zlog::init_test();
}
```

### 3. cx.update / cx.new / entity assertions / run_until_parked

`TestAppContext` implements `AppContext`, so entities can be built and inspected directly. Real example — `/Users/user/zed/crates/language/src/buffer_tests.rs:70-93`:

```rust
#[gpui::test]
fn test_set_line_ending(cx: &mut TestAppContext) {
    let base = cx.new(|cx| Buffer::local("one\ntwo\nthree\n", cx));
    base.update(cx, |_buffer, cx| {
        cx.subscribe(&base_replica, |this, _, event, cx| { ... }).detach();
    });
    // read state:
    // base.read_with(cx, |buffer, _| assert_eq!(buffer.text(), "..."));
}
```

Sync tests can also take `cx: &mut gpui::App` directly (`buffer_tests.rs:46-68`: `fn test_line_endings(cx: &mut gpui::App)` then `cx.new(|cx| { ... })`).

For global/app-level work use `cx.update(|cx: &mut App| ...)` (`test_context.rs:206`) and `cx.update_global::<G, _>(...)` (`test_context.rs:433`).

**run_until_parked**: `pub fn run_until_parked(&self)` on both `TestAppContext` (`test_context.rs:449`) and `VisualTestContext` (`test_context.rs:738`) — "Wait until there are no more pending tasks." Call it after any operation that spawns foreground/background tasks (FS scans, entity events) before asserting. Note `dispatch_action`, `simulate_keystrokes`, and `simulate_input` on `TestAppContext`/`VisualTestContext` already call `background_executor.run_until_parked()` internally (`test_context.rs:454-494`).

Other useful `TestAppContext` methods (`test_context.rs`): `add_window(build_window) -> WindowHandle<V>` (:219), `add_empty_window() -> &mut VisualTestContext` (:266), `add_window_view(...)` (:287), `executor() -> BackgroundExecutor` (:196), `simulate_prompt_answer(&str)` (:355), `events::<Evt, T>(...)` (:539), `condition(...)` (:648 — "Prefer run_until_parked"). `VisualTestContext` adds window-scoped `dispatch_action(action)` (:743), `simulate_keystrokes("cmd-shift-p enter")` (:767), `simulate_input("abc")` (:773), mouse simulation (:778-808).

### 4. Async tests with executor timers

Per repo CLAUDE.md: use GPUI executor timers, not `smol::Timer::after`, so `run_until_parked()` tracks them. Signature: `BackgroundExecutor::timer(&self, duration: Duration) -> Task<()>` (`/Users/user/zed/crates/gpui/src/executor.rs:162`). Test-only clock control: `advance_clock(&self, duration: Duration)` — "move time forward. This does not run any tasks, but does make timers ready" (`executor.rs:177`), `tick() -> bool` (run one task), `simulate_random_delay()` (`executor.rs:171`).

Real polling loop — `/Users/user/zed/crates/agent/src/tests/mod.rs:2477-2493`:

```rust
cx.background_executor
    .timer(Duration::from_millis(10))
    .await;
...
let timeout = cx.background_executor.timer(Duration::from_secs(5));
futures::select! {
    _ = cancel_task.fuse() => {}
    _ = timeout.fuse() => {
        panic!("cancel task timed out - tool did not respond to cancellation");
    }
}
```

Timers do NOT wait for wall-clock time: the test dispatcher advances the fake clock when the scheduler parks, so a 5s timer resolves instantly once no other task can make progress (`executor.rs:186-198`, comments on `run_until_parked`). Explicit clock control example — `/Users/user/zed/crates/gpui/src/util.rs:228-251`:

```rust
#[gpui::test]
async fn test_with_timeout(cx: &mut TestAppContext) {
    let fut = cx
        .executor()
        .timer(long_duration)
        .with_timeout(short_duration, &cx.executor());
    cx.executor().advance_clock(short_duration * 2);
    futures::FutureExt::now_or_never(fut)
        .unwrap_or_else(|| panic!("timeout should have triggered"))
        .expect_err("timeout");
}
```

Note `cx.executor()` and the public field `cx.background_executor` are the same `BackgroundExecutor` (`test_context.rs:196`).

### 5. Running tests

- Per-crate: `cargo test -p project_panel` (standard; `#[gpui::test]` output "is understood by other rust test runners, so you can use it with cargo test or cargo-nextest" — `/Users/user/zed/crates/gpui/src/test.rs:7-8`).
- Workspace: docs say `cargo test --workspace` (`/Users/user/zed/docs/src/development/macos.md:58`), but on macOS a "too many open files" failure is expected; docs recommend `cargo install cargo-nextest --locked` then `cargo nextest run --workspace --no-fail-fast` (`macos.md:169-172`).
- CI uses nextest: `cargo nextest run --workspace --no-fail-fast --no-tests=warn` (`/Users/user/zed/.github/workflows/release.yml:46`) and per-package `cargo nextest run -p "$PACKAGE_NAME" --no-fail-fast --no-tests=warn` (`.github/workflows/extension_tests.yml:102`).
- Workspace nextest config: `/Users/user/zed/.config/nextest.toml` — default `slow-timeout = { period = "60s", terminate-after = 1 }` (tests that exceed 60s are killed), `package(db)` tests run single-threaded, and specific slow tests get 300s overrides. Keep new GPUI tests under 60s of real time (fake-clock timers are fine).
- Deterministic replay: rerun a flaky seeded test with `SEED=<n> cargo test -p <crate> <test_name>`; increase coverage with `#[gpui::test(iterations = 10)]`.
- Lint with `./script/clippy` (not `cargo clippy`), per repo CLAUDE.md.