# Progress: staged/unstaged/branch diff filtering (Phase 2 revision)

Companion to [`prd-staged-unstaged-diff.md`](prd-staged-unstaged-diff.md). Tracks
the remaining work for the Phase 2 revision. Six followups surfaced after the
first PoC build (A1–A6); A7 was added later after A5 landed and the
staging-grouped section headers were observed to be inert outside the
`+`/`-` button. A8 was added after the Staged filter was observed to be
editable — edits leak into the staged view because it shows the live worktree
buffer rather than the git index. A1–A8 are now done.

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

Initial build landed; A1–A8 done.

- [x] M4 — `Section::Staged` / `Section::Unstaged` variants, `group_by` setting,
      `(section, repo_path)` entry key (`d58c593003 feat(git_ui): group git panel entries by staging state`)
- [x] M5 — initial hover-revealed `+`/`-` per row, sticky-by-key fix
      (`e1a59f5944 feat: add staging affordances and fix stale selections`)
- [x] A1 — fix `+`/`-` hover regression (see below)
- [x] A2 — section-header `+`/`-` buttons in staging-grouped mode
- [x] A3 — per-section diff stats
- [x] A4 — `select_entry_by_path` sticky + filter-aware + `preferred_section`
- [x] A5 — click → filter switch coupling
- [x] A6 — `SetSortBy` action + "Sort by" submenu, remove `ToggleGroupBy`
- [x] A7 — section-header body click opens matching per-base `ProjectDiff`
- [x] A8 — Staged filter is a read-only git-index snapshot (fixes the edit leak)
- [x] A9 — Staged view visual parity: synthetic-file snapshot so it renders like the Unstaged view (syntax highlighting, file header/icon, status badge, read-only lock)

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

### A6 — "Sort by" submenu replaces "Group by" toggle (M4) — DONE

User story: 16 (revised). PRD: M4 "Menu surfacing", appendix A6.

- [x] Defined `SortBy { Status, Path, Staging }` enum.
- [x] Added parameterized action `git_panel::SetSortBy { mode: SortBy }`
      with a handler that maps to `(group_by, sort_by_path)`:
      - `Status → (Status, false)`
      - `Path → (Status, true)`
      - `Staging → (Staging, *)` (sort_by_path left as-is)
- [x] Removed the `ToggleGroupBy` action declaration, handler, and
      `on_action` wiring. Chose to drop entirely rather than alias — no
      default keybinding referenced it, and aliasing would have required
      a translation that can't preserve the toggle semantics anyway.
- [x] Removed the `ToggleSortByPath` action declaration, handler, and
      `on_action` wiring (same rationale).
- [x] Updated the panel kebab menu (`git_panel_context_menu`,
      `crates/git_ui/src/git_panel.rs`):
      - Removed the standalone "Group by …" and "Sort by …" toggle entries.
      - Added a `.submenu(sort_by_submenu_label(group_by, sort_by_path),
        …)` whose builder pushes three `ContextMenuEntry`s (By Status / By
        Path / By Staging) each `.toggleable(IconPosition::Start, …)` with
        the active radio set from `current_sort_by(group_by,
        sort_by_path)` and dispatching `SetSortBy { mode }`.
      - Parent label renders the active option inline (e.g. `Sort by:
        Status`).
      - "By Path" is `.disabled(true)` when `tree_view` is on. No tooltip
        is attached — `ContextMenuEntry` does not currently expose a
        tooltip API and `documentation_aside` would be visually heavy
        for a one-line hint. The greyed-out item next to the active Tree
        View toggle is self-explanatory. Revisit if the team wants the
        explanatory hint.
- [x] No legacy keybindings to remove — searched
      `assets/keymaps/default-*.json` and neither `git_panel::ToggleGroupBy`
      nor `git_panel::ToggleSortByPath` is bound.
- [x] No `settings_ui/src/page_data.rs` changes needed — that file
      references the underlying settings fields (`git_panel.sort_by_path`,
      `git_panel.group_by`) via `SettingField`, not the action names. The
      action removal is invisible to the settings page.
