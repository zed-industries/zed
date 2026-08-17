use crate::core::Atomic;
use crate::counter::{CounterWithValueType, GenericLocalCounter};
use crate::histogram::{Instant, LocalHistogram};
use crate::metrics::MayFlush;
use crate::timer;
use parking_lot::Mutex;
use std::thread::LocalKey;

/// Delegator for auto flush-able local counter
pub trait CounterDelegator<T: 'static + MayFlush, V: CounterWithValueType> {
    /// Get the root local metric for delegate
    fn get_root_metric(&self) -> &'static LocalKey<T>;

    /// Get the final counter for delegate
    fn get_local<'a>(&self, root_metric: &'a T) -> &'a GenericLocalCounter<V::ValueType>;
}

/// Delegator for auto flush-able local counter
pub trait HistogramDelegator<T: 'static + MayFlush> {
    /// Get the root local metric for delegate
    fn get_root_metric(&self) -> &'static LocalKey<T>;

    /// Get the final counter for delegate
    fn get_local<'a>(&self, root_metric: &'a T) -> &'a LocalHistogram;
}

/// Auto flush-able local counter
#[derive(Debug)]
pub struct AFLocalCounter<T: 'static + MayFlush, V: CounterWithValueType, D: CounterDelegator<T, V>>
{
    /// Delegator to get thread local metric
    delegator: D,
    /// Phantomdata marker
    _p: std::marker::PhantomData<(Mutex<T>, Mutex<V>)>,
}

impl<T: 'static + MayFlush, V: CounterWithValueType, D: CounterDelegator<T, V>>
    AFLocalCounter<T, V, D>
{
    /// Construct a new AFLocalCounter from delegator.
    pub fn new(delegator: D) -> AFLocalCounter<T, V, D> {
        timer::ensure_updater();
        AFLocalCounter {
            delegator,
            _p: std::marker::PhantomData,
        }
    }
}

/// Auto flush-able local counter
impl<T: 'static + MayFlush, V: CounterWithValueType, D: CounterDelegator<T, V>>
    AFLocalCounter<T, V, D>
{
    #[inline]
    /// Get the root local metric for delegate
    fn get_root_metric(&self) -> &'static LocalKey<T> {
        self.delegator.get_root_metric()
    }

    #[inline]
    /// Get the final counter for delegate
    fn get_counter<'a>(&self, root_metric: &'a T) -> &'a GenericLocalCounter<V::ValueType> {
        self.delegator.get_local(root_metric)
    }

    /// Increase the given value to the local counter,
    /// and try to flush to global
    /// # Panics
    ///
    /// Panics in debug build if the value is < 0.
    #[inline]
    pub fn inc_by(&self, v: <V::ValueType as Atomic>::T) {
        self.get_root_metric().with(|m| {
            let counter = self.get_counter(m);
            counter.inc_by(v);
            m.may_flush();
        })
    }

    /// Increase the local counter by 1,
    /// and try to flush to global.
    #[inline]
    pub fn inc(&self) {
        self.get_root_metric().with(|m| {
            let counter = self.get_counter(m);
            counter.inc();
            m.may_flush();
        })
    }

    /// Return the local counter value.
    #[inline]
    pub fn get(&self) -> <V::ValueType as Atomic>::T {
        self.get_root_metric().with(|m| {
            let counter = self.get_counter(m);
            counter.get()
        })
    }

    /// Restart the counter, resetting its value back to 0.
    #[inline]
    pub fn reset(&self) {
        self.get_root_metric().with(|m| {
            let counter = self.get_counter(m);
            counter.reset();
        })
    }

    /// trigger flush of LocalKey<T>
    #[inline]
    pub fn flush(&self) {
        self.get_root_metric().with(|m| m.flush())
    }
}

/// Auto flush-able local counter
#[derive(Debug)]
pub struct AFLocalHistogram<T: 'static + MayFlush, D: HistogramDelegator<T>> {
    /// Delegator to get thread local metric
    delegator: D,
    /// Phantomdata marker
    _p: std::marker::PhantomData<Mutex<T>>,
}

impl<T: 'static + MayFlush, D: HistogramDelegator<T>> AFLocalHistogram<T, D> {
    /// Construct a new AFLocalHistogram from delegator
    pub fn new(delegator: D) -> AFLocalHistogram<T, D> {
        timer::ensure_updater();
        AFLocalHistogram {
            delegator,
            _p: std::marker::PhantomData,
        }
    }
}

impl<M: 'static + MayFlush, D: HistogramDelegator<M>> AFLocalHistogram<M, D> {
    /// Add a single observation to the [`Histogram`](crate::Histogram).
    pub fn observe(&self, v: f64) {
        self.delegator.get_root_metric().with(|m| {
            let local = self.delegator.get_local(m);
            local.observe(v);
            m.may_flush();
        })
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
        self.delegator
            .get_root_metric()
            .with(|m| self.delegator.get_local(m).clear())
    }

    /// Flush the local metrics to the [`Histogram`](crate::Histogram) metric.
    pub fn flush(&self) {
        self.delegator
            .get_root_metric()
            .with(|m| self.delegator.get_local(m).flush());
    }

    /// Return accumulated sum of local samples.
    pub fn get_sample_sum(&self) -> f64 {
        self.delegator
            .get_root_metric()
            .with(|m| self.delegator.get_local(m).get_sample_sum())
    }

    /// Return count of local samples.
    pub fn get_sample_count(&self) -> u64 {
        self.delegator
            .get_root_metric()
            .with(|m| self.delegator.get_local(m).get_sample_count())
    }
}
