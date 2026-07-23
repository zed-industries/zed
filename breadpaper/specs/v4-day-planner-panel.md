# BreadPaper V4 — Day Planner Context Panel

**Status:** Scope-locked from design interview (2026-07-22), ready for implementation
**Owner:** Diego · **Date:** 2026-07-22
**Companion docs:** `../VISION.md` (§5.3 Context right rail, §12 Milestone 3 "Page-aware Context right rail"), `v1-daily-panel.md` (vault + note model this builds on), `v3-areas.md` (panel/section patterns, viewing-mode helper)

---

## 1. Summary

V4 introduces the first **page-aware Context panel**: a right-dock GPUI panel that reads the **daily note open in the active editor**, parses its checklist for timed tasks, and renders them on a **vertical, calendar-style day grid** — hours down the left gutter, tasks placed as blocks at their time and height-scaled to their duration, overlapping tasks laid out in side-by-side columns (Google-Calendar style). Tasks **without** a time appear as chips in an **unscheduled strip** across the top.

The panel is **read-only** and **derived** — it never writes to the note. Its one interaction is **reveal-on-click**: clicking a block (or an unscheduled chip) selects and scrolls to that task's line in the markdown editor and paints a transient row highlight, so the visual schedule and the source text stay tightly coupled. As the user edits the note, the grid re-parses and re-renders live.

This realizes VISION Milestone 3's first slice — the day-planner Context rail — while deferring the week-calendar and finance-dashboard context views, and deferring any write-back (drag-to-reschedule) to a later increment.

## 2. Goals & success criteria

**Primary:** Prove the **page-aware Context rail** end-to-end — a right-dock panel that reflects the *content* of the active document (not just the vault), turning a plain Markdown checklist into a legible visual day schedule, with a tight click-to-source coupling. This is the first panel whose contents change with the *open file*, not the workspace.

**Secondary:** Make the daily note's plan glanceable. A user should see, at a glance, how their day is blocked out, what's unscheduled, what's done, and where two tasks collide — and jump from any block to its exact line in one click.

**Definition of done:**
1. With a **daily note active** in the editor and the right dock open, the Day Planner panel renders an hour grid with the note's timed tasks placed at the correct time and duration.
2. Tasks written as `- [ ] 08:00 – 11:00 Label` (range) and `- [ ] 09:30 Label` (start-only) both render as blocks; time-less checkbox tasks render as **unscheduled chips** in the top strip.
3. **Completed** tasks (`- [x]`) render struck-through and muted; **incomplete** tasks render in the accent fill.
4. Two tasks whose times overlap render **side by side** in separate columns, neither obscuring the other.
5. **Clicking** any block or chip **selects + scrolls to** that task's line in the active editor and paints a transient row highlight; nothing in the note is modified.
6. Editing a task's time/text/checkbox in the editor updates the grid **live** (within a short debounce), without a manual refresh.
7. When the active item is **not a daily note** (or not a vault), the panel shows a gentle hint state, not a crash or blank.
8. The panel is a native right-dock GPUI `Panel` with a **unique `activation_priority`**, a small isolated diff in the `breadpaper` crate.

## 3. Non-goals (explicitly out of V4)

- **Any write-back.** No drag-to-reschedule, resize, drag-to-create, checkbox-toggle-from-the-grid, or any edit to the note. V4 is a read-only projection. (Drag-to-edit is the natural V5 follow-up; §10.)
- **Non-daily context views.** The week calendar (on weekly notes) and finance dashboard (on finance files) are the same "page-aware rail" mechanism but different renderers — out of scope here. V4 renders **daily notes only**.
- **External calendar data.** The Obsidian reference shows "(work) Busy" blocks and an "Active clocks" list sourced from calendar/clock plugins. V4 parses **only the Markdown note**. Google-Calendar/MCP overlays are a later Context increment.
- **Recurring tasks, multi-day / all-day spans, timezones.** A block's start and end are wall-clock times on the note's own day. A task with no end (start-only) gets a default duration; a task crossing midnight is clamped to the day (§5.5).
- **Reordering / editing the unscheduled strip.**
- **A settings UI** for the grid. Configuration is via `config.toml` (§6); defaults must be good enough to never require it.
- **Rendering when multiple notes are visible in a split.** V4 tracks the single **active** editor item.