- [x] Tests in `crates/git_ui/src/git_panel.rs`:
      - `test_set_sort_by_status_writes_status_grouping_and_clears_path_sort`
      - `test_set_sort_by_path_writes_status_grouping_and_enables_path_sort`
      - `test_set_sort_by_staging_preserves_existing_sort_by_path`
        (verifies the `Staging → (Staging, *)` "leave sort_by_path
        as-is" requirement by going through the file-backed write path
        twice — `SetSortBy(Path)` then `SetSortBy(Staging)` — and
        asserting `sort_by_path == true` survives).
      - `test_current_sort_by_maps_settings_to_radio_state` (pure
        helper covering all four `(group_by, sort_by_path)` quadrants,
        including the "Staging dominates sort_by_path" rule).
      - `test_sort_by_submenu_label_reflects_current_sort_mode` (pure
        helper covering the inline `Sort by: <mode>` parent-label
        format for all four quadrants).
      Menu render assertion against the live `ContextMenu` items is
      not done — `ContextMenu::items` is private to the `ui` crate and
      adding a public accessor felt disproportionate. The pure-helper
      tests cover the "active option" and "Path-disabled-in-tree-mode"
      logic that drives rendering, and the existing
      `test_toggle_group_by_updates_git_panel_setting` was deleted
      (the three SetSortBy tests subsume its coverage).

### A7 — Section-header body click opens matching per-base ProjectDiff (M5) — DONE

User story: 35. PRD: M5 "Section-header level", appendix A7.

Supersedes A2's "the body click is intentionally a no-op" rule for the
Staged and Unstaged headers in staging-grouped mode. The body becomes a
click target that opens the matching `DiffBase` view via A5's
`deploy_at(target_base, …)`. The `+`/`-` button retains the
bulk-stage/unstage responsibility.

- [x] In `render_list_header` (`crates/git_ui/src/git_panel.rs`), added a
      second `.when(...)` branch (sibling to the existing
      `needs_whole_row_toggle` branch) that attaches `.on_click(...)` for
      staging-grouped `Section::Staged` and `Section::Unstaged` headers.
      Conflicts in staging-grouped mode keeps its Checkbox + whole-row
      toggle via the existing branch; status-grouped headers are
      untouched.
- [x] Handler computes `target_base = match section { Staged →
      DiffBase::Staged, Unstaged → DiffBase::Unstaged }` directly — no
      call to `target_diff_base_for_click` because the mapping collapses
      to a constant for these two sections (including under the Branch
      override).
- [x] Handler calls `ProjectDiff::deploy_at(workspace, None, target_base,
      window, cx)` via `self.workspace.update(...)`.
- [x] Handler does **not** mutate `selected_entry` directly. The naive
      version still loses the user's panel selection because
      `deploy_at` triggers `EditorEvent::SelectionsChanged`, which
      re-syncs the panel via `select_entry_by_path`. To preserve story
      35's "view-switch, not navigation" semantics:
      - New field `GitPanel::suppress_next_path_sync: bool`.
      - Set to `true` by the header-body click handler before
        `deploy_at`. Cleared async by a follow-up `cx.spawn` after
        `SUPPRESS_PATH_SYNC_WINDOW` (100 ms) so legitimate user-driven
        editor navigation resumes syncing the panel.
      - `select_entry_by_path` short-circuits when the flag is set
        (without clearing it — the timer owns the clear). The flag is
        checked before any other work, so all `SelectionsChanged`
        events fired during the suppression window are ignored.
- [x] Handler calls `cx.stop_propagation()` (defensive — the `+`/`-`
      button already stops propagation, but explicit is safer).
- [x] **Renamed the existing legacy test:**
      `test_staging_grouped_section_header_body_click_does_not_toggle` →
      `test_staging_grouped_section_header_body_click_does_not_stage_files`.
      The leading comment was rewritten — the body click is no longer a
      no-op; what the test now guards is that the body click does **not**
      bulk-stage the section. The two assertions (`entry_by_key(Unstaged,
      ...)` still resolves; `staging_group_staged_count == 0`) stay
      unchanged.
