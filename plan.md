# Plan: Await Initial Worktree Scan Completion + Task Inventory Readiness

## Goal

Provide an async primitive on `Project` that resolves when:

1. All visible worktrees have finished their initial scan (git repos detected, entries populated)
2. The task inventory has been updated with any worktree-local task files discovered during scanning

## Key Existing Infrastructure

- `LocalWorktree.is_scanning` — `watch::channel<bool>` toggled by the background scanner
- `LocalWorktree::scan_complete()` — future that awaits `is_scanning` becoming `false`
- `RemoteWorktree` — **has no `scan_complete()` or `is_scanning`**. Uses `completed_scan_id` + `wait_for_snapshot(scan_id)` via proto messages with `is_last_update`
- `WorktreeStoreEvent::WorktreeAdded` / `WorktreeRemoved` — emitted when worktrees join/leave
- `SettingsObserver` subscribes to `UpdatedEntries` on each worktree → **spawns async work** (`cx.spawn().detach()`) to load task files from disk → then calls `Inventory::update_file_based_tasks`
- `Project::git_scans_complete()` — test-only (`#[cfg(feature = "test-support")]`), joins `scan_complete()` for all worktrees then waits on git repo barriers
- `postage::barrier` — widely used in the codebase for one-shot "done" signaling
- `.shared()` (`futures::future::Shared`) — widely used for shareable futures

## Critical Timing Issue

`scan_complete()` resolving does **not** guarantee the task inventory is up to date. The event propagation is:

1. `BackgroundScanner` sends `ScanState::Updated { scanning: false, snapshot, changes }`
2. `scan_state_updater` processes this in a single `Entity::update` closure:
   - Sets `is_scanning = false`
   - Calls `set_snapshot()` which synchronously emits `Event::UpdatedEntries`
3. `SettingsObserver` receives `UpdatedEntries` synchronously, but **spawns a detached async task** to load file contents from disk before calling `task_store.update_user_tasks()`
4. `scan_complete()` resolves (watcher sees `is_scanning = false`)

So there's a gap: `scan_complete()` resolves, but the `SettingsObserver`'s spawned task to load `.zed/tasks.json` and update the inventory hasn't finished yet.

## Implementation

### 1. Use a `watch::channel<bool>` on `WorktreeStore` (mirroring `is_scanning`)

`WorktreeStore` already owns the worktree lifecycle — it adds/removes worktrees, subscribes to their events, and is an `Entity` accessible from `Project` and many sub-stores. It's the natural place to track aggregate scan state.

```rust
// In WorktreeStore struct
initial_scan_complete: (watch::Sender<bool>, watch::Receiver<bool>),
```

The channel defaults to `false`. The `watch::Receiver` is `Clone`, so multiple callers can subscribe. Adding/removing worktrees writes to the `Sender`, and all awaiting futures automatically see the change.

Note: There is no `_initial_scan_monitor: Task<()>` field. Instead, `spawn_initial_scan_monitor` stores its task transiently — the caller (e.g. `add()` / `remove_worktree()`) is responsible for spawning and detaching the monitor task as needed. This avoids an extra field on the struct.

### 2. Add `WorktreeStore::wait_for_initial_scan(&self) -> impl Future<Output = ()>`

```rust
pub fn wait_for_initial_scan(&self) -> impl Future<Output = ()> {
    let mut rx = self.initial_scan_complete.1.clone();
    async move {
        let mut done = *rx.borrow();
        while !done {
            if let Some(value) = rx.recv().await {
                done = value;
            } else {
                break;
            }
        }
    }
}
```

Same pattern as `LocalWorktree::scan_complete()`. Returns a future (not a `Task`) that resolves when the watch channel reads `true`.

`Project` can expose a thin wrapper:

```rust
// On Project
pub fn wait_for_initial_scan(&self, cx: &App) -> impl Future<Output = ()> {
    self.worktree_store.read(cx).wait_for_initial_scan()
}
```

### 3. Drive the watch channel from `WorktreeStore::add` / `remove_worktree`

In `WorktreeStore::add`, after inserting the worktree:

```rust
if worktree.read(cx).is_visible() {
    *self.initial_scan_complete.0.borrow_mut() = false;
    self.spawn_initial_scan_monitor(cx);
}
```

In `WorktreeStore::remove_worktree`, after removing:

```rust
self.spawn_initial_scan_monitor(cx);
```

This keeps the logic co-located with the worktree lifecycle — no need for `Project` to handle it in `on_worktree_store_event`.

### 4. The scan monitor task

```rust
fn spawn_initial_scan_monitor(&mut self, cx: &mut Context<Self>) {
    let scan_futures: Vec<_> = self.visible_worktrees(cx)
        .filter_map(|wt| {
            let wt = wt.read(cx);
            wt.as_local().map(|local| local.scan_complete())
        })
        .collect();

    // Drop any previous monitor task (it becomes irrelevant)
    self._initial_scan_monitor = cx.spawn(async move |this, cx| {
        futures::future::join_all(scan_futures).await;
        this.update(cx, |this, _cx| {
            *this.initial_scan_complete.0.borrow_mut() = true;
        }).ok();
    });
}
```

Replacing `_initial_scan_monitor` drops the old task (the old set of worktrees is stale).

### 5. Unified `wait_for_snapshot` on both worktree variants

`LocalWorktree` now has `snapshot_subscriptions: VecDeque<(usize, oneshot::Sender<()>)>` and `wait_for_snapshot(scan_id)`, matching the existing `RemoteWorktree` pattern. Subscriptions are drained in `set_snapshot()` when `completed_scan_id` catches up. This means both variants support the same `wait_for_snapshot(scan_id)` API.

The `WorktreeStore` scan monitor can use `wait_for_snapshot(1)` on any worktree (local or remote) to await the initial scan, since `scan_id` starts at `1` and `completed_scan_id` starts at `0`.

A unified `Worktree::wait_for_snapshot` on the enum can dispatch to either variant:

```rust
// On Worktree enum
pub fn wait_for_snapshot(&mut self, scan_id: usize) -> impl Future<Output = Result<()>> {
    match self {
        Worktree::Local(local) => local.wait_for_snapshot(scan_id).boxed(),
        Worktree::Remote(remote) => remote.wait_for_snapshot(scan_id).boxed(),
    }
}
```

Previously only `RemoteWorktree` had this. The old plan mentioned adding a `scan_complete()` on the enum, but `wait_for_snapshot` is more general and already proven in the codebase:

```rust
// Old plan reference (no longer needed)
// Worktree::Local(local) => local.scan_complete().boxed(),
// Worktree::Remote(remote) => remote.wait_for_snapshot(remote.scan_id)
                .map(|_| ())
                .boxed()
        }
    }
}
```

This removes the `as_local()?` filter — all worktrees participate.

### 6. Task inventory: `TaskStore` pending update tracking

The async work that updates the task inventory lives in `SettingsObserver` — it spawns one task per `UpdatedEntries` event to load files from disk, then synchronously calls `task_store.update_user_tasks()`. `TaskStore` itself is purely synchronous and has no awareness of pending updates.

We add a barrier-based tracking mechanism to `TaskStore` so callers can await pending updates:

```rust
// In StoreState (inside TaskStore::Functional)
pending_update_barriers: Vec<postage::barrier::Receiver>,
```

**`TaskStore::register_pending_update()`** — called by `SettingsObserver` before spawning its async task. Returns a `barrier::Sender` that the spawned task holds. When the task finishes (after calling `update_user_tasks`), the sender is dropped, signaling the receiver:

```rust
pub fn register_pending_update(&mut self) -> barrier::Sender {
    let (tx, rx) = postage::barrier::channel();
    if let TaskStore::Functional(state) = self {
        state.pending_update_barriers.push(rx);
    }
    tx
}
```

**`TaskStore::pending_updates_completed()`** — drains all current barriers into a future that resolves when all pending updates are done:

```rust
pub fn pending_updates_completed(&mut self) -> impl Future<Output = ()> {
    let barriers = match self {
        TaskStore::Functional(state) => std::mem::take(&mut state.pending_update_barriers),
        TaskStore::Noop => Vec::new(),
    };
    async move {
        futures::future::join_all(
            barriers.into_iter().map(|mut rx| async move { rx.recv().await; })
        ).await;
    }
}
```

**`SettingsObserver::update_local_worktree_settings`** — before spawning the async task, register the pending update and pass the sender into the spawned task:

```rust
// Before the cx.spawn block:
let _barrier = task_store.update(cx, |store, _| store.register_pending_update());

cx.spawn(async move |this, cx| {
    let settings_contents = futures::future::join_all(settings_contents).await;
    cx.update(|cx| { /* ... update_settings ... */ });
    drop(_barrier); // signals completion (also dropped automatically at end of scope)
}).detach();
```

This gives callers a clean composable API:

```rust
// Wait for worktree scan to finish
project.wait_for_initial_scan().await;
// Then wait for any task inventory updates triggered by the scan
project.task_store.update(cx, |store, _| store.pending_updates_completed()).await;
```

The separation is semantically correct: the initial scan detects files, and the task store updates are downstream reactions. A caller who only cares about the scan can stop at step 1. A caller who needs tasks can await step 2.

## Testing

Integration test in `crates/project/tests/integration/project_tests.rs`.

### Test: `test_initial_scan_complete`

```rust
#[gpui::test]
async fn test_initial_scan_complete(cx: &mut gpui::TestAppContext) {
    init_test(cx);
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/root", json!({
        "a": {
            ".zed": { "tasks.json": r#"[{"label": "task-a", "command": "echo a"}]"# },
            ".git": {},
            "src": { "main.rs": "" }
        },
        "b": {
            ".zed": { "tasks.json": r#"[{"label": "task-b", "command": "echo b"}]"# },
            ".git": {},
            "src": { "lib.rs": "" }
        },
        "c": { "src": { "lib.rs": "" } },
    })).await;

    // 1. Create project with one worktree — scan completes during Project::test
    let project = Project::test(fs.clone(), ["/root/a".as_ref()], cx).await;

    // 1. Initial scan already done — both should resolve immediately
    let done = project.read_with(cx, |p, cx| p.wait_for_initial_scan(cx));
    done.await;
    let tasks_done = project.update(cx, |p, cx| {
        p.task_store.update(cx, |store, _| store.pending_updates_completed())
    });
    tasks_done.await;

    // Verify task inventory has task-a and git repo detected
    // ... assert task-a in inventory, git repo for /root/a detected ...

    // 2. Add a second visible worktree — initial scan resets
    let (tree_b, _) = project.update(cx, |p, cx| {
        p.find_or_create_worktree("/root/b", true, cx)
    }).await.unwrap();

    // initial_scan_complete should be false now (check via worktree_store)
    project.read_with(cx, |p, cx| {
        let store = p.worktree_store.read(cx);
        assert!(!store.initial_scan_completed());
    });

    // Wait for scan, then wait for task store to process the discovered files
    let done = project.read_with(cx, |p, cx| p.wait_for_initial_scan(cx));
    done.await;
    let tasks_done = project.update(cx, |p, cx| {
        p.task_store.update(cx, |store, _| store.pending_updates_completed())
    });
    tasks_done.await;

    // Verify task inventory now has BOTH task-a and task-b
    // ... assertions ...

    // 3. Remove worktree b — should not block
    let b_id = tree_b.read_with(cx, |t, _| t.id());
    project.update(cx, |p, cx| p.remove_worktree(b_id, cx));

    let done = project.read_with(cx, |p, cx| p.wait_for_initial_scan(cx));
    done.await; // Should resolve quickly — only worktree a remains, already scanned

    // 4. Add worktree c (no tasks.json) — should still complete, inventory unchanged
    project.update(cx, |p, cx| {
        p.find_or_create_worktree("/root/c", true, cx)
    }).await.unwrap();

    let done = project.read_with(cx, |p, cx| p.wait_for_initial_scan(cx));
    done.await;
    let tasks_done = project.update(cx, |p, cx| {
        p.task_store.update(cx, |store, _| store.pending_updates_completed())
    });
    tasks_done.await;
    // task-a still present, no new tasks from c

    // 5. Add a non-visible worktree — should NOT reset initial_scan_complete
    project.update(cx, |p, cx| {
        p.find_or_create_worktree("/root/b", false /* not visible */, cx)
    }).await.unwrap();
    project.read_with(cx, |p, cx| {
        assert!(p.worktree_store.read(cx).initial_scan_completed()); // Still true
    });
}
```

