# PRD: Codex-style staged/unstaged/branch diff filtering

Status: Proof-of-concept · Related: issue #26560, prior PRs #36646 / #46541 / #48792

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
16. As a developer, I want to toggle the panel grouping mode via a setting and
    an action, consistent with how the tree-view toggle already works, so that
    it feels native.
17. As a developer in the staging-grouped panel mode, I want a `-` button to
    appear on hover under the Staged section, so that I can unstage a file
    unambiguously.
18. As a developer in the staging-grouped panel mode, I want a `+` button to
    appear on hover under the Unstaged section, so that I can stage a file
    unambiguously.
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
    to be independent controls, so that changing one does not surprise me by
    changing the other.
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

### Module M4 — Panel section grouping (`git_ui`, git panel)

A deep, pure module. Given the repository's status entries and a grouping mode,
it produces the sectioned entry list (section headers + entries in order). It
encapsulates both the existing status grouping and the new staging grouping
behind one interface.

- Add new `Section` variants `Staged` and `Unstaged` (Conflicts retained).
- Add a `GitPanelSettings` field `group_by` with values `status` (default) and
  `staging`. Add a toggle action mirroring the existing tree-view toggle.
- Grouping mode is orthogonal to the flat/tree view setting.
- Because a partially-staged file appears under **both** the Staged and the
  Unstaged section, the panel can no longer identify a row by `RepoPath` alone.
  The current entry index is a `HashMap<RepoPath, _>`, so a duplicated path's
  second occurrence would overwrite the first and break selection, scroll-to,
  open-diff, and the header controls. Entries must be keyed by
  `(section, repo_path)`. Section staged/unstaged counts must be derived from
  the source status entries, not by counting duplicated rendered rows.

### Module M5 — Staging affordance (`git_ui`)

The hover-revealed `+`/`-` buttons used in the staging-grouped panel mode. The
existing checkbox affordance is kept unchanged for the status-grouped mode.

### Cross-cutting decisions

- The diff filter dropdown and the panel grouping mode are independent; neither
  drives the other.
- The Uncommitted filter's diff view is unchanged, including the per-file header
  checkbox — no new status label is added there.
- "Last turn" / agent-turn diffs are not a `DiffBase` and are excluded entirely
  (no agent-crate dependency).
- Build order: Phase 1 = M1 + M2 + M3 (diff-view dropdown end to end);
  Phase 2 = M4 + M5 (panel grouping).

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
