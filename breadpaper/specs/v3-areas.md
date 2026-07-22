# BreadPaper V3 — Areas (modular life-domain packages)

**Status:** Scope-locked from design interview (2026-07-21), ready for implementation
**Owner:** Diego · **Date:** 2026-07-21
**Companion docs:** `../VISION.md` (§4.6 Modular life, §5.2 Areas rail, §5.4 Skills, §7.2 Areas-as-packages), `v1-daily-panel.md` (vault model this builds on)

---

## 1. Summary

V3 introduces **Areas** — the modular primitive at the heart of the product. An Area is an installable bundle that helps organize one domain of a person's life (finance, journaling, note-taking, weekly rhythm). Each Area packages **folders, templates, files, skills, and an explainer doc**, and can be **added or removed at any time** without touching the user's own notes.

This version delivers three things:

1. **The Area package format** — a declarative `manifest.toml` bundle, an app-shipped **catalog** of Areas, and the **materialization** flow that writes an Area's editable files into the vault and registers it in the vault config.
2. **An "Areas" section** added to the existing BreadPaper dock panel (below the Timeline navigator). Each enabled Area is an entry; clicking it opens the Area's explainer doc **in viewing (rendered) mode**. Under each Area, its **skills** are listed as clickable entries — a first, basic Skills view.
3. **The Timeline Area** — the first Area, shipped **pre-activated** in the sample vault. It is *additive*: it layers the **Week Review** skill and the **weekly progress dashboard** on top of the already-core Timeline navigator and daily/weekly note creation.

This realizes VISION Milestone 2 ("Areas & Skills framework") in its first concrete, end-to-end slice, while deferring the full skill-contract write-sandbox (still M2) and the page-aware Context right rail (M3).

## 2. Goals & success criteria

**Primary:** Prove the Area primitive end-to-end — that a life-domain can be **packaged, installed into a vault, surfaced in the panel, and removed** — using a single real Area (Timeline) as the proof, while keeping the fork's core diff small (a new section in the existing panel, not a new dock panel).

**Secondary:** A user can see the Areas they have, read what each one is for and what it can do (its explainer + skill list), reach a skill in one click, and add or remove an Area without fear of losing their writing.

**Definition of done:**
1. The sample vault scaffolds with the **Timeline Area pre-installed and enabled**, registered in `.breadpaper/config.toml`, with its files materialized on disk.
2. The BreadPaper left-dock panel shows an **"Areas"** section listing each enabled Area beneath the existing Timeline navigator.
3. Clicking an Area entry opens its explainer doc (e.g. `areas/Timeline.md`) **rendered in viewing mode**, not as a raw editable buffer.
4. Each Area entry expands to list its **skills**; clicking a skill opens the skill file in viewing mode (its plain-language description + body).
5. Clicking the Timeline Area's **dashboard** surface opens the weekly progress page (`_weekly/site/index.html`).
6. An **Add Area** affordance lists catalog Areas not yet installed and, on add, materializes the Area's files and registers it — the new entry appears in the Areas section without a restart.
7. A **Remove Area** action **prompts** the user to choose between *deactivate (keep all files)* and *deactivate and delete the Area's shipped files* — and the destructive path **never** deletes user-authored notes (`daily/`, `weekly/` note content).
8. Removing the Timeline Area does **not** break the Timeline navigator or daily/weekly note creation (they are core, not Area-owned).

## 3. Non-goals (explicitly out of V3)

- **A second real Area** (Finance, Journaling, Team). V3 ships the format + one Area (Timeline). Others are catalog stubs at most.
- **The full skill contract & write sandbox** — enforced, previewable, dry-run read/write scopes (VISION M2). V3 *declares* a skill's `reads`/`writes` in the manifest (so the explainer can show scope and M2 can enforce it) but does **not** sandbox execution.
- **One-click skill *execution*.** Clicking a skill opens it for viewing. Actually *running* it through the user's LLM/agent depends on the BYO-LLM agent rails (VISION M1, not yet built); wiring "Run" to the agent panel is a flagged stretch (§6.4), not a committed deliverable.
- **The page-aware Context right rail** (day-planner / week calendar / finance dashboard per open doc) — VISION M3.
- **A polished Area gallery** — browsing, search, screenshots, versions/updates. V3's "Add Area" is a minimal list of installable catalog Areas.
- **In-app rendering of the dashboard HTML.** Zed has no web view; the dashboard opens in the system browser (§7.4).
- **Backfilling the author's real vault** into the Area layout. V3 targets the fresh sample vault.

