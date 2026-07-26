# BreadPaper V6 — Backlog (Soon / Someday holding pen)

**Status:** Draft from feature request (2026-07-24) — open assumptions pending review
**Owner:** Diego · **Date:** 2026-07-24
**Companion docs:** `../VISION.md` (§5.1 Timeline, §5.4 Skills, §12 Milestone 1), `v1-daily-panel.md` (vault/config model), `v3-areas.md` (Timeline Area, skill manifest), `v4-day-planner-panel.md` (checklist-parsing panel precedent), `v5-agent-and-onboarding.md` (Run rails the wrap skills ride on)

---

## 1. Summary

V6 gives unfinished work a place to go instead of dying silently in yesterday's note. Today the wrap skills *report* carried-over tasks ("Open (carried forward)") but nothing holds them; each day starts from a blank template and lingering tasks only survive if the user re-types them. The **Backlog** closes that loop with three pieces:

1. **The backlog file** — a single `backlog.md` in the vault with three sections: **Soon**, **Someday**, and **Completed**. Soon/Someday hold open checklist tasks; Completed is the dated audit trail of what got done.
2. **The Backlog panel** — a new native GPUI `Panel` in the **bottom dock** rendering the file as a live checklist, grouped by Soon and Someday. Task text is editable inline. Checking a task off does two things atomically: appends it as done to **today's daily note** and moves it to the backlog's Completed section stamped with today's date.
3. **Wrap-skill capture** — the Timeline Area's **Wrap Today**, **Wrap Yesterday**, and **Week Review** skills gain a step: after identifying unfinished tasks, the agent asks the user whether to move **all, none, or some** of them to the backlog (or simply ignore them), then appends the chosen ones.

The split follows the established doctrine: the file convention and the panel are **core** (like daily/weekly note creation and the Timeline panel), while the capture behavior ships inside the Timeline Area's materialized skills. Removing the Area never breaks the panel.

## 2. Goals & success criteria

**Primary:** Unfinished tasks flow from daily/weekly notes into one visible, editable holding pen — and from the holding pen back into a daily note the moment they're done — without the user ever hand-migrating a task between files again.

**Secondary:** The panel proves the "editable checklist over a plain file" pattern (a step beyond the read-only Day Planner): panel edits are surgical, the file remains the single source of truth, and hand edits / agent edits show up live.

**Definition of done:**
1. The sample vault scaffolds `backlog.md` with the three sections (`## Soon`, `## Someday`, `## Completed`), and `[backlog]` config resolves its path.
2. A **Backlog panel** appears in the bottom dock (toggle + keybinding), rendering open tasks grouped under **Soon** and **Someday** headers, in file order.
3. Clicking a task's text edits it **inline**; committing (Enter / blur) writes a surgical edit to `backlog.md` that changes only that line. Escape cancels.
4. Checking a task appends it as a done item (`- [x] …`) to **today's** daily note (created from template if missing — the create-if-missing invariant) and moves the task line, **with any indented children**, to the Completed section suffixed with today's date. If the daily-note write fails, the backlog is left untouched and an error toast surfaces.
5. Edits to `backlog.md` from anywhere else (editor buffer, wrap skill, external tool) re-render the panel live.
6. All rewrites of `backlog.md` are **round-trip safe**: content the panel doesn't model (prose between sections, unknown sections, task children, blank-line style) is preserved verbatim.
7. The wrap skills, when they find unfinished tasks, present them and ask the user to move **all / none / some** to the backlog; chosen tasks are appended to **Soon** (or **Someday** if the user says so), **deduplicated** against tasks already in the backlog, and the move is reported in the appended closure/review section.
8. Non-vault workspace → the panel shows the standard gentle non-vault state; missing `backlog.md` in a vault → empty sections with a hint, file created on first write.

## 3. Non-goals (explicitly out of V6)

