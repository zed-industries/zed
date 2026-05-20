# Progress: staged/unstaged/branch diff filtering (Phase 2 revision)

Companion to [`prd-staged-unstaged-diff.md`](prd-staged-unstaged-diff.md). Tracks
the remaining work for the Phase 2 revision (six followups surfaced after the
first PoC build).

## Status legend

- `[ ]` not started
- `[~]` in progress
- `[x]` done
- `[?]` blocked / open question

## Phase 1 — Diff-view dropdown (M1 + M2 + M3)

Landed in the current branch (`staged-unstaged-diff-filtering`). No further
work expected unless a Phase 2 followup uncovers a regression.

- [x] M1 — diff filter resolver (`a6f627eafd feat(project): add diff filter resolver`)
- [x] M2 — single-sided diff loaders (`a6f627eafd feat(project): add single-sided diff loaders`)
- [x] M3 — toolbar dropdown + toolbar-stays-visible + addon re-application
      (`fae3de13b7 feat(git_ui): add project diff filter switching`)

## Phase 2 — Panel grouping (M4 + M5)

Initial build landed; six followups remain.

- [x] M4 — `Section::Staged` / `Section::Unstaged` variants, `group_by` setting,
      `(section, repo_path)` entry key (`d58c593003 feat(git_ui): group git panel entries by staging state`)
- [x] M5 — initial hover-revealed `+`/`-` per row, sticky-by-key fix
      (`e1a59f5944 feat: add staging affordances and fix stale selections`)
- [x] A1 — fix `+`/`-` hover regression (see below)
- [x] A2 — section-header `+`/`-` buttons in staging-grouped mode
- [ ] A3 — per-section diff stats
- [ ] A4 — `select_entry_by_path` sticky + filter-aware + `preferred_section`
- [ ] A5 — click → filter switch coupling
- [ ] A6 — `SetSortBy` action + "Sort by" submenu, remove `ToggleGroupBy`

## Followups (detailed checklists)

### A1 — `+`/`-` button disappears on hover (M5)

User story: 34. PRD: M5 "Wrapper-hitbox constraint", appendix A1.

- [x] Remove `.occlude()` from the per-row staging-control wrapper
      (`crates/git_ui/src/git_panel.rs` ≈ line 6459)
- [x] Remove `.occlude()` from the directory-row staging-control wrapper
      (≈ line 6695)
- [x] Verify the per-row click handler still calls `cx.stop_propagation()` in
      both `Checkbox` and `Action` branches (file rows: lines 6493/6529;
      directory rows: lines 6715/6757)
- [x] Test (gpui visual context): position the cursor on the staging-control
      wrapper, click, and assert the file's staging state changed. Tests
      clickability rather than visibility because `cx.debug_bounds` is
      registered before the `style.visibility == Hidden` early-return in
      `Interactivity::paint`, so `.is_some()` checks are not sensitive to
      `visible_on_hover`. Mouse-listener registration *is* gated on
      visibility, so a click on an invisible button fails to fire — which is
      the user's actual complaint
      (`test_staging_group_button_remains_clickable_when_cursor_enters_button`).

**Dependency:** must land **before** A2, since A2's header buttons reuse the
same wrapper pattern.

### A2 — Section-header `+`/`-` in staging-grouped mode (M5)

User stories: 17, 18 (revised). PRD: M5 "Section-header level", appendix A2.

- [x] Branch `render_list_header` on `group_by` via
      `staging_affordance_for_section(..., StagingAffordanceTarget::Section)`
      (`crates/git_ui/src/git_panel.rs:6109`)
- [x] Staged header → render hover-revealed `−` ("Unstage All Changes")
- [x] Unstaged header → render hover-revealed `+` ("Stage All Changes")
- [x] Conflicts header → render existing checkbox + whole-row toggle (unchanged)
- [x] Remove whole-row `on_click` toggle for Staged / Unstaged headers in
      staging-grouped mode; keep it for Conflicts and for all status-grouped
      headers (the `match staging_affordance` at the end of `render_list_header`
      only attaches `on_click` in the `Checkbox` arm).
- [x] Wrapper for header buttons must not use `.occlude()` (constraint from A1)
- [x] Tests landed in `crates/git_ui/src/git_panel.rs`:
      - `test_unstaged_section_header_stages_all_unstaged_entries_on_click`
      - `test_staged_section_header_unstages_all_staged_entries_on_click`
      - `test_staging_grouped_section_header_body_click_does_not_toggle`
      - `test_status_grouped_section_header_whole_row_click_still_toggles_staging`
      - extended `test_staging_group_uses_explicit_plus_minus_affordances` with
        the new `StagingAffordanceTarget::Section` cases (Conflicts header keeps
        the checkbox)

**Dependency:** A1.

### A3 — Per-section diff stats (M4)

User story: 33. PRD: M4 "Per-section diff stats", appendix A3.

Three stats per file must be plumbed end-to-end: `git diff` → `git` crate →
`project::StatusEntry` (in-process) → proto wire → renderer.

