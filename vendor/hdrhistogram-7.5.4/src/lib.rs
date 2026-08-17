//! HdrSample is a port of Gil Tene's HdrHistogram to native Rust. It provides recording and
//! analyzing of sampled data value counts across a large, configurable value range with
//! configurable precision within the range. The resulting "HDR" histogram allows for fast and
//! accurate analysis of the extreme ranges of data with non-normal distributions, like latency.
//!
//! # HdrHistogram
//!
//! What follows is a description from [the HdrHistogram
//! website](https://hdrhistogram.github.io/HdrHistogram/). Users are encouraged to read the
//! documentation from the original [Java
//! implementation](https://github.com/HdrHistogram/HdrHistogram), as most of the concepts
//! translate directly to the Rust port.
//!
//! HdrHistogram supports the recording and analyzing of sampled data value counts across a
//! configurable integer value range with configurable value precision within the range. Value
//! precision is expressed as the number of significant digits in the value recording, and provides
//! control over value quantization behavior across the value range and the subsequent value
//! resolution at any given level.
//!
//! For example, a Histogram could be configured to track the counts of observed integer values
//! between 0 and 3,600,000,000 while maintaining a value precision of 3 significant digits across
//! that range. Value quantization within the range will thus be no larger than 1/1,000th (or 0.1%)
//! of any value. This example Histogram could be used to track and analyze the counts of observed
//! response times ranging between 1 microsecond and 1 hour in magnitude, while maintaining a value
//! resolution of 1 microsecond up to 1 millisecond, a resolution of 1 millisecond (or better) up
//! to one second, and a resolution of 1 second (or better) up to 1,000 seconds. At it's maximum
//! tracked value (1 hour), it would still maintain a resolution of 3.6 seconds (or better).
//!
//! HDR Histogram is designed for recording histograms of value measurements in latency and
//! performance sensitive applications. Measurements show value recording times as low as 3-6
//! nanoseconds on modern (circa 2014) Intel CPUs. The HDR Histogram maintains a fixed cost in both
//! space and time. A Histogram's memory footprint is constant, with no allocation operations
//! involved in recording data values or in iterating through them. The memory footprint is fixed
//! regardless of the number of data value samples recorded, and depends solely on the dynamic
//! range and precision chosen. The amount of work involved in recording a sample is constant, and
//! directly computes storage index locations such that no iteration or searching is ever involved
//! in recording data values.
//!
//! If you are looking for FFI bindings to
//! [`HdrHistogram_c`](https://github.com/HdrHistogram/HdrHistogram_c), you want the
//! [`hdrhistogram_c`](https://crates.io/crates/hdrhistogram_c) crate instead.
//!
//! # Interacting with the library
//!
//! HdrSample's API follows that of the original HdrHistogram Java implementation, with some
//! modifications to make its use more idiomatic in Rust. The description in this section has been
//! adapted from that given by the [Python port](https://github.com/HdrHistogram/HdrHistogram_py),
//! as it gives a nicer first-time introduction to the use of HdrHistogram than the Java docs do.
//!
//! HdrSample is generally used in one of two modes: recording samples, or querying for analytics.
//! In distributed deployments, the recording may be performed remotely (and possibly in multiple
//! locations), to then be aggregated later in a central location for analysis.
//!
//! ## Recording samples
//!
//! A histogram instance is created using the `::new` methods on the `Histogram` struct. These come
//! in three variants: `new`, `new_with_max`, and `new_with_bounds`. The first of these only sets
//! the required precision of the sampled data, but leaves the value range open such that any value
//! may be recorded. A `Histogram` created this way (or one where auto-resize has been explicitly
//! enabled) will automatically resize itself if a value that is too large to fit in the current
//! dataset is encountered. `new_with_max` sets an upper bound on the values to be recorded, and
//! disables auto-resizing, thus preventing any re-allocation during recording. If the application
//! attempts to record a larger value than this maximum bound, the `record` call will return an
//! error. Finally, `new_with_bounds` restricts the lowest representable value of the dataset,
//! such that a smaller range needs to be covered (thus reducing the overall allocation size).
//!
//! For example the example below shows how to create a `Histogram` that can count values in the
//! `[1..3600000]` range with 1% precision, which could be used to track latencies in the range `[1
//! msec..1 hour]`).
//!
//! ```
//! use hdrhistogram::Histogram;
//! let mut hist = Histogram::<u64>::new_with_bounds(1, 60 * 60 * 1000, 2).unwrap();
//!
//! // samples can be recorded using .record, which will error if the value is too small or large
//! hist.record(54321).expect("value 54321 should be in range");
//!
//! // for ergonomics, samples can also be recorded with +=
//! // this call will panic if the value is out of range!
//! hist += 54321;
//!
//! // if the code that generates the values is subject to Coordinated Omission,
//! // the self-correcting record method should be used instead.
//! // for example, if the expected sampling interval is 10 msec:
//! hist.record_correct(54321, 10).expect("value 54321 should be in range");
//! ```
//!
//! Note the `u64` type. This type can be changed to reduce the storage overhead for all the
//! histogram bins, at the cost of a risk of saturating if a large number of samples end up in the
//! same bin.
//!
//! ## Querying samples
//!
//! At any time, the histogram can be queried to return interesting statistical measurements, such
//! as the total number of recorded samples, or the value at a given quantile:
//!
//! ```
//! use hdrhistogram::Histogram;
//! let hist = Histogram::<u64>::new(2).unwrap();
//! // ...
//! println!("# of samples: {}", hist.len());
//! println!("99.9'th percentile: {}", hist.value_at_quantile(0.999));
//! ```
//!
//! Several useful iterators are also provided for quickly getting an overview of the dataset. The
//! simplest one is `iter_recorded()`, which yields one item for every non-empty sample bin. All
//! the HdrHistogram iterators are supported in HdrSample, so look for the `*Iterator` classes in
//! the [Java documentation](https://hdrhistogram.github.io/HdrHistogram/JavaDoc/).
//!
//! ```
//! use hdrhistogram::Histogram;
//! let hist = Histogram::<u64>::new(2).unwrap();
//! // ...
//! for v in hist.iter_recorded() {
//!     println!("{}'th percentile of data is {} with {} samples",
//!         v.percentile(), v.value_iterated_to(), v.count_at_value());
//! }
//! ```
//!
//! ## Panics and error handling
//!
//! As long as you're using safe, non-panicking functions (see below), this library should never
//! panic. Any panics you encounter are a bug; please file them in the issue tracker.
//!
//! A few functions have their functionality exposed via `AddAssign` and `SubAssign`
//! implementations. These alternate forms are equivalent to simply calling `unwrap()` on the
//! normal functions, so the normal rules of `unwrap()` apply: view with suspicion when used in
//! production code, etc.
//!
//! | Returns Result                 | Panics on error    | Functionality                   |
//! | ------------------------------ | ------------------ | ------------------------------- |
//! | `h.record(v)`                  | `h += v`           | Increment count for value `v`   |
//! | `h.add(h2)`                    | `h += h2`          | Add `h2`'s counts to `h`        |
//! | `h.subtract(h2)`               | `h -= h2`          | Subtract `h2`'s counts from `h` |
//!
//! Other than the panicking forms of the above functions, everything will return `Result` or
//! `Option` if it can fail.
//!
//! ## `usize` limitations
//!
//! Depending on the configured number of significant digits and maximum value, a histogram's
//! internal storage may have hundreds of thousands of cells. Systems with a 16-bit `usize` cannot
//! represent pointer offsets that large, so relevant operations (creation, deserialization, etc)
//! will fail with a suitable error (e.g. `CreationError::UsizeTypeTooSmall`). If you are using such
//! a system and hitting these errors, reducing the number of significant digits will greatly reduce
//! memory consumption (and therefore the need for large `usize` values). Lowering the max value may
//! also help as long as resizing is disabled.
//!
//! 32- and above systems will not have any such issues, as all possible histograms fit within a
//! 32-bit index.
//!
//! ## Floating point accuracy
//!
//! Some calculations inherently involve floating point values, like `value_at_quantile`, and are
//! therefore subject to the precision limits of IEEE754 floating point calculations. The user-
//! visible consequence of this is that in certain corner cases, you might end up with a bucket (and
//! therefore value) that is higher or lower than it would be if the calculation had been done
//! with arbitrary-precision arithmetic. However, double-precision IEEE754 (i.e. `f64`) is very
//! good at its job, so these cases should be rare. Also, we haven't seen a case that was off by
//! more than one bucket.
//!
//! To minimize FP precision losses, we favor working with quantiles rather than percentiles. A
//! quantile represents a portion of a set with a number in `[0, 1]`. A percentile is the same
//! concept, except it uses the range `[0, 100]`. Working just with quantiles means we can skip an
//! FP operation in a few places, and therefore avoid opportunities for precision loss to creep in.
//!
//! # Limitations and Caveats
//!
//! As with all the other HdrHistogram ports, the latest features and bug fixes from the upstream
//! HdrHistogram implementations may not be available in this port. A number of features have also
//! not (yet) been implemented:
//!
//!  - Concurrency support (`AtomicHistogram`, `ConcurrentHistogram`, …).
//!  - `DoubleHistogram`.
//!  - The `Recorder` feature of HdrHistogram.
//!  - Value shifting ("normalization").
//!  - Textual output methods. These seem almost orthogonal to HdrSample, though it might be
//!    convenient if we implemented some relevant traits (CSV, JSON, and possibly simple
//!    `fmt::Display`).
//!
//! Most of these should be fairly straightforward to add, as the code aligns pretty well with the
//! original Java/C# code. If you do decide to implement one and send a PR, please make sure you
//! also port the [test
//! cases](https://github.com/HdrHistogram/HdrHistogram/tree/master/src/test/java/org/HdrHistogram),
//! and try to make sure you implement appropriate traits to make the use of the feature as
//! ergonomic as possible.

