# Visual Test Runner Improvements

This document describes improvements to make the visual test runner in `crates/zed/src/visual_test_runner.rs` more deterministic, faster, and less hacky.

## Background

The visual test runner captures screenshots of Zed's UI and compares them against baseline images. It currently works but has several issues:

1. **Non-deterministic timing**: Uses `timer().await` calls scattered throughout
2. **Real filesystem I/O**: Uses `tempfile` and `RealFs` instead of `FakeFs`
3. **Dead code**: Unused variables and counters from removed print statements
4. **Code duplication**: Similar setup code repeated across tests
5. **Limited production code coverage**: Some areas use stubs where real code could run

## How to Run the Visual Tests

```bash
# Run the visual tests (compare against baselines)
cargo run -p zed --bin zed_visual_test_runner --features visual-tests

# Update baseline images (when UI intentionally changes)
UPDATE_BASELINE=1 cargo run -p zed --bin zed_visual_test_runner --features visual-tests
```

The test runner is a separate binary, not a `#[test]` function. It uses `Application::new().run()` to create a real GPUI application context.

---

## Improvement 1: Replace Timer-Based Waits with `run_until_parked()`

### Problem

The code is littered with timing-based waits like:

```rust
cx.background_executor()
    .timer(std::time::Duration::from_millis(500))
    .await;
```

These appear ~15 times in the file. They are:
- **Slow**: Adds up to several seconds of unnecessary waiting
- **Non-deterministic**: Could flake on slow CI machines
- **Arbitrary**: The durations (100ms, 200ms, 300ms, 500ms, 2s) were chosen by trial and error

### Solution

Use `run_until_parked()` which runs all pending async tasks until there's nothing left to do. This is:
- **Instant**: Returns immediately when work is complete
- **Deterministic**: Waits exactly as long as needed
- **Standard**: Used throughout Zed's test suite

### How to Implement

The challenge is that the visual test runner uses `AsyncApp` (from `cx.spawn()`), not `TestAppContext`. The `run_until_parked()` method is on `BackgroundExecutor` which is accessible via `cx.background_executor()`.

However, `run_until_parked()` is a **blocking** call that runs the executor synchronously, while the visual tests are currently written as async code. You'll need to restructure the code.

**Option A: Keep async structure, call run_until_parked between awaits**

```rust
// Before:
cx.background_executor()
    .timer(std::time::Duration::from_millis(500))
    .await;

// After - run all pending work synchronously:
cx.background_executor().run_until_parked();
```

But this won't work directly in async context because `run_until_parked()` blocks.

**Option B: Restructure to use synchronous test pattern**

The standard Zed test pattern uses `#[gpui::test]` with `TestAppContext`:

```rust
#[gpui::test]
async fn test_something(cx: &mut TestAppContext) {
    cx.run_until_parked();  // This works!
}
```

For the visual test runner, you'd need to convert from `Application::new().run()` to the test harness. This is a bigger change but would be more idiomatic.

**Option C: Use cx.refresh() + small delay for rendering**

For purely rendering-related waits, `cx.refresh()` forces a repaint. A single small delay after refresh may be acceptable for GPU readback timing:

```rust
cx.refresh().ok();
// Minimal delay just for GPU work, not async task completion
cx.background_executor()
    .timer(std::time::Duration::from_millis(16))  // ~1 frame
    .await;
```

### Locations to Change

Search for `timer(std::time::Duration` in the file. Each occurrence should be evaluated:

| Line | Current Duration | Purpose | Replacement |
|------|-----------------|---------|-------------|
| 160-162 | 500ms | Wait for worktree | `run_until_parked()` or await the task |
| 190-192 | 500ms | Wait for panel add | `run_until_parked()` |
| 205-207 | 500ms | Wait for panel render | `cx.refresh()` |
| 248-250 | 500ms | Wait for item activation | `run_until_parked()` |
| 258-260 | 2000ms | Wait for UI to stabilize | `cx.refresh()` + minimal delay |
| 294-296 | 500ms | Wait for panel close | `run_until_parked()` |
| 752-754 | 100ms | Wait for worktree scan | `run_until_parked()` |
| 860-862 | 100ms | Wait for workspace init | `run_until_parked()` |
| 881-883 | 200ms | Wait for panel ready | `run_until_parked()` |
| 893-895 | 200ms | Wait for thread view | `run_until_parked()` |
| 912-914 | 500ms | Wait for response | `run_until_parked()` |
| 937-939 | 300ms | Wait for refresh | `cx.refresh()` |
| 956-958 | 300ms | Wait for UI update | `run_until_parked()` |
| 968-970 | 300ms | Wait for refresh | `cx.refresh()` |

### How to Verify

