# Remote Worktree Support — Summary of Changes

## Problem
The agent panel's "create new thread in worktree" feature only supported local projects. Remote (SSH/WSL/Docker) projects need the same capability, plus correct sidebar integration.

## Changes Made

### 1. `HeadlessRemoteClientDelegate` (`remote_connection/src/remote_connection.rs`)
New public struct implementing `RemoteClientDelegate` without UI. Forwards binary downloads to `AutoUpdater`, drops password prompts with a log warning.

### 2. Remote worktree workspace creation (`agent_ui/src/agent_panel.rs`)
- `handle_worktree_requested`: extracts `remote_connection_options` from project, fails early if disconnected
- `open_worktree_workspace_and_start_thread`: new remote branch using `remote::connect()` → `RemoteClient::new()` → `Project::remote()` → `open_remote_project_with_existing_connection()` + `multi_workspace.add()`

### 3. Sidebar remote thread support (`sidebar/src/sidebar.rs`)
- `ThreadEntryWorkspace::Closed` now carries `host: Option<RemoteConnectionOptions>`
- `open_workspace_and_activate_thread`: branches on `host` — remote uses headless delegate flow, local unchanged
- All pattern match sites updated, `activate_archived_thread` looks up host from project group keys
- Worktree tooltip says "Remote" vs "Local" (`ui/src/components/ai/thread_item.rs`)

### 4. Proto: `root_repo_common_dir` in `WorktreeMetadata` + `AddWorktreeResponse`
- `proto/worktree.proto`: added `optional string root_repo_common_dir` to both messages
- `remote_server/headless_project.rs`: includes value in `AddWorktreeResponse`
- `worktree/worktree.rs`: `Worktree::remote()` sets it from metadata; `metadata_proto()` includes it; `apply_remote_update` only updates when `Some` (never clears)
- `project/worktree_store.rs`: passes through in `create_remote_worktree`, `worktree_metadata_protos`; emits new `WorktreeUpdatedRootRepoCommonDir` event
- `project/project.rs`: new `Event::WorktreeUpdatedRootRepoCommonDir`, forwarded from worktree store

### 5. Stale key cleanup (`workspace/src/multi_workspace.rs`)
- `subscribe_to_workspace`: handles `WorktreeUpdatedRootRepoCommonDir` — adds correct key, removes stale keys, notifies
- New `remove_stale_project_group_keys()` method

### 6. Dependency changes
- `agent_ui/Cargo.toml`: added `remote`, `remote_connection` to deps; added remote test infra to dev-deps
- `sidebar/Cargo.toml`: added `remote_connection`, `futures` to deps; added remote test infra to dev-deps

### 7. Tests
- `agent_ui`: `test_worktree_creation_for_remote_project` — verifies remote code path is taken
- `sidebar`: `test_clicking_closed_remote_thread_opens_remote_workspace` — verifies grouping and stale key cleanup

## What's Left
See `plan.md`.
