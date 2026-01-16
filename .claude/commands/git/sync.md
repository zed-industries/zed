---
description: Full sync workflow - stash, pull, push, restore - keeps your branch in sync with remote safely
allowed-tools: Bash(git:*), Read, Edit, Write, Glob, Grep
---

# Git Sync - Full Branch Synchronization

A complete sync workflow that safely synchronizes your local branch with remote, handling all edge cases.

## Overview

This command performs a full sync:
1. Stash any local changes
2. Pull remote changes (with conflict resolution if needed)
3. Push local commits
4. Restore stashed changes

## Step 1: Pre-Sync Analysis

```bash
git status
git branch -vv
git stash list
git rev-list --left-right --count origin/$(git branch --show-current)...HEAD 2>/dev/null
```

Present:
- Current branch and tracking status
- Local commits not pushed
- Remote commits not pulled
- Uncommitted changes

## Step 2: Handle Uncommitted Changes

If there are uncommitted changes:

**Ask the user:**
"You have uncommitted changes. How would you like to proceed?
1. Stash them temporarily (will restore after sync)
2. Commit them first (will include in push)
3. Discard them (WARNING: loses changes)"

If stashing:
```bash
git stash push -m "Sync stash $(date +%Y-%m-%d_%H:%M)"
```

If committing, follow the push.md commit message format.

## Step 3: Pull Remote Changes

```bash
git fetch origin
git pull origin $(git branch --show-current)
```

If conflicts occur:
- Follow the conflict resolution protocol from pull.md
- Handle each file one at a time
- Never lose any code from either side
- Get user approval for each resolution

## Step 4: Push Local Commits

If there are local commits to push:
```bash
git log origin/$(git branch --show-current)..HEAD --oneline
```

Show what will be pushed, then:
```bash
git push origin $(git branch --show-current)
```

## Step 5: Restore Stashed Changes

If changes were stashed in Step 2:
```bash
git stash pop
```

If stash pop causes conflicts, resolve them interactively.

## Step 6: Final Status

```bash
git status
git log --oneline -5
```

Confirm:
- Branch is in sync with remote
- All local changes are intact
- No uncommitted changes (unless intentional)

## Safety Rules

- **NEVER force push**
- **NEVER auto-resolve conflicts**
- **NEVER discard changes without explicit user approval**
- **ALWAYS show what will happen before doing it**
- **ALWAYS restore stashed changes**

Start by analyzing the current repository state.
