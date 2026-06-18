---
title: Git History and Blame - Zed
description: Browse Git Graph, commit views, file history, inline blame, and commit permalinks in Zed.
---

# History and Blame

Use Zed's history tools to inspect commits, understand why code changed, and
share links to specific lines or commits.

## Git Panel history {#git-panel-history}

Open the Git Panel with {#action git_panel::ToggleFocus}, then switch to the
History tab to see recent commits for the active repository. Select a commit to
open its details and changed files.

## Git Graph {#git-graph}

Open Git Graph with {#action git_graph::Open} or from the Git Panel history
area.

Git Graph shows the commit graph for a repository. From Git Graph you can:

- inspect commit metadata and changed files
- open a commit view
- copy a commit SHA or tag
- search commits
- run custom Git command tasks from a commit context menu

Useful Git Graph actions:

| Action                              | Keybinding                      |
| ----------------------------------- | ------------------------------- |
| {#action git_graph::Open}           | {#kb git_graph::Open}           |
| {#action git_graph::OpenCommitView} | {#kb git_graph::OpenCommitView} |
| {#action git_graph::CopyCommitSha}  | {#kb git_graph::CopyCommitSha}  |
| {#action git_graph::FocusSearch}    | {#kb git_graph::FocusSearch}    |

Use [custom Git command tasks](../tasks.md#custom-git-commands) when you want to
add repository-specific commands to the Git Graph context menu.

## File History {#file-history}

File History shows commits that changed a selected file, folder, or project
path. Open File History with {#action git::FileHistory}, or from context menus
in:

- the Project Panel
- the Git Panel
- an editor tab
- an editor buffer context menu

Selecting a commit opens a diff view for that path at that commit.

## Blame {#blame}

Use {#action git::Blame} to show Git blame for the current file. Zed also shows
inline blame for the current line when inline blame is enabled.

Open the Settings Editor and search for `inline_blame`, or configure:

```json [settings]
{
  "git": {
    "inline_blame": {
      "enabled": true,
      "delay_ms": 600,
      "show_commit_summary": true
    }
  }
}
```

Use {#action editor::ToggleGitBlameInline} to toggle inline blame.

## Permalinks from history {#permalinks}

Commit views and editor selections can create links to hosted source when Zed
can identify the Git hosting provider. Use {#action
editor::CopyPermalinkToLine} or {#action editor::OpenPermalinkToLine} from the
editor.

See [GitHub and Pull Requests](./github-and-pull-requests.md#permalinks) for
provider support and self-hosted configuration.

## Known boundaries {#boundaries}

Git Graph, File History, and blame are local history tools. Zed does not support
full pull request review, hosted issue triage, or host-specific history filters
inside these views.

## See also {#see-also}

- [Diffs and Review](./diffs-and-review.md): Review current and branch changes.
- [GitHub and Pull Requests](./github-and-pull-requests.md): Configure hosted
  links.
- [Settings and Actions](./settings-and-actions.md): Configure inline blame and
  history actions.
