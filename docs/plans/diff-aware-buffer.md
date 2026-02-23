# Diff-Aware Buffer: Planning Document

## Executive Summary

This document describes a redesign of how Zed represents diffs, moving diff awareness from a synchronization layer between two independent editors down into the `text::Buffer` itself. The core idea: a buffer optionally knows about a **diff base** — a prior version of its content — and can describe the relationship between its current text and that base as classified regions (unchanged, added, removed). This makes the buffer's text summary inherently two-dimensional: it carries both buffer-text dimensions and diff-base dimensions. These dual dimensions propagate up through excerpts and the multibuffer via the existing SumTree summary machinery, eliminating the need for a separate `DiffTransform` tree, the `Companion` synchronization system, spacer blocks, excerpt mirroring, and the inverted-diff concept entirely.

## Motivation

### The Current Architecture

Split diff (side-by-side diff view) is currently built as a **coordination problem**. Two entirely separate editors, each with their own multibuffer, display pipeline, scroll state, and selections, are kept in sync by an elaborate synchronization layer:

- **Two multibuffers**: The RHS editor has an editable `MultiBuffer` with excerpts from the working buffer. The LHS editor has a read-only `MultiBuffer` with excerpts from the base text buffer (owned by `BufferDiff`). The same `BufferDiff` entity is registered on the LHS in "inverted" mode.

- **`Companion` struct** (`crates/editor/src/display_map.rs` ~L232): Maintains bidirectional hash maps mapping excerpt IDs, buffer IDs, and row conversion functions between the two sides.

- **Spacer blocks** (`crates/editor/src/display_map/block_map.rs`, `spacer_blocks()` ~L1207): The block map inserts invisible spacer blocks on the shorter side of each hunk to keep rows aligned. This requires threading points through the full display pipeline (`InlayMap → FoldMap → TabMap → WrapMap`) on both sides to compute wrap counts, then computing deltas, then inserting spacers at the right positions.

- **Balancing blocks**: Custom blocks (diagnostics, etc.) on one side trigger mirrored "balancing" blocks on the other side.

- **`SharedScrollAnchor`** (`crates/editor/src/scroll.rs` ~L79): A shared scroll anchor entity is shared between both editors, with display-map-aware resolution.

- **Cursor syncing** (`crates/editor/src/split.rs` ~L732): `sync_cursor_to_other_side` translates cursor positions between the two editors.

- **Action interception**: Actions on the LHS (stage, restore, open file) are intercepted and translated to RHS coordinates before execution.

- **Excerpt mirroring** (`crates/editor/src/split.rs` ~L1871): `LhsEditor::sync_path_excerpts` mirrors excerpt additions, removals, and expansions from the RHS to the LHS.

- **`DiffTransform` tree** (`crates/multi_buffer/src/multi_buffer.rs` ~L638): A secondary `SumTree<DiffTransform>` in `MultiBufferSnapshot` interleaves `BufferContent` and `DeletedHunk` items to inject base text into the multibuffer's coordinate space. This tree must be walked in lockstep with the `excerpts` tree by `MultiBufferCursor`.

- **O(n) coordinate translation**: `BufferDiffSnapshot::patch_for_buffer_range` and `patch_for_base_text_range` (`crates/buffer_diff/src/buffer_diff.rs` ~L412-510) build `Patch<Point>` (a flat `Vec<Edit<T>>`) for coordinate translation, requiring linear traversal.

### Problems with the Current Architecture

1. **Every new editor feature must be taught about the companion system.** Diagnostics, inlay hints, folds, breakpoints — anything that adds blocks or affects layout must consider how it interacts with the two-editor coordination.

2. **Spacer block alignment is fragile.** It reverse-engineers wrap counts through the entire display pipeline of a companion editor. Changes to any display pipeline stage can break alignment.

3. **The architecture is asymmetric.** The RHS is a real editor operating on a real buffer. The LHS is a synthetic read-only mirror that exists only to show the "other side." This asymmetry creates special cases throughout the codebase.

4. **Coordinate translation is O(n)** through `Patch<Point>`, rather than O(log n) through a SumTree.

5. **The `DiffTransform` tree and excerpt tree must be walked in lockstep**, creating complex cursor logic (`MultiBufferCursor` ~200 lines) that is a frequent source of bugs.

6. **`sync_diff_transforms`** (`crates/multi_buffer/src/multi_buffer.rs` ~L3127) is approximately 300 lines of the most complex code in the multibuffer, rebuilding the `DiffTransform` tree in response to buffer edits and diff recomputations.

### The Proposed Architecture

The buffer itself becomes diff-aware. `text::Buffer` gains an optional **diff base**: a prior version of its content, plus a SumTree that classifies how the buffer's text relates to that base. The classification uses standard diff terminology:

- **Unchanged**: Text present in both the buffer and the diff base.
- **Added**: Text present in the buffer but absent from the diff base.
- **Removed**: Text absent from the buffer but present in the diff base.

The text summary that propagates up through excerpts and the multibuffer includes diff base dimensions alongside the existing buffer text dimensions. This makes the dual coordinate space (buffer text vs. diff base text) a native property of the SumTree, eliminating the need for a separate `DiffTransform` tree.

The buffer doesn't know about "left side" or "right side" — those are rendering concepts. It knows about "my text" and "the diff base." The editor decides how to present them: side by side, inline, unified, etc.

---

## Detailed Design

### Layer 1: `text::Buffer` and `text::BufferSnapshot`

#### New Data Structures

The following structures are added to the `text` crate:

```rust
/// The kind of a diff region, describing the relationship between
/// the buffer's text and the diff base text.
pub enum DiffRegionKind {
    /// Text is present in both the buffer and the diff base.
    /// The buffer text and base text are identical for this region.
    Unchanged,
    /// Text is present in the buffer but not in the diff base.
    /// This region has zero length in diff-base coordinates.
    Added,
    /// Text is present in the diff base but not in the buffer.
    /// This region has zero length in buffer coordinates.
    Removed,
}

/// A region in the diff, representing a contiguous span of text
/// with a uniform classification.
pub struct DiffRegion {
    pub kind: DiffRegionKind,
    /// Number of bytes this region spans in the buffer's text.
    /// Zero for Removed regions.
    pub buffer_len: usize,
    /// Number of bytes this region spans in the diff base text.
    /// Zero for Added regions.
    pub base_len: usize,
}

/// Summary of a DiffRegion, carrying TextSummary for both
/// the buffer text dimension and the diff base text dimension.
pub struct DiffRegionSummary {
    /// TextSummary accumulated over buffer text
    /// (Unchanged buffer_len + Added buffer_len).
    pub buffer: TextSummary,
    /// TextSummary accumulated over diff base text
    /// (Unchanged base_len + Removed base_len).
    pub base: TextSummary,
}

/// The diff base: a prior version of the buffer's content,
/// plus a SumTree describing how the buffer's current text
/// relates to it.
pub struct DiffBase {
    /// The text of the diff base (e.g., git HEAD content).
    pub text: Rope,
    /// Classified regions describing the relationship between
    /// the buffer's visible_text and this base text.
    pub regions: SumTree<DiffRegion>,
}

/// A text summary that includes both buffer text dimensions
/// and diff base dimensions.
pub struct DiffTextSummary {
    /// Summary of the buffer's text.
    pub buffer: TextSummary,
    /// Summary of the corresponding diff base text.
    /// When no diff base is set, this equals `buffer`.
    pub diff_base: TextSummary,
}
```

