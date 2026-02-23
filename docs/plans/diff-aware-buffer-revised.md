# Diff-Aware Buffer Redesign — Revised Plan

## Overview

This document is the current source of truth for the diff-aware buffer redesign. It supersedes the earlier planning documents (`diff-aware-buffer.md` and `diff-aware-buffer-handoff.md`) which remain useful as background reading for the original architecture and its problems.

**The fundamental shift:** Diff awareness moves from being bolted onto the multibuffer via a separate `DiffTransform` tree and `Companion` synchronization system, to being a native property of the content — baked into the summaries that flow through the SumTree, threaded as metadata through every display pipeline layer, and rendered by a single diff-aware editor.

Three architectural pillars:

1. **The MultiBuffer is composed on `Entity<BufferDiff>`.** `BufferDiff` wraps a buffer and optionally holds a diff base. A buffer with no diff is the degenerate case. Different views of the same buffer can have independent diffs.

2. **A `DiffMap` pipeline stage produces a unified diff text stream.** It sits between the MultiBuffer and InlayMap, introducing Removed text from the base and tagging every chunk with `diff_status`. Downstream layers carry this metadata through, and some make behavioral decisions based on it (editability, fold boundaries, etc.).

3. **One diff-aware editor renders everything.** Split view is a rendering mode of the `EditorElement`, not a structural concept. There are no two synchronized editors — one editor, one multibuffer, one display pipeline, one cursor. The renderer draws two panels from the same display snapshot, routing content to left/right based on `diff_status`.

---

## Architecture Decision: Separate `BufferDiff` Entity

The `BufferDiff` entity is the fundamental unit that the `MultiBuffer` is composed on. It wraps an `Entity<language::Buffer>` and optionally holds a diff base (and secondary diff base for staging).

### Rationale

Different views of the same live buffer legitimately need different diffs simultaneously:

- The **regular editor** shows a git diff (HEAD → working copy) for gutter indicators.
- The **agent diff pane** shows an action-log diff (pre-agent state → working copy) for the same buffer.
- During **agent review**, the action-log diff replaces the git diff on the regular editor's multibuffer (`agent_diff.rs` ~L1489 adds the action log's diff to the regular editor's multibuffer).

If diff state lived on `language::Buffer` itself, these scenarios would require mutating the buffer's diff base — clobbering the git state for every other view. The separate entity makes the diff-to-buffer association a property of the view (multibuffer), not the data (buffer).

Many current `BufferDiff` constructions are artifacts of the current architecture. The git UI views (`commit_view`, `file_diff_view`, `multi_diff_view`), the edit prediction rating modal, and similar callers all create purpose-built buffers that aren't shared with the editor. For these, setting a diff base directly on the buffer would work fine. But the action log and agent diff cases are genuine multi-diff scenarios that require independent diff entities wrapping the same live buffer.

### What Stays the Same

- `language::Buffer` is **unchanged**. It does not gain diff fields.
- `text::Buffer` and `text::BufferSnapshot` are **unchanged**.
- The rope, CRDT, fragment tree, and all text operations are unmodified.

### What Changes

- `BufferDiff` is restructured: base text becomes a `Rope` (not `Entity<language::Buffer>`), and the hunk tree becomes `SumTree<DiffRegion>`. This eliminates one entity per diff compared to today.
- `MultiBuffer` is composed on `Entity<BufferDiff>`. Its `buffers` map becomes `buffer_diffs`. Excerpt snapshots carry diff region information.
- The `DiffTransform` tree, `Companion` system, `LhsEditor`, spacer blocks, and excerpt mirroring are all eliminated.
- A new `DiffMap` display pipeline stage handles the unified diff text stream.
- The editor and `EditorElement` become natively diff-aware. Split view is a rendering mode, not a structural concept.

---

## Data Model

### `DiffRegion` and the Region Tree

The relationship between buffer text and base text is described by a `SumTree<DiffRegion>`. Each region is classified:

- **Unchanged** — text present in both buffer and base (identical content). `buffer_len > 0`, `base_len > 0`, and they are equal.
- **Added** — text present in buffer only. `buffer_len > 0`, `base_len == 0`.
- **Removed** — text present in base only. `buffer_len == 0`, `base_len > 0`.

The tree uses length-based boundaries, not anchors. Base text is a `Rope` with no CRDT, so anchor-based boundaries don't work for the base side.

**Key invariants:**

- The cumulative `buffer_len` across all regions equals the buffer's text length.
- The cumulative `base_len` across all regions equals the base text length.
- Adjacent regions never have the same `kind` (they would be merged).

```rust
// In the `text` crate:

pub enum DiffRegionKind {
    Unchanged,
    Added,
    Removed,
}

pub struct DiffRegion {
    pub kind: DiffRegionKind,
    pub buffer_len: usize,
    pub base_len: usize,
}
```

### Three-Dimensional Summary

`DiffRegionSummary` carries three `TextSummary` dimensions plus `DiffStats`:

```rust
pub struct DiffRegionSummary {
    /// Cumulative buffer text extent (Unchanged + Added regions).
    pub buffer: TextSummary,
    /// Cumulative base text extent (Unchanged + Removed regions).
    pub base: TextSummary,
    /// Cumulative unified diff extent (all regions — the total vertical space).
    pub diff: TextSummary,
    /// Accumulated diff statistics.
    pub stats: DiffStats,
}

pub struct DiffStats {
    pub lines_added: u32,
    pub lines_removed: u32,
    pub bytes_added: usize,
    pub bytes_removed: usize,
}
```

For each region kind:

| Kind      | `buffer`            | `base`                       | `diff`              | `stats`                 |
| --------- | ------------------- | ---------------------------- | ------------------- | ----------------------- |
| Unchanged | buffer text summary | base text summary (= buffer) | buffer text summary | all zeros               |
| Added     | buffer text summary | zero                         | buffer text summary | lines/bytes from buffer |
| Removed   | zero                | base text summary            | base text summary   | lines/bytes from base   |

The `diff` dimension for each region is whichever side has text: for Unchanged and Added it's the buffer summary, for Removed it's the base summary. The total `diff` extent equals `buffer.lines + stats.lines_removed` (equivalently `base.lines + stats.lines_added`). This is the **unified diff height** — the total vertical space needed to display the complete diff.

All three dimensions are seekable in O(log n) through the SumTree. The `diff` dimension is what the display pipeline operates in for vertical layout.

### `DiffStats` in the Summary Algebra

`DiffStats` fields accumulate through SumTree addition (each field is simply added). This means:

- Total diff stats for a multibuffer: `snapshot.excerpts.summary().diff_stats` — **O(1)**.
- Diff stats for a range: SumTree range query — **O(log n)**.
- Diff stats per buffer in a multibuffer: walk relevant excerpts — **O(log n) per excerpt**.

