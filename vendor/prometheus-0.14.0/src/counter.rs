// Copyright 2014 The Prometheus Authors
// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::atomic64::{Atomic, AtomicF64, AtomicU64, Number};
use crate::desc::Desc;
use crate::errors::Result;
use crate::metrics::{Collector, LocalMetric, Metric, Opts};
use crate::proto;
use crate::value::{Value, ValueType};
use crate::vec::{MetricVec, MetricVecBuilder};

/// The underlying implementation for [`Counter`] and [`IntCounter`].
#[derive(Debug)]
pub struct GenericCounter<P: Atomic> {
    v: Arc<Value<P>>,
}

/// A [`Metric`] represents a single numerical value that only ever goes up.
pub type Counter = GenericCounter<AtomicF64>;

/// The integer version of [`Counter`]. Provides better performance if metric values
/// are all positive integers (natural numbers).
pub type IntCounter = GenericCounter<AtomicU64>;

impl<P: Atomic> Clone for GenericCounter<P> {
    fn clone(&self) -> Self {
        Self {
            v: Arc::clone(&self.v),
        }
    }
}

impl<P: Atomic> GenericCounter<P> {
    /// Create a [`GenericCounter`] with the `name` and `help` arguments.
    pub fn new<S1: Into<String>, S2: Into<String>>(name: S1, help: S2) -> Result<Self> {
        let opts = Opts::new(name, help);
        Self::with_opts(opts)
    }

    /// Create a [`GenericCounter`] with the `opts` options.
    pub fn with_opts(opts: Opts) -> Result<Self> {
        Self::with_opts_and_label_values::<&str>(&opts, &[])
    }

    fn with_opts_and_label_values<V: AsRef<str>>(opts: &Opts, label_values: &[V]) -> Result<Self> {
        let v = Value::new(opts, ValueType::Counter, P::T::from_i64(0), label_values)?;
        Ok(Self { v: Arc::new(v) })
    }

    /// Increase the given value to the counter.
    ///
    /// # Panics
    ///
    /// Panics in debug build if the value is < 0.
    #[inline]
    pub fn inc_by(&self, v: P::T) {
        debug_assert!(v >= P::T::from_i64(0));
        self.v.inc_by(v);
    }

    /// Increase the counter by 1.
    #[inline]
    pub fn inc(&self) {
        self.v.inc();
    }

    /// Return the counter value.
    #[inline]
    pub fn get(&self) -> P::T {
        self.v.get()
    }

    /// Restart the counter, resetting its value back to 0.
    #[inline]
    pub fn reset(&self) {
        self.v.set(P::T::from_i64(0))
    }

    /// Return a [`GenericLocalCounter`] for single thread usage.
    pub fn local(&self) -> GenericLocalCounter<P> {
        GenericLocalCounter::new(self.clone())
    }
}

impl<P: Atomic> Collector for GenericCounter<P> {
    fn desc(&self) -> Vec<&Desc> {
        vec![&self.v.desc]
    }

    fn collect(&self) -> Vec<proto::MetricFamily> {
        vec![self.v.collect()]
    }
}

impl<P: Atomic> Metric for GenericCounter<P> {
    fn metric(&self) -> proto::Metric {
        self.v.metric()
    }
}

#[derive(Debug)]
pub struct CounterVecBuilder<P: Atomic> {
    _phantom: PhantomData<P>,
}

impl<P: Atomic> CounterVecBuilder<P> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<P: Atomic> Clone for CounterVecBuilder<P> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<P: Atomic> MetricVecBuilder for CounterVecBuilder<P> {
    type M = GenericCounter<P>;
    type P = Opts;

    fn build<V: AsRef<str>>(&self, opts: &Opts, vals: &[V]) -> Result<Self::M> {
        Self::M::with_opts_and_label_values(opts, vals)
    }
}

/// The underlying implementation for [`CounterVec`] and [`IntCounterVec`].
pub type GenericCounterVec<P> = MetricVec<CounterVecBuilder<P>>;

/// A [`Collector`] that bundles a set of [`Counter`]s that all share
/// the same [`Desc`], but have different values for their variable labels. This is
/// used if you want to count the same thing partitioned by various dimensions
/// (e.g. number of HTTP requests, partitioned by response code and method).
pub type CounterVec = GenericCounterVec<AtomicF64>;

/// The integer version of [`CounterVec`]. Provides better performance if metric
/// are all positive integers (natural numbers).
pub type IntCounterVec = GenericCounterVec<AtomicU64>;

