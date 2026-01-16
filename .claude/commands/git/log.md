---
description: Pretty git history viewer - show commits with various filters and formats
allowed-tools: Bash(git:*)
argument-hint: [number-of-commits] [--author=name] [--since=date]
---

# Git Log Viewer

View commit history with various filters and beautiful formatting.

## Default View (no arguments)

Show last 20 commits in a readable format:

```bash
git log --oneline --graph --decorate -20
```

Also show:
```bash
git log --pretty=format:"%C(yellow)%h%Creset %C(blue)%ad%Creset %C(green)%an%Creset %s" --date=short -20
```

## Filtered Views

Based on arguments ($ARGUMENTS):

### By Count
If a number is provided:
```bash
git log --oneline -N
```

### By Author
```bash
git log --author="name" --oneline -20
```

### By Date Range
```bash
git log --since="2024-01-01" --until="2024-12-31" --oneline
```

### By File
```bash
git log --follow -- path/to/file
```

## Detailed Commit View

For examining a specific commit:
```bash
git show {commit-hash} --stat
git show {commit-hash} -p
```

## Comparison Views

### What's not pushed:
```bash
git log origin/$(git branch --show-current)..HEAD --oneline
```

### What's not pulled:
```bash
git log HEAD..origin/$(git branch --show-current) --oneline
```

### Commits on branch since diverging from main:
```bash
git log main..HEAD --oneline
```

## Interactive Mode

If no arguments, show menu:
"How would you like to view the history?
1. Recent commits (last 20)
2. Commits by a specific author
3. Commits in a date range
4. Commits for a specific file
5. Unpushed commits
6. Detailed view of a specific commit"

## Pretty Formats Available

- **oneline**: Short hash + message
- **graph**: Visual branch structure
- **detailed**: Full commit info with stats
- **files**: Show which files changed in each commit

Present the log output in a clear, readable format.