- **No task metadata** — no priorities, due dates, tags, or ordering UI. Order = file order; reorder by editing the file. (Drag-to-reorder is a possible later nicety, not V6.)
- **No scheduling from the backlog** — dragging a Soon task into the Day Planner grid / today's note as a *planned* (unchecked) task is deferred. V6's only backlog→note flow is mark-done.
- **No reverse sync** — checking a task inside a daily note does not touch the backlog; the backlog's Completed section is fed only by the panel's mark-done and (optionally) the skills.
- **No Completed management** — no pruning, archiving-by-month, or un-complete affordance in the panel. Completed is edited by hand if needed.
- **No automatic capture** — tasks enter the backlog only via the wrap skills' confirmed prompt or hand editing. BreadPaper itself never sweeps notes for unchecked tasks.
- **No multi-file backlogs** and no per-Area backlogs — one file per vault.
- **Weekly note as a mark-done target** — done items always land in **today's daily note**, even when the completed task originally came from a weekly wrap.

## 4. Core concepts

### 4.1 The backlog file is the source of truth
Like every BreadPaper surface, the panel is a **view over a plain Markdown file** — no hidden index, no database ("your files, forever"). Anything the panel can do, a user or their agent can do by editing `backlog.md` directly, and the panel follows.

### 4.2 Soon vs. Someday
Two-tier triage, on purpose kept to two: **Soon** = intend to do in the coming days; **Someday** = worth keeping, no commitment. The wrap skills default to Soon (the task was, after all, planned for *today*), with Someday available per task at the user's word.

### 4.3 Completed is an audit trail
Marked-done tasks are never deleted — they move to `## Completed` with a completion date. Combined with the copy appended to the daily note, a finished task is recorded both **where the work happened** (the day's note, feeding the wrap/review skills and dashboard) and **where it was tracked** (the backlog's history).

### 4.4 Core panel, Area capture
The file convention, config, and panel are **core** — always available in any vault, like daily/weekly creation. The behavior that *feeds* the backlog lives in the Timeline Area's skill files, which are materialized, user-editable Markdown (v3). This keeps the Area additive (v3 doctrine): removing it leaves the backlog panel fully functional; it just stops being fed automatically.

## 5. The backlog file

### 5.1 Format
```markdown
# Backlog

## Soon

- [ ] Renew passport
- [ ] Fix the day-planner overlap bug
  - notes and sub-items travel with their parent

## Someday

- [ ] Learn woodworking

## Completed

- [x] Book dentist appointment ✅ 2026-07-23
```

Rules:
- Sections are matched by heading text (`Soon`, `Someday`, `Completed`), case-insensitive, at any heading level; first match wins.
- A task is a top-level `- [ ]` / `- [x]` list item inside a section. Indented lines beneath it (sub-items, notes) are its **children** and travel with it when it moves.
- Completed tasks carry a completion-date suffix: ` ✅ YYYY-MM-DD` (Obsidian-Tasks-compatible; format flagged in §9).
- Anything else — prose, unknown sections, HTML comments — is preserved verbatim on every rewrite. Missing sections are appended when first needed (create-if-missing, never clobber).

### 5.2 Config (`.breadpaper/config.toml`)
Extends `VaultConfig` with the established `*Content` → `resolve()` pattern:
```toml
[backlog]
file = "backlog.md"    # vault-relative; default shown
```

### 5.3 Scaffolding
`scaffold_vault` writes a starter `backlog.md` (the three empty sections plus a one-line orienting comment) via the existing `write_if_missing` discipline. Existing vaults get the file lazily on first panel write or skill append.

## 6. Behavior specification

### 6.1 The panel
- A native GPUI `Panel` registered in the **bottom dock** (coexisting with the terminal), toggled via dock icon + keybinding, unique `activation_priority` (**10** — 0–9 are taken by upstream, Timeline, Day Planner, and Agent).
- Renders two groups, **Soon** and **Someday**, each listing its open tasks in file order: a checkbox + the task text (children are not rendered in V6 — a child-count affordance is an open nicety, §9).
- The **Completed** section renders as a collapsed group header (count only); expanding it lists dated completed tasks read-only. _(Confirm — §9.)_
- Empty backlog → gentle empty state ("Nothing in the backlog. Wrap skills can move unfinished tasks here.").
- Non-vault workspace → standard non-vault state (as the other panels); no backlog is read.