#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences
)]
// Enable feature(test) is enabled so that we can have benchmarks of private code
#![cfg_attr(all(test, feature = "bench_private"), feature(test))]

#[cfg(all(test, feature = "bench_private"))]
extern crate test;

#[cfg(feature = "serialization")]
#[macro_use]
extern crate nom;

use num_traits::ToPrimitive;
use std::borrow::Borrow;
use std::cmp;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use iterators::HistogramIterator;

/// Min value of a new histogram.
/// Equivalent to `u64::max_value()`, but const functions aren't allowed (yet).
/// See <https://github.com/rust-lang/rust/issues/24111>
const ORIGINAL_MIN: u64 = (-1_i64 >> 63) as u64;
/// Max value of a new histogram.
const ORIGINAL_MAX: u64 = 0;

/// `Histogram` is the core data structure in HdrSample. It records values, and performs analytics.
///
/// At its heart, it keeps the count for recorded samples in "buckets" of values. The resolution
/// and distribution of these buckets is tuned based on the desired highest trackable value, as
/// well as the user-specified number of significant decimal digits to preserve. The values for the
/// buckets are kept in a way that resembles floats and doubles: there is a mantissa and an
/// exponent, and each bucket represents a different exponent. The "sub-buckets" within a bucket
/// represent different values for the mantissa.
///
/// To a first approximation, the sub-buckets of the first
/// bucket would hold the values `0`, `1`, `2`, `3`, …, the sub-buckets of the second bucket would
/// hold `0`, `2`, `4`, `6`, …, the third would hold `0`, `4`, `8`, and so on. However, the low
/// half of each bucket (except bucket 0) is unnecessary, since those values are already covered by
/// the sub-buckets of all the preceeding buckets. Thus, `Histogram` keeps the top half of every
/// such bucket.
///
/// For the purposes of explanation, consider a `Histogram` with 2048 sub-buckets for every bucket,
/// and a lowest discernible value of 1:
///
/// <pre>
/// The 0th bucket covers 0...2047 in multiples of 1, using all 2048 sub-buckets
/// The 1st bucket covers 2048..4097 in multiples of 2, using only the top 1024 sub-buckets
/// The 2nd bucket covers 4096..8191 in multiple of 4, using only the top 1024 sub-buckets
/// ...
/// </pre>
///
/// Bucket 0 is "special" here. It is the only one that has 2048 entries. All the rest have
/// 1024 entries (because their bottom half overlaps with and is already covered by the all of
/// the previous buckets put together). In other words, the `k`'th bucket could represent `0 *
/// 2^k` to `2048 * 2^k` in 2048 buckets with `2^k` precision, but the midpoint of `1024 * 2^k
/// = 2048 * 2^(k-1)`, which is the k-1'th bucket's end. So, we would use the previous bucket
/// for those lower values as it has better precision.
///
#[derive(Debug, Clone)]
pub struct Histogram<T: Counter> {
    auto_resize: bool,

    // >= 2 * lowest_discernible_value
    highest_trackable_value: u64,
    // >= 1
    lowest_discernible_value: u64,
    // in [0, 5]
    significant_value_digits: u8,

    // in [1, 64]
    bucket_count: u8,
    // 2^(sub_bucket_half_count_magnitude + 1) = [2, 2^18]
    sub_bucket_count: u32,
    // sub_bucket_count / 2 = [1, 2^17]
    sub_bucket_half_count: u32,
    // In [0, 17]
    sub_bucket_half_count_magnitude: u8,
    // The bottom sub bucket's bits set, shifted by unit magnitude.
    // The highest bit will be (one-indexed) sub bucket count magnitude + unit_magnitude.
    sub_bucket_mask: u64,

    // Number of leading zeros that would be used by the largest value in bucket 0.
    // in [1, 63]
    leading_zero_count_base: u8,

    // Largest exponent of 2 that's smaller than the lowest discernible value. In [0, 62].
    unit_magnitude: u8,
    // low unit_magnitude bits set
    unit_magnitude_mask: u64,

    max_value: u64,
    min_non_zero_value: u64,

    total_count: u64,
    counts: Vec<T>,
}

/// Module containing the implementations of all `Histogram` iterators.
pub mod iterators;

impl<T: Counter> Histogram<T> {
    // ********************************************************************************************
    // Histogram administrative read-outs
    // ********************************************************************************************

    /// Get the current number of distinct values that can be represented in the histogram.
    pub fn distinct_values(&self) -> usize {
        self.counts.len()
    }

    /// Get the lowest discernible value for the histogram in its current configuration.
    pub fn low(&self) -> u64 {
        self.lowest_discernible_value
    }

    /// Get the highest trackable value for the histogram in its current configuration.
    pub fn high(&self) -> u64 {
        self.highest_trackable_value
    }

    /// Get the number of significant value digits kept by this histogram.
    pub fn sigfig(&self) -> u8 {
        self.significant_value_digits
    }

    /// Get the total number of samples recorded.
    #[deprecated(since = "6.0.0", note = "use `len` instead")]
    pub fn count(&self) -> u64 {
        self.total_count
    }

    /// Get the total number of samples recorded.
    pub fn len(&self) -> u64 {
        self.total_count
    }

    /// Returns true if this histogram has no recorded values.
    pub fn is_empty(&self) -> bool {
        self.total_count == 0
    }

    /// Get the number of buckets used by the histogram to cover the highest trackable value.
    ///
    /// This method differs from `.len()` in that it does not count the sub buckets within each
    /// bucket.
    ///
    /// This method is probably only useful for testing purposes.
    pub fn buckets(&self) -> u8 {
        self.bucket_count
    }

    /// Returns true if this histogram is currently able to auto-resize as new samples are recorded.
    pub fn is_auto_resize(&self) -> bool {
        self.auto_resize
    }

    // ********************************************************************************************
    // Methods for looking up the count for a given value/index
    // ********************************************************************************************

    /// Find the bucket the given value should be placed in.
    /// Returns `None` if the corresponding index cannot be represented in `usize`.
    fn index_for(&self, value: u64) -> Option<usize> {
        let bucket_index = self.bucket_for(value);
        let sub_bucket_index = self.sub_bucket_for(value, bucket_index);

        debug_assert!(sub_bucket_index < self.sub_bucket_count);
        debug_assert!(bucket_index == 0 || (sub_bucket_index >= self.sub_bucket_half_count));

        // Calculate the index for the first entry that will be used in the bucket (halfway through
        // sub_bucket_count). For bucket_index 0, all sub_bucket_count entries may be used, but
        // bucket_base_index is still set in the middle.
        let bucket_base_index =
            (i32::from(bucket_index) + 1) << self.sub_bucket_half_count_magnitude;

        // Calculate the offset in the bucket. This subtraction will result in a positive value in
        // all buckets except the 0th bucket (since a value in that bucket may be less than half
        // the bucket's 0 to sub_bucket_count range). However, this works out since we give bucket 0
        // twice as much space.
        let offset_in_bucket = sub_bucket_index as i32 - self.sub_bucket_half_count as i32;

        let index = bucket_base_index + offset_in_bucket;
        // This is always non-negative because offset_in_bucket is only negative (and only then by
        // sub_bucket_half_count at most) for bucket 0, and bucket_base_index will be halfway into
        // bucket 0's sub buckets in that case.
        debug_assert!(index >= 0);
        index.to_usize()
    }

    /// Find the bucket the given value should be placed in.
    /// If the value is bigger than what this histogram can express, the last valid bucket index
    /// is returned instead.
    fn index_for_or_last(&self, value: u64) -> usize {
        self.index_for(value)
            .map_or(self.last_index(), |i| cmp::min(i, self.last_index()))
    }

    /// Get a mutable reference to the count bucket for the given value, if it is in range.
    fn mut_at(&mut self, value: u64) -> Option<&mut T> {
        self.index_for(value)
            .and_then(move |i| self.counts.get_mut(i))
    }

    /// Get the index of the last histogram bin.
    fn last_index(&self) -> usize {
        self.distinct_values()
            .checked_sub(1)
            .expect("Empty counts array?")
    }

