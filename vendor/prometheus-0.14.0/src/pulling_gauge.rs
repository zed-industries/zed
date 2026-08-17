use std::{collections::HashMap, fmt, sync::Arc};

use crate::{
    core::Collector,
    proto::{Gauge, Metric, MetricFamily, MetricType},
};

/// A [Gauge] that returns the value from a provided function on every collect run.
///
/// This metric is the equivalant of Go's
/// <https://pkg.go.dev/github.com/prometheus/client_golang@v1.11.0/prometheus#GaugeFunc>
///
/// # Examples
/// ```
/// # use prometheus::{Registry, PullingGauge};
/// # // We are stubbing out std::thread::available_parallelism since it's not available in the
/// # // oldest Rust version that we support.
/// # fn available_parallelism() -> f64 { 0.0 }
///
/// let registry = Registry::new();
/// let gauge = PullingGauge::new(
///     "available_parallelism",
///     "The available parallelism, usually the numbers of logical cores.",
///     Box::new(|| available_parallelism())
/// ).unwrap();
/// registry.register(Box::new(gauge));
/// ```
#[derive(Clone)]
pub struct PullingGauge {
    desc: crate::core::Desc,
    value: Arc<Box<dyn Fn() -> f64 + Send + Sync>>,
}

impl fmt::Debug for PullingGauge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PullingGauge")
            .field("desc", &self.desc)
            .field("value", &"<opaque>")
            .finish()
    }
}

impl PullingGauge {
    /// Create a new [`PullingGauge`].
    pub fn new<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
        value: Box<dyn Fn() -> f64 + Send + Sync>,
    ) -> crate::Result<Self> {
        Ok(PullingGauge {
            value: Arc::new(value),
            desc: crate::core::Desc::new(name.into(), help.into(), Vec::new(), HashMap::new())?,
        })
    }

    fn metric(&self) -> Metric {
        let mut gauge = Gauge::default();
        let getter = &self.value;
        gauge.set_value(getter());

        Metric::from_gauge(gauge)
    }
}

impl Collector for PullingGauge {
    fn desc(&self) -> Vec<&crate::core::Desc> {
        vec![&self.desc]
    }

    fn collect(&self) -> Vec<crate::proto::MetricFamily> {
        let mut m = MetricFamily::default();
        m.set_name(self.desc.fq_name.clone());
        m.set_help(self.desc.help.clone());
        m.set_field_type(MetricType::GAUGE);
        m.set_metric(vec![self.metric()]);
        vec![m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::Collector;
    #[cfg(feature = "protobuf")]
    use crate::proto_ext::MessageFieldExt;

    #[test]
    fn test_pulling_gauge() {
        const VALUE: f64 = 10.0;

        let gauge =
            PullingGauge::new("test_gauge", "Purely for testing", Box::new(|| VALUE)).unwrap();

        let metrics = gauge.collect();
        assert_eq!(metrics.len(), 1);

        assert_eq!(VALUE, metrics[0].get_metric()[0].get_gauge().get_value());
    }
}