After making changes:
1. Run the tests: `cargo run -p zed --bin zed_visual_test_runner --features visual-tests`
2. They should pass with the same baseline images
3. They should run faster (measure before/after)

---

## Improvement 2: Use `FakeFs` Instead of Real Filesystem

### Problem

The code currently uses:
```rust
let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));
let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
```

This is:
- **Slow**: Real I/O is slower than in-memory operations
- **Non-deterministic**: Filesystem timing varies
- **Messy**: Leaves temp directories on failure

### Solution

Use `FakeFs` which is an in-memory filesystem used throughout Zed's tests:

```rust
use fs::FakeFs;

let fs = FakeFs::new(cx.background_executor().clone());
fs.insert_tree(
    "/project",
    json!({
        "src": {
            "main.rs": "fn main() { println!(\"Hello\"); }"
        },
        "Cargo.toml": "[package]\nname = \"test\""
    }),
).await;
```

### How to Implement

1. **Add the dependency** in `crates/zed/Cargo.toml` if not present:
   ```toml
   fs = { workspace = true, features = ["test-support"] }
   ```

2. **Replace RealFs creation** in `init_app_state()` (around line 655):

   ```rust
   // Before:
   let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));
   
   // After:
   let fs = FakeFs::new(cx.background_executor().clone());
   ```

3. **Replace tempdir + file creation** (around lines 66-71 and 518-643):

   ```rust
   // Before:
   let temp_dir = tempfile::tempdir().expect("...");
   let project_path = temp_dir.path().join("project");
   std::fs::create_dir_all(&project_path).expect("...");
   create_test_files(&project_path);
   
   // After:
   let fs = FakeFs::new(cx.background_executor().clone());
   fs.insert_tree("/project", json!({
       "src": {
           "main.rs": include_str!("test_content/main.rs"),
           "lib.rs": include_str!("test_content/lib.rs"),
       },
       "Cargo.toml": include_str!("test_content/Cargo.toml"),
       "README.md": include_str!("test_content/README.md"),
   })).await;
   let project_path = Path::new("/project");
   ```

4. **For the test image** (around line 726):

   ```rust
   // Before:
   let temp_dir = tempfile::tempdir()?;
   let project_path = temp_dir.path().join("project");
   std::fs::create_dir_all(&project_path)?;
   let image_path = project_path.join("test-image.png");
   std::fs::write(&image_path, EMBEDDED_TEST_IMAGE)?;
   
   // After:
   fs.insert_file("/project/test-image.png", EMBEDDED_TEST_IMAGE.to_vec()).await;
   let project_path = Path::new("/project");
   ```

5. **Update `init_app_state`** to accept `fs` as a parameter instead of creating it internally.

### Reference Example

See `crates/project_panel/src/project_panel_tests.rs` lines 17-62 for a complete example of using `FakeFs` with `insert_tree()`.

### How to Verify

1. Run the tests - they should produce identical screenshots
2. Verify no temp directories are created in `/tmp` or equivalent
3. Check that tests run faster

---

## Improvement 3: Remove Dead Code

### Problem

After removing print statements, there's leftover dead code:

```rust
let _ = update_baseline;  // Line 63 - was used in removed print

let mut passed = 0;       // Lines 263-265
let mut failed = 0;
let mut updated = 0;
// ... counters incremented but never read

let _ = (passed, updated);  // Line 327 - silences warning
```

### Solution

Remove the unused code and restructure to not need counters.

### How to Implement

1. **Remove the `let _ = update_baseline;`** on line 63 - `update_baseline` is already used later

2. **Simplify test result handling**:

   ```rust
   // Before:
   let mut passed = 0;
   let mut failed = 0;
   let mut updated = 0;
   
   match test_result {
       Ok(TestResult::Passed) => passed += 1,
       Ok(TestResult::BaselineUpdated(_)) => updated += 1,
       Err(_) => failed += 1,
   }
   // ... repeat for each test ...
   
   let _ = (passed, updated);
   
   if failed > 0 {
       std::process::exit(1);
   }
   
   // After:
   let mut any_failed = false;
   
   if run_visual_test("project_panel", ...).await.is_err() {
       any_failed = true;
   }
   
   if run_visual_test("workspace_with_editor", ...).await.is_err() {
       any_failed = true;
   }
   
   if run_agent_thread_view_test(...).await.is_err() {
       any_failed = true;
   }
   
   if any_failed {
       std::process::exit(1);
   }
   ```

3. **Or collect results into a Vec**:

   ```rust
   let results = vec![
       run_visual_test("project_panel", ...).await,
       run_visual_test("workspace_with_editor", ...).await,
       run_agent_thread_view_test(...).await,
   ];
   
   if results.iter().any(|r| r.is_err()) {
       std::process::exit(1);
   }
   ```

### How to Verify

