# Project: AsyncApp Result Removal - Codebase Migration

## Prerequisites

This brief depends on the completion of `async-app-result-removal.md`, which:
- Adds trampoline check infrastructure in dispatchers
- Updates `AsyncApp` API with `try_update()` / `update()` pattern
- Removes `AppContext::Result<T>` associated type
- Removes `Flatten` trait

## Overview

This phase migrates the entire codebase to use the new `AsyncApp` API. Since foreground tasks are now automatically cancelled when the app dies, most callsites can remove their error handling.

## Phase 1: Audit Cross-Boundary Awaits

### Goal

Identify places where background tasks await foreground tasks, as these are the edge cases that need `try_update()`.

### Search Patterns

```
# Background tasks awaiting foreground work
background_executor().spawn(...).await

# Channels receiving from foreground tasks
receiver.recv().await  # where sender is in foreground

# Task<T> held across thread boundaries
let task: Task<T> = cx.spawn(...)  # then moved to background
```

### Known Patterns to Check

1. **cx.spawn(...).await from background** - If a background task stores an `AsyncApp` and calls `spawn`, then awaits the result
2. **Channel patterns** - Foreground sends via channel, background receives
3. **Task handles passed to background** - A `Task<T>` created in foreground, awaited in background

### Action Items

- [ ] Search for `background_spawn` calls that contain awaits on foreground work
- [ ] Audit `Task<T>` usage to find cross-boundary cases
- [ ] Create list of callsites requiring `try_update()` instead of `update()`
- [ ] Document any patterns that cannot be safely migrated

## Phase 2: Codebase Migration

### Migration Strategy

1. **Automated pass**: Use search-and-replace for simple patterns
2. **Manual review**: Handle complex cases requiring `try_update()`

### Simple Patterns (Automated)

```rust
// Before
cx.update(|app| { ... })?
cx.update(|app| { ... }).ok();
cx.update(|app| { ... }).unwrap();

// After
cx.update(|app| { ... })
```

### Complex Patterns (Manual)

```rust
// Background awaiting foreground - use try_update
cx.background_spawn(async move {
    // Before
    let result = task.await?;
    
    // After - if task could be cancelled
    let Some(result) = cx.try_update(|app| { ... }) else {
        return; // or handle gracefully
    };
});
```

### Crate Priority Order

Migrate in dependency order to catch issues early:

1. `crates/gpui` - Core framework
2. `crates/language` - Language support
3. `crates/project` - Project management
4. `crates/editor` - Editor core
5. `crates/workspace` - Workspace management
6. `crates/agent` and `crates/agent_ui` - AI agent
7. Remaining crates alphabetically

### Per-Crate Checklist

For each crate:
- [ ] Find all `AsyncApp` / `AsyncWindowContext` usage
- [ ] Categorize: simple removal vs. needs `try_update()`
- [ ] Apply changes
- [ ] Run `cargo test -p <crate>` 
- [ ] Run `./script/clippy`

## Phase 3: Testing & Cleanup

### Remove Dead Code

After migration, search for and remove:

```rust
// Dead imports
use anyhow::{Context, Result};  // if only used for AsyncApp errors

// Dead error handling
.context("app was released")
.context("window was closed")

// Unused Result type aliases
type Result<T> = anyhow::Result<T>;  // if only used for AsyncApp
```

### Update Documentation

- [ ] Update `AsyncApp` rustdoc to explain new semantics
- [ ] Update GPUI documentation/examples
- [ ] Add migration guide for external users (if any)

### New Tests

Add tests to prevent regression:

- [ ] Test: `update()` works when app is alive
- [ ] Test: `try_update()` returns `None` when app is gone
- [ ] Test: Tasks are cancelled (not panicking) when app dies
- [ ] Test: Nested tasks both cancel cleanly

### Final Validation

- [ ] `cargo test` (full suite)
- [ ] `./script/clippy` 
- [ ] Manual testing: heavy async workloads
- [ ] Manual testing: rapid window open/close
- [ ] Manual testing: quit app with pending tasks

## Estimated Scope

Based on grep analysis:
- ~500+ callsites using `AsyncApp::update()` or similar
- ~50 crates with potential changes
- Most changes are mechanical `.unwrap()` / `?` removal
- ~10-20 complex cases requiring `try_update()`

## Risk Mitigation

### Rollback Plan

If issues are discovered post-merge:
1. Revert migration commits (codebase changes)
2. Keep infrastructure changes (they're backwards compatible)
3. Re-evaluate edge cases

### Incremental Rollout

Consider migrating in stages:
1. First: `crates/gpui` only - validate core behavior
2. Second: High-traffic crates (`editor`, `workspace`, `project`)
3. Third: Remaining crates

## Files Reference

Key files for finding callsites:
- `crates/gpui/src/app/async_context.rs` - `AsyncApp` definition
- Search: `update(|`, `update_entity(`, `read_entity(`
- Search: `.unwrap()`, `.ok()`, `?` following async context calls