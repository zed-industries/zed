# Migration Status: Move `block` from `BackgroundExecutor` to `ForegroundExecutor`

## Testing Order (Topological Sort)

Test crates in this order to catch issues early. If a tier fails, fix it before proceeding.

### Tier 0: Foundation (no `#[gpui::test]`)
```bash
cargo test -q -p scheduler
```

### Tier 1: Core Test Infrastructure
```bash
cargo test -q -p gpui_macros
cargo test -q -p gpui
```

### Tier 2: Core Dependencies
```bash
cargo test -q -p text
cargo test -q -p rope
cargo test -q -p clock
cargo test -q -p sum_tree
cargo test -q -p collections
cargo test -q -p util
```

### Tier 3: Language Infrastructure
```bash
cargo test -q -p language
cargo test -q -p lsp
cargo test -q -p buffer_diff
cargo test -q -p multi_buffer
```

### Tier 4: Project Infrastructure
```bash
cargo test -q -p worktree
cargo test -q -p project
cargo test -q -p fs
cargo test -q -p git
```

### Tier 5: Editor & Workspace
```bash
cargo test -q -p editor
cargo test -q -p workspace
cargo test -q -p command_palette
cargo test -q -p search
```

### Tier 6: Agent & Collab
```bash
cargo test -q -p agent
cargo test -q -p agent_ui
cargo test -q -p collab
cargo test -q -p collab_ui
```

### Tier 7: Application
```bash
cargo test -q -p zed
```

### Full workspace test (after all tiers pass)
```bash
cargo test --workspace
```

---

## Reference PR
https://github.com/zed-industries/zed/pull/37837

## Rationale
- If you're on a **background thread**, you're already async - just `await`. No need to block.
- If you're on the **foreground/main thread**, you may need to block the UI thread synchronously - that's when `block_on` is needed.
- `ForegroundExecutor` has a session ID that needs to be passed to the scheduler to avoid deadlocks.

## What Was Done

### 1. Added blocking methods to `ForegroundExecutor` in GPUI
**File:** `crates/gpui/src/executor.rs`
- Added `block_test()` - for test harness
- Added `block_on()` - delegates to inner scheduler's `block_on`
- Added `block_with_timeout()` - delegates to inner scheduler's `block_with_timeout`

### 2. Removed blocking methods from `BackgroundExecutor` in GPUI
**File:** `crates/gpui/src/executor.rs`
- Removed `block_test()`
- Removed `block()`
- Removed `block_with_timeout()`

### 3. Updated `Scope::drop` to use scheduler directly
**File:** `crates/gpui/src/executor.rs`
- Changed from `self.executor.block(...)` to `self.executor.inner.scheduler().block(None, future, None)`
- This bypasses the public API and calls the scheduler directly with `session_id: None`

### 4. Updated all call sites from `background_executor().block()` to `foreground_executor().block_on()`

Files updated:
- `crates/agent_ui/src/completion_provider.rs`
- `crates/agent_ui/src/language_model_selector.rs` (also added `fg_executor` field to `ModelMatcher`)
- `crates/agent_ui/src/profile_selector.rs` (also added `foreground` field to `ProfilePickerDelegate`)
- `crates/collab_ui/src/collab_panel/channel_modal.rs`
- `crates/collab_ui/src/collab_panel.rs`
- `crates/component_preview/src/component_preview_example.rs`
- `crates/extension_host/src/extension_host.rs`
- `crates/extension_host/benches/extension_compilation_benchmark.rs`
- `crates/project/src/project_settings.rs`
- `crates/project/src/project_tests.rs`
- `crates/project_symbols/src/project_symbols.rs`
- `crates/remote_server/src/unix.rs`
- `crates/storybook/src/stories/picker.rs`
- `crates/zed/src/main.rs`
- `crates/zed/src/zed.rs`
- `crates/buffer_diff/src/buffer_diff.rs`
- `crates/language/src/buffer_tests.rs`
- `crates/language/src/buffer.rs`
- `crates/language_models/src/provider/open_ai.rs`
- `crates/multi_buffer/src/multi_buffer_tests.rs`
- `crates/gpui/src/app.rs` (shutdown method)
- `crates/editor/src/display_map/wrap_map.rs`
- `crates/editor/src/indent_guides.rs`
- `crates/command_palette/src/command_palette.rs`
- `crates/agent/src/edit_agent/evals.rs`

### 5. Special cases handled

#### Worktree (background thread blocking)
**File:** `crates/worktree/src/worktree.rs`
- Changed `self.executor.block(...)` to `futures::executor::block_on(...)` 
- This is legitimate blocking on a background thread for I/O operations
- Removed unused `executor` field from `LocalSnapshot` struct

