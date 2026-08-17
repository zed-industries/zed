use crate::core::counter::Counter;
use crate::iterators::{HistogramIterator, PickMetadata, PickyIterator};
use crate::Histogram;

/// An iterator that will yield at quantile steps through the histogram's value range.
pub struct Iter<'a, T: 'a + Counter> {
    hist: &'a Histogram<T>,
    ticks_per_half_distance: u32,
    quantile_to_iterate_to: f64,
    reached_end: bool,
}

impl<'a, T: 'a + Counter> Iter<'a, T> {
    /// Construct a new iterator. See `Histogram::iter_quantiles` for details.
    pub fn new(
        hist: &'a Histogram<T>,
        ticks_per_half_distance: u32,
    ) -> HistogramIterator<'a, T, Iter<'a, T>> {
        assert!(
            ticks_per_half_distance > 0,
            "Ticks per half distance must be > 0"
        );

        HistogramIterator::new(
            hist,
            Iter {
                hist,
                ticks_per_half_distance,
                quantile_to_iterate_to: 0.0,
                reached_end: false,
            },
        )
    }
}

impl<'a, T: 'a + Counter> PickyIterator<T> for Iter<'a, T> {
    #[allow(clippy::float_cmp)]
    fn pick(&mut self, _: usize, running_total: u64, count_at_index: T) -> Option<PickMetadata> {
        if count_at_index == T::zero() {
            return None;
        }

        // This calculation, combined with the `quantile * count` in `value_at_quantile`, tends
        // to produce a count_at_quantile that is 1 ulp wrong. That's just the way IEEE754 works.
        let current_quantile = running_total as f64 / self.hist.len() as f64;
        if current_quantile < self.quantile_to_iterate_to {
            return None;
        }

        // Because there are effectively two quantiles in play (the quantile of the value for the
        // bucket we're at aka "value quantile", and the quantile we're iterating to aka "iteration
        // quantile", which may be significantly different, especially in highly non-uniform
        // distributions), the behavior around 1.0 is a little tricky.
        //
        // The desired behavior is that we always iterate until the iteration quantile reaches 1.0,
        // but if the value quantile reaches 1.0 (by reaching the last index with a non-zero count)
        // before that point, we skip the remaining intermediate iterations in that same index and
        // jump straight to iteration quantile 1.0.
        // This is effectively a policy decision, but it is consistent with other iterators: they
        // don't just stop when the quantile reaches 1.0 upon first entering the final bucket.
        // At the same time, it's arguably unhelpful to have a bunch of all-but-identical quantile
        // ticks, hence skipping the intermediate iterations. (This is also how the Java impl
        // behaves.)
        //
        // Note that it is impossible to have the value quantile lower than the iteration quantile
        // since the value quantile incorporates the count for the entire bucket when it's first
        // entered, while the hypothetical fractional count that the iteration quantile would use is
        // necessarily less than that.
        //
        // Cases for ending iteration:
        // 1. Iteration quantile reaches 1.0 along with the value quantile reaching 1.0 at the max
        //    value index
        // 2. Iteration quantile is below 1.0 when the value quantile reaches 1.0 at the max value
        //    index
        // 3. Same as #1, but not at the max value index because total count has saturated. This
        //    means that more() will not be called.
        // 4. Same as #2, but not at the max value index because total count has saturated. See #3.

        if self.reached_end {
            // #3, #4 part 2: Need to check here, not just in `more()`: when we see quantile 1.0 and
            // set `reached_end`, `more()` will not be called (because we haven't reached the last
            // non-zero-count index) so it can't stop iteration, and we must stop it here.
            //
            // This will be hit for all remaining non-zero-count indices, then control will proceed
            // to `more()`.
            return None;
        }

        // #1: If we reach iteration quantile 1.0 at the same time as value quantile 1.0 (because we
        // moved to the final non-zero-count index exactly when the iteration ticked over to 1.0),
        // we want to emit a value at that point, but not proceed past that.
        // #2, last phase: This could also be the second visit to the max value index in the #2 case
        // where `quantile_to_iterate_to` has been set to 1.0.
        // #3, #4 last phase: Similar, but iteration proceeded normally up to 1.0 without any
        // last-bucket skipping because it wasn't at the last bucket.
        if self.quantile_to_iterate_to == 1.0 {
            // We want to pick this value but not do the math below because it doesn't work when
            // quantile >= 1.0.
            //
            // We also want to prevent any further iteration.
            self.reached_end = true;
            return Some(PickMetadata::new(Some(1.0), None));
        }

        // #2, first phase:
        // Value quantile reached 1.0 while the iteration quantile is somewhere below 1.0 (it can be
        // arbitrarily close to 0 for lopsided histograms). So, we continue with normal quantile
        // tick logic for the first time, and pick up the #2 case in `more()` below.

        // The choice to maintain fixed-sized "ticks" in each half-distance to 100% [starting from
        // 0%], as opposed to a "tick" size that varies with each interval, was made to make the
        // steps easily comprehensible and readable to humans. The resulting quantile steps are
        // much easier to browse through in a quantile distribution output, for example.
        //
        // We calculate the number of equal-sized "ticks" that the 0-1 range will be divided by
        // at the current scale. The scale is determined by the quantile level we are iterating
        // to. The following math determines the tick size for the current scale, and maintain a
        // fixed tick size for the remaining "half the distance to 100%" [from either 0% or from
        // the previous half-distance]. When that half-distance is crossed, the scale changes and
        // the tick size is effectively cut in half.
        //
        // Calculate the number of times we've halved the distance to 100%, This is 1 at 50%, 2 at
        // 75%, 3 at 87.5%, etc. 2 ^ num_halvings is the number of slices that will fit into 100%.
        // At 50%, num_halvings would be 1, so 2 ^ 1 would yield 2 slices, etc. At any given number
        // of slices, the last slice is what we're going to traverse the first half of. With 1 total
        // slice, traverse half to get to 50%. Then traverse half of the last (second) slice to get
        // to 75%, etc.
        // Minimum of 0 (1.0/1.0 = 1, log 2 of which is 0) so unsigned cast is safe.
        // Won't hit the `inf` case because quantile < 1.0, so this should yield an actual number.
        let num_halvings = (1.0 / (1.0 - self.quantile_to_iterate_to)).log2() as u32;
        // Calculate the total number of ticks in 0-1 given that half of each slice is tick'd.
        // The number of slices is 2 ^ num_halvings, and each slice has two "half distances" to
        // tick, so we add an extra power of two to get ticks per whole distance.
        // Use u64 math so that there's less risk of overflow with large numbers of ticks and data
        // that ends up needing large numbers of halvings.
        let total_ticks = u64::from(self.ticks_per_half_distance)
            .checked_mul(
                1_u64
                    .checked_shl(num_halvings + 1)
                    .expect("too many halvings"),
            )
            .expect("too many total ticks");
        let increment_size = 1.0_f64 / total_ticks as f64;

        let metadata = PickMetadata::new(Some(self.quantile_to_iterate_to), None);

        let sum = self.quantile_to_iterate_to + increment_size;
        self.quantile_to_iterate_to = if sum == self.quantile_to_iterate_to {
            // the iteration has reached the point where the increment is too small to actually
            // change an f64 slightly smaller than 1.0, so just short circuit to 1.0.
            // This happens easily in case #4, and plausibly in #3: it will iterate up to 1.0
            // without any skipping, which will
            1.0
        } else {
            sum
        };
        Some(metadata)
    }

    fn more(&mut self, _: usize) -> bool {
        // One of the end cases has already declared we're done.
        if self.reached_end {
            return false;
        }

        // #2, middle phase: already picked the max-value index once with iteration quantile < 1.0,
        // and `more()` is now called (for the first time), so iterate one more time, but jump to
        // quantile 1.0 while doing so. We don't set `reached_end` here because we do want 1 more
        // iteration.
        self.quantile_to_iterate_to = 1.0;
        true
    }
}