This replaces the current O(n)-in-hunks computation used by `CommitView::calculate_changed_lines` and various diff stats displays.

### `rope::TextSummary` Is Unchanged

The three-dimensional summary (`DiffRegionSummary`) lives at the buffer-diff level, not the rope level. Adding diff fields to `rope::TextSummary` would bloat every chunk summary in every rope by ~50% for a feature most ropes don't use. The rope stays lean.

### `BufferDiff` Entity Structure

```rust
pub struct BufferDiff {
    pub buffer_id: BufferId,
    buffer: Entity<language::Buffer>,
    diff_base: Option<DiffBaseState>,
    secondary_diff_base: Option<DiffBaseState>,
    diff_version: usize,
    _buffer_subscription: gpui::Subscription,
}

struct DiffBaseState {
    text: Rope,
    regions: SumTree<DiffRegion>,
    word_diffs: Vec<WordDiffHunk>,
}
```

Compared to today:

- `SumTree<InternalDiffHunk>` is replaced by `SumTree<DiffRegion>`.
- `Entity<language::Buffer>` for base text is replaced by `Rope`. This eliminates one entity per diff and removes the need for CRDT machinery on base text.
- The `BufferDiff` subscribes to the buffer for edit-time region updates (via `_buffer_subscription`).
- The secondary diff (for staging status) is inlined as a `DiffBaseState` rather than being a separate `Entity<BufferDiff>`.

### Snapshot Structure

```rust
pub struct BufferDiffSnapshot {
    pub buffer: language::BufferSnapshot,
    diff_base: Option<DiffBaseSnapshot>,
    secondary_diff_base: Option<DiffBaseSnapshot>,
    diff_version: usize,
}

struct DiffBaseSnapshot {
    text: Rope,
    regions: SumTree<DiffRegion>,
    word_diffs: Vec<WordDiffHunk>,
}
```

The snapshot carries the buffer snapshot alongside the diff information. When there's no diff, `diff_base` is `None` — the degenerate case. The `MultiBuffer` works with this snapshot type for all excerpt operations.

### Degenerate Case (No Diff)

When a `BufferDiff` has no base text (`diff_base: None`):

- The region tree is empty.
- `diff_text_summary_for_range` returns `buffer: TextSummary, base: zero, diff: buffer, stats: zero`.
- The excerpt's `diff_base_summary` is zero and `diff_stats` is zero.
- The `diff` dimension equals the `text` dimension.
- The `DiffMap` pipeline stage is a passthrough (zero cost).

This means every buffer can flow through the same diff-aware pipeline with no overhead when there's no diff.

---

## MultiBuffer Composition on `BufferDiff`

### Structural Changes

`MultiBuffer`'s fundamental unit changes from `Entity<Buffer>` to `Entity<BufferDiff>`:

```rust
pub struct MultiBuffer {
    snapshot: RefCell<MultiBufferSnapshot>,
    buffer_diffs: BTreeMap<BufferId, BufferDiffState>,
    // ... other fields unchanged ...
}

struct BufferDiffState {
    diff: Entity<BufferDiff>,
    last_version: RefCell<clock::Global>,
    last_diff_version: Cell<usize>,
    last_non_text_state_update_count: Cell<usize>,
    excerpts: Vec<Locator>,
    _subscriptions: [gpui::Subscription; 2], // buffer events + diff events
}
```

The `diffs: TreeMap<BufferId, DiffState>` map and the `DiffTransform` SumTree are eliminated. The `add_diff()` / `add_inverted_diff()` methods are eliminated.

### Excerpt Changes

The `Excerpt` carries the `BufferDiffSnapshot` and three summary dimensions:

```rust
struct Excerpt {
    id: ExcerptId,
    locator: Locator,
    buffer_id: BufferId,
    buffer_diff: BufferDiffSnapshot,
    range: Range<text::Anchor>,
    max_buffer_row: BufferRow,
    text_summary: TextSummary,
    diff_base_summary: TextSummary,
    diff_summary: TextSummary,      // unified diff extent
    diff_stats: DiffStats,
    has_trailing_newline: bool,
}
```

`ExcerptSummary` carries all four accumulable fields:

```rust
pub struct ExcerptSummary {
    excerpt_id: ExcerptId,
    excerpt_locator: Option<Locator>,
    widest_line_number: u32,
    text: MBTextSummary,
    diff_base: MBTextSummary,
    diff: MBTextSummary,       // unified diff extent
    diff_stats: DiffStats,
}
```

The `diff` field serves as the primary vertical dimension for the display pipeline. In the degenerate case (no diff), `diff == text`.

### API Surface

```rust
impl MultiBuffer {
    /// Primary constructor — takes a BufferDiff.
    pub fn singleton(buffer_diff: Entity<BufferDiff>, cx: &mut Context<Self>) -> Self;

    /// Convenience — wraps a buffer in a no-diff BufferDiff.
    pub fn singleton_buffer(buffer: Entity<Buffer>, cx: &mut Context<Self>) -> Self;

    pub fn push_excerpts(
        &mut self,
        buffer_diff: Entity<BufferDiff>,
        ranges: impl IntoIterator<Item = ExcerptRange<Point>>,
        cx: &mut Context<Self>,
    ) -> Vec<ExcerptId>;

    /// Convenience — wraps buffer in a no-diff BufferDiff.
    pub fn push_buffer_excerpts(
        &mut self,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = ExcerptRange<Point>>,
        cx: &mut Context<Self>,
    ) -> Vec<ExcerptId>;
}
```

Callers that work with diffs (editor, agent diff, git UI) use the primary `Entity<BufferDiff>` API. Callers that don't care about diffs (search results, scratch buffers, completion menus) use the convenience methods.

### `MultiBufferCursor` Simplification

The current `MultiBufferCursor` walks two SumTrees in lockstep: the excerpt tree and the `DiffTransform` tree. This is ~200 lines of complex cursor logic.

In the new model, the cursor walks a single excerpt tree. The diff information is a native property of each excerpt's summary. The `DiffTransform` cursor, lockstep logic, `DiffTransformSummary`, and all associated dimension types are eliminated.

```rust
struct MultiBufferCursor<'a, MBD, BD> {
    excerpts: Cursor<'a, 'static, Excerpt, ExcerptDimension<MBD>>,
    cached_region: OnceCell<Option<MultiBufferRegion<'a, MBD, BD>>>,
}
```

### Sync Path

