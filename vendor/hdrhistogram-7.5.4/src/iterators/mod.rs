use crate::core::counter::Counter;
use crate::Histogram;

/// An iterator that iterates over histogram quantiles.
pub mod quantile;

/// An iterator that iterates linearly over histogram values.
pub mod linear;

/// An iterator that iterates logarithmically over histogram values.
pub mod log;

/// An iterator that iterates over recorded histogram values.
pub mod recorded;

/// An iterator that iterates over histogram values.
pub mod all;

/// Extra information about the picked point in the histogram provided by the picker.
pub struct PickMetadata {
    /// Supply the quantile iterated to in the last `pick()`, if available. If `None` is provided,
    /// the quantile of the current value will be used instead. Probably only useful for the
    /// quantile iterator.
    quantile_iterated_to: Option<f64>,

    /// Supply the value iterated to in the last `pick()`, if the picker can supply a more useful
    /// value than the largest value represented by the bucket.
    value_iterated_to: Option<u64>,
}

impl PickMetadata {
    fn new(quantile_iterated_to: Option<f64>, value_iterated_to: Option<u64>) -> PickMetadata {
        PickMetadata {
            quantile_iterated_to,
            value_iterated_to,
        }
    }
}

/// A trait for designing an subset iterator over values in a `Histogram`.
pub trait PickyIterator<T: Counter> {
    /// Return `Some` if an `IterationValue` should be emitted at this point.
    ///
    /// `index` is a valid index in the relevant histogram.
    ///
    /// This will be called with the same index until it returns `None`. This enables modes of
    /// iteration that pick different values represented by the same bucket, for instance.
    fn pick(
        &mut self,
        index: usize,
        total_count_to_index: u64,
        count_at_index: T,
    ) -> Option<PickMetadata>;

    /// Should we keep iterating even though the last index with non-zero count has already been
    /// picked at least once?
    ///
    /// This will be called on every iteration once the last index with non-zero count has been
    /// picked, even if the index was not advanced in the last iteration (because `pick()` returned
    /// `Some`).
    fn more(&mut self, index_to_pick: usize) -> bool;
}

/// `HistogramIterator` provides a base iterator for a `Histogram`.
///
/// It will iterate over all discrete values until there are no more recorded values (i.e., *not*
/// necessarily until all bins have been exhausted). To facilitate the development of more
/// sophisticated iterators, a *picker* is also provided, which is allowed to only select some bins
/// that should be yielded. The picker may also extend the iteration to include a suffix of empty
/// bins.
pub struct HistogramIterator<'a, T: 'a + Counter, P: PickyIterator<T>> {
    hist: &'a Histogram<T>,
    total_count_to_index: u64,
    count_since_last_iteration: u64,
    count_at_index: T,
    current_index: usize,
    last_picked_index: Option<usize>,
    max_value_index: usize,
    fresh: bool,
    ended: bool,
    picker: P,
}

/// The value emitted at each step when iterating over a `Histogram`.
#[derive(Debug, PartialEq)]
pub struct IterationValue<T: Counter> {
    value_iterated_to: u64,
    quantile: f64,
    quantile_iterated_to: f64,
    count_at_value: T,
    count_since_last_iteration: u64,
}

impl<T: Counter> IterationValue<T> {
    /// Create a new IterationValue.
    pub fn new(
        value_iterated_to: u64,
        quantile: f64,
        quantile_iterated_to: f64,
        count_at_value: T,
        count_since_last_iteration: u64,
    ) -> IterationValue<T> {
        IterationValue {
            value_iterated_to,
            quantile,
            quantile_iterated_to,
            count_at_value,
            count_since_last_iteration,
        }
    }

    /// The value iterated to. Some iterators provide a specific value inside the bucket, while
    /// others just use the highest value in the bucket.
    pub fn value_iterated_to(&self) -> u64 {
        self.value_iterated_to
    }

    /// Percent of recorded values that are at or below the current bucket.
    /// This is simply the quantile multiplied by 100.0, so if you care about maintaining the best
    /// floating-point precision, use `quantile()` instead.
    pub fn percentile(&self) -> f64 {
        self.quantile * 100.0
    }

    /// Quantile of recorded values that are at or below the current bucket.
    pub fn quantile(&self) -> f64 {
        self.quantile
    }

