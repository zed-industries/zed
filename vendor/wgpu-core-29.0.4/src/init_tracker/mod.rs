/*! Lazy initialization of texture and buffer memory.

The WebGPU specification requires all texture & buffer memory to be
zero initialized on first read. To avoid unnecessary inits, we track
the initialization status of every resource and perform inits lazily.

The granularity is different for buffers and textures:

- Buffer: Byte granularity to support usecases with large, partially
  bound buffers well.

- Texture: Mip-level per layer. That is, a 2D surface is either
  completely initialized or not, subrects are not tracked.

Every use of a buffer/texture generates a InitTrackerAction which are
recorded and later resolved at queue submit by merging them with the
current state and each other in execution order.

It is important to note that from the point of view of the memory init
system there are two kind of writes:

- **Full writes**: Any kind of memcpy operation. These cause a
  `MemoryInitKind.ImplicitlyInitialized` action.

- **(Potentially) partial writes**: For example, write use in a
  Shader. The system is not able to determine if a resource is fully
  initialized afterwards but is no longer allowed to perform any
  clears, therefore this leads to a
  `MemoryInitKind.NeedsInitializedMemory` action, exactly like a read
  would.

 */

use core::{fmt, iter, ops::Range};

use smallvec::SmallVec;

mod buffer;
mod texture;

pub(crate) use buffer::{BufferInitTracker, BufferInitTrackerAction};
pub(crate) use texture::{
    has_copy_partial_init_tracker_coverage, TextureInitRange, TextureInitTracker,
    TextureInitTrackerAction,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum MemoryInitKind {
    // The memory range is going to be written by an already initialized source,
    // thus doesn't need extra attention other than marking as initialized.
    ImplicitlyInitialized,
    // The memory range is going to be read, therefore needs to ensure prior
    // initialization.
    NeedsInitializedMemory,
}

// Most of the time a resource is either fully uninitialized (one element) or
// initialized (zero elements).
type UninitializedRangeVec<Idx> = SmallVec<[Range<Idx>; 1]>;

/// Tracks initialization status of a linear range from 0..size
#[derive(Debug, Clone)]
pub(crate) struct InitTracker<Idx: Ord + Copy + Default> {
    /// Non-overlapping list of all uninitialized ranges, sorted by
    /// range end.
    uninitialized_ranges: UninitializedRangeVec<Idx>,
}

pub(crate) struct UninitializedIter<'a, Idx: fmt::Debug + Ord + Copy> {
    uninitialized_ranges: &'a UninitializedRangeVec<Idx>,
    drain_range: Range<Idx>,
    next_index: usize,
}

impl<'a, Idx> Iterator for UninitializedIter<'a, Idx>
where
    Idx: fmt::Debug + Ord + Copy,
{
    type Item = Range<Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        self.uninitialized_ranges
            .get(self.next_index)
            .and_then(|range| {
                if range.start < self.drain_range.end {
                    self.next_index += 1;
                    Some(
                        range.start.max(self.drain_range.start)
                            ..range.end.min(self.drain_range.end),
                    )
                } else {
                    None
                }
            })
    }
}

pub(crate) struct InitTrackerDrain<'a, Idx: fmt::Debug + Ord + Copy> {
    uninitialized_ranges: &'a mut UninitializedRangeVec<Idx>,
    drain_range: Range<Idx>,
    first_index: usize,
    next_index: usize,
}

impl<'a, Idx> Iterator for InitTrackerDrain<'a, Idx>
where
    Idx: fmt::Debug + Ord + Copy,
{
    type Item = Range<Idx>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(r) = self
            .uninitialized_ranges
            .get(self.next_index)
            .and_then(|range| {
                if range.start < self.drain_range.end {
                    Some(range.clone())
                } else {
                    None
                }
            })
        {
            self.next_index += 1;
            Some(r.start.max(self.drain_range.start)..r.end.min(self.drain_range.end))
        } else {
            let num_affected = self.next_index - self.first_index;
            if num_affected == 0 {
                return None;
            }
            let first_range = &mut self.uninitialized_ranges[self.first_index];

            // Split one "big" uninitialized range?
            if num_affected == 1
                && first_range.start < self.drain_range.start
                && first_range.end > self.drain_range.end
            {
                let old_start = first_range.start;
                first_range.start = self.drain_range.end;
                self.uninitialized_ranges
                    .insert(self.first_index, old_start..self.drain_range.start);
            }
            // Adjust border ranges and delete everything in-between.
            else {
                let remove_start = if first_range.start >= self.drain_range.start {
                    self.first_index
                } else {
                    first_range.end = self.drain_range.start;
                    self.first_index + 1
                };

                let last_range = &mut self.uninitialized_ranges[self.next_index - 1];
                let remove_end = if last_range.end <= self.drain_range.end {
                    self.next_index
                } else {
                    last_range.start = self.drain_range.end;
                    self.next_index - 1
                };

                self.uninitialized_ranges.drain(remove_start..remove_end);
            }

            None
        }
    }
}

