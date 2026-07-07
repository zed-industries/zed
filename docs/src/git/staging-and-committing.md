---
title: Git Staging and Committing - Zed
description: Stage, unstage, commit, amend, uncommit, and generate commit messages with Zed's Git tools.
---

# Staging and Committing

Use the Git Panel and Project Diff to decide what belongs in the next commit,
then commit from Zed without leaving the editor.

## Stage changes {#stage-changes}

You can stage from the Git Panel or Project Diff:

| Scope      | Action                                                                    |
| ---------- | ------------------------------------------------------------------------- |
| One file   | Click the file checkbox in the Git Panel, or use {#action git::StageFile} |
| One hunk   | Use hunk controls in Project Diff or {#action git::StageAndNext}          |
| A range    | Select status entries and use {#action git::StageRange}                   |
| Everything | Use {#action git::StageAll}                                               |

To undo staging, use the matching unstage controls, {#action git::UnstageFile},
{#action git::UnstageAndNext}, or {#action git::UnstageAll}.

## Commit from the Git Panel {#commit}

Type a commit message in the Git Panel commit editor, then use
{#action git::Commit} or {#kb git::Commit}.

Zed commits staged changes. If tracked files have unstaged changes and nothing
is staged, the Git Panel can stage tracked changes as part of the commit flow.
Untracked files must be staged before they are committed.

Use {#action git::ExpandCommitEditor} or {#kb git::ExpandCommitEditor} when you
need more space for a longer commit message.

## Amend the last commit {#amend}

Use {#action git::Amend} from the Git Panel when staged changes should be added
to the previous commit. Zed loads the previous commit message while amend mode
is active.

Review staged state before amending. Amend rewrites the last commit.

## Uncommit the last commit {#uncommit}

After committing, the Git Panel shows the previous commit. Use
{#action git::Uncommit} to undo the last commit while keeping its changes in the
working tree.

This is equivalent to a soft reset of the last commit. Use the terminal for more
advanced reset workflows.

## Generate a commit message with AI {#ai-commit-message}

Focus the Git Panel commit editor, then use {#action git::GenerateCommitMessage},
{#kb git::GenerateCommitMessage}, or the pencil button to generate a commit
message.

AI commit generation requires an LLM provider. Start with [AI Quick
Start](../ai/quick-start.md) if you have not configured one.

To choose the model used for commit messages, add:

```json [settings]
{
  "agent": {
    "commit_message_model": {
      "provider": "anthropic",
      "model": "claude-4-5-haiku"
    }
  }
}
```

To add instructions only for commit message generation:

```json [settings]
{
  "agent": {
    "commit_message_instructions": "Use the Conventional Commits format: <type>(<scope>): <description>."
  }
}
```

For instructions that apply to the agent more broadly, use [AI
Instructions](../ai/instructions.md).

## Commit from the terminal {#terminal-editor}

To use Zed as your Git commit message editor for command-line commits:

```sh
git config --global core.editor "zed --wait"
```

Or set:

```sh
export GIT_EDITOR="zed --wait"
```

## See also {#see-also}

- [Diffs and Review](./diffs-and-review.md): Review hunks before staging.
- [Branches and Sync](./branches-and-sync.md): Push or publish after committing.
- [Agents and Git](./agents-and-git.md): Review changes produced by agents.