## 4. Core concepts

### 4.1 Page-aware Context panel
Unlike the Timeline panel (which reflects the *vault*), this panel reflects the **active editor item**. It subscribes to `workspace::Event::ActiveItemChanged`; whenever the active item resolves to a **daily note** in the current vault, the panel parses that note's buffer and renders its schedule. Otherwise it shows a hint state (§7.4). This "the right rail follows the open document" behavior is the reusable primitive VISION §5.3 describes; V4 builds it once, for daily notes.

### 4.2 Timed task, unscheduled task
The panel's model of the note is a list of **items**, each derived from one Markdown checkbox line (§5). An item is either:
- **Timed** — its label begins with a parseable time (a range or a start-only time). Rendered as a **block** on the grid.
- **Unscheduled** — a checkbox task with no leading time. Rendered as a **chip** in the top strip.

Every item carries: its **checkbox state** (done/not-done), its **display label** (the text after the time), and its **source location** (the buffer row of the line) for reveal-on-click.

### 4.3 Derived, never authoritative
The grid is a *view* of the text. The Markdown is the single source of truth (VISION principle 1). The panel holds no state the note doesn't; re-parsing the buffer must fully reconstruct the view. This is what keeps "read-only + click-to-reveal" honest and makes live updates trivial (re-parse on edit).

## 5. Parsing model (be precise — this is the contract)

### 5.1 What is a candidate line
A candidate is a Markdown **task list item**: a line matching, after leading whitespace,

```
[-*+] \s+ \[( |x|X)\] \s+ <text>
```

i.e. a bullet marker (`-`, `*`, or `+`), a `[ ]` / `[x]` / `[X]` checkbox, then the task text. The checkbox state is **done** for `x`/`X`, **not-done** for a space. Plain bullets (no `[ ]`), headings, and paragraphs are **not** candidates (per the locked decision "Checkbox tasks w/ time prefix"). Nested/indented tasks are candidates (indentation is ignored for parsing; it does not imply sub-scheduling in V4).

### 5.2 Parse scope (heading if present, else whole file)
1. Look for the configured **planner heading** (`[day_planner].heading`, default `"Day planner"`) — matched against any ATX heading (`#`…`######`) whose trimmed text equals the configured string, **case-insensitively**.
2. **If found:** parse candidate lines from the line *after* that heading up to (but not including) the next heading of **equal or higher level** (same or fewer `#`). This is the "section" under the planner heading.
3. **If not found:** parse candidate lines across the **whole note**.

Rationale: the sample `templates/daily.md` already ships a `## Day planner` section (`v1-daily-panel.md` §5.3), so notes created by BreadPaper get scoped parsing for free, while hand-made notes still work.

### 5.3 Extracting the time token
For each candidate, examine the task text for a **leading** time token (leading = at the very start of the text, after the checkbox). Accepted forms:

| Form | Example | Meaning |
|---|---|---|
| **Range** | `08:00 – 11:00 Label`, `8:00-11:00 Label`, `08:00 to 11:00 Label` | explicit start + end |
| **Start-only** | `09:30 Standup` | start; end = start + default duration (§5.5) |
| **None** | `Workout` | unscheduled |

Time grammar:
- A time is `H:MM` or `HH:MM`, 24-hour: hours `0`–`23`, minutes `00`–`59`. (12-hour `am/pm` is **not** parsed in V4 — flagged §9.)
- A range separator is one of: `–` (en-dash), `—` (em-dash), `-` (hyphen), or the word `to`, with **optional surrounding spaces**. So `08:00–11:00`, `08:00 – 11:00`, `08:00 - 11:00`, `08:00 to 11:00` all parse.
- After the time token there must be at least one whitespace char, then the **label** is the remaining text (trimmed). A label may be empty (block with no title).
- If the text starts with something time-like but malformed (e.g. `25:99`, `8:` ), it is treated as **not a time** → the item is unscheduled and the raw text is its label. Never error, never drop the task.

