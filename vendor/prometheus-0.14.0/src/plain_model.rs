// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

//! Non-generated version of required structures provided by the protobuf.
//! This version is used when the `protobuf` feature is turned off.

#![allow(missing_docs)]

#[derive(PartialEq, Clone, Default, Debug)]
pub struct LabelPair {
    name: String,
    value: String,
}

impl LabelPair {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> LabelPair {
        Default::default()
    }

    #[deprecated(
        note = "This method is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn set_name(&mut self, v: String) {
        self.name = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.name()` instead")]
    pub fn get_name(&self) -> &str {
        self.name()
    }

    /// Returns the name of this label pair.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_value(&mut self, v: String) {
        self.value = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.value()` instead")]
    pub fn get_value(&self) -> &str {
        self.value()
    }

    /// Returns the value of this label pair.
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Gauge {
    value: f64,
}

impl Gauge {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Gauge {
        Default::default()
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Counter {
    value: f64,
}

impl Counter {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Counter {
        Default::default()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Quantile {
    quantile: f64,
    value: f64,
}

impl Quantile {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Quantile {
        Default::default()
    }

    pub fn set_quantile(&mut self, v: f64) {
        self.quantile = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.quantile()` instead")]
    pub fn get_quantile(&self) -> f64 {
        self.quantile()
    }

    /// Returns the quantile of this quantile.
    pub fn quantile(&self) -> f64 {
        self.quantile
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.value()` instead")]
    pub fn get_value(&self) -> f64 {
        self.value()
    }

    /// Returns the value of this quantile.
    pub fn value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Summary {
    sample_count: u64,
    sample_sum: f64,
    quantile: Vec<Quantile>,
}

impl Summary {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Summary {
        Default::default()
    }

    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.sample_count()` instead")]
    pub fn get_sample_count(&self) -> u64 {
        self.sample_count
    }

    /// Returns the sample count of this summary.
    pub fn sample_count(&self) -> u64 {
        self.sample_count
    }

    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.sample_sum()` instead")]
    pub fn get_sample_sum(&self) -> f64 {
        self.sample_sum()
    }

    /// Returns the sample sum of this summary.
    pub fn sample_sum(&self) -> f64 {
        self.sample_sum
    }

    pub fn set_quantile(&mut self, v: Vec<Quantile>) {
        self.quantile = v;
    }

    pub fn get_quantile(&self) -> &[Quantile] {
        &self.quantile
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Untyped {
    value: f64,
}

impl Untyped {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Untyped {
        Default::default()
    }

    #[deprecated(
        note = "Untyped struct is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    #[deprecated(
        note = "Untyped struct is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn get_value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Histogram {
    sample_count: u64,
    sample_sum: f64,
    bucket: Vec<Bucket>,
}

impl Histogram {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Histogram {
        Default::default()
    }

    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = v;
    }

    pub fn get_sample_count(&self) -> u64 {
        self.sample_count
    }

    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = v;
    }

    pub fn get_sample_sum(&self) -> f64 {
        self.sample_sum
    }

    pub fn set_bucket(&mut self, v: Vec<Bucket>) {
        self.bucket = v;
    }

    pub fn get_bucket(&self) -> &[Bucket] {
        &self.bucket
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Bucket {
    cumulative_count: u64,
    upper_bound: f64,
}

impl Bucket {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Bucket {
        Default::default()
    }

    pub fn set_cumulative_count(&mut self, v: u64) {
        self.cumulative_count = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.cumulative_count()` instead")]
    pub fn get_cumulative_count(&self) -> u64 {
        self.cumulative_count()
    }

    /// Returns the cumulative count of this bucket.
    pub fn cumulative_count(&self) -> u64 {
        self.cumulative_count
    }

    pub fn set_upper_bound(&mut self, v: f64) {
        self.upper_bound = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.upper_bound()` instead")]
    pub fn get_upper_bound(&self) -> f64 {
        self.upper_bound()
    }

    /// Returns the upper bound of this bucket.
    pub fn upper_bound(&self) -> f64 {
        self.upper_bound
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Metric {
    // message fields
    label: Vec<LabelPair>,
    gauge: Gauge,
    counter: Counter,
    summary: Summary,
    untyped: Untyped,
    histogram: Histogram,
    timestamp_ms: i64,
}

impl Metric {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Metric {
        Default::default()
    }

    /// Creates a new metric with the specified label pairs.
    pub fn from_label(label: Vec<LabelPair>) -> Self {
        Metric {
            label,
            ..Default::default()
        }
    }

    /// Creates a new metric with the specified gauge value.
    pub fn from_gauge(gauge: Gauge) -> Self {
        Metric {
            gauge: gauge.into(),
            ..Default::default()
        }
    }

    pub fn set_label(&mut self, v: Vec<LabelPair>) {
        self.label = v;
    }

    pub fn mut_label(&mut self) -> &mut [LabelPair] {
        &mut self.label
    }

    pub fn take_label(&mut self) -> Vec<LabelPair> {
        ::std::mem::replace(&mut self.label, Vec::new())
    }

    pub fn get_label(&self) -> &[LabelPair] {
        &self.label
    }

    pub fn set_gauge(&mut self, v: Gauge) {
        self.gauge = v;
    }

    pub fn get_gauge(&self) -> &Gauge {
        &self.gauge
    }

    pub fn set_counter(&mut self, v: Counter) {
        self.counter = v;
    }

    pub fn get_counter(&self) -> &Counter {
        &self.counter
    }

    pub fn set_summary(&mut self, v: Summary) {
        self.summary = v;
    }

    pub fn get_summary(&self) -> &Summary {
        &self.summary
    }

    #[deprecated(
        note = "This method is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn set_untyped(&mut self, v: Untyped) {
        self.untyped = v;
    }

    #[deprecated(
        note = "This method is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn get_untyped(&self) -> &Untyped {
        &self.untyped
    }

    pub fn set_histogram(&mut self, v: Histogram) {
        self.histogram = v;
    }

    pub fn get_histogram(&self) -> &Histogram {
        &self.histogram
    }

    pub fn set_timestamp_ms(&mut self, v: i64) {
        self.timestamp_ms = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.timestamp_ms()` instead")]
    pub fn get_timestamp_ms(&self) -> i64 {
        self.timestamp_ms()
    }

    /// Returns the timestamp of this metric.
    pub fn timestamp_ms(&self) -> i64 {
        self.timestamp_ms
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
pub enum MetricType {
    COUNTER,
    GAUGE,
    SUMMARY,
    UNTYPED,
    HISTOGRAM,
}

impl Default for MetricType {
    fn default() -> Self {
        MetricType::COUNTER
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct MetricFamily {
    name: String,
    help: String,
    field_type: MetricType,
    metric: Vec<Metric>,
}

impl MetricFamily {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> MetricFamily {
        Default::default()
    }

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn set_name(&mut self, v: String) {
        self.name = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.name()` instead")]
    pub fn get_name(&self) -> &str {
        self.name()
    }

    /// Returns the name of this metric family.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_help(&mut self, v: String) {
        self.help = v;
    }

    #[deprecated(since = "0.14.0", note = "Please use `.help()` instead")]
    pub fn get_help(&self) -> &str {
        self.help()
    }

    /// Returns the help text of this metric family.
    pub fn help(&self) -> &str {
        &self.help
    }

    pub fn set_field_type(&mut self, v: MetricType) {
        self.field_type = v;
    }

    pub fn get_field_type(&self) -> MetricType {
        self.field_type
    }

    #[deprecated(
        note = "This method is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn clear_metric(&mut self) {
        self.metric.clear();
    }

    pub fn set_metric(&mut self, v: Vec<Metric>) {
        self.metric = v;
    }

    pub fn mut_metric(&mut self) -> &mut Vec<Metric> {
        &mut self.metric
    }

    pub fn take_metric(&mut self) -> Vec<Metric> {
        ::std::mem::replace(&mut self.metric, Vec::new())
    }

    pub fn get_metric(&self) -> &[Metric] {
        &self.metric
    }
}
