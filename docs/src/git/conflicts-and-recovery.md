---
title: Git Conflicts and Recovery - Zed
description: Resolve merge conflicts, use conflict buttons, restore hunks, and manage stashes from Zed.
---

# Conflicts and Recovery

Zed helps you resolve conflict regions after Git reports a conflict and provides
stash and restore actions for everyday recovery.

## Resolve conflicts {#resolve-conflicts}

When a merge, rebase, cherry-pick, or pull creates conflicts, Zed highlights
conflict regions in the editor. Conflicted files also appear in the Git Panel
and Project Diff.

Each conflict region shows buttons for choosing content:

- **Use [branch-name]** keeps one side.
- **Use [other-branch]** keeps the other side.
- **Use Both** keeps both sides, with your branch's changes first.

After resolving all conflict regions in a file, stage the file and continue the
Git operation.

## Use the terminal for operation control {#terminal-fallback}

Zed's conflict buttons resolve file contents. Zed does not provide a complete
merge, rebase, cherry-pick, or three-way merge UI.

Use the [terminal](../terminal.md) for commands such as:

```sh
git rebase --continue
git merge --abort
git cherry-pick --continue
```

## Restore changes {#restore}

Use restore actions when you want to discard changes:

| Job                                     | Action                             |
| --------------------------------------- | ---------------------------------- |
| Restore selected hunks                  | {#action git::Restore}             |
| Restore selected hunks and move forward | {#action git::RestoreAndNext}      |
| Restore a file from the Git Panel       | {#action git::RestoreFile}         |
| Restore all tracked files               | {#action git::RestoreTrackedFiles} |
| Move untracked files to trash           | {#action git::TrashUntrackedFiles} |

Review the diff before restoring. Restore actions discard work.

## Stashes {#stashes}

Use stashes to set aside uncommitted work without committing it.

| Job                    | Action                    |
| ---------------------- | ------------------------- |
| Stash all changes      | {#action git::StashAll}   |
| Apply the latest stash | {#action git::StashApply} |
| Pop the latest stash   | {#action git::StashPop}   |
| Open the stash picker  | {#action git::ViewStash}  |

The stash picker lets you browse stash entries, open stash diffs, apply, pop,
and drop entries.

In a stash diff view:

| Action      | Keybinding                   |
| ----------- | ---------------------------- |
| Apply stash | {#kb git::ApplyCurrentStash} |
| Pop stash   | {#kb git::PopCurrentStash}   |
| Drop stash  | {#kb git::DropCurrentStash}  |

## See also {#see-also}

- [Status and Changes](./status-and-changes.md): Find conflicted files.
- [Diffs and Review](./diffs-and-review.md): Review conflict diffs.
- [Branches and Sync](./branches-and-sync.md): Sync or switch after recovery.