### 5.4 The display label
The block/chip label is the task text **with the leading time token removed** (for timed items) or the full task text (for unscheduled). Trailing metadata the user may keep (tags, links) is left in the label verbatim — the panel does not strip `#tags` or `[[links]]`; it renders them as plain text (no link resolution in V4).

### 5.5 Duration & clamping rules
- **Range** with `end > start`: duration = `end − start`.
- **Range** with `end ≤ start` (typo, or crosses midnight): clamp `end` to `start + default_duration`, and surface nothing to the user (silent, forgiving). *(Alternative — treat `end < start` as crossing midnight and clamp to `24:00` — flagged §9; V4 takes the simpler forgiving clamp.)*
- **Start-only**: duration = `[day_planner].default_duration_minutes` (default **30**).
- A block is never rendered shorter than a **minimum visual height** (§7.2) regardless of duration, so a 5-minute task stays clickable and legible.
- A block's end past `day_end` extends the grid (§7.1), never overflows it.

### 5.6 Source location
Each item records the **0-based buffer row** of its source line (the checkbox line). This is what reveal-on-click (§8) turns into an editor anchor range. Parsing works off the `MultiBufferSnapshot` of the active editor's buffer, so rows are always current for the buffer version parsed.

## 6. Configuration (`config.toml`)

Extends the per-vault `VaultConfig` (v1 §5.2, v3 §5.4) with an optional `[day_planner]` table. All fields default; a vault with no `[day_planner]` table gets the defaults below, so **no migration is required**.

```toml
[day_planner]
heading              = "Day planner"   # planner section heading (case-insensitive); empty = always whole-file
day_start            = "06:00"         # top of the grid; auto-expands earlier to fit tasks
day_end              = "24:00"         # bottom of the grid; auto-expands later to fit tasks
default_duration_minutes = 30          # duration for start-only tasks
show_now_indicator   = true            # draw a "now" line when viewing today's note
```

Parsing reuses the existing `*Content` → `resolve()` / `deny_unknown_fields` pattern in `vault.rs`. `day_start`/`day_end` accept `HH:MM` (24-hour; `24:00` = end of day); invalid values fall back to the default with a logged warning (never panic — CLAUDE.md).

## 7. Visual design (the grid) — be specific

### 7.1 Layout regions (top to bottom)
```
┌─────────────────────────────────────────┐
│  Header:  Mon, Jul 20   ·  [date nav?]   │   ← §7.5, minimal
├─────────────────────────────────────────┤
│  Unscheduled:  [▢ Workout] [✔ Planning]  │   ← §7.3 chip strip, wraps
├──────┬──────────────────────────────────┤
│  06  │                                   │
│  07  │                                   │   ← §7.2 hour grid
│  08  │ ▓▓ 08:00–11:00 Evaluate…          │      (scrollable)
│  09  │ ▓▓                                │
│ ─────│───────────  now ─────────────     │   ← §7.4 now line
│  10  │ ▓▓                                │
│  11  │ ░░ 11:00 Add vnp3 charts…         │
│  …   │                                   │
└──────┴──────────────────────────────────┘
```

- **Left gutter:** fixed-width (≈ 44px) column of hour labels (`6`, `7`, … `23`), right-aligned, baseline-aligned to each hour's gridline. Muted text color (`cx.theme().colors().text_muted`).
- **Grid body:** vertically scrollable; horizontal gridlines at each hour (`border_color`/`border_variant`), and a fainter half-hour line optional (default off).
- The unscheduled strip and header are **sticky** (do not scroll with the grid body).

### 7.2 Block geometry & the placement math
Let `hour_height` = `[day_planner]` derived pixels per hour (default **48px**; may scale with panel width but constant vertically). Define `grid_start_min` = min(`day_start`, earliest task start) rounded **down** to the hour; `grid_end_min` = max(`day_end`, latest task end) rounded **up** to the hour.