When the buffer is edited or the diff changes, the sync path rebuilds the affected excerpt with a fresh `BufferDiffSnapshot`. The three summary dimensions (`text`, `diff_base`, `diff`) and `diff_stats` are recomputed from the snapshot's region tree. The `sync_diff_transforms` function (~300 lines) and `recompute_diff_transforms_for_edit` (~200 lines) are eliminated entirely.

---

## The Unified Diff Coordinate Space

The three-dimensional summary on `DiffRegionSummary` provides a **unified diff coordinate space** that is the key to the entire design.

### Why Three Dimensions

- **`buffer`** — the working copy text. Offsets, points, and anchors refer to this space. This is the coordinate space of the CRDT buffer.
- **`base`** — the diff base text. Used for reading Removed text, coordinate translation, and staging operations.
- **`diff`** — the unified diff extent. This is the total vertical space needed to display the complete diff. Both sides' text contributes: Unchanged lines appear once, Added lines appear once, Removed lines appear once. The total is `unchanged_lines + added_lines + removed_lines`.

### Vertical Space Accounting

The `diff` dimension answers the question: "how tall is this diff?" For a concrete example:

```
Unchanged: "hello\nworld\n"      → 2 buffer lines, 2 base lines, 2 diff lines
Removed:   "old line\n"          → 0 buffer lines, 1 base line,  1 diff line
Added:     "new line 1\nnew 2\n" → 2 buffer lines, 0 base lines, 2 diff lines
Unchanged: "goodbye\n"           → 1 buffer line,  1 base line,  1 diff line
```

Total: buffer = 5 lines, base = 4 lines, diff = 6 lines.

In the unified diff stream (what the display pipeline processes):

- Row 0: "hello\n" — diff_status: Unchanged
- Row 1: "world\n" — diff_status: Unchanged
- Row 2: "old line\n" — diff_status: Removed (from base text)
- Row 3: "new line 1\n" — diff_status: Added (from buffer text)
- Row 4: "new 2\n" — diff_status: Added (from buffer text)
- Row 5: "goodbye\n" — diff_status: Unchanged

In split view rendering, each panel sees 6 rows:

- Left panel: rows 0-1 text, row 2 text (Removed, red bg), rows 3-4 gap (Added), row 5 text.
- Right panel: rows 0-1 text, row 2 gap (Removed), rows 3-4 text (Added, green bg), row 5 text.

The alignment is free — both panels have 6 rows because both are rendering from the same unified diff stream. Scroll position is one value. No synchronization logic.

### Coordinate Translation

All coordinate translations are O(log n) via the region tree:

- `to_diff_base_offset(buffer_offset) -> base_offset` — seek by buffer dimension, read base dimension.
- `from_diff_base_offset(base_offset) -> buffer_offset` — seek by base dimension, read buffer dimension.
- `to_diff_offset(buffer_offset) -> diff_offset` — seek by buffer dimension, read diff dimension.
- `from_diff_offset(diff_offset) -> buffer_offset` — seek by diff dimension, read buffer dimension.
- Analogous `Point` variants for all of the above.

### How the Dimensions Flow Through the Architecture

```
DiffRegion SumTree
  └── DiffRegionSummary { buffer, base, diff, stats }

BufferDiffSnapshot
  └── Contains the region tree, provides O(log n) queries in all three dimensions

Excerpt
  └── text_summary (buffer), diff_base_summary (base), diff_summary (diff), diff_stats

ExcerptSummary
  └── text (buffer), diff_base (base), diff (diff), diff_stats

MultiBufferSnapshot
  └── excerpts: SumTree<Excerpt> — seekable by all four accumulated dimensions

DiffMap (new pipeline stage)
  └── Operates in the `diff` dimension
  └── Produces unified text stream with diff_status on every chunk

InlayMap → FoldMap → TabMap → WrapMap → BlockMap
  └── All carry diff_status through their Chunk types
  └── Some make behavioral decisions (editability, fold boundaries)

EditorElement
  └── In split mode: draws two panels from the same display snapshot
  └── Routes content to left/right based on diff_status
```

---

## Display Pipeline: The `DiffMap`

### Position in the Pipeline

```
MultiBuffer(BufferDiff)
  → DiffMap          ← NEW: produces unified diff text stream
    → InlayMap
      → FoldMap
        → TabMap
          → WrapMap
            → BlockMap
              → display rows
```

### What the DiffMap Does

The DiffMap takes the `MultiBufferSnapshot` (which carries diff region information in its excerpts) and produces a unified text stream in the `diff` coordinate space. Each chunk in this stream carries a `diff_status` annotation.

For a buffer with no diff (degenerate case), the DiffMap is a zero-cost passthrough: diff coordinates equal buffer coordinates, and all chunks have `diff_status: None`.

For a buffer with a diff, the DiffMap:

1. **Iterates through excerpt diff regions** in order.
2. For **Unchanged** regions: yields the buffer text with `diff_status: Unchanged`.
3. For **Added** regions: yields the buffer text with `diff_status: Added`.
4. For **Removed** regions (when expanded): yields the base text (read from the diff base `Rope`) with `diff_status: Removed`.
5. For **Removed** regions (when collapsed in inline mode): yields nothing (the region is elided from the stream). A gutter indicator marks the hunk boundary.

### DiffMap Transforms

Like other pipeline stages, the DiffMap maintains a `SumTree<DiffMapTransform>` with input/output summary pairs:

```rust
enum DiffMapTransform {
    /// Pass-through: input excerpt text maps directly to output.
    /// Covers Unchanged and Added regions (their text is in the buffer).
    Isomorphic(TextSummary),

    /// Inserted content from the diff base that doesn't exist in the
    /// MultiBuffer's buffer coordinate space.
    DiffBaseInsertion {
        output: TextSummary,
        buffer_id: BufferId,
        base_byte_range: Range<usize>,
    },

    /// Elided content: a collapsed Removed region that produces no output.
    /// In split view, this variant is never used (all hunks are expanded).
    Elided,
}
```

### Expand/Collapse State

The DiffMap holds the expand/collapse state for inline diff hunks:

- **Inline view, collapsed hunk (default):** The Removed region produces an `Elided` transform. No base text appears in the stream. The gutter shows a diff indicator.
- **Inline view, expanded hunk:** The Removed region produces a `DiffBaseInsertion` transform. The base text appears in the stream with `diff_status: Removed`.
- **Split view:** All hunks are always expanded. Every Removed region produces a `DiffBaseInsertion`.

The editor tracks which hunks are expanded via a set of hunk identifiers (buffer anchors that survive edits). The DiffMap consults this set when building its transforms.

### The `sync` Method

The DiffMap's `sync` follows the same pattern as `InlayMap::sync` (see `inlay_map.rs` ~L555):

