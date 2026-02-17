# Plan: Fix multi-workspace serialization reliability

## Problem Summary

PR #49380 introduces multi-workspace serialization fixes but has several issues
that undermine its goals. This plan addresses all of them.

---

## Fix 1: Await flush tasks before quitting (Critical)

### Problem

In the quit handler (`crates/zed/src/zed.rs`), `flush_serialization` returns a
`Task<()>` representing an async DB write, but it's `.detach()`ed. Then
`cx.quit()` is called immediately. The process can exit before the writes
complete, defeating the entire purpose of flushing.

### Change

In `fn quit` in `crates/zed/src/zed.rs`, collect the `Task<()>` values returned
by `flush_serialization` and await them all before calling `cx.quit()`:

```rust
// Replace the current flush loop with:
let mut flush_tasks = Vec::new();
for window in &workspace_windows {
    if let Some(tasks) = window
        .update(cx, |multi_workspace, window, cx| {
            multi_workspace
                .workspaces()
                .iter()
                .map(|workspace| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.flush_serialization(window, cx)
                    })
                })
                .collect::<Vec<_>>()
        })
        .log_err()
    {
        flush_tasks.extend(tasks);
    }
}
futures::future::join_all(flush_tasks).await;

cx.update(|cx| cx.quit());
```

---

## Fix 2: Re-serialize MultiWorkspace state after database_id is assigned (Significant)

### Problem

In `MultiWorkspace::create_workspace` (`crates/workspace/src/multi_workspace.rs`),
`self.activate(...)` calls `self.serialize(cx)`, which writes
`active_workspace_id: self.workspace().read(cx).database_id()`. But the new
workspace's `database_id` is still `None` at this point — the async `next_id()`
task hasn't run yet. So the multi-workspace state is persisted with
`active_workspace_id: None`, and nothing ever corrects it.

### Change

In the `cx.spawn_in` closure inside `create_workspace`, after setting the
database_id, re-serialize the `MultiWorkspace` state. Use `_this`
(currently ignored as `_this`) to call `serialize`:

```rust
cx.spawn_in(window, async move |this, cx| {
    let workspace_id = crate::persistence::DB.next_id().await?;
    this.update(cx, |this, cx| {
        weak_workspace
            .update(cx, |workspace, _cx| {
                workspace.set_database_id(workspace_id);
            })
            .log_err();
        this.serialize(cx);
    })
    .log_err();
    anyhow::Ok(())
})
.detach_and_log_err(cx);
```

Note: `serialize` must also be made `pub(crate)` (or just `pub`) since it's
currently `fn serialize(&self, cx: &mut App)` — but it's called from within the
same file so visibility isn't actually a problem here. The key change is using
`this` instead of `_this`.

---

## Fix 3: Trigger initial workspace serialization after database_id is set (Significant)

### Problem

`serialize_workspace_internal` has a guard: `let Some(database_id) =
self.database_id() else { return Task::ready(()); }`. Until `database_id` is
`Some`, the individual workspace is never serialized. `next_id()` creates a bare
DB row, but the workspace's content (panes, docks, session_id, window_id) is
never written until something else triggers the throttled `serialize_workspace`.
For a newly created empty workspace, nothing may trigger that before quit.

### Change

In the same `cx.spawn_in` closure (from Fix 2), after setting the database_id,
also trigger the workspace's own serialization. This requires the window, which
is available via `update_in`:

```rust
cx.spawn_in(window, async move |this, cx| {
    let workspace_id = crate::persistence::DB.next_id().await?;
    this.update_in(cx, |this, window, cx| {
        weak_workspace
            .update(cx, |workspace, cx| {
                workspace.set_database_id(workspace_id);
                workspace.serialize_workspace(window, cx);
            })
            .log_err();
        this.serialize(cx);
    })
    .log_err();
    anyhow::Ok(())
})
.detach_and_log_err(cx);
```

`serialize_workspace` is currently `fn serialize_workspace(&mut self, window:
&mut Window, cx: &mut Context<Self>)` — it's the throttled entry point. Since
this is the very first serialization for this workspace, the throttle timer
won't be active, so it will fire immediately. This also requires either making
`serialize_workspace` `pub(crate)` or adding a dedicated method. Since
`flush_serialization` is already `pub`, calling that would also work and is
simpler — it bypasses the throttle and serializes immediately:

```rust
weak_workspace
    .update(cx, |workspace, cx| {
        workspace.set_database_id(workspace_id);
        workspace.flush_serialization(window, cx).detach();
    })
    .log_err();
```