#### Changes to `BufferSnapshot`

`text::BufferSnapshot` (`crates/text/src/text.rs` ~L106) gains an optional diff base:

```rust
pub struct BufferSnapshot {
    visible_text: Rope,
    deleted_text: Rope,
    fragments: SumTree<Fragment>,
    insertions: SumTree<InsertionFragment>,
    insertion_slices: TreeSet<InsertionSlice>,
    undo_map: UndoMap,
    pub version: clock::Global,
    remote_id: BufferId,
    replica_id: ReplicaId,
    line_ending: LineEnding,

    // NEW
    diff_base: Option<DiffBase>,
}
```

This is **non-replicated, local-only state**, in the same category as the `History` (undo/redo stacks), `deferred_ops`, and `subscriptions` on `text::Buffer`. CRDT operations do not interact with it. Remote replicas do not send or receive diff base information.

#### SumTree Implementation for DiffRegion

`DiffRegion` implements `sum_tree::Item` with `DiffRegionSummary` as its summary type. The summary's context type is `()` (context-less), similar to `ExcerptSummary`.

For a `DiffRegion`:

- **Unchanged** (`buffer_len == base_len`): Both `buffer` and `base` get the `TextSummary` computed from the corresponding slice of `visible_text`. They are identical.
- **Added** (`base_len == 0`): `buffer` gets the `TextSummary` of the buffer text slice. `base` is `TextSummary::default()` (zero).
- **Removed** (`buffer_len == 0`): `base` gets the `TextSummary` of the diff base text slice. `buffer` is `TextSummary::default()` (zero).

