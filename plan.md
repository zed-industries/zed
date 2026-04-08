# Plan: Fix sidebar flicker when remote workspace is added

## Context

Read `summary.md` for all changes made so far. This plan covers the remaining flicker bug.

## The Bug

When a remote workspace is added to the sidebar, the project group briefly flickers (appears as a separate group for 1-2 frames). This happens because:

1. **Server-side `set_snapshot`** in `zed/crates/worktree/src/worktree.rs` (~line 1205) unconditionally recomputes `root_repo_common_dir` from `git_repositories`:

   ```rust
   new_snapshot.root_repo_common_dir = new_snapshot
       .local_repo_for_work_directory_path(RelPath::empty())
       .map(|repo| SanitizedPath::from_arc(repo.common_dir_abs_path.clone()));
   ```

   During early scan passes, `.git` hasn't been discovered yet, so this overwrites the correct value (set by `Worktree::local()` during creation) with `None`.

2. The server sends an `UpdateWorktree` message with `root_repo_common_dir = None`.

3. The client's `apply_remote_update` in `zed/crates/worktree/src/worktree.rs` (~line 2437) currently has a partial fix that only updates when `Some`:
   ```rust
   if let Some(dir) = update.root_repo_common_dir.map(...) {
       self.root_repo_common_dir = Some(dir);
   }
   ```
   This prevents the client from clearing it, but the real fix should be server-side.

## What To Do

### Step 1: Add flicker detection to the existing test

Extend `test_clicking_closed_remote_thread_opens_remote_workspace` in `zed/crates/sidebar/src/sidebar_tests.rs` to catch transient flicker. Use the `observe_self` pattern from `test_clicking_worktree_thread_does_not_briefly_render_as_separate_project` (line ~3326-3397), which installs an observer that fires on **every notification** and panics if more than one project header ever appears:

```rust
sidebar
    .update(cx, |_, cx| cx.observe_self(assert_sidebar_state))
    .detach();
```

Add this observer BEFORE the stale key injection / workspace addition steps. The callback should assert that there is never more than one project group header at any point during the test. This catches the case where an `UpdateWorktree` message with `root_repo_common_dir = None` temporarily creates a wrong project group key.

Since the full remote mock connection is hard to set up for a second connection, an alternative approach: simulate the `UpdateWorktree` message arriving with `root_repo_common_dir = None` by directly calling the worktree's update mechanism on the existing project. Or, test at a lower level by verifying that `set_snapshot` doesn't clear `root_repo_common_dir`.

### Step 2: Fix the server-side root cause

In `zed/crates/worktree/src/worktree.rs`, find `set_snapshot` (~line 1200-1210). Change the `root_repo_common_dir` recomputation to not downgrade once set:

```rust
// Before (overwrites unconditionally):
new_snapshot.root_repo_common_dir = new_snapshot
    .local_repo_for_work_directory_path(RelPath::empty())
    .map(|repo| SanitizedPath::from_arc(repo.common_dir_abs_path.clone()));

// After (preserve existing value if scan hasn't discovered repo yet):
new_snapshot.root_repo_common_dir = new_snapshot
    .local_repo_for_work_directory_path(RelPath::empty())
    .map(|repo| SanitizedPath::from_arc(repo.common_dir_abs_path.clone()))
    .or(self.snapshot.root_repo_common_dir.clone());
```

This ensures the value discovered by `Worktree::local()` during creation is preserved until the scanner finds the repo and confirms/updates it.

### Step 3: Verify the client-side guard is still useful

The `apply_remote_update` change (only update when `Some`) is a defense-in-depth measure. With the server fix, the server should never send `None` after having the correct value. But keeping the client guard is good practice. Verify the test passes with both fixes.

### Step 4: Update `summary.md`

Add the flicker fix to the summary of changes.

## Important Notes

- Use sub-agents for research tasks to keep context manageable
- The key test pattern is `cx.observe_self(callback)` which fires on every `cx.notify()` — this catches transient states that `run_until_parked` would miss
- Read `test_clicking_worktree_thread_does_not_briefly_render_as_separate_project` (~line 3262-3397) for the full example of this testing pattern
- After all changes, run `cargo check` on all affected packages and run the sidebar + agent_ui tests
