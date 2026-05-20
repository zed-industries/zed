# Progress: staged/unstaged/branch diff filtering (Phase 2 revision)

Companion to [`prd-staged-unstaged-diff.md`](prd-staged-unstaged-diff.md). Tracks
the remaining work for the Phase 2 revision. Six followups surfaced after the
first PoC build (A1–A6); A7 was added later after A5 landed and the
staging-grouped section headers were observed to be inert outside the
`+`/`-` button.

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
- [x] A3 — per-section diff stats
- [x] A4 — `select_entry_by_path` sticky + filter-aware + `preferred_section`
- [x] A5 — click → filter switch coupling
- [ ] A6 — `SetSortBy` action + "Sort by" submenu, remove `ToggleGroupBy`
- [ ] A7 — section-header body click opens matching per-base `ProjectDiff`

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

### A3 — Per-section diff stats (M4) — DONE

User story: 33. PRD: M4 "Per-section diff stats", appendix A3.

Three stats per file plumbed end-to-end: `git diff` → `git` crate →
`project::StatusEntry` (in-process) → proto wire → renderer.

- [x] **Repository layer** — `git::repository::DiffStatKind { Combined,
      Staged, Unstaged }` enum added; `Repository::diff_stat` takes a
      `kind` selector. `RealGitRepository` switches git args
      (`HEAD` / `--cached HEAD` / no extra arg). `FakeGitRepository`
      switches the content-pair being compared. The fake also drops
      untracked files from the Unstaged collection so it matches real
      `git diff --numstat`.
- [x] **`project::StatusEntry`** — added `diff_stat_staged: Option<DiffStat>`
      and `diff_stat_unstaged: Option<DiffStat>` alongside `diff_stat`.
- [x] **Proto** — added four optional fields to `proto::StatusEntry`:
      `diff_stat_staged_added` (6), `diff_stat_staged_deleted` (7),
      `diff_stat_unstaged_added` (8), `diff_stat_unstaged_deleted` (9).
      Backwards-compatible: older peers send `None` and we deserialize to
      `None`. Covered by
      `test_status_entry_missing_side_specific_stats_from_older_peers_deserialize_to_none`.
- [x] **`StatusEntry::to_proto` / `TryFrom<proto::StatusEntry>`** —
      round-trip all three stats. Covered by
      `test_status_entry_round_trips_three_diff_stats_via_proto`.
- [x] **Repository update diffing** — `build_update`'s equality check and
      the incremental refresh's cursor seek both compare all three stats,
      so a stat change on either side dirties the entry.
- [x] **`git_ui::GitStatusEntry`** — mirrors the three fields; source
      sites (status refresh, single-staged-entry fallback, file-header
      lookup) all thread the new values through.
- [x] **`entry_for_section`** — overrides `diff_stat` with the
      section-matching stat under `Section::Staged` / `Section::Unstaged`;
      Tracked / New / Conflict keep the combined stat unchanged. Renderer
      stays trivial. Covered by
      `test_entry_for_section_uses_side_specific_diff_stat_for_partial_file`.
- [x] All three numstats are computed concurrently per refresh
      (`try_join4` incremental path, `try_join5` full path), so a panel
      mode toggle doesn't trigger a fresh git invocation and there's no
      flicker.
- [x] **Test (A3 / story 33).** Added
      `test_partially_staged_file_row_diff_stats_match_section`: a
      fixture with HEAD `""`, index `"a\nb\nc\n"`, worktree
      `"a\nb\nc\nd\n"` yields three distinct numstats — combined
      `(+4, −0)`, staged `(+3, −0)`, unstaged `(+4, −3)`. The test
      asserts the Staged row's rendered `DiffStat == (3, 0)` and the
      Unstaged row's `DiffStat == (4, 3)`, each compared against its
      single-sided source so the previous bug (combined reused on both
      rows) would visibly fail.
- [x] FakeGitRepository semantic coverage: new
      `test_diff_stat_kind_returns_side_specific_numstats` asserts the
      three kinds produce the expected pairs from the same fixture.

