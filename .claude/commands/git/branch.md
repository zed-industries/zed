---
description: Branch management - create, switch, delete, or list branches with safety checks
allowed-tools: Bash(git:*)
argument-hint: [create|switch|delete|list] [branch-name]
---

# Git Branch Management

Safe branch operations with checks to prevent accidental data loss.

## Available Operations

Based on arguments ($ARGUMENTS), perform the appropriate action:

### `list` or no argument - Show All Branches

```bash
# Local branches with tracking info
git branch -vv

# Remote branches
git branch -r

# Show current branch
git branch --show-current
```

Present:
- Current branch (highlighted)
- Local branches with last commit
- Which branches track remotes
- How far ahead/behind each is

### `create [name]` - Create New Branch

```bash
git status
```

If uncommitted changes exist, warn user and ask how to proceed.

Create and switch to new branch:
```bash
git checkout -b {branch-name}
```

Ask: "Should I push this branch to remote and set up tracking?"

If yes:
```bash
git push -u origin {branch-name}
```

### `switch [name]` - Switch Branches

```bash
git status
```

If uncommitted changes:
"You have uncommitted changes. Options:
1. Stash them before switching
2. Commit them first
3. Carry them to the new branch (if possible)
4. Cancel the switch"

Show available branches:
```bash
git branch -a
```

Then switch:
```bash
git checkout {branch-name}
```

### `delete [name]` - Delete a Branch

**Safety checks first:**

```bash
# Check if branch is merged
git branch --merged | grep {branch-name}

# Check if branch has unmerged commits
git log main..{branch-name} --oneline
```

If branch has unmerged commits:
"WARNING: This branch has commits not merged to main:
{list commits}

Are you absolutely sure you want to delete it?"

If merged or user confirms:
```bash
git branch -d {branch-name}
```

Ask: "Delete from remote too?"
```bash
git push origin --delete {branch-name}
```

## Branch Naming Suggestions

When creating branches, suggest naming conventions:
- `feature/description` - New features
- `fix/description` - Bug fixes
- `hotfix/description` - Urgent production fixes
- `refactor/description` - Code refactoring
- `docs/description` - Documentation updates

## Safety Rules

- **NEVER delete main, master, or production branches**
- **ALWAYS check for uncommitted changes before switching**
- **ALWAYS check for unmerged commits before deleting**
- **ALWAYS confirm before deleting branches with unmerged work**
- **NEVER force delete (-D) without explicit user approval**

Start by determining the operation from arguments.
