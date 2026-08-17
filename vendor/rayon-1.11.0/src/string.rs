//! This module contains the parallel iterator types for owned strings
//! (`String`). You will rarely need to interact with it directly
//! unless you have need to name one of the iterator types.

use crate::iter::plumbing::*;
use crate::math::simplify_range;
use crate::prelude::*;
use std::ops::{Range, RangeBounds};

impl<'a> ParallelDrainRange<usize> for &'a mut String {
    type Iter = Drain<'a>;
    type Item = char;

    fn par_drain<R: RangeBounds<usize>>(self, range: R) -> Self::Iter {
        Drain {
            range: simplify_range(range, self.len()),
            string: self,
        }
    }
}

/// Draining parallel iterator that moves a range of characters out of a string,
/// but keeps the total capacity.
#[derive(Debug)]
pub struct Drain<'a> {
    string: &'a mut String,
    range: Range<usize>,
}

impl<'a> ParallelIterator for Drain<'a> {
    type Item = char;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.string[self.range.clone()]
            .par_chars()
            .drive_unindexed(consumer)
    }
}

impl<'a> Drop for Drain<'a> {
    fn drop(&mut self) {
        // Remove the drained range.
        self.string.drain(self.range.clone());
    }
}
