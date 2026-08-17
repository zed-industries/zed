// Copyright 2014 The Prometheus Authors
// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::cmp::{Eq, Ord, Ordering, PartialOrd};
use std::collections::HashMap;

use crate::desc::{Desc, Describer};
use crate::errors::Result;
use crate::proto::{self, LabelPair};
use crate::timer;
use std::cell::Cell;

pub const SEPARATOR_BYTE: u8 = 0xFF;

/// An interface for collecting metrics.
pub trait Collector: Sync + Send {
    /// Return descriptors for metrics.
    fn desc(&self) -> Vec<&Desc>;

    /// Collect metrics.
    fn collect(&self) -> Vec<proto::MetricFamily>;
}

/// An interface models a single sample value with its meta data being exported to Prometheus.
pub trait Metric: Sync + Send + Clone {
    /// Return the protocol Metric.
    fn metric(&self) -> proto::Metric;
}

/// An interface models a Metric only usable in single thread environment.
pub trait LocalMetric {
    /// Flush the local metrics to the global one.
    fn flush(&self);
}

/// An interface models a LocalMetric with try to flush functions.
/// Not intend to be implemented by user manually, used in macro generated code.
pub trait MayFlush: LocalMetric {
    /// If the LocalMetric is already flushed in last `flush_interval_sec` seconds, then do nothing,
    /// else flush and update last flush time.
    fn try_flush(&self, last_flush: &Cell<u64>, flush_interval_millis: u64) {
        let now = timer::recent_millis();
        let last_tick = last_flush.get();
        if now < last_tick + flush_interval_millis {
            return;
        }
        self.flush();
        last_flush.set(now);
    }

    /// Open to implementation to fill try_flush parameters
    fn may_flush(&self);
}

/// A struct that bundles the options for creating most [`Metric`] types.
#[derive(Debug, Clone)]
pub struct Opts {
    /// namespace, subsystem, and name are components of the fully-qualified
    /// name of the [`Metric`] (created by joining these components with
    /// "_"). Only Name is mandatory, the others merely help structuring the
    /// name. Note that the fully-qualified name of the metric must be a
    /// valid Prometheus metric name.
    pub namespace: String,
    /// namespace, subsystem, and name are components of the fully-qualified
    /// name of the [`Metric`] (created by joining these components with
    /// "_"). Only Name is mandatory, the others merely help structuring the
    /// name. Note that the fully-qualified name of the metric must be a
    /// valid Prometheus metric name.
    pub subsystem: String,
    /// namespace, subsystem, and name are components of the fully-qualified
    /// name of the [`Metric`] (created by joining these components with
    /// "_"). Only Name is mandatory, the others merely help structuring the
    /// name. Note that the fully-qualified name of the metric must be a
    /// valid Prometheus metric name.
    pub name: String,

    /// help provides information about this metric. Mandatory!
    ///
    /// Metrics with the same fully-qualified name must have the same Help
    /// string.
    pub help: String,

    /// const_labels are used to attach fixed labels to this metric. Metrics
    /// with the same fully-qualified name must have the same label names in
    /// their ConstLabels.
    ///
    /// Note that in most cases, labels have a value that varies during the
    /// lifetime of a process. Those labels are usually managed with a metric
    /// vector collector (like CounterVec, GaugeVec). ConstLabels
    /// serve only special purposes. One is for the special case where the
    /// value of a label does not change during the lifetime of a process,
    /// e.g. if the revision of the running binary is put into a
    /// label. Another, more advanced purpose is if more than one [`Collector`]
    /// needs to collect Metrics with the same fully-qualified name. In that
    /// case, those Metrics must differ in the values of their
    /// ConstLabels. See the [`Collector`] examples.
    ///
    /// If the value of a label never changes (not even between binaries),
    /// that label most likely should not be a label at all (but part of the
    /// metric name).
    pub const_labels: HashMap<String, String>,

    /// variable_labels contains names of labels for which the metric maintains
    /// variable values. Metrics with the same fully-qualified name must have
    /// the same label names in their variable_labels.
    ///
    /// Note that variable_labels is used in `MetricVec`. To create a single
    /// metric must leave it empty.
    pub variable_labels: Vec<String>,
}

