# Remote support for archived thread worktrees

## Goal

Add remote support to the sidebar thread archive flow so that:

- archiving the last thread associated with a git-linked worktree removes that linked worktree from disk,
- the worktree's staged and unstaged state is snapshotted before removal, and
- unarchiving restores the git worktree and reapplies its saved state.

## Research summary

I reviewed the current archive/unarchive flow and the relevant git/project remote plumbing.

### Main archive/unarchive flow

The feature is split across these files:

- `crates/sidebar/src/sidebar.rs`
  - `archive_thread`
  - `archive_and_activate`
  - `start_archive_worktree_task`
  - `archive_worktree_roots`
  - `activate_archived_thread`
- `crates/agent_ui/src/thread_worktree_archive.rs`
  - `build_root_plan`
  - `persist_worktree_state`
  - `remove_root`
  - `rollback_persist`
  - `restore_worktree_via_git`
  - `cleanup_archived_worktree_record`
  - `find_or_create_repository`
- `crates/agent_ui/src/thread_metadata_store.rs`
  - archived thread metadata and archived worktree DB records
- `crates/project/src/git_store.rs`
  - repository operations used to snapshot, remove, recreate, and restore worktrees

### What already works remotely in `git_store.rs`

These operations already have remote support and appear sufficient for part of the archive/restore flow:

- `Repository::head_sha`
- `Repository::create_worktree`
- `Repository::create_worktree_detached`
- `Repository::remove_worktree`
- `Repository::change_branch`
- `Repository::create_branch`
- generic checkpoint operations:
  - `Repository::checkpoint`
  - `Repository::restore_checkpoint`
  - `Repository::compare_checkpoints`
  - `Repository::diff_checkpoints`

Remote repository metadata propagation also already exists:

- `RepositorySnapshot::initial_update`
- `RepositorySnapshot::build_update`
- `GitStore::handle_update_repository`

That includes linked worktree metadata such as `original_repo_abs_path` and `linked_worktrees`.

### Confirmed missing git operations in `git_store.rs`

These are the git operations used by the archive-thread feature that still explicitly bail for remote repositories:

- `Repository::update_ref`
- `Repository::delete_ref`
- `Repository::repair_worktrees`
- `Repository::create_archive_checkpoint`
- `Repository::restore_archive_checkpoint`

These five are the direct `git_store.rs` gaps for the feature.

### Why those five matter

`thread_worktree_archive.rs` depends on them like this:

- `persist_worktree_state`
  - `head_sha`
  - `create_archive_checkpoint`
  - `update_ref`
- `rollback_persist`
  - `delete_ref`
- `restore_worktree_via_git`
  - `repair_worktrees`
  - `create_worktree_detached`
  - `change_branch`
  - `create_branch`
  - `restore_archive_checkpoint`
- `cleanup_archived_worktree_record`
  - `delete_ref`

So remote archive/restore cannot work end to end until those operations are handled remotely.

## Important conclusion

My original guess that only the remote git operation implementation was missing is close, but not fully complete.

### What is true

For the core git behavior, the missing work is mostly the remote implementation of the five operations above.

### What else is still missing

There are at least two non-git gaps:

1. `crates/agent_ui/src/thread_worktree_archive.rs`
   - `find_or_create_repository` falls back to `Project::local(...)`.
   - That is local-only behavior and is not a valid fallback for remote-only repositories.

2. `crates/sidebar/src/sidebar.rs`
   - `activate_archived_thread` currently reopens archived threads with `ProjectGroupKey::new(None, path_list.clone())`.
   - That drops `ThreadMetadata.remote_connection` and appears wrong for archived remote threads.

## Remote transport / RPC work

### SSH remote projects

For SSH remote projects, the architecture is already close.

`GitStore::init` already handles several remote git requests, and `crates/proto/proto/git.proto` already defines messages for adjacent operations like:

- `GitGetHeadSha`
- `GitCreateWorktree`
- `GitRemoveWorktree`
- `GitCreateCheckpoint`
- `GitRestoreCheckpoint`

But there are no existing proto messages for the archive-specific operations we need:

- update ref
- delete ref
- repair worktrees
- create archive checkpoint
- restore archive checkpoint

That means SSH remote support still needs:

- new proto messages,
- new `GitStore` request handlers, and
- new remote branches in `Repository` for the five missing operations.

### Collab / shared projects

Collab is less complete than SSH remote.

In `crates/collab/src/rpc.rs`:

- `GitGetWorktrees`, `GitGetHeadSha`, and `GitCreateWorktree` are forwarded.
- `GitRemoveWorktree` and `GitRenameWorktree` are explicitly blocked for guests.
- checkpoint RPCs are not forwarded there.

That means if we want this feature to work for collab guests as well, we likely need additional collab policy and forwarding work beyond the SSH remote changes.

## Proposed scope decision

Before implementation, decide whether this task should target:

1. SSH remote projects only, or
2. SSH remote projects plus collab/shared projects.

Recommended first scope:

- implement SSH remote support first,
- then decide separately whether collab guest support should be enabled or intentionally blocked.

That is the smallest path to making remote archive/unarchive work without taking on host/guest authorization design at the same time.

## Implementation plan

### Phase 1: make the archive flow remote-aware at the UI/helper layer

1. Fix archived-thread reopen to preserve remote host info.
   - File: `crates/sidebar/src/sidebar.rs`
   - Update `activate_archived_thread` to use `metadata.remote_connection` when reconstructing the `ProjectGroupKey` for restored archived threads.
   - Audit both reopen paths in that function.

2. Replace the local-only repository fallback in `find_or_create_repository`.
   - File: `crates/agent_ui/src/thread_worktree_archive.rs`
   - Design a remote-aware way to obtain a `Repository` handle when the repo is not already loaded in an open workspace.
   - Avoid falling back to `Project::local(...)` for remote repositories.
   - Prefer reusing an existing remote `Project` or introducing a helper that can resolve repositories through the current remote project/session.

### Phase 2: add missing remote git operations in `git_store.rs`

Add remote support for these methods in `crates/project/src/git_store.rs`:

- `Repository::update_ref`
- `Repository::delete_ref`
- `Repository::repair_worktrees`
- `Repository::create_archive_checkpoint`
- `Repository::restore_archive_checkpoint`

Implementation shape should mirror existing remote methods like `head_sha`, `create_worktree`, and `remove_worktree`.

### Phase 3: add proto definitions for the missing operations

File: `crates/proto/proto/git.proto`

Add request/response messages for:

- updating a ref,
- deleting a ref,
- repairing worktrees,
- creating an archive checkpoint,
- restoring an archive checkpoint.

Notes:

- `create_archive_checkpoint` needs a response containing both the staged and unstaged commit SHAs.
- The existing generic checkpoint RPCs are not a direct drop-in replacement because archive checkpoint creation returns two detached commits and archive restore consumes two SHAs.

### Phase 4: add request handlers in `GitStore`

File: `crates/project/src/git_store.rs`

1. Register the new request handlers in `GitStore::init`.
2. Add handler functions analogous to the existing ones for worktree and checkpoint operations.
3. Route each request to the local backend implementation.

### Phase 5: decide and implement collab behavior

File: `crates/collab/src/rpc.rs`

Decide whether collab guests should support this feature.

If yes:

- forward the new archive-related git requests,
- revisit `GitRemoveWorktree` guest restrictions,
- decide whether restore/removal should run as a host-mediated operation rather than a guest-local plan.

If no:

- explicitly gate archive-worktree behavior for collab so failure is intentional and user-facing rather than a runtime error deep in the task.

## What I think is left to do

### Definitely required

- remote implementations for the five missing `git_store.rs` operations,
- proto + request-handler plumbing for those operations,
- remote-aware repository lookup in `find_or_create_repository`,
- preserving `remote_connection` when reopening archived threads.

### Probably required if collab is in scope

- collab RPC forwarding for the new requests,
- a policy decision around guest permission to remove/repair worktrees,
- possibly host-side orchestration for archive/remove/restore rather than guest-side planning.

## Suggested order of work

1. Fix `activate_archived_thread` to preserve `remote_connection`.
2. Design the replacement for `find_or_create_repository`.
3. Add proto messages and `GitStore` handlers.
4. Add remote `Repository` branches for the five missing methods.
5. Run the full archive/unarchive flow against an SSH remote project.
6. Decide collab scope and either implement it or explicitly gate it.

## Validation checklist

### Local regression

- Archiving the last thread for a linked worktree still snapshots state and removes the worktree from disk.
- Unarchiving still restores the worktree, branch, index, and working tree state.
- Cleanup still removes the archive ref when the archived worktree record is no longer referenced.

### SSH remote

- Archiving a thread for a remote linked worktree snapshots state remotely.
- The linked worktree is removed remotely.
- Unarchiving recreates the worktree remotely at the original commit.
- Branch restore works.
- Staged and unstaged state is restored.
- Reopened thread lands back in the correct remote workspace.

### Collab, if in scope

- Archive/unarchive works for the intended actor model.
- Permission errors are surfaced clearly when an actor is not allowed to remove or repair worktrees.

## Open questions

1. Should archived-thread worktree restore support SSH remote only, or collab too?
2. Is there already a preferred helper for creating or resolving remote `Project` instances that `find_or_create_repository` should use instead of inventing a new path?
3. Should collab guests be allowed to remove linked worktrees at all, or should only the host be allowed to execute that part of the flow?
4. Do we want to reuse the existing generic checkpoint RPCs conceptually, or keep archive checkpointing as a separate RPC because it has different semantics?

## Short answer to the original question

No, it is not only the remote git operation implementation.

The missing remote git operations in `git_store.rs` are the biggest gap, but there are also at least two additional issues to solve:

- archived-thread reopen currently drops `remote_connection`, and
- `find_or_create_repository` has a local-only fallback that is not suitable for remote repositories.
