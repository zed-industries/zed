# GitStore host sharing context

This note tracks the `GitStore` portion of the project/host split. `GitStore` is now host-shaped state shared by multiple `Project`s, so project-specific git state must move out of `GitStore` and into `Project` or a project-scoped view.

## Core bug: active repository lived on the host store

`GitStore.active_repo_id` used to be a single host-global pointer. In a multi-tenant host, this caused:

- Project B could report Project A's active repository.
- Focusing a file or selecting a repository in one project could change the active repository in sibling projects.
- `GitStoreEvent::ActiveRepositoryChanged` was ownership-blind and could not be correct for every project.
- `GitStoreEvent::RepositoryUpdated(_, _, is_active)` computed `is_active` against host-global state, so it was wrong whenever projects had different active repositories.
- UI/actions that routed through the active repo could operate on a sibling project's repository.

Current branch status:

- `GitStore.active_repo_id` has been removed.
- `GitStoreEvent::ActiveRepositoryChanged` has been removed.
- `GitStoreEvent::RepositoryUpdated` now carries only `(RepositoryId, RepositoryEvent)`; `Project` computes active-ness and re-emits `GitEvent::RepositoryUpdated { repo_id, event, is_active }`.
- Active repository state now lives on `Project` as `active_repository_id: Option<RepositoryId>`.
- Project-scoped APIs now include:
  - `Project::active_repository_id()`
  - `Project::active_repository(cx)`
  - `Project::set_active_repository_id(...)`
  - `Project::set_active_repository(...)`
  - `Project::set_active_repository_for_path(...)`
  - `Project::set_active_repository_for_worktree(...)`
- `Project::active_repository(cx)` filters through the project's owned repository ids and returns `None` for stale or unowned ids instead of indexing into the host repository map.
- `Project::on_git_store_event` assigns the first claimed repository as active and, when the active repository is removed, falls back to another repository owned by the same project.

Remaining active-repo follow-up:

- Add a direct regression/property case for active-repo fallback: when a project has two owned repositories and the active one is removed, fallback must choose another repository owned by that same project; if none remain, active repo becomes `None`.

## Leak definition for this refactor

The important leak class is not only process-wide entity leaks. It is also project-resource leaks inside host-level stores.

Before host sharing, dropping a `Project` dropped its `GitStore`, `BufferStore`, `LspStore`, etc. Now those stores live as long as the shared `Host`, so dropping one `Project` must release resources that were exclusively associated with that project while preserving resources still used by sibling projects.

For `GitStore`, a leak means:

- a `Repository` entity remains in the shared host `GitStore` even though no live `Project` owns it; or
- a `Project` still claims a repository id after that repository is no longer associated with one of the project's worktrees; or
- active-repository state, subscriptions, diffs, jobs, or snapshots keep a project-only repository alive after that project is dropped.

### Ownership inversion: `GitStore` keeps `WeakEntity`

The current branch enforces the leak rule structurally rather than by
bookkeeping:

- `GitStore.repositories` is `HashMap<RepositoryId, WeakEntity<Repository>>`.
- `Project.repositories` is `HashMap<RepositoryId, Entity<Repository>>` (strong).
- `GitStoreEvent::RepositoryAdded(id, Entity<Repository>)` carries the
  strong handle so the owning project(s) can claim it during dispatch.
- `GitStore` registers a `cx.observe_release` callback per repository it
  creates; when the last project drops its strong handle, the callback
  prunes `repositories` / `worktree_ids` and emits `RepositoryRemoved` +
  `RepositorySnapshotRemovedForDownstream`.
- The two pre-existing pruning paths (`WorktreeRemoved` when
  `worktree_ids` empties, and the per-update `removed_ids` loop in
  `update_repositories_from_worktree`) now also `cx.emit
RepositoryRemoved` so projects drop their strong handles and the
  `observe_release` callback becomes an idempotent no-op.

This is what guarantees that a dropped tenant cannot leak repositories
into a shared host `GitStore`, and is what the property test in
`crates/project/tests/integration/multi_tenant.rs` validates.