impl Opts {
    /// `new` creates the Opts with the `name` and `help` arguments.
    pub fn new<S1: Into<String>, S2: Into<String>>(name: S1, help: S2) -> Opts {
        Opts {
            namespace: "".to_owned(),
            subsystem: "".to_owned(),
            name: name.into(),
            help: help.into(),
            const_labels: HashMap::new(),
            variable_labels: Vec::new(),
        }
    }

    /// `namespace` sets the namespace.
    pub fn namespace<S: Into<String>>(mut self, namespace: S) -> Self {
        self.namespace = namespace.into();
        self
    }

    /// `subsystem` sets the sub system.
    pub fn subsystem<S: Into<String>>(mut self, subsystem: S) -> Self {
        self.subsystem = subsystem.into();
        self
    }

    /// `const_labels` sets the const labels.
    pub fn const_labels(mut self, const_labels: HashMap<String, String>) -> Self {
        self.const_labels = const_labels;
        self
    }

    /// `const_label` adds a const label.
    pub fn const_label<S1: Into<String>, S2: Into<String>>(mut self, name: S1, value: S2) -> Self {
        self.const_labels.insert(name.into(), value.into());
        self
    }

    /// `variable_labels` sets the variable labels.
    pub fn variable_labels(mut self, variable_labels: Vec<String>) -> Self {
        self.variable_labels = variable_labels;
        self
    }

    /// `variable_label` adds a variable label.
    pub fn variable_label<S: Into<String>>(mut self, name: S) -> Self {
        self.variable_labels.push(name.into());
        self
    }

    /// `fq_name` returns the fq_name.
    pub fn fq_name(&self) -> String {
        build_fq_name(&self.namespace, &self.subsystem, &self.name)
    }
}

impl Describer for Opts {
    fn describe(&self) -> Result<Desc> {
        Desc::new(
            self.fq_name(),
            self.help.clone(),
            self.variable_labels.clone(),
            self.const_labels.clone(),
        )
    }
}

impl Ord for LabelPair {
    fn cmp(&self, other: &LabelPair) -> Ordering {
        self.name().cmp(other.name())
    }
}

impl Eq for LabelPair {}

impl PartialOrd for LabelPair {
    fn partial_cmp(&self, other: &LabelPair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// `build_fq_name` joins the given three name components by "_". Empty name
/// components are ignored. If the name parameter itself is empty, an empty
/// string is returned, no matter what. [`Metric`] implementations included in this
/// library use this function internally to generate the fully-qualified metric
/// name from the name component in their Opts. Users of the library will only
/// need this function if they implement their own [`Metric`] or instantiate a Desc
/// directly.
fn build_fq_name(namespace: &str, subsystem: &str, name: &str) -> String {
    if name.is_empty() {
        return "".to_owned();
    }

    if !namespace.is_empty() && !subsystem.is_empty() {
        return format!("{}_{}_{}", namespace, subsystem, name);
    } else if !namespace.is_empty() {
        return format!("{}_{}", namespace, name);
    } else if !subsystem.is_empty() {
        return format!("{}_{}", subsystem, name);
    }

    name.to_owned()
}

#[cfg(test)]
mod tests {
    use std::cmp::{Ord, Ordering};

    use super::*;
    use crate::proto::LabelPair;

    fn new_label_pair(name: &str, value: &str) -> LabelPair {
        let mut l = LabelPair::default();
        l.set_name(name.to_owned());
        l.set_value(value.to_owned());
        l
    }

    #[test]
    fn test_label_cmp() {
        let tbl = vec![
            ("k1", "k2", Ordering::Less),
            ("k1", "k1", Ordering::Equal),
            ("k1", "k0", Ordering::Greater),
        ];

        for (l1, l2, order) in tbl {
            let lhs = new_label_pair(l1, l1);
            let rhs = new_label_pair(l2, l2);
            assert_eq!(lhs.cmp(&rhs), order);
        }
    }

    #[test]
    fn test_build_fq_name() {
        let tbl = vec![
            ("a", "b", "c", "a_b_c"),
            ("", "b", "c", "b_c"),
            ("a", "", "c", "a_c"),
            ("", "", "c", "c"),
            ("a", "b", "", ""),
            ("a", "", "", ""),
            ("", "b", "", ""),
            (" ", "", "", ""),
        ];

        for (namespace, subsystem, name, res) in tbl {
            assert_eq!(&build_fq_name(namespace, subsystem, name), res);
        }
    }

    #[test]
    fn test_different_generic_types() {
        Opts::new(format!("{}_{}", "string", "label"), "&str_label");
    }
}
