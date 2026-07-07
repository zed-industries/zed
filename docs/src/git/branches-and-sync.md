---
title: Git Branches and Sync - Zed
description: Create, switch, delete, fetch, pull, push, force push, publish branches, and open pull request links from Zed.
---

# Branches and Sync

Use Zed's branch and remote controls to move between branches and synchronize
with remotes.

## Create and switch branches {#branches}

Open the branch picker with {#action git::Branch}, {#action git::Switch},
{#action git::CheckoutBranch}, or the branch selector in the title bar or Git
Panel.

From the picker you can:

- create a branch
- switch to a local branch
- check out a remote branch
- filter remote branches
- delete or force-delete local branches

You cannot delete the branch that is currently checked out. Switch to another
branch first.

## Fetch, pull, and push {#sync}

Use the Git Panel buttons or Command Palette actions:

| Job                           | Action                    |
| ----------------------------- | ------------------------- |
| Fetch from the default remote | {#action git::Fetch}      |
| Fetch from a chosen remote    | {#action git::FetchFrom}  |
| Pull                          | {#action git::Pull}       |
| Pull with rebase              | {#action git::PullRebase} |
| Push                          | {#action git::Push}       |
| Push to a chosen remote       | {#action git::PushTo}     |
| Force push                    | {#action git::ForcePush}  |

When a repository has multiple remotes, Zed shows a remote selector in the Git
Panel. Use the selector to choose the remote for fetch, pull, or push variants.

## Push configuration {#push-configuration}

Zed follows Git's push configuration. When pushing, Zed checks:

1. `pushRemote` configured for the current branch
2. `remote.pushDefault` in your Git config
3. the branch's tracking remote

Configure these values with Git itself, for example through `git config`.

## Publish and create pull request links {#publish-and-pr}

Use {#action git::CreatePullRequest} to open the hosting provider's pull request
or merge request creation URL when Zed can build one for the active branch.

This is a publishing handoff to the host. Zed does not provide a full in-editor
pull request review or comment-posting workflow. See [Git Hosting and Pull
Requests](./github-and-pull-requests.md) for hosting boundaries.

## Compare branches {#compare-branches}

Use {#action git::BranchDiff} to compare against the default branch, or
{#action git::CompareWithBranch} from Project Diff to choose another base.

See [Diffs and Review](./diffs-and-review.md#branch-diff) for the supported
branch-diff workflow and its boundaries.

## Stash before switching {#stash-before-switching}

When you need to set aside work before switching branches, use:

| Job                       | Action                    |
| ------------------------- | ------------------------- |
| Stash all changes         | {#action git::StashAll}   |
| Apply the latest stash    | {#action git::StashApply} |
| Pop the latest stash      | {#action git::StashPop}   |
| Browse and manage stashes | {#action git::ViewStash}  |

For stash diff and recovery details, see [Conflicts and
Recovery](./conflicts-and-recovery.md#stashes).

## See also {#see-also}

- [Worktrees](./worktrees.md): Keep multiple checkouts of one repository.
- [Git Hosting and Pull Requests](./github-and-pull-requests.md): Configure hosting
  links and PR handoffs.
- [Settings and Actions](./settings-and-actions.md): Find sync and branch
  actions.
