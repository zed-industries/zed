---
description: Safe undo operations - revert commits, unstage files, or discard changes with safety checks
allowed-tools: Bash(git:*)
argument-hint: [commit|stage|changes]
---

# Git Undo Operations (Safe Mode)

Safely undo various git operations with confirmation and backups.

## CRITICAL SAFETY RULES

- **NEVER run destructive commands without showing what will happen first**
- **NEVER use --hard without explicit confirmation and showing consequences**
- **ALWAYS offer to create a backup branch before destructive operations**
- **ALWAYS show the user what will be lost before proceeding**

## Available Operations

Based on arguments ($ARGUMENTS):

### `commit` - Undo Last Commit

```bash
git log --oneline -3
```

"The last commit is: {commit message}

How would you like to undo it?
1. Keep changes staged (soft reset) - safest
2. Keep changes unstaged (mixed reset) - safe
3. Discard changes completely (hard reset) - DESTRUCTIVE"

For options 1-2:
```bash
git reset --soft HEAD~1  # or --mixed
```

For option 3:
```bash
# First, create backup
git branch backup-before-reset-$(date +%Y%m%d%H%M%S)
git reset --hard HEAD~1
```

### `stage` - Unstage Files

```bash
git diff --cached --stat
```

"These files are staged. Which would you like to unstage?
1. All files
2. Specific file(s)"

```bash
git reset HEAD {file}  # or git reset HEAD for all
```

Note: This does NOT discard changes, just unstages them.

### `changes` - Discard Working Directory Changes

```bash
git status
git diff --stat
```

**WARNING FLOW:**
1. Show exactly what will be lost
2. "Are you SURE you want to discard these changes? This CANNOT be undone."
3. If confirmed, offer to stash instead: "Would you like to stash these changes instead of discarding? You can always drop the stash later."
4. Only if user insists on discard:

```bash
git checkout -- {file}  # specific file
# or
git checkout -- .  # all files
```

### No Argument - Interactive Mode

"What would you like to undo?
1. Undo the last commit (keep changes)
2. Unstage files (keep changes)
3. Discard uncommitted changes (DESTRUCTIVE)
4. Revert a pushed commit (creates new commit)
5. Show me what I can safely undo"

## Reverting Pushed Commits

If the commit is already pushed:
```bash
git revert {commit-hash}
```

This creates a NEW commit that undoes the changes - safe for shared branches.

## Recovery Information

After any undo operation, remind user:
"If you need to recover:
- Backup branches: `git branch -a | grep backup`
- Reflog (last 30 days): `git reflog`
- Stashes: `git stash list`"

## What CAN'T Be Undone

Warn if user is about to do something unrecoverable:
- `git clean -fd` (untracked files)
- `git reset --hard` (uncommitted changes)
- `git stash drop` (stashed changes)
- Force push over remote commits

Start by determining the operation and showing current state.
