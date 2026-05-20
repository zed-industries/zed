# PRD: Codex-style staged/unstaged/branch diff filtering

Status: Proof-of-concept · Phase 2 revision (post first-build feedback) ·
Related: issue #26560, prior PRs #36646 / #46541 / #48792

## Problem Statement

When I work with git in Zed, the project diff view shows my staged and unstaged
changes interleaved into a single combined buffer. I cannot ask Zed to show me
*only* what is staged, *only* what is not yet staged, or how my branch differs
from its base. When a single file has some hunks staged and some not, the
combined view makes it genuinely hard to tell at a glance what is going where.

This is especially painful when reviewing the output of an AI agent: I like to
stage the changes I have accepted and leave the rest unstaged, but Zed gives me
no clean way to see "just the unstaged remainder." Today I fall back to the
terminal or another editor for staging, which defeats the point of Zed's git
integration.

The git panel has the same gap: it groups files by tracked/untracked status,
never by staging state, so I cannot scan a "Staged" list versus an "Unstaged"
list the way I can in other git clients.

## Solution

Two coordinated additions, both opt-in and non-destructive to current behavior:

1. **A diff filter dropdown** in the project diff toolbar that switches the view
   between four categories — **Uncommitted** (today's combined view, the
   default), **Staged**, **Unstaged**, and **Branch** (diff against the merge
   base). Selecting a category recomputes the diff so I see exactly that slice
   of my changes.

2. **An opt-in git panel grouping mode** that groups files into **Staged** and
   **Unstaged** sections (plus Conflicts), with hover-revealed `+`/`-` buttons
   to stage and unstage. The existing status grouping (Conflicts / Tracked /
   Untracked, with checkboxes) is untouched and remains the default.

The result: explicit, switchable categories — the model the Codex app uses —
so I can review and stage with full clarity without leaving Zed.

## User Stories

1. As a developer reviewing my work, I want to switch the project diff view to
   show only staged changes, so that I can verify exactly what my next commit
   will contain.
2. As a developer reviewing my work, I want to switch the project diff view to
   show only unstaged changes, so that I can see what I have not yet committed
   to including.
3. As a developer reviewing a feature branch, I want to switch the project diff
   view to show the diff against the branch's merge base, so that I can review
   the whole branch as a unit.
4. As a developer, I want the project diff view to default to the existing
   combined "Uncommitted" view, so that my current workflow is unchanged unless
   I choose otherwise.
5. As a developer, I want the selected diff filter to persist across editor
   restarts, so that I do not have to reselect it every session.
6. As a developer with a partially-staged file, I want that file to appear in
   both the Staged and the Unstaged filter, so that I can review each side
   independently.
7. As a developer in the Staged filter, I want unstaging a hunk to remove just
   that hunk from view (and remove the file only when its last staged hunk is
   gone), so that the view always reflects reality.
8. As a developer in the Unstaged filter, I want staging a hunk to remove just
   that hunk from view (and remove the file only when its last unstaged hunk is
   gone), so that the view always reflects reality.
9. As a developer in the Branch filter, I want hunk-level staging controls
   hidden, so that I treat it as a read-only review of the branch.
10. As a developer, I want the diff filter dropdown to clearly show which
    category is currently active, so that I am never confused about what I am
    looking at.
11. As a developer reviewing an AI agent's output, I want to stage the changes I
    accept and then switch to the Unstaged filter, so that I see only the
    agent's remaining changes that I have not yet approved.
12. As a developer, I want the existing "Changes since <branch>" entry point to
    simply land me in the Branch filter, so that there is one consistent model
    for branch diffs.
13. As a developer, I want the Uncommitted filter's diff view — including the
    per-file header checkbox — to behave exactly as it does today, so that
    nothing I rely on regresses.
14. As a developer, I want to enable an opt-in git panel mode that groups files
    into Staged and Unstaged sections, so that I can scan staging state the way
    I do in other git clients.
15. As a developer, I want the git panel's default grouping (Conflicts /
    Tracked / Untracked) to remain unchanged, so that the new mode is purely
    additive.
16. As a developer, I want to switch the panel grouping mode from a "Sort by"
    submenu (with radio options "By Status", "By Path", "By Staging") rather
    than a standalone toggle, so that the grouping/sort decision lives in one
    consolidated control. The currently-active option is shown inline on the
    parent menu entry, and "By Path" is disabled when Tree View is active.
17. As a developer in the staging-grouped panel mode, I want a `-` button to
    appear on hover on each row under the Staged section, so that I can unstage
    a file unambiguously. The same hover-revealed `-` appears on the **Staged
    section header** to unstage every file in that section in one click.
18. As a developer in the staging-grouped panel mode, I want a `+` button to
    appear on hover on each row under the Unstaged section, so that I can stage
    a file unambiguously. The same hover-revealed `+` appears on the **Unstaged
    section header** to stage every file in that section in one click.
19. As a developer with a partially-staged file, I want it listed under both the
    Staged and the Unstaged section of the grouped panel, so that I can act on
    either side.
20. As a developer, I want the +/- buttons to appear only on hover, so that the
    panel stays visually calm.
21. As a developer, I want panel grouping to be independent of flat/tree view,
    so that I can combine staging grouping with a directory tree if I want.
22. As a developer, I want a Conflicts section to remain visible in the
    staging-grouped mode, so that merge conflicts are never hidden.
