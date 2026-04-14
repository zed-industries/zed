# Plan

## Scope

This PR is limited to sidebar/archive thread matching and reopen correctness for remote host identity.

It includes a shared normalized remote identity helper reused by sidebar/archive and workspace persistence matching paths. It does not change the thread metadata DB schema.

Session-list history transport through `AgentSessionInfo.meta` is a separate follow-up.

## Already on main

These were completed in the previous PR and are now merged:

1. `ThreadMetadata` carries persisted `remote_connection: Option<RemoteConnectionOptions>` in `sidebar_threads` (JSON text column).
2. Live thread metadata writes persist `project.remote_connection_options(cx)` via `handle_conversation_event`.
3. ACP import threads carry the originating store's `remote_connection`.
4. One-time startup migration backfills `remote_connection` from workspace history (KVP-guarded).
5. `change_worktree_paths` and `change_worktree_paths_by_main` already accept `remote_connection` and filter by it.

## Current PR

### Done

1. Added normalized remote identity helper in `crates/remote/src/remote_identity.rs`.
   - `RemoteConnectionIdentity` enum: SSH (host + username + port), WSL (distro + user), Docker (container_id + name + remote_user).
   - `remote_connection_identity(&RemoteConnectionOptions) -> RemoteConnectionIdentity`
   - `same_remote_connection_identity(Option<&RCO>, Option<&RCO>) -> bool`
   - Tests covering SSH/WSL/Docker field selection and None-vs-Some matching.
   - Re-exported from `crates/remote/src/remote.rs` and `crates/workspace/src/workspace.rs`.
2. Refactored workspace persistence `get_or_create_remote_connection_query` to normalize via `RemoteConnectionIdentity` instead of raw `RemoteConnectionOptions` field access.
3. Made `ThreadMetadataStore` lookups host-aware in `crates/agent_ui/src/thread_metadata_store.rs`.
   - `entries_for_path(...)` and `entries_for_main_worktree_path(...)` now accept `Option<&RemoteConnectionOptions>` and post-filter via `same_remote_connection_identity`.
   - All callers in `sidebar.rs` pass `group_key.host()` or `metadata.remote_connection`; test callers pass `None` for local threads.

4. Filtered sidebar threads by matching remote connection in `crates/sidebar/src/sidebar.rs`.
   - All `entries_for_main_worktree_path` and `entries_for_path` calls in `rebuild_contents` now pass `group_host.as_ref()`.
   - `archive_thread` passes the thread's own `remote_connection` when counting remaining threads.

5. Made sidebar workspace lookup / activation flows host-aware in `crates/sidebar/src/sidebar.rs`.
   - `find_current_workspace_for_path_list` and `find_open_workspace_for_path_list` now compare `remote_connection_options` via `same_remote_connection_identity`.
   - All three `ProjectGroupKey::new(None, ...)` in `activate_archived_thread` replaced with `ProjectGroupKey::new(metadata.remote_connection.clone(), ...)`.
6. Made archive/worktree-reference logic host-aware.
   - Moved `path_is_referenced_by_other_unarchived_threads` into `ThreadMetadataStore` with a `remote_connection` parameter; call site passes `metadata.remote_connection.as_ref()`.
   - `archive_thread` workspace removal fallback now uses `find_or_create_workspace` with `project_group_key.host()`.
   - Neighbor selection remains host-agnostic by design (picks nearest visible thread for UX).

### Separate follow-ups

7. Session-list history: transport remote identity through `AgentSessionInfo.meta` or a typed field.
8. Reuse normalized remote identity in recent-project and workspace matching code (separate PRs).

## Normalized remote identity notes

- Do not use `RemoteConnectionOptions::display_name()` as identity.
- Do not rely on raw `RemoteConnectionOptions ==` — use `same_remote_connection_identity(...)` from `crates/remote`.
- The normalized helper follows workspace persistence identity semantics:
  - SSH: host + username + port
  - WSL: distro + user
  - Docker: container id + name + remote user
- `crates/project/src/trusted_worktrees.rs` has `RemoteHostLocation`, but it drops fields workspace persistence treats as identity — not suitable here.

## Adjacent follow-ups

These are not required for this PR but should reuse the normalized remote identity helper:

1. Recent-project filtering in `crates/recent_projects/src/recent_projects.rs`
2. Sidebar recent-project filtering in `crates/recent_projects/src/sidebar_recent_projects.rs`
3. Worktree/workspace history dedupe in `crates/workspace/src/persistence.rs` (`resolve_worktree_workspaces`)
4. Workspace/window host matching in `crates/workspace/src/workspace.rs` (`workspace_windows_for_location`)
5. Open-workspace lookup in `crates/workspace/src/multi_workspace.rs` (`workspace_for_paths`)

## Notes

- New migrations always have to be added to the end of the migration list or else they will fail.
- Keep storing full `RemoteConnectionOptions` in thread metadata; use normalized identity for matching only.

## Done so far

- [x] Added persisted `remote_connection` storage to `sidebar_threads` (on main)
- [x] Wired DB save/load for `ThreadMetadata.remote_connection` (on main)
- [x] Persisted `remote_connection_options(cx)` in live thread metadata updates (on main)
- [x] Threaded per-store `remote_connection` through ACP thread import (on main)
- [x] Added one-time backfill for old native thread metadata rows (on main)
- [x] Added normalized remote identity helper (`RemoteConnectionIdentity`)
- [x] Refactored workspace persistence to use normalized identity
- [x] Make `ThreadMetadataStore` lookups host-aware
- [x] Filter sidebar threads by matching remote connection in `rebuild_contents`
- [x] Use remote host in sidebar workspace lookup / activation flows
- [x] Make archive/worktree-reference matching host-aware
