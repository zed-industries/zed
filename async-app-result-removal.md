# Project: Remove Result from AsyncApp Return Types

## Problem

`AsyncApp` holds a `Weak<AppCell>` to avoid leaking the app through cyclic references. Every method must upgrade this weak pointer and return `Result<T>` to handle the case where the app was dropped. This pushes error handling complexity to every callsite throughout the codebase.

```rust
// Current: Every call requires error handling
cx.update(|app| { ... })?
cx.update(|app| { ... }).ok();
cx.update(|app| { ... }).unwrap();
```

## Solution

Move the app-alive check into the executor's task execution path. Before running any task that needs the app, check if the app is still alive. If not, cancel the task by dropping its runnable. This guarantees that any code inside a foreground task only executes when the app is alive, allowing us to remove the `Result` wrapper.

## Technical Background

### How async-task Works

The `async-task` crate's `spawn()` returns a `(Runnable, Task)` pair:
- `Runnable`: Executes the future via `runnable.run()`
- `Task`: A handle to await the future's output
- Dropping a `Runnable` cancels the task
- The `schedule` closure determines where/when the runnable is dispatched

### Current Executor Architecture

Tasks are spawned with metadata and a schedule closure:
```rust
async_task::Builder::new()
    .metadata(RunnableMeta { location })
    .spawn(
        move |_| future,
        move |runnable| {
            dispatcher.dispatch(RunnableVariant::Meta(runnable), label, priority)
        },
    )
```

The dispatcher queues the runnable, and a trampoline function executes it:
```rust
extern "C" fn trampoline(runnable: *mut c_void) {
    let task = unsafe { Runnable::<RunnableMeta>::from_raw(...) };
    task.run();  // <-- We add the check BEFORE this
}
```

### Why This Is Safe

1. **Foreground tasks run on the main thread.** The App also lives on the main thread. While a task runs synchronously, nothing else on the main thread can run, so the App cannot be dropped mid-execution.

2. **The check happens before each poll.** Between await points, the task yields. When rescheduled, the trampoline checks the app status again. If dead, the task is cancelled before user code runs.

3. **Recursive cancellation handles nested awaits.** If an outer foreground task awaits an inner foreground task and the app dies:
   - Inner task scheduled → trampoline sees app dead → cancels (drops runnable)
   - Outer task woken → scheduled → trampoline sees app dead → cancels
   - Neither task's code runs after app death → no panic

4. **Background tasks cannot hold AsyncApp.** `AsyncApp` contains `Weak<Rc<...>>` which is `!Send`, so it cannot be moved into a `Send` future on a background thread.

### The One Edge Case

A background task (no app check) awaiting a foreground task (has app check):
- Foreground task cancelled when app dies
- Background task's await sees cancellation → panic

Solution: Provide `try_update()` returning `Option<T>` for these cases.

## API Changes

### AsyncApp Methods

Rename existing methods and add non-Result versions:

| Old Signature | New Signature |
|--------------|---------------|
| `fn update(...) -> Result<R>` | `fn update(...) -> R` (panics if app gone) |
| (new) | `fn try_update(...) -> Option<R>` (returns None if app gone) |

Apply the same pattern to:
- `update`
- `read_entity` / `update_entity`
- `read_global` / `update_global`
- `read_window` / `update_window`
- `new` / `reserve_entity` / `insert_entity`
- `refresh`
- `open_window`
- `subscribe`
- `has_global`

### AppContext Trait

Remove the `Result<T>` associated type entirely. All context types will now return values directly:

```rust
// Before
pub trait AppContext {
    type Result<T>;
    fn new<T>(...) -> Self::Result<Entity<T>>;
    fn update_entity<T, R>(...) -> Self::Result<R>;
    // etc.
}

impl AppContext for AsyncApp {
    type Result<T> = Result<T>;
    // ...
}

// After
pub trait AppContext {
    fn new<T>(...) -> Entity<T>;
    fn update_entity<T, R>(...) -> R;
    // etc.
}

impl AppContext for AsyncApp {
    // All methods return values directly, panicking if app is gone
    // ...
}
```

### Remove Flatten Trait

The `Flatten` trait in `gpui.rs` exists solely to handle `Result<Result<T>>` when `WeakEntity` (returns `Result`) is used with `AsyncApp` (also returns `Result`). With this change:
- `AsyncApp` methods return `T` directly
- `WeakEntity::update` etc. return `Result<T>` (only for the weak upgrade)
- No `Result<Result<T>>` case exists, so `Flatten` can be removed

