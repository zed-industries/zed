---
title: Git Diffs and Review - Zed
description: Review working-tree changes, branch diffs, single-file diffs, and agent-produced changes in Zed.
---

# Diffs and Review

Zed uses editable diff surfaces for reviewing Git changes. Most diff views are
[multibuffers](../multibuffers.md), so you can edit excerpts while reviewing.

## Choose a review path {#choose-review-path}

| Goal                                         | Use                                                                |
| -------------------------------------------- | ------------------------------------------------------------------ |
| Review all working-tree changes              | {#action git::Diff} or **Open Diff** in the Git Panel              |
| Review one changed file                      | **Open Diff (File)** from the Git Panel file context menu          |
| Compare your work against the default branch | {#action git::BranchDiff}                                          |
| Compare your work against a specific branch  | {#action git::CompareWithBranch} from Project Diff                 |
| Ask an agent to review a branch diff         | {#action git::ReviewDiff} or [Agents and Git](./agents-and-git.md) |
| Review changes made by an agent              | [Agent change review](./agents-and-git.md#review-agent-changes)    |

## Project Diff {#project-diff}

Open Project Diff with {#action git::Diff} or {#kb git::Diff}. Project Diff
shows changed hunks across the active repository.

From Project Diff you can:

- edit changed excerpts directly
- stage or unstage individual hunks
- stage or unstage all hunks
- restore selected hunks
- switch between split and unified diff styles

Useful hunk actions:

| Action                        | Keybinding                |
| ----------------------------- | ------------------------- |
| {#action git::StageAndNext}   | {#kb git::StageAndNext}   |
| {#action git::UnstageAndNext} | {#kb git::UnstageAndNext} |
| {#action git::StageAll}       | {#kb git::StageAll}       |
| {#action git::UnstageAll}     | {#kb git::UnstageAll}     |
| {#action git::Restore}        | {#kb git::Restore}        |

## Branch Diff and Compare With Branch {#branch-diff}

Use {#action git::BranchDiff} to compare the working directory against the
repository's default branch, usually `main` or `master`.

Use {#action git::CompareWithBranch} from Project Diff to choose a different
base branch. Zed opens the branch picker, then updates Project Diff to compare
against the selected branch.

> **Note:** Branch Diff opens the Project Diff review surface. Zed does not
> support inline branch-diff hunks in ordinary editor buffers. Editor gutter
> indicators show working-tree changes.

## Staged and unstaged review {#staged-and-unstaged-review}

Zed tracks staged, unstaged, and partially staged state in the Git Panel and in
diff hunk controls. You can stage or unstage hunks while reviewing.

Zed does not support a separate PR-style review surface that shows only the
staged commit apart from all unstaged work. Use hunk-level stage state, the Git
Panel, and Project Diff together to verify what will be committed. For commit
mechanics, see [Staging and Committing](./staging-and-committing.md).

## Diff view style {#diff-view-style}

Zed supports split and unified diff views. Split view shows old and new content
side by side. Unified view shows changes inline.

Open the Settings Editor with {#action zed::OpenSettings} and search for
`diff_view_style`, or add:

```json [settings]
{
  "diff_view_style": "unified"
}
```

The setting applies to Project Diff, File History, commit views, and stash diff
views.

## Word diff highlighting {#word-diff}

Zed highlights changed words inside modified lines. To disable word diff for a
language, add:

```json [settings]
{
  "languages": {
    "Markdown": {
      "word_diff_enabled": false
    }
  }
}
```

## See also {#see-also}

- [Status and Changes](./status-and-changes.md): Find changed files before
  opening a diff.
- [History and Blame](./history-and-blame.md): Review previous commits and file
  history.
- [Agents and Git](./agents-and-git.md): Review agent edits and branch diffs.