impl<'a, Idx> Drop for InitTrackerDrain<'a, Idx>
where
    Idx: fmt::Debug + Ord + Copy,
{
    fn drop(&mut self) {
        if self.next_index <= self.first_index {
            for _ in self {}
        }
    }
}

impl<Idx> InitTracker<Idx>
where
    Idx: fmt::Debug + Ord + Copy + Default,
{
    pub(crate) fn new(size: Idx) -> Self {
        Self {
            uninitialized_ranges: iter::once(Idx::default()..size).collect(),
        }
    }

    /// Checks for uninitialized ranges within a given query range.
    ///
    /// If `query_range` includes any uninitialized portions of this init
    /// tracker's resource, return the smallest subrange of `query_range` that
    /// covers all uninitialized regions.
    ///
    /// The returned range may be larger than necessary, to keep this function
    /// O(log n).
    pub(crate) fn check(&self, query_range: Range<Idx>) -> Option<Range<Idx>> {
        let index = self
            .uninitialized_ranges
            .partition_point(|r| r.end <= query_range.start);
        self.uninitialized_ranges
            .get(index)
            .and_then(|start_range| {
                if start_range.start < query_range.end {
                    let start = start_range.start.max(query_range.start);
                    match self.uninitialized_ranges.get(index + 1) {
                        Some(next_range) => {
                            if next_range.start < query_range.end {
                                // Would need to keep iterating for more
                                // accurate upper bound. Don't do that here.
                                Some(start..query_range.end)
                            } else {
                                Some(start..start_range.end.min(query_range.end))
                            }
                        }
                        None => Some(start..start_range.end.min(query_range.end)),
                    }
                } else {
                    None
                }
            })
    }

    // Returns an iterator over the uninitialized ranges in a query range.
    pub(crate) fn uninitialized(&mut self, drain_range: Range<Idx>) -> UninitializedIter<'_, Idx> {
        let index = self
            .uninitialized_ranges
            .partition_point(|r| r.end <= drain_range.start);
        UninitializedIter {
            drain_range,
            uninitialized_ranges: &self.uninitialized_ranges,
            next_index: index,
        }
    }

    // Drains uninitialized ranges in a query range.
    pub(crate) fn drain(&mut self, drain_range: Range<Idx>) -> InitTrackerDrain<'_, Idx> {
        let index = self
            .uninitialized_ranges
            .partition_point(|r| r.end <= drain_range.start);
        InitTrackerDrain {
            drain_range,
            uninitialized_ranges: &mut self.uninitialized_ranges,
            first_index: index,
            next_index: index,
        }
    }
}

impl InitTracker<u32> {
    // Makes a single entry uninitialized if not already uninitialized
    pub(crate) fn discard(&mut self, pos: u32) {
        // first range where end>=idx
        let r_idx = self.uninitialized_ranges.partition_point(|r| r.end < pos);
        if let Some(r) = self.uninitialized_ranges.get(r_idx) {
            // Extend range at end
            if r.end == pos {
                // merge with next?
                if let Some(right) = self.uninitialized_ranges.get(r_idx + 1) {
                    if right.start == pos + 1 {
                        self.uninitialized_ranges[r_idx] = r.start..right.end;
                        self.uninitialized_ranges.remove(r_idx + 1);
                        return;
                    }
                }
                self.uninitialized_ranges[r_idx] = r.start..(pos + 1);
            } else if r.start > pos {
                // may still extend range at beginning
                if r.start == pos + 1 {
                    self.uninitialized_ranges[r_idx] = pos..r.end;
                } else {
                    // previous range end must be smaller than idx, therefore no merge possible
                    self.uninitialized_ranges.push(pos..(pos + 1));
                }
            }
        } else {
            self.uninitialized_ranges.push(pos..(pos + 1));
        }
    }
}

#[cfg(test)]
mod test {
    use alloc::{vec, vec::Vec};
    use core::ops::Range;

    type Tracker = super::InitTracker<u32>;

    #[test]
    fn check_for_newly_created_tracker() {
        let tracker = Tracker::new(10);
        assert_eq!(tracker.check(0..10), Some(0..10));
        assert_eq!(tracker.check(0..3), Some(0..3));
        assert_eq!(tracker.check(3..4), Some(3..4));
        assert_eq!(tracker.check(4..10), Some(4..10));
    }