impl<P: Atomic> GenericCounterVec<P> {
    /// Create a new [`GenericCounterVec`] based on the provided
    /// [`Opts`] and partitioned by the given label names. At least one label name must be
    /// provided.
    pub fn new(opts: Opts, label_names: &[&str]) -> Result<Self> {
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let opts = opts.variable_labels(variable_names);
        let metric_vec =
            MetricVec::create(proto::MetricType::COUNTER, CounterVecBuilder::new(), opts)?;

        Ok(metric_vec as Self)
    }

    /// Return a [`GenericLocalCounterVec`] for single thread usage.
    pub fn local(&self) -> GenericLocalCounterVec<P> {
        GenericLocalCounterVec::new(self.clone())
    }
}

/// The underlying implementation for [`LocalCounter`]
/// and [`LocalIntCounter`].
#[derive(Debug)]
pub struct GenericLocalCounter<P: Atomic> {
    counter: GenericCounter<P>,
    val: RefCell<P::T>,
}

/// For auto_flush::AFLocalCounter to use to make type inference possible
pub trait CounterWithValueType {
    ///the exact type which implements Atomic
    type ValueType: Atomic;
}

impl<P: Atomic> CounterWithValueType for GenericLocalCounter<P> {
    type ValueType = P;
}

/// An unsync [`Counter`].
pub type LocalCounter = GenericLocalCounter<AtomicF64>;

/// The integer version of [`LocalCounter`]. Provides better performance
/// are all positive integers (natural numbers).
pub type LocalIntCounter = GenericLocalCounter<AtomicU64>;

impl<P: Atomic> GenericLocalCounter<P> {
    fn new(counter: GenericCounter<P>) -> Self {
        Self {
            counter,
            val: RefCell::new(P::T::from_i64(0)),
        }
    }

    /// Increase the given value to the local counter.
    ///
    /// # Panics
    ///
    /// Panics in debug build if the value is < 0.
    #[inline]
    pub fn inc_by(&self, v: P::T) {
        debug_assert!(v >= P::T::from_i64(0));
        *self.val.borrow_mut() += v;
    }

    /// Increase the local counter by 1.
    #[inline]
    pub fn inc(&self) {
        *self.val.borrow_mut() += P::T::from_i64(1);
    }

    /// Return the local counter value.
    #[inline]
    pub fn get(&self) -> P::T {
        *self.val.borrow()
    }

    /// Restart the counter, resetting its value back to 0.
    #[inline]
    pub fn reset(&self) {
        *self.val.borrow_mut() = P::T::from_i64(0);
    }

    /// Flush the local metrics to the [`Counter`].
    #[inline]
    pub fn flush(&self) {
        if *self.val.borrow() == P::T::from_i64(0) {
            return;
        }
        self.counter.inc_by(*self.val.borrow());
        *self.val.borrow_mut() = P::T::from_i64(0);
    }
}

impl<P: Atomic> LocalMetric for GenericLocalCounter<P> {
    /// Flush the local metrics to the [`Counter`].
    #[inline]
    fn flush(&self) {
        GenericLocalCounter::flush(self);
    }
}

impl<P: Atomic> Clone for GenericLocalCounter<P> {
    fn clone(&self) -> Self {
        Self::new(self.counter.clone())
    }
}

/// The underlying implementation for [`LocalCounterVec`]
/// and [`LocalIntCounterVec`].
pub struct GenericLocalCounterVec<P: Atomic> {
    vec: GenericCounterVec<P>,
    local: HashMap<u64, GenericLocalCounter<P>>,
}

impl<P: Atomic> std::fmt::Debug for GenericLocalCounterVec<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GenericLocalCounterVec ({} locals)",
            self.local.keys().len()
        )
    }
}

/// An unsync [`CounterVec`].
pub type LocalCounterVec = GenericLocalCounterVec<AtomicF64>;

/// The integer version of [`LocalCounterVec`].
/// Provides better performance if metric values are all positive
/// integers (natural numbers).
pub type LocalIntCounterVec = GenericLocalCounterVec<AtomicU64>;

impl<P: Atomic> GenericLocalCounterVec<P> {
    fn new(vec: GenericCounterVec<P>) -> Self {
        let local = HashMap::with_capacity(vec.v.children.read().len());
        Self { vec, local }
    }