Future improvement: `GitStore::repositories()` currently upgrades weak
entries into a newly allocated `HashMap<RepositoryId, Entity<Repository>>`
for compatibility with existing call sites. Prefer narrower host APIs
(`repository(id)`, counts, or iterator-style accessors) so callers do not
need to materialize or accidentally cache host-level strong handles.

### Effect-flush quirk in tests

`cx.run_until_parked()` drains the dispatcher but does **not** trigger
the effect flush that calls `release_dropped_entities`. Tests that drop
a `Project` and then expect the host stores to have cleaned up must
round-trip through `cx.update(|_| {})` (which calls `flush_effects`)
before (or instead of) `cx.run_until_parked()`. The
`GitStorePropertyWorld::drop_project` helper does this; ad-hoc tests
that drop a `Project` need to follow the same pattern.

For other host stores, analogous leaks are project-only buffers, LSP servers, subscriptions, diagnostics, tasks, images, debug sessions, etc. surviving after their owning project drops.

## GPUI leak detector support

GPUI already has entity leak detection:

- `gpui/test-support` enables `gpui/leak-detection`.
- `#[gpui::test]` holds a ref-count guard and tears down the app, then `LeakDetector` panics on leaked handles.
- `LEAK_BACKTRACE=1` prints allocation backtraces for leaked handles.

This catches strong-reference cycles and unreleased entity handles, but it does not by itself prove that shared host stores released project-exclusive resources while the host remains alive.

Useful test-support additions:

- Add a scoped leak/resource checker for multi-tenant fixtures.
- Create multiple projects sharing one host.
- Drop or release one project while another project keeps the host alive.
- Assert host stores no longer contain resources owned only by the dropped project.
- Then drop the remaining projects and rely on GPUI leak detection for global entity leaks.

The `test_project_drop_releases_repository` test in
`crates/project/tests/integration/multi_tenant.rs` is the targeted
regression for the `GitStore` slice of this checklist; the property
test covers it under randomized workloads.

## Property-test direction

A property test is likely the most complete way to validate host sharing. Instead of hand-writing many scenario tests, generate a sequence of operations against one host and assert invariants after each operation.

Current implementation direction: the property test should read like a small state machine, not a mini framework. It should generate only valid operations from the current state, apply them through real `FakeFs` / `Project` / `GitStore` / `WorktreeStore` flows, and validate invariants at operation boundaries after the host store reaches a quiescent state.

### Model state

The test model should track:

- live projects
- live visible worktrees per project
- expected repository ids owned by each project
- expected active repository id per project
- expected host repository ids
- dropped projects
- possibly remote projects/clients in a later phase

### Operations

Candidate generated operations:

- create project on an existing host
- add/open visible worktree
- add/open same visible worktree in another project
- add/open disjoint visible worktree
- add nested repository or submodule-like repository
- focus path in a project
- set active repository by worktree
- set active repository by repository id
- remove worktree from a project
- drop project
- create/delete/rename repository under a worktree
- trigger repository events (`StatusesChanged`, `HeadChanged`, `BranchListChanged`)
- trigger repository added/removed events
- share/unshare project and collect outbound repository updates

Remote-client variants should be considered after local-host properties are stable.

Current first slice generates:

- open project with one initial visible worktree;
- add another visible worktree to a live project;
- drop a live project.

The visible worktree sharing invariant was moved out of the GitStore property test and into a planned WorktreeStore property test (`WORKTREE_STORE_PROPERTY_TEST.md`).

### Invariants after every operation

Per project:

- `Project::repositories(cx)` equals the model's expected project-owned repository ids.
- `Project::active_repository_id()` equals the model's expected active repository id.
- If active repo is `Some`, it is owned by the project.
- If active repo is `Some`, it exists in the host `GitStore`.
- Active repo selection is not affected by sibling project focus/selection operations.
- Project-owned repositories are associated with at least one of that project's worktrees.
- Repository path containment is sane for at least one owned worktree.
- Sibling repository events do not alter the project's repository set or active repo.

For host `GitStore`:

- Every host repository is owned by at least one live project, unless it is an explicitly allowed host-only/invisible resource.
- No repository associated only with a dropped project remains.
- A repository opened by multiple projects remains while at least one owning project is alive.
- No active-repository field exists on `GitStore` after the refactor.

For events:

- Active repository changes are emitted only by the affected `Project`.
- Repository updates from sibling-owned repos are ignored by project-scoped consumers.
- Downstream repository broadcasts include only repos owned by the sharing project.

Current property-test subscription shape:

- `GitStorePropertyWorld` is a GPUI entity that owns the generated projects.
- Each generated project installs:
  - `cx.observe(project, ...)` to verify invariants after project notifications;
  - `cx.subscribe(&git_store, ...)` to verify invariants after host `GitStore` events.
- Subscriptions are detached and rely on GPUI cleanup.

## Repository ownership invariant

A project owns a repository if the repository is associated with at least one worktree owned by the project.

A reusable assertion should check:

1. every repo id in `Project::repositories(cx)` exists in `GitStore::repositories()`;
2. every project-owned repo has host worktree associations;
3. at least one associated worktree id belongs to the project;
4. the repository path is equal to, an ancestor of, or contained by an owned worktree path, depending on normal repo / project-opened-subdir / nested-repo cases.

Remote/collab repos currently may not have `worktree_ids_for_repository`; this should be treated as a known gap unless remote multi-tenant support is added.

## Active repository invariants

Assert more than path equality:

- active repo id is project-owned;
- active repo id exists in the host store;
- active repo path matches the expected project worktree/repo;
- active repo does not change when sibling projects focus paths or select repos;
- removing an active repo falls back only to another repo owned by the same project;
- if no owned repos remain, active repo becomes `None`;
- stale active ids never panic.

Fallback selection is now deterministic in the current branch: `Project::next_active_repository_id` sorts project-owned repositories by `work_directory_abs_path` instead of relying on `HashMap` iteration.

This still needs a direct targeted test/property operation that removes the active repository while another project-owned repository remains.

## External repository events

Projects should ignore repository events that do not pertain to them.

A useful support pattern:

1. snapshot a project's git state (`repository_ids`, `active_repository_id`, active path);
2. trigger a repository event for a sibling-owned repo;
3. assert the snapshot is unchanged;
4. optionally assert no project-scoped active-repo event was emitted.

Events to cover in property tests or helpers:

- `RepositoryEvent::StatusesChanged`
- `RepositoryEvent::HeadChanged`
- `RepositoryEvent::BranchListChanged`
- `RepositoryEvent::StashEntriesChanged`
- `RepositoryEvent::GitWorktreeListChanged`
- `RepositoryEvent::PendingOpsChanged`
- `RepositoryEvent::GraphEvent`
- `GitStoreEvent::RepositoryAdded`
- `GitStoreEvent::RepositoryRemoved`
- `RepositorySnapshotForDownstream`
- `RepositorySnapshotRemovedForDownstream`
- `ForwardRepositoryUpdate`
- `ForwardRepositoryRemove`

`GitStoreEvent::RepositoryUpdated(repo_id, event)` is host-scoped. Project-scoped consumers should subscribe to `Project::GitEvent::RepositoryUpdated { repo_id, event, is_active }`, which is ownership-filtered by `Project::on_git_store_event` and computes `is_active` against that project's `active_repository_id`.

## Nested repos and same-repo sharing

Nested repositories/submodules should be included in property generation:

- outer repo `/repos/alpha`
- nested repo `/repos/alpha/vendor/lib`
- focusing a path under the nested repo should select the deepest matching owned repo;
- focusing a path under only the outer repo should select the outer repo;
- sibling projects should not see nested/outer repos unless they own the relevant worktree.

Same-repo sharing should be supported:

- if two projects open the same repo path, both may own the same repository id;
- active repo may be the same id in both projects;
- dropping one project must not remove the repository while another project still owns it;
- active-repo events should remain project-scoped even when the repo id is shared.

## Notes to include in implementation plan

- `Project::shared` / downstream repository broadcasts should use project-owned repos, not all host repos.
- `Project` wrappers should be preferred for project-scoped status/path lookup APIs (`status_for_buffer_id`, `project_path_git_status`, repository/path lookup).
- `GitStore::original_repo_path_for_worktree` currently prioritizes host active repo; this should become project-scoped or accept an explicit preferred repo id.
- Async stale-result races in UI paths may be pre-existing. Note them, but do not expand this refactor to fix all such bugs unless they are direct regressions from host-store reuse.
