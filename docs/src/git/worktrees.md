---
title: Git Worktrees - Zed
description: Create, switch, open, delete, and configure linked Git worktrees in Zed, including worktrees for parallel agent work.
---

# Worktrees

Git worktrees let one repository have multiple checkouts on disk. In Zed, they
are useful when you want separate branches or tasks without stashing or
disturbing your main checkout.

This page covers linked Git worktrees and how they fit with Zed projects,
branches, terminals, and agent threads. Zed also uses "worktree" for opened file
and folder roots in its [trust model](../worktree-trust.md).

## Projects, Zed worktrees, and Git worktrees {#projects-zed-worktrees-git-worktrees}

A Zed project is the workspace context that owns your file tree, Git state,
search scope, terminals, tasks, and agent threads. A project can contain one
folder, multiple folders, or a mix of Git repositories and non-Git folders.

A Zed worktree is an opened file or folder root inside a project. That is broader
than a Git worktree. For example, a normal folder, a single opened file, and a
linked Git worktree can all be Zed worktrees.

A linked Git worktree is a separate checkout managed by Git. It has its own
working tree and checked-out commit or branch, while sharing Git metadata with
the original repository.

Branches are checked out inside a Git worktree. Creating or switching a Git
worktree changes the checkout you are working in; choosing a branch changes what
that checkout points at.

## Open the worktree picker {#open-worktree-picker}

Open the worktree picker from the title bar, next to the project picker, or run
{#action git::Worktree}.

From the picker you can:

- create a linked worktree from the current branch or default branch
- type a name or let Zed choose one
- switch the current workspace to an existing worktree
- open an existing worktree in a new window
- delete linked worktrees that are not currently open in the project

Worktree creation requires a Git repository in the current project and is not
supported in collaborative projects.

## Choose a branch after creating a worktree {#choose-branch}

New worktrees are created in detached HEAD state. After switching to the new
worktree, use the branch picker next to the worktree picker to create or check
out a branch.

If a branch is already checked out in another worktree, Zed keeps the current
worktree detached until you choose a different branch. This avoids checking out
the same branch in multiple worktrees.

The worktree picker and branch picker are separate. Use the worktree picker to
choose a checkout, then use the branch picker in that checkout to create,
switch, or check out a branch.

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

This means a Zed project is not the same thing as one Git repository. A project
can span multiple roots, and each Git root keeps its own Git state.

## Where worktree context appears {#worktree-context-ui}

Zed shows project, worktree, and branch context in different places:

| Surface | What it shows or controls |
| --- | --- |
| Project picker | The current project or recent project you are opening. |
| Worktree picker | The linked Git worktree checkout for the current project. |
| Branch picker | The branch or detached commit checked out in the active Git worktree. |
| Project Panel | The roots in the current project, including multi-root folders and linked Git worktrees. |
| Git Panel | The active Git repository. In multi-root projects, use the repository selector before staging, committing, fetching, pulling, or pushing. |
| Threads Sidebar | Threads grouped by project. Threads in linked Git worktrees appear with the main checkout's project group. |
| Terminal Threads | Terminal-backed threads with their own terminal working directory and shell environment. |

When you are choosing where an operation runs, use these rules:

- Git Panel actions apply to the active repository in the Git Panel.
- Branch picker actions apply to the active Git worktree checkout.
- Worktree picker actions switch, create, open, or delete linked Git worktrees for
  the current project.
- Normal terminals and Terminal Threads run commands from their terminal working
  directory.
- Agent threads run in the project/worktree context where the thread was
  created. Starting a thread from a project group in the Threads Sidebar scopes it
  to that project. Starting work in a linked Git worktree scopes edits to that
  checkout.
- `create_worktree` task hooks run after Zed creates a linked Git worktree and
  receive `ZED_WORKTREE_ROOT` and `ZED_MAIN_GIT_WORKTREE`.

## Where threads and terminals run {#thread-terminal-context}

Agent threads and Terminal Threads run in a project/worktree context. Threads
that run in linked Git worktrees appear under the same project group as the main
checkout, so related work stays together in the Threads Sidebar.

Terminal Threads use the terminal's working directory and shell environment.
They are closed rather than archived into Thread History. Closing the last
Terminal Thread reference to a Zed-managed linked Git worktree may still trigger
safe worktree cleanup. See [Terminal Threads](../ai/terminal-threads.md) for
Terminal Thread behavior and support boundaries.

## Worktrees and agents {#worktrees-and-agents}

Worktrees are useful when multiple agent threads may edit the same files. Start
one thread in a new worktree so its edits, branch, and Git state are isolated
from your main checkout.

For thread-specific behavior, see [Worktree
Isolation](../ai/parallel-agents.md#worktree-isolation). For the Git review
handoff after an agent edits files, see [Agents and Git](./agents-and-git.md).

When a thread uses a linked Git worktree, archiving or restoring the thread may
save or restore that worktree's Git state. This behavior is conditional: Zed only
cleans up worktrees it can identify as safe to manage, and Terminal Threads do
not move to Thread History.

## See also {#see-also}

- [Branches and Sync](./branches-and-sync.md): Choose a branch in a worktree.
- [Tasks](../tasks.md#hooks): Automate setup after creating a worktree.
- [Parallel Agents](../ai/parallel-agents.md): Run threads in isolated
  worktrees.
- [Windows & Projects](../windows-and-projects.md): Understand Zed projects and
  multi-root projects.
- [Zed and trusted worktrees](../worktree-trust.md): Understand Zed's broader
  worktree trust model.
