# Wrap Today

Close out **today's** daily note. Read what you planned and did, pull the day's commits, scan the last few days for multi-day context, offer unfinished tasks to the backlog, then append an AI review with suggestions to today's note. Append-only — it never rewrites what you wrote.

**Reads:** today's daily note, the prior few daily notes, the backlog file, git commits, GitHub (`gh`) / GitLab (`glab`) as available.
**Writes (append-only):** today's daily note, the backlog file.

> **Note paths and filenames are vault-configured.** Read `.breadpaper/config.toml` first: `[daily]` / `[weekly]` set each note kind's `dir` and moment-style `filename` format, and `[backlog]` sets `file`. This skill's examples use the defaults (`daily/YYYY-MM-DD.md`, `weekly/GGGG-[W]WW.md`, `backlog.md`); when the config — or the vault's existing notes — use different names, follow those silently. That's configuration, not a doc mismatch worth reporting.

## 1. Locate today's note

1. Compute **today's** date and resolve today's note path from the config (default: `daily/YYYY-MM-DD.md`).
2. Look for that file. If it doesn't exist yet, create it — the Timeline convention is create-if-missing.

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

## 5. Offer unfinished tasks to the backlog

The vault keeps a holding pen — the configured backlog file, `backlog.md` by default — with the sections `## Soon`, `## Someday`, and `## Completed` (the Backlog panel in the bottom dock renders it). Unfinished tasks shouldn't die in today's note:

1. List the unfinished (`- [ ]`) tasks you found in the note.
2. Ask the user **one** question: move **all** of them to the backlog, **none** (they stay only in the note), or **some** (the user picks which)? The default destination is **Soon**; the user can say "someday" for any task. If there are no unfinished tasks, skip this step silently.
3. **Deduplicate before appending:** skip any chosen task whose text already appears in `backlog.md` — compare whitespace-insensitively and ignore any ` ✅ YYYY-MM-DD` completion suffix — in **any** section, `## Completed` included. Report skips as "already in backlog: …".
4. Append the remaining tasks as `- [ ] <task text>` at the end of the matching section. Create `backlog.md` or a missing section heading if needed — never rewrite, reorder, or delete anything already in the file.

Never move a task without the user's answer, and never edit the tasks inside the daily note itself — the note stays append-only.

## 6. Append the review

Append (never overwrite) a `# Daily Closure` section at the end of today's note:

```
# Daily Closure

## Done
- What actually got finished today

## Open (carried forward)
- [ ] Unchecked task worth continuing tomorrow → backlog (Soon)
- [ ] Task the user chose to leave in the note

## Commits
- repo@abc123 — commit message

## Suggestions
- One or two concrete, actionable nudges (don't pad)
```

Keep it short and factual. Base "carried forward" on the unchecked tasks plus anything the recent-days scan shows lingering. Suffix each task that moved in step 5 with `→ backlog (Soon)` (or `(Someday)`) so the note records where it went; tasks that were skipped as duplicates get `→ already in backlog`.

## Output

Show the user the review and confirm it was appended to today's note, including which tasks (if any) moved to the backlog.