## 4. Core concepts

### 4.1 Area
An **Area** is a bundle that organizes one domain of a user's life. It contributes some or all of: **folders** (scaffolded into the vault), **templates**, **static files** (e.g. a dashboard), **skills** (LLM rituals), and a required **explainer doc**. Areas are **opt-in and independent** (VISION principle 6, "Modular life") — a user can run only Timeline, or add more.

> **Naming note (open):** "Areas" is the VISION-committed working name but collides with two things — the PARA `02 - areas/` folder in the author's real vault (areas of *responsibility*, a different meaning) and the "Timeline" navigator section vs. the "Timeline" *Area* (same word, two panel elements). Flagged in §9; not re-decided here.

### 4.2 Catalog vs. installed (the delivery model)
- **Catalog** — a set of Areas that **ships inside the BreadPaper binary**. This is the source the "Add Area" list reads from, and the future gallery.
- **Installed** — when an Area is added to a vault, BreadPaper **materializes** it: copies the Area's editable parts (manifest, skills, explainer doc, dashboard/static files) as **real files into the vault**, scaffolds its folders (create-if-missing, never clobbering), and **registers** it in `.breadpaper/config.toml`.

Materialization is what honors two VISION principles at once: **"Everything is editable"** (once installed, an Area's skills and templates are ordinary files the user or their LLM can open and change) and **"Your files, forever"** (the vault is self-describing and complete on disk; if BreadPaper vanished, the Area's files still open in any editor).

### 4.3 The Areas section
Not a new dock panel — a **new section inside the existing BreadPaper panel**, below the Timeline navigator. This keeps the fork surface tiny (no new `activation_priority`, no new `add_panel_when_ready` wiring) and matches the request ("a new header added to the BreadPaper pane for Areas").

### 4.4 Viewing mode
An Area's explainer doc and its skill files open **rendered** (Zed's markdown preview), not as raw editable buffers — the user is *reading about* the Area, not editing it. Zed has no one-shot "open path as preview" call, so this is a two-step (§7.3).

### 4.5 Skills view (basic)
The Skills view in V3 is the **expandable skill list under each Area entry** in the panel. Each skill shows its name; clicking opens the skill file in viewing mode. This is the minimum inspectable surface for rituals; the rich editable Skills view with enforced scope is deferred (M2).

## 5. The Area package format

### 5.1 Catalog package layout (shipped in the app)
Each catalog Area is a directory of source assets compiled into / bundled with the binary:

```
<area-id>/
  manifest.toml          # the declarative spec (below)
  doc.md                 # explainer, materialized to the vault's areas/<Name>.md
  skills/
    <skill-id>.md        # one file per skill
  assets/                # optional static files (e.g. dashboard html/js seed)
    ...
```

### 5.2 `manifest.toml` schema
The declarative bundle spec — VISION open-question Q2, answered here:

```toml
schema  = 1
id      = "timeline"                 # stable slug; the config registry key
name    = "Timeline"                 # display name in the Areas section
version = 1                          # bumped when the catalog package changes
summary = "Daily & weekly rhythm — weekly review and a progress dashboard."
doc     = "areas/Timeline.md"        # vault-relative path the explainer is materialized to

# Folders/files created in the vault on install. create-if-missing; never clobber.
[[scaffold]]
kind = "dir"
path = "_weekly/site"

[[scaffold]]
kind   = "file"
path   = "_weekly/site/index.html"   # vault-relative destination
source = "assets/index.html"         # path within the catalog package

[[scaffold]]
kind   = "file"
path   = "_weekly/site/data.js"
source = "assets/data.seed.js"       # seed feed; the skill appends to it thereafter

# Skills the Area contributes. Materialized to `file`; listed in the panel + explainer.
[[skill]]
id      = "week-review"
name    = "Week Review"
file    = "skills/timeline/week-review.md"   # vault-relative destination
summary = "Aggregate the week's notes + PRs/MRs, append a review, feed the dashboard."
reads   = ["daily/**", "weekly/**", "mcp:github", "mcp:gitlab"]   # declared; not yet enforced
writes  = ["weekly/<week>.md (append)", "_weekly/site/data.js (append)"]

# Non-skill surfaces the Area exposes in the panel (e.g. a dashboard to open).
[[surface]]
kind = "dashboard"
name = "Weekly Dashboard"
open = "_weekly/site/index.html"     # opened in the system browser (§7.4)
```

Rules:
- All paths are **vault-relative** except `source`/`file`-within-catalog inputs.
- `scaffold` is **idempotent and non-destructive** — reuses `vault.rs`'s existing `write_if_missing` discipline; re-installing or scaffolding over a populated folder is safe.
- `reads`/`writes` on a skill are **declarations only** in V3 — surfaced in the explainer and the (future) skill contract, not enforced.

### 5.3 Installed layout in the vault
After installing Timeline, the vault contains:

```
<vault-root>/
  .breadpaper/
    config.toml                    # + [areas] registry (§5.4)
    areas/
      timeline/
        manifest.toml              # installed copy (records version + what it owns)
  areas/
    Timeline.md                    # explainer doc (opened in viewing mode)
  skills/
    timeline/
      week-review.md               # the Week Review skill
  _weekly/
    site/
      index.html                   # dashboard viewer
      data.js                      # dashboard feed (appended to by the skill)
```

The installed `manifest.toml` is the **provenance record**: it lists exactly the files the Area owns, which drives removal (§6.6) and the modified-since-install check.

### 5.4 Config registry (`.breadpaper/config.toml`)
Extends the existing `VaultConfig` (which today parses `schema`, `[daily]`, `[weekly]`, `[history]`) with an `[[areas.installed]]` array. Display order in the panel = array order.

```toml
schema = 1

[daily]  # …unchanged…
[weekly] # …unchanged…

[[areas.installed]]
id      = "timeline"
enabled = true
version = 1            # the installed package version (for future update detection)
```

Parsing mirrors the existing `*Content` → `resolve()` pattern in `vault.rs` (all fields defaulted, `deny_unknown_fields`), so a vault with no `[[areas.installed]]` simply has an empty Areas section. A registered-but-`enabled = false` Area stays materialized on disk but is hidden from the panel and its skills are not surfaced.

## 6. Behavior specification

### 6.1 The panel gains an Areas section
- The existing BreadPaper panel (`TimelinePanel`, left dock) renders, **below** the Timeline navigator entries, an **"Areas"** header followed by one row per **enabled** installed Area (registry order).
- If no Areas are enabled, the section shows a gentle empty state with the **Add Area** affordance (§6.5).
- When the workspace is not a vault, the whole panel already shows the non-vault state (unchanged from V1); the Areas section does not render.
- **Rename consideration:** with two sections in one panel, the panel's persistent name/title stays "BreadPaper"; the two sections are labeled "Timeline" and "Areas" (see the §9 naming flag for the Timeline/Timeline clash).

### 6.2 Rendering the Areas tree
Each Area row is expandable. Expanded, it shows:
- Its **skills** (`[[skill]]` entries) as clickable rows, each showing the skill `name`.
- Its **surfaces** (`[[surface]]`), e.g. the Timeline Area's "Weekly Dashboard" row.
- The Area's own label click target = **open the explainer doc in viewing mode** (§6.3).

### 6.3 Opening an Area doc in viewing mode
Clicking an Area's name opens its `doc` (e.g. `areas/Timeline.md`) **rendered**:
1. Open the file as an editor via the existing `workspace.open_abs_path(path, OpenOptions { visible: Some(OpenVisible::All), .. }, …)` pattern (as `open_note` already does).
2. Convert/replace it into a markdown **preview** item (§7.3), so the user lands on the rendered doc.

