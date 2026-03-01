# Testing Patterns

**Analysis Date:** 2026-03-01

## Test Framework

**Runner:**
- GPUI (`gpui::test` macro) - primary test framework for GUI and async code
- Standard Rust tests (via `#[test]`) for non-GUI logic
- Config: Rust 1.93 toolchain via `rust-toolchain.toml`

**Assertion Library:**
- Standard `assert_eq!`, `assert!` macros
- `pretty_assertions::assert_eq` for detailed comparison output (used in `crates/editor/src/test.rs`)

**Run Commands:**
```bash
cargo test                      # Run all tests
cargo test --test *            # Run integration tests
./script/clippy                # Run clippy checks
```

## Test File Organization

**Location:**
- Co-located with source: `src/test.rs` or `src/tests/` directory within same crate
- Integration tests in `crates/*/tests/integration/` directory
- Special test modules marked with `#[cfg(test)]`

**Naming:**
- Test files: `*_test.rs`, `*_tests.rs`, or `test.rs` module
- Test modules: `#[cfg(test)] mod console;` (example: `crates/debugger_ui/src/tests/console.rs`)
- Test functions: `#[gpui::test] async fn test_<description>(...)`

**Structure:**
```
crates/debugger_ui/
├── src/
│   ├── debugger_panel.rs
│   ├── tests.rs                    # Re-exports shared test utilities
│   └── tests/
│       ├── console.rs              # #[cfg(test)] mod console;
│       ├── debugger_panel.rs       # #[cfg(test)] mod debugger_panel;
│       ├── inline_values.rs        # #[cfg(test)] mod inline_values;
│       └── variable_list.rs        # #[cfg(test)] mod variable_list;
└── Cargo.toml
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod console {
    use crate::{
        tests::{active_debug_session_panel, start_debug_session},
        *,
    };
    use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
    use project::{FakeFs, Project};

    #[gpui::test]
    async fn test_handle_output_event(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        init_test(cx);

        // Setup
        let fs = FakeFs::new(executor.clone());
        fs.insert_tree(path!("/project"), json!({ "main.rs": "..." })).await;

        // Execute
        // ...

        // Assert
        cx.run_until_parked();
        assert_eq!(expected, actual);
    }
}
```

**Patterns:**
- Test functions marked with `#[gpui::test]` or `#[perf]` (performance instrumentation)
- Function signature includes `executor: BackgroundExecutor, cx: &mut TestAppContext` for async tests
- `init_test(cx)` called first to initialize logging, settings, and test context
- Setup phase creates test state and fixtures
- `cx.run_until_parked()` drives async operations to completion
- `cx.update(...)` or `entity.update(cx, |...| ...)` for entity state mutations
- Assertions verify final state

## Mocking

**Framework:** Manual mocking through test doubles and fake implementations

**Patterns:**
```rust
// Fake file system
let fs = FakeFs::new(executor.clone());
fs.insert_tree(path!("/project"), json!({
    "main.rs": "First line\nSecond line",
})).await;

// Fake project
let project = Project::test(fs, [path!("/project").as_ref()], cx).await;

// Fake debug adapter client events
client.on_request::<StackTrace, _>(move |_, _| {
    Ok(dap::StackTraceResponse {
        stack_frames: Vec::default(),
        total_frames: None,
    })
});

client.fake_event(dap::messages::Events::Output(dap::OutputEvent {
    category: Some(dap::OutputEventCategory::Stdout),
    output: "output text".to_string(),
    // ... other fields
})).await;
```

**What to Mock:**
- File systems: `FakeFs::new()` for deterministic testing
- External services: Fake clients, test doubles for adapters
- Event streams: Fake event methods on clients
- Database connections: Test-specific database setup

**What NOT to Mock:**
- Core language parsing/compilation - use real language servers where possible
- Core editor operations - test against real `Editor` and `MultiBuffer` types
- GPUI entity and context operations - use real `TestAppContext` and `VisualTestContext`

## Fixtures and Factories