- [ ] **Repository layer** — extend `diff_stat` in
      `crates/git/src/repository.rs:967-970` and the real impl at `:2127` to
      cover three queries (combined `HEAD`, staged `--cached HEAD`, unstaged
      no-`--cached`). Either add two sibling methods or add a
      `DiffStatKind { Combined, Staged, Unstaged }` selector argument.
- [ ] **`project::StatusEntry`** (`crates/project/src/git_store.rs:220-225`)
      — add `diff_stat_staged: Option<DiffStat>` and `diff_stat_unstaged:
      Option<DiffStat>` alongside the existing `diff_stat`.
- [ ] **Proto** (`crates/proto/proto/git.proto:319-326`) — add four
      optional fields: `diff_stat_staged_added` (6), `diff_stat_staged_deleted`
      (7), `diff_stat_unstaged_added` (8), `diff_stat_unstaged_deleted` (9).
      Optional + new field numbers keep the wire backwards-compatible with
      older collab versions; missing values deserialize to `None`.
- [ ] **`StatusEntry::to_proto` and `TryFrom<proto::StatusEntry>`**
      (`git_store.rs:228-268`) — serialize/deserialize all three stats.
      Treat missing remote staged/unstaged values as "unknown" (same
      handling as today for the combined stat).
- [ ] **Repository update diffing** — the dirty-detection path that compares
      old vs new entries to decide which to push downstream must compare
      the two new stat fields as well, otherwise stat changes won't trigger
      a panel refresh.
- [ ] **`git_ui::GitStatusEntry`** (`crates/git_ui/src/git_panel.rs:617-624`)
      — mirror the three fields.
- [ ] **`entry_for_section`** (`git_panel.rs:755-765`) — assign the
      section-matching stat to the duplicated entry's display field (or
      have the renderer read the matching field by `Section` directly).
- [ ] Refresh all three stats in the same status refresh debounce; no
      mode-toggle flicker.
- [ ] **Test (A3 / story 33).** Build a fixture with a partially-staged
      file whose staged-side and unstaged-side numstats **intentionally
      differ** (e.g. staged `+3 −0`, unstaged `+1 −2`). Assert the Staged
      row's rendered `DiffStat == (3, 0)` and the Unstaged row's
      `DiffStat == (1, 2)`. Do not assert mere inequality of the two
      rendered values — legitimate coincidence (`+1 −0` on both sides)
      must not flake the test.

### A4 — Selection identity for partially-staged files (M4)

User story: 29 (reinforced). PRD: M4 "Selection identity", appendix A4.

- [ ] Add `preferred_section: Option<Section>` arg to
      `GitPanel::select_entry_by_path`
      (`crates/git_ui/src/git_panel.rs:1113`).
- [ ] Compute the **target section** *before* any stickiness check:
      `target: Option<Section> =
      preferred_section.or_else(|| section_from_diff_base())`
      where `DiffBase::Staged → Some(Section::Staged)`,
      `DiffBase::Unstaged → Some(Section::Unstaged)`,
      and `DiffBase::Head` / `DiffBase::Merge → None`.
- [ ] **Narrow-sticky no-op:** return without changes when **both**
      `selected.repo_path == target_path` **and**
      `target.map_or(true, |s| selected.section == s)`. When `target` is
      `Some(s)`, the selected row's section must equal `s`; when `target`
      is `None`, the section is unconstrained so the current duplicate row
      is preserved (no first-match flip).
- [ ] Re-resolve otherwise: when `target = Some(s)`, pick the entry whose
      `repo_path` matches *and* whose `section == s`; when `target = None`,
      fall back to the existing first-match heuristic.
- [ ] Update `ProjectDiff::handle_editor_event` call site to pass `None`
      for `preferred_section` (`crates/git_ui/src/project_diff.rs:786`).
- [ ] Update panel-internal callers (row click, scroll-to, reveal) to pass
      the explicit section when known.
- [ ] Test (row-click): synthetic Unstaged-row click then synthetic
      `EditorEvent::SelectionsChanged` → `selected_entry` still points to
      the Unstaged row. Repeat converse.
- [ ] Test (filter change for the same path): start with selection on the
      Staged row of `partial.rs`; flip the filter to Unstaged (so
      `target = Section::Unstaged`); call `select_entry_by_path` for
      `partial.rs` with `preferred_section = None` → selection must move
      to the Unstaged row of the same file, *not* stick on Staged.

**Dependency:** independent, but the A4 fix is observably correct only once
A5's click-handler dispatches the explicit section.

### A5 — Click → diff opened under target `DiffBase` (M3 + M4)

User story: 32, 23 (revised). PRD: M3 "Click → filter coupling", appendix A5.

A separate workspace dispatch of a `DiffBase` change does not work — by the
time it lands, `ProjectDiff::deploy_at` has already attached or created a
view under the wrong base. The target base must be threaded through the
open-diff API.