    #[test]
    fn check_for_drained_tracker() {
        let mut tracker = Tracker::new(10);
        tracker.drain(0..10);
        assert_eq!(tracker.check(0..10), None);
        assert_eq!(tracker.check(0..3), None);
        assert_eq!(tracker.check(3..4), None);
        assert_eq!(tracker.check(4..10), None);
    }

    #[test]
    fn check_for_partially_filled_tracker() {
        let mut tracker = Tracker::new(25);
        // Two regions of uninitialized memory
        tracker.drain(0..5);
        tracker.drain(10..15);
        tracker.drain(20..25);

        assert_eq!(tracker.check(0..25), Some(5..25)); // entire range

        assert_eq!(tracker.check(0..5), None); // left non-overlapping
        assert_eq!(tracker.check(3..8), Some(5..8)); // left overlapping region
        assert_eq!(tracker.check(3..17), Some(5..17)); // left overlapping region + contained region

        // right overlapping region + contained region (yes, doesn't fix range end!)
        assert_eq!(tracker.check(8..22), Some(8..22));
        // right overlapping region
        assert_eq!(tracker.check(17..22), Some(17..20));
        // right non-overlapping
        assert_eq!(tracker.check(20..25), None);
    }

    #[test]
    fn drain_already_drained() {
        let mut tracker = Tracker::new(30);
        tracker.drain(10..20);

        // Overlapping with non-cleared
        tracker.drain(5..15); // Left overlap
        tracker.drain(15..25); // Right overlap
        tracker.drain(0..30); // Inner overlap

        // Clear fully cleared
        tracker.drain(0..30);

        assert_eq!(tracker.check(0..30), None);
    }

    #[test]
    fn drain_never_returns_ranges_twice_for_same_range() {
        let mut tracker = Tracker::new(19);
        assert_eq!(tracker.drain(0..19).count(), 1);
        assert_eq!(tracker.drain(0..19).count(), 0);

        let mut tracker = Tracker::new(17);
        assert_eq!(tracker.drain(5..8).count(), 1);
        assert_eq!(tracker.drain(5..8).count(), 0);
        assert_eq!(tracker.drain(1..3).count(), 1);
        assert_eq!(tracker.drain(1..3).count(), 0);
        assert_eq!(tracker.drain(7..13).count(), 1);
        assert_eq!(tracker.drain(7..13).count(), 0);
    }

    #[test]
    fn drain_splits_ranges_correctly() {
        let mut tracker = Tracker::new(1337);
        assert_eq!(
            tracker.drain(21..42).collect::<Vec<Range<u32>>>(),
            vec![21..42]
        );
        assert_eq!(
            tracker.drain(900..1000).collect::<Vec<Range<u32>>>(),
            vec![900..1000]
        );

        // Split ranges.
        assert_eq!(
            tracker.drain(5..1003).collect::<Vec<Range<u32>>>(),
            vec![5..21, 42..900, 1000..1003]
        );
        assert_eq!(
            tracker.drain(0..1337).collect::<Vec<Range<u32>>>(),
            vec![0..5, 1003..1337]
        );
    }

    #[test]
    fn discard_adds_range_on_cleared() {
        let mut tracker = Tracker::new(10);
        tracker.drain(0..10);
        tracker.discard(0);
        tracker.discard(5);
        tracker.discard(9);
        assert_eq!(tracker.check(0..1), Some(0..1));
        assert_eq!(tracker.check(1..5), None);
        assert_eq!(tracker.check(5..6), Some(5..6));
        assert_eq!(tracker.check(6..9), None);
        assert_eq!(tracker.check(9..10), Some(9..10));
    }

    #[test]
    fn discard_does_nothing_on_uncleared() {
        let mut tracker = Tracker::new(10);
        tracker.discard(0);
        tracker.discard(5);
        tracker.discard(9);
        assert_eq!(tracker.uninitialized_ranges.len(), 1);
        assert_eq!(tracker.uninitialized_ranges[0], 0..10);
    }

    #[test]
    fn discard_extends_ranges() {
        let mut tracker = Tracker::new(10);
        tracker.drain(3..7);
        tracker.discard(2);
        tracker.discard(7);
        assert_eq!(tracker.uninitialized_ranges.len(), 2);
        assert_eq!(tracker.uninitialized_ranges[0], 0..3);
        assert_eq!(tracker.uninitialized_ranges[1], 7..10);
    }

    #[test]
    fn discard_merges_ranges() {
        let mut tracker = Tracker::new(10);
        tracker.drain(3..4);
        tracker.discard(3);
        assert_eq!(tracker.uninitialized_ranges.len(), 1);
        assert_eq!(tracker.uninitialized_ranges[0], 0..10);
    }
}
