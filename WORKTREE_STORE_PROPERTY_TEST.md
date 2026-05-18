# WorktreeStore property test direction

This note tracks invariants that should move out of the GitStore property test and into a dedicated WorktreeStore property test.

## Principle

The property test should read like a small state machine, not a mini framework. Generate only valid operations from the current model state, apply them through real `FakeFs` / `Project` / `WorktreeStore` flows, and assert invariants after operations and from event/observe subscriptions.

## Visible worktree sharing

Visible worktrees should generally be shared across projects that target the same host and open the same path. This invariant belongs at the WorktreeStore layer rather than in the GitStore property test.

Invariant:

- If multiple live projects have a visible worktree for the same host/path, they should reference the same backing `Worktree` entity.
- `Project::worktrees(cx)` should still expose only the worktrees visible to that project, even when the backing entity is host-shared.
- Dropping one project should remove only that project's association; the shared worktree should remain while at least one live project still references it.

Candidate generated operations:

- open a project on an existing host/path;
- open a project on a disjoint path;
- add a visible worktree path to a live project;
- add the same visible worktree path to another live project;
- drop a project;
- remove a worktree from one project while another project still references it.

Candidate invariants:

- all generated projects share the same host `WorktreeStore`;
- same visible path maps to one shared `Worktree` entity;
- every project-visible worktree is present in that project's `Project::worktrees(cx)` view;
- a worktree entity remains alive while at least one live project references it;
- dropping/removing one project association does not remove the worktree from sibling project views;
- host worktrees without live project associations are released, except for explicitly allowed host-only/invisible resources.
