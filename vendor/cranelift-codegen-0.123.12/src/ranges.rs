//! The [`Ranges`] type stores a list of contiguous index ranges that
//! span some other list's full length.

use alloc::vec::Vec;
use core::ops::Range;

/// A list of contiguous index ranges.
#[derive(Default)]
pub struct Ranges {
    ranges: Vec<u32>,
    reverse: bool,
}

impl Ranges {
    /// Constructs a new, empty, list of ranges with at least the
    /// specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut new = Ranges::default();
        new.reserve(capacity);
        new
    }

    /// Add a new range which begins at the end of the previous range
    /// and ends at the specified offset, exclusive.
    pub fn push_end(&mut self, end: usize) {
        debug_assert!(!self.reverse);
        // To keep this implementation simple we explicitly store the
        // starting index, which is always 0, so that all ranges are
        // represented by adjacent pairs in the list. But we add it
        // lazily so that an empty list doesn't have to allocate.
        if self.ranges.is_empty() {
            self.ranges.push(0);
        }
        self.ranges.push(u32::try_from(end).unwrap());
    }

    /// Number of ranges in this list.
    pub fn len(&self) -> usize {
        self.ranges.len().saturating_sub(1)
    }

    /// Reserves capacity for at least `additional` more ranges to be
    /// added to this list.
    pub fn reserve(&mut self, mut additional: usize) {
        if additional > 0 && self.ranges.is_empty() {
            additional = additional.saturating_add(1);
        }
        self.ranges.reserve(additional);
    }

    /// Get the range at the specified index.
    pub fn get(&self, index: usize) -> Range<usize> {
        let len = self.len();
        assert!(index < len, "index {index} is too big for length {len}");
        let index = self.map_index(index);
        self.ranges[index] as usize..self.ranges[index + 1] as usize
    }

    /// Visit ranges in unspecified order, paired with the index each
    /// range occurs at.
    pub fn iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = (usize, Range<usize>)> + ExactSizeIterator + '_ {
        self.ranges
            .windows(2)
            .enumerate()
            .map(|(index, range)| (self.map_index(index), range[0] as usize..range[1] as usize))
    }

    /// Reverse this list of ranges, so that the first range is at the
    /// last index and the last range is at the first index.
    ///
    /// ```ignore
    /// use cranelift_codegen::ranges::Ranges;
    /// let mut ranges = Ranges::default();
    /// ranges.push_end(4);
    /// ranges.push_end(6);
    /// ranges.reverse_index();
    /// assert_eq!(ranges.get(0), 4..6);
    /// assert_eq!(ranges.get(1), 0..4);
    /// ```
    pub fn reverse_index(&mut self) {
        // We can't easily change the order of the endpoints in
        // self.ranges: they need to be in ascending order or our
        // compressed representation gets complicated. So instead we
        // change our interpretation of indexes using map_index below,
        // controlled by a simple flag. As a bonus, reversing the list
        // is constant-time!
        self.reverse = !self.reverse;
    }

    fn map_index(&self, index: usize) -> usize {
        if self.reverse {
            // These subtractions can't overflow because callers
            // enforce that 0 <= index < self.len()
            self.len() - 1 - index
        } else {
            index
        }
    }

    /// Update these ranges to reflect that the list they refer to has
    /// been reversed. Afterwards, the ranges will still be indexed
    /// in the same order, but the first range will refer to the
    /// same-length range at the end of the target list instead of at
    /// the beginning, and subsequent ranges will proceed backwards
    /// from there.
    ///
    /// ```ignore
    /// use cranelift_codegen::ranges::Ranges;
    /// let mut ranges = Ranges::default();
    /// ranges.push_end(4);
    /// ranges.push_end(6);
    /// ranges.reverse_target(6);
    /// assert_eq!(ranges.get(0), 2..6);
    /// assert_eq!(ranges.get(1), 0..2);
    /// ```
    pub fn reverse_target(&mut self, target_len: usize) {
        let target_len = u32::try_from(target_len).unwrap();
        // The last endpoint added should be the same as the current
        // length of the target list.
        debug_assert_eq!(target_len, *self.ranges.last().unwrap_or(&0));
        for end in self.ranges.iter_mut() {
            *end = target_len - *end;
        }
        // Put the endpoints back in ascending order, but that means
        // now our indexes are backwards.
        self.ranges.reverse();
        self.reverse_index();
    }
}
