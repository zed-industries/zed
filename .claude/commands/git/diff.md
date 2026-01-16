---
description: Smart diff viewer - compare changes between commits, branches, or working directory
allowed-tools: Bash(git:*), Read
argument-hint: [staged|branch-name|commit-hash]
---

# Git Diff Viewer

Intelligently show differences with context and explanations.

## Default Behavior (no arguments)

Show all current changes:

```bash
# Unstaged changes
echo "=== Unstaged Changes ==="
git diff --stat
git diff

# Staged changes
echo "=== Staged Changes ==="
git diff --cached --stat
git diff --cached
```

## Argument-Based Views

Based on arguments ($ARGUMENTS):

### `staged` - Only Staged Changes
```bash
git diff --cached --stat
git diff --cached
```

### Branch Name - Compare with Branch
```bash
git diff {branch-name}..HEAD --stat
git diff {branch-name}..HEAD
```

### Commit Hash - Compare with Commit
```bash
git diff {commit-hash}..HEAD --stat
git diff {commit-hash}..HEAD
```

### File Path - Changes to Specific File
```bash
git diff -- {file-path}
git log --oneline -5 -- {file-path}
```

## Smart Analysis

For each diff shown, provide:

1. **Summary**: What files changed and how much
2. **Context**: Brief explanation of what the changes do
3. **Additions**: New code/features added (green/+)
4. **Deletions**: Code removed (red/-)
5. **Modifications**: Changed logic

## Comparison Options

Offer these views:
- `git diff` - Working directory vs staged
- `git diff HEAD` - Working directory vs last commit
- `git diff --cached` - Staged vs last commit
- `git diff branch1..branch2` - Between branches
- `git diff HEAD~3..HEAD` - Last 3 commits

## Interactive Mode

If no arguments:
"What would you like to compare?
1. Show all current changes (staged + unstaged)
2. Show only staged changes
3. Compare with another branch
4. Compare with a specific commit
5. Show changes for a specific file"

## Large Diff Handling

If diff is very large (>500 lines):
- Show stat summary first
- Ask if user wants full diff or file-by-file
- Offer to focus on specific files

Present diffs in a clear, readable format with context.