Seeking by `buffer` dimension navigates buffer-text space (equivalent to today's `usize` offset). Seeking by `base` dimension navigates diff-base space. Both are O(log n) through the same tree.

#### New APIs on `BufferSnapshot`

```rust
impl BufferSnapshot {
    // === Existing APIs: UNCHANGED ===
    // len(), text(), text_for_range(), text_summary_for_range(),
    // Point, usize, Anchor — all work exactly as today.
    // They operate on the buffer's visible_text (the working copy).

    // === New: Diff Base Awareness ===

    /// Returns true if a diff base is currently set on this buffer.
    pub fn has_diff_base(&self) -> bool;

    /// Returns a reference to the diff base, if set.
    pub fn diff_base(&self) -> Option<&DiffBase>;

    /// Returns the text summary for a buffer range, including both
    /// buffer text dimensions and diff base dimensions.
    ///
    /// When no diff base is set, `diff_base == buffer` in the result.
    pub fn diff_text_summary_for_range(&self, range: Range<usize>) -> DiffTextSummary;

    /// Iterate diff regions intersecting a buffer offset range.
    /// Each region carries its kind (Unchanged/Added/Removed) and
    /// the text summaries for both dimensions.
    pub fn diff_regions(&self, range: Range<usize>) -> DiffRegions<'_>;

    /// Translate a buffer offset to the corresponding diff base offset.
    /// For positions within Unchanged regions, this gives the exact
    /// corresponding position. For positions within Added regions,
    /// this gives the base offset at the boundary of the enclosing hunk.
    pub fn to_diff_base_offset(&self, offset: usize) -> usize;

    /// Translate a diff base offset to the corresponding buffer offset.
    pub fn from_diff_base_offset(&self, base_offset: usize) -> usize;

    /// Point-based coordinate translation.
    pub fn to_diff_base_point(&self, point: Point) -> Point;
    pub fn from_diff_base_point(&self, base_point: Point) -> Point;

    /// Read diff base text for a range in base offset coordinates.
    pub fn diff_base_text_for_range(&self, base_range: Range<usize>) -> Chunks<'_>;

    /// Read diff base text as a string (convenience).
    pub fn diff_base_text(&self) -> Option<String>;
}
```

The `DiffRegions` iterator walks the `diff_base.regions` SumTree. Each yielded item provides:

- The `DiffRegionKind`
- The buffer offset range this region covers
- The diff base offset range this region covers
- Access to the text content (from `visible_text` for Unchanged/Added, from `diff_base.text` for Removed)
- The `TextSummary` for both dimensions

#### New APIs on `Buffer`

```rust
impl Buffer {
    /// Set the diff base for this buffer. This replaces any existing
    /// diff base. The `regions` tree describes how the buffer's current
    /// visible text relates to `base_text`.
    ///
    /// This is typically called when a background diff computation
    /// completes.
    pub fn set_diff_base(&mut self, base_text: Rope, regions: SumTree<DiffRegion>);

    /// Clear the diff base. The buffer reverts to having no diff
    /// awareness. All diff base dimensions become equal to buffer
    /// text dimensions.
    pub fn clear_diff_base(&mut self);
}
```

Both methods update `self.snapshot.diff_base` and notify subscribers. The notification mechanism should signal that diff base dimensions changed, allowing the multibuffer to update excerpt summaries.

#### Interaction with Buffer Edits

When the buffer is edited (via `apply_local_edit` or `apply_remote_edit`), the CRDT updates `visible_text` and the fragment tree. The diff base's `regions` tree uses buffer offsets to reference positions in the visible text. These positions shift when the buffer is edited.

**Approach**: After a buffer edit produces its `Patch<usize>` (the set of changed ranges), walk the `DiffRegion` tree for the affected range and update the `buffer_len` and `buffer` TextSummary of each touched region. The `base_len` and `base` summaries are unaffected (base text is immutable). For Added and Unchanged regions, recompute the `buffer` TextSummary from the current `visible_text`. For Removed regions, nothing changes (they have `buffer_len == 0`).

This is O(log n + k) where k is the number of touched regions — typically 1 or 2 for a single edit.

**Classification staleness**: After an edit, the _classification_ of regions might be stale. For example, editing text within an `Unchanged` region means that text is no longer truly unchanged — the buffer side differs from the base side. However, the tree remains structurally valid: the buffer-side summaries are accurate (they were just recomputed), and the base-side summaries are accurate (base text didn't change). The classification will be corrected when the next background diff computation completes and calls `set_diff_base` with fresh results.

This is the same behavior users see today: you type, and the gutter diff markers update a beat later when the background diff finishes.

#### The Degenerate Case (No Diff)

When `diff_base` is `None`:

- `has_diff_base()` returns `false`
- `diff_text_summary_for_range(range)` returns `DiffTextSummary { buffer: summary, diff_base: summary }` — both fields are the same `TextSummary` computed from `visible_text`. This is trivially computed (just copy the buffer summary).
- `diff_regions(range)` yields a single `Unchanged` region spanning the entire range
- `to_diff_base_offset(offset)` returns `offset`
- All coordinate translations are identity functions

No behavioral overhead. The only cost is the `Option<DiffBase>` field (one pointer).

### Layer 2: `DiffTextSummary` — The Dual-Dimension Summary

#### Why a New Summary Type

`rope::TextSummary` (`crates/rope/src/rope.rs` ~L1192) describes a single piece of text. It's computed from text content by scanning characters. It's used as the summary type for every chunk in every `Rope` in the system — syntax trees, undo history, buffer content.

Adding diff base fields to `rope::TextSummary` would bloat every chunk summary by ~16-24 bytes for a feature that most ropes don't use. `TextSummary` is currently ~48 bytes; a ~33-50% increase is unacceptable at the rope level.

`DiffTextSummary` lives in the `text` crate, at the buffer level. It wraps two `TextSummary` values — one for the buffer text, one for the diff base — and is what the excerpt/multibuffer layer uses. The `rope::TextSummary` type is unchanged.

#### How It Flows

1. **Buffer**: `diff_text_summary_for_range(range)` walks the diff region tree and accumulates both `buffer` and `base` TextSummary values for the given buffer range.

2. **Excerpt**: Stores both `text_summary: TextSummary` (the buffer text, same as today) and `diff_base_summary: TextSummary` (the diff base, new). Both are populated from `DiffTextSummary` when the excerpt is constructed or updated.

3. **ExcerptSummary**: Accumulates both `text: MBTextSummary` and `diff_base: MBTextSummary`. The SumTree addition logic handles them independently.

4. **MultiBuffer**: The excerpt SumTree carries both dimensions at every node. Seeking by `text` dimension navigates buffer-text space. Seeking by `diff_base` dimension navigates diff-base space. Same tree, different dimension.

### Layer 3: `Excerpt` and `MultiBuffer` Changes

#### Excerpt Changes

```rust
struct Excerpt {
    id: ExcerptId,
    locator: Locator,
    buffer_id: BufferId,
    buffer: Arc<BufferSnapshot>,
    range: ExcerptRange<text::Anchor>,
    max_buffer_row: BufferRow,
    text_summary: TextSummary,           // buffer text, same as today
    diff_base_summary: TextSummary,      // NEW: diff base text for this range
    has_trailing_newline: bool,
}
```

When an excerpt is constructed:

```rust
let buffer_range = range.context.to_offset(&buffer);
let diff_summary = buffer.diff_text_summary_for_range(buffer_range);
Excerpt {
    text_summary: diff_summary.buffer,
    diff_base_summary: diff_summary.diff_base,
    ...
}
```

#### ExcerptSummary Changes

```rust
pub struct ExcerptSummary {
    excerpt_id: ExcerptId,
    excerpt_locator: Locator,
    widest_line_number: u32,
    text: MBTextSummary,           // buffer text dimension, same as today
    diff_base: MBTextSummary,      // NEW: diff base dimension
}
```

`diff_base` accumulates through the SumTree exactly like `text` does. The `add_summary` implementation adds both independently:

```rust
fn add_summary(&mut self, summary: &Self) {
    // existing
    self.text += summary.text;
    // new
    self.diff_base += summary.diff_base;
    // ... other fields
}
```

#### MultiBufferSnapshot Changes

Fields **removed**:

- `diffs: TreeMap<BufferId, DiffStateSnapshot>` — diff state is in the buffer now
- `diff_transforms: SumTree<DiffTransform>` — absorbed into excerpt summaries
- `has_inverted_diff: bool` — no inverted concept
- `all_diff_hunks_expanded: bool` — expand/collapse moves to the editor/rendering layer
- `show_deleted_hunks: bool` — rendering concern
- `use_extended_diff_range: bool` — rendering concern

```rust
pub struct MultiBufferSnapshot {
    excerpts: SumTree<Excerpt>,
    // REMOVED: diffs, diff_transforms, has_inverted_diff,
    //          all_diff_hunks_expanded, show_deleted_hunks,
    //          use_extended_diff_range
    non_text_state_update_count: usize,
    edit_count: usize,
    is_dirty: bool,
    has_deleted_file: bool,
    has_conflict: bool,
    singleton: bool,
    excerpt_ids: SumTree<ExcerptIdMapping>,
    replaced_excerpts: TreeMap<ExcerptId, ExcerptId>,
    trailing_excerpt_update_count: usize,
    show_headers: bool,
}
```

#### MultiBufferCursor Simplification

Today (`crates/multi_buffer/src/multi_buffer.rs` ~L1035):

```rust
struct MultiBufferCursor<'a, MBD, BD> {
    excerpts: Cursor<'a, 'static, Excerpt, ExcerptDimension<MBD>>,
    diff_transforms: Cursor<'a, 'static, DiffTransform, DiffTransforms<MBD>>,
    diffs: &'a TreeMap<BufferId, DiffStateSnapshot>,
    cached_region: OnceCell<Option<MultiBufferRegion<'a, MBD, BD>>>,
}
```

Becomes:

```rust
struct MultiBufferCursor<'a, MBD, BD> {
    excerpts: Cursor<'a, 'static, Excerpt, ExcerptDimension<MBD>>,
    cached_region: OnceCell<Option<MultiBufferRegion<'a, MBD, BD>>>,
}
```

The lockstep walking logic (`next()`, `prev()`, `seek()`, `seek_forward()` — approximately 200 lines) is replaced by single-tree cursor operations. The `MultiBufferRegion` no longer needs `is_main_buffer` or `diff_hunk_status` fields — diff region information comes from the buffer snapshot.

#### Seeking by Diff Base Dimension

A new `DiffBaseOffset`, `DiffBasePoint`, or similar dimension type can be added that extracts the `diff_base` field from `ExcerptSummary` during SumTree traversal. This enables:

```rust
// Seek to row 42 in diff-base coordinates
let mut cursor = snapshot.excerpts.cursor::<DiffBasePoint>(());
cursor.seek(&DiffBasePoint(Point::new(42, 0)), Bias::Right);
```

This is the same pattern as existing dimension types (`MultiBufferOffset`, `Point`, etc.) — just targeting the new `diff_base` field.

#### MultiBufferChunks Changes

Today, `MultiBufferChunks` (`crates/multi_buffer/src/multi_buffer.rs` ~L974) walks the `DiffTransform` tree to decide whether to yield buffer text or base text:

```rust
match diff_transform {
    DiffTransform::BufferContent { .. } => { /* read from buffer */ }
    DiffTransform::DeletedHunk { .. } => { /* read from base text */ }
}
```

In the new model, `MultiBufferChunks` iterates excerpts and delegates to the buffer snapshot. Two iteration modes are needed:

1. **Buffer text only** (today's default, for the right side of split diff and for non-diff views): Iterate buffer text from `visible_text`. Removed regions are skipped (they have zero buffer length). This is equivalent to today's behavior when no diff hunks are expanded.

2. **Diff-aware iteration** (for renderers that need to show both sides): Iterate the buffer's diff regions. For each region, yield chunks tagged with their `DiffRegionKind`. For Unchanged and Added regions, chunks come from `visible_text`. For Removed regions, chunks come from `diff_base.text`.

The caller (the editor/renderer) chooses which mode to use.

#### What Happens to Hunk Expand/Collapse

Today, expanding a diff hunk causes `sync_diff_transforms` to insert a `DiffTransform::DeletedHunk` item into the `DiffTransform` tree. Collapsing removes it. This controls whether base text is visible in the multibuffer's output coordinate space.

In the new model, the diff content is always in the buffer. "Expanding" and "collapsing" become rendering decisions, not multibuffer data structure changes. This could be implemented as:

- A set of "expanded hunk ranges" maintained by the editor
- Alternatively, hunk collapsing could potentially be implemented via the fold mechanism — a collapsed hunk is a fold over the removed region

The exact mechanism for expand/collapse is a design decision to be refined during implementation. The key point: the data is always there; the question is whether the renderer shows it.

#### What Happens to `diff_hunks_in_range`

Today, `MultiBufferSnapshot::diff_hunks_in_range` queries the `DiffStateSnapshot` (derived from `BufferDiff`) to find hunks intersecting a multibuffer range. It handles the `is_inverted` case for the LHS editor.

In the new model, this method queries the buffer snapshots' diff region trees directly. Since the diff regions are in the buffer, and the excerpts hold buffer snapshots, the method walks the excerpt tree and for each excerpt, asks the buffer snapshot for diff regions intersecting the relevant buffer range. No separate `DiffStateSnapshot`, no inverted diff handling.

The returned `MultiBufferDiffHunk` type can remain largely the same — it describes a hunk with its multibuffer row range, buffer range, diff base byte range, and status.

### Layer 4: `BufferDiff` — The Orchestrator

#### Changed Role

`BufferDiff` (`crates/buffer_diff/src/buffer_diff.rs`) currently:

1. Owns the base text as an `Entity<language::Buffer>`
2. Runs the diff algorithm (via libgit2's patience diff) on a background thread
3. Stores the result as a `SumTree<InternalDiffHunk>`
4. Provides coordinate translation via `Patch<Point>`
5. Handles staging/unstaging via `secondary_diff`
6. Notifies the multibuffer of diff changes

In the new model:

1. **Base text storage moves to `text::Buffer`** — the buffer holds its own `DiffBase` with a `Rope`
2. **Diff algorithm**: Still runs on a background thread, still uses libgit2. But the output is a `SumTree<DiffRegion>` instead of `SumTree<InternalDiffHunk>`
3. **Writing results**: Instead of storing hunks internally, `BufferDiff` calls `buffer.set_diff_base(base_text, regions)` on the main thread
4. **Coordinate translation**: Eliminated from `BufferDiff` — the buffer's diff region tree provides O(log n) translation natively
5. **Staging/unstaging**: Still requires `BufferDiff` to reconstruct index text. The hunk information needed for staging can be derived from the buffer's diff regions. The `secondary_diff` concept may need to be adapted — see "Staging and Secondary Diffs" below
6. **Notifications**: The buffer notifies its subscribers when `set_diff_base` is called. The multibuffer reacts by updating excerpt summaries

#### Diff Computation Output

Today, `compute_hunks()` (`crates/buffer_diff/src/buffer_diff.rs` ~L951) produces a `SumTree<InternalDiffHunk>`. In the new model, it produces a `(Rope, SumTree<DiffRegion>)` — the base text rope and the classified regions.

The diff algorithm (libgit2's `GitPatch::from_buffers`) produces hunks. Each hunk has:

- Added lines (present in buffer, absent from base)
- Deleted lines (present in base, absent from buffer)
- Context lines (unchanged)

The conversion to `DiffRegion` is straightforward:

- Context between hunks → `Unchanged` region
- Deleted lines in a hunk → `Removed` region
- Added lines in a hunk → `Added` region
- Context within hunks (between added/deleted sections) → `Unchanged` region

The `DiffRegion` items are ordered by their position in the interleaved (buffer + base) stream. The SumTree's `DiffRegionSummary` accumulates `buffer` and `base` TextSummary values, enabling O(log n) seeking in either coordinate space.

#### Word Diffs

Today, `InternalDiffHunk` stores `base_word_diffs: Vec<Range<usize>>` and `buffer_word_diffs: Vec<Range<Anchor>>` for highlighting individual word changes within a hunk.

Word diffs could be stored as additional metadata on `DiffRegion` items, or as a separate structure indexed by buffer range. The exact placement is a design decision for implementation, but the word diff data still needs to be computed and stored somewhere accessible to the renderer.

#### Staging and Secondary Diffs

Today, `BufferDiff` supports a `secondary_diff: Option<Entity<BufferDiff>>` for staging (working copy vs. index vs. HEAD). The uncommitted diff (HEAD → working) has a secondary unstaged diff (index → working). Staging a hunk modifies the index text.

In the new model, the buffer has one diff base at a time. Multiple diff relationships (HEAD, index) could be handled by:

1. **Multiple diff bases on the buffer**: The buffer could support more than one named diff base (e.g., `"head"`, `"index"`). Each would have its own `DiffBase` with its own regions tree. The text summary would need additional dimensions, or the rendering layer would select which diff base to display.

2. **Keeping `BufferDiff` as the staging orchestrator**: `BufferDiff` continues to manage the relationship between HEAD, index, and working copy for staging purposes. It uses the buffer's diff region tree for display but maintains its own internal state for staging operations. The staging operation itself (reconstructing index text from hunks) doesn't change fundamentally.

3. **One diff base at a time, with staging metadata alongside**: The buffer has one diff base (the one currently being displayed, typically HEAD). Staging information (which hunks are staged) is metadata on the diff regions or alongside them.

The exact approach for staging needs to be refined during implementation. The key constraint: staging must continue to work correctly, and the hunk status indicators (staged/unstaged/partially staged) must continue to display correctly in the gutter.

### Layer 5: The Editor — Rendering Decisions

#### The Editor's Role

The buffer provides content and classifications. The editor decides how to present them. The buffer doesn't know about "left side" or "right side" — those are rendering concepts.

#### Split View Rendering

Today: Two `Editor` entities, each with its own `MultiBuffer` and display pipeline, bridged by `Companion`.

New model: One `MultiBuffer`, one `MultiBufferSnapshot`. The `SplitEditorView` renders two panels from the same snapshot. Each panel iterates diff regions and makes rendering decisions:

**Buffer side (right panel):**

- For `Unchanged` regions: render the text normally
- For `Added` regions: render the text with insertion styling
- For `Removed` regions: render vertical gap (blank lines) equal to the region's `base` TextSummary line count, for alignment with the base side

**Base side (left panel):**

- For `Unchanged` regions: render the text normally (read from buffer's `visible_text`)
- For `Removed` regions: render the text with deletion styling (read from buffer's `diff_base.text`)
- For `Added` regions: render vertical gap equal to the region's `buffer` TextSummary line count

**Inline diff (unified view):**

- Iterate all regions
- `Removed` text: render with deletion markers (red/strikethrough)
- `Added` text: render with insertion markers (green/highlight)
- `Unchanged` text: render normally

#### Display Pipeline Fork

The display pipeline `MultiBuffer → InlayMap → FoldMap → TabMap → WrapMap → BlockMap` needs to fork for split view. Each side gets its own pipeline, because:

- The two sides have different text content (added text appears only on the right; removed text appears only on the left)
- Soft wrapping differs between sides (different column widths)
- Inlay hints, folds, etc. may differ

Both pipelines read from the same `MultiBufferSnapshot`. The fork happens at the point where the multibuffer's chunks are iterated — one side iterates buffer text (Unchanged + Added), the other iterates base text (Unchanged + Removed).

The `SplitSide` enum (`crates/editor/src/element.rs` ~L202) remains — it's a rendering concept, not a buffer concept.

#### Alignment

Alignment between the two sides comes from the diff region tree. A `Removed` region's `base` TextSummary tells the right-side renderer how many blank lines to insert. An `Added` region's `buffer` TextSummary tells the left-side renderer how many blank lines to insert.

Soft wrapping may cause different line counts per side for corresponding regions. The renderer compares wrapped line counts for corresponding regions and adds padding as needed. This is local to each region, computed from information already in the tree. No companion maps, no cross-editor coordinate threading.

This replaces the current `spacer_blocks` mechanism (~100 lines of intricate coordinate mapping in `crates/editor/src/display_map/block_map.rs` ~L1207).

#### Cursor and Selection Model

In the new model, cursor position is in buffer coordinates (the primary coordinate space). When the user clicks on the right panel, the cursor is at a buffer offset. When the user clicks on the left panel, the click position is in diff-base coordinates; the editor translates it to the nearest buffer offset via `from_diff_base_offset` (O(log n)).

There's no "sync cursor to other side" — the cursor exists in one coordinate space (buffer offsets), and both sides know how to render a cursor indicator at the corresponding position by translating through the diff region tree.

Selections work the same way: they're in buffer coordinates, rendered in both panels with appropriate coordinate translation.

### What Gets Eliminated

| Component                                             | Location                    | Lines (approx) | Status                   |
| ----------------------------------------------------- | --------------------------- | -------------- | ------------------------ |
| `SumTree<DiffTransform>`                              | `multi_buffer.rs` ~L610     | —              | **Removed**              |
| `DiffTransform` enum                                  | `multi_buffer.rs` ~L638     | 20             | **Removed**              |
| `DiffTransformSummary`                                | `multi_buffer.rs` ~L859     | 10             | **Removed**              |
| `DiffTransformHunkInfo`                               | `multi_buffer.rs` ~L665     | 20             | **Removed**              |
| `sync_diff_transforms`                                | `multi_buffer.rs` ~L3127    | 300+           | **Removed**              |
| `recompute_diff_transforms_for_edit`                  | `multi_buffer.rs` ~L3270    | 150+           | **Removed**              |
| `push_buffer_content_transform`                       | `multi_buffer.rs`           | 50+            | **Removed**              |
| `push_deleted_hunk_transform`                         | `multi_buffer.rs`           | 50+            | **Removed**              |
| `MultiBufferCursor` lockstep logic                    | `multi_buffer.rs` ~L6975    | 200+           | **Simplified**           |
| `diffs: TreeMap<BufferId, DiffStateSnapshot>`         | `multi_buffer.rs` ~L609     | —              | **Removed**              |
| `DiffState` / `DiffStateSnapshot`                     | `multi_buffer.rs` ~L522/538 | 15             | **Removed**              |
| `add_inverted_diff`                                   | `multi_buffer.rs` ~L2624    | 10             | **Removed**              |
| `inverted_buffer_diff_changed`                        | `multi_buffer.rs` ~L2416    | 40             | **Removed**              |
| `has_inverted_diff` flag                              | `multi_buffer.rs`           | —              | **Removed**              |
| `Companion` struct                                    | `display_map.rs` ~L232      | 100+           | **Removed**              |
| `CompanionExcerptPatch`                               | `display_map.rs` ~L183      | 10             | **Removed**              |
| Spacer blocks (`spacer_blocks()`)                     | `block_map.rs` ~L1207       | 100+           | **Removed**              |
| Balancing blocks                                      | `block_map.rs`              | 50+            | **Removed**              |
| `LhsEditor` struct                                    | `split.rs` ~L338            | 5              | **Removed**              |
| `sync_path_excerpts`                                  | `split.rs` ~L1871           | 40             | **Removed**              |
| `sync_cursor_to_other_side`                           | `split.rs` ~L732            | 30             | **Removed**              |
| `convert_lhs_rows_to_rhs` / `convert_rhs_rows_to_lhs` | `split.rs` ~L41-57          | 100+           | **Removed**              |
| `SharedScrollAnchor` complexity                       | `scroll.rs` ~L79            | —              | **Simplified**           |
| `patch_for_buffer_range` O(n)                         | `buffer_diff.rs` ~L412      | 40             | **Replaced** by O(log n) |
| `patch_for_base_text_range` O(n)                      | `buffer_diff.rs` ~L460      | 40             | **Replaced** by O(log n) |
| `buffer_point_to_base_text_range`                     | `buffer_diff.rs`            | 10             | **Replaced**             |
| `base_text_point_to_buffer_point`                     | `buffer_diff.rs`            | 10             | **Replaced**             |
| `SumTree<InternalDiffHunk>` as diff representation    | `buffer_diff.rs`            | —              | **Replaced**             |
| Base text `Entity<language::Buffer>` in `BufferDiff`  | `buffer_diff.rs`            | —              | **Replaced** by `Rope`   |
| `MultiBufferExcerpt.diff_transforms` cursor           | `multi_buffer.rs` ~L758     | —              | **Removed**              |

### What Gets Added

| Component                                             | Location                  | Purpose                                        |
| ----------------------------------------------------- | ------------------------- | ---------------------------------------------- |
| `DiffBase` struct                                     | `text` crate              | Holds base text `Rope` + diff region `SumTree` |
| `DiffRegion` / `DiffRegionKind` / `DiffRegionSummary` | `text` crate              | Diff region tree items and summary             |
| `DiffTextSummary`                                     | `text` crate              | Dual-dimension text summary                    |
| `diff_base` field on `BufferSnapshot`                 | `text` crate              | Optional diff base on snapshot                 |
| `set_diff_base` / `clear_diff_base`                   | `text::Buffer`            | Writing/swapping diff content                  |
| `diff_text_summary_for_range`                         | `text::BufferSnapshot`    | Query both dimensions                          |
| `diff_regions()` iterator                             | `text::BufferSnapshot`    | Iterate classified content                     |
| `to_diff_base_offset` / `from_diff_base_offset`       | `text::BufferSnapshot`    | O(log n) coordinate translation                |
| `diff_base_summary` field on `Excerpt`                | `multi_buffer` crate      | Diff base dimension in excerpts                |
| `diff_base` field on `ExcerptSummary`                 | `multi_buffer` crate      | Accumulated diff base dimension                |
| `DiffBasePoint` / `DiffBaseOffset` dimension types    | `multi_buffer` crate      | Seeking by diff base dimension                 |
| Diff-aware chunks iteration                           | `multi_buffer` + `editor` | Content iteration with classification          |
| Split renderer using diff regions                     | `editor` crate            | Replaces two-editor split                      |

---

## Migration Strategy

The migration is designed to be incremental. The new and old systems coexist during the transition. Each phase is independently testable and deployable.

### Phase 1: Add `DiffBase` to `text::Buffer`

**Goal**: Establish the diff base concept at the text buffer level. Nothing downstream changes yet.

**Work**:

1. Define `DiffRegion`, `DiffRegionKind`, `DiffRegionSummary`, `DiffBase`, `DiffTextSummary` in the `text` crate
2. Implement `sum_tree::Item` for `DiffRegion` with `DiffRegionSummary`
3. Add `diff_base: Option<DiffBase>` to `BufferSnapshot`
4. Implement `set_diff_base()` and `clear_diff_base()` on `Buffer`
5. Implement `diff_text_summary_for_range()`, `diff_regions()`, `to_diff_base_offset()`, `from_diff_base_offset()`, and the Point equivalents on `BufferSnapshot`
6. Implement diff region summary update on buffer edit (recompute `buffer` TextSummary for touched regions)
7. Write comprehensive tests:
   - Setting and clearing diff bases
   - Coordinate translation roundtrips
   - Region iteration correctness
   - Summary accuracy after edits
   - Degenerate case (no diff base)
   - Edge cases: empty buffer, empty base, entirely added file, entirely deleted file

**Validation**: Unit tests in the `text` crate. No integration changes needed.

### Phase 2: Wire `BufferDiff` to Write Diff Regions

**Goal**: Make `BufferDiff` produce `DiffRegion` trees and write them to the buffer via `set_diff_base`.

**Work**:

1. Add a function that converts diff computation results (from libgit2's `GitPatch`) into `(Rope, SumTree<DiffRegion>)`
2. After diff computation completes, call `buffer.set_diff_base(base_text, regions)` in addition to (not instead of) the existing hunk tree update
3. Verify that the buffer's diff regions match the existing hunk list

**Validation**: Integration tests that compare `buffer.diff_regions()` output against `BufferDiff.hunks_intersecting_range()` for the same buffer and base text. These should agree on the classification and ranges.

### Phase 3: Add `diff_base_summary` to Excerpt and ExcerptSummary

**Goal**: The multibuffer's excerpt tree carries diff base dimensions.

**Work**:

1. Add `diff_base_summary: TextSummary` to `Excerpt`
2. Add `diff_base: MBTextSummary` to `ExcerptSummary`
3. When excerpts are created/updated, populate `diff_base_summary` from the buffer snapshot's `diff_text_summary_for_range`
4. Implement `DiffBaseOffset` and `DiffBasePoint` dimension types for seeking
5. Add `diff_base_text_summary_for_range` to `MultiBufferSnapshot`
6. Write tests verifying that seeking by diff base dimension gives correct results

**Validation**: The existing `DiffTransform` tree is still present and still used. The new dimension is purely additive. Tests compare results from the new dimension-based seeking against the existing `DiffTransform`-based computation.

### Phase 4: Build Diff-Region-Aware Chunk Iteration

**Goal**: The multibuffer can yield chunks classified by diff region kind.

**Work**:

1. Add a `DiffChunk` type that wraps a text chunk with its `DiffRegionKind`
2. Add a `diff_chunks()` method to `MultiBufferSnapshot` (or adapt `MultiBufferChunks`) that yields `DiffChunk` items
3. For each excerpt, iterate the buffer's diff regions and yield chunks from `visible_text` (for Unchanged/Added) or `diff_base.text` (for Removed)
4. Write tests verifying that the combined text output matches the existing `MultiBufferChunks` output when diff hunks are expanded

**Validation**: Compare text output against the existing system.

### Phase 5: Build the New Split Renderer

**Goal**: One multibuffer, two display pipelines, rendering via diff regions.

**Work**:

1. Build a left-side display pipeline that reads from the same `MultiBufferSnapshot`, iterating Unchanged + Removed regions
2. Build alignment logic that uses diff region summaries to compute gap sizes
3. Build cursor rendering that translates buffer coordinates to diff-base coordinates for the left panel
4. Wire this into `SplitEditorView` alongside (or behind a feature flag vs.) the existing two-editor approach
5. Extensive visual testing: compare rendering output between old and new systems

**Validation**: Side-by-side comparison with the existing split diff. Feature flag for gradual rollout.

### Phase 6: Remove the Old Infrastructure

**Goal**: Delete the old system once the new one is validated.

**Work**:

1. Remove `SumTree<DiffTransform>` and all related types from `MultiBufferSnapshot`
2. Remove `sync_diff_transforms` and `recompute_diff_transforms_for_edit`
3. Remove `DiffState`, `DiffStateSnapshot`, `diffs` map from `MultiBufferSnapshot`
4. Remove `add_inverted_diff`, `inverted_buffer_diff_changed`, `has_inverted_diff`
5. Remove `Companion` struct and all companion-related code from `DisplayMap`
6. Remove `CompanionExcerptPatch` and row conversion functions
7. Remove spacer blocks and balancing blocks from `BlockMap`
8. Remove `LhsEditor`, `sync_path_excerpts`, `sync_cursor_to_other_side` from split.rs
9. Simplify `SharedScrollAnchor`
10. Remove `patch_for_buffer_range`, `patch_for_base_text_range` from `BufferDiffSnapshot`
11. Remove base text `Entity<language::Buffer>` from `BufferDiff` (replaced by `Rope` in buffer's `DiffBase`)
12. Remove `MultiBufferExcerpt.diff_transforms` cursor
13. Update all tests

**Validation**: Full test suite pass. No regressions.

---

## Open Questions and Risks

### Staging and Secondary Diffs

The current staging system relies on `BufferDiff` having a `secondary_diff` that represents the index (staged) state. Staging a hunk reconstructs the index text by splicing buffer text and base text based on hunk boundaries. This needs to continue working.

**Risk**: The staging operation currently works with `InternalDiffHunk` ranges. In the new model, it would work with `DiffRegion` ranges from the buffer. The conversion should be straightforward, but needs careful implementation.

**Decision needed**: Should the buffer support multiple named diff bases (for HEAD and index simultaneously), or should staging remain a `BufferDiff` concern that uses a single diff base on the buffer?

### Expand/Collapse Mechanism

Today, expanding a hunk inserts a `DeletedHunk` into the `DiffTransform` tree, making the base text visible in the multibuffer's output coordinate space. Collapsing removes it.

**Decision needed**: How does expand/collapse work in the new model? Options:

1. A set of "expanded ranges" maintained by the editor, consulted during rendering
2. Folding — collapsed hunks are folds over the removed regions
3. Always expanded (for split view) with collapse only available in inline view

### Performance of `set_diff_base`

Calling `set_diff_base` rebuilds the entire diff region tree. For large files with many hunks, this tree could have hundreds or thousands of items.

**Risk**: Low. SumTree construction from a sorted list of items is O(n), and the diff computation itself (running libgit2) is much more expensive than tree construction. The tree construction happens on the main thread but should be fast (microseconds for typical file sizes).

### Performance of Edit-Time Region Summary Updates

When the buffer is edited, we walk the diff region tree to update summaries for affected regions.

**Risk**: Low. Typical edits touch 1-2 regions. The update is O(log n + k) where k is small. But we need to be careful about the implementation — SumTree doesn't support in-place mutation, so we'd need to rebuild the affected portion of the tree (similar to how `sync_diff_transforms` works today).

### Anchor Validity in Diff Regions

Diff regions for Unchanged and Added content reference positions in the buffer's `visible_text` via buffer offsets. When the buffer is edited, these offsets shift.

**Risk**: Medium. The current `InternalDiffHunk` uses `text::Anchor` for its `buffer_range`, which automatically adjusts with edits. `DiffRegion` uses `buffer_len: usize` (a length, not a position), which doesn't need to adjust — but the tree's position-based seeking depends on accurate cumulative lengths. The edit-time update step (recomputing `buffer` summaries for touched regions) handles this.

**Alternative**: Store `DiffRegion` boundaries as `text::Anchor` pairs instead of lengths. This would make them automatically track edits, at the cost of needing anchor resolution during tree construction. Worth exploring during implementation.

### Three-Way Diff and Conflict Resolution

The architecture naturally extends to three-way diffs by adding a third dimension to the summary (e.g., for merge base, ours, theirs). This isn't in scope for the initial implementation but the design should not preclude it.

**Note**: The `DiffTextSummary` structure can be extended with additional fields for additional diff bases without changing the fundamental architecture.

### Collaboration / Remote Buffers

The diff base is local-only state. In a collaborative editing session, each participant computes their own diff independently. This is correct — diff base (git HEAD, index, etc.) is a local repository concern, not a document-level concern.

**Risk**: None. The CRDT is unaffected.

---

## Key Files Reference

Files that will be **modified** in this redesign:

| File                                         | What Changes                                                                                                                                                                                       |
| -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/text/src/text.rs`                    | Add `DiffBase`, `DiffRegion`, `DiffTextSummary` types. Add `diff_base` to `BufferSnapshot`. Add `set_diff_base`/`clear_diff_base` to `Buffer`. Add diff query APIs.                                |
| `crates/multi_buffer/src/multi_buffer.rs`    | Add `diff_base_summary` to `Excerpt`/`ExcerptSummary`. Remove `DiffTransform` tree, `diffs` map, `sync_diff_transforms`, lockstep cursor logic. Simplify `MultiBufferCursor`, `MultiBufferChunks`. |
| `crates/buffer_diff/src/buffer_diff.rs`      | Change diff output from `SumTree<InternalDiffHunk>` to `SumTree<DiffRegion>`. Call `buffer.set_diff_base()`. Simplify or remove coordinate translation methods.                                    |
| `crates/editor/src/split.rs`                 | Remove `LhsEditor`, `Companion` wiring, excerpt mirroring, cursor syncing. Simplify `SplittableEditor`.                                                                                            |
| `crates/editor/src/split_editor_view.rs`     | Render two panels from one multibuffer using diff regions.                                                                                                                                         |
| `crates/editor/src/display_map.rs`           | Remove `Companion` struct. Remove companion-related methods.                                                                                                                                       |
| `crates/editor/src/display_map/block_map.rs` | Remove `spacer_blocks()`, balancing blocks.                                                                                                                                                        |
| `crates/editor/src/scroll.rs`                | Simplify `SharedScrollAnchor`.                                                                                                                                                                     |

Files that will be **added**:

| File                                               | Purpose                                                                                                    |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `crates/text/src/diff.rs` (or inline in `text.rs`) | `DiffBase`, `DiffRegion`, `DiffRegionKind`, `DiffRegionSummary`, `DiffTextSummary`, `DiffRegions` iterator |

```

Now, here's the handoff prompt:

---

## Handoff Prompt

You can use the following prompt to continue refining this plan with the next agent:

---

**Context: Diff-Aware Buffer Redesign — Continuing Plan Refinement**

You are continuing a design conversation about fundamentally rethinking how split diff (side-by-side diff view) works in Zed. A detailed planning document has been written at `zed/docs/plans/diff-aware-buffer.md`. Read this document first to understand the full design.

**Stay in planning mode** — help me think through the design, identify problems, refine the approach, and connect ideas to concrete code. Do not implement anything yet.

### Summary of the Design

The core idea: push diff awareness into `text::Buffer` itself. A buffer optionally holds a **diff base** (a prior version of its content) plus a `SumTree<DiffRegion>` that classifies how its current text relates to that base using standard diff terminology: **Unchanged**, **Added**, **Removed**.

The key innovation: the text summary that flows up through excerpts and the multibuffer includes **diff base dimensions** alongside the existing buffer text dimensions. `DiffTextSummary` carries both `buffer: TextSummary` and `diff_base: TextSummary`. This makes the dual coordinate space (buffer text vs. diff base text) a native property of the SumTree, enabling O(log n) seeking in either space through the same tree.

This eliminates the `DiffTransform` tree, the `Companion` synchronization system, spacer blocks, excerpt mirroring, the inverted-diff concept, and the `LhsEditor` entirely. The buffer doesn't know about "left" or "right" — those are rendering concerns. It knows about "my text" and "the diff base."

### Key Design Decisions Already Made

1. **The diff base lives in `text::BufferSnapshot`** as an `Option<DiffBase>`, containing a `Rope` (base text) and `SumTree<DiffRegion>` (classified regions). It is non-replicated, local-only state.

2. **`rope::TextSummary` is unchanged.** The dual-dimension summary (`DiffTextSummary`) lives at the buffer level, not the rope level, to avoid bloating every chunk summary in every rope.

3. **The buffer's primary coordinate space (offset, Point) is unchanged.** It always refers to the working copy text. The diff base is a secondary, queryable space — not a peer coordinate space.

4. **The buffer uses Unchanged/Added/Removed terminology**, not Left/Right. Left/Right are rendering concepts belonging to the editor.

5. **Writing a diff base is an async-friendly operation.** `BufferDiff` computes the diff on a background thread and calls `buffer.set_diff_base(base_text, regions)` when done. Swapping diff bases is just calling `set_diff_base` with different content. Clearing the diff is `clear_diff_base()`.

6. **Edits between diff recomputations** update the `DiffRegion` summaries (recompute `buffer` TextSummary for touched regions) but leave the classification potentially stale. The next `set_diff_base` corrects everything.

7. **The `Excerpt` gains a `diff_base_summary: TextSummary` field**, and `ExcerptSummary` gains a `diff_base: MBTextSummary` field. These propagate through the SumTree, eliminating the need for a separate `DiffTransform` tree.

### Areas That Need Further Refinement

1. **Staging and secondary diffs**: The current system uses `BufferDiff.secondary_diff` (index diff) alongside the primary (HEAD diff) for staging. How does this work when the diff base is in the buffer? Should the buffer support multiple named diff bases? Or should staging remain a `BufferDiff` concern? Read the staging code in `crates/buffer_diff/src/buffer_diff.rs` (`stage_or_unstage_hunks_impl` ~L544) to understand the current approach.

2. **Expand/collapse mechanism**: Today, expanding a hunk inserts a `DeletedHunk` into the `DiffTransform` tree. In the new model, the diff content is always in the buffer. How should expand/collapse work? Should it use the fold mechanism? A set of expanded ranges? Always expanded for split view?

3. **The display pipeline fork**: Both sides of a split view need their own `InlayMap → FoldMap → TabMap → WrapMap → BlockMap` pipeline, but reading from the same `MultiBufferSnapshot`. How exactly does the fork happen? What does the left-side pipeline's input look like? How does it get chunks from the buffer's diff base text?

4. **Anchor-based vs. length-based DiffRegion boundaries**: The planning document uses `buffer_len: usize` and `base_len: usize` in `DiffRegion`. Should these instead be `text::Anchor` pairs that automatically track edits? What are the tradeoffs?

5. **Word diffs**: Where do word-level diff ranges live in the new model? Today they're on `InternalDiffHunk`. They need to be accessible to the renderer.

6. **`MultiBufferDiffHunk`**: This type is used extensively by the editor for gutter rendering, hunk navigation, staging UI, etc. How does it get populated in the new model? It currently comes from `diff_hunks_in_range` which queries `DiffStateSnapshot`.

7. **How does `diff_text_summary_for_range` actually work?** Trace through the implementation: it needs to walk the `DiffRegion` tree, handling partial overlaps at the start and end of the range. What does this look like concretely?

8. **How does the edit-time region summary update work?** When the buffer is edited, the `DiffRegion` tree needs its `buffer` TextSummary values updated for affected regions. SumTree doesn't support in-place mutation — so we need to rebuild the affected portion. What does this look like?

### Key Files to Study

- `crates/text/src/text.rs` — `Buffer`, `BufferSnapshot`, `Fragment`, `FragmentTextSummary`, the CRDT edit paths
- `crates/text/src/patch.rs` — `Patch<T>`, the current O(n) coordinate translation
- `crates/buffer_diff/src/buffer_diff.rs` — `BufferDiff`, `InternalDiffHunk`, `compute_hunks`, `stage_or_unstage_hunks_impl`, `patch_for_buffer_range`
- `crates/multi_buffer/src/multi_buffer.rs` — `MultiBufferSnapshot`, `Excerpt`, `ExcerptSummary`, `DiffTransform`, `DiffTransformSummary`, `MultiBufferCursor`, `MultiBufferChunks`, `sync_diff_transforms`
- `crates/editor/src/split.rs` — `SplittableEditor`, `LhsEditor`, `sync_path_excerpts`, `sync_cursor_to_other_side`
- `crates/editor/src/display_map.rs` — `Companion`, `CompanionExcerptPatch`, `DisplayMap`
- `crates/editor/src/display_map/block_map.rs` — `spacer_blocks()`
- `crates/editor/src/split_editor_view.rs` — `SplitEditorView`
- `crates/rope/src/rope.rs` — `TextSummary`, `TextDimension`, `Chunks`
- `zed/docs/plans/diff-aware-buffer.md` — The full planning document
```