    // ********************************************************************************************
    // Histograms should be cloneable.
    // ********************************************************************************************

    /// Get a copy of this histogram, corrected for coordinated omission.
    ///
    /// To compensate for the loss of sampled values when a recorded value is larger than the
    /// expected interval between value samples, the new histogram will include an auto-generated
    /// additional series of decreasingly-smaller (down to the `interval`) value records for each
    /// count found in the current histogram that is larger than the `interval`.
    ///
    /// Note: This is a post-correction method, as opposed to the at-recording correction method
    /// provided by `record_correct`. The two methods are mutually exclusive, and only one of the
    /// two should be be used on a given data set to correct for the same coordinated omission
    /// issue.
    ///
    /// See notes in the description of the Histogram calls for an illustration of why this
    /// corrective behavior is important.
    ///
    /// If `interval` is larger than 0, add auto-generated value records as appropriate if value is
    /// larger than `interval`.
    pub fn clone_correct(&self, interval: u64) -> Histogram<T> {
        let mut h = Histogram::new_from(self);
        for v in self.iter_recorded() {
            h.record_n_correct(v.value_iterated_to(), v.count_at_value(), interval)
                .expect("Same dimensions; all values should be representable");
        }
        h
    }

    /// Overwrite this histogram with the given histogram. All data and statistics in this
    /// histogram will be overwritten.
    pub fn set_to<B: Borrow<Histogram<T>>>(&mut self, source: B) -> Result<(), AdditionError> {
        self.reset();
        self.add(source.borrow())
    }

    /// Overwrite this histogram with the given histogram while correcting for coordinated
    /// omission. All data and statistics in this histogram will be overwritten. See
    /// `clone_correct` for more detailed explanation about how correction is applied
    pub fn set_to_corrected<B: Borrow<Histogram<T>>>(
        &mut self,
        source: B,
        interval: u64,
    ) -> Result<(), RecordError> {
        self.reset();
        self.add_correct(source, interval)
    }

    // ********************************************************************************************
    // Add and subtract methods for, well, adding or subtracting two histograms
    // ********************************************************************************************

    /// Add the contents of another histogram to this one.
    ///
    /// Returns an error if values in the other histogram cannot be stored; see `AdditionError`.
    pub fn add<B: Borrow<Histogram<T>>>(&mut self, source: B) -> Result<(), AdditionError> {
        let source = source.borrow();

        // If source is empty there's nothing to add
        if source.is_empty() {
            return Ok(());
        }

        // make sure we can take the values in source
        let top = self.highest_equivalent(self.value_for(self.last_index()));
        if top < source.max() {
            if !self.auto_resize {
                return Err(AdditionError::OtherAddendValueExceedsRange);
            }
            // We're growing the histogram, so new high > old high and is therefore >= 2x low.
            self.resize(source.max())
                .map_err(|_| AdditionError::ResizeFailedUsizeTypeTooSmall)?;
        }

        let matching_buckets = self.bucket_count == source.bucket_count
            && self.sub_bucket_count == source.sub_bucket_count
            && self.unit_magnitude == source.unit_magnitude;
        if matching_buckets && self.is_empty() {
            // Counts arrays are of the same length and meaning.
            // If self is empty (all counters are zeroes) we can copy the source histogram with a memory copy.
            self.counts[..].copy_from_slice(&source.counts[..]);
            self.total_count = source.total_count;
            self.min_non_zero_value = source.min_non_zero_value;
            self.max_value = source.max_value;
        } else if matching_buckets {
            // Counts arrays are of the same length and meaning,
            // so we can just iterate and add directly:
            let mut observed_other_total_count: u64 = 0;
            for i in 0..source.distinct_values() {
                let other_count = source
                    .count_at_index(i)
                    .expect("iterating inside source length");
                if other_count != T::zero() {
                    // indexing is safe: same configuration as `source`, and the index was valid for
                    // `source`.
                    self.counts[i] = self.counts[i].saturating_add(other_count);
                    observed_other_total_count =
                        observed_other_total_count.saturating_add(other_count.as_u64());
                }
            }

            self.total_count = self.total_count.saturating_add(observed_other_total_count);
            let mx = source.max();
            if mx > self.max() {
                self.update_max(mx);
            }
            let mn = source.min_nz();
            if mn < self.min_nz() {
                self.update_min(mn);
            }
        } else {
            // Arrays are not a direct match (or the other could change on the fly in some valid
            // way), so we can't just stream through and add them. Instead, go through the array
            // and add each non-zero value found at it's proper value:

            // Do max value first, to avoid max value updates on each iteration:
            let other_max_index = source
                .index_for(source.max())
                .expect("Index for max value must exist");
            let other_count = source
                .count_at_index(other_max_index)
                .expect("max's index must exist");
            self.record_n(source.value_for(other_max_index), other_count)
                .expect("Record must succeed; already resized for max value");

            // Record the remaining values, up to but not including the max value:
            for i in 0..other_max_index {
                let other_count = source
                    .count_at_index(i)
                    .expect("index before max must exist");
                if other_count != T::zero() {
                    self.record_n(source.value_for(i), other_count)
                        .expect("Record must succeed; already recorded max value");
                }
            }
        }

        // TODO:
        // if source.start_time < self.start_time {
        //     self.start_time = source.start_time;
        // }
        // if source.end_time > self.end_time {
        //     self.end_time = source.end_time;
        // }
        Ok(())
    }

    /// Add the contents of another histogram to this one, while correcting for coordinated
    /// omission.
    ///
    /// To compensate for the loss of sampled values when a recorded value is larger than the
    /// expected interval between value samples, the values added will include an auto-generated
    /// additional series of decreasingly-smaller (down to the given `interval`) value records for
    /// each count found in the current histogram that is larger than `interval`.
    ///
    /// Note: This is a post-recording correction method, as opposed to the at-recording correction
    /// method provided by `record_correct`. The two methods are mutually exclusive, and only one
    /// of the two should be be used on a given data set to correct for the same coordinated
    /// omission issue.
    ///
    /// See notes in the description of the `Histogram` calls for an illustration of why this
    /// corrective behavior is important.
    ///
    /// See `RecordError` for error conditions.
    pub fn add_correct<B: Borrow<Histogram<T>>>(
        &mut self,
        source: B,
        interval: u64,
    ) -> Result<(), RecordError> {
        let source = source.borrow();

        for v in source.iter_recorded() {
            self.record_n_correct(v.value_iterated_to(), v.count_at_value(), interval)?;
        }
        Ok(())
    }

    /// Subtract the contents of another histogram from this one.
    ///
    /// See `SubtractionError` for error conditions.
    pub fn subtract<B: Borrow<Histogram<T>>>(
        &mut self,
        subtrahend: B,
    ) -> Result<(), SubtractionError> {
        let subtrahend = subtrahend.borrow();

        // If the source is empty there's nothing to subtract
        if subtrahend.is_empty() {
            return Ok(());
        }

        // make sure we can take the values in source
        let top = self.highest_equivalent(self.value_for(self.last_index()));
        if top < self.highest_equivalent(subtrahend.max()) {
            return Err(SubtractionError::SubtrahendValueExceedsMinuendRange);
        }

        let old_min_highest_equiv = self.highest_equivalent(self.min());
        let old_max_lowest_equiv = self.lowest_equivalent(self.max());

        // If total_count is at the max value, it may have saturated, so we must restat
        let mut needs_restat = self.total_count == u64::max_value();

        for i in 0..subtrahend.distinct_values() {
            let other_count = subtrahend
                .count_at_index(i)
                .expect("index inside subtrahend len must exist");
            if other_count != T::zero() {
                let other_value = subtrahend.value_for(i);
                {
                    let mut_count = self.mut_at(other_value);

                    if let Some(c) = mut_count {
                        // TODO Perhaps we should saturating sub here? Or expose some form of
                        // pluggability so users could choose to error or saturate? Both seem
                        // useful. It's also sort of inconsistent with overflow, which now
                        // saturates.
                        *c = (*c)
                            .checked_sub(&other_count)
                            .ok_or(SubtractionError::SubtrahendCountExceedsMinuendCount)?;
                    } else {
                        panic!("Tried to subtract value outside of range: {}", other_value);
                    }
                }

                // we might have just set the min / max to have zero count.
                if other_value <= old_min_highest_equiv || other_value >= old_max_lowest_equiv {
                    needs_restat = true;
                }

                if !needs_restat {
                    // if we're not already going to recalculate everything, subtract from
                    // total_count
                    self.total_count = self
                        .total_count
                        .checked_sub(other_count.as_u64())
                        .expect("total count underflow on subtraction");
                }
            }
        }

        if needs_restat {
            let l = self.distinct_values();
            self.restat(l);
        }

        Ok(())
    }

    // ********************************************************************************************
    // Setters and resetters.
    // ********************************************************************************************

    /// Clear the contents of this histogram while preserving its statistics and configuration.
    pub fn clear(&mut self) {
        for c in &mut self.counts {
            *c = T::zero();
        }
        self.total_count = 0;
    }

