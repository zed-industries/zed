---
description: Interactive stash management - save, list, apply, pop, or drop stashes with clear descriptions
allowed-tools: Bash(git:*)
argument-hint: [save|list|pop|apply|drop]
---

# Git Stash Management

Interactive stash management with clear descriptions and safe operations.

## Available Operations

Based on the argument provided ($ARGUMENTS), perform the appropriate action:

### `save` or no argument - Stash Current Changes

```bash
git status
git diff --stat
```

If there are changes to stash:
1. Show what will be stashed
2. Ask for a descriptive message: "What should we call this stash?"
3. Create the stash:
```bash
git stash push -m "{user's description}"
```

### `list` - Show All Stashes

```bash
git stash list
```

For each stash, also show:
```bash
git stash show stash@{N} --stat
```

Present as a numbered list with:
- Stash index
- Description/message
- Files affected
- When it was created

### `pop` - Apply and Remove Latest Stash

```bash
git stash list
```

Show the latest stash and ask: "Apply and remove this stash? (stash@{0})"

If confirmed:
```bash
git stash pop
```

If conflicts occur, help resolve them interactively.

### `apply` - Apply Without Removing

```bash
git stash list
```

Ask which stash to apply (default: latest)

```bash
git stash apply stash@{N}
```

Stash remains in the list for potential reuse.

### `drop` - Delete a Stash

```bash
git stash list
```

**WARNING:** Show the stash contents first:
```bash
git stash show stash@{N} -p
```

Ask: "Are you sure you want to permanently delete this stash? This cannot be undone."

If confirmed:
```bash
git stash drop stash@{N}
```

## Safety Rules

- **NEVER drop a stash without showing its contents first**
- **NEVER drop a stash without explicit confirmation**
- **ALWAYS show what's in a stash before operations**
- **If pop fails due to conflicts, the stash is preserved**

## Interactive Mode (no arguments)

If no argument provided, show current stash list and ask:
"What would you like to do?
1. Save current changes to a new stash
2. Pop the latest stash
3. Apply a specific stash
4. Drop a stash
5. Just view the stash list"

Start by checking if there are arguments, then proceed accordingly.