### RunnableMeta

Add optional app liveness tracking using plain `sync::Arc<()>` and `sync::Weak<()>`:

```rust
/// App stores an Arc<()> that acts as a liveness sentinel.
/// When the App is dropped, this Arc is dropped, invalidating all Weak references.
pub struct App {
    // ...
    pub(crate) liveness: std::sync::Arc<()>,
    // ...
}

/// RunnableMeta stores an optional Weak<()> to check if the app is still alive.
pub struct RunnableMeta {
    pub location: &'static Location<'static>,
    /// Weak reference to check if the app is still alive before running.
    /// This is `Some` for foreground tasks spawned with app tracking.
    pub app: Option<std::sync::Weak<()>>,
}

impl RunnableMeta {
    /// Returns true if the app is still alive (or if no app tracking is configured).
    pub fn is_app_alive(&self) -> bool {
        match &self.app {
            Some(weak) => weak.strong_count() > 0,
            None => true,
        }
    }
}
```

**Why plain `sync::Arc<()>` and `sync::Weak<()>`:**

The original design used wrapper types (`AppLiveness` and `AppLivenessToken`), but these were
unnecessary indirection. Before that, `MainThreadWeak` wrapping `rc::Weak<AppCell>` with
`unsafe impl Send + Sync` was used, which was **unsound** because async-task wakers carry the
metadata and can be stored by I/O primitives and later dropped on background threads.

Using plain `sync::Weak<()>`:
- Is genuinely `Send + Sync` (no unsafe needed)
- Clone/drop are just cheap atomic ref count operations
- Safe to drop from any thread
- Automatically becomes invalid when the `Arc<()>` is dropped (when app dies)
- No wrapper types needed - the `is_app_alive()` check is on `RunnableMeta` itself

## Implementation Phases

### Phase 1: Add Trampoline Check Infrastructure (macOS + Test) ✅ COMPLETED

Start with macOS and Test dispatcher to validate the approach.

1. ✅ Add `liveness: sync::Arc<()>` field to `App` struct in `crates/gpui/src/app.rs`
2. ✅ Update `RunnableMeta` to use `Option<sync::Weak<()>>` for app field and add `is_app_alive()` method
3. ✅ Update Mac dispatcher trampoline to check `app_token.is_alive()` before `run()`:
   - `crates/gpui/src/platform/mac/dispatcher.rs`
4. ✅ Update Test dispatcher `tick()` to check `app_token.is_alive()` before `run()`:
   - `crates/gpui/src/platform/test/dispatcher.rs`
5. ✅ Add `ForegroundExecutor::spawn_context` that accepts `sync::Weak<()>`
6. ✅ Update `AsyncApp::spawn` and `AsyncWindowContext::spawn` to use `spawn_context`
7. ✅ Add `liveness: sync::Arc<()>` to `App` struct
8. ✅ Update `AsyncApp` to store `liveness_token: sync::Weak<()>` and pass it to spawn_context
9. ✅ Write tests to validate cancellation behavior

**Bug Found & Fixed:** The original `MainThreadWeak` design was unsound. Wakers from foreground
tasks can be stored by I/O primitives and dropped on background threads. Since `rc::Weak` uses
non-atomic ref counting, this caused panics. Fixed by using plain `sync::Weak<()>` which is thread-safe.
Added `test_foreground_waker_dropped_on_background_thread` to verify the fix.

**Simplification:** Removed the `AppLiveness` and `AppLivenessToken` wrapper types in favor of
using plain `sync::Arc<()>` and `sync::Weak<()>` directly. The `is_app_alive()` check is now a
method on `RunnableMeta` itself.

### Phase 2: Extend to Other Platforms

After validating on macOS:

1. Update trampoline function in Linux dispatcher:
   - `crates/gpui/src/platform/linux/dispatcher.rs`
2. Update trampoline function in Windows dispatcher:
   - `crates/gpui/src/platform/windows/dispatcher.rs`

### Phase 3: Update AsyncApp API & Remove AppContext::Result

1. Rename `update() -> Result<R>` to `try_update() -> Option<R>`
2. Add new `update() -> R` that panics if app is gone
3. Apply same pattern to all fallible methods
4. Remove `type Result<T>` from `AppContext` trait
5. Update all trait method signatures to return `R` directly
6. Remove `Flatten` trait from `gpui.rs`
7. Update `WeakEntity::update`, `WeakEntity::read_with`, etc. to remove `Flatten` bounds

