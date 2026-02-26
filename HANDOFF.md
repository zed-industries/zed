# Handoff: Merge Zed `main` into `ex-gpui-fixes` and Clean Up Ex-Only Infrastructure

## Branch & Repo State

- **Repo:** `/Users/nathan/src/zed`
- **Working branch:** `platform-neutral-tests` (forked from `ex-gpui-fixes` at `7d98f81`)
- **Target:** merge `main` (`c5aea77`) into this branch
- **Ex repo:** `/Users/nathan/src/worktrees/ex/main`

The `ex-gpui-fixes` branch carries ~70 commits on top of an earlier main merge (`29c6e08`). It added several GPUI features for Ex's editor tests. Some of those features are now unwanted — they conflict with main's direction or have been superseded by better approaches.

## Strategy: Remove Unwanted Features First, Then Merge

A previous attempt merged main first and resolved 8 conflicts, but ran into cascading problems because the removed features were entangled with conflict regions. The cleaner approach:

1. **Pre-merge commit:** delete/revert the unwanted features on our branch so they don't appear in any conflict hunks.
2. **Merge main:** with the unwanted code already gone, conflicts will be smaller and purely about the features we're keeping.
3. **Post-merge verification:** `cargo check` the relevant crates.

---

## Phase 1: Pre-Merge Cleanup (on `platform-neutral-tests` branch)

### 1A. Delete files

| File | Why |
|------|-----|
| `crates/gpui/src/app/test_app.rs` (635 lines) | Duplicates main's `TestAppContext`/`VisualTestContext`. Causes `TestWindow` naming collision. |
| `crates/gpui/src/platform/headless_metal.rs` (901 lines) | macOS-only. Depends on `MetalRenderer::new_headless()` which main removed when Metal was extracted to `gpui_macos`. Not worth porting across the crate boundary. |
| `Cargo.toml.full` | Debugging artifact. |
| `branch_divergence.sh` | Debugging artifact. |
| `branch_divergence_graphviz.sh` | Debugging artifact. |
| `branches.dot` | Debugging artifact. |
| `branches.png` | Debugging artifact. |
| `crates/scheduler/full_integration_plan.md` | Planning doc, not needed in tree. |
| `STATUS.md` | Stale status doc from scheduler integration work. |

### 1B. Edit `crates/gpui/src/app.rs`

Remove two blocks that reference the deleted `test_app.rs`:

```
#[cfg(any(test, feature = "test-support"))]
pub use test_app::*;
```

```
#[cfg(any(test, feature = "test-support"))]
mod test_app;
```

**Do NOT remove** the `current_platform` import or usage — that function still exists on our branch and main moved it to `gpui_platform`, so the merge will reconcile it.

### 1C. Edit `crates/gpui/src/platform.rs`

Remove these additions (each with its `#[cfg(...)]` attribute):

1. `mod headless_metal;`
2. `pub use headless_metal::{HeadlessMetalAppContext, HeadlessMetalPlatform, HeadlessMetalWindow};`
3. `pub use mac::MacTextSystem;` (the `// Re-export MacTextSystem publicly` block)

**Do NOT remove** `current_platform()`, `background_executor()`, or `mod windows;` — those exist in some form on both branches and will be handled by the merge (main moved them to `gpui_platform`).

**Do NOT remove** the `PlatformTextSystem` trait doc comments or visibility change (`pub trait` vs `pub(crate) trait`). Main uses `#[expect(missing_docs)]` instead of per-method doc comments — the merge will pick the right version.

### 1D. Edit `crates/gpui/src/scene.rs`

Remove the entire `test_scene` module and its re-export. This includes:

- `pub mod test_scene { ... }` — contains `RenderedQuad`, `RenderedGlyph`, `SceneSnapshot`, `Diagnostic<T>`, `ErasedTypedDiagnostic` and all their impls
- `#[cfg(any(test, feature = "test-support"))] pub use test_scene::*;`
- The `diagnostics: Vec<test_scene::Diagnostic<()>>` field on `Scene` (with its `#[cfg]`)
- The `typed_diagnostics: Vec<test_scene::ErasedTypedDiagnostic>` field on `Scene` (with its `#[cfg]`)
- `self.diagnostics.clear()` and `self.typed_diagnostics.clear()` in `Scene::clear()`
- Diagnostics sorting in `Scene::finish()`
- Diagnostics cloning/inclusion in any `snapshot()` method
- The `Scene::snapshot()` method entirely (it returns the deleted `SceneSnapshot` type)
- Any diagnostics mention in `Scene::summary()` format string

**Why:** The viewport prepaint code in Ex now supersedes the scene inspection approach. We discovered that asserting on prepaint results directly is more reliable than recording side-channel metadata.

### 1E. Edit `crates/gpui/src/window.rs`

Remove these methods (each has a test-support version and a no-op version):

- `record_diagnostic` (2 versions — `#[cfg(any(test, feature = "test-support"))]` and `#[cfg(not(...))]`)
- `record_typed_diagnostic` (2 versions, same pattern)
- `scene_snapshot()` (references the deleted `test_scene::SceneSnapshot`)

### 1F. Revert `TestPlatformWindow` rename in `crates/gpui/src/platform/test/window.rs`

Our branch renamed `TestWindow` → `TestPlatformWindow` and `TestWindowState` → `TestPlatformWindowState` to disambiguate from our (now-deleted) `test_app::TestWindow`. Since `test_app.rs` is being deleted, revert the rename back to main's names:

- `TestPlatformWindow` → `TestWindow`
- `TestPlatformWindowState` → `TestWindowState`

Also applies to references in:
- `crates/gpui/src/app/test_context.rs`
- `crates/gpui/src/platform/test/platform.rs`
- `crates/gpui/src/platform.rs` (the `as_test()` method signature)

**Keep** the `simulate_resize` bug fix (the `lock.bounds.size = size;` line added before the callback check) — that's a real fix.

### 1G. Verify the cleanup compiles

```bash
cargo check -p gpui --features test-support
```

Then commit: `"Remove ex-only test infrastructure before merging main"`

---

## Phase 2: Merge `main` into `platform-neutral-tests`

```bash
git merge main
```

### Expected conflicts and resolution

After Phase 1 cleanup, conflicts should be reduced. The remaining conflicts will be in:

| File | Likely conflict | Resolution |
|------|----------------|------------|
| `crates/gpui/src/platform.rs` | `current_platform()` / `background_executor()` — our branch has them inline, main moved them to `gpui_platform` | Take main's version (delete the functions from platform.rs). The `mod windows` block was also moved. |
| `crates/gpui/src/platform/mac/status_item.rs` | modify/delete — file moved to `gpui_macos` on main | `git rm` (take main's deletion). |
| `crates/gpui/src/platform/test/window.rs` | `TestWindow` visibility (`pub` on main, `pub(crate)` on older base) | Take main's `pub struct TestWindow`. |
| `crates/gpui/src/scene.rs` | Field visibility (`pub(crate)` on our base vs `pub` on main) | Take main's `pub` visibility for all Vec fields. |
| `crates/gpui/src/text_system.rs` | `SUBPIXEL_VARIANTS_*` formatting, `RenderGlyphParams` doc style | Take main's versions (`#[expect(missing_docs)]`, shorter formatting). |
| `crates/gpui/src/text_system/line_layout.rs` | `FontRun` doc comments | Take main's version (no per-field docs, `#[expect(missing_docs)]`). |
| `crates/gpui_macos/src/gpui_macos.rs` | Our `pub use text_system::MacTextSystem` | Take main's version (`pub use platform::MacPlatform` only). |
| `crates/gpui_macos/src/metal_renderer.rs` | Our `new_headless()` / `opaque` field | Take main's version entirely. |

### What to KEEP from our branch

These features auto-merged without conflicts in the previous attempt, and should do so again:

- **Scheduler integration:** `TestDispatcher` delegates to `TestScheduler`, `SharedRng`, `is_ready()` on `Task`, element arena on `App`, scoped test draws
- **Text system extensions:** `ShapedLine::split_at()`, `ShapedLine::width()`, `LineCacheKey`, `shape_line_cached`, `try_layout_line_cached`
- **Pointer capture:** `captured_hitbox` on `Window`
- **`render_to_image`** on `PlatformWindow` trait (already exists on main with default bail)
- **Color extensions:** `Hsla` additions in `color.rs`

### Post-merge verification

```bash
cargo check -p gpui --features test-support
cargo check -p gpui_macos
cargo check -p gpui_platform
```

---

## Phase 3: Migrate Ex's editor4 Tests (separate task, in Ex repo)

After the merge lands, Ex needs to stop using `HeadlessMetalAppContext`. This is a separate task done in `/Users/nathan/src/worktrees/ex/main`.

### The problem

Ex's editor4 tests (47 call sites across 7 test files) all use `HeadlessMetalAppContext` which provided:

1. **Real text shaping** via `MacTextSystem` — editor4 layout tests need actual glyph measurements, not the zero-width glyphs from `NoopTextSystem`
2. **Screenshot capture** via headless Metal rendering — only used in `visual_tests.rs`

### Files using `HeadlessMetalAppContext`

| File | Call sites | Notes |
|------|-----------|-------|
| `cursor_movement_tests.rs` | 6 | Needs real text shaping |
| `scroll_tests.rs` | 7 | Needs real text shaping |
| `block_container_tests.rs` | 3 | Needs real text shaping |
| `cursor_height_on_headings.rs` | 1 | Needs real text shaping |
| `input_handler_tests.rs` | 1 | Needs real text shaping |
| `visual_tests.rs` | 1 | Also uses `capture_screenshot` |
| `wrap_index_tests.rs` | 1 | Needs real text shaping |

### Migration approach

The standard `TestAppContext` uses `TestPlatform` which constructs a `NoopTextSystem`. The fix is to allow plugging in a real text system.

**Option A — Small upstream change (preferred):** Add a builder method or configuration on `TestAppContext` (or `TestPlatform`) to accept a custom `Arc<dyn PlatformTextSystem>`. Something like:

```rust
// In gpui's test infrastructure
impl TestPlatform {
    pub fn with_text_system(text_system: Arc<dyn PlatformTextSystem>) -> Self { ... }
}
```

Then Ex tests do:

```rust
use gpui_macos::MacTextSystem;
let platform = TestPlatform::with_text_system(Arc::new(MacTextSystem::new()));
let cx = TestAppContext::new(platform);
```

This is a small, clean change that lives in gpui and is generally useful.

**Option B — Ex-local wrapper:** Build a thin wrapper in `editor4/tests/test_util/` that constructs its own test context with `MacTextSystem`. More self-contained but duplicates infrastructure.

**For `visual_tests.rs` screenshots:** This can be gated behind `#[cfg(target_os = "macos")]` with a `--features screenshots` flag, or dropped entirely if the viewport prepaint approach gives sufficient test coverage.

### Key constraint

`PlatformTextSystem` must be `pub` (not `pub(crate)`) for Ex to implement Option A. Our branch already made this change. Main uses `pub(crate)` — check whether the merge kept it `pub`. If not, that's a one-line fix.

---

## Quick Reference: Commit Sequence

1. `Remove ex-only test infrastructure before merging main` (Phase 1 cleanup)
2. `Merge branch 'main' into platform-neutral-tests` (Phase 2 merge)
3. (Optional) `Add text system configuration to TestPlatform` (Phase 3 upstream change)

## Verification Commands

```bash
# After Phase 1
cargo check -p gpui --features test-support

# After Phase 2
cargo check -p gpui --features test-support
cargo check -p gpui_macos
cargo check -p gpui_platform

# After Phase 3 (in Ex repo)
cargo check -p editor4
cargo test -p editor4
```