- [x] **New tests** in `crates/git_ui/src/git_panel.rs`:
      - `test_clicking_staged_header_body_opens_staged_diff` —
        staging-grouped mode, simulate-click left of the
        `git-panel-section-header-stage-control-Staged` bounds; asserts
        the active `ProjectDiff` has `diff_base == Staged`.
      - `test_clicking_unstaged_header_body_opens_unstaged_diff` — mirror
        for the Unstaged header.
      - `test_clicking_staged_header_body_does_not_change_panel_selection`
        — sets `selected_entry` to the Unstaged row of `unstaged.rs`,
        clicks the Staged header body, asserts `selected_entry` is
        unchanged. RED on a single-file fixture (the editor sync moves
        the selection to the first file in the newly opened diff);
        GREEN after wiring the `suppress_next_path_sync` flag.
      - `test_clicking_staged_header_body_while_branch_diff_open_exits_to_staged`
        — opens a `DiffBase::Merge` `ProjectDiff` first, then
        body-clicks the Staged header; asserts the resulting active
        diff has `diff_base == Staged`. Branch-override regression
        guard analogous to A5's
        `test_clicking_staged_row_while_branch_diff_open_exits_to_staged_base`.
      - `test_clicking_staged_header_button_does_not_switch_filter` —
        clicks directly on the `−` button; asserts (a) the bulk-unstage
        fires (file moves from Staged to Unstaged) and (b) the active
        `ProjectDiff`'s entity_id and `diff_base` are both unchanged
        (the existing `Head` view stays active). Regression guard for
        the `cx.stop_propagation()` chain on the button.

**Dependency:** A5 (uses `ProjectDiff::deploy_at(target_base, …)`).

### A8 — Staged filter is a read-only git-index snapshot (M2 + M3) — DONE

User stories: 36, 37, 38, 39, 40, 41. PRD: M2 "Revised for the Staged filter",
appendix A8.

> **Superseded in part by A9.** A8's snapshot buffer was file-less
> (`Buffer::local`); A9 reworked it to carry a synthetic read-only
> `language::File` so the Staged view renders like a real file. The write-path
> map and no-leak behavior below are unchanged; only the buffer construction and
> the `render_buffer_header` path-key fallback (now removed) differ. See A9.

**Resolution.** For `DiffBase::Staged` the displayed buffer is now the git
**index** content as a file-less, read-only `Buffer::local` snapshot (diff base =
`HEAD`), built by `GitStore::open_staged_index_snapshot`
(`crates/project/src/git_store.rs`). Inline unstage writes the index through an
explicit `repo_path` override map (`GitStore::index_snapshot_repos:
HashMap<BufferId, (WeakEntity<Repository>, RepoPath)>`) consumed by
`on_index_snapshot_diff_event`, plus a multibuffer fallback in
`Editor::do_stage_or_unstage` (`buffer_for_id(..).or_else(|| multibuffer.buffer(..))`)
— the synthetic buffer is never registered in the project buffer store. The
chosen design keeps inline unstage (the fallback in the "Risks" note below was
not needed). Unstaged / Uncommitted / Branch are unchanged.

**Problem.** The Staged filter shows excerpts of the **live worktree buffer**
(`Capability::ReadWrite`, `project_diff.rs:408`) with `DiffHunkFilter::Staged`,
not the git index. The filter only governs which hunks are highlighted, so
later worktree edits appear in the staged view as unhighlighted context even
though git records them as unstaged. Fix: for `DiffBase::Staged` only, show the
git **index** content as a **read-only** snapshot. Scope is **Staged only** —
Unstaged / Uncommitted stay live editable worktree; Branch unchanged.

