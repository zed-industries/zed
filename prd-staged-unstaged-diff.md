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
35. As a developer in the staging-grouped panel, I want clicking the body of
    the Staged or Unstaged section header (anywhere except the `+`/`-` button)
    to open the matching per-base `ProjectDiff` — Staged header body →
    `DiffBase::Staged`, Unstaged header body → `DiffBase::Unstaged` — so that
    I can swap between the two single-sided diff views from the panel itself
    without reaching for the toolbar dropdown. The `+`/`-` button still
    bulk-stages or bulk-unstages the section without switching the diff
    filter. The Conflicts section header is unchanged.
36. As a developer in the Staged filter, I want the diff to be a **read-only
    snapshot of the git index** (`HEAD`→index) whose displayed buffer is the
    index content itself — not the live worktree file — so that editing it is
    impossible and later worktree edits never silently leak into the staged
    view as unhighlighted context. (Bug A8.)
37. As a developer in the read-only Staged filter, I want to still unstage
    individual hunks inline, so that I can refine what is staged without
    leaving the view. The index buffer is read-only to text editing, but the
    stage/unstage hunk controls remain active. (Reinforces story 30 for the
    new index-backed Staged buffer.)
38. As a developer in the read-only Staged filter, I want activating a hunk or
    the file header (Enter / double-click / an "Open File" affordance) to open
    the actual worktree file in a normal editable editor, so that there is a
    clear path from reviewing the staged snapshot to making new (unstaged)
    edits — the "open the file" model other clients use.
39. As a developer, I want the Staged snapshot to reload whenever the git index
    changes — unstaging from within this view, the panel/header `+`/`-`
    controls, or an external `git add`/`reset` in the terminal — so that the
    staged view always reflects the current index.
40. As a developer, I want the read-only Staged index buffer (which has no file
    on disk) to be non-savable and non-reloadable, so that `Cmd-S` and reload
    are no-ops rather than errors, and `ProjectDiff::active_path`
    (`project_diff.rs:674`, which reads `buffer.file()?` at `:682`) still
    resolves the focused excerpt to its `repo_path` via a stored-`repo_path`
    fallback rather than returning `None`.
41. As a developer in the read-only Staged filter, I want unstaging a hunk to
    actually write the git index, which requires the inline staging-write path
    to resolve the file-less index excerpt to its `repo_path` — because the
    existing path (`Editor::do_stage_or_unstage` → `project.buffer_for_id` →
    `BufferDiffEvent::HunksStagedOrUnstaged` → `GitStore::on_buffer_diff_event`
    → `repository_and_path_for_buffer_id` → `buffer.project_path(cx)`) returns
    `None` for a file-less buffer and silently skips the `set_index_text` job.
    Without this plumbing the controls render but unstaging is a no-op.
42. As a developer in the staging-grouped panel, I want discard to be
    unavailable on Staged rows (only Unstage), so I never throw away staged
    work from the staged side. (Bug A10 — today discard reverts the whole file
    to HEAD regardless of side.)
43. As a developer in the staging-grouped panel, I want discarding an Unstaged
    row to discard *only* the unstaged changes — restore the worktree from the
    index (`git checkout -- <file>`), preserving any staged hunks — rather than
    reverting the whole file to HEAD.
44. As a developer, I want discarding an Unstaged row of a *truly untracked*
    file to trash it (as today), while a staged-new file that has further
    unstaged edits is restored from the index (its staged addition preserved),
    so discard never deletes staged content. `is_created()` is too coarse to
    make this distinction (it is true for both); the truly-untracked test is
    `FileStatus::Untracked`.
45. As a developer who presses the discard key (backspace/delete) on a Staged
    row, I want it to be a no-op with a brief hint to unstage first, so the
    keybinding never silently reverts staged work — the guard must live in
    `revert_selected`/`revert_entry`, not only in the omitted menu item.
46. As a developer, I want right-clicking the Staged or Unstaged section header
    to open a context menu of section-appropriate bulk actions, modeled on the
    VS Code git panel, so I can act on a whole section without reaching for the
    kebab and without new always-visible header icons. (Headers have no
    right-click menu today.)
