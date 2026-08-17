// Copyright 2014 The Prometheus Authors
// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::From;
use std::sync::{
    atomic::{AtomicU64 as StdAtomicU64, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant as StdInstant};

use crate::atomic64::{Atomic, AtomicF64, AtomicU64};
use crate::desc::{Desc, Describer};
use crate::errors::{Error, Result};
use crate::metrics::{Collector, LocalMetric, Metric, Opts};
use crate::proto;
use crate::value::make_label_pairs;
use crate::vec::{MetricVec, MetricVecBuilder};

/// The default [`Histogram`] buckets. The default buckets are
/// tailored to broadly measure the response time (in seconds) of a
/// network service. Most likely, however, you will be required to define
/// buckets customized to your use case.
pub const DEFAULT_BUCKETS: &[f64; 11] = &[
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];

/// Used for the label that defines the upper bound of a
/// bucket of a histogram ("le" -> "less or equal").
pub const BUCKET_LABEL: &str = "le";

#[inline]
fn check_bucket_label(label: &str) -> Result<()> {
    if label == BUCKET_LABEL {
        return Err(Error::Msg(
            "`le` is not allowed as label name in histograms".to_owned(),
        ));
    }

    Ok(())
}

fn check_and_adjust_buckets(mut buckets: Vec<f64>) -> Result<Vec<f64>> {
    if buckets.is_empty() {
        buckets = Vec::from(DEFAULT_BUCKETS as &'static [f64]);
    }

    for (i, upper_bound) in buckets.iter().enumerate() {
        if i < (buckets.len() - 1) && *upper_bound >= buckets[i + 1] {
            return Err(Error::Msg(format!(
                "histogram buckets must be in increasing \
                 order: {} >= {}",
                upper_bound,
                buckets[i + 1]
            )));
        }
    }

    let tail = *buckets.last().unwrap();
    if tail.is_sign_positive() && tail.is_infinite() {
        // The +Inf bucket is implicit. Remove it here.
        buckets.pop();
    }

    Ok(buckets)
}

/// A struct that bundles the options for creating a [`Histogram`] metric. It is
/// mandatory to set Name and Help to a non-empty string. All other fields are
/// optional and can safely be left at their zero value.
#[derive(Clone, Debug)]
pub struct HistogramOpts {
    /// A container holding various options.
    pub common_opts: Opts,

    /// Defines the buckets into which observations are counted. Each
    /// element in the slice is the upper inclusive bound of a bucket. The
    /// values must be sorted in strictly increasing order. There is no need
    /// to add a highest bucket with +Inf bound, it will be added
    /// implicitly. The default value is DefBuckets.
    pub buckets: Vec<f64>,
}

impl HistogramOpts {
    /// Create a [`HistogramOpts`] with the `name` and `help` arguments.
    pub fn new<S1: Into<String>, S2: Into<String>>(name: S1, help: S2) -> HistogramOpts {
        HistogramOpts {
            common_opts: Opts::new(name, help),
            buckets: Vec::from(DEFAULT_BUCKETS as &'static [f64]),
        }
    }

    /// `namespace` sets the namespace.
    pub fn namespace<S: Into<String>>(mut self, namespace: S) -> Self {
        self.common_opts.namespace = namespace.into();
        self
    }

    /// `subsystem` sets the sub system.
    pub fn subsystem<S: Into<String>>(mut self, subsystem: S) -> Self {
        self.common_opts.subsystem = subsystem.into();
        self
    }

    /// `const_labels` sets the const labels.
    pub fn const_labels(mut self, const_labels: HashMap<String, String>) -> Self {
        self.common_opts = self.common_opts.const_labels(const_labels);
        self
    }

    /// `const_label` adds a const label.
    pub fn const_label<S1: Into<String>, S2: Into<String>>(mut self, name: S1, value: S2) -> Self {
        self.common_opts = self.common_opts.const_label(name, value);
        self
    }

    /// `variable_labels` sets the variable labels.
    pub fn variable_labels(mut self, variable_labels: Vec<String>) -> Self {
        self.common_opts = self.common_opts.variable_labels(variable_labels);
        self
    }

    /// `variable_label` adds a variable label.
    pub fn variable_label<S: Into<String>>(mut self, name: S) -> Self {
        self.common_opts = self.common_opts.variable_label(name);
        self
    }

    /// `fq_name` returns the fq_name.
    pub fn fq_name(&self) -> String {
        self.common_opts.fq_name()
    }

    /// `buckets` set the buckets.
    pub fn buckets(mut self, buckets: Vec<f64>) -> Self {
        self.buckets = buckets;
        self
    }
}

impl Describer for HistogramOpts {
    fn describe(&self) -> Result<Desc> {
        self.common_opts.describe()
    }
}

impl From<Opts> for HistogramOpts {
    fn from(opts: Opts) -> HistogramOpts {
        HistogramOpts {
            common_opts: opts,
            buckets: Vec::from(DEFAULT_BUCKETS as &'static [f64]),
        }
    }
}

/// Representation of a hot or cold shard.
///
/// See [`HistogramCore`] for details.
#[derive(Debug)]
struct Shard {
    sum: AtomicF64,
    count: AtomicU64,
    buckets: Vec<AtomicU64>,
}

impl Shard {
    fn new(num_buckets: usize) -> Self {
        let mut buckets = Vec::new();
        for _ in 0..num_buckets {
            buckets.push(AtomicU64::new(0));
        }

        Shard {
            sum: AtomicF64::new(0.0),
            count: AtomicU64::new(0),
            buckets,
        }
    }
}

/// Index into an array of [`Shard`]s.
///
/// Used in conjunction with [`ShardAndCount`] below.
#[derive(Debug, Clone, Copy)]
enum ShardIndex {
    /// First index. Corresponds to 0.
    First,
    /// Second index. Corresponds to 1.
    Second,
}

impl ShardIndex {
    /// Inverse the given [`ShardIndex`].
    fn inverse(self) -> ShardIndex {
        match self {
            ShardIndex::First => ShardIndex::Second,
            ShardIndex::Second => ShardIndex::First,
        }
    }
}

impl From<u64> for ShardIndex {
    fn from(index: u64) -> Self {
        match index {
            0 => ShardIndex::First,
            1 => ShardIndex::Second,
            _ => panic!(
                "Invalid shard index {:?}. A histogram only has two shards.",
                index
            ),
        }
    }
}

impl From<ShardIndex> for usize {
    fn from(index: ShardIndex) -> Self {
        match index {
            ShardIndex::First => 0,
            ShardIndex::Second => 1,
        }
    }
}

/// An atomic u64 with the most significant used as a [`ShardIndex`] and the
/// remaining 63 bits used to count [`Histogram`] observations.
#[derive(Debug)]
struct ShardAndCount {
    inner: StdAtomicU64,
}

impl ShardAndCount {
    /// Return a new [`ShardAndCount`] with both the most significant bit
    /// i.e. the `ShardIndex` and the remaining 63 bit i.e. the observation
    /// count set to 0.
    fn new() -> Self {
        ShardAndCount {
            inner: StdAtomicU64::new(0),
        }
    }

    /// Flip the most significant bit i.e. the [`ShardIndex`] leaving the
    /// remaining 63 bits unchanged.
    fn flip(&self, ordering: Ordering) -> (ShardIndex, u64) {
        let n = self.inner.fetch_add(1 << 63, ordering);

        ShardAndCount::split_shard_index_and_count(n)
    }

    /// Get the most significant bit i.e. the [`ShardIndex`] as well as the
    /// remaining 63 bits i.e. the observation count.
    fn get(&self) -> (ShardIndex, u64) {
        let n = self.inner.load(Ordering::Relaxed);

        ShardAndCount::split_shard_index_and_count(n)
    }

    /// Increment the observation count leaving the most significant bit i.e.
    /// the [`ShardIndex`] untouched.
    fn inc_by(&self, delta: u64, ordering: Ordering) -> (ShardIndex, u64) {
        let n = self.inner.fetch_add(delta, ordering);

        ShardAndCount::split_shard_index_and_count(n)
    }

    /// Increment the observation count by one leaving the most significant bit
    /// i.e. the [`ShardIndex`] untouched.
    fn inc(&self, ordering: Ordering) -> (ShardIndex, u64) {
        self.inc_by(1, ordering)
    }

    fn split_shard_index_and_count(n: u64) -> (ShardIndex, u64) {
        let shard = n >> 63;
        let count = n & ((1 << 63) - 1);

        (shard.into(), count)
    }
}

/// Core datastructure of a Prometheus histogram
///
/// # Atomicity across collects
///
/// A histogram supports two main execution paths:
///
/// 1. `observe` which increases the overall observation counter, updates the
///    observation sum and increases a single bucket counter.
///
/// 2. `proto` (aka. collecting the metric, from now on referred to as the
///    collect operation) which snapshots the state of the histogram and exposes
///    it as a Protobuf struct.
///
/// If an observe and a collect operation interleave, the latter could be
/// exposing a snapshot of the histogram that does not uphold all histogram
/// invariants. For example for the invariant that the overall observation
/// counter should equal the sum of all bucket counters: Say that an `observe`
/// increases the overall counter but before updating a specific bucket counter
/// a collect operation snapshots the histogram.
///
/// The below implementation of `HistogramCore` prevents such race conditions by
/// using two shards, one hot shard for `observe` operations to record their
/// observation and one cold shard for collect operations to collect a
/// consistent snapshot of the histogram.
///
/// `observe` operations hit the hot shard and record their observation. Collect
/// operations switch hot and cold, wait for all `observe` calls to finish on
/// the previously hot now cold shard and then expose the consistent snapshot.
#[derive(Debug)]
pub struct HistogramCore {
    desc: Desc,
    label_pairs: Vec<proto::LabelPair>,

    /// Mutual exclusion to serialize collect operations. No two collect
    /// operations should operate on this datastructure at the same time. (See
    /// struct documentation for details.) `observe` operations can operate in
    /// parallel without holding this lock.
    collect_lock: Mutex<()>,

    /// An atomic u64 where the first bit determines the currently hot shard and
    /// the remaining 63 bits determine the overall count.
    shard_and_count: ShardAndCount,
    /// The two shards where `shard_and_count` determines which one is the hot
    /// and which one the cold at any given point in time.
    shards: [Shard; 2],

    upper_bounds: Vec<f64>,
}

impl HistogramCore {
    pub fn new<V: AsRef<str>>(opts: &HistogramOpts, label_values: &[V]) -> Result<HistogramCore> {
        let desc = opts.describe()?;

        for name in &desc.variable_labels {
            check_bucket_label(name)?;
        }
        for pair in &desc.const_label_pairs {
            check_bucket_label(pair.name())?;
        }

        let label_pairs = make_label_pairs(&desc, label_values)?;

        let buckets = check_and_adjust_buckets(opts.buckets.clone())?;

        Ok(HistogramCore {
            desc,
            label_pairs,

            collect_lock: Mutex::new(()),

            shard_and_count: ShardAndCount::new(),
            shards: [Shard::new(buckets.len()), Shard::new(buckets.len())],

            upper_bounds: buckets,
        })
    }

    /// Record a given observation (f64) in the histogram.
    //
    // First increase the overall observation counter and thus learn which shard
    // is the current hot shard. Subsequently on the hot shard update the
    // corresponding bucket count, adjust the shard's sum and finally increase
    // the shard's count.
    pub fn observe(&self, v: f64) {
        // The collect code path uses `self.shard_and_count` and
        // `self.shards[x].count` to ensure not to collect data from a shard
        // while observe calls are still operating on it.
        //
        // To ensure the above, this `inc` needs to use `Acquire` ordering to
        // force anything below this line to stay below it.
        let (shard_index, _count) = self.shard_and_count.inc(Ordering::Acquire);

        let shard: &Shard = &self.shards[usize::from(shard_index)];

        // Try find the bucket.
        let mut iter = self
            .upper_bounds
            .iter()
            .enumerate()
            .filter(|&(_, f)| v <= *f);
        if let Some((i, _)) = iter.next() {
            shard.buckets[i].inc_by(1);
        }

        shard.sum.inc_by(v);
        // Use `Release` ordering to ensure all operations above stay above.
        shard.count.inc_by_with_ordering(1, Ordering::Release);
    }

    /// Make a snapshot of the current histogram state exposed as a Protobuf
    /// struct.
    //
    // Acquire the collect lock, switch the hot and the cold shard, wait for all
    // remaining `observe` calls to finish on the previously hot now cold shard,
    // snapshot the data, update the now hot shard and reset the cold shard.
    pub fn proto(&self) -> proto::Histogram {
        let collect_guard = self.collect_lock.lock().expect("Lock poisoned");

        // `flip` needs to use AcqRel ordering to ensure the lock operation
        // above stays above and the histogram operations (especially the shard
        // resets) below stay below.
        let (cold_shard_index, overall_count) = self.shard_and_count.flip(Ordering::AcqRel);

        let cold_shard = &self.shards[usize::from(cold_shard_index)];
        let hot_shard = &self.shards[usize::from(cold_shard_index.inverse())];

        // Wait for all currently active `observe` calls on the now cold shard
        // to finish. The above call to `flip` redirects all future `observe`
        // calls to the other previously cold, now hot, shard. Thus once the
        // cold shard counter equals the value of the global counter when the
        // shards were flipped, all in-progress `observe` calls are done. With
        // all of them done, the cold shard is now in a consistent state.
        //
        // `observe` uses `Release` ordering. `compare_exchange` needs to use
        // `Acquire` ordering to ensure that (1) one sees all the previous
        // `observe` stores to the counter and (2) to ensure the below shard
        // modifications happen after this point, thus the shard is not modified
        // by any `observe` operations.
        while cold_shard
            .count
            .compare_exchange_weak(
                overall_count,
                // While at it, reset cold shard count on success.
                0,
                Ordering::Acquire,
                Ordering::Acquire,
            )
            .is_err()
        {}

        // Get cold shard sum and reset to 0.
        //
        // Use `Acquire` for load and `Release` for store to ensure not to
        // interfere with previous or upcoming collect calls.
        let cold_shard_sum = cold_shard.sum.swap(0.0, Ordering::AcqRel);

        let mut h = proto::Histogram::default();
        h.set_sample_sum(cold_shard_sum);
        h.set_sample_count(overall_count);

        let mut cumulative_count = 0;
        let mut buckets = Vec::with_capacity(self.upper_bounds.len());
        for (i, upper_bound) in self.upper_bounds.iter().enumerate() {
            // Reset the cold shard and update the hot shard.
            //
            // Use `Acquire` for load and `Release` for store to ensure not to
            // interfere with previous or upcoming collect calls.
            let cold_bucket_count = cold_shard.buckets[i].swap(0, Ordering::AcqRel);
            hot_shard.buckets[i].inc_by(cold_bucket_count);

            cumulative_count += cold_bucket_count;
            let mut b = proto::Bucket::default();
            b.set_cumulative_count(cumulative_count);
            b.set_upper_bound(*upper_bound);
            buckets.push(b);
        }
        h.set_bucket(buckets);

        // Update the hot shard.
        hot_shard.count.inc_by(overall_count);
        hot_shard.sum.inc_by(cold_shard_sum);

        drop(collect_guard);

        h
    }

    fn sample_sum(&self) -> f64 {
        // Make sure to not overlap with any collect calls, as they might flip
        // the hot and cold shards.
        let _guard = self.collect_lock.lock().expect("Lock poisoned");

        let (shard_index, _count) = self.shard_and_count.get();
        self.shards[shard_index as usize].sum.get()
    }

    fn sample_count(&self) -> u64 {
        self.shard_and_count.get().1
    }
}

// We have to wrap libc::timespec in order to implement std::fmt::Debug.
#[cfg(all(feature = "nightly", target_os = "linux"))]
pub struct Timespec(libc::timespec);

#[cfg(all(feature = "nightly", target_os = "linux"))]
impl std::fmt::Debug for Timespec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Timespec {{ tv_sec: {}, tv_nsec: {} }}",
            self.0.tv_sec, self.0.tv_nsec
        )
    }
}

#[derive(Debug)]
pub enum Instant {
    Monotonic(StdInstant),
    #[cfg(all(feature = "nightly", target_os = "linux"))]
    MonotonicCoarse(Timespec),
}

impl Instant {
    pub fn now() -> Instant {
        Instant::Monotonic(StdInstant::now())
    }

    #[cfg(all(feature = "nightly", target_os = "linux"))]
    pub fn now_coarse() -> Instant {
        Instant::MonotonicCoarse(get_time_coarse())
    }

    #[cfg(all(feature = "nightly", not(target_os = "linux")))]
    pub fn now_coarse() -> Instant {
        Instant::Monotonic(StdInstant::now())
    }

    pub fn elapsed(&self) -> Duration {
        match self {
            // We use `saturating_duration_since` to avoid panics caused by non-monotonic clocks.
            Instant::Monotonic(i) => StdInstant::now().saturating_duration_since(*i),

            // It is different from `Instant::Monotonic`, the resolution here is millisecond.
            // The processors in an SMP system do not start all at exactly the same time
            // and therefore the timer registers are typically running at an offset.
            // Use millisecond resolution for ignoring the error.
            // See more: https://linux.die.net/man/2/clock_gettime
            #[cfg(all(feature = "nightly", target_os = "linux"))]
            Instant::MonotonicCoarse(t) => {
                let now = get_time_coarse();
                let now_ms = now.0.tv_sec * MILLIS_PER_SEC + now.0.tv_nsec / NANOS_PER_MILLI;
                let t_ms = t.0.tv_sec * MILLIS_PER_SEC + t.0.tv_nsec / NANOS_PER_MILLI;
                let dur = now_ms - t_ms;
                if dur >= 0 {
                    Duration::from_millis(dur as u64)
                } else {
                    Duration::from_millis(0)
                }
            }
        }
    }

    #[inline]
    pub fn elapsed_sec(&self) -> f64 {
        duration_to_seconds(self.elapsed())
    }
}

#[cfg(all(feature = "nightly", target_os = "linux"))]
use self::coarse::*;

#[cfg(all(feature = "nightly", target_os = "linux"))]
mod coarse {
    use crate::histogram::Timespec;
    pub use libc::timespec;
    use libc::{clock_gettime, CLOCK_MONOTONIC_COARSE};

    pub const NANOS_PER_MILLI: i64 = 1_000_000;
    pub const MILLIS_PER_SEC: i64 = 1_000;

    pub fn get_time_coarse() -> Timespec {
        let mut t = Timespec(timespec {
            tv_sec: 0,
            tv_nsec: 0,
        });
        assert_eq!(
            unsafe { clock_gettime(CLOCK_MONOTONIC_COARSE, &mut t.0) },
            0
        );
        t
    }
}

/// Timer to measure and record the duration of an event.
///
/// This timer can be stopped and observed at most once, either automatically (when it
/// goes out of scope) or manually.
/// Alternatively, it can be manually stopped and discarded in order to not record its value.
#[must_use = "Timer should be kept in a variable otherwise it cannot observe duration"]
#[derive(Debug)]
pub struct HistogramTimer {
    /// A histogram for automatic recording of observations.
    histogram: Histogram,
    /// Whether the timer has already been observed once.
    observed: bool,
    /// Starting instant for the timer.
    start: Instant,
}

impl HistogramTimer {
    fn new(histogram: Histogram) -> Self {
        Self {
            histogram,
            observed: false,
            start: Instant::now(),
        }
    }

    #[cfg(feature = "nightly")]
    fn new_coarse(histogram: Histogram) -> Self {
        HistogramTimer {
            histogram,
            observed: false,
            start: Instant::now_coarse(),
        }
    }

    /// Observe and record timer duration (in seconds).
    ///
    /// It observes the floating-point number of seconds elapsed since the timer
    /// started, and it records that value to the attached histogram.
    pub fn observe_duration(self) {
        self.stop_and_record();
    }

    /// Observe, record and return timer duration (in seconds).
    ///
    /// It observes and returns a floating-point number for seconds elapsed since
    /// the timer started, recording that value to the attached histogram.
    pub fn stop_and_record(self) -> f64 {
        let mut timer = self;
        timer.observe(true)
    }

    /// Observe and return timer duration (in seconds).
    ///
    /// It returns a floating-point number of seconds elapsed since the timer started,
    /// without recording to any histogram.
    pub fn stop_and_discard(self) -> f64 {
        let mut timer = self;
        timer.observe(false)
    }

    fn observe(&mut self, record: bool) -> f64 {
        let v = self.start.elapsed_sec();
        self.observed = true;
        if record {
            self.histogram.observe(v);
        }
        v
    }
}

impl Drop for HistogramTimer {
    fn drop(&mut self) {
        if !self.observed {
            self.observe(true);
        }
    }
}

/// A [`Metric`] counts individual observations from an event or sample stream
/// in configurable buckets. Similar to a [`Summary`](crate::proto::Summary),
/// it also provides a sum of observations and an observation count.
///
/// On the Prometheus server, quantiles can be calculated from a [`Histogram`] using
/// the [`histogram_quantile`][1] function in the query language.
///
/// Note that Histograms, in contrast to Summaries, can be aggregated with the
/// Prometheus query language (see [the prometheus documentation][2] for
/// detailed procedures). However, Histograms require the user to pre-define
/// suitable buckets, (see [`linear_buckets`] and [`exponential_buckets`] for
/// some helper provided here) and they are in general less accurate. The
/// Observe method of a [`Histogram`] has a very low performance overhead in
/// comparison with the Observe method of a Summary.
///
/// [1]: https://prometheus.io/docs/prometheus/latest/querying/functions/#histogram_quantile
/// [2]: https://prometheus.io/docs/practices/histograms/
#[derive(Clone, Debug)]
pub struct Histogram {
    core: Arc<HistogramCore>,
}

impl Histogram {
    /// `with_opts` creates a [`Histogram`] with the `opts` options.
    pub fn with_opts(opts: HistogramOpts) -> Result<Histogram> {
        Histogram::with_opts_and_label_values::<&str>(&opts, &[])
    }

    fn with_opts_and_label_values<V: AsRef<str>>(
        opts: &HistogramOpts,
        label_values: &[V],
    ) -> Result<Histogram> {
        let core = HistogramCore::new(opts, label_values)?;

        Ok(Histogram {
            core: Arc::new(core),
        })
    }
}

impl Histogram {
    /// Add a single observation to the [`Histogram`].
    pub fn observe(&self, v: f64) {
        self.core.observe(v)
    }

    /// Return a [`HistogramTimer`] to track a duration.
    pub fn start_timer(&self) -> HistogramTimer {
        HistogramTimer::new(self.clone())
    }

    /// Return a [`HistogramTimer`] to track a duration.
    /// It is faster but less precise.
    #[cfg(feature = "nightly")]
    pub fn start_coarse_timer(&self) -> HistogramTimer {
        HistogramTimer::new_coarse(self.clone())
    }

    /// Observe execution time of a closure, in second.
    pub fn observe_closure_duration<F, T>(&self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let instant = Instant::now();
        let res = f();
        let elapsed = instant.elapsed_sec();
        self.observe(elapsed);
        res
    }

    /// Observe execution time of a closure, in second.
    #[cfg(feature = "nightly")]
    pub fn observe_closure_duration_coarse<F, T>(&self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let instant = Instant::now_coarse();
        let res = f();
        let elapsed = instant.elapsed_sec();
        self.observe(elapsed);
        res
    }

    /// Return a [`LocalHistogram`] for single thread usage.
    pub fn local(&self) -> LocalHistogram {
        LocalHistogram::new(self.clone())
    }

    /// Return accumulated sum of all samples.
    pub fn get_sample_sum(&self) -> f64 {
        self.core.sample_sum()
    }

    /// Return count of all samples.
    pub fn get_sample_count(&self) -> u64 {
        self.core.sample_count()
    }
}

impl Metric for Histogram {
    fn metric(&self) -> proto::Metric {
        let mut m = proto::Metric::from_label(self.core.label_pairs.clone());

        let h = self.core.proto();
        m.set_histogram(h);

        m
    }
}

impl Collector for Histogram {
    fn desc(&self) -> Vec<&Desc> {
        vec![&self.core.desc]
    }

    fn collect(&self) -> Vec<proto::MetricFamily> {
        let mut m = proto::MetricFamily::default();
        m.set_name(self.core.desc.fq_name.clone());
        m.set_help(self.core.desc.help.clone());
        m.set_field_type(proto::MetricType::HISTOGRAM);
        m.set_metric(vec![self.metric()]);

        vec![m]
    }
}

#[derive(Clone, Debug)]
pub struct HistogramVecBuilder {}

impl MetricVecBuilder for HistogramVecBuilder {
    type M = Histogram;
    type P = HistogramOpts;

    fn build<V: AsRef<str>>(&self, opts: &HistogramOpts, vals: &[V]) -> Result<Histogram> {
        Histogram::with_opts_and_label_values(opts, vals)
    }
}

/// A [`Collector`] that bundles a set of Histograms that all share the
/// same [`Desc`], but have different values for their variable labels. This is used
/// if you want to count the same thing partitioned by various dimensions
/// (e.g. HTTP request latencies, partitioned by status code and method).
pub type HistogramVec = MetricVec<HistogramVecBuilder>;

impl HistogramVec {
    /// Create a new [`HistogramVec`] based on the provided
    /// [`HistogramOpts`] and partitioned by the given label names. At least
    /// one label name must be provided.
    pub fn new(opts: HistogramOpts, label_names: &[&str]) -> Result<HistogramVec> {
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let opts = opts.variable_labels(variable_names);
        let metric_vec =
            MetricVec::create(proto::MetricType::HISTOGRAM, HistogramVecBuilder {}, opts)?;

        Ok(metric_vec as HistogramVec)
    }

    /// Return a `LocalHistogramVec` for single thread usage.
    pub fn local(&self) -> LocalHistogramVec {
        let vec = self.clone();
        LocalHistogramVec::new(vec)
    }
}

/// Create `count` buckets, each `width` wide, where the lowest
/// bucket has an upper bound of `start`. The final +Inf bucket is not counted
/// and not included in the returned slice. The returned slice is meant to be
/// used for the Buckets field of [`HistogramOpts`].
///
/// The function returns an error if `count` is zero or `width` is zero or
/// negative.
pub fn linear_buckets(start: f64, width: f64, count: usize) -> Result<Vec<f64>> {
    if count < 1 {
        return Err(Error::Msg(format!(
            "LinearBuckets needs a positive count, count: {}",
            count
        )));
    }
    if width <= 0.0 {
        return Err(Error::Msg(format!(
            "LinearBuckets needs a width greater then 0, width: {}",
            width
        )));
    }

    let buckets: Vec<_> = (0..count)
        .map(|step| start + width * (step as f64))
        .collect();

    Ok(buckets)
}

/// Create `count` buckets, where the lowest bucket has an
/// upper bound of `start` and each following bucket's upper bound is `factor`
/// times the previous bucket's upper bound. The final +Inf bucket is not counted
/// and not included in the returned slice. The returned slice is meant to be
/// used for the Buckets field of [`HistogramOpts`].
///
/// The function returns an error if `count` is zero, if `start` is zero or
/// negative, or if `factor` is less than or equal 1.
pub fn exponential_buckets(start: f64, factor: f64, count: usize) -> Result<Vec<f64>> {
    if count < 1 {
        return Err(Error::Msg(format!(
            "exponential_buckets needs a positive count, count: {}",
            count
        )));
    }
    if start <= 0.0 {
        return Err(Error::Msg(format!(
            "exponential_buckets needs a positive start value, \
             start: {}",
            start
        )));
    }
    if factor <= 1.0 {
        return Err(Error::Msg(format!(
            "exponential_buckets needs a factor greater than 1, \
             factor: {}",
            factor
        )));
    }

    let mut next = start;
    let mut buckets = Vec::with_capacity(count);
    for _ in 0..count {
        buckets.push(next);
        next *= factor;
    }

    Ok(buckets)
}

/// `duration_to_seconds` converts Duration to seconds.
#[inline]
pub fn duration_to_seconds(d: Duration) -> f64 {
    let nanos = f64::from(d.subsec_nanos()) / 1e9;
    d.as_secs() as f64 + nanos
}

#[derive(Clone, Debug)]
pub struct LocalHistogramCore {
    histogram: Histogram,
    counts: Vec<u64>,
    count: u64,
    sum: f64,
}

/// An unsync [`Histogram`].
#[derive(Debug)]
pub struct LocalHistogram {
    core: RefCell<LocalHistogramCore>,
}

impl Clone for LocalHistogram {
    fn clone(&self) -> LocalHistogram {
        let core = self.core.clone();
        let lh = LocalHistogram { core };
        lh.clear();
        lh
    }
}

/// An unsync [`HistogramTimer`].
#[must_use = "Timer should be kept in a variable otherwise it cannot observe duration"]
#[derive(Debug)]
pub struct LocalHistogramTimer {
    /// A local histogram for automatic recording of observations.
    local: LocalHistogram,
    /// Whether the timer has already been observed once.
    observed: bool,
    /// Starting instant for the timer.
    start: Instant,
}

impl LocalHistogramTimer {
    fn new(histogram: LocalHistogram) -> Self {
        Self {
            local: histogram,
            observed: false,
            start: Instant::now(),
        }
    }

    #[cfg(feature = "nightly")]
    fn new_coarse(histogram: LocalHistogram) -> Self {
        Self {
            local: histogram,
            observed: false,
            start: Instant::now_coarse(),
        }
    }

    /// Observe and record timer duration (in seconds).
    ///
    /// It observes the floating-point number of seconds elapsed since the timer
    /// started, and it records that value to the attached histogram.
    pub fn observe_duration(self) {
        self.stop_and_record();
    }

    /// Observe, record and return timer duration (in seconds).
    ///
    /// It observes and returns a floating-point number for seconds elapsed since
    /// the timer started, recording that value to the attached histogram.
    pub fn stop_and_record(self) -> f64 {
        let mut timer = self;
        timer.observe(true)
    }

    /// Observe and return timer duration (in seconds).
    ///
    /// It returns a floating-point number of seconds elapsed since the timer started,
    /// without recording to any histogram.
    pub fn stop_and_discard(self) -> f64 {
        let mut timer = self;
        timer.observe(false)
    }

    fn observe(&mut self, record: bool) -> f64 {
        let v = self.start.elapsed_sec();
        self.observed = true;
        if record {
            self.local.observe(v);
        }
        v
    }
}

impl Drop for LocalHistogramTimer {
    fn drop(&mut self) {
        if !self.observed {
            self.observe(true);
        }
    }
}

impl LocalHistogramCore {
    fn new(histogram: Histogram) -> LocalHistogramCore {
        let counts = vec![0; histogram.core.upper_bounds.len()];

        LocalHistogramCore {
            histogram,
            counts,
            count: 0,
            sum: 0.0,
        }
    }

    pub fn observe(&mut self, v: f64) {
        // Try find the bucket.
        let mut iter = self
            .histogram
            .core
            .upper_bounds
            .iter()
            .enumerate()
            .filter(|&(_, f)| v <= *f);
        if let Some((i, _)) = iter.next() {
            self.counts[i] += 1;
        }

        self.count += 1;
        self.sum += v;
    }

    pub fn clear(&mut self) {
        for v in &mut self.counts {
            *v = 0
        }

        self.count = 0;
        self.sum = 0.0;
    }

    pub fn flush(&mut self) {
        // No cached metric, return.
        if self.count == 0 {
            return;
        }

        {
            // The collect code path uses `self.shard_and_count` and
            // `self.shards[x].count` to ensure not to collect data from a shard
            // while observe calls are still operating on it.
            //
            // To ensure the above, this `inc` needs to use `Acquire` ordering
            // to force anything below this line to stay below it.
            let (shard_index, _count) = self
                .histogram
                .core
                .shard_and_count
                .inc_by(self.count, Ordering::Acquire);
            let shard = &self.histogram.core.shards[shard_index as usize];

            for (i, v) in self.counts.iter().enumerate() {
                if *v > 0 {
                    shard.buckets[i].inc_by(*v);
                }
            }

            shard.sum.inc_by(self.sum);
            // Use `Release` ordering to ensure all operations above stay above.
            shard
                .count
                .inc_by_with_ordering(self.count, Ordering::Release);
        }

        self.clear()
    }

    fn sample_sum(&self) -> f64 {
        self.sum
    }

    fn sample_count(&self) -> u64 {
        self.count
    }
}

impl LocalHistogram {
    fn new(histogram: Histogram) -> LocalHistogram {
        let core = LocalHistogramCore::new(histogram);
        LocalHistogram {
            core: RefCell::new(core),
        }
    }

    /// Add a single observation to the [`Histogram`].
    pub fn observe(&self, v: f64) {
        self.core.borrow_mut().observe(v);
    }

    /// Return a `LocalHistogramTimer` to track a duration.
    pub fn start_timer(&self) -> LocalHistogramTimer {
        LocalHistogramTimer::new(self.clone())
    }

    /// Return a `LocalHistogramTimer` to track a duration.
    /// It is faster but less precise.
    #[cfg(feature = "nightly")]
    pub fn start_coarse_timer(&self) -> LocalHistogramTimer {
        LocalHistogramTimer::new_coarse(self.clone())
    }

    /// Observe execution time of a closure, in second.
    pub fn observe_closure_duration<F, T>(&self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let instant = Instant::now();
        let res = f();
        let elapsed = instant.elapsed_sec();
        self.observe(elapsed);
        res
    }

    /// Observe execution time of a closure, in second.
    #[cfg(feature = "nightly")]
    pub fn observe_closure_duration_coarse<F, T>(&self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let instant = Instant::now_coarse();
        let res = f();
        let elapsed = instant.elapsed_sec();
        self.observe(elapsed);
        res
    }

    /// Clear the local metric.
    pub fn clear(&self) {
        self.core.borrow_mut().clear();
    }

    /// Flush the local metrics to the [`Histogram`] metric.
    pub fn flush(&self) {
        self.core.borrow_mut().flush();
    }

    /// Return accumulated sum of local samples.
    pub fn get_sample_sum(&self) -> f64 {
        self.core.borrow().sample_sum()
    }

    /// Return count of local samples.
    pub fn get_sample_count(&self) -> u64 {
        self.core.borrow().sample_count()
    }
}

impl LocalMetric for LocalHistogram {
    /// Flush the local metrics to the [`Histogram`] metric.
    fn flush(&self) {
        LocalHistogram::flush(self);
    }
}

impl Drop for LocalHistogram {
    fn drop(&mut self) {
        self.flush()
    }
}

/// An unsync [`HistogramVec`].
#[derive(Debug)]
pub struct LocalHistogramVec {
    vec: HistogramVec,
    local: HashMap<u64, LocalHistogram>,
}

impl LocalHistogramVec {
    fn new(vec: HistogramVec) -> LocalHistogramVec {
        let local = HashMap::with_capacity(vec.v.children.read().len());
        LocalHistogramVec { vec, local }
    }

    /// Get a [`LocalHistogram`] by label values.
    /// See more [`MetricVec::with_label_values`].
    pub fn with_label_values<'a>(&'a mut self, vals: &[&str]) -> &'a LocalHistogram {
        let hash = self.vec.v.hash_label_values(vals).unwrap();
        let vec = &self.vec;
        self.local
            .entry(hash)
            .or_insert_with(|| vec.with_label_values(vals).local())
    }

    /// Remove a [`LocalHistogram`] by label values.
    /// See more [`MetricVec::remove_label_values`].
    pub fn remove_label_values(&mut self, vals: &[&str]) -> Result<()> {
        let hash = self.vec.v.hash_label_values(vals)?;
        self.local.remove(&hash);
        self.vec.v.delete_label_values(vals)
    }

    /// Flush the local metrics to the [`HistogramVec`] metric.
    pub fn flush(&self) {
        for h in self.local.values() {
            h.flush();
        }
    }
}

