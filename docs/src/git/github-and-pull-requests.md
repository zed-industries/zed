---
title: Git Hosting and Pull Requests - Zed
description: Configure Git hosting links, copy permalinks, and open pull request creation URLs from Zed.
---

# Git Hosting and Pull Requests

Zed integrates with Git hosting providers for links, permalinks, commit
references, and pull request creation handoffs.

## Supported hosting links {#hosting-links}

Zed recognizes common hosting providers from your Git remote URL and turns
commit hashes, issue references, pull request references, and merge request
references into clickable links.

Built-in provider support includes GitHub, GitLab, Bitbucket, SourceHut,
Codeberg, Gitee, Azure DevOps, Gitea, and Forgejo.

## Self-hosted providers {#self-hosted}

If your remote URL does not identify the provider clearly, configure
`git_hosting_providers`:

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

Supported `provider` values include `github`, `gitlab`, `bitbucket`, `gitea`,
`forgejo`, and `sourcehut`. The `name` field is optional.

## Permalinks {#permalinks}

Use {#action editor::CopyPermalinkToLine} to copy a permanent link to the
current line or selected line range. Use {#action editor::OpenPermalinkToLine}
to open the hosted source link.

Permalinks point at a specific commit when the hosting provider supports that
URL format.

## Create pull request links {#create-pull-request}

Use {#action git::CreatePullRequest} to open the hosting provider's pull request
or merge request creation page for the active branch when Zed can build the URL.

This action is a handoff to the host. Finish the pull request or merge request
in the browser.

## PR review boundary {#pr-review-boundary}

Zed does not support full in-editor pull request review. Creating a PR link,
opening permalinks, and clicking hosted references are handoffs to the Git host;
they are not workflows for reviewing PR files, posting review comments, or
managing hosted PR state from Zed.

For branch-level code review inside Zed, use [Branch Diff](./diffs-and-review.md#branch-diff)
or [Agents and Git](./agents-and-git.md#review-branch-diff).

## See also {#see-also}

- [Branches and Sync](./branches-and-sync.md): Publish and push branches.
- [Diffs and Review](./diffs-and-review.md): Review branch diffs in Zed.
- [Settings and Actions](./settings-and-actions.md): Configure providers and
  permalink actions.