## Files to Modify

### Core Changes
- `crates/gpui/src/app/async_context.rs` - AsyncApp implementation
- `crates/gpui/src/executor.rs` - RunnableMeta, spawn functions
- `crates/gpui/src/platform.rs` - RunnableVariant if needed
- `crates/gpui/src/gpui.rs` - AppContext trait (remove Result type), remove Flatten trait
- `crates/gpui/src/app/entity_map.rs` - Update WeakEntity to remove Flatten usage

### Dispatcher Changes
- `crates/gpui/src/platform/mac/dispatcher.rs` - trampoline function (Phase 1)
- `crates/gpui/src/platform/test/dispatcher.rs` - trampoline function (Phase 1)
- `crates/gpui/src/platform/linux/dispatcher.rs` - trampoline function (Phase 2)
- `crates/gpui/src/platform/windows/dispatcher.rs` - trampoline function (Phase 2)

### External Crate Changes
- `crates/eval/src/example.rs` - ExampleContext's AppContext impl (update to remove Result type)

### Realtime Tasks (No Changes Needed)
Realtime/audio tasks use a separate code path with their own thread and channel. They don't use `AsyncApp` and won't be affected.

## Testing Strategy

### Unit Tests (`crates/gpui/src/executor.rs`)

**`test_task_cancelled_when_app_dropped`**
Verifies the core mechanism: tasks don't execute after the app is gone.

Test spec:
```
spawn task that sets flag on completion
run_until_parked (task starts, blocks on channel)
quit app
assert flag was never set
```

**`test_nested_tasks_both_cancel`**
Validates recursive cancellation: awaiting a cancelled task doesn't panic when both have app checks.

Test spec:
```
spawn outer task that awaits inner task
both tasks set flags if they run after quit
quit app
assert neither flag was set (no panic from awaiting cancelled inner)
```

**`test_try_update_returns_none_when_app_gone`**
Confirms the fallback API works for edge cases like background-awaits-foreground.

Test spec:
```
get async_cx, quit app
call try_update
assert returns None
```

**`test_foreground_waker_dropped_on_background_thread`**
Verifies that foreground task wakers can be safely dropped on background threads.
This test reproduces the original bug where `MainThreadWeak` would panic.

Test spec:
```
spawn foreground task that stores its waker in shared storage
spawn background task that replaces the waker (dropping the old one on background thread)
run until parked
assert no panic (with old MainThreadWeak this would panic)
```

### GPUI Example (`crates/gpui/examples/async_cancellation.rs`)

Interactive demo for manual validation of cancellation behavior.
```
window with "Spawn Task" button
each task runs 10-second timer, updating status each second
closing window cancels pending tasks (no panic, no zombie tasks)
```

Run with: `cargo run -p gpui --example async_cancellation`

### Integration Tests (`crates/gpui/src/app/async_context.rs`)

**`test_spawn_executes_when_app_alive`**
Basic spawn/await still works.

**`test_update_entity_works`**
Entity operations via AsyncApp still work.

### Validation Checklist

1. `cargo test -p gpui --features test-support` passes
2. `cargo run -p gpui --example async_cancellation` works correctly
3. `cargo clippy -p gpui` passes (ignore pre-existing warnings)
4. Manual test: close windows with pending tasks, verify no panics


## Future Work (Separate Brief)

The following phases will be addressed in a separate brief after this work is validated:

1. **Audit Cross-Boundary Awaits** - Search for patterns where background code awaits foreground tasks and migrate to `try_update()`
2. **Codebase Migration** - Update all ~500+ callsites to remove `.unwrap()`, `?`, `.ok()` from `update()` calls
3. **Cleanup** - Remove dead error handling code, update documentation

## Note: Window Context Fallibility

Window operations (`update_window`, `read_window`) will continue to return `Result<T>` because windows can be closed independently of whether the app is alive. This is already reflected in the `AppContext` trait, where these methods return `Result<T>` directly rather than `Self::Result<T>`. 

`AsyncWindowContext::update()` internally calls `update_window`, so it will remain fallible. This is orthogonal to the app-alive check—we're removing `Result` wrappers for operations that only depend on the app being alive, not for operations that depend on a specific window existing.