    /// Reset the contents and statistics of this histogram, preserving only its configuration.
    pub fn reset(&mut self) {
        self.clear();

        self.reset_max(ORIGINAL_MAX);
        self.reset_min(ORIGINAL_MIN);
        // self.normalizing_index_offset = 0;
        // self.start_time = time::Instant::now();
        // self.end_time = time::Instant::now();
        // self.tag = String::new();
    }

    /// Control whether or not the histogram can auto-resize and auto-adjust it's highest trackable
    /// value as high-valued samples are recorded.
    pub fn auto(&mut self, enabled: bool) {
        self.auto_resize = enabled;
    }

    // ********************************************************************************************
    // Construction.
    // ********************************************************************************************

    /// Construct an auto-resizing `Histogram` with a lowest discernible value of 1 and an
    /// auto-adjusting highest trackable value. Can auto-resize up to track values up to
    /// `(i64::max_value() / 2)`.
    ///
    /// See [`new_with_bounds`] for info on `sigfig`.
    ///
    /// [`new_with_bounds`]: #method.new_with_bounds
    pub fn new(sigfig: u8) -> Result<Histogram<T>, CreationError> {
        let mut h = Self::new_with_bounds(1, 2, sigfig);
        if let Ok(ref mut h) = h {
            h.auto_resize = true;
        }
        h
    }

    /// Construct a `Histogram` given a known maximum value to be tracked, and a number of
    /// significant decimal digits. The histogram will be constructed to implicitly track
    /// (distinguish from 0) values as low as 1. Auto-resizing will be disabled.
    ///
    /// See [`new_with_bounds`] for info on `high` and `sigfig`.
    ///
    /// [`new_with_bounds`]: #method.new_with_bounds
    pub fn new_with_max(high: u64, sigfig: u8) -> Result<Histogram<T>, CreationError> {
        Self::new_with_bounds(1, high, sigfig)
    }

    /// Construct a `Histogram` with known upper and lower bounds for recorded sample values.
    ///
    /// `low` is the lowest value that can be discerned (distinguished from 0) by the histogram,
    /// and must be a positive integer that is >= 1. It may be internally rounded down to nearest
    /// power of 2. Providing a lowest discernible value (`low`) is useful is situations where the
    /// units used for the histogram's values are much smaller that the minimal accuracy required.
    /// E.g. when tracking time values stated in nanosecond units, where the minimal accuracy
    /// required is a microsecond, the proper value for `low` would be 1000. If you're not sure,
    /// use 1.
    ///
    /// `high` is the highest value to be tracked by the histogram, and must be a
    /// positive integer that is `>= (2 * low)`. If you're not sure, use `u64::max_value()`.
    ///
    /// `sigfig` Specifies the number of significant figures to maintain. This is the number of
    /// significant decimal digits to which the histogram will maintain value resolution and
    /// separation. Must be in the range [0, 5]. If you're not sure, use 3. As `sigfig` increases,
    /// memory usage grows exponentially, so choose carefully if there will be many histograms in
    /// memory at once or if storage is otherwise a concern.
    ///
    /// Returns an error if the provided parameters are invalid; see `CreationError`.
    pub fn new_with_bounds(low: u64, high: u64, sigfig: u8) -> Result<Histogram<T>, CreationError> {
        // Verify argument validity
        if low < 1 {
            return Err(CreationError::LowIsZero);
        }
        if low > u64::max_value() / 2 {
            // avoid overflow in 2 * low
            return Err(CreationError::LowExceedsMax);
        }
        if high < 2 * low {
            return Err(CreationError::HighLessThanTwiceLow);
        }
        if sigfig > 5 {
            return Err(CreationError::SigFigExceedsMax);
        }

        // Given a 3 decimal point accuracy, the expectation is obviously for "+/- 1 unit at 1000".
        // It also means that it's "ok to be +/- 2 units at 2000". The "tricky" thing is that it is
        // NOT ok to be +/- 2 units at 1999. Only starting at 2000. So internally, we need to
        // maintain single unit resolution to 2x 10^decimal_points.

        // largest value with single unit resolution, in [2, 200_000].
        let largest = 2 * 10_u32.pow(u32::from(sigfig));

        let unit_magnitude = (low as f64).log2().floor() as u8;
        let unit_magnitude_mask = (1 << unit_magnitude) - 1;

        // We need to maintain power-of-two sub_bucket_count (for clean direct indexing) that is
        // large enough to provide unit resolution to at least
        // largest_value_with_single_unit_resolution. So figure out
        // largest_value_with_single_unit_resolution's nearest power-of-two (rounded up), and use
        // that.
        // In [1, 18]. 2^18 > 2 * 10^5 (the largest possible
        // largest_value_with_single_unit_resolution)
        let sub_bucket_count_magnitude = (f64::from(largest)).log2().ceil() as u8;
        let sub_bucket_half_count_magnitude = sub_bucket_count_magnitude - 1;
        let sub_bucket_count = 1_u32 << u32::from(sub_bucket_count_magnitude);

        if unit_magnitude + sub_bucket_count_magnitude > 63 {
            // sub_bucket_count entries can't be represented, with unit_magnitude applied, in a
            // u64. Technically it still sort of works if their sum is 64: you can represent all
            // but the last number in the shifted sub_bucket_count. However, the utility of such a
            // histogram vs ones whose magnitude here fits in 63 bits is debatable, and it makes
            // it harder to work through the logic. Sums larger than 64 are totally broken as
            // leading_zero_count_base would go negative.
            return Err(CreationError::CannotRepresentSigFigBeyondLow);
        };

        let sub_bucket_half_count = sub_bucket_count / 2;
        // sub_bucket_count is always at least 2, so subtraction won't underflow
        let sub_bucket_mask = (u64::from(sub_bucket_count) - 1) << unit_magnitude;

        let mut h = Histogram {
            auto_resize: false,

            highest_trackable_value: high,
            lowest_discernible_value: low,
            significant_value_digits: sigfig,

            // set by resize() below
            bucket_count: 0,
            sub_bucket_count,

            // Establish leading_zero_count_base, used in bucket_index_of() fast path:
            // subtract the bits that would be used by the largest value in bucket 0.
            leading_zero_count_base: 64 - unit_magnitude - sub_bucket_count_magnitude,
            sub_bucket_half_count_magnitude,

            unit_magnitude,
            sub_bucket_half_count,

            sub_bucket_mask,

            unit_magnitude_mask,
            max_value: ORIGINAL_MAX,
            min_non_zero_value: ORIGINAL_MIN,

            total_count: 0,
            // set by alloc() below
            counts: Vec::new(),
        };

        // Already checked that high >= 2*low
        h.resize(high)
            .map_err(|_| CreationError::UsizeTypeTooSmall)?;
        Ok(h)
    }

    /// Construct a `Histogram` with the same range settings as a given source histogram,
    /// duplicating the source's start/end timestamps (but NOT its contents).
    pub fn new_from<F: Counter>(source: &Histogram<F>) -> Histogram<T> {
        let mut h = Self::new_with_bounds(
            source.lowest_discernible_value,
            source.highest_trackable_value,
            source.significant_value_digits,
        )
        .expect("Using another histogram's parameters failed");

        // h.start_time = source.start_time;
        // h.end_time = source.end_time;
        h.auto_resize = source.auto_resize;
        h.counts.resize(source.distinct_values(), T::zero());
        h
    }

    // ********************************************************************************************
    // Recording samples.
    // ********************************************************************************************

    /// Record `value` in the histogram.
    ///
    /// Returns an error if `value` exceeds the highest trackable value and auto-resize is
    /// disabled.
    pub fn record(&mut self, value: u64) -> Result<(), RecordError> {
        self.record_n(value, T::one())
    }

    /// Record `value` in the histogram, clamped to the range of the histogram.
    ///
    /// This method cannot fail, as any values that are too small or too large to be tracked will
    /// automatically be clamed to be in range. Be aware that this *will* hide extreme outliers
    /// from the resulting histogram without warning. Since the values are clamped, the histogram
    /// will also not be resized to accomodate the value, even if auto-resize is enabled.
    pub fn saturating_record(&mut self, value: u64) {
        self.saturating_record_n(value, T::one())
    }

    /// Record multiple samples for a value in the histogram, adding to the value's current count.
    ///
    /// `count` is the number of occurrences of this value to record.
    ///
    /// Returns an error if `value` cannot be recorded; see `RecordError`.
    pub fn record_n(&mut self, value: u64, count: T) -> Result<(), RecordError> {
        self.record_n_inner(value, count, false)
    }

    /// Record multiple samples for a value in the histogram, each one clamped to the histogram's
    /// range.
    ///
    /// `count` is the number of occurrences of this value to record.
    ///
    /// This method cannot fail, as values that are too small or too large to be recorded will
    /// automatically be clamed to be in range. Be aware that this *will* hide extreme outliers
    /// from the resulting histogram without warning. Since the values are clamped, the histogram
    /// will also not be resized to accomodate the value, even if auto-resize is enabled.
    pub fn saturating_record_n(&mut self, value: u64, count: T) {
        self.record_n_inner(value, count, true).unwrap()
    }