- [x] **Index-backed buffer (M2, stories 36).** `GitStore::open_staged_index_snapshot`
      loads `(HEAD, index)` text and builds a file-less `Buffer::local(index)` set
      to `Capability::ReadOnly`; primary `BufferDiff` base = `HEAD`. The Staged
      branch is taken in `branch_diff.rs::load_buffer` (early-return for
      `DiffType::HeadToIndex` with no branch base). Unstaged keeps the
      worktree-buffer + secondary-diff route. *(Test: `test_staged_index_snapshot_shows_index_not_worktree`.)*
- [x] **Read-only enforcement is automatic (story 37, rendering half).** The
      index buffer is `ReadOnly` while the ProjectDiff multibuffer stays
      `ReadWrite`, so `editor.read_only(cx)` is `false` and the inline hunk
      controls render. No editor-level read-only flag. *(Tests:
      `test_staged_filter_rejects_inline_text_edits` asserts the edit is rejected
      and capability `ReadOnly`; `test_unstaged_filter_accepts_inline_text_edits`
      is the scope guard.)*
- [x] **Inline unstage write-path plumbing (story 41) — the hard part.** Chose
      the **explicit `repo_path` override map**: `GitStore::index_snapshot_repos:
      HashMap<BufferId, (WeakEntity<Repository>, RepoPath)>`, populated by
      `open_staged_index_snapshot` and consumed by a dedicated
      `on_index_snapshot_diff_event` handler (mirrors `on_buffer_diff_event` but
      uses the stored `(repo, repo_path)` instead of `buffer.project_path`). The
      secondary `BufferDiff` base = index makes the primary hunks `NoSecondaryHunk`
      (unstage-able). `Editor::do_stage_or_unstage` resolves the file-less buffer
      via `buffer_for_id(..).or_else(|| self.buffer.read(cx).buffer(buffer_id))`.
      The synthetic buffer is **not** registered in the project buffer store.
      *(Tests: project-level `test_staged_index_snapshot_unstage_writes_the_index`;
      git_ui in-editor `test_staged_filter_inline_unstage_writes_the_index` —
      verified RED when the `do_stage_or_unstage` fallback is removed.)*
- [x] **Index-content computation (story 41, second risk).** With base = `HEAD`
      and the index as the buffer, `stage_or_unstage_hunks` reverts the hunk to
      the `HEAD` text. Confirmed end-to-end: the index reverts from
      `"one\nTWO STAGED\nthree\n"` to `"one\ntwo\nthree\n"`.
- [x] **Refresh on any index change (story 39).** The existing
      `BranchDiffEvent::FileListChanged` → `Self::refresh` path rebuilds the
      snapshot, so it tracks inline unstage, panel/header `+`/`-`, and external
      `git add`/`reset`. *(Tests: `test_staged_filter_reloads_when_index_changes`
      (external `git add`/`reset`); `test_staged_filter_panel_unstage_writes_the_index`
      (panel/header `-` keeps the open snapshot consistent).)*
- [x] **File-less plumbing (story 40).** Save and reload on the read-only,
      never-dirty index buffer are no-ops (the file-less save path is never hit).
      `ProjectDiff::active_path` resolves the focused index excerpt to its
      worktree `ProjectPath` via `worktree_project_path_for_buffer`, which
      delegates to `GitStore::repository_and_path_for_buffer_id` (the snapshot
      map) → `repo_path_to_project_path`. *(Tests:
      `test_staged_filter_save_and_reload_are_safe_noops`,
      `test_staged_filter_active_path_resolves_for_index_buffer`.)*
- [x] **Open-to-edit (story 38).** For the Staged base,
      `configure_editor_for_diff_base` sets `rhs_editor.set_delegate_open_excerpts(true)`,
      so activating a hunk/header (Enter / clickable header filename / double-click,
      all via `open_excerpts_common`) emits `OpenExcerptsRequested`;
      `ProjectDiff::handle_editor_event` resolves each file-less buffer to its
      worktree `ProjectPath` and `workspace.open_path(..)`s the editable file.
      `render_buffer_header` also falls back to the multibuffer path key so the
      file-less excerpt header still shows the filename (and stays activatable).
      *(Test: `test_staged_filter_open_to_edit_opens_worktree_file` — asserts the
      opened editor's buffer has `file().is_some()` and capability `ReadWrite`;
      verified RED before delegation (it opened the file-less buffer).)*