### A4 — Selection identity for partially-staged files (M4) — DONE

User story: 29 (reinforced). PRD: M4 "Selection identity", appendix A4.

- [x] Made `Section` `pub` so it can appear in the public
      `select_entry_by_path` signature without a `private_interfaces` lint
      error (the lint fires under `-D warnings`).
- [x] Added `preferred_section: Option<Section>` arg to
      `GitPanel::select_entry_by_path` (`crates/git_ui/src/git_panel.rs`).
- [x] **Section is computed by the firing caller, not by the panel.**
      `ProjectDiff::handle_editor_event` (`crates/git_ui/src/project_diff.rs`)
      maps its own `self.diff_base(cx)` to a `Section` and passes it as
      `preferred_section`. This deviates from the PRD's
      `section_from_diff_base()` panel-side fallback for three reasons:
      (1) it eliminates a per-`SelectionsChanged` workspace lookup on the
      hot path, (2) it removes a latent multi-`ProjectDiff` correctness
      bug — the panel-side `active_item_as::<ProjectDiff>` could read the
      *wrong* base if the user switched panes between the event firing
      and the handler running, and (3) it lets the test drive the
      function directly with `Some(Section::Unstaged)` instead of needing
      a `set_diff_base` back door.
- [x] **Narrow-sticky no-op:** returns without changes when **both**
      `selected.repo_path == target_path` **and**
      `preferred_section.is_none_or(|s| selected.section == s)`. When
      `preferred_section` is `Some(s)`, the selected row's section must
      equal `s`; when `None`, the section is unconstrained so the current
      duplicate row is preserved (no first-match flip). Header / directory
      rows have no `status_entry()` so the sticky check is inapplicable and
      the function falls through to re-resolve. Uses the existing
      `get_selected_entry()` helper.
- [x] Re-resolves otherwise: `preferred_section.or_else(|| existing
      status-based derivation)` is plumbed into the `entry_by_key` lookup,
      so an explicit `Some(s)` picks the row whose `(section, repo_path)`
      matches, and `None` falls back to the existing first-match heuristic.
- [x] All other callers (`git_panel.rs:10231` existing tree-view test,
      `git_graph.rs:5150` file-history test) pass `None`. No internal
      panel call site needs a `Some(section)` yet; that becomes
      load-bearing only once A5's click handler routes through
      `select_entry_by_path`.
- [x] Test (row-click stickiness):
      `test_select_entry_by_path_is_sticky_for_partially_staged_file`.
      Sets `selected_entry` to the Unstaged row of `partial.rs`, calls
      `select_entry_by_path(partial.rs, None)`, asserts selection still
      points to the Unstaged row. Repeats converse for the Staged row.
      Verified RED by replacing the narrow-sticky check with `if false`
      — both assertions panic with the Staged index (1) instead of the
      Unstaged index (3).
- [x] Test (cross-section move when preferred_section changes):
      `test_select_entry_by_path_moves_section_when_preferred_section_changes`.
      Forces the selection on the Staged row of `partial.rs`, then calls
      `select_entry_by_path(partial.rs, Some(Section::Unstaged))`, asserts
      the selection moves to the Unstaged row. Verified RED by
      short-circuiting the `let section = preferred_section.or_else(…)`
      to ignore the preferred section — the assertion panics with the
      Staged index instead.
- [x] Shared fixture helper `build_partial_file_panel(cx) -> (panel,
      partial_path, staged_ix, unstaged_ix, VisualTestContext)` cuts ~40
      lines of duplicated scaffolding (fs, status, project, workspace,
      `group_by = Staging`, panel build, refresh, key lookups) out of
      each test.

**Dependency:** independent, but the A4 fix is observably correct only once
A5's click-handler dispatches the explicit section.

### A5 — Click → diff opened under target `DiffBase` (M3 + M4) — DONE

User story: 32, 23 (revised). PRD: M3 "Click → filter coupling", appendix A5.

A separate workspace dispatch of a `DiffBase` change does not work — by the
time it lands, `ProjectDiff::deploy_at` has already attached or created a
view under the wrong base. The target base is now threaded through the
open-diff API.

