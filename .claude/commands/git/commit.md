---
description: Create detailed commit with comprehensive message - analyzes all changes, commits locally WITHOUT pushing
allowed-tools: Bash(git:*), Read, Glob, Grep
---

# Git Commit with Detailed Message (Local Only)

Create a well-documented local commit. This does NOT push - use `/git/push` when ready to sync with remote.

## Commit Message Format

```
{emoji} {type}: {short description}

- {detailed change 1}

- {detailed change 2}

- {detailed change 3}
```

### Types & Emojis:
- ✨ `feat:` - New features or functionality
- 🐛 `fix:` - Bug fixes and error corrections
- 📚 `docs:` - Documentation changes
- ♻️ `refactor:` - Code refactoring (no functionality change)
- ⚡ `perf:` - Performance improvements
- ✅ `test:` - Adding or updating tests
- 🔧 `chore:` - Dependencies, build tools, maintenance
- 💄 `style:` - Code formatting, whitespace, linting
- 🚀 `ci:` - CI/CD pipeline changes
- 🔒 `security:` - Security-related fixes
- 🔀 `merge:` - Merge commits
- 🗃️ `db:` - Database schema or migration changes
- 🎨 `ui:` - UI/UX improvements

## Step 1: Analyze Current State

```bash
git status
git diff --stat
git diff --cached --stat
```

**Check for ALL change types:**
- Modified files (M)
- New/untracked files (?)
- Deleted files (D) - these are easy to miss!
- Renamed files (R)

## Step 2: Review All Changes in Detail

For each modified file, examine what changed:
```bash
git diff <file>
git diff --cached <file>
```

**Build a comprehensive understanding of:**
- What was added (new features, files, functions)
- What was modified (logic changes, refactors, fixes)
- What was removed (deleted code, deprecated features)
- The purpose/intent behind these changes

## Step 3: Stage Changes

If files are not staged:
- Ask user: "Stage all changes, or select specific files?"

```bash
git add -A  # all changes including deletions
# or
git add <specific-files>
# or for deleted files specifically:
git add -u  # stages modifications and deletions (not new files)
```

**IMPORTANT: Handle Deleted Files**
- `git status` shows deleted files with `deleted:` or `D` prefix
- Deleted files MUST be staged with `git add -A` or `git add <deleted-file>` or `git rm <file>`
- Always include deleted files in the commit - don't leave them unstaged
- In the commit message, use "Removed" or "Deleted" as the action verb for these files

## Step 4: Create Detailed Commit Message

Based on analysis, create a comprehensive commit message.

**Example:**
```
✨ feat: enhance educational standards integration and update UI components

- Added new scripts for seeding and backfilling educational standards in package.json

- Updated vitest configuration to include server test files for comprehensive testing

- Refactored dashboard and revenue components to improve UI and remove unused imports

- Enhanced hero sections across various components to better reflect the educational focus and improve messaging

- Updated layout components to ensure consistent styling and improved responsiveness

- Integrated new standards alignment features in the backend to support educational content better
```

### Message Rules:
- **Title:** {emoji} {type}: {concise but descriptive summary}
- **Body:** Bullet points explaining EVERY significant change
- **Each bullet:** Start with action verb (Added, Updated, Refactored, Fixed, Removed, Enhanced, Integrated, etc.)
- **Be specific:** Mention components, files, or features affected
- **No vague statements:** List actual changes, not "various improvements"
- **NO mention of Claude, AI, Co-Authored-By, or any automation**

## Step 5: Show Preview and Confirm

```
Here's the commit message I've prepared:

---
{full commit message}
---

This will commit locally only (not push). Ready to commit?
```

## Step 6: Commit Locally

Use heredoc for multiline message:
```bash
git commit -m "$(cat <<'EOF'
✨ feat: your title here

- First change description

- Second change description

- Third change description
EOF
)"
```

## Step 7: Confirm Success

```bash
git log --oneline -1
git status
```

Show:
- The new commit hash and message
- Current status (clean working directory)
- Reminder: "This commit is LOCAL only. Use `/git/push` when ready to push to remote."

## Multiple Logical Changes?

If changes cover multiple unrelated things, ask:
"These changes seem to cover different features/fixes. Would you like to:
1. One combined commit (if changes are related)
2. Separate commits for each logical change (better audit trail)
3. Let me suggest how to split them"

## Safety Notes

- This command NEVER pushes
- You can make multiple local commits before pushing
- Use `/git/undo commit` to undo if needed
- Use `/git/push` when ready to sync with remote

Start by analyzing the current git status and changes.