For a block with `start_min`, `end_min` (minutes since midnight):
```
top_px    = (start_min - grid_start_min) / 60 * hour_height
height_px = max(MIN_BLOCK_PX, (end_min - start_min) / 60 * hour_height)   // MIN_BLOCK_PX ≈ 18
```
The block is an absolutely-positioned rounded rectangle within the grid body's column area. It renders:
- A small **time caption** (`08:00 – 11:00`) at the top, in a smaller/muted type.
- The **label**, wrapping, clipped to the block height with ellipsis when it overflows.
- **State styling:** not-done → filled with the accent (`cx.theme().colors().editor_highlighted_line_background` or an accent like the reference's purple; pick a theme token, §9); done → desaturated/dimmed fill **and** strikethrough on the label (matches the editor's own rendering of `[x]`, and the reference screenshot).

### 7.3 Unscheduled strip
- A horizontal, wrapping row of **chips** across the top, one per time-less task, in source order.
- Each chip: a checkbox glyph reflecting done/not-done, then the label (truncated). Done chips are struck-through + muted.
- If there are no unscheduled tasks, the strip collapses (zero height), it does not show an empty box.
- Clicking a chip behaves exactly like clicking a block (§8).

### 7.4 The "now" indicator
- When the active note is **today's** daily note (its resolved date equals the local calendar date) and `show_now_indicator = true`, draw a thin horizontal accent line across the grid body at the current time's `top_px`, with a small dot at the gutter edge.
- Not drawn for past/future daily notes (a note dated last Tuesday should not show "now"). The line updates on a coarse timer (once/minute is plenty) — use a GPUI executor timer, not `smol::Timer` (CLAUDE.md / repo `.rules`).

### 7.5 Header
Minimal: the note's date, formatted (e.g. `Mon, Jul 20`), derived from the resolved note date. **No** in-panel date navigation in V4 (that's the Timeline panel's job); flagged as a possible add in §9. Keep the header to one line so the grid gets the space.

### 7.6 Overlap layout (columns) — the algorithm
Google-Calendar-style side-by-side placement:
1. Sort blocks by `start_min`, then by `end_min`.
2. Group into **clusters**: a maximal run of blocks where each overlaps at least one other in the run (transitive). Concretely: iterate sorted blocks, keep a running cluster; a block joins the current cluster if its `start_min < cluster_max_end`; otherwise it starts a new cluster. Track `cluster_max_end = max(end_min)` seen in the cluster.
3. Within a cluster, assign each block the **leftmost column** whose last block ends at or before this block's start (greedy interval-coloring). The cluster's **column count** = the number of columns used.
4. Render: within the cluster's horizontal span (the full grid-body width), each block gets `width = span_width / column_count` and `x = column_index * width`, with a small gutter between columns. Non-overlapping blocks (cluster of one) take the full width.

Two blocks with identical times get two columns (each half width), never stacked/hidden. This guarantees Definition-of-Done #4.

## 8. Interaction: reveal-on-click (the one interaction)

Clicking a block or chip reveals its source line in the active editor. Implementation uses the confirmed editor API surface:

1. Get the editor: `workspace.active_item(cx).and_then(|item| item.downcast::<Editor>())`. If the active item is no longer that note's editor, no-op (the panel will already have re-evaluated on `ActiveItemChanged`).
2. `editor.update_in(cx, |editor, window, cx| { … })`:
   - `let snapshot = editor.buffer().read(cx).snapshot(cx);`
   - Build an anchor range spanning the task's line from the stored **row**: clip `Point::new(row, 0)` and the line end with `snapshot.clip_point(…, Bias::Left)`, then `let start = snapshot.anchor_before(start_point); let end = snapshot.anchor_after(end_point);` (anchors track edits — the go-to-line pattern at `crates/go_to_line/src/go_to_line.rs:189`).
   - **Move + scroll:** `editor.change_selections(SelectionEffects::scroll(Autoscroll::center()).nav_history(true), window, cx, |s| s.select_anchor_ranges([start..start]))` so the cursor lands on the line and Cmd+`-` (back) works.
   - **Transient highlight:** define a private marker type `DayPlannerHighlight`; call `editor.clear_row_highlights::<DayPlannerHighlight>()` then `editor.highlight_rows::<DayPlannerHighlight>(start..end, |cx| cx.theme().colors().editor_highlighted_line_background, RowHighlightOptions { autoscroll: true, ..Default::default() }, cx)`. The marker keeps our highlight isolated and clearable without touching the fixed `HighlightKey` enum.
   - Focus the editor: `editor.focus_handle(cx).focus(window, cx)`.
