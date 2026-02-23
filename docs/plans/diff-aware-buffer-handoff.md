# Diff-Aware Buffer Redesign — Handoff Prompt

## Context

You are continuing a design conversation about fundamentally rethinking how diff (side-by-side diff view, inline diff, gutter indicators, diff stats) works in Zed. A detailed planning document exists at `zed/docs/plans/diff-aware-buffer.md` — read it first for background, but know that the design has evolved significantly beyond what that document describes. This handoff captures the current state of our thinking.

**Stay in planning mode** — help think through the design, identify problems, refine the approach, and connect ideas to concrete code. Do not implement anything yet. The goal is a plan that's detailed enough to implement confidently.

---

## The Core Idea

Push diff awareness down so it's a native property of the content the editor renders, not something bolted on after the fact. The key question we're still refining: **where exactly does the diff state live?**

Three options have been discussed:

1. **Diff state on `text::Buffer`** — the original plan in `diff-aware-buffer.md`. The CRDT buffer itself holds an optional diff base.

2. **`BufferDiff` as the fundamental unit MultiBuffer indexes** — instead of MultiBuffer holding `Entity<Buffer>` and optionally associating `Entity<BufferDiff>`, MultiBuffer is built on `Entity<BufferDiff>`, which wraps a buffer and optionally has a diff base. The degenerate case (no diff) is trivially cheap.

3. **Merge diff state into `language::Buffer`** — if every buffer in the editor will go through BufferDiff anyway, the wrapper entity is just indirection. The buffer itself should hold the diff base, regions, and stats. MultiBuffer continues to hold `Entity<Buffer>` (actually `Entity<language::Buffer>`) and the diff information is always there.

We're leaning toward option 2 or 3. The next agent should help us decide. The key tradeoffs are captured in the "Open Decision" section below.

Regardless of which option we choose, the downstream design is the same: the excerpt holds a snapshot that carries dual-dimension summaries (buffer text + diff base text) and diff stats, the `DiffTransform` tree and `Companion` system are eliminated, and the editor natively renders diffable content.

---

## What We've Decided

### 1. The diff region model

The relationship between buffer text and base text is described by a `SumTree<DiffRegion>`. Each region is classified:

- **Unchanged** — text present in both buffer and base (identical content). Both `buffer_len` and `base_len` are nonzero and equal.
- **Added** — text present in buffer only. `buffer_len > 0`, `base_len == 0`.
- **Removed** — text present in base only. `buffer_len == 0`, `base_len > 0`.

The region tree uses length-based boundaries, not anchors. Base text is a `Rope` (no CRDT, no anchors), so anchor-based boundaries don't work for the base side. Length-based regions are structurally simple: the cumulative `buffer_len` across all regions equals the buffer's text length; the cumulative `base_len` equals the base text length.

### 2. Dual-dimension summaries

`DiffRegionSummary` carries both `buffer: TextSummary` and `base: TextSummary`. Seeking by the `buffer` dimension navigates buffer-text space; seeking by the `base` dimension navigates base-text space. Both are O(log n) through the same tree.

This dual-dimension nature propagates up through excerpts. The `Excerpt` stores both `text_summary: TextSummary` (buffer text) and `diff_base_summary: TextSummary` (base text). `ExcerptSummary` carries both `text: MBTextSummary` and `diff_base: MBTextSummary`. The SumTree addition logic handles them independently.

This eliminates the need for the `DiffTransform` tree entirely. The dual dimensions are a native property of the excerpt SumTree.

### 3. Diff stats in the summary algebra

`DiffRegionSummary` also carries `DiffStats`:

```rust
pub struct DiffStats {
    pub lines_added: u32,
    pub lines_removed: u32,
    pub bytes_added: usize,
    pub bytes_removed: usize,
}
```

For an `Added` region: `lines_added = buffer_summary.lines.row`, `bytes_added = buffer_summary.len`.
For a `Removed` region: `lines_removed = base_summary.lines.row`, `bytes_removed = base_summary.len`.
For `Unchanged`: all zeros.

These accumulate through the SumTree (addition is trivial — just add each field). The `Excerpt` stores `diff_stats: DiffStats`, and `ExcerptSummary` carries `diff_stats: DiffStats`. This means:

- Total diff stats for a multibuffer: `snapshot.excerpts.summary().diff_stats` — **O(1)**.
- Diff stats for a range: SumTree range query — **O(log n)**.
- Diff stats per buffer in a multi-buffer: walk relevant excerpts — **O(log n) per excerpt**.

This replaces the current O(n)-in-hunks computation that `CommitView::calculate_changed_lines` (in `crates/git_ui/src/commit_view.rs` ~L437) and `DiffStats::all_files` (in `crates/agent_ui/src/acp/thread_view/active_thread.rs` ~L132) do by iterating every hunk.

### 4. `rope::TextSummary` is unchanged

The dual-dimension summary (`DiffTextSummary { buffer: TextSummary, diff_base: TextSummary }`) lives at the buffer-diff level, not the rope level. Adding diff fields to `rope::TextSummary` would bloat every chunk summary in every rope by ~50% for a feature most ropes don't use. The rope stays lean.

### 5. The buffer's primary coordinate space is unchanged

Offsets, Points, and Anchors always refer to the working copy (buffer) text. The diff base is a secondary, queryable space. Coordinate translation (`to_diff_base_offset`, `from_diff_base_offset`) is O(log n) via the region tree.

### 6. Unchanged/Added/Removed terminology, not Left/Right

The buffer/diff layer uses diff terminology. Left/Right are rendering concepts belonging to the editor and split view.

### 7. Writing a diff base is an async-friendly operation