47. As a developer, I want the Staged header menu to offer **Unstage All** and
    **Stash Staged Changes** (`git stash push --staged`, staged side only) and
    no discard; and the Unstaged header menu to offer **Stage All** and
    **Discard All Unstaged Changes** and no stash — "stash only the unstaged
    side" has no clean git primitive, so stash belongs to the staged side.
48. As a developer choosing **Discard All Unstaged Changes**, I want it to clear
    the whole Unstaged section — restoring tracked-unstaged files from the index
    and trashing untracked files — behind one confirmation that names both
    counts, so the section ends up empty as the label implies.
49. As a developer, I want the default status-grouped panel mode and the panel
    kebab menu to keep their current discard/stash behavior (revert-to-HEAD)
    unchanged, so this rework is purely additive to staging-grouped mode.

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

**Revised for the Staged filter (see A8).** The secondary-diff route above is
retained for the **Unstaged** filter (live worktree buffer, index↔worktree
secondary diff). The **Staged** filter is reworked to show the git index as a
**read-only** snapshot: a file-less `Buffer` built from `load_index_text`
(`Capability::ReadOnly`) diffed against `HEAD` (`load_committed_text`). This is
what prevents later worktree edits from leaking into the staged view. Inline
unstage is preserved because the multibuffer stays `Capability::ReadWrite`
(editor not read-only → controls render) while the index excerpt's per-buffer
`ReadOnly` capability blocks text edits. See followup A8 for the full rationale
and the file-less-buffer plumbing it requires.

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
  `+` button that stages every file in the section. The button itself is the
  bulk-stage / bulk-unstage click target. The remainder of the header row
  (the "body") is a separate click target that **switches the diff view**
  (story 35, appendix A7): a click on the Staged header body opens a
  `DiffBase::Staged` `ProjectDiff` via A5's `deploy_at(target_base=Staged)`,
  and the Unstaged header body opens `DiffBase::Unstaged`. The button's
  `cx.stop_propagation()` (per A1's fix) prevents button clicks from
  cascading into the body handler, so the two targets stay disjoint. The
  Conflicts section header still uses the existing checkbox + whole-row
  toggle.

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
- **A7 / story 35** — M5 UI tests in staging-grouped mode:
  *Body click opens matching base:* simulate-clicking the header body
  (left of the `git-panel-section-header-stage-control-Staged` bounds)
  opens an active `ProjectDiff` with `diff_base == Staged`; same for
  Unstaged. *Branch override:* with a `DiffBase::Merge` view already
  open, a header body click on Staged exits Merge and lands on
  `DiffBase::Staged` (analogue of A5's branch-override row-click test).
  *Selection untouched:* with `selected_entry` set to a known row,
  clicking the Staged header body leaves `selected_entry` unchanged —
  the click is a view-switch, not a navigation. *No-stage side effect
  (renamed legacy test):* the existing
  `test_staging_grouped_section_header_body_click_does_not_toggle` is
  retitled to make the no-stage assertion explicit
  (`..._does_not_stage_files`); its assertions stand unchanged — the
  body click must not bulk-stage anything. *Button still wins:*
  simulate-click the `−` button itself; assert the bulk-unstage fires
  and **no** new `DiffBase::Staged` view is created — the existing
  `Head` view stays active. Regression guard for the
  `cx.stop_propagation()` chain on the button.
- **A8 / stories 36–41** — M2/M3 integration + UI tests on the read-only
  index-snapshot Staged filter:
  *Read-only:* in the Staged filter, a simulated text edit is rejected and the
  buffer content is unchanged; assert the index buffer's capability is
  `ReadOnly` while `editor.read_only(cx)` is `false` (so hunk controls still
  render). *No leak:* with a partially-staged file, edit the worktree out of
  band; assert the Staged view's buffer equals the index content and does
  **not** contain the new worktree edit. *Inline unstage writes the index
  (story 41 regression guard):* unstage a hunk from the **in-editor** control
  in the Staged view; assert `set_index_text` actually ran — the git index
  reverts that hunk toward `HEAD` and the hunk disappears once the snapshot
  reloads. This must fail if the file-less excerpt's `repo_path` plumbing is
  missing (the silent-no-op regression). *Panel unstage still works:* unstage
  the same file from the panel/header `-` control and assert the index changes
  (guards that the explicit-`RepoPath` `stage_or_unstage_entries` path is
  unaffected). *Refresh on
  external change:* stage a file via the backend (simulating terminal
  `git add`) with the Staged view open; assert the snapshot reloads to include
  it (converse for `reset`). *Open-to-edit:* activating a hunk/header in the
  Staged view opens the worktree file in an editable editor (capability
  `ReadWrite`, `buffer.file()` is `Some`). *Save/reload safety:* save and
  reload on the Staged index buffer are no-ops and do not error.
  *Unstaged stays editable:* a text edit in the Unstaged filter is accepted
  (regression guard that the read-only scope is Staged-only).
- **A10 / stories 42–49** — section-aware discard/stash + section-header
  context menus, in staging-grouped mode:
  *Per-row discard (M2/integration).* Partial file, discard the Unstaged row →
  the worktree matches the index, the **git index is unchanged** (staged hunks
  preserved), the Unstaged row disappears and the Staged row remains. Discard an
  untracked Unstaged row → trashed. Discard a staged-new-with-edits Unstaged row
  → restored from index, **not** trashed (staged addition preserved). *Staged
  side (UI).* The Staged-row right-click menu has no "Discard Changes" item
  (Unstage File present); pressing backspace on a Staged row makes **no** index
  or worktree change and shows a hint toast. *Stash staged.* "Stash Staged
  Changes" runs `git stash push --staged`: the staged hunks leave the index, the
  worktree/unstaged side is intact, and a stash entry is created. *Header menus.*
  Right-clicking the Staged header shows Unstage All + Stash Staged (no discard);
  the Unstaged header shows Stage All + Discard All Unstaged (no stash); the
  Conflicts header is unchanged. *Discard all unstaged.* The confirmation names
  the tracked-restore and untracked-trash counts; on confirm the tracked-unstaged
  files are restored from index, untracked files are trashed, the Unstaged
  section ends empty, and the Staged side is untouched. *Regression guards.*
  In status-grouped mode the per-row "Discard Changes" still reverts to HEAD, and
  the panel kebab menu is unchanged.

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

### A7 — Section-header body click opens the matching per-base ProjectDiff (M5, story 35)

**Observed.** After A2 landed, the Staged and Unstaged section headers in
staging-grouped mode became inert outside the `+`/`-` button. The most
natural "switch between Staged and Unstaged diff views" interaction —
clicking the section header itself — does nothing, so users have to reach
for the toolbar dropdown to swap filters. The header looks clickable
(`cursor_pointer`, hover background) but isn't.

**Design decision.** Make the body of the Staged and Unstaged section
headers a click target that opens the matching per-base `ProjectDiff` via
A5's `deploy_at`:

- **Click target.** The header click handler calls
  `ProjectDiff::deploy_at(workspace, None, target_base, …)` where
  `target_base = match section { Staged → DiffBase::Staged, Unstaged →
  DiffBase::Unstaged }`. No specific entry is passed; the view opens at
  its default scroll position. A5's invariant — never retarget across
  `DiffBase` values — carries over, so header clicks activate the
  existing matching per-base view when one is already open and otherwise
  create a fresh one.
- **Branch override is automatic.** The mapping is unconditional on
  `current_base` for `Section::Staged` / `Section::Unstaged`, so the
  full A5 precedence (`target_diff_base_for_click`) collapses to a
  constant for these two sections. No additional logic is needed at the
  header click site.
- **Selection is left alone.** The panel's `selected_entry` stays put;
  the header click is a view-switch, not a navigation. (Compare A5's
  row click, which moves the selected entry to the clicked row.)
- **Affordance reuses existing visuals.** The header row already carries
  `.cursor_pointer()` and a hover background; the new click target is
  discovered through hover, matching the per-row affordance.
- **Empty section is still a target.** Clicking the Staged header when
  no files are staged still opens an empty `DiffBase::Staged` view —
  the placeholder ("No staged changes") is the affordance.
- **Conflicts header is unchanged.** It still uses the existing
  checkbox + whole-row toggle. Status-grouped header bodies are also
  untouched.
- **The `+`/`-` button's `cx.stop_propagation()`** (A1's fix) prevents
  button clicks from cascading into the body handler, so bulk
  stage/unstage stays disjoint from the diff-view switch.

This supersedes A2's "the whole-row click on the header does not toggle —
only the button itself is the click target" rule for the Staged and
Unstaged headers: the body is now clickable, but the click no longer
toggles staging (the `+`/`-` button retains that responsibility) — it
switches the diff filter via the A5 deploy path. Documented in M5
("Section-header level") and story 35.

### A8 — Staged filter is editable and leaks later edits into the staged view (M2 / M3)

**Observed.** Opening the Staged filter shows the staged changes, but the
buffer is editable. Editing it — or editing the file elsewhere while the Staged
view is open — leaves the new text visible in the Staged view (unhighlighted,
but present), even though git correctly records the edit as unstaged. The staged
view therefore misrepresents what is actually staged.

**Root cause.** The Staged view does not show the git index; it shows excerpts
of the **live worktree buffer** — the same `Buffer` entity as the file on disk —
created `Capability::ReadWrite` (`project_diff.rs:408`), with
`DiffHunkFilter::Staged` applied. There is no per-`DiffBase` read-only gating:
`configure_editor_for_diff_base` only hides hunk controls for `Merge`, never
sets read-only (`project_diff.rs:602-631`). The hunk filter governs which hunks
are *highlighted as staged*, but the underlying text is worktree text, so
unstaged edits appear as ordinary context lines.

**Design decision.** For the **Staged** filter only, show the actual git
**index content** as a **read-only snapshot** instead of the live worktree:

- **Index-backed buffer.** Load index text via `load_index_text` and build a
  synthetic, file-less `Buffer` with `Capability::ReadOnly`; diff base = `HEAD`
  (`load_committed_text`). Because the displayed buffer *is* the index, later
  worktree edits cannot appear in it. Unstaged and Uncommitted are unchanged
  (live editable worktree); Branch is unchanged. (Scope: Staged only.)
- **Inline unstage preserved — rendering (story 37).** `Capability::ReadOnly`
  is enforced per-buffer in both the editor input path (`input.rs:92-104`) and
  the multibuffer edit router (`multi_buffer.rs:1586+`), so typing into the
  index excerpt is rejected. The ProjectDiff multibuffer stays
  `Capability::ReadWrite`, so `editor.read_only(cx)` remains false and the
  inline stage/unstage hunk controls still render (`element.rs:11211`). No
  separate editor-level read-only flag is used (that would hide the controls).
- **Inline unstage preserved — write path (story 41).** Rendering the control
  is necessary but *not sufficient*: the write path
  `Editor::do_stage_or_unstage` (`editor/src/git.rs:1417`) →
  `project.buffer_for_id` (`:1425`) → `BufferDiffEvent::HunksStagedOrUnstaged`
  → `GitStore::on_buffer_diff_event` (`git_store.rs:1986`) →
  `repository_and_path_for_buffer_id` (`:2088`) → `buffer.project_path(cx)`
  (`:2094`) resolves the target repo/path **from the buffer's project path**. A
  file-less `Buffer::local` is neither registered in the project buffer store
  (so `buffer_for_id` misses) nor has a `project_path` (so
  `repository_and_path_for_buffer_id` returns `None` and the
  `spawn_set_index_text_job` is silently skipped). This rework therefore
  **requires** new `repo_path` plumbing: register the synthetic index buffer
  with the project keyed to its `repo_path`, or carry an explicit `repo_path`
  through the staging-write path so `set_index_text` runs. Without it, the
  control renders but unstaging is a silent no-op.
- **Refresh on any index change (story 39).** The snapshot reloads by hooking
  the existing repository status-refresh signal, so unstaging from this view,
  the panel/header `+`/`-`, and external `git add`/`reset` all keep it current.
- **File-less-buffer plumbing (story 40).** Save and reload are disabled /
  no-ops for the index buffer (the local save path rejects file-less buffers
  with "buffer doesn't have a file"), and `ProjectDiff::active_path`
  (`project_diff.rs:674`, which reads `buffer.file()?` at `:682`) must map the
  focused index excerpt back to its `repo_path` via a path-key / stored
  `repo_path` fallback rather than returning `None` when `buffer.file()` is
  `None`.
- **Open-to-edit (story 38).** Activating a hunk or the file header in the
  read-only Staged view opens the **actual worktree file** in a normal editor,
  giving an explicit path from reviewing the snapshot to making new (unstaged)
  edits.

**Implementation risks to validate.** Two distinct concerns, both on the inline
unstage path:

1. **Repo/path resolution (story 41) — the harder one.** As detailed in the
   write-path bullet above, the inline editor staging path resolves the target
   repo/path from `buffer.project_path(cx)`
   (`repository_and_path_for_buffer_id`, `git_store.rs:2094`), which is `None`
   for a file-less buffer, so the index write is silently skipped. Note the
   bulk *panel* path is different: `stage_or_unstage_entries`
   (`git_store.rs:6088-6250`) takes an explicit `Vec<RepoPath>` and is already
   buffer-agnostic — so panel/header unstaging would keep working, while only
   the *inline* (in-editor) control needs the new `repo_path` plumbing. This is
   the change most likely to require touching `editor/src/git.rs` and the
   `GitStore` buffer→repo resolution, not just the diff loader.
2. **Index-content computation.** Inline unstage today derives the new index
   text from the worktree buffer + the secondary (index↔worktree) diff. With an
   index-backed buffer and base = `HEAD`, "unstage a hunk" means reverting that
   hunk in the index toward `HEAD`; the `stage_or_unstage_hunks` wiring
   (`editor/src/git.rs`, `git_store.rs` `set_index_text`) must be configured so
   the computed index content is correct for this base/target pair.

Documented as stories 36–41; supersedes the Staged half of M2's "secondary-diff
route" decision.

### A9 — Staged view visual parity (file rendering) (M2 / M3)

Backfill stub — A9 was tracked and completed in the progress file
(`prd-staged-unstaged-diff-progress.md`) without a matching appendix entry; this
note closes the gap so the appendix sequence is continuous.

**Observed.** A8's index snapshot was a *file-less* `Buffer::local`, so the
Staged view rendered without syntax highlighting, file icon, parent path, status
badge, or a read-only affordance — it looked unlike the Unstaged view.

**Design decision.** Build the snapshot with a synthetic read-only
`language::File` (the `commit_view.rs` `GitBlob` pattern, `DiskState::Historic`,
real repo path) instead of file-less `Buffer::local`, detect and async-load the
language for both the buffer and its diff base text, and register
`BranchDiffAddon` for the status badge — keeping the index-content + read-only
behavior from A8 intact. See the progress file's A9 section for the full
checklist (including the deleted-line `language_changed` follow-up and the
deferred snapshot-caching task).

### A10 — Discard reverts both sides; staging-grouped menus are not section-aware (M2 / M4 / M5)

**Observed.** Right-clicking a file in the git panel and choosing **Discard
Changes** (or pressing backspace/delete) reverts the file *fully to HEAD*,
discarding staged **and** unstaged changes together — even in staging-grouped
mode where the row represents only one side. There is no "discard only the
unstaged part." Separately, the new Staged/Unstaged **section headers** expose
only the `+`/`-` button and a body-click; there is no section-aware way to bulk
discard, and stash (kebab "Stash All") stashes everything rather than the staged
side.

**Root cause.** `git::RestoreFile` → `revert_selected` (`git_panel.rs:1863`) →
`revert_entry` (`git_panel.rs:1951`) unconditionally unstages
(`change_file_stage(false, …)`) and then `checkout_files("HEAD", …)` →
`git checkout HEAD -- <path>` (`repository.rs:1465`), which overwrites both the
index and the worktree from HEAD. Neither `revert_entry` nor
`deploy_entry_context_menu` (`git_panel.rs:6435`) branches on `entry.section`,
even though every `GitStatusEntry` already carries its `section`
(`git_panel.rs:679`). `render_list_header` (`git_panel.rs:6303`) attaches no
right-click handler. There is no "restore worktree from index" primitive
(`checkout_files` always takes a commit) and no staged-only stash
(`stash_paths`, `repository.rs:2230`, runs plain `git stash push
--include-untracked`).

**Design decision.** Make the destructive/staging menu actions **section-aware,
VS Code-style, scoped to staging-grouped mode only** (status-grouped mode and the
panel kebab menu are untouched — story 49):

- **Per-row discard** (`revert_entry`, branch on `entry.section`):
  - **Staged row** → no-op + hint toast ("Unstage before discarding"). The
    item is also omitted from the Staged-row menu in `deploy_entry_context_menu`.
    The guard lives in `revert_selected`/`revert_entry` because backspace/delete
    bind straight to `git::RestoreFile`. (Stories 42, 45.)
  - **Unstaged row, tracked** → restore worktree from index
    (`git checkout -- <file>`), preserving staged hunks. Prompt reworded to
    "Discard unstaged changes to X?". (Story 43.)
  - **Unstaged row, truly untracked** (`FileStatus::Untracked`, not the broader
    `is_created()`) → trash. A staged-new file with further unstaged edits is
    restored from index, not trashed. (Story 44.)
  - **Tracked / New / Conflict** (status-grouped, or Conflicts in either mode)
    → unchanged revert-to-HEAD. (Story 49.)
- **Section-header right-click menu** — new `on_mouse_down(Right)` on
  `render_list_header` deploying a section-specific `ContextMenu`:
  - **Staged header** → Unstage All + **Stash Staged Changes**. (Stories 46, 47.)
  - **Unstaged header** → Stage All + **Discard All Unstaged Changes**. (Stories
    46, 47.)
  - **Conflicts header** → unchanged.
- **Discard All Unstaged Changes** clears the whole Unstaged section: restore all
  tracked-unstaged from index + trash all untracked, behind one confirmation
  naming both counts (untracked files live in the Unstaged section —
  `status.rs:146` maps Untracked → Unstaged). (Story 48.)
- **Stash Staged Changes** → `git stash push --staged` (staged side only); offered
  on the Staged header only. (Story 47.)

**New git primitives required** (git → project/proto → git_ui):

1. **Restore worktree from index** — `git checkout -- <paths>` (no commit).
   `checkout_files` (`repository.rs:1451`) always takes a commit, so this is a new
   trait method + Local/Remote impl + a new proto request for the collab/remote
   path; the `project` wrapper mirrors `checkout_files` (`git_store.rs:5465`).
2. **Stash staged only** — `git stash push --staged`. New method + proto request,
   mirroring `stash_paths`. ⚠️ Requires **git ≥ 2.35** (Jan 2022); the repo has no
   git-version gating today — raise this as a compatibility note for the team.

**Implementation surface.** `crates/git/src/repository.rs` (two new primitives);
`crates/project/src/git_store.rs` (two new `project`/`Repository` methods + proto
wiring); `crates/proto/proto/git.proto` (two new request messages);
`crates/git_ui/src/git_panel.rs` (`revert_selected`/`revert_entry` section branch
+ toast + wording; `deploy_entry_context_menu` omit-on-Staged;
`render_list_header` right-click → new `deploy_section_context_menu`; new bulk
discard-all-unstaged handler).

**Dependency.** Builds on M4 (the `(section, repo_path)` entry key and the
`Section::Staged`/`Unstaged` variants) and the staging-grouped section headers
(A2/A7). Independent of A8/A9's read-only snapshot work.