1. Takes a new `MultiBufferSnapshot` and a set of `Edit<MultiBufferOffset>` edits.
2. Rebuilds the affected portion of its transform tree.
3. Produces transformed `Edit<DiffOffset>` edits for the layer above (InlayMap).

When the diff base changes (new diff computation completes), the DiffMap receives this as an excerpt rebuild from the MultiBuffer. The affected regions get new transforms. In the worst case (full diff recomputation changes many regions), the DiffMap rebuild touches all transforms for that excerpt — but this is bounded by the number of diff regions, not the text size.

---

## Diff Awareness Through the Pipeline

The layers above the DiffMap are not diff-unaware — they carry diff metadata and some make behavioral decisions based on it. This follows the exact same pattern as `is_inlay` today: the InlayMap sets it, every subsequent layer's `Chunk` type carries it, and the renderer consumes it.

### The `diff_status` Field

Every layer's `Chunk` type gains a `diff_status` field:

```rust
pub enum DiffChunkStatus {
    /// Normal buffer text, or Unchanged region in a diff. Editable.
    Unchanged,
    /// Added text from the buffer. Editable. Addition styling.
    Added,
    /// Removed/base text from the diff base. Not editable. Deletion styling.
    Removed,
}
```

This appears on `language::Chunk`, the fold map's `Chunk`, `HighlightedChunk`, and is consumed at every level that needs it.

### Layer-by-Layer Awareness

**DiffMap** — introduces `diff_status`. Handles coordinate transformation between buffer space and diff space. Handles expand/collapse of Removed regions. Reads base text from the diff base `Rope` for Removed regions.

**InlayMap** — carries `diff_status` through its chunks. Inlay positions are buffer anchors, which resolve to positions in Unchanged and Added regions only. Removed text has no buffer anchors, so inlays naturally cannot be placed there. No special logic needed.

**FoldMap** — carries `diff_status` through its chunks. Fold ranges should respect diff region boundaries: a fold should not span from Unchanged into Removed text. The FoldMap can enforce this by checking `diff_status` when validating fold ranges.

**TabMap** — carries `diff_status` through. Tabs are tabs regardless of diff status. No behavioral changes.

**WrapMap** — carries `diff_status` through. Wrapping is based on line width regardless of diff status. No behavioral changes.

**BlockMap** — carries `diff_status` through. Diff hunk controls (stage/unstage buttons, expand/collapse indicators) are positioned based on diff region boundaries. These are custom blocks placed at hunk boundaries, using the same block infrastructure as diagnostics.

**Renderer (EditorElement)** — consumes `diff_status` for:

- Background colors (green for Added, red for Removed).
- Cursor constraints (Removed text is read-only; the cursor can enter it for copy/reference but edits are rejected).
- Gutter rendering (diff status markers, line numbers from buffer vs base).
- Word-diff highlighting within Added/Removed regions.
- Split view panel routing (see below).

### Editability

Edits landing in Removed regions are rejected. This happens at the MultiBuffer level during `convert_edits_to_buffer_edits`, which checks `diff_status` in the same way it currently checks `is_main_buffer` (~L1466 in `multi_buffer.rs`). Removed text has no buffer positions for edits to target — the rejection is structural, not just a flag check.

---

## One Editor, One Pipeline, Split as a Rendering Mode

### The Single Editor Model

Today's split diff uses `SplittableEditor` which maintains two full `Entity<Editor>` instances (`rhs_editor` and `lhs_editor`), each with their own multibuffer, display map, and subscriptions. ~1000 lines of `split.rs` synchronize excerpts, cursors, selections, scroll positions, focus, staging actions, and hunk translation between them.

In the new model: **one editor, one multibuffer, one display pipeline, one cursor.** The `EditorElement` renders one or two panels from the same display snapshot. Split view is a rendering mode, not a structural concept.

### How Split Rendering Works

The display pipeline produces a unified diff text stream. Every display row has a `diff_status`. When the `EditorElement` renders in split mode, for each row:

- **Unchanged** → render text on both panels.
- **Added** → render text on the right panel, draw empty gap on the left panel.
- **Removed** → render text on the left panel, draw empty gap on the right panel.

Both panels always have the same number of visual rows (the unified diff height). Scroll position is one value applied to both panels. Alignment is free.

### Cursor and Selections

The cursor lives in the unified diff coordinate space. Its visual position depends on the `diff_status` of the text it's on:

- Cursor on **Unchanged** text: shown on both panels.
- Cursor on **Added** text: shown on the right panel only.
- Cursor on **Removed** text: shown on the left panel only (read-only mode — can copy but not edit).

The selection model doesn't change. Selections are ranges in the unified text. The renderer draws them on the appropriate panel(s).

### Line Numbers

In split view:

- Left gutter: base text line numbers. Unchanged rows show base line numbers. Removed rows show base line numbers. Added rows show no line number (gap).
- Right gutter: buffer text line numbers. Unchanged rows show buffer line numbers. Added rows show buffer line numbers. Removed rows show no line number (gap).

The line numbers are derived from the `base` and `buffer` dimensions of the diff region tree — O(log n) lookup for any display row.

### Scroll Synchronization

There is no scroll synchronization problem because there is only one scroll position. Both panels scroll together because they render from the same display snapshot. The scroll offset is in unified diff coordinates.

### What Gets Eliminated

The entire `SplittableEditor` / `LhsEditor` / `Companion` apparatus:

- `SplittableEditor` struct and its ~700 lines of logic
- `LhsEditor` struct
- `Companion` struct and companion-related code in `DisplayMap`
- `CompanionExcerptPatch` type
- `sync_path_excerpts` — excerpt mirroring between two multibuffers
- `sync_cursor_to_other_side` — cursor translation
- `convert_lhs_rows_to_rhs` / `convert_rhs_rows_to_lhs` — row conversion
- `translate_lhs_selections_to_rhs` — selection translation
- `translate_lhs_hunks_to_rhs` — hunk translation for staging
- Spacer blocks and balancing blocks in `BlockMap`
- All `is_inverted` / `add_inverted_diff` logic

### Inline vs Split View

These are orthogonal axes:

- **DiffMap expand/collapse state** controls whether Removed text appears in the unified stream. In split view, all hunks are always expanded. In inline view, hunks can be individually expanded/collapsed.
- **EditorElement rendering mode** controls whether to draw one panel (inline) or two panels (split). This is purely a rendering concern.

An editor in diff mode can switch between inline and split rendering without rebuilding the multibuffer, the display pipeline, or any state. The DiffMap adjusts its expand state (expand all for split, respect per-hunk state for inline) and the EditorElement switches its layout.

### Wrapping in Split View

Both panels render from the same display snapshot, so text wrapping is computed once. For split view, the recommended approach is to disable soft wrapping (as most diff viewers do). Both panels use the same font and styling, so line heights are consistent.

