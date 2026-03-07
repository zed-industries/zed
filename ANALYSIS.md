# Crash Analysis: Slice index bounds error in SelectionsCollection::disjoint_in_range

## Crash Summary
- **Sentry Issue:** ZED-473 (https://sentry.io/organizations/zed-dev/issues/7171509125/)
- **Error:** slice index starts at <int> but ends at <int> (panic when start_ix > end_ix in slice operation)
- **Crash Site:** `editor::selections_collection::SelectionsCollection::disjoint_in_range` in `selections_collection.rs` line 234
- **Event Count:** 1108 occurrences
- **Platform:** Windows

## Root Cause

The `disjoint_in_range` function in `SelectionsCollection` performs two separate binary searches to find the start and end indices of selections within a given anchor range:

```rust
let start_ix = match self.disjoint
    .binary_search_by(|probe| probe.end.cmp(&range.start, snapshot.buffer_snapshot()))
{
    Ok(ix) | Err(ix) => ix,
};
let end_ix = match self.disjoint
    .binary_search_by(|probe| probe.start.cmp(&range.end, snapshot.buffer_snapshot()))
{
    Ok(ix) => ix + 1,
    Err(ix) => ix,
};
resolve_selections_wrapping_blocks(&self.disjoint[start_ix..end_ix], snapshot).collect()
```

The crash occurs when `start_ix > end_ix`, which can happen when:

1. **Stale anchors:** The selections in `disjoint` contain anchors pointing to excerpts that no longer exist in the provided snapshot, causing anchor comparisons to produce inconsistent ordering results.

2. **Concurrent modification:** During the prepaint phase (as shown in the stack trace), the editor element reads selections while the underlying buffer/excerpt structure may have been modified, leading to a mismatch between the selections' anchors and the snapshot.

3. **Excerpt removal/reordering:** When excerpts are removed from a MultiBuffer, the selections may still reference those removed excerpts. The anchor comparison logic in `Anchor::cmp` falls back to `Ordering::Equal` when an excerpt cannot be found, which can cause the binary search invariants to be violated.

The stack trace shows this occurring during `EditorElement::prepaint` -> `editor.selections.disjoint_in_range()`, which happens during window layout/rendering. This timing-sensitive code path is vulnerable when the editor's selection state becomes temporarily inconsistent with the display snapshot.

## Reproduction

The test creates a MultiBuffer with multiple excerpts, establishes selections across them, then removes an excerpt. When `disjoint_in_range` is called with the updated snapshot but before selections are refreshed, the binary search can produce invalid indices.

Run the reproduction test with:
```
cargo test -p editor test_disjoint_in_range_with_stale_selections
```

## Suggested Fix

The fix should guard the slice operation to ensure `start_ix <= end_ix`. When `start_ix > end_ix`, return an empty vector since this indicates the selections and snapshot are in an inconsistent state (no valid selections in the requested range):

```rust
pub fn disjoint_in_range<D>(
    &self,
    range: Range<Anchor>,
    snapshot: &DisplaySnapshot,
) -> Vec<Selection<D>>
where
    D: MultiBufferDimension + Sub + AddAssign<<D as Sub>::Output> + Ord + std::fmt::Debug,
{
    let start_ix = match self
        .disjoint
        .binary_search_by(|probe| probe.end.cmp(&range.start, snapshot.buffer_snapshot()))
    {
        Ok(ix) | Err(ix) => ix,
    };
    let end_ix = match self
        .disjoint
        .binary_search_by(|probe| probe.start.cmp(&range.end, snapshot.buffer_snapshot()))
    {
        Ok(ix) => ix + 1,
        Err(ix) => ix,
    };
    // Guard against inconsistent state where binary search produces start > end
    // (can happen when selections reference stale/removed excerpts)
    if start_ix > end_ix {
        return Vec::new();
    }
    resolve_selections_wrapping_blocks(&self.disjoint[start_ix..end_ix], snapshot).collect()
}
```

This is a defensive fix that prevents the panic while maintaining correct behavior - when selections are stale, returning an empty result is appropriate since those selections don't actually exist in the current snapshot.