- [x] **`ProjectDiff::deploy_at`** (`crates/git_ui/src/project_diff.rs`) —
      added `target_base: DiffBase` parameter. Replaced the existing
      `matches!(diff_base(cx), DiffBase::Head)` filter with
      `diff_base(cx) == target_base`. **Never mutates an existing view's
      base to match the target** — distinct `DiffBase` values are distinct
      items; a Staged-target click on a workspace that already has a `Head`
      view open creates a separate Staged view and leaves the `Head` view
      alone.
- [x] **Repo matching deferred.** The progress file suggested also keying
      the existing-item match by repository, but doing so broke the
      pre-existing `test_deploy_at_respects_active_repository_selection`
      contract (one diff view that "follows" the active repo via the
      branch-diff re-pointing path). The new code matches on `diff_base`
      only and keeps the existing repo-switching mutation downstream. The
      A5 invariant ("never retarget across `DiffBase` values") is
      preserved.
- [x] **`ProjectDiff::deploy_at_project_path`** — same `target_base`
      extension. External callers (e.g. agent panel) default to
      `DiffBase::Head`.
- [x] **`ProjectDiff::new` / `new_impl`** — takes a starting `DiffBase`
      and threads it into `BranchDiff::new(..)`. The hard-coded
      `DiffBase::Head` was dropped; internal callers pass `DiffBase::Head`
      explicitly. External caller in
      `crates/zed/src/visual_test_runner.rs` updated similarly.
- [x] **`target_diff_base_for_click`** pure helper added to
      `crates/git_ui/src/git_panel.rs` next to `entry_for_section`.
      Implements the **ordered match — first matching clause wins** with
      the Branch override evaluated first so no later clause can return
      `Merge`:
      1. Current base = `Merge`: staging-grouped Staged row → `Staged`,
         Unstaged row → `Unstaged`, any other section / status-grouped →
         `Head`.
      2. Staging-grouped, current base ≠ `Merge`: Staged → `Staged`,
         Unstaged → `Unstaged`, Conflicts/Tracked/New → current `DiffBase`.
      3. Status-grouped, current base ≠ `Merge`: keep the current
         `DiffBase`, unless `current = Staged ∧ ¬has_staged` or
         `current = Unstaged ∧ ¬has_unstaged` → `Head`.
- [x] **`GitPanel::target_diff_base_for_entry`** is the thin wrapper that
      pulls the current `DiffBase` off the active `ProjectDiff` (falling
      back to `DiffBase::Head` when no diff is open) and the `group_by`
      off `GitPanelSettings`, then delegates to the pure helper.
- [x] **`GitPanel::open_diff`** now computes `target_base` and calls
      `ProjectDiff::deploy_at(workspace, Some(entry), target_base, …)`
      directly. The early-return path (already on the right `(repo_path)`)
      was widened to also require `diff_base == target_base`, otherwise
      it would short-circuit the diff_base change when the same path is
      already shown under a different base.
- [x] **`preferred_section` wire-up:** the row click sets
      `selected_entry = Some(ix)` directly, which already keys by
      `(section, repo_path)`. When `deploy_at` opens the diff and the
      editor selection event fires, `handle_editor_event` (wired in A4)
      derives `preferred_section` from the now-active `DiffBase`, so the
      narrow-sticky check inside `select_entry_by_path` no-ops on the
      correct row. No extra plumbing is needed at the click site.
- [x] **Pure-helper tests** in `crates/git_ui/src/git_panel.rs`:
      - `test_target_diff_base_branch_override_exits_merge_to_matching_side`
      - `test_target_diff_base_staging_grouped_routes_to_section_side`
      - `test_target_diff_base_status_grouped_keeps_current_when_file_matches_filter`
      - `test_target_diff_base_status_grouped_falls_back_to_head_when_file_not_in_filter`