    /// Quantile iterated to, which may be different than `quantile()` when an iterator provides
    /// information about the specific quantile it's iterating to.
    pub fn quantile_iterated_to(&self) -> f64 {
        self.quantile_iterated_to
    }

    /// Recorded count for values equivalent to `value`
    pub fn count_at_value(&self) -> T {
        self.count_at_value
    }

    /// Number of values traversed since the last iteration step
    pub fn count_since_last_iteration(&self) -> u64 {
        self.count_since_last_iteration
    }
}

impl<'a, T: Counter, P: PickyIterator<T>> HistogramIterator<'a, T, P> {
    fn new(h: &'a Histogram<T>, picker: P) -> HistogramIterator<'a, T, P> {
        HistogramIterator {
            hist: h,
            total_count_to_index: 0,
            count_since_last_iteration: 0,
            count_at_index: T::zero(),
            current_index: 0,
            last_picked_index: None,
            max_value_index: h.index_for(h.max()).expect("Either 0 or an existing index"),
            picker,
            fresh: true,
            ended: false,
        }
    }
}

impl<'a, T: 'a, P> Iterator for HistogramIterator<'a, T, P>
where
    T: Counter,
    P: PickyIterator<T>,
{
    type Item = IterationValue<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // here's the deal: we are iterating over all the indices in the histogram's .count array.
        // however, most of those values (especially towards the end) will be zeros, which the
        // original HdrHistogram implementation doesn't yield (probably with good reason -- there
        // could be a lot of them!). so, what we do instead is iterate over indicies until we reach
        // the total *count*. After that, we iterate only until .more() returns false, at which
        // point we stop completely.

        // rust doesn't support tail call optimization, so we'd run out of stack if we simply
        // called self.next() again at the bottom. instead, we loop when we would have yielded None
        // unless we have ended.
        while !self.ended {
            // have we reached the end?
            if self.current_index == self.hist.distinct_values() {
                self.ended = true;
                return None;
            }

            // Have we already picked the index with the last non-zero count in the histogram?
            if self.last_picked_index >= Some(self.max_value_index) {
                // is the picker done?
                if !self.picker.more(self.current_index) {
                    self.ended = true;
                    return None;
                }
            } else {
                // nope -- alright, let's keep iterating
                assert!(self.current_index < self.hist.distinct_values());

                if self.fresh {
                    // at a new index, and not past the max, so there's nonzero counts to add
                    self.count_at_index = self
                        .hist
                        .count_at_index(self.current_index)
                        .expect("Already checked that current_index is < counts len");

                    self.total_count_to_index = self
                        .total_count_to_index
                        .saturating_add(self.count_at_index.as_u64());
                    self.count_since_last_iteration = self
                        .count_since_last_iteration
                        .saturating_add(self.count_at_index.as_u64());

                    // make sure we don't add this index again
                    self.fresh = false;
                }
            }

            // figure out if picker thinks we should yield this value
            if let Some(metadata) = self.picker.pick(
                self.current_index,
                self.total_count_to_index,
                self.count_at_index,
            ) {
                let quantile = self.total_count_to_index as f64 / self.hist.len() as f64;
                let val = IterationValue {
                    value_iterated_to: metadata.value_iterated_to.unwrap_or_else(|| {
                        self.hist
                            .highest_equivalent(self.hist.value_for(self.current_index))
                    }),
                    quantile,
                    quantile_iterated_to: metadata.quantile_iterated_to.unwrap_or(quantile),
                    count_at_value: self
                        .hist
                        .count_at_index(self.current_index)
                        .expect("current index cannot exceed counts length"),
                    count_since_last_iteration: self.count_since_last_iteration,
                };

                // Note that we *don't* increment self.current_index here. The picker will be
                // exposed to the same value again after yielding. This is to allow a picker to
                // pick multiple times at the same index. An example of this is how the linear
                // picker may be using a step size smaller than the bucket size, so it should
                // step multiple times without advancing the index.

                self.count_since_last_iteration = 0;
                self.last_picked_index = Some(self.current_index);
                return Some(val);
            }

            // check the next entry
            self.current_index += 1;
            self.fresh = true;
        }
        None
    }
}
