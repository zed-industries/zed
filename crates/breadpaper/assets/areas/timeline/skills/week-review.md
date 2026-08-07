# Week Review

Review the previous week's daily notes, GitHub activity, and GitLab (SPI) activity, then produce a summary organized by project, write it to the weekly markdown file, and append a structured entry to the dashboard data (`weekly/site/data.js`).

**Reads:** the daily and weekly notes, the backlog file, GitHub (`gh`), GitLab (`glab`).
**Writes (append-only):** the weekly `.md` file, `weekly/site/data.js`, the backlog file.

> **Note paths and filenames are vault-configured.** Read `.breadpaper/config.toml` first: `[daily]` / `[weekly]` set each note kind's `dir` and moment-style `filename` format, and `[backlog]` sets `file`. This skill's examples use the defaults (`daily/YYYY-MM-DD.md`, `weekly/GGGG-[W]WW.md`, `backlog.md`); when the config — or the vault's existing notes — use different names, follow those silently. That's configuration, not a doc mismatch worth reporting.

## 1. Determine the week and locate the weekly file

1. The review covers **Monday–Sunday of the week before today**. Compute those two dates (YYYY-MM-DD).
2. Resolve the weekly file from the config (`[weekly]` `dir` + `filename`), formatting the pattern with the week's Monday. The default names weeks by **ISO week** — `GGGG-[W]WW`, i.e. ISO week-year, a literal `W`, and the zero-padded ISO week number — so the week starting Mon 2026-07-20 is `weekly/2026-W30.md` by default. If the file doesn't exist yet, create it.
3. The **week id** used everywhere below is the weekly filename without `.md` (e.g. `2026-W30`).

## 2. Read the daily notes

1. Find all daily notes within the range — match them against the configured daily dir and filename format.
2. Read each one. Extract tasks and activities from day-planner sections, conversation notes, and anything else relevant.
3. If a weekly note already exists, read it and fold in its `# Week Goals`, `# Tentative`, and `# Personal` sections (these become the goals in the dashboard) plus any context.

## 3. Collect GitHub PRs (`gh`)

Fetch everything you touched in the range:
- Created: `gh search prs --author=@me --created=YYYY-MM-DD..YYYY-MM-DD`
- Reviewed: `gh search prs --reviewed-by=@me --updated=YYYY-MM-DD..YYYY-MM-DD`
- Merged: `gh search prs --author=@me --merged=YYYY-MM-DD..YYYY-MM-DD`

Deduplicate. Each PR has a repo, number, title, and status (open/merged; reviewed if you reviewed it).

## 4. Collect GitLab MRs (`glab`) — SPI internal

Your SPI work lives on `gitlab.spimageworks.com` and is a large share of real output (deploys, releases, fixes) that GitHub does not see. One-time setup (already done on this machine): `glab` is installed and authenticated as **dtavares** (user id **163**). If a call fails with auth errors, re-run `glab auth login --hostname gitlab.spimageworks.com` (needs a `read_api` PAT).

Always prefix calls with the host:

```bash
export GITLAB_HOST=gitlab.spimageworks.com
```

For a single week the counts are small, so one page (`per_page=100`) is enough — no pagination needed. (Only for multi-week backfills does `glab api --paginate` matter, and it concatenates one JSON array per page; merge with `jq -s 'add'`.)

**Authored MRs** (→ `created`), created in the window:
```bash
glab api "merge_requests?scope=all&author_username=dtavares&created_after=YYYY-MM-DDT00:00:00Z&created_before=YYYY-MM-DDT00:00:00Z&per_page=100"
```
Map each: `ref` = `<project-path-tail>!<iid>` (e.g. `shottree3!265`), `title`, `status` = `merged`/`open`/`closed` (GitLab `merged`→`merged`, `opened`→`open`, `closed`→`closed`). Mark `draft: true` when the title starts with `Draft:`/`[Draft]`/`WIP:`.