However, `flush_serialization` takes `&mut Window` and `&mut App` while we're
inside `workspace.update(cx, ...)` where `cx` is `&mut Context<Workspace>`.
`Context<Workspace>` derefs to `App`, so this should work. But confirm during
implementation that the borrow checker is happy — if not, split into two
sequential calls.

**Preferred combined form for Fixes 2 and 3:**

```rust
let weak_workspace = new_workspace.downgrade();
cx.spawn_in(window, async move |this, cx| {
    let workspace_id = crate::persistence::DB.next_id().await?;
    this.update_in(cx, |this, window, cx| {
        if let Some(workspace) = weak_workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.set_database_id(workspace_id);
            });
            // The workspace now has a database_id, so serialization will
            // actually persist its state (panes, docks, session_id, etc.).
            workspace.update(cx, |workspace, cx| {
                workspace.flush_serialization(window, cx).detach();
            });
        }
        // Re-serialize multi-workspace state so active_workspace_id is
        // no longer None.
        this.serialize(cx);
    })
    .log_err();
    anyhow::Ok(())
})
.detach_and_log_err(cx);
```

---

## Fix 4: Delete workspace row on removal instead of just clearing session_id (Moderate)

### Problem

`remove_workspace` calls `set_session_id(workspace_id, None)` but doesn't
delete the row. The `next_id()` call created a row with all-NULL columns (except
`workspace_id` and `timestamp`). The cleanup paths in `recent_workspaces_on_disk`
filter with `WHERE paths IS NOT NULL OR remote_connection_id IS NOT NULL`, so
these orphan rows are never returned and never deleted. They accumulate forever.

Additionally, if `set_session_id` fails or the detached task is dropped on
process exit, the row retains its `session_id` and becomes a zombie that gets
restored on next launch.

### Change

In `MultiWorkspace::remove_workspace` (`crates/workspace/src/multi_workspace.rs`):

1. Replace `set_session_id(workspace_id, None)` with
   `delete_workspace_by_id(workspace_id)`. This solves both the orphan
   accumulation and the zombie problem — a deleted row can't be restored.

2. Change `.detach()` to `.detach_and_log_err(cx)` for consistency and
   visibility.

```rust
if let Some(workspace_id) = removed_workspace.read(cx).database_id() {
    cx.background_spawn(async move {
        crate::persistence::DB
            .delete_workspace_by_id(workspace_id)
            .await
            .log_err();
    })
    .detach_and_log_err(cx);
}
```