23. As a developer, I want the diff filter dropdown and the panel grouping mode
    to be independent *settings* — changing one does not silently change the
    other. The only coupling is at the moment of a row click in staging-grouped
    mode (see story 32), which is an explicit user action, not a state-sync.
24. As a developer staging from the grouped panel, I want a file to move between
    the Staged and Unstaged sections as its staging state changes, so that the
    panel stays accurate.
25. As a developer, I want Staged/Unstaged categories to never show a "partially
    staged" indicator, because each already shows only one side of the change.
26. As a maintainer evaluating this PoC, I want all new logic encapsulated in
    testable modules, so that I can verify correctness without running the UI.
27. As a developer switching from the Branch filter back to Uncommitted, I want
    the staging controls restored immediately, so that I can resume staging
    without reopening the diff.
28. As a developer changing the diff filter, I want the toolbar (and its
    dropdown) to stay visible for every category, so that I can switch again
    without losing the control.
29. As a developer with a partially-staged file in the staging-grouped panel, I
    want selecting, scrolling to, or opening its row under one section to act on
    exactly that section's row, so that the duplicate listing never causes the
    wrong action.
30. As a developer in the Staged filter, I want each visible hunk to be
    unstage-able, and in the Unstaged filter stage-able, so that the staging
    controls actually work within the filtered views.
31. As a developer selecting the Branch filter, I want a clear error if the
    default branch cannot be determined, so that I am not left looking at a
    silently broken or empty view.
32. As a developer in the staging-grouped panel, I want clicking a row to
    automatically switch the diff filter to match that row's section — Staged
    row → Staged filter, Unstaged row → Unstaged filter — so that the diff I
    open shows exactly the side I clicked on. In status-grouped mode the
    filter is left alone, unless the current filter would not contain the
    clicked file (e.g. filter=Staged but the file is fully unstaged), in which
    case it falls back to Uncommitted. The Branch filter always exits on a
    panel row click, regardless of mode.
33. As a developer with a partially-staged file in the staging-grouped panel,
    I want each section row to display the diff stats sourced from **that
    side's numstat only** — the Staged row shows the result of
    `git diff --numstat --cached HEAD`, the Unstaged row shows the result of
    `git diff --numstat` (without `--cached`). The two values may legitimately
    coincide for a given file (e.g. both sides happen to be `+1 −0`); what
    matters is that each row's data source is the matching single-sided
    numstat, not the combined HEAD→worktree numstat reused for both.
34. As a developer in the staging-grouped panel, I want hovering a row or a
    section header (Staged or Unstaged) to reveal the `+`/`-` button and keep
    it visible while my cursor is over the button itself — moving onto the
    button must not hide it. The Conflicts section header is unchanged from
    today.

## Implementation Decisions

### Module M1 — Diff filter resolver (`project` crate)

A deep, pure module. Given the active `DiffBase` and the repository's status
entries, it resolves which files belong in the view and which `DiffType` each
file's diff should use. No UI, no async.

- Extend the existing `DiffBase` enum with two new variants: `Staged` and
  `Unstaged`, alongside the current `Head` (combined uncommitted) and `Merge`
  (branch). This reuses the existing `set_diff_base` → `DiffBaseChanged` event →
  `refresh()` machinery and the existing database persistence of `DiffBase`.
- Resolution rules:
  - `Head` → all files with uncommitted changes; combined diff (current path).
  - `Staged` → files whose index differs from `HEAD`; `DiffType::HeadToIndex`.
  - `Unstaged` → files whose worktree differs from the index, plus untracked
    files; `DiffType::HeadToWorktree`.
  - `Merge` → tree diff against the merge base (current branch-diff path).
- A partially-staged file is included in **both** the `Staged` and `Unstaged`
  result sets. Inclusion is content-driven: a file appears in a filter only
  while it still has at least one hunk for that filter.

### Module M2 — Single-sided diff loader (`project` crate)

A new `Project` method (or pair of methods) that produces a staged-only or
unstaged-only `BufferDiff`, sitting alongside the existing
`open_uncommitted_diff` and `open_diff_since`.

A `BufferDiff` diffs its `base_text` against a buffer snapshot, and that
snapshot is the **live worktree** state. A naive `HEAD → buffer` diff therefore
shows *all* uncommitted changes, so for a partially-staged file a "staged-only"
diff would leak unstaged worktree hunks. The loader must therefore:

- For the **Staged** view, compare `HEAD` against the **index** content — i.e.
  supply an index-backed target snapshot, not the live worktree buffer.
- For the **Unstaged** view, compare the **index** content against the worktree
  buffer.

Equivalently, this may be implemented by filtering hunks using the
staged/unstaged status `BufferDiff` already tracks via its `secondary_diff`,
rather than computing a fresh base/target pair. Whichever mechanism is chosen,
it must guarantee the Staged view never contains an unstaged hunk and the
Unstaged view never contains a staged-only hunk.

The loader must also set each hunk's `DiffHunkSecondaryStatus` correctly. The
staging controls derive Stage-vs-Unstage availability from that status, where
`HasSecondaryHunk` means the hunk is **unstaged** (stage-able) and
`NoSecondaryHunk` means it is **staged** (unstage-able). A standalone
`BufferDiff` defaults every hunk to `NoSecondaryHunk` (staged), so without
correction the Unstaged view's hunks would be treated as staged and offered
"Unstage" rather than "Stage". The loader must:

- In the **Staged** view, mark hunks as staged (`NoSecondaryHunk`) so they are
  unstage-able.
- In the **Unstaged** view, mark hunks as unstaged (`HasSecondaryHunk`) so they
  are stage-able.

Correct hunk status only fixes which button is *shown* — the staging *action*
must also work. `stage_or_unstage_hunks` is a no-op that returns `None` when the
`BufferDiff` has no `secondary_diff`, and the editor discards that `None`. So a
freshly computed base/target `BufferDiff` would display the right button while
clicking it silently fails to touch the index. The loader must therefore retain
(or provide) a real `secondary_diff` — or an equivalent staging backend — so
that staging and unstaging within the Staged and Unstaged filters actually
mutate the git index. This makes the secondary-diff route the safer choice over
a fresh base/target pair.

The buffer-loading path (`load_buffer` / `load_buffers`) selects the loader
based on the resolved `DiffType` from M1.

### Module M3 — Diff filter dropdown (`git_ui`, project diff toolbar)

A dropdown control added to `ProjectDiffToolbar` using Zed's existing dropdown
primitive. Its displayed selection is derived from the current `DiffBase`;
choosing an option updates the `DiffBase`. Default selection is `Uncommitted`.

Switching `DiffBase` alone is **not sufficient**:

- `ProjectDiffToolbar` currently attaches only when the diff base is `Head`. It
  must remain attached for **all four** filter values, otherwise the dropdown
  (which the toolbar hosts) would disappear the moment a non-`Uncommitted`
  filter is selected.
- The per-hunk staging controls and the editor addon (`GitPanelAddon` vs
  `BranchDiffAddon`) are currently configured **once at `ProjectDiff`
  construction** from the initial `DiffBase`. They must be **re-applied on every
  `DiffBaseChanged`**, not only when buffers refresh. Every transition among
  Uncommitted / Staged / Unstaged / Branch must leave the toolbar visible and
  the staging controls correctly configured: enabled for
  Uncommitted/Staged/Unstaged, hidden for Branch.
- Selecting **Branch** is not a synchronous `set_diff_base`. `DiffBase::Merge`
  requires a `base_ref`, which must be resolved asynchronously from the active
  repository's default branch — the same path `new_with_default_branch`
  already uses via `repo.default_branch(true)`. Selecting Branch must: resolve
  the default branch asynchronously; on success apply
  `set_diff_base(Merge { base_ref })`; on failure (no active repository, or the
  default branch cannot be determined) surface a user-visible error and leave
  the previously selected filter active. The dropdown must not show Branch as
  selected until the base ref has resolved.

**Click → filter coupling (added in revision).** The diff filter can also be
switched from the git panel: when a user clicks a row in staging-grouped mode,
the panel computes a target `DiffBase` and opens the diff under that base
(story 32). The rule:

Evaluated in order — the **first matching clause wins**. The Branch override
runs first so that no later clause can land back on `Merge`.

1. **Branch override (any mode).** If the current `DiffBase` is `Merge`, the
   click always exits Branch:
   - Staging-grouped, Staged row → `Staged`.
   - Staging-grouped, Unstaged row → `Unstaged`.
   - Staging-grouped, Conflicts row → `Head`. ("Unchanged" cannot apply
     here — it would leave the user on `Merge`, which contradicts story
     32. Conflicts has no staging side, so `Head` is the only sane
     fallback.)
   - Status-grouped, any row → `Head`.
2. **Staging-grouped mode (current base ≠ `Merge`).** Staged row → `Staged`;
   Unstaged row → `Unstaged`; Conflicts row → target unchanged (current
   `DiffBase`, which is now guaranteed not to be `Merge`).
3. **Status-grouped mode (current base ≠ `Merge`).** Target = current
   `DiffBase`, unless the current filter would not contain the clicked file
   (e.g. filter=`Staged` but the row is fully unstaged), in which case
   target = `Head`.

**`ProjectDiff::deploy_at` must carry the target `DiffBase`, not rely on a
separate workspace dispatch.** Today the function (`project_diff.rs:238-299`)
does two things that break the naive "dispatch filter switch, then open
diff" sequence:

1. `items_of_type::<ProjectDiff>(cx).find(|item| matches!(item.read(cx).diff_base(cx), DiffBase::Head))`
   (`project_diff.rs:254-256`) finds an existing view *only* when its base is
   `Head`. A Staged-row click that needs to land on `DiffBase::Staged`
   would either match this `Head`-only filter (wrong base reused) or fall
   through to create a fresh view.
2. The fresh-view path calls `Self::new(...)` (`project_diff.rs:371-380`),
   which hard-codes `DiffBase::Head` for the new `BranchDiff`.

The revision therefore requires three coupled changes:

- Extend `deploy_at`'s signature to take `target_base: DiffBase`. Find an
  existing item by `diff_base(cx) == target_base` **and** matching
  repository; activate it if found. Otherwise create a new view under
  `target_base`. **Never mutate an existing view's base to match the
  target** — a view at `DiffBase::Head` and a view at `DiffBase::Staged`
  are distinct items (the A5 test asserts that a Staged-row click does not
  reuse an existing `Head` view by retargeting it).
- Extend `ProjectDiff::new` (and `new_impl`) to take a starting `DiffBase`,
  threaded into `BranchDiff::new(target_base, ...)`. Drop the hard-coded
  `DiffBase::Head` at `project_diff.rs:378`. Existing callers that want
  Uncommitted pass `DiffBase::Head` explicitly.