#### LiveKit (Priority::Realtime removal)
**File:** `crates/livekit_client/src/livekit_client/playback/source.rs`
- Changed `Priority::Realtime(RealtimePriority::Audio)` to `Priority::High`
- The scheduler crate's `Priority` enum doesn't have `Realtime` variant

#### Dead code warnings
**File:** `crates/buffer_diff/src/buffer_diff.rs`
- Added `#[allow(dead_code)]` to `empty()`, `unchanged()`, and `new_with_base_text()` functions

### 6. Updated `gpui_macros` test macro ✅ COMPLETED
**File:** `crates/gpui_macros/src/test.rs`

The `#[gpui::test]` macro was updated to use `ForegroundExecutor::block_test()` instead of `BackgroundExecutor::block_test()`:

Changed:
```rust
let executor = gpui::BackgroundExecutor::new(std::sync::Arc::new(dispatcher.clone()));
executor.block_test(#inner_fn_name(#inner_fn_args));
```

To:
```rust
let foreground_executor = gpui::ForegroundExecutor::new(std::sync::Arc::new(dispatcher.clone()));
foreground_executor.block_test(#inner_fn_name(#inner_fn_args));
```

### 7. Fixed remaining test compilation issues ✅ COMPLETED

#### `crates/collab_ui/src/collab_panel.rs`
- Removed redundant `.clone()` on the last usage of `executor` (line 856)

#### `crates/agent_ui/src/inline_assistant.rs`
- Extracted `foreground_executor` before `block_test` calls to avoid borrow conflicts
- Changed from `cx.executor().block_test(...)` to `foreground_executor.block_test(...)`

#### `crates/agent_ui/src/language_model_selector.rs`
- Added `.clone()` when calling `cx.foreground_executor()` since it returns a reference

#### `crates/agent_ui/src/profile_selector.rs`
- Added missing `foreground` field to `ProfilePickerDelegate` in tests

#### `crates/agent/src/edit_agent/evals.rs`
- Extracted `foreground_executor` before `block_test` call to avoid borrow conflicts

## Current Build Status
- `cargo check --workspace` - ✅ PASSES
- `./script/clippy` - ✅ PASSES

## Known Test Failures

### Pre-existing (not caused by this migration)
| Crate | Test | Status | Notes |
|-------|------|--------|-------|
| gpui | `key_dispatch::tests::test_pending_input_observers_notified_on_focus_change` | ❌ FAILS | Pre-existing failure, verified by testing before/after stash |

### Caused by Migration (need fixing)
| Crate | Test | Status | Notes |
|-------|------|--------|-------|
| (none yet) | | | |

### Hanging Tests (timeout)
| Crate | Test | Status | Notes |
|-------|------|--------|-------|
| (none yet) | | | |

## Test Progress

| Tier | Crate | Build | Tests | Notes |
|------|-------|-------|-------|-------|
| 0 | scheduler | ✅ | ✅ 13 passed | |
| 1 | gpui_macros | ✅ | ✅ 9 passed | |
| 1 | gpui | ✅ | ⚠️ 82/83 passed | 1 pre-existing failure (see above) |
| 2 | text | | | |
| 2 | rope | | | |
| 2 | clock | | | |
| 2 | sum_tree | | | |
| 2 | collections | | | |
| 2 | util | | | |
| 3 | language | | | |
| 3 | lsp | | | |
| 3 | buffer_diff | | | |
| 3 | multi_buffer | | | |
| 4 | worktree | | | |
| 4 | project | | | |
| 4 | fs | | | |
| 4 | git | | | |
| 5 | editor | | | |
| 5 | workspace | | | |
| 5 | command_palette | | | |
| 5 | search | | | |
| 6 | agent | | | |
| 6 | agent_ui | | | |
| 6 | collab | | | |
| 6 | collab_ui | | | |
| 7 | zed | | | |

## What Still Needs To Be Done

### 1. Run full test suite
Follow the tiered testing order above to validate the migration.

## Commands to Verify Progress
```bash
# Check compilation
cargo check -q --workspace

# Run clippy
./script/clippy

# Run tests (tiered - see above)
cargo test -q -p gpui
cargo test --workspace
```

## Key Insight from PR
In the PR's `Scope::drop` implementation (commit 57e5a38), blocking is done by calling the scheduler directly:
```rust
self.executor.scheduler().block(
    None,  // session_id is None for internal blocking
    async { ... }.boxed(),
    None,
);
```

This pattern allows internal blocking without exposing a public `block` method on `BackgroundExecutor`.