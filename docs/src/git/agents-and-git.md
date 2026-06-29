---
title: Agents and Git - Zed
description: Review agent changes, use branch diff context, isolate agents in Git worktrees, and generate commit messages in Zed.
---

# Agents and Git

Agent workflows often create, review, and isolate Git changes. This page covers
the Git side of those workflows. Model setup, tools, profiles, and permissions
stay canonical in the [AI docs](../ai/quick-start.md).

## Review agent changes {#review-agent-changes}

After an agent edits your project, the Agent Panel shows a changed-files
summary. Open the review view with {#action agent::OpenAgentDiff} or {#kb
agent::OpenAgentDiff}.

From the agent diff you can accept or reject individual hunks or the whole set
of changes. The review view is separate from the normal Git index. After
accepting changes, use [Diffs and Review](./diffs-and-review.md) and [Staging
and Committing](./staging-and-committing.md) to stage and commit the result.

If `agent.single_file_review` is enabled, agent edit diffs can also appear
inline in individual files. While single-file review is active, the agent review
diff temporarily overrides the buffer's normal Git diff.

```json [settings]
{
  "agent": {
    "single_file_review": true
  }
}
```

## Review a branch diff with an agent {#review-branch-diff}

Use {#action git::ReviewDiff} to open an agent thread with branch-diff context.
You can also add branch diff context from the Agent Panel message editor by
typing `@` and choosing **Branch Diff**.

Branch diff context uses the Project Diff branch comparison model. See [Branch
Diff and Compare With Branch](./diffs-and-review.md#branch-diff) for the Git
review surface and boundary.

## Isolate agent work in a worktree {#agent-worktrees}

If multiple agent threads may edit the same files, create or switch to a linked
Git worktree before starting one of the threads. Worktrees isolate the checkout,
branch, and working tree.

Worktrees are managed from the title bar picker or {#action git::Worktree}. For
thread-specific behavior, see [Worktree
Isolation](../ai/parallel-agents.md#worktree-isolation).

## Generate commit messages {#agent-commit-messages}

AI commit message generation is a Git Panel workflow that uses your configured
LLM provider. Use {#action git::GenerateCommitMessage} from the commit editor.

See [Generate a commit message with
AI](./staging-and-committing.md#ai-commit-message) for Git-specific settings and
[AI Quick Start](../ai/quick-start.md) for provider setup.

## See also {#see-also}

- [Agent Panel](../ai/agent-panel.md): Prompt agents, add context, and review
  changes.
- [Parallel Agents](../ai/parallel-agents.md): Run multiple threads and isolate
  worktrees.
- [Diffs and Review](./diffs-and-review.md): Review Git branch and working-tree
  diffs.
