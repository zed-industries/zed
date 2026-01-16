---
description: Push local commits to remote - handles pull/merge if needed, syncs your branch with origin
allowed-tools: Bash(git:*), Read, Edit, Write, Glob, Grep
---

# Git Push - Sync Local Commits to Remote

Push your local commits to remote. Handles merge conflicts if remote has new commits.

## Step 1: Pre-Push Analysis

```bash
git status
git branch -vv
```

Check for unpushed commits:
```bash
git log origin/$(git branch --show-current)..HEAD --oneline 2>/dev/null || echo "No remote tracking yet"
```

Check if remote has new commits:
```bash
git fetch origin
git log HEAD..origin/$(git branch --show-current) --oneline 2>/dev/null || echo "No new remote commits"
```

## Step 2: Handle Uncommitted Changes

If there are uncommitted changes (modified, new, or deleted files):

**Check for ALL change types:**
- Modified files (M)
- New/untracked files (?)
- Deleted files (D) - easy to miss!
- Renamed files (R)

"You have uncommitted changes. Options:
1. Commit them first (use `/git/commit`)
2. Stash them temporarily
3. Cancel push"

**IMPORTANT:** Deleted files show as `deleted:` or `D` in git status. These MUST be committed too:
```bash
git add -A  # stages all changes including deletions
# or
git add -u  # stages modifications and deletions (not new files)
```

Don't proceed until working directory is clean or changes are stashed.

## Step 3: Show What Will Be Pushed

```bash
git log origin/$(git branch --show-current)..HEAD --pretty=format:"%h %s" 2>/dev/null
```

Display:
- Number of commits to push
- Each commit's message
- Ask: "Push these commits to origin?"

## Step 4: Check if Pull Needed First

If remote has commits you don't have:

```bash
git log HEAD..origin/$(git branch --show-current) --oneline
```

"Remote has new commits. Need to pull and merge first."

Attempt merge:
```bash
git pull origin $(git branch --show-current)
```

### If Merge Conflicts Occur:

Handle conflicts one file at a time:

1. **Show the conflict:**
   - Display conflicted file with markers
   - "YOUR version (local)" vs "THEIR version (remote/teammate)"

2. **Analyze both versions:**
   - What does your code do?
   - What does their code do?
   - Is one better/more complete than the other?

3. **Discuss tradeoffs:**
   - Be objective - if their code is better, say so
   - Explain WHY one might be preferable
   - Don't default to keeping yours

4. **Present options:**
   - Keep YOUR version
   - Keep THEIR version (if it's better)
   - Merge both (if complementary)
   - Custom merge

5. **Get explicit decision** before resolving

6. **After each file:**
   - Show resolved version
   - Get approval
   - `git add <file>`

7. **Complete merge:**
```bash
git commit -m "🔀 merge: integrate changes from origin/$(git branch --show-current)"
```

## Step 5: Push to Remote

```bash
git push origin $(git branch --show-current)
```

If push fails (more remote changes during merge):
- Fetch and merge again
- Never force push

## Step 6: Confirm Success

```bash
git status
git log --oneline -3
```

Show:
- Push successful confirmation
- Branch is up to date with remote
- Recent commit history

## New Branch? Set Upstream

If branch doesn't exist on remote yet:
```bash
git push -u origin $(git branch --show-current)
```

## Safety Rules

- **NEVER use `--force` or `-f`**
- **NEVER push to main/master without explicit confirmation**
- **ALWAYS show what will be pushed before pushing**
- **ALWAYS pull/merge if remote has new commits**
- **Handle conflicts through discussion, not auto-resolution**

## Quick Flow Summary

1. Check for uncommitted changes → handle them
2. Check for unpushed commits → show them
3. Check for remote commits → pull/merge if needed
4. Resolve any conflicts through discussion
5. Push to remote
6. Confirm success

Start by analyzing the current state.