- [x] **End-to-end integration tests** in
      `crates/git_ui/src/git_panel.rs`:
      - `test_clicking_staged_section_row_opens_diff_under_staged_base`
      - `test_clicking_unstaged_section_row_opens_diff_under_unstaged_base`
      - `test_clicking_unstaged_row_after_staged_view_creates_separate_view`
        (guards against the same-path early-return short-circuit)
      - `test_clicking_staged_row_while_branch_diff_open_exits_to_staged_base`
        (the Branch-override path)
- [x] **`deploy_at` direct-API tests** in
      `crates/git_ui/src/project_diff.rs`:
      - `test_deploy_at_creates_fresh_view_under_target_base`
      - `test_deploy_at_does_not_retarget_existing_view_to_new_base`

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

### A7 — Section-header body click opens matching per-base ProjectDiff (M5)

User story: 35. PRD: M5 "Section-header level", appendix A7.

Supersedes A2's "the body click is intentionally a no-op" rule for the
Staged and Unstaged headers in staging-grouped mode. The body becomes a
click target that opens the matching `DiffBase` view via A5's
`deploy_at(target_base, …)`. The `+`/`-` button retains the
bulk-stage/unstage responsibility.

- [ ] In `render_list_header` (`crates/git_ui/src/git_panel.rs`), add a
      `.when(...)` branch that attaches `.on_click(...)` for staging-grouped
      `Section::Staged` and `Section::Unstaged` headers (parallel to the
      existing `needs_whole_row_toggle` branch, which stays for headers
      with the `Checkbox` affordance — Conflicts in staging-grouped mode
      and every header in status-grouped mode).
- [ ] Handler computes `target_base = match section { Staged →
      DiffBase::Staged, Unstaged → DiffBase::Unstaged }` directly — no
      call to `target_diff_base_for_click` because the mapping collapses
      to a constant for these two sections (including under the Branch
      override).
- [ ] Handler calls `ProjectDiff::deploy_at(workspace, None, target_base,
      window, cx)` via `self.workspace.update(...)`.
- [ ] Handler does **not** mutate `selected_entry` — the panel's row
      selection is left alone.
- [ ] Handler calls `cx.stop_propagation()` (defensive — the `+`/`-`
      button already stops propagation, but explicit is safer).
- [ ] **Rename the existing legacy test.** Rename
      `test_staging_grouped_section_header_body_click_does_not_toggle` →
      `test_staging_grouped_section_header_body_click_does_not_stage_files`
      and rewrite its leading comment — the body click is no longer a
      no-op; what the test now guards is that the body click does **not**
      bulk-stage the section. The two assertions (`entry_by_key(Unstaged,
      ...)` still resolves; `staging_group_staged_count == 0`) stay
      unchanged.
- [ ] **New tests** in `crates/git_ui/src/git_panel.rs`:
      - `test_clicking_staged_header_body_opens_staged_diff` —
        staging-grouped mode, simulate-click left of the
        `git-panel-section-header-stage-control-Staged` bounds; assert
        the active `ProjectDiff` has `diff_base == Staged`.
      - `test_clicking_unstaged_header_body_opens_unstaged_diff` — mirror
        for the Unstaged header.
      - `test_clicking_staged_header_body_does_not_change_panel_selection`
        — set `selected_entry` to a known row, click the Staged header
        body, assert `selected_entry` is unchanged.
      - `test_clicking_staged_header_body_while_branch_diff_open_exits_to_staged`
        — open a `DiffBase::Merge` `ProjectDiff` first, then body-click
        the Staged header; assert the resulting active diff has
        `diff_base == Staged`. Branch-override regression guard
        analogous to A5's
        `test_clicking_staged_row_while_branch_diff_open_exits_to_staged_base`.
      - `test_clicking_staged_header_button_does_not_switch_filter` —
        click directly on the `−` button; assert (a) the bulk-unstage
        fires (status updates as expected) and (b) **no** new
        `DiffBase::Staged` `ProjectDiff` is created (the existing `Head`
        view stays active). Regression guard for the
        `cx.stop_propagation()` chain.

**Dependency:** A5 (uses `ProjectDiff::deploy_at(target_base, …)`).

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