impl LocalMetric for LocalHistogramVec {
    /// Flush the local metrics to the [`HistogramVec`] metric.
    fn flush(&self) {
        LocalHistogramVec::flush(self)
    }
}

impl Clone for LocalHistogramVec {
    fn clone(&self) -> LocalHistogramVec {
        LocalHistogramVec::new(self.vec.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::f64;
    use std::thread;
    use std::time::Duration;

    use super::*;
    use crate::metrics::{Collector, Metric};

    #[test]
    fn test_histogram() {
        let opts = HistogramOpts::new("test1", "test help")
            .const_label("a", "1")
            .const_label("b", "2");
        let histogram = Histogram::with_opts(opts).unwrap();
        histogram.observe(1.0);

        let timer = histogram.start_timer();
        thread::sleep(Duration::from_millis(100));
        timer.observe_duration();

        let timer = histogram.start_timer();
        let handler = thread::spawn(move || {
            let _timer = timer;
            thread::sleep(Duration::from_millis(400));
        });
        assert!(handler.join().is_ok());

        let mut mfs = histogram.collect();
        assert_eq!(mfs.len(), 1);

        let mf = mfs.pop().unwrap();
        let m = mf.get_metric().first().unwrap();
        assert_eq!(m.get_label().len(), 2);
        let proto_histogram = m.get_histogram();
        assert_eq!(proto_histogram.get_sample_count(), 3);
        assert!(proto_histogram.get_sample_sum() >= 1.5);
        assert_eq!(proto_histogram.get_bucket().len(), DEFAULT_BUCKETS.len());

        let buckets = vec![1.0, 2.0, 3.0];
        let opts = HistogramOpts::new("test2", "test help").buckets(buckets.clone());
        let histogram = Histogram::with_opts(opts).unwrap();
        let mut mfs = histogram.collect();
        assert_eq!(mfs.len(), 1);

        let mf = mfs.pop().unwrap();
        let m = mf.get_metric().first().unwrap();
        assert_eq!(m.get_label().len(), 0);
        let proto_histogram = m.get_histogram();
        assert_eq!(proto_histogram.get_sample_count(), 0);
        assert!((proto_histogram.get_sample_sum() - 0.0) < f64::EPSILON);
        assert_eq!(proto_histogram.get_bucket().len(), buckets.len())
    }

    #[test]
    #[cfg(feature = "nightly")]
    fn test_histogram_coarse_timer() {
        let opts = HistogramOpts::new("test1", "test help");
        let histogram = Histogram::with_opts(opts).unwrap();

        let timer = histogram.start_coarse_timer();
        thread::sleep(Duration::from_millis(100));
        timer.observe_duration();

        let timer = histogram.start_coarse_timer();
        let handler = thread::spawn(move || {
            let _timer = timer;
            thread::sleep(Duration::from_millis(400));
        });
        assert!(handler.join().is_ok());

        histogram.observe_closure_duration(|| {
            thread::sleep(Duration::from_millis(400));
        });

        let mut mfs = histogram.collect();
        assert_eq!(mfs.len(), 1);

        let mf = mfs.pop().unwrap();
        let m = mf.get_metric().get(0).unwrap();
        let proto_histogram = m.get_histogram();
        assert_eq!(proto_histogram.get_sample_count(), 3);
        assert!((proto_histogram.get_sample_sum() - 0.0) > f64::EPSILON);
    }

    #[test]
    #[cfg(feature = "nightly")]
    fn test_instant_on_smp() {
        let zero = Duration::from_millis(0);
        for i in 0..100_000 {
            let now = Instant::now();
            let now_coarse = Instant::now_coarse();
            if i % 100 == 0 {
                thread::yield_now();
            }
            assert!(now.elapsed() >= zero);
            assert!(now_coarse.elapsed() >= zero);
        }
    }

    #[test]
    fn test_buckets_invalidation() {
        let table = vec![
            (vec![], true, DEFAULT_BUCKETS.len()),
            (vec![-2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0], true, 7),
            (vec![-2.0, -1.0, -0.5, 10.0, 0.5, 1.0, 2.0], false, 7),
            (
                vec![-2.0, -1.0, -0.5, 0.0, 0.5, 1.0, f64::INFINITY],
                true,
                6,
            ),
        ];

        for (buckets, is_ok, length) in table {
            let got = check_and_adjust_buckets(buckets);
            assert_eq!(got.is_ok(), is_ok);
            if is_ok {
                assert_eq!(got.unwrap().len(), length);
            }
        }
    }

    #[test]
    fn test_buckets_functions() {
        let linear_table = vec![
            (
                -15.0,
                5.0,
                6,
                true,
                vec![-15.0, -10.0, -5.0, 0.0, 5.0, 10.0],
            ),
            (-15.0, 0.0, 6, false, vec![]),
            (-15.0, 5.0, 0, false, vec![]),
        ];

        for (param1, param2, param3, is_ok, vec) in linear_table {
            let got = linear_buckets(param1, param2, param3);
            assert_eq!(got.is_ok(), is_ok);
            if got.is_ok() {
                assert_eq!(got.unwrap(), vec);
            }
        }

        let exponential_table = vec![
            (100.0, 1.2, 3, true, vec![100.0, 120.0, 144.0]),
            (100.0, 0.5, 3, false, vec![]),
            (100.0, 1.2, 0, false, vec![]),
        ];

        for (param1, param2, param3, is_ok, vec) in exponential_table {
            let got = exponential_buckets(param1, param2, param3);
            assert_eq!(got.is_ok(), is_ok);
            if got.is_ok() {
                assert_eq!(got.unwrap(), vec);
            }
        }
    }

    #[test]
    fn test_duration_to_seconds() {
        let tbls = vec![(1000, 1.0), (1100, 1.1), (100_111, 100.111)];
        for (millis, seconds) in tbls {
            let d = Duration::from_millis(millis);
            let v = duration_to_seconds(d);
            assert!((v - seconds).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_histogram_vec_with_label_values() {
        let vec = HistogramVec::new(
            HistogramOpts::new("test_histogram_vec", "test histogram vec help"),
            &["l1", "l2"],
        )
        .unwrap();

        assert!(vec.remove_label_values(&["v1", "v2"]).is_err());
        vec.with_label_values(&["v1", "v2"]).observe(1.0);
        assert!(vec.remove_label_values(&["v1", "v2"]).is_ok());

        assert!(vec.remove_label_values(&["v1"]).is_err());
        assert!(vec.remove_label_values(&["v1", "v3"]).is_err());
    }

    #[test]
    fn test_histogram_vec_with_owned_label_values() {
        let vec = HistogramVec::new(
            HistogramOpts::new("test_histogram_vec", "test histogram vec help"),
            &["l1", "l2"],
        )
        .unwrap();

        let v1 = "v1".to_string();
        let v2 = "v2".to_string();
        let v3 = "v3".to_string();

        assert!(vec.remove_label_values(&[v1.clone(), v2.clone()]).is_err());
        vec.with_label_values(&[v1.clone(), v2.clone()])
            .observe(1.0);
        assert!(vec.remove_label_values(&[v1.clone(), v2.clone()]).is_ok());

        assert!(vec.remove_label_values(&[v1.clone()]).is_err());
        assert!(vec.remove_label_values(&[v1.clone(), v3.clone()]).is_err());
    }

    #[test]
    fn test_histogram_vec_with_opts_buckets() {
        let labels = ["l1", "l2"];
        let buckets = vec![1.0, 2.0, 3.0];
        let vec = HistogramVec::new(
            HistogramOpts::new("test_histogram_vec", "test histogram vec help")
                .buckets(buckets.clone()),
            &labels,
        )
        .unwrap();

        let histogram = vec.with_label_values(&["v1", "v2"]);
        histogram.observe(1.0);

        let m = histogram.metric();
        assert_eq!(m.get_label().len(), labels.len());

        let proto_histogram = m.get_histogram();
        assert_eq!(proto_histogram.get_sample_count(), 1);
        assert!((proto_histogram.get_sample_sum() - 1.0) < f64::EPSILON);
        assert_eq!(proto_histogram.get_bucket().len(), buckets.len())
    }

    #[test]
    fn test_histogram_local() {
        let buckets = vec![1.0, 2.0, 3.0];
        let opts = HistogramOpts::new("test_histogram_local", "test histogram local help")
            .buckets(buckets.clone());
        let histogram = Histogram::with_opts(opts).unwrap();
        let local = histogram.local();

        let check = |count, sum| {
            let m = histogram.metric();
            let proto_histogram = m.get_histogram();
            assert_eq!(proto_histogram.get_sample_count(), count);
            assert!((proto_histogram.get_sample_sum() - sum) < f64::EPSILON);
        };

        local.observe(1.0);
        local.observe(4.0);
        check(0, 0.0);

        local.flush();
        check(2, 5.0);

        local.observe(2.0);
        local.clear();
        check(2, 5.0);

        local.observe(2.0);
        drop(local);
        check(3, 7.0);
    }

    #[test]
    fn test_histogram_vec_local() {
        let vec = HistogramVec::new(
            HistogramOpts::new("test_histogram_vec_local", "test histogram vec help"),
            &["l1", "l2"],
        )
        .unwrap();
        let mut local_vec = vec.local();

        vec.remove_label_values(&["v1", "v2"]).unwrap_err();
        local_vec.remove_label_values(&["v1", "v2"]).unwrap_err();

        let check = |count, sum| {
            let ms = vec.collect()[0].take_metric();
            let proto_histogram = ms[0].get_histogram();
            assert_eq!(proto_histogram.get_sample_count(), count);
            assert!((proto_histogram.get_sample_sum() - sum) < f64::EPSILON);
        };

        {
            // Flush LocalHistogram
            let h = local_vec.with_label_values(&["v1", "v2"]);
            h.observe(1.0);
            h.flush();
            check(1, 1.0);
        }

        {
            // Flush LocalHistogramVec
            local_vec.with_label_values(&["v1", "v2"]).observe(4.0);
            local_vec.flush();
            check(2, 5.0);
        }
        {
            // Reset ["v1", "v2"]
            local_vec.remove_label_values(&["v1", "v2"]).unwrap();

            // Flush on drop
            local_vec.with_label_values(&["v1", "v2"]).observe(2.0);
            drop(local_vec);
            check(1, 2.0);
        }
    }

    /// Ensure that when an observe and a collect operation interleave, the
    /// latter does not expose a snapshot of the histogram that does not uphold
    /// all histogram invariants.
    #[test]
    fn atomic_observe_across_collects() {
        let done = Arc::new(std::sync::atomic::AtomicBool::default());
        let histogram =
            Histogram::with_opts(HistogramOpts::new("test_name", "test help").buckets(vec![1.0]))
                .unwrap();

        let done_clone = done.clone();
        let histogram_clone = histogram.clone();
        let observing_thread = std::thread::spawn(move || loop {
            if done_clone.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            for _ in 0..1_000_000 {
                histogram_clone.observe(1.0);
            }
        });

        let mut sample_count = 0;
        let mut cumulative_count = 0;
        let mut sample_sum = 0;
        for _ in 0..1_000_000 {
            let metric = &histogram.collect()[0].take_metric()[0];
            let proto = metric.get_histogram();

            sample_count = proto.get_sample_count();
            sample_sum = proto.get_sample_sum() as u64;
            // There is only one bucket thus the `[0]`.
            cumulative_count = proto.get_bucket()[0].cumulative_count();

            if sample_count != cumulative_count {
                break;
            }

            // Observation value is always `1.0` thus count and sum should
            // always equal. The number of `observe` calls is limited to
            // 1_000_000, thus the sum is limited to 1_000_000. A float 64 is
            // able to represent the sum accurately up to 9_007_199_254_740_992.
            if sample_count != sample_sum {
                break;
            }
        }

        done.store(true, std::sync::atomic::Ordering::Relaxed);
        observing_thread.join().unwrap();

        if sample_count != cumulative_count {
            panic!(
                "Histogram invariant violated: For a histogram with a single \
                 bucket observing values below the bucket's upper bound only \
                 the histogram's count should always be equal to the buckets's \
                 cumulative count, got {:?} and {:?} instead.",
                sample_count, cumulative_count,
            );
        }

        if sample_count != sample_sum {
            panic!(
                "Histogram invariant violated: For a histogram which is only \
                 ever observing a value of `1.0` the sample count should equal \
                 the sum, instead got: {:?} and {:?}",
                sample_count, sample_sum,
            )
        }
    }

    #[test]
    fn test_error_on_inconsistent_label_cardinality() {
        let hist = Histogram::with_opts(
            histogram_opts!(
                "example_histogram",
                "Used as an example",
                vec![0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 5.0]
            )
            .variable_label("example_variable"),
        );

        if let Err(Error::InconsistentCardinality { expect, got }) = hist {
            assert_eq!(1, expect);
            assert_eq!(0, got);
        } else {
            panic!("Expected InconsistentCardinality error.")
        }
    }
}