- Apply the same change to `deploy_at_project_path` (`project_diff.rs:301`)
  so external callers (e.g. agent panel) can target a base too. The default
  for that path is `DiffBase::Head`.

The git panel row click then calls
`ProjectDiff::deploy_at(workspace, Some(entry), target_base, ...)` with the
target computed per the rule above — no separate workspace dispatch. The
click handler also sets `selected_entry` synchronously (using the clicked
row's index) so the subsequent `EditorEvent::SelectionsChanged`
re-resolution (see M4) finds the panel already on the right
`(section, repo_path)` and is a no-op.

### Module M4 — Panel section grouping (`git_ui`, git panel)

A deep, pure module. Given the repository's status entries and a grouping mode,
it produces the sectioned entry list (section headers + entries in order). It
encapsulates both the existing status grouping and the new staging grouping
behind one interface.

- Add new `Section` variants `Staged` and `Unstaged` (Conflicts retained).
- Keep the existing `GitPanelSettings` fields `group_by` (`status` default /
  `staging`) and `sort_by_path` (`bool`) as the storage layer — they remain
  independent and persistent.
- **Replace** the separate `ToggleGroupBy` and `ToggleSortByPath` toggle
  actions with a **single parameterized action** `git_panel::SetSortBy
  { mode: SortBy }` where `SortBy ∈ { Status, Path, Staging }`. The action
  handler maps each value to the underlying `(group_by, sort_by_path)` pair:
  `Status → (Status, false)`, `Path → (Status, true)`, `Staging → (Staging, *)`
  (sort_by_path is left as-is). This single action replaces both legacy
  toggles in the command palette and keybindings.
- **Menu surfacing.** The panel kebab menu replaces the two flat toggle entries
  with one **"Sort by …"** submenu containing three radio items mapped to
  `SetSortBy(Status)`, `SetSortBy(Path)`, `SetSortBy(Staging)`. The parent
  entry shows the active option inline (e.g. `Sort by: Status ▸`). When
  Tree View is active the **"By Path"** item is disabled with a tooltip
  ("Switch to Flat View to sort by path"). The standalone **"Tree View"**
  toggle entry is unchanged.
- Grouping mode is orthogonal to the flat/tree view setting.
- Because a partially-staged file appears under **both** the Staged and the
  Unstaged section, the panel can no longer identify a row by `RepoPath` alone.
  The current entry index is a `HashMap<RepoPath, _>`, so a duplicated path's
  second occurrence would overwrite the first and break selection, scroll-to,
  open-diff, and the header controls. Entries must be keyed by
  `(section, repo_path)`. Section staged/unstaged counts must be derived from
  the source status entries, not by counting duplicated rendered rows.
- **Per-section diff stats (story 33).** A single `DiffStat` per file
  (HEAD→worktree combined) is insufficient for staging-grouped rows. Each
  `GitStatusEntry` must cache three stats, all refreshed in the same status
  refresh path:
  - `diff_stat_combined` — `git diff --numstat --no-renames HEAD`
    (today's stat; used in Status-grouped mode and as the file-header total).
  - `diff_stat_staged` — `git diff --numstat --no-renames --cached HEAD`
    (used when rendering a row under `Section::Staged`).
  - `diff_stat_unstaged` — `git diff --numstat --no-renames`
    (used when rendering a row under `Section::Unstaged`).

  The renderer picks the stat by section. Always computing all three avoids
  flicker on mode toggle and keeps the renderer trivial; the two extra
  numstats are bounded by repo size, not by file count, so the overhead is
  small relative to the existing status refresh.
- **Selection identity for partially-staged files (story 29).**
  `GitPanel::select_entry_by_path` must be updated to:
  1. Accept an optional `preferred_section: Option<Section>` arg. Internal
     callers that know the section (e.g. the row click handler) pass `Some`.
     The `handle_editor_event::SelectionsChanged` path passes `None`.
  2. Compute a **target section** before any stickiness check:
     `target: Option<Section> = preferred_section.or_else(|| section_from_diff_base())`,
     where `DiffBase::Staged → Some(Section::Staged)`,
     `DiffBase::Unstaged → Some(Section::Unstaged)`, and
     `DiffBase::Head` / `DiffBase::Merge → None`.
  3. Be **narrow-sticky by `(section?, path)`**: leave the selection alone
     when **both**
     - `selected.repo_path == target_path`, *and*
     - `target.map_or(true, |s| selected.section == s)` — i.e. when
       `target` is `Some(s)`, the selected row's section must match `s`;
       when `target` is `None` (Head/Merge filter, no `preferred_section`),
       the section is left unconstrained and the current duplicate row is
       preserved.

     Sticky-by-path alone is too coarse — it would block legitimate
     cross-section moves for the same partially-staged file when the
     filter or `preferred_section` changes the desired side.
  4. Otherwise, re-resolve: when `target = Some(s)`, pick the entry whose
     `repo_path` matches *and* whose `section == s`; when `target = None`,
     fall back to the existing first-match heuristic (preserves today's
     behaviour for status-grouped mode and for `Head`/`Merge` filters).

### Module M5 — Staging affordance (`git_ui`)

The hover-revealed `+`/`-` buttons used in the staging-grouped panel mode,
applied at **two levels**:

- **Row level.** Each Staged-section row has a hover-revealed `−` button;
  each Unstaged-section row has a hover-revealed `+` button. The existing
  checkbox affordance is kept unchanged for the status-grouped mode and for
  the Conflicts section header in either mode.
- **Section-header level (added in revision, stories 17/18).** The Staged
  section header has a hover-revealed `−` button that unstages every file in
  the section in one click; the Unstaged section header has a hover-revealed
  `+` button that stages every file in the section. The whole-row click on
  the header **does not toggle** in staging-grouped mode — only the button
  itself is the click target. The Conflicts section header still uses the
  existing checkbox + whole-row toggle.

**Wrapper-hitbox constraint (story 34, bug A1 in the followups appendix).**
The `visible_on_hover` mechanism relies on the parent's group-hover state. A
`div().occlude()` wrapper around the button breaks this: `.occlude()` sets
`HitboxBehavior::BlockMouse`, the hit-test loop breaks before reaching the
parent row's hitbox, the row's `group_hover` flips off, and the button hides
again. The staging-control wrapper (both per-row and per-header) **must not
use `.occlude()`** — the button's own `on_click` calls `cx.stop_propagation()`,
which is sufficient to prevent the row's open-diff handler from firing.

### Cross-cutting decisions

- The diff filter dropdown and the panel grouping mode are independent
  **settings**; neither *silently* drives the other. The only coupling is at
  the moment of an explicit row click in staging-grouped mode, which
  intentionally switches the filter to match the clicked section (M3 + story
  32). Changing the grouping mode does not change the filter, and changing
  the filter does not change the grouping mode.
- The Uncommitted filter's diff view is unchanged, including the per-file header
  checkbox — no new status label is added there.
- "Last turn" / agent-turn diffs are not a `DiffBase` and are excluded entirely
  (no agent-crate dependency).
- Build order: Phase 1 = M1 + M2 + M3 (diff-view dropdown end to end);
  Phase 2 = M4 + M5 (panel grouping). A Phase 2 revision (this revision)
  addresses four bugs and two design refinements surfaced in the first PoC
  build — see the Implementation followups appendix.

## Testing Decisions

A good test here exercises **external behavior**, not internal structure: it
constructs inputs (status entries, a selected `DiffBase` or grouping mode) and
asserts the observable output (the resolved file/`DiffType` set; the sectioned
entry list; the rendered control state) — never private fields or call order.

All five modules are tested:

- **M1 — Diff filter resolver:** unit tests. For each `DiffBase`, build a set of
  status entries (covering staged-only, unstaged-only, partially-staged,
  untracked, and conflicted files) and assert the resolved file set and
  per-file `DiffType`. Net-new coverage — the branch-diff code currently has no
  tests.
- **M4 — Panel section grouping:** unit tests. For each grouping mode, build
  status entries and assert the section headers and entry order, including a
  partially-staged file appearing under both Staged and Unstaged. Acceptance
  test: with a partially-staged (duplicated) file, assert that selecting,
  scrolling to, and opening the diff of the row under one section resolves to
  that row and not the other, and that each section's staged/unstaged counts
  remain correct. Prior art: `test_bulk_staging`,
  `test_bulk_staging_with_sort_by_paths`, and the tree-view tests in the git
  panel, which build fake repositories and assert on the entries list.
- **M2 — Single-sided diff loader:** integration test against a fake/temporary
  git repository. Acceptance test: a partially-staged file with a **dirty
  worktree** — assert the staged-only `BufferDiff` contains exactly the staged
  hunks and **no unstaged worktree hunks**, and the unstaged-only `BufferDiff`
  the converse. Also assert each hunk's `DiffHunkSecondaryStatus`: staged-only
  loader hunks are `NoSecondaryHunk` (unstage-able), unstaged-only loader
  hunks are `HasSecondaryHunk` (stage-able). Acceptance test for the action
  path: stage a hunk in the Unstaged filter and unstage a hunk in the Staged
  filter, and assert that the git index actually changes and the hunk
  disappears from the filtered view. Prior art: the existing `test_open_diff`
  git panel test.
- **M3 / M5 — UI controls:** light UI tests using `gpui::test` and the visual
  test context. M3: the dropdown reflects the current `DiffBase` and selecting
  an option updates it; acceptance test for transitions — switching to Branch
  and back to Uncommitted must keep the toolbar visible and **restore the
  staging controls** (hidden under Branch, enabled again under Uncommitted),
  covering all Head/Staged/Unstaged/Branch transitions. Acceptance test for the
  Branch failure path: when the default branch cannot be resolved, an error is
  surfaced and the previously selected filter remains active. M5: hovering a
  grouped-panel row reveals the `+`/`-` button and clicking it changes the
  file's staging state. Prior art: the existing async git panel tests.

**Phase 2 revision tests (covering the followups appendix).** Each followup
gets a dedicated assertion:

- **A1 / story 34** — UI test: hover a Staged-mode row, assert the
  `−` button is rendered visibly; move the simulated cursor onto the
  button's bounds, assert the button is **still** rendered visibly
  (`HitboxBehavior::BlockMouse` regression guard).
- **A2 / stories 17, 18** — UI test: in staging-grouped mode, the Staged
  header renders a hover-revealed `−` (not a checkbox), the Unstaged header
  renders a hover-revealed `+`. Clicking each invokes the matching bulk
  action and updates the section's contents. The Conflicts header still
  renders the existing checkbox.
- **A3 / story 33** — M4 unit test extended: for a partially-staged file,
  the Staged-section row's rendered `DiffStat` matches the staged-only
  numstat (`git diff --numstat --cached HEAD`) and the Unstaged-section
  row's `DiffStat` matches the unstaged-only numstat (`git diff --numstat`).
  Use a fixture where the staged-side and unstaged-side numstats
  **intentionally differ** (e.g. staged `+3 −0`, unstaged `+1 −2`) so the
  previous bug — reusing the combined `+4 −2` for both rows — would
  visibly fail. **Do not assert mere inequality** of the two rendered
  stats; each value is asserted against its single-sided source.
  Legitimate coincidence (both sides happen to be `+1 −0`) must not
  flake the test.
- **A4 / story 29 (reinforced)** — Two M4 acceptance tests.
  *Row-click stickiness:* click the Unstaged row of a partial file; after
  the synthetic `EditorEvent::SelectionsChanged` is dispatched,
  `git_panel.selected_entry` still points to the Unstaged row. Repeat
  converse for the Staged row.
  *Cross-section move on filter change:* start with selection on the
  Staged row of `partial.rs`; flip the active `DiffBase` to `Unstaged`
  (no `preferred_section`); call `select_entry_by_path(partial.rs, None)`
  → selection must move to the Unstaged row of the same file, *not*
  remain stuck on Staged. This guards against a path-only stickiness
  regression.
- **A5 / story 32** — M3 integration tests on `ProjectDiff::deploy_at`:
  - *Fresh-view target:* clicking a Staged row in staging-grouped mode
    while no `ProjectDiff` exists creates a view whose
    `diff_base(cx) == DiffBase::Staged`. Repeat for Unstaged.
  - *No retargeting:* with an existing `DiffBase::Head` view already
    open, a Staged-row click must **not** reuse that view as the Staged
    target — it activates or creates a separate `DiffBase::Staged` view.
    The existing `Head` view's `diff_base` must remain `Head` after the
    click. Repeat for Unstaged.
  - *Status-grouped, current base contains the file:* target equals the
    current base (no change).
  - *Status-grouped, current base does not contain the file:* target
    falls back to `Head`.
  - *Branch override beats everything:* with current base = `Merge`,
    every row-click variant produces a non-`Merge` target:
    staging-grouped Staged/Unstaged rows → `Staged`/`Unstaged`,
    staging-grouped Conflicts row → `Head` (the "Conflicts → unchanged"
    clause is **not** allowed to apply here), status-grouped any row →
    `Head`.
- **A6 / story 16** — M4 unit test: dispatching
  `SetSortBy(Status)` / `(Path)` / `(Staging)` updates `group_by` and
  `sort_by_path` to the expected pair. Menu-render test: the kebab menu
  shows a "Sort by …" submenu with the active option inline, and "By
  Path" disabled when Tree View is active.

## Out of Scope

- The "Last turn" / agent-turn diff category from the Codex app — excluded to
  avoid an agent-crate dependency.
- A **per-file** staged/unstaged scoping toggle within the Uncommitted filter
  (showing only one side of a single partially-staged file while others stay
  combined). Deferred as a possible Phase 3; it requires mixed `DiffType`s
  within one multibuffer and a per-file state dimension.
- Line-level (sub-hunk) staging.
- A branch base-ref picker — the Branch filter reuses Zed's existing
  default-branch detection.
- Any change to the Uncommitted filter's existing diff-view behavior or UI.
- Replacing checkboxes with `+`/`-` buttons in the status-grouped panel mode or
  anywhere in the diff view.

## Further Notes

- **Prior art:** PR #48792 ("Add support for staged and unstaged changes in
  project diff view", open) already adds a select box for staged/unstaged diff
  views. It should be studied before and during Phase 1 — compare its approach
  to the `DiffBase`-extension plan here, and reference it when sharing findings
  with the Zed team.
- This is a proof-of-concept whose secondary goal is to learn the git/diff area
  before opening a discussion with the Zed team. Two findings are expected to be
  worth raising explicitly: the absence of a single-sided-diff `Project` API
  (M2), and the UX of entries disappearing as they leave the active filter —
  the design concern the team has previously flagged.
- The existing "Changes since <branch>" action naturally becomes equivalent to
  selecting the Branch filter; it is left in place as a shortcut into that
  filter rather than being removed.

## Implementation Followups (Phase 2 revision)

Surfaced after the first PoC build. Each entry pairs an observed bug or UX
refinement with the redesign that addresses it.

### A1 — `+`/`-` button disappears when the cursor enters it (M5)

**Observed.** Hovering a row in staging-grouped mode reveals the `+` / `−`
button as expected, but moving the cursor *onto* the button hides it again
(only the tooltip remains). The button is therefore unclickable.

**Root cause.** The staging-control wrapper at `git_panel.rs:6459` (per row)
applies `.occlude()`, which sets `HitboxBehavior::BlockMouse`. `Window::hit_test`
(`gpui/src/window.rs:919-921`) iterates hitboxes in reverse and `break`s on
the first `BlockMouse`, so the parent row's hitbox never enters
`hit_test.ids`. The row's group-hover state flips off, and the button — which
is `invisible()` by default and only becomes visible while the group is
hovered (`ui/src/traits/visible_on_hover.rs:13-16`) — hides itself.

**Fix.** Remove `.occlude()` from the staging-control wrapper at row level
(and from the equivalent directory-row wrapper). The button's `on_click`
already calls `cx.stop_propagation()`, which prevents the row's `open_diff`
from firing. Apply the same wrapper rule to the new section-header buttons
(A2): they must not use `.occlude()` either. Documented as the wrapper-hitbox
constraint in M5.

### A2 — Section headers still render checkboxes in staging-grouped mode (M5)

**Observed.** In staging-grouped mode, the **Staged** and **Unstaged** section
headers still display the existing bulk checkbox affordance, rather than the
section-level `+` / `−` button the user expects to mirror the per-row UI.

**Root cause.** `render_list_header` (`git_panel.rs:6109-6164`) unconditionally
renders a `Checkbox`; it does not branch on `group_by`.

**Fix.** Branch the header renderer on `group_by`:

- Status-grouped mode: render the existing checkbox + whole-row toggle.
- Staging-grouped mode:
  - Staged header → hover-revealed `−` (Unstage All in section).
  - Unstaged header → hover-revealed `+` (Stage All in section).
  - Conflicts header → unchanged checkbox + whole-row toggle.
- In staging-grouped mode the whole-row `on_click` toggle is removed for the
  Staged and Unstaged headers; only the button is the click target.

Documented as story 17/18 (extended) and in M5.

### A3 — Partially-staged file rows share the wrong (combined) `+N −M` data source (M4)

**Observed.** A file that has some hunks staged and some unstaged is correctly
listed under both sections (story 19), but each row sources its `+N −M` from
the **same** combined HEAD→worktree numstat. The two rows therefore display
the same numbers as a consequence of the shared data source — not because
the two sides happen to have the same numstat. Equal numbers are not by
themselves a bug (both sides can legitimately be `+1 −0`); the bug is the
shared source.

**Root cause.** `entry_for_section` (`git_panel.rs:755-765`) clones the
`GitStatusEntry`, including the single `diff_stat` field, when duplicating a
partial file into both sections. The underlying numstat is produced once via
`git diff --numstat HEAD` (`git/src/repository.rs:2127`). Only one `DiffStat`
is plumbed end-to-end: `project::StatusEntry` (`crates/project/src/git_store.rs:220-225`)
serializes a single `diff_stat`, the proto message `StatusEntry`
(`crates/proto/proto/git.proto:319-326`) has only `diff_stat_added` /
`diff_stat_deleted`, and `StatusEntry::to_proto` / `TryFrom<proto::StatusEntry>`
(`git_store.rs:228-268`) move that single value across the wire.

**Fix.** Cache three numstats per file, plumbed all the way from `git diff`
through to the renderer:

1. **Repository layer (`crates/git/src/repository.rs`):** the existing
   `diff_stat()` method takes only path prefixes. Either add two siblings
   (`diff_stat_staged()` running with `--cached HEAD`, `diff_stat_unstaged()`
   running with no `--cached`), or extend the signature with a `kind:
   DiffStatKind { Combined, Staged, Unstaged }` selector.
2. **`project::StatusEntry` (`git_store.rs:220-225`):** add
   `diff_stat_staged: Option<DiffStat>` and `diff_stat_unstaged: Option<DiffStat>`
   alongside the existing `diff_stat` (renamed in spirit to
   `diff_stat_combined`; keep the field name for proto compat if helpful).
3. **Proto (`crates/proto/proto/git.proto:319-326`):** add four optional
   fields — `diff_stat_staged_added` (field 6), `diff_stat_staged_deleted`
   (7), `diff_stat_unstaged_added` (8), `diff_stat_unstaged_deleted` (9).
   Optional + new field numbers keep the wire format backwards compatible
   with collab versions that do not yet emit them.
4. **`StatusEntry::to_proto` / `TryFrom<proto::StatusEntry>`
   (`git_store.rs:228-268`):** serialize and deserialize all three stats.
   Missing staged/unstaged values from older peers fall back to `None` and
   the renderer treats them as "stat not yet known" (same handling as today
   for the combined stat).
5. **Repository update diffing (the path that recomputes which entries
   changed and pushes downstream notifications):** entries whose
   combined/staged/unstaged stat changed must mark dirty so the panel
   refreshes. The existing dirty-detection path that compares old vs new
   `diff_stat` must be extended to the two new fields.
6. **`git_ui::GitStatusEntry` (`git_panel.rs:617-624`):** mirror the three
   fields. `entry_for_section` (`git_panel.rs:755-765`) assigns the
   section-matching stat to the duplicated entry's display field (or the
   renderer reads the matching field directly by `Section`).

Renderer rule: Staged rows display the staged-only stat; Unstaged rows
display the unstaged-only stat; Status-grouped rows display the combined
stat (unchanged). Documented in M4 and story 33.

### A4 — Clicking a row in one section selects the other section's row (M4)

**Observed.** With a partially-staged file, clicking the row in the
**Unstaged** section visibly selects the **Staged** section's row instead
(and vice versa). The diff that opens corresponds to the wrong section.

**Root cause.** `GitPanel::on_click` for a row sets `selected_entry = ix`
correctly. `open_diff` then opens the project diff, which fires an editor
`EditorEvent::SelectionsChanged { local: true }`. `ProjectDiff::handle_editor_event`
(`project_diff.rs:778-790`) calls `git_panel.select_entry_by_path(project_path)`,
and the path-based resolver at `git_panel.rs:1145-1151` always picks
`Section::Staged` whenever `status.staging().has_staged()` is true. The
explicit Unstaged selection is overwritten within the same tick.

**Fix.** Make `select_entry_by_path` narrow-sticky and filter-aware:

1. Add an optional `preferred_section: Option<Section>` arg. Internal panel
   callers that know which side they mean pass `Some`; the editor-event
   re-resolution passes `None`.
2. Compute the **target section** *before* the sticky check:
   `target: Option<Section> =
   preferred_section.or_else(|| section_from_current_diff_base())`
   where `DiffBase::Staged → Some(Section::Staged)`,
   `DiffBase::Unstaged → Some(Section::Unstaged)`, and
   `DiffBase::Head` / `DiffBase::Merge → None`.
3. **Narrow-sticky no-op:** return without changes when **both**
   - `selected.repo_path == target_path`, *and*
   - `target.map_or(true, |s| selected.section == s)`.

   When `target = Some(s)`, the selected row's section must equal `s`.
   When `target = None`, the section is unconstrained and the current
   duplicate row is preserved.

   *Path alone is not enough.* If selection is on the Staged row of
   `foo.rs` and `target` is `Some(Unstaged)`, the panel must move to the
   Unstaged row of the same file. *Target alone is not enough.* If
   `target` is `None` (Head/Merge filter), a click on `foo.rs` while
   already on the Unstaged duplicate row of `foo.rs` should not flip to
   the Staged duplicate row via a first-match re-resolution.
4. Otherwise re-resolve: when `target = Some(s)`, pick the entry whose
   `repo_path` matches *and* whose `section == s`; when `target = None`,
   fall back to the existing first-match heuristic.

This handles the row-click race correctly: by the time the editor event
fires after a row click, `selected_entry` already sits on the right
`(section, repo_path)` pair, the narrow-sticky check succeeds, and the
selection is left alone. When the user instead changes the filter (e.g.
selects Unstaged in the dropdown while looking at a partial file), `target`
shifts to `Some(Section::Unstaged)`, the `section` mismatch is detected,
and the panel moves to the matching row.

Documented in M4 and story 29 (reinforced).

### A5 — Clicking a row should open the diff under the matching `DiffBase`, not relink to an Uncommitted view (M3, design refinement)

**Observed.** In staging-grouped mode, clicking a row in the Staged section
opens the diff under whatever filter is currently active (typically
Uncommitted), not under the Staged filter. The duplicate listing of a
partially-staged file becomes confusing because both row clicks land on the
same combined diff view.

**Root cause.** Even with the click handler dispatching a filter-switch
action, `ProjectDiff::deploy_at` (`crates/git_ui/src/project_diff.rs:238-299`)
currently:

- Filters `items_of_type::<ProjectDiff>(cx)` by
  `matches!(diff_base(cx), DiffBase::Head)` only (line 254-256). A Staged
  target either picks an existing Head view (wrong base reused) or falls
  through.
- Creates fresh views via `Self::new(...)` (line 371-380), which hard-codes
  `DiffBase::Head` for the new `BranchDiff`.

A separate workspace dispatch that mutates `DiffBase` *after* `deploy_at`
runs cannot fix this — the diff view is already attached to the wrong base.

**Design decision.** Couple the row click to the diff open by passing the
target `DiffBase` through, rather than relying on a follow-up dispatch:

1. Extend `ProjectDiff::deploy_at` (and `deploy_at_project_path`) to take a
   `target_base: DiffBase` parameter. Find an existing item by
   `diff_base(cx) == target_base` **and** matching repository; activate it
   if found, otherwise create a new view under `target_base`. **Never
   mutate an existing view's base to match the target** — distinct
   `DiffBase` values are distinct items (the A5 test asserts that a
   Staged-row click does not reuse an existing `Head` view by retargeting
   it).
2. Extend `ProjectDiff::new` / `new_impl` to take a starting `DiffBase`
   threaded into `BranchDiff::new`. Drop the hard-coded `DiffBase::Head` at
   `project_diff.rs:378`.
3. The git panel row click calls
   `ProjectDiff::deploy_at(workspace, Some(entry), target_base, …)` with
   `target_base` computed per the precedence rule in M3 (Branch override
   first, then mode-specific clauses).

The coupling is intentionally asymmetric — the filter changes as a side
effect of an explicit row click, but neither setting *silently* drives the
other (cross-cutting decisions). Documented as story 32 and in M3 ("Click →
filter coupling").

### A6 — Menu integration: "Sort by" submenu replaces standalone "Group by" entry (M4, design refinement)

**Observed.** The first PoC build added a standalone **"Group by Status /
Staging"** toggle entry to the panel menu, sitting next to the existing
**"Sort by Path / Status"** toggle. Two related decisions in two adjacent
toggles felt fragmented compared to Zed's other native menus.

**Design decision.** Replace both toggle entries with a single **"Sort by …"**
submenu containing three radio options (Status / Path / Staging). The two
underlying settings (`group_by`, `sort_by_path`) stay; they are jointly set
by a new parameterized action `git_panel::SetSortBy { mode: SortBy }`. The
**"Tree View"** toggle is unchanged. Documented in M4 ("Menu surfacing") and
story 16 (revised).
