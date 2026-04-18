# Worktree-switch refactor follow-up hand-off

This document captures the work developed as a follow-up on top of Danilo Leal's original PR:

- Original PR: `#54183`
- Original title: `Move the worktree picker to the title bar + make it always visible`
- Original branch: `worktree-picker-title-bar`

The goal of this follow-up was **not** to change the visible worktree-picker product direction from the original PR. The goal was to finish the worktree-switch refactor so that switching from one workspace/worktree to another exposes the **source workspace directly**, and lets higher layers interpret that directly rather than relying on copied transition state or transient mailboxes.

## Architectural intent implemented in this follow-up

The intended model for worktree switching is now:

- the workspace/worktree switching path knows **which workspace it is leaving**
- generic layers expose only `source_workspace: Option<WeakEntity<Workspace>>`
- feature layers interpret that source workspace directly
- `AgentPanel` initialization inspects the source workspace's `AgentPanel` directly
- there is **no** copied draft mailbox/store and **no** generic transition payload blob

## What was explicitly avoided

The follow-up work intentionally avoided these designs:

- no `WorkspaceTransitionState` / `PanelTransitionState` / `AgentPanelTransitionState`
- no generic payload object passed through workspace APIs
- no `MultiWorkspace.pending_worktree_switch_text`
- no `agent_ui` global draft handoff store
- no side-channel source-stashes / destination-consumes mailbox design
- no agent-specific transfer state in `MultiWorkspace`
- no agent draft transport via `WorktreeCreationChanged`

## Core code changes made

### 1. Canonical activation API

`MultiWorkspace::activate(...)` was changed to the canonical form:

- `activate(workspace, source_workspace, window, cx)`

where:

- `source_workspace: Option<WeakEntity<Workspace>>`

This is the one canonical activation form. Ordinary activation paths now pass `None`. Worktree-switch activation paths pass `Some(source_workspace)`.

Related changes:

- `MultiWorkspaceEvent::ActiveWorkspaceChanged` now carries:
  - `source_workspace: Option<WeakEntity<Workspace>>`
- subscribers that do not care about the source just match `ActiveWorkspaceChanged { .. }`
- zed's agent panel setup path consumes the explicit source from that event

### 2. Generic worktree/workspace switching now threads source workspace explicitly

In `crates/git_ui/src/worktree_service.rs`:

- the source workspace is already available as the workspace we are switching away from
- `open_worktree_workspace(...)` now passes that source workspace into the destination activation path explicitly
- after finding or creating the destination workspace, activation is performed with:
  - `multi_workspace.activate(new_workspace.clone(), Some(source_workspace.clone()), window, cx)`

This preserves the intended primitive:

- "we are activating this destination workspace"
- "here is the workspace we are coming from"

### 3. Agent panel setup path is now source-workspace aware

In `crates/zed/src/zed.rs`:

- `ensure_agent_panel_for_workspace(...)` now takes:
  - `source_workspace: Option<WeakEntity<Workspace>>`
- after ensuring the `AgentPanel` is present, it calls into `AgentPanel` only if a source workspace was explicitly provided
- the `MultiWorkspaceEvent::ActiveWorkspaceChanged { source_workspace }` subscription forwards that explicit source into the panel setup path

This keeps the source interpretation in the panel install/setup flow instead of in runtime draft-transport subscriptions.

### 4. AgentPanel now derives state directly from the source AgentPanel

In `crates/agent_ui/src/agent_panel.rs`:

Added source-aware initialization helpers:

- `destination_has_meaningful_state(...)`
- `active_initial_content(...)`
- `source_panel_initialization(...)`
- `initialize_from_source_workspace_if_needed(...)`

The important behavior is:

- look up the source workspace directly
- find its `AgentPanel` directly
- read the source panel's active draft/editor state directly
- initialize the destination panel only if the destination is effectively fresh/uninitialized

The destination overwrite policy is conservative:

- if destination panel already has meaningful initialized state, do nothing
- if destination is fresh, initialize it from the source panel in one shot

The source-derived data comes from the source `AgentPanel` itself, not from a copied temporary store.

## Transient mechanisms that were removed

These mechanisms were removed from the on-disk diff:

- `PendingWorktreeDraftStore` in `agent_panel.rs`
- `stash_pending_worktree_draft(...)`
- `take_pending_worktree_draft(...)`
- the `AgentPanel` `WorktreeCreationChanged` subscription used only for draft handoff
- the `did_stash_worktree_draft` field
- the temporary `Workspace.pending_source_workspace` approach that was briefly introduced during the refactor attempt
- the separate `activate_with_source_workspace(...)` method, in favor of a single canonical `activate(...)`

There should now be no transient handoff store/mailbox in the codepath described above.

## Files materially changed in this follow-up

These files contain the main architectural changes:

- `crates/git_ui/src/worktree_service.rs`
- `crates/workspace/src/multi_workspace.rs`
- `crates/workspace/src/workspace.rs`
- `crates/agent_ui/src/agent_panel.rs`
- `crates/zed/src/zed.rs`

Other touched files are mostly call-site updates caused by the canonical `activate(...)` signature and related event shape changes:

- `crates/agent_ui/src/conversation_view.rs`
- `crates/call/src/call_impl/mod.rs`
- `crates/recent_projects/src/remote_connections.rs`
- `crates/recent_projects/src/remote_servers.rs`
- `crates/sidebar/src/sidebar.rs`
- `crates/workspace/src/persistence.rs`
- `crates/zed/src/visual_test_runner.rs`
- plus some already-existing branch work in `git_ui` files

## Important current status / remaining work

The architectural refactor is in place, but this follow-up is **not fully mechanically finished**.

### Remaining known work

There are still remaining old 3-argument `activate(...)` test call sites that need to be updated to the canonical 4-argument form:

- `mw.activate(workspace, None, window, cx)`

The most obvious remaining cluster is in:

- `crates/sidebar/src/sidebar_tests.rs`

At the time of hand-off, that file had unsaved editor changes in the environment, so I intentionally did **not** force-save or overwrite it. That means the branch may still contain compile fallout from those unchanged old call sites.

There may also be a few other remaining mechanical call-site updates in tests/helpers that should be cleaned up with a repo-wide search for:

- `mw.activate(`
- `multi_workspace.activate(`

and then converting remaining old-form calls to:

- `activate(workspace, None, window, cx)`

unless the call is a true worktree-switch case that should pass `Some(source_workspace)`.

### Diagnostics status

I ran targeted `cargo check` attempts during the refactor and used those diagnostics to fix the main architectural call sites.

The last known remaining failures were mechanical old-signature call sites after the canonical `activate(...)` change, not new architectural design issues.

I stopped short of further blind edits in files with unsaved buffer state.

## Suggested verification steps for the next person

1. Search for all remaining `activate(...)` call sites and ensure they use the canonical signature.
2. Run a targeted `cargo check` again after finishing those mechanical updates.
3. Verify these behaviors manually or with tests:
   - switching from workspace A to workspace B restores workspace-level dock/file/focus state from A
   - destination `AgentPanel` initializes from source `AgentPanel` when destination is fresh
   - destination `AgentPanel` does **not** overwrite meaningful existing destination state
   - no draft text is transported through any copied mailbox/store
4. Consider adding or updating tests for:
   - source-aware activation event payload
   - conservative destination initialization behavior
   - reused destination workspace cases

## Summary of the intended final model

The target model for this follow-up is:

- workspace switching knows what workspace it is leaving
- generic layers only expose that source workspace
- feature layers interpret it themselves
- `AgentPanel` initialization directly inspects the source `AgentPanel`
- no copied draft mailbox/store exists anywhere

If additional cleanup is needed, it should stay within that architecture and avoid reintroducing any transient transfer state.