1. Run `cargo clippy -p zed --features visual-tests` - should have no warnings about unused variables
2. Run the tests - behavior should be unchanged

---

## Improvement 4: Consolidate Test Setup Code

### Problem

There's significant duplication between:
- Main workspace test setup (lines 106-260)
- Agent panel test setup (lines 713-900)

Both create:
- Projects with `Project::local()`
- Worktrees with `find_or_create_worktree()`
- Windows with `cx.open_window()`
- Wait for things to settle

### Solution

Extract common setup into helper functions.

### How to Implement

1. **Create a `TestWorkspace` struct**:

   ```rust
   struct TestWorkspace {
       window: WindowHandle<Workspace>,
       project: Entity<Project>,
   }
   
   impl TestWorkspace {
       async fn new(
           app_state: Arc<AppState>,
           size: Size<Pixels>,
           project_path: &Path,
           cx: &mut AsyncApp,
       ) -> Result<Self> {
           let project = cx.update(|cx| {
               Project::local(
                   app_state.client.clone(),
                   app_state.node_runtime.clone(),
                   app_state.user_store.clone(),
                   app_state.languages.clone(),
                   app_state.fs.clone(),
                   None,
                   false,
                   cx,
               )
           })?;
           
           // Add worktree
           let add_task = project.update(cx, |project, cx| {
               project.find_or_create_worktree(project_path, true, cx)
           })?;
           add_task.await?;
           
           // Create window
           let bounds = Bounds {
               origin: point(px(0.0), px(0.0)),
               size,
           };
           
           let window = cx.update(|cx| {
               cx.open_window(
                   WindowOptions {
                       window_bounds: Some(WindowBounds::Windowed(bounds)),
                       focus: false,
                       show: false,
                       ..Default::default()
                   },
                   |window, cx| {
                       cx.new(|cx| {
                           Workspace::new(None, project.clone(), app_state.clone(), window, cx)
                       })
                   },
               )
           })??;
           
           cx.background_executor().run_until_parked();
           
           Ok(Self { window, project })
       }
   }
   ```

2. **Create a `setup_project_panel` helper**:

   ```rust
   async fn setup_project_panel(
       workspace: &TestWorkspace,
       cx: &mut AsyncApp,
   ) -> Result<Entity<ProjectPanel>> {
       let panel_task = workspace.window.update(cx, |_ws, window, cx| {
           let weak = cx.weak_entity();
           window.spawn(cx, async move |cx| ProjectPanel::load(weak, cx).await)
       })?;
       
       let panel = panel_task.await?;
       
       workspace.window.update(cx, |ws, window, cx| {
           ws.add_panel(panel.clone(), window, cx);
           ws.open_panel::<ProjectPanel>(window, cx);
       })?;
       
       cx.background_executor().run_until_parked();
       
       Ok(panel)
   }
   ```

3. **Use helpers in tests**:

   ```rust
   // Test 1
   let workspace = TestWorkspace::new(
       app_state.clone(),
       size(px(1280.0), px(800.0)),
       &project_path,
       &mut cx,
   ).await?;
   
   setup_project_panel(&workspace, &mut cx).await?;
   open_file(&workspace, "src/main.rs", &mut cx).await?;
   
   run_visual_test("project_panel", workspace.window.into(), &mut cx, update_baseline).await?;
   ```

### How to Verify

1. Tests should produce identical screenshots
2. Code should be shorter and more readable
3. Adding new tests should be easier

---

## Improvement 5: Exercise More Production Code

### Problem

Some tests use minimal stubs where real production code could run deterministically.

### Current State (Good)

The agent thread test already runs the **real** `ReadFileTool`:
```rust
let tool = Arc::new(agent::ReadFileTool::new(...));
tool.clone().run(input, event_stream, cx).await?;
```

This is great! It exercises real tool execution.

### Opportunities for More Coverage

1. **Syntax highlighting**: Register real language grammars so the editor shows colored code

   ```rust
   // Currently just:
   let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
   
   // Could add:
   languages.register_native_grammars([
       tree_sitter_rust::LANGUAGE.into(),
       tree_sitter_markdown::LANGUAGE.into(),
   ]);
   ```

2. **File icons**: The project panel could show real file icons by registering file types

3. **Theme loading**: Currently uses `LoadThemes::JustBase`. Could load a full theme:
   ```rust
   theme::init(theme::LoadThemes::All, cx);
   ```
   (But this might make tests slower - evaluate trade-off)

4. **More tool types**: Test other tools like `ListFilesTool`, `GrepTool` that don't need network

### How to Implement Syntax Highlighting

1. Add language dependencies to `Cargo.toml`:
   ```toml
   tree-sitter-rust = { workspace = true }
   tree-sitter-markdown = { workspace = true }
   ```