3. **Selected state in the panel:** the clicked block gets a selected outline; only one item is selected at a time. Selection is panel-local UI state (not derived from the note).

The highlight is transient: it is cleared and repainted on the next click, and cleared when the active item changes.

## 9. Behavior specification

### 9.1 Activation & data flow
- The panel subscribes to `workspace::Event::ActiveItemChanged`. On change, it resolves the active item's path (mirror `active_item_path` in `timeline_panel.rs:607`) and asks the vault whether that path is a **daily note** (`Vault::note_path(NoteKind::Daily, date)` round-trip, or a reverse `daily_note_date(path)` helper on `Vault`). If yes, capture a `WeakEntity<Editor>` and its buffer, parse, render. If no, hint state (§9.4).
- On the **active editor's buffer** emitting an edit event, **re-parse** (debounced ≈150ms via a GPUI executor timer) and `cx.notify()`. Store the parsed model against the buffer so redundant re-parses are skipped.
- Parsing is cheap (a single pass over the planner section) and runs on the **foreground**; no background thread needed. (If a note is pathologically large this can move to `background_spawn`; flagged §9, not required for V4.)

### 9.2 Rendering states
| State | Panel shows |
|---|---|
| Active = today's daily note, has timed tasks | Full grid + now line + unscheduled strip. |
| Active = a daily note, no timed/unscheduled tasks | Empty grid with a hint: *"No tasks yet. Add `- [ ] 09:00 – 10:00 Task` under your Day planner heading."* |
| Active = a daily note, only unscheduled tasks | Grid (empty of blocks) + the chip strip. |
| Active = non-daily note, or non-note, or not a vault | Muted hint: *"Open a daily note to see its schedule."* No grid. |

### 9.3 Live update guarantees
Any edit to the note that changes a task's time, label, checkbox, or membership updates the grid within the debounce window. Deleting the planner heading falls back to whole-file parsing (§5.2) automatically on the next parse. Toggling `[ ]`↔`[x]` in the editor re-styles the block (done/strikethrough) live.

### 9.4 Failure modes
| Condition | Behavior |
|---|---|
| Active editor's buffer unreadable / entity dropped | Hint state; no crash. `WeakEntity` upgrade failure is a no-op. |
| A candidate line has a malformed time | Treated as unscheduled with raw label (§5.3); never dropped, never an error. |
| `day_start`/`day_end` malformed in config | Fall back to defaults, log a warning. |
| Click target's row no longer exists (note shrank) | Clip to buffer end; select the nearest valid position; never panic (use clipped points, not raw indexing — CLAUDE.md). |
| Overlapping tasks exceed panel width (many columns) | Columns shrink to a minimum width; block labels truncate; horizontal legibility degrades gracefully (no clipping of blocks off-panel). |

## 10. Implementation notes (for engineering)