### Edge Cases

| Scenario                                    | Expected Behavior                                                         |
| ------------------------------------------- | ------------------------------------------------------------------------- |
| Initial scan already complete               | `wait_for_initial_scan()` resolves immediately                            |
| Add visible worktree                        | Watch resets to `false`, new monitor spawned, awaits all visible scans    |
| Remove visible worktree                     | Monitor respawned with remaining set, resolves if all done                |
| Add non-visible worktree                    | No effect on `initial_scan_complete`                                      |
| Rapid add then remove before scan completes | Monitor is replaced on each event; final state reflects current worktrees |
| No visible worktrees                        | `initial_scan_complete` is `true` (vacuously — nothing to scan)           |

# TODO: Initial Scan Completion Feature

See `plan.md` at repo root for full design. Test is at `crates/project/tests/integration/project_tests.rs` → `test_initial_scan_complete`.

The test currently fails because all implementations are stubs.

## Done

- [x] Stub `WorktreeStore::wait_for_initial_scan()` → returns `async {}` (in `worktree_store.rs`)
- [x] Stub `WorktreeStore::initial_scan_completed()` → returns `false` (in `worktree_store.rs`)
- [x] Stub `TaskStore::pending_updates_completed()` → returns `async {}` (in `task_store.rs`)
- [x] Thin wrapper `Project::wait_for_initial_scan()` → delegates to `WorktreeStore` (in `project.rs`)
- [x] Failing integration test covering scan completion, git repo detection (`observe_new<Repository>`), and task inventory

## To implement

- [x] Add `initial_scan_complete: (watch::Sender<bool>, watch::Receiver<bool>)` field to `WorktreeStore` — added, but default value and driving logic still TBD
- [x] Implement `wait_for_initial_scan()` using the watch receiver — implemented, but depends on channel being driven correctly
- [x] Implement `initial_scan_completed()` by reading the watch channel — implemented, but depends on channel being driven correctly
- [x] Add `snapshot_subscriptions` and `wait_for_snapshot(scan_id)` to `LocalWorktree` (mirrors `RemoteWorktree` pattern)
- [x] Add unified `Worktree::wait_for_snapshot(scan_id)` on the enum that dispatches to either variant
- [ ] Implement `spawn_initial_scan_monitor()` on `WorktreeStore` — uses `wait_for_snapshot(1)` for all visible worktrees, then sets watch to `true`
- [ ] Call `spawn_initial_scan_monitor()` from `WorktreeStore::add()` (when visible) and `remove_worktree()`
- [ ] Set `initial_scan_complete` to `false` in `WorktreeStore::add()` when the added worktree is visible
- [ ] Add `pending_update_barriers: Vec<postage::barrier::Receiver>` to `TaskStore::StoreState`
- [ ] Implement `TaskStore::register_pending_update()` → returns `barrier::Sender`
- [ ] Implement `TaskStore::pending_updates_completed()` — drains barriers, joins them
- [ ] In `SettingsObserver::update_local_worktree_settings()`, call `register_pending_update()` before spawning the async task, pass the sender into the spawned closure
- [ ] Make the test pass

## Edge-case tests to add after initial implementation

- [ ] Adding a new visible worktree resets `initial_scan_completed` to false and makes `wait_for_initial_scan` wait for the new tree
- [ ] Removing a visible worktree makes the future resolve without waiting for the removed tree
- [ ] Adding a non-visible worktree does NOT reset scan completion
- [ ] Rapid add/remove before scan completes still resolves correctly
