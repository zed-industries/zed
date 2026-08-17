use core::ops;

/// Generate an iteration sequence. This provides *fair* iteration when multiple
/// futures need to be polled concurrently. Only use when the number of items
/// is not known at compile time, else [`super::Indexer`] is more efficient.
pub(crate) struct DynIndexer {
    offset: usize,
    max: usize,
}

impl DynIndexer {
    pub(crate) fn new(max: usize) -> Self {
        Self { offset: 0, max }
    }

    /// Generate a range between `0..max`, incrementing the starting point
    /// for the next iteration.
    pub(crate) fn iter(&mut self) -> DynIndexIter {
        // Increment the starting point for next time.
        let offset = self.offset;
        if self.max > 0 {
            self.offset = (self.offset + 1).wrapping_rem(self.max);
        }

        DynIndexIter {
            iter: (0..self.max),
            offset,
        }
    }
}

pub(crate) struct DynIndexIter {
    iter: ops::Range<usize>,
    offset: usize,
}

impl Iterator for DynIndexIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|pos| (pos + self.offset).wrapping_rem(self.iter.end))
    }
}