- [ ] **`ProjectDiff::deploy_at`** (`crates/git_ui/src/project_diff.rs:238-299`)
      — add `target_base: DiffBase` parameter. Replace the existing
      `matches!(diff_base(cx), DiffBase::Head)` (line 254-256) with
      `diff_base(cx) == target_base`. Also key the existing-item match by
      repository (existing logic at line 278-292) to avoid reusing a view
      from a different repo. **Never mutate an existing view's base to
      match the target** — distinct `DiffBase` values are distinct items;
      a Staged-target click on a workspace that already has a `Head` view
      open must create or activate a separate Staged view, not retarget
      the Head one. Remove the existing-mismatch reuse path entirely.
- [ ] **`ProjectDiff::deploy_at_project_path`** (line 301) — same
      `target_base` extension. External callers (e.g. agent panel) default
      to `DiffBase::Head`.
- [ ] **`ProjectDiff::new` / `new_impl`** (`project_diff.rs:371-380`) — take
      a starting `DiffBase` and thread it into `BranchDiff::new(..)`. Drop
      the hard-coded `DiffBase::Head` at line 378. Existing internal
      callers pass `DiffBase::Head` explicitly.
- [ ] In the panel row `on_click` handler
      (`crates/git_ui/src/git_panel.rs:6538-6549`), compute `target_base`
      via an **ordered match — first matching clause wins**, with the
      Branch override evaluated first so no later clause can return
      `Merge`:
      1. Current base = `Merge`: staging-grouped Staged row → `Staged`,
         Unstaged row → `Unstaged`, Conflicts row → `Head`;
         status-grouped any row → `Head`. ("Conflicts → unchanged" must
         not apply here — it would leave the user on `Merge`.)
      2. Staging-grouped mode (current base ≠ `Merge`): Staged row →
         `Staged`, Unstaged row → `Unstaged`, Conflicts row → current
         `DiffBase` (now guaranteed not `Merge`).
      3. Status-grouped mode (current base ≠ `Merge`): current
         `DiffBase`, unless the current filter wouldn't contain the file
         → `Head`.
- [ ] Call `ProjectDiff::deploy_at(workspace, Some(entry), target_base, …)`
      directly from the click handler (no preceding workspace dispatch).
- [ ] Pass the row's `Section` as `preferred_section` to
      `select_entry_by_path` from the click handler (joint task with A4).
- [ ] **Test.** Staged-row click in staging-grouped mode while no
      `ProjectDiff` exists → the newly created view has
      `diff_base(cx) == DiffBase::Staged`. Same for Unstaged. Repeat with
      an existing `DiffBase::Head` view open — the click must not reuse
      that view as a Staged target; it must activate or create a separate
      `DiffBase::Staged` view. Conflicts row leaves the current base
      alone. From Branch filter, any row click results in a non-`Merge`
      target.

**Dependency:** A4 (the click handler is the same place that passes
`preferred_section` to `select_entry_by_path`).

### A6 — "Sort by" submenu replaces "Group by" toggle (M4)

User story: 16 (revised). PRD: M4 "Menu surfacing", appendix A6.

- [ ] Define `SortBy { Status, Path, Staging }` enum (action arg)
- [ ] Add parameterized action `git_panel::SetSortBy { mode: SortBy }` with a
      handler that maps to `(group_by, sort_by_path)`:
      - `Status → (Status, false)`
      - `Path → (Status, true)`
      - `Staging → (Staging, *)` (leave sort_by_path as-is)
- [ ] Remove the `ToggleGroupBy` action declaration and its handler
- [ ] Remove the `ToggleSortByPath` action declaration and its handler
      (or keep the action as an alias that dispatches `SetSortBy` —
      decide during implementation)
- [ ] Update the panel kebab menu (`git_panel_context_menu`,
      `crates/git_ui/src/git_panel.rs:187-258`):
      - Remove the standalone "Group by …" entry
      - Replace the "Sort by …" toggle with a submenu containing three
        radio items dispatching `SetSortBy(Status|Path|Staging)`
      - Show the active option inline on the parent (e.g.
        `Sort by: Status ▸`)
      - Disable "By Path" with a tooltip when Tree View is active
- [ ] Remove the standalone keybindings (if any) for `ToggleGroupBy` and
      `ToggleSortByPath`
- [ ] Update `settings_ui/src/page_data.rs` references to the actions
- [ ] Test: dispatching each `SetSortBy(mode)` updates both settings as
      expected; menu rendering shows the active radio + disables Path in
      tree mode

## Cross-cutting tasks

- [ ] Update the existing test
      `test_bulk_staging_with_sort_by_paths` (and friends) to use
      `SetSortBy` instead of `ToggleSortByPath`, if those toggles are
      removed
- [ ] Run `./script/clippy` and the full panel test module after each
      followup
- [ ] Manual smoke: open a repo with a partially-staged file, exercise
      each followup end-to-end before committing

## Open questions for the Zed team (carried from Phase 1)

- The absence of a single-sided-diff `Project` API (M2) — should this
  become a first-class `Project` method, or should `BufferDiff`
  internally support filtering by `DiffHunkSecondaryStatus`?
- The UX of entries disappearing as they leave the active filter — the
  Zed team has previously flagged this; with the click → filter
  coupling now landed (A5), gather concrete examples before raising.
