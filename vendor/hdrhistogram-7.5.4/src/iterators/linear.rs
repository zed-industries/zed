use crate::core::counter::Counter;
use crate::iterators::{HistogramIterator, PickMetadata, PickyIterator};
use crate::Histogram;

/// An iterator that will yield at fixed-size steps through the histogram's value range.
pub struct Iter<'a, T: 'a + Counter> {
    hist: &'a Histogram<T>,

    // > 0
    value_units_per_bucket: u64,
    current_step_highest_value_reporting_level: u64,
    current_step_lowest_value_reporting_level: u64,
}

impl<'a, T: 'a + Counter> Iter<'a, T> {
    /// Construct a new linear iterator. See `Histogram::iter_linear` for details.
    pub fn new(
        hist: &'a Histogram<T>,
        value_units_per_bucket: u64,
    ) -> HistogramIterator<'a, T, Iter<'a, T>> {
        assert!(
            value_units_per_bucket > 0,
            "value_units_per_bucket must be > 0"
        );

        let new_lowest = hist.lowest_equivalent(value_units_per_bucket - 1);
        HistogramIterator::new(
            hist,
            Iter {
                hist,
                value_units_per_bucket,
                // won't underflow because value_units_per_bucket > 0
                current_step_highest_value_reporting_level: value_units_per_bucket - 1,
                current_step_lowest_value_reporting_level: new_lowest,
            },
        )
    }
}

impl<'a, T: 'a + Counter> PickyIterator<T> for Iter<'a, T> {
    fn pick(&mut self, index: usize, _: u64, _: T) -> Option<PickMetadata> {
        let val = self.hist.value_for(index);
        if val >= self.current_step_lowest_value_reporting_level || index == self.hist.last_index()
        {
            let metadata =
                PickMetadata::new(None, Some(self.current_step_highest_value_reporting_level));
            self.current_step_highest_value_reporting_level += self.value_units_per_bucket;
            self.current_step_lowest_value_reporting_level = self
                .hist
                .lowest_equivalent(self.current_step_highest_value_reporting_level);
            Some(metadata)
        } else {
            None
        }
    }

    fn more(&mut self, index_to_pick: usize) -> bool {
        // If the next iterate will not move to the next sub bucket index (which is empty if
        // if we reached this point), then we are not yet done iterating (we want to iterate
        // until we are no longer on a value that has a count, rather than until we first reach
        // the last value that has a count. The difference is subtle but important)...
        // When this is called, we're about to begin the "next" iteration, so
        // current_step_highest_value_reporting_level has already been incremented,
        // and we use it without incrementing its value.
        let next_index = index_to_pick.checked_add(1).expect("usize overflow");
        self.current_step_highest_value_reporting_level < self.hist.value_for(next_index)
    }
}