Same mechanism is used to open a **skill file** in viewing mode (§6.4).

### 6.4 The Skills view (basic)
- Clicking a skill row opens the **skill file in viewing mode** — the user reads the skill's plain-language description and body (what it reads, what it writes, what it does).
- **Stretch (flagged, not committed):** a "Run" affordance on a skill row that dispatches the skill to the user's LLM/agent (Zed's agent panel). This depends on the BYO-LLM connection (VISION M1) not yet built; ship the view without it and add Run when the agent rails land. Do **not** fake execution.

### 6.5 Adding an Area
- An **Add Area** affordance (a `[+ Add Area]` row at the bottom of the Areas section) lists **catalog Areas not already installed** in this vault.
- On selecting one, BreadPaper **materializes** it (all on a background thread, mirroring `ensure_note`):
  1. For each `[[scaffold]]`: create dirs / write files **if missing** (never clobber).
  2. Copy the explainer `doc` and each skill `file` into the vault.
  3. Write the installed `manifest.toml` copy under `.breadpaper/areas/<id>/`.
  4. Append `[[areas.installed]]` to `config.toml` with `enabled = true`.
- The Areas section refreshes and shows the new Area **without a restart** (the panel already re-runs `refresh_vault_status` on worktree/entry changes; installation triggers the same refresh).
- If the Area is already registered but disabled, "Add" just flips `enabled = true` (no re-copy) unless files are missing, in which case missing files are re-materialized.