    fn record_n_inner(&mut self, mut value: u64, count: T, clamp: bool) -> Result<(), RecordError> {
        let recorded_without_resize = if let Some(c) = self.mut_at(value) {
            *c = (*c).saturating_add(count);
            true
        } else {
            false
        };

        if !recorded_without_resize {
            if clamp {
                value = if value > self.highest_trackable_value {
                    self.highest_trackable_value
                } else {
                    // must be smaller than the lowest_discernible_value, since self.mut_at(value)
                    // failed, and it's not too large (per above).
                    self.lowest_discernible_value
                };

                let c = self
                    .mut_at(value)
                    .expect("unwrap must succeed since low and high are always representable");
                *c = c.saturating_add(count);
            } else if !self.auto_resize {
                return Err(RecordError::ValueOutOfRangeResizeDisabled);
            } else {
                // We're growing the histogram, so new high > old high and is therefore >= 2x low.
                self.resize(value)
                    .map_err(|_| RecordError::ResizeFailedUsizeTypeTooSmall)?;
                self.highest_trackable_value =
                    self.highest_equivalent(self.value_for(self.last_index()));

                {
                    let c = self.mut_at(value).expect("value should fit after resize");
                    // after resize, should be no possibility of overflow because this is a new slot
                    *c = (*c)
                        .checked_add(&count)
                        .expect("count overflow after resize");
                }
            }
        }

        self.update_min_max(value);
        self.total_count = self.total_count.saturating_add(count.as_u64());
        Ok(())
    }

    /// Record a value in the histogram while correcting for coordinated omission.
    ///
    /// See `record_n_correct` for further documentation.
    pub fn record_correct(&mut self, value: u64, interval: u64) -> Result<(), RecordError> {
        self.record_n_correct(value, T::one(), interval)
    }

    /// Record multiple values in the histogram while correcting for coordinated omission.
    ///
    /// To compensate for the loss of sampled values when a recorded value is larger than the
    /// expected interval between value samples, this method will auto-generate and record an
    /// additional series of decreasingly-smaller (down to `interval`) value records.
    ///
    /// Note: This is a at-recording correction method, as opposed to the post-recording correction
    /// method provided by `correct_clone`. The two methods are mutually exclusive, and only one of
    /// the two should be be used on a given data set to correct for the same coordinated omission
    /// issue.
    ///
    /// Returns an error if `value` exceeds the highest trackable value and auto-resize is
    /// disabled.
    pub fn record_n_correct(
        &mut self,
        value: u64,
        count: T,
        interval: u64,
    ) -> Result<(), RecordError> {
        self.record_n(value, count)?;
        if interval == 0 {
            return Ok(());
        }

        if value > interval {
            // only enter loop when calculations will stay non-negative
            let mut missing_value = value - interval;
            while missing_value >= interval {
                self.record_n_inner(missing_value, count, false)?;
                missing_value -= interval;
            }
        }

        Ok(())
    }

    // ********************************************************************************************
    // Iterators
    // ********************************************************************************************

