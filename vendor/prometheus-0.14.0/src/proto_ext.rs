use protobuf::{EnumOrUnknown, MessageField};

use crate::proto::{
    Bucket, Counter, Gauge, Histogram, LabelPair, Metric, MetricFamily, MetricType, Quantile,
    Summary,
};

impl Metric {
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

    #[deprecated(since = "0.14.0", note = "Please use `.timestamp_ms()` instead")]
    /// Returns the timestamp of this metric.
    pub fn get_timestamp_ms(&self) -> i64 {
        self.timestamp_ms()
    }

    /// Returns the summary of this metric.
    pub fn get_summary(&self) -> &MessageField<Summary> {
        &self.summary
    }

    /// Sets the summary of this metric to the specified summary.
    pub fn set_summary(&mut self, summary: Summary) {
        self.summary = summary.into();
    }

    /// Returns the value of the counter for this metric.
    pub fn get_counter(&self) -> &MessageField<Counter> {
        &self.counter
    }

    /// Sets the counter of this metric to the specified counter.
    pub fn set_counter(&mut self, counter: Counter) {
        self.counter = counter.into();
    }

    /// Returns all label pairs associated with this metric.
    pub fn get_label(&self) -> &[LabelPair] {
        &self.label
    }

    /// Sets the label pairs associated with this metric.
    pub fn set_label(&mut self, label: Vec<LabelPair>) {
        self.label = label;
    }

    /// Returns all label pairs associated with ownership.
    pub fn take_label(&mut self) -> Vec<LabelPair> {
        std::mem::take(&mut self.label)
    }

    /// Returns the gauge of this metric.
    pub fn get_gauge(&self) -> &MessageField<Gauge> {
        &self.gauge
    }

    /// Sets the gauge of this metric to the specified gauge.
    pub fn set_gauge(&mut self, gauge: Gauge) {
        self.gauge = gauge.into();
    }

    /// Returns the histogram of this metric.
    pub fn get_histogram(&self) -> &MessageField<Histogram> {
        &self.histogram
    }

    /// Sets the histogram of this metric to the specified histogram.
    pub fn set_histogram(&mut self, histogram: Histogram) {
        self.histogram = histogram.into();
    }
}

impl MetricFamily {
    #[deprecated(since = "0.14.0", note = "Please use `.name()` instead")]
    /// Returns the name of this metric family.
    pub fn get_name(&self) -> &str {
        self.name()
    }

    #[deprecated(since = "0.14.0", note = "Please use `.help()` instead")]
    /// Returns the help text of this metric family.
    pub fn get_help(&self) -> &str {
        self.help()
    }

    /// Sets the metric for this metric family (replaces any existing metrics).
    pub fn set_metric(&mut self, metric: Vec<Metric>) {
        self.metric = metric;
    }

    /// Returns the type of this metric family.
    pub fn get_field_type(&self) -> MetricType {
        self.type_()
    }

    /// Sets the type of this metric family.
    pub fn set_field_type(&mut self, t: MetricType) {
        self.type_ = t.into();
    }

    /// Returns all metrics in this metric family.
    pub fn get_metric(&self) -> &[Metric] {
        &self.metric
    }

    /// Returns all metrics in this metric family mutably.
    pub fn mut_metric(&mut self) -> &mut Vec<Metric> {
        &mut self.metric
    }

    /// Returns all metrics in this metric family with taking ownership.
    pub fn take_metric(&mut self) -> Vec<Metric> {
        std::mem::take(&mut self.metric)
    }
}

impl Summary {
    /// Sets the quantiles for this summary.
    pub fn set_quantile(&mut self, quantiles: Vec<Quantile>) {
        self.quantile = quantiles;
    }

    /// Returns the quantiles of this summary.
    pub fn get_quantile(&self) -> &[Quantile] {
        &self.quantile
    }

    #[deprecated(since = "0.14.0", note = "Please use `.sample_count()` instead")]
    /// Returns the sample count of this summary.
    pub fn get_sample_count(&self) -> u64 {
        self.sample_count()
    }

    #[deprecated(since = "0.14.0", note = "Please use `.sample_sum()` instead")]
    /// Returns the sample sum of this summary.
    pub fn get_sample_sum(&self) -> f64 {
        self.sample_sum()
    }
}

impl Quantile {
    #[deprecated(since = "0.14.0", note = "Please use `.quantile()` instead")]
    /// Returns the quantile of this quantile.
    pub fn get_quantile(&self) -> f64 {
        self.quantile()
    }

    #[deprecated(since = "0.14.0", note = "Please use `.value()` instead")]
    /// Returns the value of this quantile.
    pub fn get_value(&self) -> f64 {
        self.value()
    }
}

pub trait MessageFieldExt {
    /// Returns the value of the wrapped gauge.
    #[allow(dead_code)]
    fn get_value(&self) -> f64;
}

impl MessageFieldExt for MessageField<Gauge> {
    fn get_value(&self) -> f64 {
        self.value()
    }
}

impl MessageFieldExt for MessageField<Counter> {
    fn get_value(&self) -> f64 {
        self.value()
    }
}

impl Histogram {
    /// Returns the sample count of this histogram.
    pub fn get_sample_count(&self) -> u64 {
        self.sample_count.unwrap_or_default()
    }

    /// Returns the sample sum of this histogram.
    pub fn get_sample_sum(&self) -> f64 {
        self.sample_sum.unwrap_or_default()
    }

    /// Returns all buckets in this histogram.
    pub fn get_bucket(&self) -> &[Bucket] {
        &self.bucket
    }

    /// Sets the buckets of this histogram.
    pub fn set_bucket(&mut self, bucket: Vec<Bucket>) {
        self.bucket = bucket;
    }
}

impl Bucket {
    #[deprecated(since = "0.14.0", note = "Please use `.cumulative_count()` instead")]
    /// Returns the cumulative count of this bucket.
    pub fn get_cumulative_count(&self) -> u64 {
        self.cumulative_count()
    }

    #[deprecated(since = "0.14.0", note = "Please use `.upper_bound()` instead")]
    /// Returns the upper bound of this bucket.
    pub fn get_upper_bound(&self) -> f64 {
        self.upper_bound()
    }
}

impl LabelPair {
    #[deprecated(since = "0.14.0", note = "Please use `.value()` instead")]
    /// Returns the value of this label pair.
    pub fn get_value(&self) -> &str {
        self.value()
    }

    #[deprecated(since = "0.14.0", note = "Please use `.name()` instead")]
    /// Returns the name of this label pair.
    pub fn get_name(&self) -> &str {
        self.name()
    }
}

impl From<Counter> for MessageField<Counter> {
    fn from(value: Counter) -> Self {
        MessageField::some(value)
    }
}

impl From<Gauge> for MessageField<Gauge> {
    fn from(value: Gauge) -> Self {
        MessageField::some(value)
    }
}

impl From<Histogram> for MessageField<Histogram> {
    fn from(value: Histogram) -> Self {
        MessageField::some(value)
    }
}

impl From<Summary> for MessageField<Summary> {
    fn from(value: Summary) -> Self {
        MessageField::some(value)
    }
}

impl From<MetricType> for Option<EnumOrUnknown<MetricType>> {
    fn from(value: MetricType) -> Self {
        Some(EnumOrUnknown::from(value))
    }
}
