---
title: Git Worktrees - Zed
description: Create, switch, open, delete, and configure linked Git worktrees in Zed, including worktrees for parallel agent work.
---

# Worktrees

Git worktrees let one repository have multiple checkouts on disk. Use worktrees
when you want separate branches or tasks without stashing or disturbing your
main checkout.

## Open the worktree picker {#open-worktree-picker}

Open the worktree picker from the title bar, next to the project picker, or run
{#action git::Worktree}.

From the picker you can:

- create a linked worktree from the current branch or default branch
- type a name or let Zed choose one
- switch the current workspace to an existing worktree
- open an existing worktree in a new window
- delete linked worktrees that are not currently open in the project

## Choose a branch after creating a worktree {#choose-branch}

New worktrees are created in detached HEAD state. After switching to the new
worktree, use the branch picker next to the worktree picker to create or check
out a branch.

If a branch is already checked out in another worktree, Zed keeps the current
worktree detached until you choose a different branch. This avoids checking out
the same branch in multiple worktrees.

## Configure where worktrees are created {#worktree-directory}

The `git.worktree_directory` setting controls where Zed creates linked
worktrees. By default, Zed creates worktrees under `../worktrees` relative to
the repository's working directory.

```json [settings]
{
  "git": {
    "worktree_directory": "../worktrees"
  }
}
```

See [All Settings](../reference/all-settings.md#git-worktree-directory) for the
full setting reference.

## Run setup after worktree creation {#setup-hook}

Use the [`create_worktree` task hook](../tasks.md#hooks) to run setup commands
after Zed creates a linked worktree. The hook receives `ZED_WORKTREE_ROOT` for
the new worktree and `ZED_MAIN_GIT_WORKTREE` for the original repository.

## Multi-root workspaces {#multi-root-workspaces}

If a project contains multiple Git repositories, Zed creates a linked worktree
for each repository when you create a worktree from the picker. Non-Git folders
in the same project are included in the new workspace as-is.

## Worktrees and agents {#worktrees-and-agents}

Worktrees are useful when multiple agent threads may edit the same files. Start
one thread in a new worktree so its edits, branch, and Git state are isolated
from your main checkout.

For thread-specific behavior, see [Worktree
Isolation](../ai/parallel-agents.md#worktree-isolation) and [Agents and
Git](./agents-and-git.md).

## See also {#see-also}

- [Branches and Sync](./branches-and-sync.md): Choose a branch in a worktree.
- [Tasks](../tasks.md#hooks): Automate setup after creating a worktree.
- [Parallel Agents](../ai/parallel-agents.md): Run threads in isolated
  worktrees.