    /// Iterate through histogram values by quantile levels.
    ///
    /// The iteration mechanic for this iterator may appear somewhat confusing, but it yields
    /// fairly pleasing output. The iterator starts with a *quantile step size* of
    /// `1/halving_period`. For every iteration, it yields a value whose quantile is that much
    /// greater than the previously emitted quantile (i.e., initially 0, 0.1, 0.2, etc.). Once
    /// `halving_period` values have been emitted, the quantile  step size is halved, and the
    /// iteration continues.
    ///
    /// `ticks_per_half_distance` must be at least 1.
    ///
    /// The iterator yields an `iterators::IterationValue` struct.
    ///
    /// One subtlety of this iterator is that you can reach a value whose cumulative count yields
    /// a quantile of 1.0 far sooner than the quantile iteration would reach 1.0. Consider a
    /// histogram with count 1 at value 1, and count 1000000 at value 1000. At any quantile
    /// iteration above `1/1000001 = 0.000000999`, iteration will have necessarily proceeded to
    /// the index for value 1000, which has all the remaining counts, and therefore quantile (for
    /// the value) of 1.0. This is why `IterationValue` has both `quantile()` and
    /// `quantile_iterated_to()`. Additionally, to avoid a bunch of unhelpful iterations once
    /// iteration has reached the last value with non-zero count, quantile iteration will skip
    /// straight to 1.0 as well.
    ///
    /// ```
    /// use hdrhistogram::Histogram;
    /// use hdrhistogram::iterators::IterationValue;
    /// let mut hist = Histogram::<u64>::new_with_max(10000, 4).unwrap();
    /// for i in 0..10000 {
    ///     hist += i;
    /// }
    ///
    /// let mut perc = hist.iter_quantiles(1);
    ///
    /// println!("{:?}", hist.iter_quantiles(1).collect::<Vec<_>>());
    ///
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(hist.value_at_quantile(0.0001), 0.0001, 0.0, 1, 1))
    /// );
    /// // step size = 50
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(hist.value_at_quantile(0.5), 0.5, 0.5, 1, 5000 - 1))
    /// );
    /// // step size = 25
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(hist.value_at_quantile(0.75), 0.75, 0.75, 1, 2500))
    /// );
    /// // step size = 12.5
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(hist.value_at_quantile(0.875), 0.875, 0.875, 1, 1250))
    /// );
    /// // step size = 6.25
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(hist.value_at_quantile(0.9375), 0.9375, 0.9375, 1, 625))
    /// );
    /// // step size = 3.125
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(hist.value_at_quantile(0.9688), 0.9688, 0.96875, 1, 313))
    /// );
    /// // etc...
    /// ```
    pub fn iter_quantiles(
        &self,
        ticks_per_half_distance: u32,
    ) -> HistogramIterator<T, iterators::quantile::Iter<T>> {
        // TODO upper bound on ticks per half distance? 2^31 ticks is not useful
        iterators::quantile::Iter::new(self, ticks_per_half_distance)
    }

    /// Iterates through histogram values using linear value steps. The iteration is performed in
    /// steps of size `step`, each one yielding the count for all values in the preceeding value
    /// range of size `step`. The iterator terminates when all recorded histogram values are
    /// exhausted.
    ///
    /// The iterator yields an `iterators::IterationValue` struct.
    ///
    /// ```
    /// use hdrhistogram::Histogram;
    /// use hdrhistogram::iterators::IterationValue;
    /// let mut hist = Histogram::<u64>::new_with_max(1000, 3).unwrap();
    /// hist += 100;
    /// hist += 500;
    /// hist += 800;
    /// hist += 850;
    ///
    /// let mut perc = hist.iter_linear(100);
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(99, hist.quantile_below(99), hist.quantile_below(99), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(199, hist.quantile_below(199), hist.quantile_below(199), 0, 1))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(299, hist.quantile_below(299), hist.quantile_below(299), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(399, hist.quantile_below(399), hist.quantile_below(399), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(499, hist.quantile_below(499), hist.quantile_below(499), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(599, hist.quantile_below(599), hist.quantile_below(599), 0, 1))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(699, hist.quantile_below(699), hist.quantile_below(699), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(799, hist.quantile_below(799), hist.quantile_below(799), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(899, hist.quantile_below(899), hist.quantile_below(899), 0, 2))
    /// );
    /// assert_eq!(perc.next(), None);
    /// ```
    pub fn iter_linear(&self, step: u64) -> HistogramIterator<T, iterators::linear::Iter<T>> {
        iterators::linear::Iter::new(self, step)
    }

    /// Iterates through histogram values at logarithmically increasing levels. The iteration is
    /// performed in steps that start at `start` and increase exponentially according to `exp`. The
    /// iterator terminates when all recorded histogram values are exhausted.
    ///
    /// The iterator yields an `iterators::IterationValue` struct.
    ///
    /// ```
    /// use hdrhistogram::Histogram;
    /// use hdrhistogram::iterators::IterationValue;
    /// let mut hist = Histogram::<u64>::new_with_max(1000, 3).unwrap();
    /// hist += 100;
    /// hist += 500;
    /// hist += 800;
    /// hist += 850;
    ///
    /// let mut perc = hist.iter_log(1, 10.0);
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(0, hist.quantile_below(0), hist.quantile_below(0), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(9, hist.quantile_below(9), hist.quantile_below(9), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(99, hist.quantile_below(99), hist.quantile_below(99), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(999, hist.quantile_below(999), hist.quantile_below(999), 0, 4))
    /// );
    /// assert_eq!(perc.next(), None);
    /// ```
    pub fn iter_log(&self, start: u64, exp: f64) -> HistogramIterator<T, iterators::log::Iter<T>> {
        iterators::log::Iter::new(self, start, exp)
    }

    /// Iterates through all recorded histogram values using the finest granularity steps supported
    /// by the underlying representation. The iteration steps through all non-zero recorded value
    /// counts, and terminates when all recorded histogram values are exhausted.
    ///
    /// The iterator yields an `iterators::IterationValue` struct.
    ///
    /// ```
    /// use hdrhistogram::Histogram;
    /// use hdrhistogram::iterators::IterationValue;
    /// let mut hist = Histogram::<u64>::new_with_max(1000, 3).unwrap();
    /// hist += 100;
    /// hist += 500;
    /// hist += 800;
    /// hist += 850;
    ///
    /// let mut perc = hist.iter_recorded();
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(100, hist.quantile_below(100), hist.quantile_below(100), 1, 1))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(500, hist.quantile_below(500), hist.quantile_below(500), 1, 1))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(800, hist.quantile_below(800), hist.quantile_below(800), 1, 1))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(850, hist.quantile_below(850), hist.quantile_below(850), 1, 1))
    /// );
    /// assert_eq!(perc.next(), None);
    /// ```
    pub fn iter_recorded(&self) -> HistogramIterator<T, iterators::recorded::Iter> {
        iterators::recorded::Iter::new(self)
    }

    /// Iterates through all histogram values using the finest granularity steps supported by the
    /// underlying representation. The iteration steps through all possible unit value levels,
    /// regardless of whether or not there were recorded values for that value level, and
    /// terminates when all recorded histogram values are exhausted.
    ///
    /// The iterator yields an `iterators::IterationValue` struct.
    ///
    /// ```
    /// use hdrhistogram::Histogram;
    /// use hdrhistogram::iterators::IterationValue;
    /// let mut hist = Histogram::<u64>::new_with_max(10, 1).unwrap();
    /// hist += 1;
    /// hist += 5;
    /// hist += 8;
    ///
    /// let mut perc = hist.iter_all();
    /// assert_eq!(perc.next(), Some(IterationValue::new(0, 0.0, 0.0, 0, 0)));
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(1, hist.quantile_below(1), hist.quantile_below(1), 1, 1))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(2, hist.quantile_below(2), hist.quantile_below(2), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(3, hist.quantile_below(3), hist.quantile_below(3), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(4, hist.quantile_below(4), hist.quantile_below(4), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(5, hist.quantile_below(5), hist.quantile_below(5), 1, 1))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(6, hist.quantile_below(6), hist.quantile_below(6), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(7, hist.quantile_below(7), hist.quantile_below(7), 0, 0))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(8, hist.quantile_below(8), hist.quantile_below(8), 1, 1))
    /// );
    /// assert_eq!(
    ///     perc.next(),
    ///     Some(IterationValue::new(9, hist.quantile_below(9), hist.quantile_below(9), 0, 0))
    /// );
    /// assert_eq!(perc.next(), Some(IterationValue::new(10, 1.0, 1.0, 0, 0)));
    /// ```
    pub fn iter_all(&self) -> HistogramIterator<T, iterators::all::Iter> {
        iterators::all::Iter::new(self)
    }

    // ********************************************************************************************
    // Data statistics
    // ********************************************************************************************

    /// Get the lowest recorded value level in the histogram.
    /// If the histogram has no recorded values, the value returned will be 0.
    pub fn min(&self) -> u64 {
        if self.total_count == 0
            || self
                .count_at_index(0)
                .expect("counts array must be non-empty")
                != T::zero()
        {
            0
        } else {
            self.min_nz()
        }
    }

    /// Get the highest recorded value level in the histogram.
    /// If the histogram has no recorded values, the value returned is undefined.
    pub fn max(&self) -> u64 {
        if self.max_value == ORIGINAL_MAX {
            ORIGINAL_MAX
        } else {
            self.highest_equivalent(self.max_value)
        }
    }

    /// Get the lowest recorded non-zero value level in the histogram.
    /// If the histogram has no recorded values, the value returned is `u64::max_value()`.
    pub fn min_nz(&self) -> u64 {
        if self.min_non_zero_value == ORIGINAL_MIN {
            ORIGINAL_MIN
        } else {
            self.lowest_equivalent(self.min_non_zero_value)
        }
    }

    /// Determine if two values are equivalent with the histogram's resolution. Equivalent here
    /// means that value samples recorded for any two equivalent values are counted in a common
    /// total count.
    pub fn equivalent(&self, value1: u64, value2: u64) -> bool {
        self.lowest_equivalent(value1) == self.lowest_equivalent(value2)
    }

    /// Get the computed mean value of all recorded values in the histogram.
    pub fn mean(&self) -> f64 {
        if self.total_count == 0 {
            return 0.0;
        }

        self.iter_recorded().fold(0.0_f64, |total, v| {
            // TODO overflow?
            total
                + self.median_equivalent(v.value_iterated_to()) as f64 * v.count_at_value().as_f64()
                    / self.total_count as f64
        })
    }

    /// Get the computed standard deviation of all recorded values in the histogram
    pub fn stdev(&self) -> f64 {
        if self.total_count == 0 {
            return 0.0;
        }

        let mean = self.mean();
        let geom_dev_tot = self.iter_recorded().fold(0.0_f64, |gdt, v| {
            let dev = self.median_equivalent(v.value_iterated_to()) as f64 - mean;
            gdt + (dev * dev) * v.count_since_last_iteration() as f64
        });

        (geom_dev_tot / self.total_count as f64).sqrt()
    }

    /// Get the value at a given percentile.
    ///
    /// This is simply `value_at_quantile` multiplied by 100.0. For best floating-point precision,
    /// use `value_at_quantile` directly.
    pub fn value_at_percentile(&self, percentile: f64) -> u64 {
        self.value_at_quantile(percentile / 100.0)
    }

    /// Get the value at a given quantile.
    ///
    /// When the given quantile is > 0.0, the value returned is the value that the given
    /// percentage of the overall recorded value entries in the histogram are either smaller than
    /// or equivalent to. When the given quantile is 0.0, the value returned is the value that
    /// all value entries in the histogram are either larger than or equivalent to.
    ///
    /// Two values are considered "equivalent" if `self.equivalent` would return true.
    ///
    /// If the total count of the histogram has exceeded `u64::max_value()`, this will return
    /// inaccurate results.
    pub fn value_at_quantile(&self, quantile: f64) -> u64 {
        // Cap at 1.0
        let quantile = if quantile > 1.0 { 1.0 } else { quantile };

        let fractional_count = quantile * self.total_count as f64;
        // If we're part-way into the next highest int, we should use that as the count
        let mut count_at_quantile = fractional_count.ceil() as u64;

        // Make sure we at least reach the first recorded entry
        if count_at_quantile == 0 {
            count_at_quantile = 1;
        }

        let mut total_to_current_index: u64 = 0;
        for i in 0..self.counts.len() {
            // Direct indexing is safe; indexes must reside in counts array.
            // TODO overflow
            total_to_current_index += self.counts[i].as_u64();
            if total_to_current_index >= count_at_quantile {
                let value_at_index = self.value_for(i);
                return if quantile == 0.0 {
                    self.lowest_equivalent(value_at_index)
                } else {
                    self.highest_equivalent(value_at_index)
                };
            }
        }

        0
    }

    /// Get the percentile of samples at and below a given value.
    ///
    /// This is simply `quantile_below* multiplied by 100.0. For best floating-point precision, use
    /// `quantile_below` directly.
    pub fn percentile_below(&self, value: u64) -> f64 {
        self.quantile_below(value) * 100.0
    }

    /// Get the quantile of samples at or below a given value.
    ///
    /// The value returned is the quantile of values recorded in the histogram that are
    /// smaller than or equivalent to the given value.
    ///
    /// Two values are considered "equivalent" if `self.equivalent` would return true.
    ///
    /// If the value is larger than the maximum representable value, it will be clamped to the
    /// max representable value.
    ///
    /// If the total count of the histogram has reached `u64::max_value()`, this will return
    /// inaccurate results.
    pub fn quantile_below(&self, value: u64) -> f64 {
        if self.total_count == 0 {
            return 1.0;
        }

        let target_index = self.index_for_or_last(value);
        // TODO use RangeInclusive when it's stable to avoid checked_add
        let end = target_index.checked_add(1).expect("usize overflow");
        // count the smaller half
        let (slice, lower) = if target_index < self.counts.len() / 2 {
            (&self.counts[0..end], true)
        } else {
            (&self.counts[end..], false)
        };
        let iter = slice.iter().map(Counter::as_u64);

        let total_to_current_index = if self.total_count < u64::MAX {
            // if the total didn't saturate then any partial count shouldn't either.
            // iter::sum optimizes better than the saturating_add fallback below
            iter.sum::<u64>()
        } else {
            iter.fold(0u64, u64::saturating_add)
        };
        let total_to_current_index = if lower {
            total_to_current_index
        } else {
            self.total_count - total_to_current_index
        };
        total_to_current_index.as_f64() / self.total_count as f64
    }

    /// Get the count of recorded values within a range of value levels (inclusive to within the
    /// histogram's resolution).
    ///
    /// `low` gives the lower value bound on the range for which to provide the recorded count.
    /// Will be rounded down with `lowest_equivalent`. Similarly, `high` gives the higher value
    /// bound on the range, and will be rounded up with `highest_equivalent`. The function returns
    /// the total count of values recorded in the histogram within the value range that is `>=
    /// lowest_equivalent(low)` and `<= highest_equivalent(high)`.
    ///
    /// If either value is larger than the maximum representable value, it will be clamped to the
    /// max representable value.
    ///
    /// The count will saturate at u64::max_value().
    pub fn count_between(&self, low: u64, high: u64) -> u64 {
        let low_index = self.index_for_or_last(low);
        let high_index = self.index_for_or_last(high);
        // TODO use RangeInclusive when it's stable to avoid checked_add
        (low_index..high_index.checked_add(1).expect("usize overflow"))
            .map(|i| self.count_at_index(i).expect("index is <= last_index()"))
            .fold(0_u64, |t, v| t.saturating_add(v.as_u64()))
    }

    /// Get the count of recorded values at a specific value (to within the histogram resolution at
    /// the value level).
    ///
    /// The count is computed across values recorded in the histogram that are within the value
    /// range that is `>= lowest_equivalent(value)` and `<= highest_equivalent(value)`.
    ///
    /// If the value is larger than the maximum representable value, it will be clamped to the
    /// max representable value.
    pub fn count_at(&self, value: u64) -> T {
        self.count_at_index(self.index_for_or_last(value))
            .expect("index is <= last_index()")
    }

    // ********************************************************************************************
    // Public helpers
    // ********************************************************************************************

    /// Get the lowest value that is equivalent to the given value within the histogram's
    /// resolution. Equivalent here means that value samples recorded for any two equivalent values
    /// are counted in a common total count.
    pub fn lowest_equivalent(&self, value: u64) -> u64 {
        let bucket_index = self.bucket_for(value);
        let sub_bucket_index = self.sub_bucket_for(value, bucket_index);
        self.value_from_loc(bucket_index, sub_bucket_index)
    }

    /// Get the highest value that is equivalent to the given value within the histogram's
    /// resolution. Equivalent here means that value samples recorded for any two equivalent values
    /// are counted in a common total count.
    ///
    /// Note that the return value is capped at `u64::max_value()`.
    pub fn highest_equivalent(&self, value: u64) -> u64 {
        if value == u64::max_value() {
            u64::max_value()
        } else {
            self.next_non_equivalent(value) - 1
        }
    }

    /// Get a value that lies in the middle (rounded up) of the range of values equivalent the
    /// given value. Equivalent here means that value samples recorded for any two equivalent
    /// values are counted in a common total count.
    ///
    /// Note that the return value is capped at `u64::max_value()`.
    pub fn median_equivalent(&self, value: u64) -> u64 {
        // adding half of the range to the bottom of the range shouldn't overflow
        self.lowest_equivalent(value)
            .checked_add(self.equivalent_range(value) >> 1)
            .expect("median equivalent should not overflow")
    }

    /// Get the next value that is *not* equivalent to the given value within the histogram's
    /// resolution. Equivalent means that value samples recorded for any two equivalent values are
    /// counted in a common total count.
    ///
    /// Note that the return value is capped at `u64::max_value()`.
    pub fn next_non_equivalent(&self, value: u64) -> u64 {
        self.lowest_equivalent(value)
            .saturating_add(self.equivalent_range(value))
    }

    /// Get the size (in value units) of the range of values that are equivalent to the given value
    /// within the histogram's resolution. Equivalent here means that value samples recorded for
    /// any two equivalent values are counted in a common total count.
    pub fn equivalent_range(&self, value: u64) -> u64 {
        let bucket_index = self.bucket_for(value);
        1_u64 << (self.unit_magnitude + bucket_index)
    }

    /// Turn this histogram into a [`SyncHistogram`].
    #[cfg(feature = "sync")]
    pub fn into_sync(self) -> SyncHistogram<T> {
        SyncHistogram::from(self)
    }

    // ********************************************************************************************
    // Internal helpers
    // ********************************************************************************************

    /// Computes the matching histogram value for the given histogram bin.
    ///
    /// `index` must be no larger than `u32::max_value()`; no possible histogram uses that much
    /// storage anyway. So, any index that comes from a valid histogram location will be safe.
    ///
    /// If the index is for a position beyond what this histogram is configured for, the correct
    /// corresponding value will be returned, but of course it won't have a corresponding count.
    ///
    /// If the index maps to a value beyond `u64::max_value()`, the result will be garbage.
    fn value_for(&self, index: usize) -> u64 {
        // Dividing by sub bucket half count will yield 1 in top half of first bucket, 2 in
        // in the top half (i.e., the only half that's used) of the 2nd bucket, etc, so subtract 1
        // to get 0-indexed bucket indexes. This will be -1 for the bottom half of the first bucket.
        let mut bucket_index = (index >> self.sub_bucket_half_count_magnitude) as isize - 1;

        // Calculate the remainder of dividing by sub_bucket_half_count, shifted into the top half
        // of the corresponding bucket. This will (temporarily) map indexes in the lower half of
        // first bucket into the top half.

        // The subtraction won't underflow because half count is always at least 1.
        // TODO precalculate sub_bucket_half_count mask if benchmarks show improvement
        let mut sub_bucket_index = ((index.to_u32().expect("index must fit in u32"))
            & (self.sub_bucket_half_count - 1))
            + self.sub_bucket_half_count;
        if bucket_index < 0 {
            // lower half of first bucket case; move sub bucket index back
            sub_bucket_index -= self.sub_bucket_half_count;
            bucket_index = 0;
        }
        self.value_from_loc(bucket_index as u8, sub_bucket_index)
    }

    /// Returns count at index, or None if out of bounds
    fn count_at_index(&self, index: usize) -> Option<T> {
        self.counts.get(index).cloned()
    }

    /// Returns an error if the index doesn't exist.
    #[cfg(feature = "serialization")]
    fn set_count_at_index(&mut self, index: usize, count: T) -> Result<(), ()> {
        let r = self.counts.get_mut(index).ok_or(())?;
        *r = count;
        Ok(())
    }

    /// Compute the lowest (and therefore highest precision) bucket index whose sub-buckets can
    /// represent the value.
    #[inline]
    fn bucket_for(&self, value: u64) -> u8 {
        // Calculates the number of powers of two by which the value is greater than the biggest
        // value that fits in bucket 0. This is the bucket index since each successive bucket can
        // hold a value 2x greater. The mask maps small values to bucket 0.
        // Will not underflow because sub_bucket_mask caps the leading zeros to no more than
        // leading_zero_count_base.
        self.leading_zero_count_base - (value | self.sub_bucket_mask).leading_zeros() as u8
    }

    /// Compute the position inside a bucket at which the given value should be recorded, indexed
    /// from position 0 in the bucket (in the first half, which is not used past the first bucket).
    /// For bucket_index > 0, the result will be in the top half of the bucket.
    #[inline]
    fn sub_bucket_for(&self, value: u64, bucket_index: u8) -> u32 {
        // Since bucket_index is simply how many powers of 2 greater value is than what will fit in
        // bucket 0 (that is, what will fit in [0, sub_bucket_count)), we shift off that many
        // powers of two, and end up with a number in [0, sub_bucket_count).
        // For bucket_index 0, this is just value. For bucket index k > 0, we know value won't fit
        // in bucket (k - 1) by definition, so this calculation won't end up in the lower half of
        // [0, sub_bucket_count) because that would mean it would also fit in bucket (k - 1).
        // As unit magnitude grows, the maximum possible bucket index should shrink because it is
        // based off of sub_bucket_mask, so this shouldn't lead to an overlarge shift.
        (value >> (bucket_index + self.unit_magnitude)) as u32
    }

    /// Compute the value corresponding to the provided bucket and sub bucket indices.
    /// The indices given must map to an actual u64; providing contrived indices that would map to
    /// a value larger than u64::max_value() will yield garbage.
    #[inline]
    fn value_from_loc(&self, bucket_index: u8, sub_bucket_index: u32) -> u64 {
        // Sum won't overflow; bucket_index and unit_magnitude are both <= 64.
        // However, the resulting shift may overflow given bogus input, e.g. if unit magnitude is
        // large and the input sub_bucket_index is for an entry in the counts index that shouldn't
        // be used (because this calculation will overflow).
        u64::from(sub_bucket_index) << (bucket_index + self.unit_magnitude)
    }

    /// Find the number of buckets needed such that `value` is representable.
    fn buckets_to_cover(&self, value: u64) -> u8 {
        // Shift won't overflow because sub_bucket_magnitude + unit_magnitude <= 63.
        // the k'th bucket can express from 0 * 2^k to sub_bucket_count * 2^k in units of 2^k
        let mut smallest_untrackable_value =
            u64::from(self.sub_bucket_count) << self.unit_magnitude;

        // always have at least 1 bucket
        let mut buckets_needed = 1;
        while smallest_untrackable_value <= value {
            if smallest_untrackable_value > u64::max_value() / 2 {
                // next shift will overflow, meaning that bucket could represent values up to ones
                // greater than i64::max_value, so it's the last bucket
                return buckets_needed + 1;
            }
            smallest_untrackable_value <<= 1;
            buckets_needed += 1;
        }
        buckets_needed
    }

    /// Compute the actual number of bins to use for the given bucket count (that is, including the
    /// sub-buckets within each top-level bucket).
    ///
    /// If we have `N` such that `sub_bucket_count * 2^N > high`, we need storage for `N+1` buckets,
    /// each with enough slots to hold the top half of the `sub_bucket_count` (the lower half is
    /// covered by previous buckets), and the +1 being used for the lower half of the 0'th bucket.
    /// Or, equivalently, we need 1 more bucket to capture the max value if we consider the
    /// sub-bucket length to be halved.
    fn num_bins(&self, number_of_buckets: u8) -> u32 {
        (u32::from(number_of_buckets) + 1) * (self.sub_bucket_half_count)
    }

    /// Resize the underlying counts array such that it can cover the given `high` value.
    ///
    /// `high` must be at least 2x the lowest discernible value.
    ///
    /// Returns an error if the new size cannot be represented as a `usize`.
    fn resize(&mut self, high: u64) -> Result<(), UsizeTypeTooSmall> {
        // will not overflow because lowest_discernible_value must be at least as small as
        // u64::max_value() / 2 to have passed initial validation
        assert!(
            high >= 2 * self.lowest_discernible_value,
            "highest trackable value must be >= (2 * lowest discernible value)"
        );

        // establish counts array length:
        let buckets_needed = self.buckets_to_cover(high);
        let len = self
            .num_bins(buckets_needed)
            .to_usize()
            .ok_or(UsizeTypeTooSmall)?;

        // establish exponent range needed to support the trackable value with no overflow:
        self.bucket_count = buckets_needed;

        // establish the new highest trackable value:
        self.highest_trackable_value = high;

        // expand counts to also hold the new counts
        self.counts.resize(len, T::zero());
        Ok(())
    }

    /// Set internally tracked max_value to new value if new value is greater than current one.
    fn update_max(&mut self, value: u64) {
        let internal_value = value | self.unit_magnitude_mask; // Max unit-equivalent value
        if internal_value > self.max_value {
            self.max_value = internal_value;
        }
    }

    /// Set internally tracked min_non_zero_value to new value if new value is smaller than current
    /// one.
    fn update_min(&mut self, value: u64) {
        if value <= self.unit_magnitude_mask {
            return; // Unit-equivalent to 0.
        }

        let internal_value = value & !self.unit_magnitude_mask; // Min unit-equivalent value
        if internal_value < self.min_non_zero_value {
            self.min_non_zero_value = internal_value;
        }
    }

    fn update_min_max(&mut self, value: u64) {
        if value > self.max_value {
            self.update_max(value);
        }
        if value < self.min_non_zero_value && value != 0 {
            self.update_min(value);
        }
    }

    fn reset_max(&mut self, max: u64) {
        self.max_value = max | self.unit_magnitude_mask; // Max unit-equivalent value
    }

    fn reset_min(&mut self, min: u64) {
        let internal_value = min & !self.unit_magnitude_mask; // Min unit-equivalent value
        self.min_non_zero_value = if min == u64::max_value() {
            min
        } else {
            internal_value
        };
    }

    /// Recalculate min, max, total_count.
    fn restat(&mut self, length_to_scan: usize) {
        self.reset_max(ORIGINAL_MAX);
        self.reset_min(ORIGINAL_MIN);

        let mut restat_state = RestatState::new();

        assert!(length_to_scan <= self.counts.len());
        for i in 0..length_to_scan {
            // Direct indexing safe because of assert above
            let count = self.counts[i];
            if count != T::zero() {
                restat_state.on_nonzero_count(i, count);
            }
        }

        restat_state.update_histogram(self);
    }
}

