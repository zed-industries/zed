# Crash Analysis: Cached hover anchors compared with a stale multibuffer snapshot

## Crash Summary

- **Sentry Event:** `592b9150b64948a49b2c2c800213f394`
- **Release:** `0c54c414d522234de7298039708ffe85a116892a` (Zed 1.10.3)
- **Error:** `anchor's path was never added to multibuffer`
- **Crash Site:** `multi_buffer::anchor::ExcerptAnchor::cmp` in `crates/multi_buffer/src/anchor.rs:107`
- **Relevant Stack:** `EditorElement::mouse_moved` → `Editor::update_hovered_link` → `show_link_definition` → `RangeInEditor::point_within_range` → `ExcerptAnchor::cmp`

## Root Cause

`ExcerptAnchor::path` is a `PathKeyIndex`, an index into the `path_keys` table of a particular `MultiBufferSnapshot` generation. The table is append-only as a `MultiBuffer` evolves, but an older snapshot owns an older `Arc<IndexSet<PathKey>>` and does not contain path keys appended after that snapshot was created.

Editor mouse listeners retain the `EditorSnapshot` used during prepaint in their `PositionMap`. If the multibuffer changes before the next paint, a mouse event can therefore be processed with an older snapshot. A hover result, meanwhile, translates its buffer range using a newer snapshot read from `editor.buffer` and caches that range in `HoveredLinkState::symbol_range`.

On a subsequent Cmd/Ctrl+Shift mouse move handled by the old position map, `show_link_definition` tests whether the old trigger point is inside the cached range. `RangeInEditor::point_within_range` passes the old snapshot to `Anchor::cmp`, even though the cached range can contain a `PathKeyIndex` created by the newer snapshot. `ExcerptAnchor::cmp` indexes the old snapshot's `path_keys`, cannot find the newer index, and panics.

This does not require anchors from different `MultiBuffer` entities. It occurs between two valid generations of the same multibuffer. `MultiBuffer::clear` is not the direct cause because it intentionally retains `path_keys`; adding or changing a path after a snapshot was taken is sufficient.

Commits `f8cfad3420` and `ca63c88f50` made hover state and its fallback `symbol_range` persist across more mouse movements. That increases the window in which a cached range can be compared with a snapshot generation different from the one that created it.

## Reproduction

The test `test_hover_link_after_multibuffer_path_changes` in `crates/editor/src/hover_links.rs` performs the following sequence:

1. Open an editor containing a URL and retain the snapshot corresponding to a painted mouse position map.
2. Update the same buffer's multibuffer path, appending a new path key.
3. Cmd/Ctrl+Shift-hover the URL through `Editor::update_hovered_link` using the retained snapshot. URL detection converts and caches the range through the current multibuffer snapshot, so the cached anchors use the new path index.
4. Move within the URL using the retained snapshot. The cached-range check compares the new anchors against the old snapshot and reaches the same panic.

The GPUI visual test harness redraws dirty windows before another simulated platform mouse event, so the test retains the old `EditorSnapshot` explicitly and invokes the same `update_hovered_link` entry point with ordinary display points. It does not construct or inject invalid anchors. The report does not identify whether its cached link came from an LSP definition, URL, file link, or document link; the test uses a URL for deterministic synchronous reproduction, while matching the report's core crash path and panic exactly.

Run:

```sh
RUST_BACKTRACE=1 cargo -q test -p editor test_hover_link_after_multibuffer_path_changes -- --nocapture
```

The test is marked with `#[should_panic(expected = "anchor's path was never added to multibuffer")]`, so the command succeeds while printing the expected panic and backtrace. The reproduced backtrace contains:

- `Editor::update_hovered_link`
- `show_link_definition`
- `RangeInEditor::point_within_range`
- `ExcerptAnchor::cmp`

## Suggested Fix

Before comparing a cached text range in `RangeInEditor::point_within_range`, call `Anchor::is_valid` for both range endpoints and the trigger anchor against `snapshot.buffer_snapshot()`. If any anchor is not valid for that snapshot generation, return `false` and issue a fresh hover request instead of calling `cmp`. `Anchor::is_valid` already handles a missing path index without panicking.

As defense-in-depth, topology-changing multibuffer events should invalidate the complete hover cache and cancel its task, including `BuffersRemoved` as well as `BufferRangesUpdated`. Async hover completion should also verify that it still corresponds to the active trigger/snapshot before installing `symbol_range`. Event invalidation alone is insufficient, however, because a mouse event from an old prepaint map can repopulate hover state after the topology event; the comparison itself must remain safe across snapshot generations.