- [x] **Tests (A8 / stories 36–41)** in `crates/git_ui/src/project_diff.rs` +
      `crates/project/tests/integration/project_tests.rs`:
      - [x] *Read-only:* `test_staged_filter_rejects_inline_text_edits`.
      - [x] *No leak:* `test_staged_filter_shows_read_only_index_snapshot_without_worktree_leak`.
      - [x] *Inline unstage writes the index (story 41 regression guard):*
        `test_staged_filter_inline_unstage_writes_the_index` (in-editor) +
        `test_staged_index_snapshot_unstage_writes_the_index` (project-level);
        fails with the silent-no-op when the plumbing is removed.
      - [x] *Panel unstage still works:* `test_staged_filter_panel_unstage_writes_the_index`.
      - [x] *Refresh on external change:* `test_staged_filter_reloads_when_index_changes`.
      - [x] *Open-to-edit:* `test_staged_filter_open_to_edit_opens_worktree_file`.
      - [x] *Save/reload safety:* `test_staged_filter_save_and_reload_are_safe_noops`.
      - [x] *Unstaged stays editable:* `test_unstaged_filter_accepts_inline_text_edits`.

**Risks / open questions.** The inline unstage write-path rework (story 41) is
the load-bearing unknown — it touches `editor/src/git.rs` and the `GitStore`
buffer→repo resolution, not just the diff loader. If that plumbing proves too
invasive for the PoC, a fallback is to drop *inline* unstage in the Staged view
and rely on the panel/header `-` controls (which already work), revising stories
37/41 — but the chosen design keeps inline unstage.

**Dependency:** builds on M2 (single-sided loaders) and M3 (filter dropdown).
Independent of A1–A7.

### A9 — Staged view visual parity (file rendering) — DONE

Follow-up from review feedback: A8's file-less snapshot rendered without syntax
highlighting, file icon, parent path, status badge, or a read-only affordance, so
the Staged view looked unlike the Unstaged view. A9 makes the snapshot render
identically by giving it a synthetic file (the proven `commit_view.rs` `GitBlob`
pattern), keeping the index-content + read-only behavior intact.

**Resolution.** `GitStore::open_staged_index_snapshot` now builds the buffer with
`Buffer::build(..)` carrying a synthetic `StagedIndexBlob` `language::File`
(`DiskState::Historic`, real repo path, worktree id resolved via
`repo_path_to_project_path`) instead of file-less `Buffer::local`. It detects and
async-loads the language (`language_for_file` + `load_language`, registry threaded
from the `Project::open_staged_index_snapshot` wrapper) and assigns it to the
buffer and both diff base texts. The Staged editor additionally registers
`BranchDiffAddon` (alongside `GitPanelAddon`) for the status badge, resolved via a
new `repository_and_path_for_buffer_id` fallback to `index_snapshot_repos`. The
file-less `path_for_buffer` fallback in `render_buffer_header` is removed.

- [x] **Synthetic file (rendering foundation).** `StagedIndexBlob: language::File`
      (`Historic` disk state, real path) attached via `Buffer::build`; the buffer
      stays `Capability::ReadOnly`, the multibuffer stays `ReadWrite`. *(Test:
      `test_staged_index_snapshot_buffer_carries_file_for_diff_view`, which also
      asserts `resolve_file_path` works without the removed fallback.)*
- [x] **Syntax highlighting (buffer: context + added lines).** Language detected
      from the file path + content and async-loaded, assigned to the buffer and both
      `set_base_text` calls. *(Test:
      `test_staged_index_snapshot_buffer_has_detected_language`.)* This colors
      context and added/green lines; deleted/red lines render from the diff
      base-text buffer and needed a separate `language_changed` call — see the
      post-A9 follow-up below.