/// Stores the state to calculate the max, min, and total count for a histogram by iterating across
/// the counts.
struct RestatState<T: Counter> {
    max_index: Option<usize>,
    min_index: Option<usize>,
    total_count: u64,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Counter> RestatState<T> {
    fn new() -> RestatState<T> {
        RestatState {
            max_index: None,
            min_index: None,
            total_count: 0,
            phantom: std::marker::PhantomData,
        }
    }

    /// Should be called on every non-zero count found
    #[inline]
    fn on_nonzero_count(&mut self, index: usize, count: T) {
        self.total_count = self.total_count.saturating_add(count.as_u64());

        self.max_index = Some(index);

        if self.min_index.is_none() && index != 0 {
            self.min_index = Some(index);
        }
    }

    /// Write updated min, max, total_count into histogram.
    /// Called once all counts have been iterated across.
    fn update_histogram(self, h: &mut Histogram<T>) {
        if let Some(max_i) = self.max_index {
            let max = h.highest_equivalent(h.value_for(max_i));
            h.update_max(max);
        }
        if let Some(min_i) = self.min_index {
            let min = h.value_for(min_i);
            h.update_min(min);
        }

        h.total_count = self.total_count;
    }
}

// ********************************************************************************************
// Trait implementations
// ********************************************************************************************

// make it more ergonomic to add and subtract histograms
impl<'a, T: Counter> AddAssign<&'a Histogram<T>> for Histogram<T> {
    fn add_assign(&mut self, source: &'a Histogram<T>) {
        self.add(source).unwrap();
    }
}

impl<T: Counter> AddAssign<Histogram<T>> for Histogram<T> {
    fn add_assign(&mut self, source: Histogram<T>) {
        self.add(&source).unwrap();
    }
}

impl<T: Counter> Add<Histogram<T>> for Histogram<T> {
    type Output = Histogram<T>;
    fn add(mut self, rhs: Histogram<T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a, T: Counter> Add<&'a Histogram<T>> for Histogram<T> {
    type Output = Histogram<T>;
    fn add(mut self, rhs: &'a Histogram<T>) -> Self::Output {
        self += rhs;
        self
    }
}

use std::iter;
impl<T: Counter> iter::Sum for Histogram<T> {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        match iter.next() {
            Some(mut first) => {
                for h in iter {
                    first += h;
                }
                first
            }
            None => Histogram::new(3).expect("histograms with sigfig=3 should always work"),
        }
    }
}

impl<'a, T: Counter> SubAssign<&'a Histogram<T>> for Histogram<T> {
    fn sub_assign(&mut self, other: &'a Histogram<T>) {
        self.subtract(other).unwrap();
    }
}

impl<T: Counter> SubAssign<Histogram<T>> for Histogram<T> {
    fn sub_assign(&mut self, source: Histogram<T>) {
        self.subtract(&source).unwrap();
    }
}

impl<T: Counter> Sub<Histogram<T>> for Histogram<T> {
    type Output = Histogram<T>;
    fn sub(mut self, rhs: Histogram<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<'a, T: Counter> Sub<&'a Histogram<T>> for Histogram<T> {
    type Output = Histogram<T>;
    fn sub(mut self, rhs: &'a Histogram<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

// make it more ergonomic to record samples
impl<T: Counter> AddAssign<u64> for Histogram<T> {
    fn add_assign(&mut self, value: u64) {
        self.record(value).unwrap();
    }
}

// allow comparing histograms
impl<T: Counter, F: Counter> PartialEq<Histogram<F>> for Histogram<T>
where
    T: PartialEq<F>,
{
    fn eq(&self, other: &Histogram<F>) -> bool {
        if self.lowest_discernible_value != other.lowest_discernible_value
            || self.significant_value_digits != other.significant_value_digits
        {
            return false;
        }
        if self.total_count != other.total_count {
            return false;
        }
        if self.max() != other.max() {
            return false;
        }
        if self.min_nz() != other.min_nz() {
            return false;
        }

        (0..self.counts.len()).all(|i| {
            self.counts[i]
                == match other.count_at_index(i) {
                    Some(c) => c,
                    None => return false,
                }
        })
    }
}

// /**
//  * Indicate whether or not the histogram is capable of supporting auto-resize functionality.
//  * Note that this is an indication that enabling auto-resize by calling set_auto_resize() is
//  * allowed, and NOT that the histogram will actually auto-resize. Use is_auto_resize() to
//  * determine if the histogram is in auto-resize mode.
//  * @return auto_resize setting
//  */
// public boolean supports_auto_resize() { return true; }

// TODO: shift
// TODO: hash

#[path = "tests/tests.rs"]
#[cfg(test)]
mod tests;

mod core;
pub mod errors;
#[cfg(feature = "serialization")]
pub mod serialization;
pub use self::core::counter::*;
pub use errors::*;
#[cfg(feature = "sync")]
pub mod sync;
#[cfg(feature = "sync")]
pub use sync::SyncHistogram;
