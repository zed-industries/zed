# Crash Analysis: SIGABRT in tree-sitter query during autoindent (ZED-9GP)

## Crash Summary
- **Sentry Issue:** ZED-9GP (7562340932) — https://sentry.io/organizations/zed-dev/issues/7562340932/
- **Error:** Fatal Error: SIGABRT (unhandled) — memory corruption / use-after-free
- **Crash Site:** `ts_tree_cursor_parent_node` (`tree_cursor.c:640`), dereferencing
  `parent_entry->subtree->ptr->production_id`
- **Channel / version:** preview, 1.8.0 (`60147a3c`), Linux/Wayland, AMD Polaris (RX 580)
- **Frequency:** 1 event (rare; classic signature of a non-deterministic data race)

## Stack (key application frames, top = crash)
```
ts_tree_cursor_parent_node            tree_cursor.c:640   (reads subtree->ptr->production_id)
ts_query_cursor__advance              query.c:4018
ts_query_cursor_next_match            query.c:4455
tree_sitter::QueryMatches::advance
language::syntax_map::SyntaxMapMatchesLayer::advance   syntax_map.rs
language::syntax_map::SyntaxMapMatches::advance        syntax_map.rs
language::buffer::BufferSnapshot::suggest_autoindents  buffer.rs   (error-query loop)
language::buffer::BufferSnapshot::suggested_indents    buffer.rs
multi_buffer::...::suggested_indents_callback          multi_buffer.rs
editor::edit_prediction::update_visible_edit_prediction edit_prediction.rs
editor::selection::selections_did_change               selection.rs
... mouse_up -> dispatch_event (Wayland)               (main thread)
```

## Root Cause

This is a **use-after-free / in-place mutation of tree-sitter subtree data while a
query cursor is walking it**. The crash dereferences `parent_entry->subtree->ptr`,
which only makes sense for a heap (internal) subtree; a freed or corrupted subtree
makes `production_id` garbage and the subsequent `ts_language_alias_at` indexing
aborts. This is the same signature as upstream/neovim reports where a `TSQueryCursor`
iterates a tree whose subtree memory was freed concurrently (e.g.
tree-sitter/tree-sitter#4044).

The crash happens **on the main thread** inside the error-query loop of
`BufferSnapshot::suggest_autoindents`, driven by edit-prediction recomputing the
suggested indent on every selection change.

Zed touches the same tree-sitter `Tree` / subtree data from **three threads**, all
sharing structure through `SumTree` Arc nodes and tree-sitter's atomic subtree
reference counts:

1. **Main thread** — runs queries against `BufferSnapshot`s (e.g.
   `suggest_autoindents`, highlighting). Reads shared subtrees.
2. **Background parse thread** — `Buffer::reparse` clones the live syntax snapshot
   (`syntax_map.snapshot()`, an Arc clone that *shares the same `Tree` objects*) and
   then runs `interpolate` (`tree.edit()`) + `parse_text` on it via
   `cx.background_spawn` (`crates/language/src/buffer.rs:1889`). Same pattern in
   `snapshot_with_edits` and `preview_edits`.
3. **Dedicated drop thread** — since #50386, `impl Drop for SyntaxSnapshot`
   (`crates/language/src/syntax_map.rs:45`) ships the layer `SumTree` (and therefore
   the `tree_sitter::Tree`s) to a single background thread for deallocation
   (`ts_tree_delete` → `ts_subtree_release`).

Subtree refcounts are atomic on Linux (`atomic.h`), and `tree.edit()` only mutates
a *copy* (`SyntaxSnapshot::interpolate` does `layer.clone()` → `ts_tree_copy` before
editing, with copy-on-write in `ts_subtree_edit`). In isolation this is *almost*
sound. The fragile spot is tree-sitter's copy-on-write decision:

```c
// subtree.c:284
MutableSubtree ts_subtree_make_mut(SubtreePool *pool, Subtree self) {
  if (self.data.is_inline) return ...;
  if (self.ptr->ref_count == 1) return ts_subtree_to_mut_unsafe(self); // <-- edits IN PLACE
  ... // else copy-on-write
```

`ref_count == 1` is a **non-atomic read** used to choose between editing a shared
subtree in place vs. copying it. The whole scheme is only safe if every owner of a
subtree is accounted for by the refcount at all times, and if frees never race with
readers. #50386 is the change that broke the prior implicit serialization: before
it, a `SyntaxSnapshot` was dropped (and its trees freed) on whichever thread held
the last reference — usually the main thread after `did_finish_parsing` swapped in
the new map, or the parse thread — so frees were ordered against that thread's other
tree-sitter work. After #50386, frees run on an **independent third thread with no
ordering** against main-thread queries or background parses that share the same
subtrees. That is exactly the new ingredient needed to turn the long-standing
cross-thread sharing into an observable use-after-free, and it lines up temporally
(the deferred drop landed Mar 2026; this crash is from a Jun 2026 preview).

## Reproduction

I was **not** able to produce a deterministic unit test. The bug is a timing-dependent
data race across three threads on tree-sitter's internal subtree refcounts/data;
a `gpui::test`-style test cannot reliably interleave the background parse thread, the
dedicated drop thread, and a main-thread query at the required granularity, and the
crash depends on the non-atomic CoW read in vendored C.

Recommended ways to surface it instead:
- Build with ThreadSanitizer or AddressSanitizer and run a stress harness that, on one
  buffer, repeatedly: edits (triggering `Buffer::reparse` → background parse), takes
  `BufferSnapshot`s and calls `suggested_indents` / highlight queries on the main
  thread, and drops snapshots (triggering the deferred-drop thread) — all in a tight
  loop across many iterations. ASAN should report the use-after-free in
  `ts_subtree_release` / `ts_tree_cursor_parent_node`.
- Alternatively, temporarily make `SyntaxSnapshot::drop` block on a barrier so the
  free overlaps a main-thread query, to confirm the window.

## Suggested Fix

Primary (lowest risk, addresses the most likely trigger):
- **Revert / gate #50386** (`impl Drop for SyntaxSnapshot` in
  `crates/language/src/syntax_map.rs:45`). It is a pure latency optimization
  (avoiding 10s-of-ms drops on the main thread) and is the most recent change that
  added unordered, concurrent freeing of tree-sitter memory. After reverting,
  re-measure the main-thread drop cost and, if still a problem, pursue a *safer*
  optimization, e.g. perform the drop on the **same single-threaded background
  executor used for that buffer's parsing** so parse and free are serialized with
  respect to each other (they still race with main-thread reads, but that path
  predates this crash). Even better: only defer the drop of snapshots that are
  provably no longer sharing subtrees with any live snapshot — hard to guarantee, so
  prefer the executor-serialization approach.

Secondary / defense-in-depth:
- The non-atomic `ref_count == 1` read in `ts_subtree_make_mut` means tree-sitter is
  not robust to *any* concurrent refcount mutation during `tree.edit()`. If we keep
  freeing trees off-thread, we should ensure no `tree.edit()` (interpolate) can run
  concurrently with a free of a tree that shares subtrees — i.e. serialize all
  parse/interpolate/free work for a given buffer onto one executor.

## Open Questions / Caveats
- The refcounting is *theoretically* atomic-safe; I could not pinpoint the exact
  illegal interleaving by static reasoning alone, so the #50386 attribution is a
  strong hypothesis (supported by the crash signature, the three-thread sharing, and
  the timeline) rather than a proven mechanism. Validation under ASAN/TSAN with the
  stress harness above is the next step before committing a fix.
- `PARSERS` / `QUERY_CURSORS` global pools are `Mutex`-guarded and were ruled out as
  the source.
