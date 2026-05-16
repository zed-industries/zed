# GitStore host sharing context

This note tracks the `GitStore` portion of the project/host split. `GitStore` is now host-shaped state shared by multiple `Project`s, so project-specific git state must move out of `GitStore` and into `Project` or a project-scoped view.

## Core bug: active repository lives on the host store

`GitStore.active_repo_id` is currently a single host-global pointer. In a multi-tenant host, this causes:

- Project B can report Project A's active repository.
- Focusing a file or selecting a repository in one project can change the active repository in sibling projects.
- `GitStoreEvent::ActiveRepositoryChanged` is ownership-blind and cannot be correct for every project.
- `GitStoreEvent::RepositoryUpdated(_, _, is_active)` computes `is_active` against host-global state, so it is wrong whenever projects have different active repositories.
- UI/actions that route through the active repo can operate on a sibling project's repository.

The production fix should make active repository state project-scoped, with project APIs such as:

- `Project::active_repository_id`
- `Project::active_repository(cx)`
- `Project::set_active_repository_id(...)`
- `Project::set_active_repository_for_path(...)`
- `Project::set_active_repository_for_worktree(...)`

`Project::active_repository(cx)` should never index into the host repository map with `[]`; stale ids should return `None` or be cleared.

## Leak definition for this refactor

The important leak class is not only process-wide entity leaks. It is also project-resource leaks inside host-level stores.

Before host sharing, dropping a `Project` dropped its `GitStore`, `BufferStore`, `LspStore`, etc. Now those stores live as long as the shared `Host`, so dropping one `Project` must release resources that were exclusively associated with that project while preserving resources still used by sibling projects.

For `GitStore`, a leak means:

- a `Repository` entity remains in the shared host `GitStore` even though no live `Project` owns it; or
- a `Project` still claims a repository id after that repository is no longer associated with one of the project's worktrees; or
- active-repository state, subscriptions, diffs, jobs, or snapshots keep a project-only repository alive after that project is dropped.

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

## Property-test direction

A property test is likely the most complete way to validate host sharing. Instead of hand-writing many scenario tests, generate a sequence of operations against one host and assert invariants after each operation.

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

Fallback selection should be deterministic. Prefer project worktree order or sorted path order, not `HashMap` iteration.

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
- `GitStoreEvent::RepositoryAdded`
- `GitStoreEvent::RepositoryRemoved`
- `RepositorySnapshotForDownstream`
- `ForwardRepositoryUpdate`
- `ForwardRepositoryRemove`

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