- **New crate module, new right-dock panel.** Add `crates/breadpaper/src/day_planner_panel.rs` (the `Panel`) and a pure `crates/breadpaper/src/day_plan.rs` (parsing + geometry, no GPUI) so the parse/layout logic is unit-testable without a window. Register the panel next to `TimelinePanel` in `breadpaper.rs`'s `init`/`add_panel_when_ready` wiring. Keep the diff isolated to the `breadpaper` crate plus the one registration site (VISION §7.1 small-fork mandate).
- **Split pure logic from rendering.** `day_plan.rs` exposes: `parse_day_plan(text: &str, config: &DayPlannerConfig) -> DayPlan` returning `Vec<PlanItem>` with `{ row, state, label, timing: Timed{start,end} | Unscheduled }`, and a `layout(&DayPlan, grid bounds) -> Vec<PlacedBlock>` implementing §7.2/§7.6. Both are ordinary functions over strings/ints — test the range/start-only/malformed/overlap/clamp cases directly (mirror the `notes.rs` test style).
- **`activation_priority` must be unique.** Timeline holds `4`; upstream uses `0–3` and `5–7`. Use the next free value (**`8`** proposed) and confirm nothing else claims it (this repo has hit priority collisions before — see the build-traps memory). Right dock; `position_is_valid` → `Right | Left`.
- **Reuse the active-item plumbing.** `active_item_path` and the `ActiveItemChanged` subscription already exist in `timeline_panel.rs`; the new panel needs the same shape plus a **buffer edit subscription** on the active editor (`cx.subscribe` to the editor entity's events; re-parse on edit). Store `WeakEntity<Editor>` for reveal-on-click.
- **Reverse date resolution.** Add `Vault::daily_note_date(&self, path: &Path) -> Option<NaiveDate>` (inverse of `note_path`) so the panel can both (a) decide "is this a daily note?" and (b) know the note's date for the header and now-line. Parse the filename with the configured `daily.filename` format.
- **Editor API (confirmed).** Reveal-on-click uses `change_selections` + `SelectionEffects::scroll(Autoscroll::center()).nav_history(true)` + `select_anchor_ranges`, and `highlight_rows::<DayPlannerHighlight>` with `RowHighlightOptions { autoscroll: true, .. }` (call sites: `crates/go_to_line/src/go_to_line.rs:198`, `:288`; `crates/journal/src/journal.rs:162`). Anchors via `snapshot.anchor_before/after` on the `MultiBufferSnapshot`. No new dependency — `editor` is already a `breadpaper` dep.
- **Rendering.** The grid body is an absolutely-positioned layer: hour gridlines + blocks positioned with `top`/`height`/`left`/`width` in `px` from §7.2/§7.6. Use `div().absolute()` children within a `relative()` grid container; make blocks `.on_click(cx.listener(...))`. The unscheduled strip is a wrapping `flex` row. Use `ui` primitives (`Label`, theme colors) for consistency with the rest of the fork.
- **Timers.** The now-line refresh uses `cx.background_executor().timer(...)` (or a `Context`-driven repaint schedule), **not** `smol::Timer` (CLAUDE.md).
- **No `unwrap()` / no `let _ =`** on fallible ops (config read, path parse, entity upgrade); propagate with `?` or `.log_err()`; surface user-facing failures as they occur (CLAUDE.md).

## 11. Open assumptions to confirm on review

1. **`activation_priority` value** — `8` proposed; confirm it's free across all registered panels.
2. **Accent color token** — which theme color the block fill uses (an accent vs. `editor_highlighted_line_background`); done-state muting token. The reference uses a purple; pick the closest theme token so it tracks light/dark themes.
3. **`end ≤ start` handling** — forgiving clamp to `start + default_duration` (§5.5) vs. interpreting as crossing midnight. Recommend the clamp for V4.
4. **12-hour times** (`9:00am`) — parse them too, or 24-hour only? V4 proposes 24-hour only (matches the reference and the template).
5. **Half-hour gridlines** and default `hour_height` (48px) — tune on first render.
6. **Header date navigation** — keep the header static (recommended) vs. add prev/next-day arrows that drive the Timeline's open action.
7. **Parse thread** — foreground parse is fine for realistic notes; confirm we don't need `background_spawn` for very large notes.
8. **Split editors** — V4 tracks only the single active item; confirm that's acceptable (a note open in two panes reveals into whichever is active).

## 12. Decision log (from design interview, 2026-07-22)

- **Interactivity:** **read-only + reveal-on-click.** The panel never writes to the note; clicking a block/chip selects, scrolls to, and highlights the source line. Drag-to-reschedule/create is explicitly deferred to a later version.
- **Schedulable syntax:** **checkbox tasks with a leading time.** Only `- [ ] `/`- [x] ` items count; a leading `HH:MM – HH:MM` (range) or `HH:MM` (start-only) makes a task **timed**, otherwise it's **unscheduled**. Plain bullets/paragraphs/headings are ignored.
- **Parse scope:** **heading if present, else whole file.** Prefer the configured `Day planner` heading's section; fall back to scanning the whole note when it's absent.
- **Placement:** **new right-dock panel** — the first page-aware Context rail (VISION §5.3 / Milestone 3), reflecting the active document rather than the vault. Its own unique `activation_priority`.
- **Layout:** Google-Calendar-style — hour grid, duration-scaled blocks, side-by-side columns for overlaps, an unscheduled chip strip on top, completed tasks struck-through, an optional "now" line for today.
