# Set Up Timeline

> This file is written for the AI agent BreadPaper launches when the Daily &
> Weekly Area is added. It's a plain skill like any other — open it, edit it,
> rerun it whenever you like (the Areas rail's "Set up with AI" action).

You are helping the user set up the **Daily & Weekly (Timeline)** Area of
their BreadPaper vault. Your job: find where their daily notes live today,
migrate them into this vault's conventions, and report honestly on what you
did.

## Ground rules

1. **Interview before acting.** Ask, wait for answers, then propose — never
   assume the user's setup.
2. **Copy, never move or delete.** Files outside this vault are read and
   copied only; the source stays untouched.
3. **Confirm before each write batch.** Show what you're about to create or
   copy and get a yes first.
4. **Never rewrite user-authored content.** If a target file already exists
   in the vault, flag the collision and skip it — don't merge, don't
   overwrite.
5. Work inside the vault you were launched in (your current directory).

## The vault's conventions

- Daily notes: `daily/YYYY-MM-DD.md` by default (e.g. `daily/2026-07-23.md`),
  created from `templates/daily.md`.
- Weekly notes: `weekly/GGGG-Www.md` ISO week by default (e.g.
  `weekly/2026-W30.md`), created from `templates/weekly.md`.
- Skills (rituals like this one): `skills/timeline/*.md`.
- The Area's explainer doc: `areas/Timeline.md`.
- The exact directories and filename formats are configurable in
  `.breadpaper/config.toml` — read it first and honor any overrides.

## The ritual

### 1. Interview

Ask where their daily notes live today. The usual answers:

- an **Obsidian vault** or plain folder of markdown files on disk,
- **Notion**, **Asana**, or another app,
- **nowhere — fresh start**.

If fresh start: skip to step 4.

### 2. Local files (first-class)

1. Ask for the folder's path. Survey it: filename patterns, date formats,
   folder structure, frontmatter conventions. Count what matches.
2. Propose a mapping to the vault's configured daily filename format (default
   `daily/YYYY-MM-DD.md`; likewise weekly where weekly notes are detectable).
   Show a few concrete before → after examples and the total counts. If the
   user would rather keep their existing naming, offer the reverse instead:
   encode their convention in `.breadpaper/config.toml` (`[daily]` /
   `[weekly]`: `dir` + moment-style `filename`) and copy the files unrenamed.
3. On approval, **copy** the matched files in. Never move; the source folder
   stays exactly as it was.
4. If a destination file already exists in this vault, don't touch it — add
   it to the collision list for the report.
5. Only convert content (frontmatter, date-titles) when the conversion is
   obvious and the user approved it explicitly; otherwise copy verbatim.

### 3. Apps and APIs (best-effort)

If their notes live in Notion, Asana, or similar: this is unscripted. Use
whatever tools, CLIs, or MCP servers you already have. If you can export and
migrate, follow the same propose → approve → copy discipline. If you can't,
say so plainly and leave the user short, honest instructions for a manual
export they can run later — don't fake progress.

### 4. Report

Summarize in the conversation:

- what was migrated (counts, date range),
- what was skipped and why (unmatched patterns, collisions),
- anything flagged for manual follow-up.

### 5. Complete

1. Create the file `.breadpaper/state/onboarded/timeline` (create the parent
   directories if needed). Write a short version of the report as its body —
   one paragraph is plenty. This is how BreadPaper knows setup finished.
2. Tell the user: **"BreadPaper will open your Timeline tour"** — the app
   watches for that file and opens `areas/Timeline.md` with what this Area
   can do. Point out they can rerun any of it later: `/wrap-today`,
   `/wrap-yesterday`, `/week-review`, or this setup itself.
