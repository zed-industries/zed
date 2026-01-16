---
description: Comprehensive git status overview - shows branch state, changes, stashes, and recent history
allowed-tools: Bash(git:*)
---

# Comprehensive Git Status Report

Provide a complete overview of the current git repository state.

## Gather All Information

Run these commands and present a clear summary:

```bash
# Current branch and tracking info
git branch -vv

# Working directory status
git status

# Stash list
git stash list

# Recent commits on current branch (last 10)
git log --oneline -10

# Check if ahead/behind remote
git rev-list --left-right --count origin/$(git branch --show-current)...HEAD 2>/dev/null || echo "No remote tracking"

# Show any untracked files
git ls-files --others --exclude-standard

# Show staged changes summary
git diff --cached --stat

# Show unstaged changes summary
git diff --stat
```

## Present Summary

Format the output as a clear report:

### Branch Info
- Current branch name
- Tracking remote (if any)
- Commits ahead/behind remote

### Working Directory
- Staged files (ready to commit)
- Modified files (not staged)
- Untracked files
- Any conflicts

### Stashes
- List any stashed changes with their descriptions

### Recent History
- Last 5-10 commits with short descriptions

### Recommendations
Based on the state, suggest next actions:
- If uncommitted changes: "You have uncommitted changes. Consider committing or stashing."
- If behind remote: "You're behind remote. Consider pulling."
- If ahead of remote: "You have unpushed commits. Consider pushing."
- If stashes exist: "You have stashed changes. Remember to pop them if needed."
