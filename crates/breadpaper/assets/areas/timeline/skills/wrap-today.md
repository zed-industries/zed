# Wrap Today

Close out **today's** daily note. Read what you planned and did, pull the day's commits, scan the last few days for multi-day context, then append an AI review with suggestions to today's note. Append-only — it never rewrites what you wrote.

**Reads:** today's `daily/YYYY-MM-DD.md`, the prior few daily notes, git commits, GitHub (`gh`) / GitLab (`glab`) as available.
**Writes (append-only):** today's daily note.

## 1. Locate today's note

1. Compute **today's** date (YYYY-MM-DD). Daily notes live in `daily/` named `YYYY-MM-DD.md`.
2. Look for `daily/<today>.md`. If it doesn't exist yet, create it — the Timeline convention is create-if-missing.

## 2. Read the day's tasks and activities

1. Parse the day-planner and task sections. Separate **checked** (`- [x]`) from **unchecked** (`- [ ]`) tasks.
2. Note anything meaningful captured during the day: decisions, conversation notes, blockers.

## 3. Pull the day's commits

Fetch what you shipped today. Use whichever sources are wired up:

- GitHub: `gh search commits --author=@me --author-date=YYYY-MM-DD` (or `gh search prs --author=@me --updated=YYYY-MM-DD..YYYY-MM-DD`).
- Local repos: `git log --author="$(git config user.email)" --since="YYYY-MM-DD 00:00" --until="YYYY-MM-DD 23:59" --oneline`.
- GitLab (SPI): `export GITLAB_HOST=gitlab.spimageworks.com` then query the events feed as the Week Review skill does.

Deduplicate. Keep repo, short SHA / ref, and message.

## 4. Scan recent days for context

Read the previous 2–3 daily notes so the review isn't myopic — catch carried-over tasks, ongoing threads, and multi-day projects. Note anything that has lingered.

## 5. Append the review

Append (never overwrite) a `# Daily Closure` section at the end of today's note:

```
# Daily Closure

## Done
- What actually got finished today

## Open (carried forward)
- [ ] Unchecked task worth continuing tomorrow

## Commits
- repo@abc123 — commit message

## Suggestions
- One or two concrete, actionable nudges (don't pad)
```

Keep it short and factual. Base "carried forward" on the unchecked tasks plus anything the recent-days scan shows lingering.

## Output

Show the user the review and confirm it was appended to today's note.
