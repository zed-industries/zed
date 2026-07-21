# BreadPaper V1 — Daily Navigation Panel

**Status:** Scope-locked, ready for implementation
**Owner:** Diego · **Date:** 2026-07-20
**Companion doc:** `../VISION.md`

---

## 1. Summary

The first BreadPaper feature and the **feasibility gate for the whole fork**: a custom left-dock panel (built as a native GPUI `Panel` in the Zed fork) that lists **Today** and **Yesterday** as one-click links to daily notes. Clicking an entry opens that day's note, creating it from a template if it doesn't yet exist. On first run, BreadPaper scaffolds and opens a sample vault so the feature works end-to-end on a clean machine.

This is deliberately narrow, but it exercises the three things the entire product depends on: **can we add a custom panel to Zed's dock, register it, and maintain it against upstream without pain?** If yes, the pane-heavy vision is real. If the GPUI panel integration proves unmaintainable, this is the moment we reconsider the fork.

## 2. Goals & success criteria

**Primary (the go/no-go):** Prove that a custom left-dock GPUI panel can be added to the Zed fork and **kept maintainable** — a small, legible diff against `upstream/main` that survives a rebase. Success is judged on the *final, real* implementation, not throwaway code ("code is cheap; maintainability is the question").

**Secondary (the feature must actually be good):** Opening today's note is a single, obvious click every morning — replacing the current "open Obsidian just to trigger the daily-note plugin" workaround.

**Definition of done:**
1. Fresh launch with no configured vault → sample vault scaffolded at the default location and opened.
2. A left-dock panel appears (coexisting with the file tree) showing **Today** and **Yesterday**.
3. Clicking **Today** opens today's note, creating it from the template (with token substitution) if missing.
4. Clicking **Yesterday** does the same for yesterday's date.
5. Existing notes are opened, **never overwritten**.
6. Opening a non-vault folder shows a graceful "not a vault" state, not a crash or blank.
7. A written **maintainability assessment**: how invasive was the core diff, and does it rebase cleanly? This is the actual deliverable of the spike.

## 3. Non-goals (explicitly out of V1)

- **This Week / Last Week** entries (weekly notes — different template & filename scheme; the next increment).
- **Invisible git / checkpointing** — creating a note is not versioned yet.
- **Right-hand context pane**, Areas pane, skills, LLM rituals, onboarding flow.
- **Multi-vault registry / switcher UI.** Switching vaults in V1 = open another folder that contains a `.breadpaper/` marker (normal workspace open). The config is shaped so a registry can be added later.
- **Existence indicators**, auto-open-on-launch, "tomorrow" navigation.
- Backfilling / migrating the author's real vault. V1 targets a **fresh default layout**, not the organically-grown existing one.

## 4. Core concepts

### 4.1 Vault
A **vault** is a folder that BreadPaper recognizes as a life-workspace. It is Zed's workspace/worktree concept under a new name. BreadPaper is **multi-vault** in principle, but **~90% of users have exactly one**, so V1 optimizes for the single-default-vault path and treats extra vaults as an expert case (just open another vault folder).

### 4.2 Vault marker & per-vault config
A folder is a vault iff it contains a **`.breadpaper/` directory** with a **`config.toml`**. The marker is what makes "import a vault from a GitHub repo" and multi-vault coherent — each vault self-describes. The panel activates only when the open workspace is a recognized vault, and reads all paths/formats **from that vault's config** (not a global setting).

`.breadpaper/` is a hidden directory by design, with room to grow (future home for invisible-git state, caches, per-vault Area packages). It remains user-editable via the file tree.

## 5. The vault: default layout & config

### 5.1 Default directory layout (what the sample vault contains)
```
<vault-root>/
  .breadpaper/
    config.toml          # marker + per-vault config
  daily/                 # daily notes, flat (no year nesting in V1)
  templates/
    daily.md             # daily-note template
  welcome.md             # opened on first run to orient the user
```

### 5.2 `config.toml` schema (V1)
```toml
schema = 1

[daily]
dir      = "daily"           # daily notes dir, relative to vault root
filename = "YYYY-MM-DD"      # moment-style date format; ".md" is appended
template = "templates/daily.md"
```
- `filename` and template tokens share **one moment-style date vocabulary** (`YYYY`, `MM`, `DD`, `dddd`, `MMMM`, `D`, …) for consistency.
- Everything is a default with a sane value; the file is meant to be edited.

### 5.3 Sample `templates/daily.md`
Mirrors the author's real daily structure, with a dated heading via substitution:
```markdown
# {{date:dddd, MMMM D, YYYY}}

## Journal

## Day planner

## Personal
```

## 6. Behavior specification

### 6.1 Vault discovery
- The panel operates on the **active workspace root**.
- If `<workspace-root>/.breadpaper/config.toml` exists → the workspace is a vault; load its config.
- Otherwise → non-vault state (§6.6).

