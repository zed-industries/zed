---
title: Git Settings and Actions - Zed
description: Find Zed Git settings, command palette actions, keybindings, Git Panel options, diff settings, and hosting provider configuration.
---

# Settings and Actions

Use this page as a workflow-oriented reference for Git settings and actions.
For generated setting details, see [All Settings](../reference/all-settings.md).

## Git Panel settings {#git-panel-settings}

Open the Settings Editor with {#action zed::OpenSettings} and search for
**Git Panel**, or configure:

```json [settings]
{
  "git_panel": {
    "dock": "left",
    "button": true,
    "tree_view": false,
    "sort_by": "path",
    "group_by": "status",
    "show_count_badge": false
  }
}
```

Common jobs:

| Goal                                  | Setting or UI                           |
| ------------------------------------- | --------------------------------------- |
| Move the Git Panel                    | **Panels > Git Panel > Git Panel Dock** |
| Hide the Git Panel status-bar button  | `git_panel.button`                      |
| Group changed files by folder         | `git_panel.tree_view` or the panel menu |
| Sort changes by path                  | `git_panel.sort_by`                     |
| Show a badge with uncommitted changes | `git_panel.show_count_badge`            |

## Version control display settings {#version-control-settings}

Use the Settings Editor under **Version Control** for gutter and blame display.

```json [settings]
{
  "git": {
    "git_gutter": "tracked_files",
    "hunk_style": "staged_hollow",
    "inline_blame": {
      "enabled": true,
      "delay_ms": 600
    }
  }
}
```

See [Status and Changes](./status-and-changes.md#editor-indicators) and [History
and Blame](./history-and-blame.md#blame) for how these settings affect daily
workflows.

## Diff settings {#diff-settings}

Use `diff_view_style` for split or unified diffs:

```json [settings]
{
  "diff_view_style": "split"
}
```

Use `word_diff_enabled` per language when word-level highlighting is too noisy:

```json [settings]
{
  "languages": {
    "Markdown": {
      "word_diff_enabled": false
    }
  }
}
```

## Worktree settings {#worktree-settings}

Use `git.worktree_directory` to choose where Zed creates linked worktrees:

```json [settings]
{
  "git": {
    "worktree_directory": "../worktrees"
  }
}
```

See [Worktrees](./worktrees.md).

## Hosting provider settings {#hosting-provider-settings}

Use `git_hosting_providers` for self-hosted Git providers:

```json [settings]
{
  "git_hosting_providers": [
    {
      "provider": "gitlab",
      "name": "Corp GitLab",
      "base_url": "https://git.example.corp"
    }
  ]
}
```

See [Git Hosting and Pull Requests](./github-and-pull-requests.md).

## Core Git actions {#core-git-actions}

| Job                     | Action                               | Keybinding                       |
| ----------------------- | ------------------------------------ | -------------------------------- |
| Open Git Panel          | {#action git_panel::ToggleFocus}     | {#kb git_panel::ToggleFocus}     |
| Open Project Diff       | {#action git::Diff}                  | {#kb git::Diff}                  |
| Branch Diff             | {#action git::BranchDiff}            | {#kb git::BranchDiff}            |
| Compare With Branch     | {#action git::CompareWithBranch}     | {#kb git::CompareWithBranch}     |
| Review Diff with agent  | {#action git::ReviewDiff}            | {#kb git::ReviewDiff}            |
| Stage all               | {#action git::StageAll}              | {#kb git::StageAll}              |
| Unstage all             | {#action git::UnstageAll}            | {#kb git::UnstageAll}            |
| Stage hunk and next     | {#action git::StageAndNext}          | {#kb git::StageAndNext}          |
| Unstage hunk and next   | {#action git::UnstageAndNext}        | {#kb git::UnstageAndNext}        |
| Commit                  | {#action git::Commit}                | {#kb git::Commit}                |
| Amend                   | {#action git::Amend}                 | {#kb git::Amend}                 |
| Uncommit                | {#action git::Uncommit}              | {#kb git::Uncommit}              |
| Generate commit message | {#action git::GenerateCommitMessage} | {#kb git::GenerateCommitMessage} |

## Branch, sync, and recovery actions {#branch-sync-recovery-actions}

| Job                   | Action                             | Keybinding                     |
| --------------------- | ---------------------------------- | ------------------------------ |
| Create branch         | {#action git::Branch}              | {#kb git::Branch}              |
| Switch branch         | {#action git::Switch}              | {#kb git::Switch}              |
| Checkout branch       | {#action git::CheckoutBranch}      | {#kb git::CheckoutBranch}      |
| Worktree picker       | {#action git::Worktree}            | {#kb git::Worktree}            |
| Fetch                 | {#action git::Fetch}               | {#kb git::Fetch}               |
| Pull                  | {#action git::Pull}                | {#kb git::Pull}                |
| Pull with rebase      | {#action git::PullRebase}          | {#kb git::PullRebase}          |
| Push                  | {#action git::Push}                | {#kb git::Push}                |
| Force push            | {#action git::ForcePush}           | {#kb git::ForcePush}           |
| Create PR URL         | {#action git::CreatePullRequest}   | {#kb git::CreatePullRequest}   |
| Stash all             | {#action git::StashAll}            | {#kb git::StashAll}            |
| Apply latest stash    | {#action git::StashApply}          | {#kb git::StashApply}          |
| Pop latest stash      | {#action git::StashPop}            | {#kb git::StashPop}            |
| View stashes          | {#action git::ViewStash}           | {#kb git::ViewStash}           |
| Restore hunks         | {#action git::Restore}             | {#kb git::Restore}             |
| Restore tracked files | {#action git::RestoreTrackedFiles} | {#kb git::RestoreTrackedFiles} |

## History and hosting actions {#history-hosting-actions}

| Job                 | Action                                 | Keybinding                         |
| ------------------- | -------------------------------------- | ---------------------------------- |
| File history        | {#action git::FileHistory}             | {#kb git::FileHistory}             |
| Blame               | {#action git::Blame}                   | {#kb git::Blame}                   |
| Toggle inline blame | {#action editor::ToggleGitBlameInline} | {#kb editor::ToggleGitBlameInline} |
| Open Git Graph      | {#action git_graph::Open}              | {#kb git_graph::Open}              |
| Copy permalink      | {#action editor::CopyPermalinkToLine}  | {#kb editor::CopyPermalinkToLine}  |
| Open permalink      | {#action editor::OpenPermalinkToLine}  | {#kb editor::OpenPermalinkToLine}  |

Not every action has a default keybinding. Add custom bindings in your
[keymap](../key-bindings.md#user-keymaps).

## See also {#see-also}

- [All Actions](../all-actions.md): Generated action reference.
- [All Settings](../reference/all-settings.md): Generated setting reference.
- [Command Palette](../command-palette.md): Run actions by name.