The diff algorithm (libgit2's patience diff) runs on a background thread. When complete, the result (`Rope` + `SumTree<DiffRegion>` + word diffs) is written to the buffer/diff entity on the main thread. Swapping diff bases is just writing new content. Clearing is just setting to None.

### 8. Edits between diff recomputations update region summaries but leave classifications stale

When the buffer is edited, the region tree's buffer-side lengths and summaries are updated for affected regions (O(log n + k) where k is the number of touched regions, typically 1-2). The `kind` classification might become stale (e.g., text edited within an `Unchanged` region is no longer truly unchanged). The next background diff recomputation corrects everything. This matches the current user experience: you type, gutter markers update a beat later.

### 9. The `DiffTransform` tree is eliminated

Today, `MultiBufferSnapshot` has a `SumTree<DiffTransform>` (~L610 in `multi_buffer.rs`) that interleaves `BufferContent` and `DeletedHunk` items. `MultiBufferCursor` walks this tree in lockstep with the excerpt tree (~200 lines of complex cursor logic). `sync_diff_transforms` (~300 lines, ~L3129) rebuilds it on every edit and diff change.

All of this is replaced by the dual-dimension `ExcerptSummary`. The `MultiBufferCursor` becomes a single-tree cursor. The sync path becomes: rebuild the excerpt with a fresh snapshot, recompute both text summaries and diff stats. Done.

### 10. The `Companion` system, `LhsEditor`, spacer blocks, and excerpt mirroring are eliminated

Today's split diff uses two separate editors with elaborate synchronization (see `crates/editor/src/split.rs`, `crates/editor/src/display_map.rs` ~L232). This is replaced by one multibuffer with two display pipelines reading from the same snapshot, one for each side.

### 11. Split view rendering uses `DiffSide`

```rust
pub enum DiffSide {
    /// Yield buffer text: Unchanged + Added regions.
    /// Removed regions are zero-width (skipped).
    /// This is the default for non-diff views.
    Buffer,
    /// Yield base text: Unchanged + Removed regions.
    /// Added regions are zero-width (skipped).
    /// This is what the left panel of a split diff sees.
    Base,
}
```

Each side of a split view gets its own display pipeline (`InlayMap → FoldMap → TabMap → WrapMap → BlockMap`). Both read from the same `MultiBufferSnapshot` but iterate different content based on `DiffSide`. The right panel sees buffer text with Added regions highlighted and gaps for Removed regions. The left panel sees base text with Removed regions highlighted and gaps for Added regions.

Alignment between sides comes from the diff region summaries: a Removed region's `base` TextSummary tells the right side how many gap lines to insert, and vice versa.

### 12. The editor natively renders diffable content

The editor doesn't discover diff information through a side channel. It's in the excerpt. `RowInfo.diff_status` is populated directly from the excerpt's diff regions, not by looking up a separate `DiffStateSnapshot` map. Hunk navigation, staging, and diff stats all query through the excerpt.

### 13. Staging stays as a git-specific concern

Staging (modifying git's index) is not a buffer-level operation. It needs:
- HEAD text and index text (managed by `BufferGitState` in `crates/project/src/git_store.rs`)
- Hunk boundaries (derived from the diff region tree)
- The buffer's working copy text

The staging logic (`stage_or_unstage_hunks_impl`, currently ~L544 in `buffer_diff.rs`) reconstructs index text by splicing buffer text and HEAD text based on hunk boundaries. This operation reads from the diff region tree but doesn't live on it.

Staging status indicators (staged/unstaged/partially staged) come from comparing a primary diff (HEAD → working) against a secondary diff (index → working). Both need to exist somewhere accessible.

### 14. Secondary diff base for staging status

To show staging indicators, we need two diff computations: HEAD → working copy (primary) and index → working copy (secondary). Each hunk's staging status is derived by comparing whether it appears in both, one, or neither diff.

This could be:
- Two `DiffBaseState` on a `BufferDiff` entity
- Two optional diff bases on `language::Buffer`
- The primary on the buffer and the secondary as metadata alongside

The exact placement depends on the open decision below, but the mechanism is the same: two sets of diff regions, compared at query time.

### 15. Word diffs are stored alongside diff regions, not on them

Word-level diff ranges (highlighting specific changed words within a hunk) are computed during the diff pass and stored as a parallel structure (e.g., `Vec<WordDiffHunk>`) on the diff base state. They're not fields on `DiffRegion` itself, because most regions (Unchanged) have no word diffs and it would bloat the SumTree items.

### 16. Diff stats don't flow through the display pipeline

The display pipeline (`InlayMap → FoldMap → TabMap → WrapMap → BlockMap`) transforms text layout. None of these transforms change diff stats — inlays don't add real "additions," folds don't create "deletions." Diff stats are queried at the `MultiBufferSnapshot` level, with display coordinates mapped back to multibuffer coordinates when needed.

The `diff_base` dimension on `ExcerptSummary` also doesn't flow through the display pipeline. It's used for base-text seeking, alignment computation, and coordinate translation — all multibuffer-level queries.

---

## Open Decision: Where Does Diff State Live?

This is the main thing the next agent should help resolve. Here are the options with their concrete tradeoffs:

### Option A: Separate `BufferDiff` entity (MultiBuffer built on `Entity<BufferDiff>`)

`BufferDiff` is an entity that wraps `Entity<language::Buffer>` and optionally holds a diff base. MultiBuffer's fundamental unit changes from `Entity<Buffer>` to `Entity<BufferDiff>`. The excerpt holds `Arc<BufferDiffSnapshot>` instead of `Arc<BufferSnapshot>`.

```rust
pub struct BufferDiff {
    buffer: Entity<language::Buffer>,
    diff_base: Option<DiffBaseState>,
    secondary_diff_base: Option<DiffBaseState>,
    diff_version: usize,
}

struct DiffBaseState {
    text: Rope,
    regions: SumTree<DiffRegion>,
    word_diffs: Vec<WordDiffHunk>,
}
```

**Pros:**
- Clean separation of concerns — buffer is text, BufferDiff adds diff awareness
- `language::Buffer` stays unchanged (already a large struct)
- The diff computation dependency (libgit2) doesn't bleed into the `language` crate
- Multiple independent diffs of the same buffer (HEAD, index, arbitrary OID) are naturally separate entities
- Non-git diffs (agent edits, edit predictions) use the same entity with different base text

**Cons:**
- Every buffer needs a `BufferDiff` wrapper, even when there's no diff — adds entity overhead
- Edit-time region updates require a subscription from `BufferDiff` to `Buffer`, rather than happening inline
- API friction: `MultiBuffer::singleton` takes `Entity<BufferDiff>`, callers must wrap first
- Two entity lifecycles to manage per buffer

**Migration scope:** MultiBuffer's `buffers: BTreeMap<BufferId, BufferState>` changes to `buffer_diffs: BTreeMap<BufferId, BufferDiffState>`. All `push_excerpts(buffer, ...)` calls change to `push_excerpts(buffer_diff, ...)`. `diffs: HashMap<BufferId, DiffState>` is eliminated. `add_diff()` is eliminated.

### Option B: Merge diff state into `language::Buffer`

`language::Buffer` gains an optional diff base directly. MultiBuffer continues to hold `Entity<Buffer>`. The excerpt holds `Arc<language::BufferSnapshot>` which now includes diff information.

```rust
// In language::Buffer:
pub struct Buffer {
    text: TextBuffer,
    // ... existing fields ...
    diff_base: Option<DiffBaseState>,
    secondary_diff_base: Option<DiffBaseState>,
    diff_version: usize,
}

pub struct BufferSnapshot {
    pub text: text::BufferSnapshot,
    pub syntax: SyntaxSnapshot,
    // ... existing fields ...
    diff_base: Option<DiffBaseSnapshot>,
    secondary_diff_base: Option<DiffBaseSnapshot>,
}
```

**Pros:**
- Simplest mental model — a buffer *is* diffable content
- No wrapper entity — no entity overhead for the common case
- Edit-time region updates can happen inside `Buffer::edit` (or its equivalent), avoiding subscription lag
- MultiBuffer API doesn't change — still takes `Entity<Buffer>`
- One entity lifecycle per buffer
- The excerpt snapshot is just `Arc<BufferSnapshot>`, same as today but with diff fields added

**Cons:**
- `language::Buffer` grows larger (already ~40 fields)
- Diff computation pulls in libgit2 as a dependency of the `language` crate, or the computation stays external and only the *storage* types live in `language`
- Multiple diffs of the same buffer (HEAD, index, OID) become awkward — does the buffer hold all of them? Or just the "primary" one?
- The buffer becomes responsible for diff region updates on edit, adding complexity to the edit path
- Non-git callers (agent UI, edit predictions) that today create standalone `BufferDiff` entities for throwaway comparisons would need to set diff bases on actual buffers

**Migration scope:** `language::Buffer` and `language::BufferSnapshot` gain diff fields. `BufferDiff` entity is eliminated. All `add_diff()` calls are replaced by `buffer.set_diff_base(...)`. Diff computation stays in a separate crate but writes results to the buffer.

### Option C: Hybrid — diff storage types in `text` crate, diff entity stays separate

`DiffRegion`, `DiffRegionSummary`, `DiffStats`, `DiffTextSummary` live in the `text` crate (they describe text relationships). `BufferDiff` stays as an entity that holds a `text::Buffer` reference and a diff base. But `text::BufferSnapshot` gains an `Option<DiffBase>` field so that the snapshot itself carries the diff information without needing a separate snapshot type.

This is essentially what the original planning document describes. The diff *types* are in `text`, the diff *entity* is in `buffer_diff`, and the *storage* is on the snapshot.

**Concern:** This splits ownership awkwardly — who updates the snapshot's diff base? The buffer doesn't know about it (it's managed by the external `BufferDiff` entity), but the snapshot carries it.

### Recommendation to explore

We were leaning toward either A or B. The key question to resolve: **is the entity overhead of option A worth the separation of concerns, or is the simplicity of option B worth the added complexity to `language::Buffer`?**

A practical test: look at the non-git callers of `BufferDiff` (agent edits in `crates/acp_thread/src/diff.rs`, action log in `crates/action_log/src/action_log.rs`, edit predictions in `crates/edit_prediction_ui/src/rate_prediction_modal.rs`, git UI views in `crates/git_ui/src/`). In option B, these would need to set diff bases on actual buffers. Does that work naturally, or does it create lifecycle problems? In option A, they create `BufferDiff` entities wrapping those buffers, which is what they already do today.

Also consider: in option B, the `language` crate would need the diff storage types (`DiffRegion`, `DiffRegionSummary`, `DiffStats`). These could live in the `text` crate (which `language` already depends on) with the *computation* staying in a separate crate.

---

## What Gets Eliminated (Regardless of Option)

| Component | Location | Lines (approx) | Status |
|-----------|----------|----------------|--------|
| `SumTree<DiffTransform>` | `multi_buffer.rs` ~L610 | — | **Removed** |
| `DiffTransform` enum | `multi_buffer.rs` ~L638 | 20 | **Removed** |
| `DiffTransformSummary` | `multi_buffer.rs` ~L804 | 10 | **Removed** |
| `DiffTransformHunkInfo` | `multi_buffer.rs` ~L656 | 20 | **Removed** |
| `sync_diff_transforms` | `multi_buffer.rs` ~L3129 | 300+ | **Removed** |
| `recompute_diff_transforms_for_edit` | `multi_buffer.rs` ~L3267 | 200+ | **Removed** |
| `push_buffer_content_transform` | `multi_buffer.rs` ~L3510 | 50+ | **Removed** |
| `extend_last_buffer_content_transform` | `multi_buffer.rs` ~L3546 | 20+ | **Removed** |
| `append_diff_transforms` | `multi_buffer.rs` ~L3471 | 20+ | **Removed** |
| `push_diff_transform` | `multi_buffer.rs` ~L3494 | 15 | **Removed** |
| `MultiBufferCursor` lockstep logic | `multi_buffer.rs` ~L6975 | 200+ | **Simplified to single cursor** |
| `MultiBufferExcerpt.diff_transforms` cursor | `multi_buffer.rs` ~L759 | — | **Removed** |
| `diffs: TreeMap<BufferId, DiffStateSnapshot>` | `multi_buffer.rs` ~L609 | — | **Removed** |
| `DiffState` / `DiffStateSnapshot` | `multi_buffer.rs` ~L522/538 | 80 | **Removed** |
| `add_diff()` / `add_inverted_diff()` | `multi_buffer.rs` ~L2609/2626 | 20 | **Removed** |
| `buffer_diff_changed()` | `multi_buffer.rs` ~L2374 | 40 | **Removed or replaced** |
| `inverted_buffer_diff_changed()` | `multi_buffer.rs` ~L2418 | 40 | **Removed** |
| `has_inverted_diff` flag | `multi_buffer.rs` ~L616 | — | **Removed** |
| `all_diff_hunks_expanded` / `show_deleted_hunks` / `use_extended_diff_range` | `multi_buffer.rs` ~L622-624 | — | **Moved to editor** |
| `Companion` struct | `display_map.rs` ~L232 | 100+ | **Removed** |
| `CompanionExcerptPatch` | `display_map.rs` ~L183 | 10 | **Removed** |
| Spacer blocks (`spacer_blocks()`) | `block_map.rs` ~L1207 | 100+ | **Removed** |
| Balancing blocks | `block_map.rs` | 50+ | **Removed** |
| `LhsEditor` struct | `split.rs` ~L338 | 5 | **Removed** |
| `sync_path_excerpts` | `split.rs` ~L1871 | 40 | **Removed** |
| `sync_cursor_to_other_side` | `split.rs` ~L732 | 30 | **Removed** |
| Row conversion functions | `split.rs` ~L41-57 | 100+ | **Removed** |
| `patch_for_buffer_range` O(n) | `buffer_diff.rs` ~L414 | 40 | **Replaced** by O(log n) |
| `patch_for_base_text_range` O(n) | `buffer_diff.rs` ~L462 | 40 | **Replaced** by O(log n) |
| `SumTree<InternalDiffHunk>` | `buffer_diff.rs` | — | **Replaced by `SumTree<DiffRegion>`** |
| Base text as `Entity<language::Buffer>` | `buffer_diff.rs` | — | **Replaced by `Rope`** |
| `is_inverted` concept | throughout | — | **Removed** |

## What Gets Added

| Component | Purpose |
|-----------|---------|
| `DiffRegion` / `DiffRegionKind` / `DiffRegionSummary` | Region tree items and summary, with dual TextSummary fields |
| `DiffStats` | Accumulated diff statistics (lines/bytes added/removed) |
| `DiffTextSummary` | `{ buffer: TextSummary, diff_base: TextSummary }` |
| `DiffBase` / `DiffBaseState` / `DiffBaseSnapshot` | Container for base text `Rope` + region tree + word diffs |
| `diff_base_summary` field on `Excerpt` | Diff base text dimension |
| `diff_base` field on `ExcerptSummary` | Accumulated diff base dimension |
| `diff_stats` field on `Excerpt` and `ExcerptSummary` | Accumulated diff statistics |
| `DiffBaseOffset` / `DiffBasePoint` dimension types | Seeking by diff base dimension in the excerpt tree |
| `DiffSide` enum | `Buffer` vs `Base` for split view chunk iteration |
| Coordinate translation APIs | `to_diff_base_offset`, `from_diff_base_offset`, etc. — O(log n) |
| `diff_stats()` / `diff_stats_for_range()` on `MultiBufferSnapshot` | O(1) total stats, O(log n) range stats |
| `diff_regions()` iterator | Iterate classified regions in a range |
| `diff_hunks()` method | Coalesced hunks (adjacent Removed+Added = Modified) for gutter/navigation |
| Split view renderer using diff regions | Replaces two-editor split with one multibuffer + two pipelines |
| `compute_diff_regions()` function | Pure function: `(base_text, buffer_text) → DiffBase`. Runs on background thread. |

---

## Areas That Need Further Design Work

### 1. Edit-time region summary updates

When the buffer is edited, the `DiffRegion` tree's buffer-side lengths need updating. SumTree doesn't support in-place mutation — you rebuild the affected portion using the cursor slice-and-rebuild pattern.

The basic approach: after an edit produces a `Patch<usize>` of changed ranges, walk the region tree for affected regions, adjust `buffer_len`, recompute the `buffer` `TextSummary` from the actual buffer text for touched regions. The `base_len` and `base` summary are unaffected.

**Key invariant that must hold:** the cumulative `buffer_len` across all regions equals `visible_text.len()` after the update.

**Tricky case:** an edit that spans multiple regions (e.g., user selects across an Unchanged + Added boundary and deletes). Both regions shrink. The simplest approach: don't try to be precise about classification — just adjust lengths so the invariant holds. The next diff recomputation fixes classifications.

If diff state lives on the buffer (option B), this update happens inline in the edit path. If separate (option A), it happens via subscription to buffer edits.

### 2. The display pipeline fork for split view

Both sides of a split view need their own `InlayMap → FoldMap → TabMap → WrapMap → BlockMap` pipeline. They read from the same `MultiBufferSnapshot` but see different text.

The fork needs a thin **projection layer** (or a mode parameter on `MultiBufferChunks`) that selects which text to yield. For `DiffSide::Buffer`: Unchanged + Added regions from `visible_text`. For `DiffSide::Base`: Unchanged + Removed regions from `diff_base.text`.

The right side (`DiffSide::Buffer`) is essentially today's behavior — Removed regions have zero buffer length and are naturally skipped. The left side (`DiffSide::Base`) is the new case — it reads from the base text rope for Removed regions.

**Open question:** Does each side get its own `MultiBufferSnapshot` variant, or is the projection handled at the chunks level? The latter is simpler but may not give you proper edit notifications for the base-text side.

### 3. Expand/collapse for inline diff

Split view is always expanded (both sides always visible). For inline diff (single editor showing deleted text inline), expand/collapse needs a mechanism.

**Recommended approach:** BlockMap-based insertion. When a hunk is "expanded" in inline view, the block map inserts custom blocks containing the removed text (read from `diff_base_text_for_range`). When "collapsed," the blocks are removed. This reuses existing block infrastructure and doesn't change the multibuffer's text length.

### 4. `MultiBufferDiffHunk` population

`MultiBufferDiffHunk` is used extensively by the editor for gutter rendering, hunk navigation, staging UI, etc. Today it comes from `diff_hunks_in_range` which queries `DiffStateSnapshot`.

In the new model, this method walks excerpts and queries each excerpt's diff regions. Adjacent `Removed` + `Added` regions at the same boundary are coalesced into a `Modified` hunk. The returned type can remain largely the same.

The `secondary_status` field (staging indicators) requires access to the secondary diff base to compare.

### 5. Staging migration

The `stage_or_unstage_hunks_impl` function (currently on `BufferDiffInner<Entity<language::Buffer>>`, ~L544 in `buffer_diff.rs`) needs to work with `DiffRegion` ranges instead of `InternalDiffHunk` ranges. The operation is conceptually the same: given hunk boundaries, splice buffer text and HEAD text to produce new index text.

The function currently uses the "unstaged diff" (index → working) hunk tree to translate buffer offsets to index offsets. In the new model, this translation comes from the secondary diff base's region tree.

### 6. `diff_text_summary_for_range` implementation

This is the core query method. Given a buffer offset range, walk the `DiffRegion` tree and accumulate both `buffer` and `base` `TextSummary` values plus `DiffStats`.

**Key subtlety:** Removed regions have `buffer_len == 0`, so they sit at a single point in buffer-offset space. When seeking by buffer dimension with `Bias::Right`, you must not skip Removed regions at the seek position. This is analogous to how the CRDT fragment tree handles zero-width tombstones.

### 7. Non-git diff callers

Several systems create `BufferDiff` entities for non-git comparisons:
- `crates/acp_thread/src/diff.rs` — AI completion diffs
- `crates/action_log/src/action_log.rs` — agent edit tracking
- `crates/edit_prediction_ui/src/rate_prediction_modal.rs` — prediction rating
- `crates/git_ui/src/{commit_view,file_diff_view,multi_diff_view,text_diff_view}.rs` — git UI views

These all follow the same pattern: create a `BufferDiff`, compute a diff against some base text, register it on a multibuffer. In the new model, these would either:
- (Option A) Create a `BufferDiff` entity wrapping the buffer — same pattern, just the entity internals change
- (Option B) Set the diff base directly on the buffer — but this may conflict if the buffer already has a git diff base

This is one of the stronger arguments for option A (separate entity).

---

## Key Files to Study

Read these to understand the current architecture and what changes:

| File | What to Look At |
|------|----------------|
| `crates/text/src/text.rs` | `Buffer`, `BufferSnapshot`, `Fragment`, `FragmentTextSummary`, `apply_local_edit` (~L842). This is where diff state would go in option C. |
| `crates/language/src/buffer.rs` | `Buffer` (~L101), `BufferSnapshot` (~L188). This is where diff state would go in option B. Already ~40 fields. |
| `crates/buffer_diff/src/buffer_diff.rs` | `BufferDiff` (~L24), `InternalDiffHunk` (~L115), `compute_hunks` (~L951), `stage_or_unstage_hunks_impl` (~L544), `patch_for_buffer_range` (~L414). The entity that gets restructured or eliminated. |
| `crates/multi_buffer/src/multi_buffer.rs` | `MultiBuffer` (~L74), `Excerpt` (~L734), `ExcerptSummary` (~L795), `DiffTransform` (~L638), `MultiBufferCursor` (~L1035-7290), `sync_diff_transforms` (~L3129), `diff_hunks_in_range` (~L3863), `MultiBufferChunks` (~L8117). The main site of structural change. |
| `crates/editor/src/split.rs` | `SplittableEditor`, `LhsEditor` (~L338), `sync_path_excerpts` (~L1871), `sync_cursor_to_other_side` (~L732). Gets largely deleted. |
| `crates/editor/src/display_map.rs` | `Companion` (~L232), `CompanionExcerptPatch` (~L183). Gets deleted. |
| `crates/editor/src/display_map/block_map.rs` | `spacer_blocks()` (~L1207). Gets deleted. |
| `crates/editor/src/element.rs` | `diff_status` usage (~search for `DiffHunkStatusKind`). Changes to read from excerpt's diff snapshot. |
| `crates/project/src/git_store.rs` | `BufferGitState` (~L111), `recalculate_diffs` (~L3193), `diff_bases_changed` (~L3137). The orchestrator that triggers diff computation. |
| `crates/rope/src/rope.rs` | `TextSummary`, `TextDimension`, `Chunks`. Unchanged by this design. |
| `crates/editor/src/split_editor_view.rs` | `SplitEditorView`. Gets rewritten to use one multibuffer with two display pipelines. |

---

## Migration Strategy

The migration is incremental. Each phase is independently testable.

### Phase 1: Add diff region types
Define `DiffRegion`, `DiffRegionKind`, `DiffRegionSummary`, `DiffStats`, `DiffTextSummary`, `DiffBase` in the `text` crate. Implement `sum_tree::Item` for `DiffRegion`. Write comprehensive unit tests for the SumTree operations: seeking by buffer dimension, seeking by base dimension, range queries for stats, handling of zero-width Removed regions.

### Phase 2: Add diff base storage
Depending on the option chosen: add `Option<DiffBase>` to `language::BufferSnapshot` (option B), or restructure `BufferDiff` to use `DiffBaseState` internally (option A). Implement `set_diff_base`, `diff_text_summary_for_range`, `diff_regions`, `diff_stats_for_range`, coordinate translation APIs. Implement edit-time region summary updates.

### Phase 3: Wire diff computation
Add a function that converts libgit2 `GitPatch` output to `DiffBase` (base text `Rope` + `SumTree<DiffRegion>` + word diffs). Wire this into the existing diff computation path — produce `DiffBase` alongside (not instead of) the existing `SumTree<InternalDiffHunk>`. Validate that both representations agree.

### Phase 4: Add dual dimensions and stats to excerpts
Add `diff_base_summary: TextSummary` and `diff_stats: DiffStats` to `Excerpt`. Add `diff_base: MBTextSummary` and `diff_stats: DiffStats` to `ExcerptSummary`. Populate during excerpt construction. Implement `DiffBaseOffset` and `DiffBasePoint` dimension types. The existing `DiffTransform` tree is still present and still used. The new dimensions are purely additive.

### Phase 5: Build diff-aware chunk iteration
Add `DiffSide` parameter to `MultiBufferChunks` (or a parallel method). For `DiffSide::Buffer`: existing behavior. For `DiffSide::Base`: iterate base text for Removed regions, buffer text for Unchanged. Validate output against existing system.

### Phase 6: Build new split renderer
One multibuffer, two display pipelines. Wire into `SplitEditorView` behind a feature flag. Extensive visual comparison testing.

### Phase 7: Remove old infrastructure
Delete `DiffTransform` tree, `sync_diff_transforms`, lockstep cursor logic, `Companion`, `LhsEditor`, spacer blocks, excerpt mirroring, `is_inverted`, `add_diff`/`add_inverted_diff`, O(n) coordinate translation. Full test suite pass.

---

## Questions for the Next Agent

1. **Option A vs B vs C** — help us decide. Study the non-git callers, the `language::Buffer` struct size, the edit-time update path, and the lifecycle implications. Make a concrete recommendation with reasoning.

2. **Edit-time region update mechanics** — trace through `apply_local_edit` (~L842 in `text.rs`) and show exactly where and how the region tree update would happen. Handle the multi-region-spanning edit case concretely.

3. **`diff_text_summary_for_range` implementation** — write detailed pseudocode showing the cursor walk, handling of partial overlaps at range boundaries, and the zero-width Removed region edge case.

4. **Split view display pipeline fork** — how does the left-side pipeline get its "edits since" stream? The base text doesn't have CRDT versions. When a diff base changes, what edits does the left-side pipeline see?

5. **Syntax highlighting of base text** — today the base text is an `Entity<language::Buffer>` that gets parsed for syntax highlighting. In the new model the base text is a `Rope`. How does the left panel of split view get syntax highlighting for Removed regions?

6. **The `MultiBufferDiffHunk` → `DiffRegion` mapping** — trace through `diff_hunks_in_range` (~L3863) and show what the new implementation looks like, including how `secondary_status` (staging indicators) is populated.

7. **Hunk expand/collapse for inline diff** — flesh out the BlockMap-based approach. How does the editor track which hunks are expanded? How does it handle expand/collapse of a hunk that straddles an excerpt boundary?

---

## Summary

The fundamental shift: diff awareness moves from being bolted onto the multibuffer via a separate `DiffTransform` tree and `Companion` synchronization system, to being a native property of the content — baked into the summaries that flow through the SumTree. This gives us O(log n) coordinate translation, O(1) diff stats, a single-cursor `MultiBufferCursor`, no lockstep walking, no excerpt mirroring, no spacer blocks, and a split view built from one multibuffer with two display pipelines instead of two synchronized editors.

The main open question is where the diff state lives (separate entity vs merged into the buffer). Everything downstream of that decision is the same.