    /// Get a [`GenericLocalCounter`] by label values.
    /// See more [MetricVec::with_label_values].
    pub fn with_label_values<'a>(&'a mut self, vals: &[&str]) -> &'a mut GenericLocalCounter<P> {
        let hash = self.vec.v.hash_label_values(vals).unwrap();
        let vec = &self.vec;
        self.local
            .entry(hash)
            .or_insert_with(|| vec.with_label_values(vals).local())
    }

    /// Remove a [`GenericLocalCounter`] by label values.
    /// See more [MetricVec::remove_label_values].
    pub fn remove_label_values(&mut self, vals: &[&str]) -> Result<()> {
        let hash = self.vec.v.hash_label_values(vals)?;
        self.local.remove(&hash);
        self.vec.v.delete_label_values(vals)
    }

    /// Flush the local metrics to the [`CounterVec`] metric.
    pub fn flush(&self) {
        for h in self.local.values() {
            h.flush();
        }
    }
}

impl<P: Atomic> LocalMetric for GenericLocalCounterVec<P> {
    /// Flush the local metrics to the [`CounterVec`] metric.
    fn flush(&self) {
        GenericLocalCounterVec::flush(self);
    }
}

impl<P: Atomic> Clone for GenericLocalCounterVec<P> {
    fn clone(&self) -> Self {
        Self::new(self.vec.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::f64;

    use super::*;
    use crate::metrics::{Collector, Opts};
    #[cfg(feature = "protobuf")]
    use crate::proto_ext::MessageFieldExt;

    #[test]
    fn test_counter() {
        let opts = Opts::new("test", "test help")
            .const_label("a", "1")
            .const_label("b", "2");
        let counter = Counter::with_opts(opts).unwrap();
        counter.inc();
        assert_eq!(counter.get() as u64, 1);
        counter.inc_by(42.0);
        assert_eq!(counter.get() as u64, 43);

        let mut mfs = counter.collect();
        assert_eq!(mfs.len(), 1);

        let mf = mfs.pop().unwrap();
        let m = mf.get_metric().first().unwrap();
        assert_eq!(m.get_label().len(), 2);
        assert_eq!(m.get_counter().get_value() as u64, 43);

        counter.reset();
        assert_eq!(counter.get() as u64, 0);
    }

    #[test]
    fn test_int_counter() {
        let counter = IntCounter::new("foo", "bar").unwrap();
        counter.inc();
        assert_eq!(counter.get(), 1);
        counter.inc_by(11);
        assert_eq!(counter.get(), 12);

        let mut mfs = counter.collect();
        assert_eq!(mfs.len(), 1);

        let mf = mfs.pop().unwrap();
        let m = mf.get_metric().first().unwrap();
        assert_eq!(m.get_label().len(), 0);
        assert_eq!(m.get_counter().get_value() as u64, 12);

        counter.reset();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_local_counter() {
        let counter = Counter::new("counter", "counter helper").unwrap();
        let local_counter1 = counter.local();
        let local_counter2 = counter.local();

        local_counter1.inc();
        local_counter2.inc();
        assert_eq!(local_counter1.get() as u64, 1);
        assert_eq!(local_counter2.get() as u64, 1);
        assert_eq!(counter.get() as u64, 0);
        local_counter1.flush();
        assert_eq!(local_counter1.get() as u64, 0);
        assert_eq!(counter.get() as u64, 1);
        local_counter2.flush();
        assert_eq!(counter.get() as u64, 2);

        local_counter1.reset();
        local_counter2.reset();
        counter.reset();
        assert_eq!(counter.get() as u64, 0);
        local_counter1.flush();
        assert_eq!(counter.get() as u64, 0);
        local_counter2.flush();
        assert_eq!(counter.get() as u64, 0);
    }

    #[test]
    fn test_int_local_counter() {
        let counter = IntCounter::new("foo", "bar").unwrap();
        let local_counter = counter.local();

        local_counter.inc();
        assert_eq!(local_counter.get(), 1);
        assert_eq!(counter.get(), 0);

        local_counter.inc_by(5);
        local_counter.flush();
        assert_eq!(local_counter.get(), 0);
        assert_eq!(counter.get(), 6);

        local_counter.reset();
        counter.reset();
        assert_eq!(counter.get(), 0);
        local_counter.flush();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_counter_vec_with_labels() {
        let vec = CounterVec::new(
            Opts::new("test_couter_vec", "test counter vec help"),
            &["l1", "l2"],
        )
        .unwrap();

        let mut labels = HashMap::new();
        labels.insert("l1", "v1");
        labels.insert("l2", "v2");
        assert!(vec.remove(&labels).is_err());

        vec.with(&labels).inc();
        assert!(vec.remove(&labels).is_ok());
        assert!(vec.remove(&labels).is_err());

        let mut labels2 = HashMap::new();
        labels2.insert("l1", "v2");
        labels2.insert("l2", "v1");

        vec.with(&labels).inc();
        assert!(vec.remove(&labels2).is_err());

        vec.with(&labels).inc();

        let mut labels3 = HashMap::new();
        labels3.insert("l1", "v1");
        assert!(vec.remove(&labels3).is_err());
    }

    #[test]
    fn test_counter_vec_with_owned_labels() {
        let vec = CounterVec::new(
            Opts::new("test_couter_vec", "test counter vec help"),
            &["l1", "l2"],
        )
        .unwrap();

        let v1 = "v1".to_string();
        let v2 = "v2".to_string();

        let mut labels = HashMap::new();
        labels.insert("l1", v1.clone());
        labels.insert("l2", v2.clone());
        assert!(vec.remove(&labels).is_err());

        vec.with(&labels).inc();
        assert!(vec.remove(&labels).is_ok());
        assert!(vec.remove(&labels).is_err());

        let mut labels2 = HashMap::new();
        labels2.insert("l1", v2.clone());
        labels2.insert("l2", v1.clone());

        vec.with(&labels).inc();
        assert!(vec.remove(&labels2).is_err());

        vec.with(&labels).inc();

        let mut labels3 = HashMap::new();
        labels3.insert("l1", v1.clone());
        assert!(vec.remove(&labels3).is_err());
    }

    #[test]
    fn test_int_counter_vec() {
        let vec = IntCounterVec::new(Opts::new("foo", "bar"), &["l1", "l2"]).unwrap();

        vec.with_label_values(&["v1", "v3"]).inc();
        assert_eq!(vec.with_label_values(&["v1", "v3"]).get(), 1);

        vec.with_label_values(&["v1", "v2"]).inc_by(12);
        assert_eq!(vec.with_label_values(&["v1", "v3"]).get(), 1);
        assert_eq!(vec.with_label_values(&["v1", "v2"]).get(), 12);

        vec.with_label_values(&["v4", "v2"]).inc_by(2);
        assert_eq!(vec.with_label_values(&["v1", "v3"]).get(), 1);
        assert_eq!(vec.with_label_values(&["v1", "v2"]).get(), 12);
        assert_eq!(vec.with_label_values(&["v4", "v2"]).get(), 2);

        vec.with_label_values(&["v1", "v3"]).inc_by(5);
        assert_eq!(vec.with_label_values(&["v1", "v3"]).get(), 6);
        assert_eq!(vec.with_label_values(&["v1", "v2"]).get(), 12);
        assert_eq!(vec.with_label_values(&["v4", "v2"]).get(), 2);
    }

    #[test]
    fn test_counter_vec_with_label_values() {
        let vec = CounterVec::new(
            Opts::new("test_vec", "test counter vec help"),
            &["l1", "l2"],
        )
        .unwrap();

        assert!(vec.remove_label_values(&["v1", "v2"]).is_err());
        vec.with_label_values(&["v1", "v2"]).inc();
        assert!(vec.remove_label_values(&["v1", "v2"]).is_ok());

        vec.with_label_values(&["v1", "v2"]).inc();
        assert!(vec.remove_label_values(&["v1"]).is_err());
        assert!(vec.remove_label_values(&["v1", "v3"]).is_err());
    }

    #[test]
    fn test_counter_vec_with_owned_label_values() {
        let vec = CounterVec::new(
            Opts::new("test_vec", "test counter vec help"),
            &["l1", "l2"],
        )
        .unwrap();

        let v1 = "v1".to_string();
        let v2 = "v2".to_string();
        let v3 = "v3".to_string();

        assert!(vec.remove_label_values(&[v1.clone(), v2.clone()]).is_err());
        vec.with_label_values(&[v1.clone(), v2.clone()]).inc();
        assert!(vec.remove_label_values(&[v1.clone(), v2.clone()]).is_ok());

        vec.with_label_values(&[v1.clone(), v2.clone()]).inc();
        assert!(vec.remove_label_values(&[v1.clone()]).is_err());
        assert!(vec.remove_label_values(&[v1.clone(), v3.clone()]).is_err());
    }

    #[test]
    fn test_counter_vec_local() {
        let vec = CounterVec::new(
            Opts::new("test_vec_local", "test counter vec help"),
            &["l1", "l2"],
        )
        .unwrap();
        let mut local_vec_1 = vec.local();
        let mut local_vec_2 = local_vec_1.clone();

        assert!(local_vec_1.remove_label_values(&["v1", "v2"]).is_err());

        local_vec_1.with_label_values(&["v1", "v2"]).inc_by(23.0);
        assert!((local_vec_1.with_label_values(&["v1", "v2"]).get() - 23.0) <= f64::EPSILON);
        assert!((vec.with_label_values(&["v1", "v2"]).get() - 0.0) <= f64::EPSILON);

        local_vec_1.flush();
        assert!((local_vec_1.with_label_values(&["v1", "v2"]).get() - 0.0) <= f64::EPSILON);
        assert!((vec.with_label_values(&["v1", "v2"]).get() - 23.0) <= f64::EPSILON);

        local_vec_1.flush();
        assert!((local_vec_1.with_label_values(&["v1", "v2"]).get() - 0.0) <= f64::EPSILON);
        assert!((vec.with_label_values(&["v1", "v2"]).get() - 23.0) <= f64::EPSILON);

        local_vec_1.with_label_values(&["v1", "v2"]).inc_by(11.0);
        assert!((local_vec_1.with_label_values(&["v1", "v2"]).get() - 11.0) <= f64::EPSILON);
        assert!((vec.with_label_values(&["v1", "v2"]).get() - 23.0) <= f64::EPSILON);

        local_vec_1.flush();
        assert!((local_vec_1.with_label_values(&["v1", "v2"]).get() - 0.0) <= f64::EPSILON);
        assert!((vec.with_label_values(&["v1", "v2"]).get() - 34.0) <= f64::EPSILON);

        // When calling `remove_label_values`, it is "flushed" immediately.
        assert!(local_vec_1.remove_label_values(&["v1", "v2"]).is_ok());
        assert!((local_vec_1.with_label_values(&["v1", "v2"]).get() - 0.0) <= f64::EPSILON);
        assert!((vec.with_label_values(&["v1", "v2"]).get() - 0.0) <= f64::EPSILON);

        local_vec_1.with_label_values(&["v1", "v2"]).inc();
        assert!(local_vec_1.remove_label_values(&["v1"]).is_err());
        assert!(local_vec_1.remove_label_values(&["v1", "v3"]).is_err());

        local_vec_1.with_label_values(&["v1", "v2"]).inc_by(13.0);
        assert!((local_vec_1.with_label_values(&["v1", "v2"]).get() - 14.0) <= f64::EPSILON);
        assert!((vec.with_label_values(&["v1", "v2"]).get() - 0.0) <= f64::EPSILON);

        local_vec_2.with_label_values(&["v1", "v2"]).inc_by(7.0);
        assert!((local_vec_2.with_label_values(&["v1", "v2"]).get() - 7.0) <= f64::EPSILON);

        local_vec_1.flush();
        local_vec_2.flush();
        assert!((vec.with_label_values(&["v1", "v2"]).get() - 21.0) <= f64::EPSILON);

        local_vec_1.flush();
        local_vec_2.flush();
        assert!((vec.with_label_values(&["v1", "v2"]).get() - 21.0) <= f64::EPSILON);
    }

    #[test]
    fn test_int_counter_vec_local() {
        let vec = IntCounterVec::new(Opts::new("foo", "bar"), &["l1", "l2"]).unwrap();
        let mut local_vec_1 = vec.local();
        assert!(local_vec_1.remove_label_values(&["v1", "v2"]).is_err());

        local_vec_1.with_label_values(&["v1", "v2"]).inc_by(23);
        assert_eq!(local_vec_1.with_label_values(&["v1", "v2"]).get(), 23);
        assert_eq!(vec.with_label_values(&["v1", "v2"]).get(), 0);

        local_vec_1.flush();
        assert_eq!(local_vec_1.with_label_values(&["v1", "v2"]).get(), 0);
        assert_eq!(vec.with_label_values(&["v1", "v2"]).get(), 23);

        local_vec_1.flush();
        assert_eq!(local_vec_1.with_label_values(&["v1", "v2"]).get(), 0);
        assert_eq!(vec.with_label_values(&["v1", "v2"]).get(), 23);

        local_vec_1.with_label_values(&["v1", "v2"]).inc_by(11);
        assert_eq!(local_vec_1.with_label_values(&["v1", "v2"]).get(), 11);
        assert_eq!(vec.with_label_values(&["v1", "v2"]).get(), 23);

        local_vec_1.flush();
        assert_eq!(local_vec_1.with_label_values(&["v1", "v2"]).get(), 0);
        assert_eq!(vec.with_label_values(&["v1", "v2"]).get(), 34);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_counter_negative_inc() {
        let counter = Counter::new("foo", "bar").unwrap();
        counter.inc_by(-42.0);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_local_counter_negative_inc() {
        let counter = Counter::new("foo", "bar").unwrap();
        let local = counter.local();
        local.inc_by(-42.0);
    }
}