### 6.6 Removing an Area (prompt on removal)
Removal always **asks** (the user's chosen model). A confirmation dialog offers two paths:

| Choice | Effect |
|---|---|
| **Deactivate (keep all files)** | Set `enabled = false` in the registry, drop the Areas-panel entry, stop surfacing its skills. **Nothing on disk is deleted.** Fully reversible via Add. |
| **Deactivate and delete Area files** | Also delete the files the installed `manifest.toml` records as Area-owned — the explainer doc, skill files, and scaffolded static assets (e.g. `_weekly/site/`). |

Guardrails on the destructive path:
- **User notes are never deleted.** Only files the Area *shipped* (recorded in its installed manifest) are candidates. `daily/` and `weekly/` **note files** authored by the user are out of scope by construction — the Timeline Area owns `_weekly/site/*` and `skills/timeline/*`, not the weekly `.md` notes.
- **Modified-since-install files are preserved, not deleted.** Before deleting a shipped file, compare it against the catalog source (hash/content). If it differs (the user or their LLM edited it), **keep it** and report it in the result ("kept 1 modified file: `skills/timeline/week-review.md`"). This protects the "everything is editable" edits from silent loss.
- The dialog **lists exactly what will be deleted** before confirming.
- Either way, the `[[areas.installed]]` entry is removed from (or disabled in) the registry so the panel no longer shows it.

### 6.7 Config registry read/write
- Read: extend `VaultConfig` parsing with the `areas` table; absent → empty.
- Write: install/enable/disable/remove edit `config.toml` in place. Preserve the rest of the file (round-trip the parsed document, or re-serialize the known schema — V3 may re-serialize the full known config; flagged in §9 if comment-preservation matters).

### 6.8 Failure modes
| Condition | Behavior |
|---|---|
| Catalog Area id unknown at Add time | Non-blocking error toast; nothing written. |
| A scaffold destination already exists | Skip it (create-if-missing); not an error. |
| Explainer/skill file missing at open time | Non-blocking toast; offer to re-materialize the Area. |
| `config.toml` write/permission error during install | Roll back: do not leave a half-registered Area; surface a toast (per CLAUDE.md, errors propagate to the UI). |
| Dashboard `open` target missing | Toast; offer to re-materialize. |
| Remove: a shipped file was modified since install | Preserve it, report it (§6.6), continue. |

## 7. The Timeline Area (the first, pre-activated Area)

The Timeline Area is **additive** — the Timeline navigator and daily/weekly create-if-missing stay core and always-on. The Area contributes only the **Week Review skill**, the **weekly dashboard**, and the **explainer doc**.

### 7.1 Manifest (catalog)
As in §5.2 (`id = "timeline"`), with one skill (`week-review`), a `dashboard` surface, and scaffold entries for `_weekly/site/index.html` + `_weekly/site/data.js`.

### 7.2 The Week Review skill (`skills/timeline/week-review.md`)
Ported from the author's working `.claude/commands/week-review.md` (reference vault). Its behavior, verbatim in intent:
- Reviews **Mon–Sun of the prior week**; locates the weekly file in `weekly/` by ISO week (`YYYY-Www.md`, e.g. `2026-W30.md`), computed from the week's Monday — realigned from the reference vault's `_weekly/YYYY_WW_Mon.md` to the layout the Area actually scaffolds.
- Reads the week's daily notes from `daily/` (`YYYY-MM-DD.md`); folds in `# Week Goals` / `# Tentative` / `# Personal`.
- Collects GitHub PRs (`gh search prs …`) and GitLab MRs (`glab api …`, host `gitlab.spimageworks.com`), dedups.
- Groups work by project, sets the `goal` flag, picks 2–3 highlights.
- **Appends** (never overwrites) a `# AI Week Review` section to the weekly `.md`.
- **Appends one week object** to `window.WEEKS` in `_weekly/site/data.js` (schema fixed by the dashboard), then verifies the file still parses.

Its manifest `writes` declaration (`weekly/<week>.md (append)`, `_weekly/site/data.js (append)`) and `reads` (`daily/**`, `weekly/**`, `mcp:github`, `mcp:gitlab`) mirror this exactly — the append-only, augmentation-not-replacement contract (VISION principle 2).

### 7.3 The dashboard
- `_weekly/site/index.html` — the self-computing static viewer (reads `window.WEEKS`, derives stats, sparklines, lingering-project and carried-over-goal warnings). Shipped as an asset; **not** regenerated by BreadPaper.
- `_weekly/site/data.js` — seeded with an empty (or single-example) `window.WEEKS` array on install; the Week Review skill appends to it each week. This is the reference implementation of VISION's "dashboards as an output type" (§7.2) — *structured feed → static HTML that computes its own analytics.*
- Exposed as a `dashboard` surface row in the panel; clicking it opens `index.html` in the system browser (§7.4).

### 7.4 The explainer doc (`areas/Timeline.md`)
The doc opened in viewing mode when the Area is clicked. Content outline (to be drafted, kept short and plain-language):
- **What the Timeline Area is for** — closing the loop on your daily/weekly rhythm; turning a week of notes + code activity into a reviewed, visualized record.
- **What it adds** — the Week Review skill and the weekly progress dashboard (the daily/weekly navigator itself is always-on, not part of this Area).
- **Skills available** — *Week Review*: what it reads (daily notes, GitHub/GitLab), what it writes (appends to the weekly note + the dashboard feed), append-only and safe.
- **The dashboard** — how to open it, what its warnings mean (time-sinks, lingering projects, carried-over goals).
- **How to run a review / where the files live** — pointers to `skills/timeline/week-review.md` and `_weekly/site/`.

## 8. Implementation notes (for engineering)

- **Keep it a section, not a panel.** Extend `TimelinePanel`'s render to add the Areas section; reuse its existing `refresh_vault_status` / subscription plumbing. No new `activation_priority` (Timeline holds 4; 0–3 and 5–7 are upstream), no new `add_panel_when_ready` wiring. This keeps the core diff small and rebase-friendly (VISION §7.1 mandate).
- **Isolate Area logic in the `breadpaper` crate.** Add an `areas` module (`crates/breadpaper/src/areas.rs`) owning: the manifest schema (serde/`toml`), the catalog registry (app-shipped source), materialize/uninstall, and the modified-since-install diff. The panel calls into it; the delta against Zed's own files stays minimal.
- **Config parsing** reuses the `vault.rs` `*Content` → `resolve()` pattern; add `AreasConfigContent` + a resolved `AreasConfig { installed: Vec<InstalledArea> }`.
- **Viewing mode is a two-step** — there is **no** one-shot "open path as preview" API. To open rendered: `workspace.open_abs_path(...)` to get the markdown `Editor`, then build a preview via the core `markdown_preview` crate (`MarkdownPreviewView::create_markdown_view` / `open_preview_in_pane`, `MarkdownPreviewMode::Default`), or dispatch the existing `zed_actions::preview::markdown::OpenPreview` action against the just-opened editor. `markdown_preview` is already a core crate — no new dependency. **Add a small helper** `open_abs_path_as_preview(...)` in the breadpaper crate and use it for both Area docs and skill files. _(Confirm the exact preview entry point on review — §9.)_
- **Materialization runs on a background thread** (mirror `ensure_note`'s `cx.background_spawn`); it is blocking file I/O. Errors propagate to a UI toast (CLAUDE.md: never silently discard; surface to the user).
- **Extend `scaffold_vault`** (`vault.rs`) so the sample vault installs the Timeline Area at creation time: write the installed manifest, the explainer doc, the skill file, the dashboard assets, and the `[[areas.installed]]` registry entry. The Timeline catalog assets become bundled resources of the `breadpaper` crate.
- **No `let _ =`, no `unwrap()`** on the fallible file/config operations (CLAUDE.md); propagate with `?` and surface failures.
- **Catalog assets** (index.html, data.seed.js, skill markdown, explainer): decide how they ship — `include_str!`/`include_dir!` bundled into the crate vs. an on-disk resources dir. `include_*` keeps the binary self-contained; flagged in §9.

## 9. Open assumptions to confirm on review

1. **Naming collision.** "Timeline" names both the navigator section and the first Area; "Areas" collides with PARA `02 - areas/`. Options: rename the Area to "Daily & Weekly" (per VISION M1) while keeping the nav section "Timeline," or keep both "Timeline" and disambiguate by placement. Recommend renaming the *Area* to avoid the in-panel clash.
2. **Preview entry point.** Which `markdown_preview` call to standardize on for "open as viewing mode" (independent preview tab vs. replace-in-place vs. to-the-side). Recommend an independent, focused preview tab (`MarkdownPreviewMode::Default`).
3. **Catalog asset packaging** — `include_dir!` bundled vs. on-disk resources (§8).
4. **`config.toml` round-tripping** — is re-serializing the known schema acceptable (may drop user comments in the config), or must we preserve the raw document?
5. **Add Area surface** — a simple in-panel list vs. a modal picker for the (currently one-item) catalog.
6. **`skills/` location** — top-level `skills/<area>/` (proposed) vs. under `.breadpaper/` vs. the reference vault's `.claude/commands/`. Proposed top-level so skills are visible and editable in the file tree.
7. **Dashboard open target** — always the system browser (§7.4), acceptable given no in-app web view?
8. **Explainer doc filename** — `areas/Timeline.md` (title-cased, human) vs. `.breadpaper/areas/timeline/doc.md` (hidden). Proposed the visible, human-named path.

## 10. Decision log (from design interview, 2026-07-21)

- **Package model:** app-shipped **catalog → materialize into the vault** on install; installed Areas are real, editable files + a `config.toml` registry entry. Honors "everything is editable" + "your files, forever."
- **Removal:** **prompt on removal** — choose *deactivate (keep all files)* or *deactivate + delete Area-shipped files*; user notes are never deletable; modified-since-install files are preserved and reported.
- **Timeline Area scope:** **additive** — the navigator + daily/weekly creation stay core; the Area adds only the Week Review skill + the weekly dashboard + the explainer doc. Removing it can't break navigation.
- **Skills:** include a **basic Skills view now** — skills listed under each Area, clicking opens the skill file in viewing mode. Rich editable Skills view + enforced read/write scope stay M2. Actual skill *execution* ("Run") deferred to the BYO-LLM rails.
- **Surface:** Areas are a **new section in the existing BreadPaper panel**, not a new dock panel (small fork surface, no new activation priority).
- **Viewing mode:** Area docs + skill files open **rendered** (markdown preview), via a two-step open-then-preview helper.