If soft wrapping is desired in the future, both panels can share the same wrap width (equal-width panels). Unchanged text wraps identically on both sides. Added/Removed text wraps normally; the gap on the other side matches the wrapped height because it occupies the same display rows.

---

## `BufferDiff` Lifecycle and Edit-Time Updates

### Creation Patterns

```rust
impl BufferDiff {
    /// Wrap a buffer with no diff base. The degenerate case.
    pub fn no_diff(buffer: Entity<Buffer>, cx: &mut Context<Self>) -> Self;

    /// Wrap a buffer. Diff base will be set later via set_diff_base.
    pub fn new(buffer: Entity<Buffer>, cx: &mut Context<Self>) -> Self;

    /// Set the diff base. Typically called after async diff computation completes.
    pub fn set_diff_base(&mut self, base: DiffBaseState, cx: &mut Context<Self>);

    /// Clear the diff base, returning to the degenerate case.
    pub fn clear_diff_base(&mut self, cx: &mut Context<Self>);
}
```

### Diff Computation

The diff algorithm runs on a background thread. The flow is:

1. Something triggers a diff (git HEAD change, file save, buffer edit debounce).
2. `compute_diff_regions(base_text: &Rope, buffer_text: &Rope) -> DiffBaseState` runs on a background thread. This is a pure function that produces a `Rope` + `SumTree<DiffRegion>` + `Vec<WordDiffHunk>`.
3. The result is written to the `BufferDiff` entity on the main thread via `set_diff_base`.
4. The `BufferDiff` increments `diff_version` and notifies.
5. The `MultiBuffer` receives the notification, rebuilds the affected excerpt with the new `BufferDiffSnapshot`, and emits its own change event.
6. The `DiffMap` syncs with the new `MultiBufferSnapshot`, rebuilding transforms for the affected regions.

The diff computation converts libgit2's `GitPatch` output to `SumTree<DiffRegion>`. Each patch hunk maps to a sequence of Removed + Added regions (with Unchanged regions filling the gaps between hunks). Word-level diffs within hunks are computed simultaneously.

### Edit-Time Region Summary Updates

When the buffer is edited between diff recomputations, the region tree's buffer-side lengths need updating so that the cumulative `buffer_len` invariant holds.

The `BufferDiff` subscribes to the buffer and receives each edit as a `Patch<usize>`. The update algorithm:

1. For each edit in the patch, seek the region tree's `buffer` dimension to find affected regions.
2. Use the SumTree cursor's slice-and-rebuild pattern to reconstruct affected regions with adjusted `buffer_len` values.
3. Recompute the `buffer` `TextSummary` for touched regions from the actual buffer text.
4. Leave `base_len` and `base` summary unchanged.
5. Leave `kind` classifications unchanged (they may become stale — the next diff recomputation fixes them).
6. If an edit completely erases a non-Removed region (buffer_len becomes 0), leave it as a zero-width item. The next recomputation cleans it up.

The update is O(log n + k) where k is the number of touched regions (typically 1–2 for a single edit). The subscription fires synchronously during the same update that processes the edit, so the region tree is consistent before the MultiBuffer's own buffer-edit subscription fires.

**Multi-region spanning edit:** When a user selects across region boundaries and deletes, multiple regions shrink. The algorithm walks all affected regions, distributes the deletion across them (each loses its overlap with the edit range), and adds the insertion to the first affected region. The invariant (total buffer_len == buffer text length) is maintained.

### `diff_text_summary_for_range` Implementation

This is the core query. Given a buffer offset range `start..end`, walk the `DiffRegion` tree and accumulate `buffer`, `base`, and `diff` `TextSummary` values plus `DiffStats`.

**Algorithm:**

1. Seek the region tree's `buffer` dimension to `start` with `Bias::Left`. This ensures we don't skip zero-width Removed regions at the seek position.
2. If the cursor lands in the middle of a region, compute the partial contribution for the suffix of that region.
3. Walk forward through complete regions, accumulating their full summaries.
4. If the last region extends past `end`, compute the partial contribution for the prefix of that region.
5. For partial contributions: Unchanged and Added regions contribute buffer text (compute TextSummary from the actual buffer rope for the relevant byte range). Removed regions contribute base text (compute TextSummary from the base rope).

**Zero-width Removed region handling:** Removed regions have `buffer_len == 0` and sit at a single point in buffer-offset space. When seeking with `Bias::Left`, the cursor lands before these zero-width items, ensuring they're included in the iteration if they fall within the query range. At `range.end`, zero-width Removed regions at exactly `range.end` are excluded (half-open range semantics).

---

## Rendering Concerns

### `RowInfo.diff_status`

Today, `RowInfo.diff_status` is populated by looking up a separate `DiffStateSnapshot` map. In the new model, it comes directly from the DiffMap's chunk metadata, which flows through all pipeline layers. The diff status for any display row is available without a separate lookup.

### `MultiBufferDiffHunk` Population

`diff_hunks_in_range` walks excerpts and queries each excerpt's diff regions:

1. For each excerpt in the range, iterate its region tree.
2. Adjacent `Removed` + `Added` regions at the same boundary are coalesced into a `Modified` hunk.
3. Buffer ranges are mapped to multibuffer ranges via the excerpt offset.
4. The `secondary_status` field (staging indicators) is computed by looking up the corresponding range in the secondary diff base's region tree.

The returned `MultiBufferDiffHunk` type remains largely the same. The computation is O(log n) per excerpt (seeking to the range) plus O(k) for the number of hunks in the range.

### Syntax Highlighting of Base Text

In the new model, base text is a `Rope`, not a `language::Buffer`. For syntax highlighting of Removed text in split view and inline expanded hunks:

**Initial approach:** Removed text renders without syntax highlighting, using flat deletion styling. This is acceptable because removed text is already prominently styled (red background) and the diff status is the primary visual signal.

**Future enhancement:** When a `BufferDiff` sets its diff base, optionally parse the base `Rope` with the same language grammar. Store the resulting syntax tree on `DiffBaseState`. The DiffMap can then provide syntax-highlighted chunks for Removed regions.

### Split View Line Numbers

In split mode, the `EditorElement` draws two gutter columns:

- **Left gutter:** Shows base text line numbers. For Unchanged rows, the base line number increments. For Removed rows, the base line number increments. For Added rows, no number is shown (gap). The base line number at any display row is derived from the `base` dimension of the diff region tree.

- **Right gutter:** Shows buffer text line numbers. For Unchanged rows, the buffer line number increments. For Added rows, the buffer line number increments. For Removed rows, no number is shown (gap). The buffer line number is the standard line number from the buffer.

---

## Staging