Note: `delete_workspace_by_id` is currently `pub async fn` on `WorkspaceDb`, so
it's already accessible. Verify that deleting the row doesn't leave dangling
references in child tables (pane_groups, items, breakpoints). Check for
`ON DELETE CASCADE` or manual cleanup. If there's no cascade, add explicit
deletion of child rows in a transaction, or rely on the fact that orphan child
rows without a parent workspace_id are harmless (they'll never be loaded).

---

## Fix 5: Await cleanup task on quit (Belt-and-suspenders for Fix 4)

### Problem

Even with Fix 4, the `delete_workspace_by_id` call is in a detached background
task. If the user removes a workspace and immediately quits, the task may be
dropped before it executes.

### Change

Store the cleanup task so the quit handler can await it. Two options:

**Option A (simpler):** The quit handler's flush loop (Fix 1) already iterates
all workspaces. Since the removed workspace is no longer in the list, its
deletion won't be covered. But since we're now deleting the row (Fix 4) rather
than just clearing session_id, the worst case if the delete is lost is an orphan
row with `session_id` still set — which would be a zombie.

To handle this: instead of a detached task, store the removal task on
`MultiWorkspace` and await it during flush.

Add a field to `MultiWorkspace`:

```rust
pending_removal_tasks: Vec<Task<()>>,
```

In `remove_workspace`, push the task instead of detaching:

```rust
if let Some(workspace_id) = removed_workspace.read(cx).database_id() {
    self.pending_removal_tasks.push(cx.background_spawn(async move {
        crate::persistence::DB
            .delete_workspace_by_id(workspace_id)
            .await
            .log_err();
    }));
}
```

In the quit handler's flush loop (Fix 1), also drain these:

```rust
for window in &workspace_windows {
    if let Some(tasks) = window
        .update(cx, |multi_workspace, window, cx| {
            let mut tasks: Vec<Task<()>> = multi_workspace
                .workspaces()
                .iter()
                .map(|workspace| {
                    workspace.update(cx, |workspace, cx| {
                        workspace.flush_serialization(window, cx)
                    })
                })
                .collect();
            tasks.append(&mut multi_workspace.pending_removal_tasks);
            tasks
        })
        .log_err()
    {
        flush_tasks.extend(tasks);
    }
}
futures::future::join_all(flush_tasks).await;
```

**Option B (simpler still):** Accept the tiny race window. If the delete is
lost, the row becomes an orphan. On next startup it might be restored as a
zombie, but the user just closes it again. Given the race requires
remove-then-quit-within-milliseconds, this may be acceptable.

**Recommendation:** Go with Option A. The code is straightforward and it
closes the hole completely.

---

## Tests to Add

All tests go in `crates/workspace/src/persistence.rs` in the existing `mod tests`
block, following the pattern of
`test_multi_workspace_serializes_on_add_and_remove`.

### Test 1: `test_flush_serialization_completes_before_quit`

**Purpose:** Verify that `flush_serialization` actually writes to the DB (the
task runs to completion, not just scheduled).

**Approach:**
1. Create a `MultiWorkspace` with a workspace that has a database_id.
2. Mutate some workspace state (e.g., change `centered_layout`).
3. Call `flush_serialization` and await the returned task.
4. Read the workspace back from the DB and verify the mutation is persisted.
5. Contrast: do NOT call `run_until_parked` — the point is that awaiting the
   task alone is sufficient.

### Test 2: `test_create_workspace_serializes_active_workspace_id_after_db_id_assigned`

**Purpose:** Verify that after `create_workspace`, the multi-workspace state
in the DB has the correct `active_workspace_id` (not `None`).

**Approach:**
1. Create a `MultiWorkspace` with one workspace (set a random database_id).
2. Call `create_workspace` on the `MultiWorkspace`.
3. `run_until_parked` to let the async `next_id()` task complete.
4. Read back `read_multi_workspace_state(window_id)`.
5. Assert `active_workspace_id` is `Some(...)` and matches the new workspace's
   `database_id()`.

This test will **fail** on the current branch (before fixes) and **pass** after
Fixes 2+3.

### Test 3: `test_create_workspace_individual_serialization`

**Purpose:** Verify that a newly created workspace's content is serialized to
the DB after its database_id is assigned.

**Approach:**
1. Create a `MultiWorkspace`, call `create_workspace`.
2. `run_until_parked`.
3. Read the new workspace's `database_id`.
4. Call `DB.workspace_for_id(workspace_id)` and assert it succeeds (the row
   has been populated with real data, not just the bare `DEFAULT VALUES` row).
5. Verify `session_id` is set (not None) in the returned serialized workspace.

### Test 4: `test_remove_workspace_deletes_db_row`

**Purpose:** Verify that removing a sidebar workspace deletes its row from the
DB entirely (not just clears session_id).

**Approach:**
1. Create a `MultiWorkspace` with two workspaces (both with database_ids).
2. Record `workspace_id` of the second workspace.
3. Call `remove_workspace(1, ...)`.
4. `run_until_parked`.
5. Call `DB.workspace_for_id(workspace_id)` and assert it returns an error
   (row not found).

### Test 5: `test_remove_workspace_not_restored_as_zombie`

**Purpose:** End-to-end test that a removed workspace doesn't appear in the
session restoration list.

**Approach:**
1. Create a `MultiWorkspace` with two workspaces, both serialized with a
   `session_id` and `window_id`.
2. Remove one workspace.
3. `run_until_parked`.
4. Call `DB.last_session_workspace_locations(session_id, ...)`.
5. Assert the removed workspace's `workspace_id` is NOT in the returned list.

### Test 6: `test_pending_removal_tasks_drained_on_flush`

**Purpose:** Verify that `pending_removal_tasks` are awaited during the flush
path (the quit handler's pattern).

**Approach:**
1. Create a `MultiWorkspace` with two workspaces.
2. Remove one workspace (this pushes a task to `pending_removal_tasks`).
3. Do NOT call `run_until_parked` yet.
4. Simulate the quit handler pattern: call `flush_serialization` on remaining
   workspaces, drain `pending_removal_tasks`, and await all tasks.
5. Verify the removed workspace's DB row is deleted.

---

## Implementation Order

1. **Fix 4** — Change `remove_workspace` to delete instead of clear session_id.
2. **Fix 5** — Add `pending_removal_tasks` field, store tasks, expose for flush.
3. **Fix 1** — Rewrite quit handler flush loop to collect and await tasks
   (including pending removals).
4. **Fixes 2+3** — Rewrite `create_workspace` spawn to re-serialize after
   database_id assignment.
5. **Tests** — Add all six tests.
6. **Verify** — Run `./script/clippy` and the new tests.

## Files Modified

- `crates/workspace/src/multi_workspace.rs` — Fixes 2, 3, 4, 5
- `crates/workspace/src/workspace.rs` — No new changes needed (flush_serialization already exists)
- `crates/zed/src/zed.rs` — Fix 1
- `crates/workspace/src/persistence.rs` — Tests 1–6