**Reviewed MRs** (→ `reviewed`): do **not** use `reviewer_username` — it returns MRs merely touched in the window and is noisy. Use real review actions from your events feed:
```bash
glab api "users/163/events?after=YYYY-MM-DD&before=YYYY-MM-DD&per_page=100"
```
Take events with `action_name: "approved"` and `target_type: "MergeRequest"` (optionally also `commented on` a `MergeRequest`/`DiffNote` on someone else's MR). Resolve the project path via `glab api "projects/<project_id>"` (field `path`) to build the `ref`. GitLab reviews are typically sparse — that is expected; your review volume is mostly on GitHub.

Note: `action_name: "accepted"` means you merged an MR (counts toward `merged` on MRs you authored); `"opened"` means you authored it. Use these to sanity-check the authored list.

## 5. Organize the content

1. **Group tasks by project.** Infer short, consistent project names (Scheduler, Cuebot, ST3, Qlite, BOM, VFO, CueWeb, Team / People, Personal / Finance, etc.). Only include what was actually worked on — no padding.
2. **Set the `goal: true` flag** on any project that served a `# Week Goals` item that week. This is the one judgment call the dashboard cannot make itself — it drives the "time sink" warning (share of work outside your goals). Leave it off for projects that were not goals. Never flag `Personal` or `Team` as lingering-exempt yourself; the page handles that.
3. **Pick 2–3 highlights** — the week's most notable accomplishments, in your own words. For an unfinished in-progress week, leave highlights empty.

## 6. Offer unfinished tasks to the backlog

The vault keeps a holding pen — the configured backlog file, `backlog.md` by default — with the sections `## Soon`, `## Someday`, and `## Completed` (the Backlog panel in the bottom dock renders it). After organizing the week you know what stayed unfinished — the unchecked (`- [ ]`) items from the weekly note's goal sections plus tasks the daily notes show lingering across the week:

1. List those unfinished tasks.
2. Ask the user **one** question: move **all** of them to the backlog, **none**, or **some** (the user picks which)? The default destination is **Soon**; the user can say "someday" for any task. If nothing is unfinished, skip this step silently.
3. **Deduplicate before appending:** skip any chosen task whose text already appears in `backlog.md` — compare whitespace-insensitively and ignore any ` ✅ YYYY-MM-DD` completion suffix — in **any** section, `## Completed` included. Report skips as "already in backlog: …".
4. Append the remaining tasks as `- [ ] <task text>` at the end of the matching section. Create `backlog.md` or a missing section heading if needed — never rewrite, reorder, or delete anything already in the file.

Record the moves in the review you write next (e.g. suffix a goal with `→ backlog (Soon)`). Never move a task without the user's answer, and never edit the weekly or daily notes' existing content.

## 7. Write the markdown review

Append (never overwrite) a `# AI Week Review` section at the end of the weekly file, in this format:

```
## Week Review: YYYY-MM-DD to YYYY-MM-DD

### Project Name
- Task or activity completed
- Another task

### Pull & Merge Requests
- **OpenCue#2425** - [scheduler] Add end-to-end stress tests (created, open) [GitHub]
- **shottree3!265** - Optimize st3-metadata search query (created, merged) [GitLab]
- **spi-centos!18** - Add en_US locale to SPI 9.5 image (reviewed) [GitLab]
```

Keep it short and factual. Tag each PR/MR with `[GitHub]` or `[GitLab]`. No commentary unless asked.

## 8. Append to the dashboard (`weekly/site/data.js`)

The dashboard reads `window.WEEKS` (an array, newest last). Append **one new week object** immediately before the closing `];` of the `window.WEEKS` array. Never rewrite existing entries — new weeks carry their MRs inline with `src` tags.

Schema (match this exactly):

```js
{
  id: "2026-W29",                    // week id = weekly filename stem
  week: 29,                          // the WW number
  label: "Week 29",
  range: "Jul 13 – Jul 19, 2026",
  status: "reviewed",                // "reviewed", or "in-progress" if the week isn't over
  goals:     [ { text: "…", done: true } ],   // from # Week Goals
  tentative: [ { text: "…", done: false } ],  // from # Tentative (or [])
  personal:  [ { text: "…", done: false } ],  // from # Personal (or [])
  highlights: [ "…", "…" ],          // 2–3, or [] for an in-progress week
  projects: [
    { name: "Scheduler", goal: true, tasks: [ "task one", "task two" ] },
    { name: "Personal / Finance", tasks: [ "…" ] }   // omit `goal` when not a week goal
  ],
  prs: {
    created: [
      { ref: "OpenCue#2425", title: "[scheduler] Add end-to-end stress tests", status: "open",   src: "github" },
      { ref: "shottree3!265", title: "Optimize st3-metadata search query",     status: "merged", src: "gitlab" },
      { ref: "shottree3!263", title: "Draft: Upgrade to Java 21", status: "open", src: "gitlab", draft: true }
    ],
    reviewed: [
      { ref: "OpenCue#2410", title: "[cueweb/docs] Add Allocations page", src: "github" },
      { ref: "spi-centos!18", title: "Add en_US locale to SPI 9.5 image",  src: "gitlab" }
    ]
  }
}
```

Rules:
- Put **every** PR/MR in `prs.created` or `prs.reviewed` with an explicit `src: "github"` or `src: "gitlab"`. `status` applies to `created` entries only.
- The dashboard computes stats, sparklines, and warnings from this data. It infers **lingering projects** and **carried-over goals** across weeks automatically — you only need accurate `projects` (with `goal` flags), `tasks`, `goals`, and `prs`. Keep project names stable across weeks so lingering detection works (the page canonicalizes common variants, but consistency helps).
- After editing, verify the file still parses: `node -e "global.window={};require('./weekly/site/data.js');console.log(window.WEEKS.length,'weeks')"`.

## Output

Show the user the markdown review (§7). Mention which tasks (if any) moved to the backlog, that the dashboard entry was appended, and that they can open `weekly/site/index.html` to view it.