### Staging Stays as a Git-Specific Concern

Staging (modifying git's index) needs:

- HEAD text and index text (managed by `BufferGitState` in `git_store.rs`).
- Hunk boundaries (derived from the diff region tree).
- The buffer's working copy text.

### Primary and Secondary Diffs

The primary diff is HEAD → working copy. The secondary diff is index → working copy. Both are stored on the `BufferDiff` entity as `DiffBaseState` fields.

Staging status for each hunk is derived by comparing the two region trees: if a hunk appears in both diffs, it's unstaged. If it appears only in the primary, it's staged. If it appears only in the secondary, it's a pending addition/removal.

### `stage_or_unstage_hunks` Migration

The staging operation reads hunk boundaries from the primary diff's region tree, reads buffer text and HEAD text, and splices them to produce new index text. The secondary diff's region tree provides the mapping between buffer offsets and index offsets.

This is conceptually the same as today's `stage_or_unstage_hunks_impl`, but operates on `SumTree<DiffRegion>` ranges instead of `InternalDiffHunk` ranges. The O(log n) coordinate translation from the region tree replaces the O(n) offset conversion.

---

## Non-Git Callers

### Callers That Use Purpose-Built Buffers

These callers create dedicated buffers for diff display. In the new model, they create a `BufferDiff` wrapping their buffer and set the diff base:

| Caller                     | Pattern                                                                     |
| -------------------------- | --------------------------------------------------------------------------- |
| `commit_view.rs`           | Create buffer at commit OID, wrap in `BufferDiff`, set base to old OID text |
| `file_diff_view.rs`        | Create old/new buffers, wrap new in `BufferDiff`, set base to old text      |
| `multi_diff_view.rs`       | Same as file_diff_view                                                      |
| `rate_prediction_modal.rs` | Create result buffer, wrap in `BufferDiff`, set base to original text       |

### Callers That Diff Live Editing Buffers

These callers create independent diffs of buffers the user is actively editing:

| Caller                   | Pattern                                                                                                                         |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| `action_log.rs`          | Wraps the live buffer in a `BufferDiff` with base = pre-edit state. Independent of the git diff.                                |
| `acp_thread/src/diff.rs` | Wraps the live buffer in a `BufferDiff` with base = pre-agent state.                                                            |
| `agent_diff.rs`          | Uses the action log's `BufferDiff`. Also pushes it into the regular editor's multibuffer during review, replacing the git diff. |
| `editor.rs`              | Gets the git diff `BufferDiff` from `open_uncommitted_diff` and uses it in the editor's multibuffer.                            |

In all cases, the caller creates an `Entity<BufferDiff>` wrapping the buffer, then passes it to `MultiBuffer::push_excerpts` or `MultiBuffer::singleton`. The same buffer entity can be wrapped by multiple independent `BufferDiff` entities with different base texts.

---

## What Gets Eliminated

| Component                                                                    | Location                      | Status                                |
| ---------------------------------------------------------------------------- | ----------------------------- | ------------------------------------- |
| `SumTree<DiffTransform>`                                                     | `multi_buffer.rs` ~L610       | **Removed**                           |
| `DiffTransform` enum                                                         | `multi_buffer.rs` ~L638       | **Removed**                           |
| `DiffTransformSummary`                                                       | `multi_buffer.rs` ~L804       | **Removed**                           |
| `DiffTransformHunkInfo`                                                      | `multi_buffer.rs` ~L656       | **Removed**                           |
| `sync_diff_transforms` (~300 lines)                                          | `multi_buffer.rs` ~L3129      | **Removed**                           |
| `recompute_diff_transforms_for_edit` (~200 lines)                            | `multi_buffer.rs` ~L3267      | **Removed**                           |
| `push_buffer_content_transform`                                              | `multi_buffer.rs` ~L3510      | **Removed**                           |
| `extend_last_buffer_content_transform`                                       | `multi_buffer.rs` ~L3546      | **Removed**                           |
| `append_diff_transforms` / `push_diff_transform`                             | `multi_buffer.rs` ~L3471/3494 | **Removed**                           |
| `MultiBufferCursor` lockstep logic (~200 lines)                              | `multi_buffer.rs` ~L6975      | **Simplified to single cursor**       |
| `MultiBufferExcerpt.diff_transforms` cursor                                  | `multi_buffer.rs` ~L759       | **Removed**                           |
| `diffs: TreeMap<BufferId, DiffStateSnapshot>`                                | `multi_buffer.rs` ~L609       | **Removed**                           |
| `DiffState` / `DiffStateSnapshot`                                            | `multi_buffer.rs` ~L522/538   | **Removed**                           |
| `add_diff()` / `add_inverted_diff()`                                         | `multi_buffer.rs` ~L2609/2626 | **Removed**                           |
| `buffer_diff_changed()` / `inverted_buffer_diff_changed()`                   | `multi_buffer.rs` ~L2374/2418 | **Removed**                           |
| `has_inverted_diff` flag                                                     | `multi_buffer.rs` ~L616       | **Removed**                           |
| `all_diff_hunks_expanded` / `show_deleted_hunks` / `use_extended_diff_range` | `multi_buffer.rs` ~L622-624   | **Moved to DiffMap**                  |
| `SplittableEditor` struct (~700 lines)                                       | `split.rs` ~L329              | **Removed**                           |
| `LhsEditor` struct                                                           | `split.rs` ~L338              | **Removed**                           |
| `Companion` struct (~100 lines)                                              | `display_map.rs` ~L232        | **Removed**                           |
| `CompanionExcerptPatch`                                                      | `display_map.rs` ~L183        | **Removed**                           |
| `sync_path_excerpts`                                                         | `split.rs` ~L1871             | **Removed**                           |
| `sync_cursor_to_other_side`                                                  | `split.rs` ~L732              | **Removed**                           |
| `convert_lhs_rows_to_rhs` / `convert_rhs_rows_to_lhs`                        | `split.rs` ~L41-57            | **Removed**                           |
| `translate_lhs_selections_to_rhs` / `translate_lhs_hunks_to_rhs`             | `split.rs` ~L76/142           | **Removed**                           |
| Spacer blocks / balancing blocks                                             | `block_map.rs` ~L1207         | **Removed**                           |
| `SumTree<InternalDiffHunk>`                                                  | `buffer_diff.rs`              | **Replaced by `SumTree<DiffRegion>`** |
| Base text as `Entity<language::Buffer>`                                      | `buffer_diff.rs`              | **Replaced by `Rope`**                |
| `is_inverted` concept                                                        | throughout                    | **Removed**                           |
| `patch_for_buffer_range` O(n)                                                | `buffer_diff.rs` ~L414        | **Replaced by O(log n) region seek**  |
| `patch_for_base_text_range` O(n)                                             | `buffer_diff.rs` ~L462        | **Replaced by O(log n) region seek**  |

## What Gets Added

| Component                                                          | Purpose                                                            |
| ------------------------------------------------------------------ | ------------------------------------------------------------------ |
| `DiffRegion` / `DiffRegionKind` / `DiffRegionSummary`              | Region tree items with three-dimensional summary                   |
| `DiffStats`                                                        | Accumulated diff statistics                                        |
| `DiffChunkStatus` enum                                             | Chunk-level diff annotation carried through all pipeline layers    |
| `DiffBaseState` / `DiffBaseSnapshot`                               | Container for base text `Rope` + region tree + word diffs          |
| `diff_base_summary` field on `Excerpt`                             | Base text dimension                                                |
| `diff_summary` field on `Excerpt`                                  | Unified diff extent dimension                                      |
| `diff_base` and `diff` fields on `ExcerptSummary`                  | Accumulated base and diff dimensions                               |
| `diff_stats` field on `Excerpt` and `ExcerptSummary`               | Accumulated diff statistics                                        |
| `DiffBaseOffset` / `DiffBasePoint` dimension types                 | Seeking by base dimension in excerpt tree                          |
| `DiffOffset` / `DiffPoint` dimension types                         | Seeking by unified diff dimension                                  |
| `DiffMap` pipeline stage                                           | Produces unified diff text stream between MultiBuffer and InlayMap |
| `DiffMapTransform` / `DiffMapSnapshot`                             | DiffMap's SumTree and snapshot types                               |
| `diff_status` field on all pipeline `Chunk` types                  | Diff annotation threading                                          |
| Coordinate translation APIs                                        | `to_diff_base_offset`, `to_diff_offset`, etc. — O(log n)           |
| `diff_stats()` / `diff_stats_for_range()` on `MultiBufferSnapshot` | O(1) total stats, O(log n) range stats                             |
| `diff_regions()` iterator                                          | Iterate classified regions in a range                              |
| `diff_hunks()` method                                              | Coalesced hunks for gutter/navigation                              |
| `compute_diff_regions()` function                                  | Pure function: `(base_text, buffer_text) → DiffBaseState`          |
| Split rendering mode on `EditorElement`                            | One editor draws two panels based on diff_status                   |

---

## Migration Strategy

The migration is incremental. Each phase is independently testable and deployable.

### Phase 1: Add diff region types (`text` crate)

Define in the `text` crate:

- `DiffRegion`, `DiffRegionKind`
- `DiffRegionSummary` with three `TextSummary` dimensions and `DiffStats`
- `impl sum_tree::Item for DiffRegion`
- `DiffBaseState` (Rope + SumTree<DiffRegion> + Vec<WordDiffHunk>)

Write comprehensive tests:

- SumTree construction from known diff outputs.
- Seeking by `buffer` dimension, `base` dimension, and `diff` dimension.
- Range queries for stats accumulation.
- Zero-width Removed region handling at seek boundaries.
- `diff_text_summary_for_range` correctness with partial overlaps.
- Coordinate translation in all directions.

**No existing behavior changes.** These are purely additive types.

### Phase 2: Restructure `BufferDiff` internals

- Replace `SumTree<InternalDiffHunk>` with `SumTree<DiffRegion>` internally.
- Replace `Entity<language::Buffer>` base text with `Rope`.
- Inline the secondary diff (no longer a separate `Entity<BufferDiff>`).
- Implement edit-time region summary updates (subscribe to buffer edits).
- Implement `diff_text_summary_for_range`, `diff_regions`, coordinate translation APIs.
- Keep the existing `BufferDiffSnapshot` public API working via an adapter layer so downstream consumers don't break yet.

**Existing behavior preserved** through the adapter layer.

### Phase 3: Wire diff computation

- Add a function that converts libgit2 `GitPatch` output to `DiffBaseState`.
- Run both old and new representations in parallel within `BufferDiff`.
- Add assertions that both representations agree on hunk boundaries, offset translations, and stats.
- Once validated, remove the old `InternalDiffHunk` path and the adapter layer.

### Phase 4: Compose MultiBuffer on BufferDiff

- Change `MultiBuffer.buffers` to `buffer_diffs: BTreeMap<BufferId, BufferDiffState>`.
- Change `push_excerpts` to take `Entity<BufferDiff>`.
- Add convenience methods (`singleton_buffer`, `push_buffer_excerpts`) that wrap in a no-diff `BufferDiff`.
- Add `diff_base_summary`, `diff_summary`, `diff_stats` to `Excerpt` and `ExcerptSummary`.
- Populate during excerpt construction from `BufferDiffSnapshot`.
- Implement `DiffBaseOffset`, `DiffBasePoint`, `DiffOffset`, `DiffPoint` dimension types.
- Simplify `MultiBufferCursor` to single-tree cursor.
- The `DiffTransform` tree still exists temporarily — the new dimensions are purely additive.
- Migrate all callers of `add_diff()` to pass `Entity<BufferDiff>` to `push_excerpts` / `singleton`.

### Phase 5: Migrate diff queries to new model

- Implement new `diff_hunks_in_range` using excerpt's region tree.
- Implement `diff_stats()` / `diff_stats_for_range()` on `MultiBufferSnapshot`.
- Populate `RowInfo.diff_status` from the pipeline's diff metadata.
- Migrate all consumers (gutter rendering, hunk navigation, staging UI, commit view stats).
- Validate output matches the old `DiffStateSnapshot`-based queries.

### Phase 6: Build the DiffMap pipeline stage

- Implement `DiffMap`, `DiffMapSnapshot`, `DiffMapTransform`.
- Implement `sync` following the `InlayMap` pattern.
- Implement chunk iteration with `DiffChunkStatus` annotations.
- Implement coordinate translation through the DiffMap layer.
- Add `diff_status` field to `language::Chunk`, fold map's `Chunk`, `HighlightedChunk`.
- Thread `diff_status` through every pipeline layer's chunk iterator.
- Support degenerate (no-diff) mode as a zero-cost passthrough.
- Support inline expanded/collapsed modes.
- Wire into `DisplayMap` between the `MultiBuffer` and `InlayMap`.
- Move hunk expand/collapse state from MultiBuffer to DiffMap.

### Phase 7: Build diff-aware editor rendering

- Add split rendering mode to `EditorElement`: draw two panels from the same display snapshot, routing content to left/right based on `diff_status`.
- Implement split view line numbers (base line numbers on left, buffer on right).
- Handle cursor rendering across panels.
- Handle mouse click hit testing across panels.
- Wire behind a feature flag for visual comparison testing.

### Phase 8: Remove old infrastructure

Delete:

- `DiffTransform` tree and all associated types.
- `sync_diff_transforms` and `recompute_diff_transforms_for_edit`.
- Lockstep cursor logic in `MultiBufferCursor`.
- `SplittableEditor`, `LhsEditor`, and all of `split.rs` except the thin `SplitEditorView` rendering wrapper.
- `Companion`, `CompanionExcerptPatch`, companion-related code in `DisplayMap`.
- Spacer blocks and balancing blocks in `BlockMap`.
- `is_inverted`, `add_diff`, `add_inverted_diff`.
- `DiffState`, `DiffStateSnapshot` on MultiBuffer.
- All O(n) coordinate translation code.

Full test suite pass. Remove feature flag.

---

## Key Files Reference

| File                                         | What to Look At                                                                                                                                               |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/text/src/text.rs`                    | `Buffer`, `BufferSnapshot`, `apply_local_edit` (~L842). The text crate gains the new diff region types but its core types are unchanged.                      |
| `crates/language/src/buffer.rs`              | `Buffer` (~L101), `BufferSnapshot` (~L188). **Unchanged** by this design.                                                                                     |
| `crates/buffer_diff/src/buffer_diff.rs`      | `BufferDiff` (~L24), `InternalDiffHunk` (~L115), `compute_hunks` (~L951), `stage_or_unstage_hunks_impl` (~L544). Gets restructured with new internals.        |
| `crates/multi_buffer/src/multi_buffer.rs`    | `MultiBuffer` (~L74), `Excerpt` (~L734), `ExcerptSummary` (~L795), `DiffTransform` (~L638), `MultiBufferCursor` (~L1035). The main site of structural change. |
| `crates/editor/src/display_map.rs`           | `DisplayMap`, `Companion` (~L232). DiffMap is added as a new pipeline stage, Companion is removed.                                                            |
| `crates/editor/src/display_map/inlay_map.rs` | `InlayMap`, `InlaySnapshot`, `Transform`, `sync`. Pattern to follow for DiffMap implementation.                                                               |
| `crates/editor/src/element.rs`               | `EditorElement` rendering. Gains split panel rendering based on `diff_status`.                                                                                |
| `crates/editor/src/split.rs`                 | `SplittableEditor` (~L329), `LhsEditor` (~L338), `Companion` usage. Gets largely deleted.                                                                     |
| `crates/editor/src/split_editor_view.rs`     | `SplitEditorView` rendering. Simplified to a thin wrapper that sets the editor's rendering mode.                                                              |
| `crates/project/src/git_store.rs`            | `BufferGitState` (~L111), `recalculate_diffs` (~L3193). The orchestrator that triggers diff computation.                                                      |
| `crates/acp_thread/src/diff.rs`              | Agent diff construction. Migrates to new BufferDiff API.                                                                                                      |
| `crates/action_log/src/action_log.rs`        | Action log diff tracking. Migrates to new BufferDiff API.                                                                                                     |
| `crates/git_ui/src/commit_view.rs`           | Commit diff view. Migrates to new BufferDiff API.                                                                                                             |

---

## Open Questions

### 1. Aligned Modified hunks in split view

The simple approach (Removed lines as rows, then Added lines as rows, with gaps on the other side) falls naturally out of the region model. Some diff viewers align Removed and Added lines side-by-side within Modified hunks (matching line 1 with line 1, etc.). This requires intra-hunk alignment logic that could be added as a DiffMap enhancement later, potentially by interleaving Removed and Added regions line-by-line within a coalesced Modified hunk.

### 2. Performance of the degenerate case

When `BufferDiff` has no diff base, the DiffMap must be zero-cost. This means the `NoDiff` path needs to be a true passthrough: no SumTree traversal, no chunk annotation, no coordinate translation. The DiffMap snapshot in no-diff mode should just hold the MultiBufferSnapshot directly and delegate all operations.

### 3. Collaboration / remote buffers

Remote buffers don't have local git state. Their `BufferDiff` entities would have no diff base (degenerate case). If/when remote diff support is added, the diff computation would happen on the host and the `DiffBaseState` would be sent to the guest. The region tree serialization is straightforward (it's a sequence of `(kind, buffer_len, base_len)` tuples).

### 4. Three-way diff and conflict resolution

The current design supports two-way diffs (base → working copy). Three-way diffs (common ancestor, ours, theirs) are a natural extension: add more dimensions to the region summary and more sources for the DiffMap. This is left as future work but the architecture doesn't preclude it.

### 5. Syntax highlighting of base text

The initial implementation shows Removed text without syntax highlighting, using flat deletion styling. Full syntax highlighting of base text requires parsing the base `Rope` with the same language grammar. This could be done lazily when a diff view is opened. The `DiffBaseState` would gain an optional `SyntaxSnapshot` field.

### 6. Hit testing in split view gap regions

When the user clicks on a gap region in split view (empty space on one side corresponding to Added/Removed text on the other), the editor needs to decide what to do. Options: snap to the nearest real text position, ignore the click, or place the cursor at the boundary of the adjacent Unchanged region. This is a UX decision to be made during Phase 7.

### 7. Search in Removed text

Should editor search (Ctrl+F) find matches in Removed text? In inline collapsed mode, Removed text isn't in the display stream, so it can't be found. In inline expanded mode and split view, the text is in the stream but not editable. The search infrastructure should probably find matches in Removed text (useful for understanding what changed) but prevent replacement operations on those matches.

---

## Summary

The redesign rests on three pillars:

1. **`BufferDiff` as the composition unit.** The MultiBuffer is built on `Entity<BufferDiff>`, not `Entity<Buffer>`. A buffer with no diff is the degenerate case — a `BufferDiff` with `diff_base: None`. Different views of the same buffer can have independent diffs.

2. **Three-dimensional summaries with diff metadata threading.** Every `DiffRegion` carries buffer, base, and unified diff `TextSummary` values, plus `DiffStats`. These accumulate through the SumTree into the `Excerpt` and `ExcerptSummary`. The `DiffMap` pipeline stage introduces `diff_status` on every chunk, and every subsequent pipeline layer carries and consumes it — for editability constraints, fold boundaries, and rendering decisions. The `DiffTransform` tree and lockstep cursor are eliminated entirely.

3. **One diff-aware editor.** Split view is a rendering mode of the `EditorElement`, not two synchronized editors. One editor, one multibuffer, one display pipeline, one cursor. The renderer draws two panels from the same display snapshot, routing content to left/right based on `diff_status`. The entire `SplittableEditor` / `LhsEditor` / `Companion` apparatus — ~1500 lines of synchronization logic — is replaced by a rendering mode flag and panel-routing logic in `EditorElement`.
