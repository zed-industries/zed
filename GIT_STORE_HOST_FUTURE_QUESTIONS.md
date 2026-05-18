# GitStore host sharing future questions

Questions and follow-up scope that should not block the immediate active-repository refactor unless they become direct regressions.

## Visible worktree sharing

Visible worktrees should be shared across projects that target the same host whenever possible. This is one of the main goals of the multi-tenant `Host` refactor: projects should reuse host-level resources instead of duplicating worktrees, repositories, scans, and watchers.

Decisions from planning:

- Repositories should be shared when multiple projects open the same repo path.
- Visible worktrees for the same host/path should also generally be shared.
- Project ownership cannot mean "unique visible worktree entity per project".
- Project ownership should be a project-to-worktree association, not ownership of the worktree entity itself.
- Dropping one project must remove only that project's association; the shared worktree/repo should remain while any project still uses it.
- `Project::worktrees(cx)` should expose the worktrees visible to that project, even when the backing worktree entities are host-shared.

Open decisions:

- What are the exceptions where visible worktrees should not be shared?
- Can a worktree be visible in two projects at once with different project ordering/selection state?
- Where does per-project worktree order live if the worktree entity is shared?

## Remote clients and collab

Should the property test include remote clients?

Recommended sequence:

1. Stabilize local-host property tests first.
2. Add remote-host variants once local invariants are clear.
3. Add collab/share variants after remote behavior is modeled.

Remote-specific concerns:

- remote repository updates may not populate `worktree_ids_for_repository` today;
- repository ownership fallback currently defaults to claim when host worktree associations are absent;
- downstream repository broadcasts must be project-scoped;
- reconnect/rejoin paths may replay repository state in a different order than local scanning.

Open decisions:

- Should remote multi-tenant use the same repository ownership invariant as local hosts?
- If remote repository updates do not include worktree associations, where should project ownership be derived from?
- Should remote active repository state be synchronized per project or remain local UI state?

## Invisible worktrees

Invisible worktrees were not the immediate focus of this active-repository refactor, but they affect repository ownership.

Open decisions:

- If a host store creates an invisible worktree due to one project's request, should every project claim it?
- If only the initiating project should claim it, how is that project identity threaded into host-store operations?
- Are invisible worktrees ever intentionally host-global resources?

## Async stale-result bugs

Some UI paths start async work from the current active repository and later write results back. If active repo changes before the async result returns, stale data can land in the UI.

This may be pre-existing and should not expand the current refactor unless a stale-result bug is directly caused by reusing host stores.

Future mitigation:

- capture repo id when spawning async repo work;
- before applying results, compare against current project active repo id;
- discard stale results or route them to the explicit repo operation that initiated them.

## Product semantics for shared repositories

Same repo path in multiple projects should share the repository entity.

Decisions from the active-repository refactor:

- Active repository state is per-Project local state, stored as `Project::active_repository_id`.
- Active repository changes for the same shared repo id emit independently per project via `Project::Event::ActiveRepositoryChanged`.
- `GitStore` no longer owns or emits active repository changes.

Open decisions:

- If project A performs git operations on a shared repo, which status/update events should project B receive?
- Should git jobs be shown in every project that owns the repo, or only the initiating project?
- Should branch changes in one project always update UI in all projects that own the repo?
