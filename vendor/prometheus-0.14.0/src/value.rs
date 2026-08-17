// Copyright 2014 The Prometheus Authors
// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use crate::atomic64::{Atomic, Number};
use crate::desc::{Desc, Describer};
use crate::errors::{Error, Result};
use crate::proto::{Counter, Gauge, LabelPair, Metric, MetricFamily, MetricType};

/// `ValueType` is an enumeration of metric types that represent a simple value
/// for [`Counter`] and [`Gauge`].
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ValueType {
    Counter,
    Gauge,
}

impl ValueType {
    /// `metric_type` returns the corresponding proto metric type.
    pub fn metric_type(self) -> MetricType {
        match self {
            ValueType::Counter => MetricType::COUNTER,
            ValueType::Gauge => MetricType::GAUGE,
        }
    }
}

/// A generic metric for [`Counter`] and [`Gauge`].
/// Its effective type is determined by `ValueType`. This is a low-level
/// building block used by the library to back the implementations of
/// [`Counter`] and [`Gauge`].
#[derive(Debug)]
pub struct Value<P: Atomic> {
    pub desc: Desc,
    pub val: P,
    pub val_type: ValueType,
    pub label_pairs: Vec<LabelPair>,
}

impl<P: Atomic> Value<P> {
    pub fn new<D: Describer, V: AsRef<str>>(
        describer: &D,
        val_type: ValueType,
        val: P::T,
        label_values: &[V],
    ) -> Result<Self> {
        let desc = describer.describe()?;
        let label_pairs = make_label_pairs(&desc, label_values)?;

        Ok(Self {
            desc,
            val: P::new(val),
            val_type,
            label_pairs,
        })
    }

    #[inline]
    pub fn get(&self) -> P::T {
        self.val.get()
    }

    #[inline]
    pub fn set(&self, val: P::T) {
        self.val.set(val);
    }

    #[inline]
    pub fn inc_by(&self, val: P::T) {
        self.val.inc_by(val);
    }

    #[inline]
    pub fn inc(&self) {
        self.inc_by(P::T::from_i64(1));
    }

    #[inline]
    pub fn dec(&self) {
        self.dec_by(P::T::from_i64(1));
    }

    #[inline]
    pub fn dec_by(&self, val: P::T) {
        self.val.dec_by(val)
    }

    pub fn metric(&self) -> Metric {
        let mut m = Metric::from_label(self.label_pairs.clone());

        let val = self.get();
        match self.val_type {
            ValueType::Counter => {
                let mut counter = Counter::default();
                counter.set_value(val.into_f64());
                m.set_counter(counter);
            }
            ValueType::Gauge => {
                let mut gauge = Gauge::default();
                gauge.set_value(val.into_f64());
                m.set_gauge(gauge);
            }
        }

        m
    }

    pub fn collect(&self) -> MetricFamily {
        let mut m = MetricFamily::default();
        m.set_name(self.desc.fq_name.clone());
        m.set_help(self.desc.help.clone());
        m.set_field_type(self.val_type.metric_type());
        m.set_metric(vec![self.metric()]);
        m
    }
}

pub fn make_label_pairs<V: AsRef<str>>(desc: &Desc, label_values: &[V]) -> Result<Vec<LabelPair>> {
    if desc.variable_labels.len() != label_values.len() {
        return Err(Error::InconsistentCardinality {
            expect: desc.variable_labels.len(),
            got: label_values.len(),
        });
    }

    let total_len = desc.variable_labels.len() + desc.const_label_pairs.len();
    if total_len == 0 {
        return Ok(vec![]);
    }

    if desc.variable_labels.is_empty() {
        return Ok(desc.const_label_pairs.clone());
    }

    let mut label_pairs = Vec::with_capacity(total_len);
    for (i, n) in desc.variable_labels.iter().enumerate() {
        let mut label_pair = LabelPair::default();
        label_pair.set_name(n.clone());
        label_pair.set_value(label_values[i].as_ref().to_owned());
        label_pairs.push(label_pair);
    }

    for label_pair in &desc.const_label_pairs {
        label_pairs.push(label_pair.clone());
    }
    label_pairs.sort();
    Ok(label_pairs)
}