### 6.2 First run
- On launch, if no vault is configured/known, BreadPaper **scaffolds the sample vault** at the default location `~/BreadPaper/` (creating `.breadpaper/config.toml`, `daily/`, `templates/daily.md`, `welcome.md`) and **opens it as the workspace**.
- `welcome.md` is opened in the editor so the user lands on something oriented, not a blank pane.
- Detection is allowed to be simple in V1 (e.g. "does the default vault exist yet?"); a real known-vaults registry is deferred.

### 6.3 The panel
- A native GPUI `Panel` registered in the **left dock**, toggled via a dock icon + keybinding.
- **Coexists** with the built-in project (file-tree) panel — does not replace it.
- Renders two entries as links: **Today** and **Yesterday**.
- No existence indicator — entries are plain links regardless of whether the underlying file exists yet.

### 6.4 Date resolution
- **Naive local calendar date** (system timezone). No day-cutoff offset. At 1:00am Tuesday, "Today" = Tuesday; the "Yesterday" link is the way back to Monday.
- **Today** = current date. **Yesterday** = current date − 1 day.
- Target path = `<vault>/<daily.dir>/<formatted-date>.md`, where the date is formatted with `daily.filename`.

### 6.5 Click behavior (both entries)
1. Compute the target path for that date.
2. **If the file exists → open it.** (Never modified, never overwritten — safety invariant.)
3. **If it does not exist →** create it from `daily.template` with token substitution (§6.7), then open it.
4. If `<daily.dir>` doesn't exist, create it silently first.

### 6.6 Non-vault state
When the open workspace has no `.breadpaper/` marker, the panel shows a gentle prompt — e.g. **"This folder isn't a BreadPaper vault"** with a **[Create vault here]** action (scaffolds a `.breadpaper/` + default dirs into the current workspace) — rather than rendering blank or erroring. _(Exact copy/affordance: confirm on review — see §8.)_

### 6.7 Template token substitution
Basic Obsidian-style substitution, applied when a note is created. V1 token set:

| Token | Expands to | Example |
|---|---|---|
| `{{date}}` | ISO date | `2026-07-20` |
| `{{date:FORMAT}}` | moment-style formatted date | `{{date:dddd, MMMM D}}` → `Monday, July 20` |
| `{{time}}` | 24h local time | `14:31` |
| `{{title}}` | note filename without extension | `2026-07-20` |

`{{date}}`/`{{date:…}}` resolve to the **note's** date (so a created "Yesterday" note is dated yesterday, not now). `{{time}}` is creation time.

### 6.8 Failure modes
| Condition | Behavior |
|---|---|
| `daily/` dir missing | Create silently, then proceed. |
| Template file missing | Create an **empty** note + non-blocking warning. _(confirm — §8)_ |
| Write / permission error | Non-blocking error toast; no partial file left behind. |
| Workspace not a vault | Non-vault state (§6.6). |

## 7. Implementation notes (for engineering)

- Built as a native GPUI `Panel` in a core Zed crate (the extension API cannot render UI — see `../VISION.md` §7.1). **Keep the core diff small, isolated, and clearly namespaced** so it rebases cleanly onto `upstream/main`.
- Prefer isolating BreadPaper logic (vault discovery, config parsing, date/template handling) into a self-contained module/crate that the panel calls, so the delta against Zed's own files stays minimal.
- Date formatting: a moment-compatible formatter for the token/filename vocabulary.
- The spike's real output includes the **maintainability writeup** in §2 — record how many upstream files were touched and how the rebase felt.

## 8. Open assumptions to confirm on review

1. **Default vault location** `~/BreadPaper/` — acceptable, or prefer `~/Documents/BreadPaper/` / XDG-style?
2. **Non-vault state** copy & the "Create vault here" action (§6.6).
3. **Template-missing fallback** = empty note + warning (§6.8) — acceptable, or should it hard-fail with a clear error?
4. **`welcome.md` content** — to be drafted; short orientation to the panel + how to edit `config.toml`.

## 9. Decision log (from design interview, 2026-07-20)

- Build the **real feature**, no throwaway lane; go/no-go judged on final maintainability.
- Vault = **open workspace** at configurable-relative-paths with sane defaults; **multi-vault** product, single-vault-default UX.
- Vault identity = **`.breadpaper/` marker + per-vault `config.toml`** (hidden dir, room to grow).
- **Sample-vault scaffolding is in scope** (end-to-end on a clean machine).
- Filenames = **clean ISO**, `daily/` is **flat**, marker stays **hidden**.
- Date = **naive local calendar**; back-navigation via the Yesterday entry.
- Template = **basic Obsidian-style token substitution**.
- Panel = **Today + Yesterday** links, **coexists** with file tree, **no existence indicator**.
- Click = **create-if-missing → open**, **never overwrite**; no auto-open; dirs created silently.
- **Invisible git deferred** to a later iteration.