**Test Data:**
```rust
// Marked text with cursor positions using symbols
cx.assert_editor_state("hjklˇ");          // ˇ marks cursor
cx.assert_editor_state("«hjklˇ»");        // « » mark selection

// Marked text for marked_text_ranges
let (unmarked_text, text_ranges) = marked_text_ranges(marked_text, true);
select_ranges(editor, marked_text, window, cx);
```

**Location:**
- Test utilities in `crates/*/src/test.rs` or `crates/*/src/tests.rs`
- Shared test helpers in `tests::`module namespace
- Example: `crates/debugger_ui/src/tests.rs` exports `init_test`, `init_test_workspace`, `start_debug_session`

**Factory Functions:**
```rust
pub fn init_test(cx: &mut gpui::TestAppContext) {
    zlog::init_test();
    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme::init(theme::LoadThemes::JustBase, cx);
        editor::init(cx);
    });
}

pub async fn init_test_workspace(
    project: &Entity<Project>,
    cx: &mut TestAppContext,
) -> WindowHandle<MultiWorkspace> {
    let workspace_handle =
        cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
    // ... initialize panels ...
    workspace_handle
}
```

## Coverage

**Requirements:** Not enforced; coverage tracking via instrumentation

**View Coverage:**
```bash
# Tests use perf macro for timing instrumentation
#[perf]
#[gpui::test]
async fn test_name(...) { ... }
```

## Test Types

**Unit Tests:**
- Test individual functions or small modules
- Use `#[gpui::test]` for GPUI-dependent code (entities, contexts, rendering)
- Use `#[test]` for pure functions
- Example: `test_handle_output_event` validates console output processing

**Integration Tests:**
- Test interactions between multiple components
- Located in `crates/*/tests/integration/` directory
- Set up full workspace or project context
- Example: `test_handle_output_event` integrates debugger UI, debug adapter, and event handling

**E2E Tests:**
- Not commonly used; integration tests serve this purpose
- Visual tests use `VisualTestContext::from_window()` for rendering verification

## Common Patterns

**Async Testing:**
```rust
#[gpui::test]
async fn test_name(cx: &mut TestAppContext) {
    init_test(cx);

    // Spawn background work
    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(...).await;

    // Drive to completion
    cx.run_until_parked();

    // Assert final state
}
```

**Error Testing:**
```rust
// Using Result unwrapping for test setup
let session = start_debug_session(&workspace, cx, |_| {}).unwrap();

// Using update_in for entity state access
workspace.update(cx, |workspace, window, cx| {
    // assertions
}).unwrap();

// Context and window updates
active_debug_session_panel(workspace, cx).update_in(cx, |item, window, cx| {
    cx.focus_self(window);
    item.running_state().clone()
});
```

**Editor Testing:**
```rust
// Test context for editors
let mut cx = VimTestContext::new(cx, true).await;
cx.simulate_keystrokes("i");
cx.assert_editor_state("testˇ");

// Buffer assertions with marked text
cx.cx.set_state("«hjklˇ»");
cx.assert_editor_state("«hjklˇ»");
```

## Timer Patterns

**In GPUI Tests:**
- Prefer `cx.background_executor().timer(duration).await` over `smol::Timer::after(...)`
- Use `cx.background_executor.timer(duration).await` in `TestAppContext` blocks
- Rationale: Avoids non-determinism; ensures work is scheduled on GPUI's dispatcher

**Example:**
```rust
#[gpui::test]
async fn test_with_delay(cx: &mut TestAppContext) {
    init_test(cx);

    // Correct: GPUI executor timer
    let executor = cx.background_executor();
    executor.timer(Duration::from_millis(100)).await;
    cx.run_until_parked();

    // Incorrect: smol::Timer causes non-determinism
    // smol::Timer::after(Duration::from_millis(100)).await;
}
```

## Test Initialization

**Standard Setup:**
```rust
#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
}

#[gpui::test]
async fn test_name(cx: &mut TestAppContext) {
    init_test(cx);  // Set up global state
    // ... test code ...
}
```

## Feature Flags

**Test Support:**
- Crates use `features = ["test-support"]` in dev-dependencies to expose test utilities
- Example: `editor = { workspace = true, features = ["test-support"] }`
- Test modules marked with `#[cfg(any(test, feature = "test-support"))]` for conditional compilation

---

*Testing analysis: 2026-03-01*