- [x] **A/M/D status badge.** `BranchDiffAddon` registered for the Staged base
      supplies `override_status_for_buffer_id`; `repository_and_path_for_buffer_id`
      falls back to `index_snapshot_repos` so it resolves the synthetic buffer.
      *(Tests: `test_staged_filter_shows_file_status_badge`;
      `test_project_diff_reapplies_editor_addon_when_diff_filter_changes` updated
      for the Staged↔Head addon transitions.)*
- [x] **Read-only lock + tooltip.** The header's existing muted `FileLock` (shown
      for any read-only buffer) now sits in a properly-rendered header; a tooltip
      was added — "Read-only — open the file to edit" for git-sourced (`Historic`)
      buffers, "Read-only" otherwise. The lock condition is unchanged.
- [x] **Stage/unstage header controls.** A free consequence of the synthetic file:
      `GitPanelAddon::render_buffer_header_controls` no longer bails on
      `buffer.file()` and resolves via `worktree_id` + path. *(Guard:
      `test_staged_index_snapshot_file_resolves_to_its_repo_path`.)*
- [x] **Removed the file-less header fallback.** `render_buffer_header` resolves
      the filename directly via `buffer.resolve_file_path` again (the synthetic
      file carries the path).

**Dependency:** builds on A8 (snapshot loader, write-path map, open-to-edit).

**Review cleanups (post-A9).** A max-effort code review applied four refinements:
(1) `ProjectDiff::active_path` no longer treats the synthetic file's repo path as
worktree-relative — the synthetic (`Historic`) buffer is routed through
`worktree_project_path_for_buffer`, which now delegates to
`GitStore::repository_and_path_for_buffer_id` (correct when repo root ≠ worktree
root, and dedups the old multibuffer path-key scan); (2) `open_staged_index_snapshot`
builds the index rope once (normalized via `LineEnding`/`new_normalized`, matching
`commit_view::build_buffer`) instead of materializing it twice; (3) the registered
and snapshot diff-event handlers share one `write_hunk_staging_to_index` helper;
(4) the stale "file-less" comments were corrected to "not registered in the buffer
store" (the buffer carries a synthetic file post-A9).

- [x] **Followup — highlight deleted (red) lines.** A9's syntax highlighting only
      reached the buffer (context + added/green lines). Deleted/red lines render from
      the diff **base-text buffer** — a separate `language::Buffer` whose language is
      set only by `BufferDiff::language_changed`, never by `set_base_text` (whose
      `language` arg feeds the diff options only). `open_staged_index_snapshot` called
      `set_base_text` but not `language_changed`, so the HEAD base buffer stayed
      language-less and deleted lines rendered uncolored. Fixed by calling
      `diff.language_changed(language, Some(language_registry), cx)` on the primary
      diff before `set_base_text`, matching the worktree-diff path
      (`git_store.rs:1242`). This was the deferred "Q#10" finding from the post-A9
      review, originally misjudged as low-risk on the wrong assumption that
      `set_base_text` highlighted the base. *(Test:
      `test_staged_index_snapshot_base_text_buffer_has_detected_language` asserts the
      base buffer carries the detected language; verified RED before the fix. Commit
      `28ae6d7db9`.)*

- [ ] **Followup — cache the staged snapshot.** `open_staged_index_snapshot` mints
      a fresh buffer + diff on every call, so the `RefreshReason`-driven rebuild
      defeats `ProjectDiff::refresh`'s `entity_id()` skip-guard (the Unstaged/Head
      paths reuse cached `self.diffs` entities). Each git status refresh re-loads
      HEAD+index text and recomputes both diffs per staged file. Bounded to the
      staged-file count and off the render thread, but wasteful — key a cache on
      `(repo_id, repo_path, head_oid, index_oid)` and reuse the entities when the
      index/HEAD blobs are unchanged. (Grammar load is already registry-cached.)

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