2. In `init_app_state` or test setup:
   ```rust
   let languages = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
   
   // Register Rust grammar
   languages.register_native_grammars([
       ("rust", tree_sitter_rust::LANGUAGE.into()),
   ]);
   
   // Register the Rust language config
   languages.register_available_language(
       LanguageConfig {
           name: "Rust".into(),
           grammar: Some("rust".into()),
           matcher: LanguageMatcher {
               path_suffixes: vec!["rs".into()],
               ..Default::default()
           },
           ..Default::default()
       },
       tree_sitter_rust::LANGUAGE.into(),
       vec![],  // No LSP adapters needed for visual tests
   );
   ```

### How to Verify

1. Update baselines after adding syntax highlighting
2. Screenshots should show colored code instead of plain text
3. Tests should still be deterministic (same colors every time)

---

## Improvement 6: Better Test Organization

### Problem

Tests are numbered in comments but the structure is ad-hoc:
```rust
// Run Test 1: Project Panel
// Run Test 2: Workspace with Editor  
// Run Test 3: Agent Thread View
```

### Solution

Create a test registry or use a more structured approach.

### How to Implement

1. **Define tests as structs**:

   ```rust
   trait VisualTest {
       fn name(&self) -> &'static str;
       async fn setup(&self, cx: &mut AsyncApp) -> Result<AnyWindowHandle>;
       async fn run(&self, window: AnyWindowHandle, cx: &mut AsyncApp, update_baseline: bool) -> Result<TestResult>;
   }
   
   struct ProjectPanelTest;
   struct WorkspaceEditorTest;
   struct AgentThreadTest;
   
   impl VisualTest for ProjectPanelTest {
       fn name(&self) -> &'static str { "project_panel" }
       // ...
   }
   ```

2. **Run all tests from a registry**:

   ```rust
   let tests: Vec<Box<dyn VisualTest>> = vec![
       Box::new(ProjectPanelTest),
       Box::new(WorkspaceEditorTest),
       Box::new(AgentThreadTest),
   ];
   
   let mut failed = false;
   for test in tests {
       match test.run(...).await {
           Ok(_) => {},
           Err(_) => failed = true,
       }
   }
   ```

This makes it easy to:
- Add new tests (just add to the vec)
- Run specific tests (filter by name)
- See all tests in one place

---

## Improvement 7: Constants and Configuration

### Problem

Magic numbers scattered throughout:
```rust
size(px(1280.0), px(800.0))  // Window size for workspace tests
size(px(500.0), px(900.0))   // Window size for agent panel
Duration::from_millis(500)   // Various delays
const MATCH_THRESHOLD: f64 = 0.99;  // Already a constant, good!
```

### Solution

Move all configuration to constants at the top of the file.

### How to Implement

```rust
// Window sizes
const WORKSPACE_WINDOW_SIZE: Size<Pixels> = size(px(1280.0), px(800.0));
const AGENT_PANEL_WINDOW_SIZE: Size<Pixels> = size(px(500.0), px(900.0));

// Timing (if any delays are still needed after Improvement 1)
const FRAME_DELAY: Duration = Duration::from_millis(16);

// Image comparison
const MATCH_THRESHOLD: f64 = 0.99;
const PIXEL_TOLERANCE: i16 = 2;  // Currently hardcoded in pixels_are_similar()
```

---

## Summary: Recommended Order of Implementation

1. **Remove dead code** (Improvement 3) - Quick win, low risk
2. **Add constants** (Improvement 7) - Quick win, improves readability
3. **Use FakeFs** (Improvement 2) - Medium effort, big determinism win
4. **Replace timers** (Improvement 1) - Medium effort, biggest determinism win
5. **Consolidate setup** (Improvement 4) - Medium effort, maintainability win
6. **Exercise more production code** (Improvement 5) - Lower priority, nice to have
7. **Better test organization** (Improvement 6) - Lower priority, nice to have for many tests

## Testing Your Changes

After each change:

```bash
# Build to check for compile errors
cargo build -p zed --features visual-tests

# Run clippy for warnings
cargo clippy -p zed --features visual-tests

# Run the tests
cargo run -p zed --bin zed_visual_test_runner --features visual-tests

# If screenshots changed intentionally, update baselines
UPDATE_BASELINE=1 cargo run -p zed --bin zed_visual_test_runner --features visual-tests
```

Baseline images are stored in `crates/zed/test_fixtures/visual_tests/`.

## Questions?

If you get stuck:
1. Look at existing tests in `crates/project_panel/src/project_panel_tests.rs` for examples of `FakeFs` and `run_until_parked()`
2. Look at `crates/gpui/src/app/test_context.rs` for `VisualTestContext` documentation
3. The CLAUDE.md file at the repo root has Rust coding guidelines
