---
title: Git and Review - Zed
description: Use Zed's Git tools to inspect status, review diffs, stage and commit changes, sync branches, browse history, and resolve conflicts.
---

# Git and Review

Zed includes Git tools for everyday version control: checking repository status,
reviewing diffs, staging and committing, syncing branches, browsing history,
resolving conflicts, and handing changes to agents.

Use this page as the entry point. Each linked page focuses on a job developers
come to the docs to do.

## Quick start {#quick-start}

| Goal                                      | Go to                                                         |
| ----------------------------------------- | ------------------------------------------------------------- |
| See changed, staged, and conflicted files | [Status and Changes](./git/status-and-changes.md)             |
| Review working-tree or branch diffs       | [Diffs and Review](./git/diffs-and-review.md)                 |
| Stage, unstage, amend, uncommit, commit   | [Staging and Committing](./git/staging-and-committing.md)     |
| Fetch, pull, push, publish, or switch     | [Branches and Sync](./git/branches-and-sync.md)               |
| Work in multiple checkouts                | [Worktrees](./git/worktrees.md)                               |
| Browse Git Graph, file history, or blame  | [History and Blame](./git/history-and-blame.md)               |
| Resolve merge conflicts or recover work   | [Conflicts and Recovery](./git/conflicts-and-recovery.md)     |
| Copy permalinks or create PR links        | [GitHub and Pull Requests](./git/github-and-pull-requests.md) |
| Review agent changes or branch diffs      | [Agents and Git](./git/agents-and-git.md)                     |
| Find Git settings and actions             | [Settings and Actions](./git/settings-and-actions.md)         |

Open the Git Panel with {#action git_panel::ToggleFocus}, {#kb
git_panel::ToggleFocus}, or the Git icon in the status bar. The Git Panel is
the main entry point for repository status, staging, committing, branch sync,
stashes, and recent commits.

Open the Project Diff with {#action git::Diff} or {#kb git::Diff} when you want
to review and edit changed hunks across the project.

## What Zed supports {#supported-workflows}

Zed supports these Git workflows in the editor:

- Repository status, changed files, staged state, conflicts, and multi-root Git
  projects.
- Working-tree diffs, branch diffs, compare-with-branch, and hunk-level stage,
  unstage, and restore actions.
- Staging, unstaging, committing, amending the last commit, uncommitting the last
  commit, and AI-generated commit messages.
- Branch creation, checkout, delete, fetch, pull, pull with rebase, push, force
  push, remote selection, and create-pull-request URLs.
- Linked Git worktrees, including multi-root worktree creation and agent
  worktree isolation.
- Git Graph, commit views, file history, inline blame, and commit/file
  permalinks.
- Conflict region buttons, conflicted files in the Git Panel, Project Diff
  conflict views, and stash operations.

For Git operations that Zed does not expose directly, use the integrated
[terminal](./terminal.md).

## Support boundaries {#support-boundaries}

Some Git-related workflows are intentionally narrow today:

- Branch Diff opens a Project Diff review surface. Zed does not currently
  document inline branch-diff hunks inside ordinary editor buffers as a
  supported workflow.
- Create Pull Request opens a host URL for creating a pull request or merge
  request. Zed does not provide a full in-editor PR review and comment-posting
  workflow.
- Conflict resolution buttons help resolve conflict regions after Git reports a
  conflict. Zed does not provide a complete merge, rebase, cherry-pick, or
  three-way merge UI.
- Agent setup, model configuration, and tool permissions live in the
  [AI docs](./ai/quick-start.md). Git docs only cover how agent workflows touch
  diffs, worktrees, and changed files.

## See also {#see-also}

- [Command Palette](./command-palette.md): Run Git actions by name.
- [Keybindings](./key-bindings.md): Bind Git actions that do not have defaults.
- [Tasks](./tasks.md#custom-git-commands): Add custom Git commands to Git
  Graph context menus.
