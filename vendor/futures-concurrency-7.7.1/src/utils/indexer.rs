use core::ops;

/// Generate an iteration sequence. This provides *fair* iteration when multiple
/// futures need to be polled concurrently.
/// It is different from [`super::DynIndexer`] in that the maximum number of items
/// is known at compile time, allowing compiler to better optimize the code, as
/// the division/modulo operations by a dynamic value are more expensive.
pub(crate) struct Indexer<const N: usize> {
    offset: usize,
}

impl<const N: usize> Indexer<N> {
    pub(crate) fn new() -> Self {
        Self { offset: 0 }
    }

    /// Generate a range between `0..max`, incrementing the starting point
    /// for the next iteration.
    pub(crate) fn iter(&mut self) -> IndexIter<N> {
        // Increment the starting point for next time.
        let offset = self.offset;
        if N > 0 {
            self.offset = (self.offset + 1).wrapping_rem(N);
        }

        IndexIter {
            iter: (0..N),
            offset,
        }
    }
}

pub(crate) struct IndexIter<const N: usize> {
    iter: ops::Range<usize>,
    offset: usize,
}

impl<const N: usize> Iterator for IndexIter<N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|pos| (pos + self.offset).wrapping_rem(N))
    }
}