### 6.2 Inline text editing
- Clicking a task's text swaps the label for a **single-line inline editor** seeded with the current text.
- **Enter / focus loss** commits: the task's line in `backlog.md` is rewritten in place (checkbox marker, indentation, children untouched). **Escape** cancels.
- Committing an **empty** string is a no-op (revert) — deleting a task is a file edit, not a panel gesture, in V6.
- A context-menu **Reveal in backlog.md** opens the file at that line (the Day Planner's reveal pattern) for anything beyond a text tweak — deleting, reordering, moving between sections, editing children.

### 6.3 Marking a task done
Checking a Soon/Someday task runs, in order:
1. **Append to today's note.** Resolve today's daily note (existing `daily` config + create-from-template-if-missing, exactly as the Timeline panel's Today entry). Append `- [x] <task text>` at the end of the note's day-planner/task section (resolved via the existing `[day_planner]` heading detection); if no such section exists, append at end of file. Never overwrite anything.
2. **Move to Completed.** In `backlog.md`, remove the task line + its children from its section and append them at the end of `## Completed` (chronological order), the task line suffixed with ` ✅ <today>`.

Ordering is deliberate: if step 1 fails (permissions, template error), step 2 never runs — the backlog is not left claiming a task completed that no note records. Failure surfaces as an error toast (errors always propagate to the UI). The two writes are performed back-to-back on one background task; the panel disables the row's checkbox while in flight.

### 6.4 Live updates
The panel re-parses on any change to `backlog.md` — buffer edits when the file is open in an editor, and worktree/fs events otherwise (the Day Planner's live re-parse plumbing, pointed at one absolute path). Panel-initiated writes go **through the open buffer when one exists** (so an open editor tab and the panel never fight), falling back to fs writes.

### 6.5 Failure modes
| Condition | Behavior |
|---|---|
| `backlog.md` missing in a vault | Empty sections + hint; created on first write. |
| A section heading missing | Treated as empty; heading appended when first written to. |
| Daily-note write fails during mark-done | Toast; backlog untouched (§6.3 ordering). |
| Backlog write fails after note append | Toast stating the note was updated but the backlog wasn't; re-check is safe (the panel re-renders from the file, still showing the task open). |
| Task line changed externally mid-edit | Commit re-locates the task by identity (section + text at edit start); if gone, drop the edit with a toast rather than guessing. |
| Duplicate task text within a section | Allowed in the file; panel renders both; internal addressing is by line, not text. |

## 7. Wrap-skill capture (Timeline Area changes)

### 7.1 The interaction contract
Each of **Wrap Today**, **Wrap Yesterday**, and **Week Review** gains a step between "identify unfinished tasks" and "append the review":

1. List the unfinished (`- [ ]`) tasks found in the note under review.
2. Ask the user one question: move **all** of them to the backlog, **none** (ignore — they stay only in the note), or **some** (user picks which). Default destination **Soon**; the user can say "someday" for any task.
3. Append the chosen tasks to the matching section of `backlog.md` (create file/sections if missing, never clobber).
4. Report the outcome in the appended closure/review section — the existing `## Open (carried forward)` block gains a note of which tasks moved, e.g. `→ backlog (Soon)`.

The question is asked **once per wrap**, in the agent conversation (the V5 terminal rails) — no new app UI is involved.

### 7.2 Dedup
Before appending, the skill checks the backlog for an existing task with the same text (whitespace-insensitive, ignoring any `✅` suffix) in **any** section, Completed included. Matches are skipped and reported ("already in backlog: …"). This is also what keeps repeated wraps of the same day from re-offering moved tasks into duplicates — the notes themselves are never rewritten (append-only principle; a `- [>]` forwarded-marker alternative is flagged in §9).

### 7.3 Package changes
- The three skill files (`skills/timeline/wrap-today.md`, `wrap-yesterday.md`, `week-review.md`) gain the capture step; their manifest `writes` declarations gain `backlog.md (append)`.
- The Timeline catalog `manifest.toml` bumps `version = 2`.
- The Area explainer doc (`areas/Timeline.md`) gains a short "Backlog" passage (what Soon/Someday mean, how wraps feed it, where the panel lives).

### 7.4 Existing vaults
There is no Area-update flow yet (deferred since v3), so vaults that installed Timeline v1 keep their materialized v1 skills; fresh scaffolds get v2. Existing users can hand-edit their skills or remove/re-add the Area (modified-file preservation applies). Building update detection on the recorded `version` is future work, not V6.

## 8. Implementation notes (for engineering)

- **Crate layout:** `crates/breadpaper/src/backlog.rs` (file model: parse into sections/tasks with byte/line spans, surgical edit + move operations, dedup helper) and `backlog_panel.rs` (the `Panel`). Panel registration mirrors the existing three; `activation_priority() = 10` with the usual uniqueness comment. Remember the panel traps: no workspace double-lease in `Panel::load` (use `cx.defer`), and fs-event effect tasks must not be cancel-by-replace.
- **Parse once, edit by span.** Parse `backlog.md` into `{sections: [{heading_line, tasks: [{line_range /* incl. children */, text, checked, date}]}], raw}` and perform edits as span replacements on the raw text — this is what makes round-trip preservation (§5.1) cheap and testable. Reuse the checkbox-line conventions from `day_plan.rs` rather than a new task grammar.
- **Buffer-first writes** (§6.4): if `backlog.md` has an open buffer, apply edits through it (project buffer APIs) so undo history and the editor view stay coherent; otherwise background fs write, mirroring `ensure_note`'s `cx.background_spawn` pattern. No `unwrap()`, no `let _ =`; every failure reaches a toast.
- **Mark-done reuses note creation.** Step 1 of §6.3 is the existing ensure-today's-note path (`notes.rs`) plus an append; do not re-implement template substitution.
- **Config:** `BacklogConfigContent { file } → BacklogConfig` in `vault.rs`, defaulted, `deny_unknown_fields`, added to `VaultConfigContent` like `day_planner`/`agent`.
- **Inline editor:** a per-row single-line `Editor` swapped in on click (Zed's rename-in-project-panel interaction is the precedent to crib).
- **Testing:** unit — parser round-trips (unknown sections, prose, children, missing headings), surgical rename, move-to-completed with children + date suffix, dedup normalization; integration — mark-done against a temp vault (note created, ordering-on-failure invariant), live re-render on external file change. Skill-side behavior is prompt content; dogfood it live (Diego drives the TUI, per the usual handoff).

## 9. Open assumptions to confirm on review

1. **File location** — `backlog.md` at vault root (visible, one file). Alternatives: `_backlog.md`, or under `daily/`. Recommend root.
2. **Completed in the panel** — collapsed read-only group with count (§6.1) vs. not rendered at all. Recommend the collapsed group (visible payoff for checking things off).
3. **Date suffix format** — ` ✅ YYYY-MM-DD` (Obsidian-Tasks-compatible) vs. plainer ` (done YYYY-MM-DD)`. Recommend `✅`.
4. **Daily-note insert point** — end of the day-planner section vs. end of file vs. a dedicated `## From backlog` heading. Recommend the day-planner section (the Day Planner panel then shows it struck through, for free).
5. **Add-item affordance** — a small `[+ Add]` row per section creating a new inline-edited task. Cheap and natural, but not in the request; recommend including.
6. **Move Soon ↔ Someday** — context-menu action on a row, or file-edit only in V6? Recommend the context-menu action (a two-line span move the model already supports).
7. **Forwarded marker** — should wrap skills also flip moved tasks to `- [>]` in the source note (visible provenance, at the cost of the skills' strict append-only promise), or rely on dedup alone (§7.2)? Recommend dedup-only.
8. **Panel keybinding & icon** — to be picked alongside the existing panel bindings.

## 10. Decision log (from feature request, 2026-07-24)

- **Backlog file** with three sections — **Soon**, **Someday**, **Completed**; completed tasks are **dated** with the day they were marked done.
- **Panel in the bottom dock**, rendered as a checklist mapping `backlog.md`, grouped by Soon and Someday; **task text is editable** in the panel.
- **Mark-done from the panel** = append the task as done to **today's daily note** + move it to the backlog's Completed section.
- **Wrap skills feed the backlog**: daily and weekly wraps offer unfinished tasks for the backlog, and the agent must **confirm — all, none, or some** (or simply ignore) — with the user before moving anything.
- Backlog file + panel are **core**; capture ships in the Timeline Area's skills (consistent with the additive-Area doctrine).
