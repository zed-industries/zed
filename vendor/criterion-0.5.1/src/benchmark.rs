use crate::{PlotConfiguration, SamplingMode};
use std::time::Duration;

// TODO: Move the benchmark config stuff to a separate module for easier use.

/// Struct containing all of the configuration options for a benchmark.
pub struct BenchmarkConfig {
    pub confidence_level: f64,
    pub measurement_time: Duration,
    pub noise_threshold: f64,
    pub nresamples: usize,
    pub sample_size: usize,
    pub significance_level: f64,
    pub warm_up_time: Duration,
    pub sampling_mode: SamplingMode,
    pub quick_mode: bool,
}

/// Struct representing a partially-complete per-benchmark configuration.
#[derive(Clone, Default)]
pub(crate) struct PartialBenchmarkConfig {
    pub(crate) confidence_level: Option<f64>,
    pub(crate) measurement_time: Option<Duration>,
    pub(crate) noise_threshold: Option<f64>,
    pub(crate) nresamples: Option<usize>,
    pub(crate) sample_size: Option<usize>,
    pub(crate) significance_level: Option<f64>,
    pub(crate) warm_up_time: Option<Duration>,
    pub(crate) sampling_mode: Option<SamplingMode>,
    pub(crate) quick_mode: Option<bool>,
    pub(crate) plot_config: PlotConfiguration,
}

impl PartialBenchmarkConfig {
    pub(crate) fn to_complete(&self, defaults: &BenchmarkConfig) -> BenchmarkConfig {
        BenchmarkConfig {
            confidence_level: self.confidence_level.unwrap_or(defaults.confidence_level),
            measurement_time: self.measurement_time.unwrap_or(defaults.measurement_time),
            noise_threshold: self.noise_threshold.unwrap_or(defaults.noise_threshold),
            nresamples: self.nresamples.unwrap_or(defaults.nresamples),
            sample_size: self.sample_size.unwrap_or(defaults.sample_size),
            significance_level: self
                .significance_level
                .unwrap_or(defaults.significance_level),
            warm_up_time: self.warm_up_time.unwrap_or(defaults.warm_up_time),
            sampling_mode: self.sampling_mode.unwrap_or(defaults.sampling_mode),
            quick_mode: self.quick_mode.unwrap_or(defaults.quick_mode),
        }
    }
}
