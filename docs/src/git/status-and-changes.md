---
title: Git Status and Changes - Zed
description: Use the Git Panel, editor gutter, and status indicators to understand changed, staged, untracked, and conflicted files in Zed.
---

# Status and Changes

Use Zed's Git status surfaces to see what changed, what is staged, and what
needs attention before you commit or sync.

## Open the Git Panel {#open-git-panel}

Open the Git Panel with {#action git_panel::ToggleFocus}, {#kb
git_panel::ToggleFocus}, or the Git icon in the status bar.

The Git Panel shows:

- the active repository and branch
- changed, untracked, staged, partially staged, and conflicted files
- the commit editor
- fetch, pull, push, stash, branch, history, and diff entry points

Zed watches the repository, so changes made from the terminal or another Git
tool update the panel.

## Read file state {#file-state}

Each changed file has a staged-state control:

| State            | Meaning                                                 |
| ---------------- | ------------------------------------------------------- |
| Unstaged         | The file has changes that are not in the index.         |
| Staged           | The file's current changes are ready to commit.         |
| Partially staged | Some hunks are staged and other hunks remain unstaged.  |
| Conflict         | Git reported a conflict that must be resolved manually. |

Use the checkbox beside a file to stage or unstage that file. Use
{#action git::StageAll} and {#action git::UnstageAll} for repository-wide
changes.

## Choose flat or tree view {#tree-view}

The Git Panel shows a flat list by default. Open the panel menu and toggle
**Tree View** to group changed files by folder.

You can also configure this in the Settings Editor under **Panels > Git Panel**,
or in [Settings and Actions](./settings-and-actions.md#git-panel-settings).

## Review a file from status {#review-file}

From a changed file in the Git Panel:

- Select the file to open its diff.
- Open the context menu and choose **View File** to open the file without a diff
  view.
- Open the context menu and choose **Open Diff (File)** to review only that
  file.
- Choose **View File History** to inspect previous commits for that path.

The Project Diff is the better entry point when you want to review several
files or hunks together. See [Diffs and Review](./diffs-and-review.md).

## Use editor indicators {#editor-indicators}

When a file has Git changes, Zed shows gutter indicators for added, modified,
and deleted lines. Expand a hunk to inspect details, then stage, unstage, or
restore the hunk.

Useful hunk actions:

| Action                                    | Keybinding                            |
| ----------------------------------------- | ------------------------------------- |
| {#action editor::ExpandAllDiffHunks}      | {#kb editor::ExpandAllDiffHunks}      |
| {#action editor::ToggleSelectedDiffHunks} | {#kb editor::ToggleSelectedDiffHunks} |
| {#action editor::GoToHunk}                | {#kb editor::GoToHunk}                |
| {#action editor::GoToPreviousHunk}        | {#kb editor::GoToPreviousHunk}        |
| {#action editor::Cancel}                  | {#kb editor::Cancel}                  |

> **Note:** Editor gutter hunks show working-tree changes. Branch Diff uses the
> Project Diff surface, not inline branch-diff hunks in ordinary editor buffers.

## Multi-root projects {#multi-root-projects}

If a project contains multiple Git repositories, the Git Panel lets you switch
between repositories. Actions such as stage, commit, fetch, pull, and push apply
to the active repository.

Use the repository selector in the panel header when the file you want is in a
different repository.

## See also {#see-also}

- [Diffs and Review](./diffs-and-review.md): Review working-tree and branch
  diffs.
- [Staging and Committing](./staging-and-committing.md): Move changes into a
  commit.
- [Settings and Actions](./settings-and-actions.md): Configure Git Panel and
  gutter behavior.
