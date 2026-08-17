use crate::core::counter::Counter;
use crate::iterators::{HistogramIterator, PickMetadata, PickyIterator};
use crate::Histogram;

/// An iterator that will yield at log-size steps through the histogram's value range.
pub struct Iter<'a, T: 'a + Counter> {
    hist: &'a Histogram<T>,

    // > 1.0
    next_value_reporting_level: f64,
    // > 1.0
    log_base: f64,

    current_step_lowest_value_reporting_level: u64,
    current_step_highest_value_reporting_level: u64,
}

impl<'a, T: 'a + Counter> Iter<'a, T> {
    /// Construct a new logarithmic iterator. See `Histogram::iter_log` for details.
    pub fn new(
        hist: &'a Histogram<T>,
        value_units_in_first_bucket: u64,
        log_base: f64,
    ) -> HistogramIterator<'a, T, Iter<'a, T>> {
        assert!(
            value_units_in_first_bucket > 0,
            "value_units_per_bucket must be > 0"
        );
        assert!(log_base > 1.0, "log_base must be > 1.0");

        let new_lowest = hist.lowest_equivalent(value_units_in_first_bucket - 1);
        HistogramIterator::new(
            hist,
            Iter {
                hist,
                log_base,
                next_value_reporting_level: value_units_in_first_bucket as f64,
                current_step_highest_value_reporting_level: value_units_in_first_bucket - 1,
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
            // implies log_base must be > 1.0
            self.next_value_reporting_level *= self.log_base;
            // won't underflow since next_value_reporting_level starts > 0 and only grows
            self.current_step_highest_value_reporting_level =
                self.next_value_reporting_level as u64 - 1;
            self.current_step_lowest_value_reporting_level = self
                .hist
                .lowest_equivalent(self.current_step_highest_value_reporting_level);
            Some(metadata)
        } else {
            None
        }
    }

    fn more(&mut self, index_to_pick: usize) -> bool {
        // If the next iterate will not move to the next sub bucket index (which is empty if if we
        // reached this point), then we are not yet done iterating (we want to iterate until we are
        // no longer on a value that has a count, rather than util we first reach the last value
        // that has a count. The difference is subtle but important)...
        self.hist
            .lowest_equivalent(self.next_value_reporting_level as u64)
            < self.hist.value_for(index_to_pick)
    }
}
