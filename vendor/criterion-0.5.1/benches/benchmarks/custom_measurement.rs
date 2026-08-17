use criterion::{
    black_box, criterion_group,
    measurement::{Measurement, ValueFormatter},
    Criterion, Throughput,
};
use std::time::{Duration, Instant};

struct HalfSecFormatter;
impl ValueFormatter for HalfSecFormatter {
    fn format_value(&self, value: f64) -> String {
        // The value will be in nanoseconds so we have to convert to half-seconds.
        format!("{} s/2", value * 2f64 * 10f64.powi(-9))
    }

    fn format_throughput(&self, throughput: &Throughput, value: f64) -> String {
        match *throughput {
            Throughput::Bytes(bytes) | Throughput::BytesDecimal(bytes) => {
                format!("{} b/s/2", (bytes as f64) / (value * 2f64 * 10f64.powi(-9)))
            }
            Throughput::Elements(elems) => format!(
                "{} elem/s/2",
                (elems as f64) / (value * 2f64 * 10f64.powi(-9))
            ),
        }
    }

    fn scale_values(&self, _typical: f64, values: &mut [f64]) -> &'static str {
        for val in values {
            *val *= 2f64 * 10f64.powi(-9);
        }

        "s/2"
    }

    fn scale_throughputs(
        &self,
        _typical: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        match *throughput {
            Throughput::Bytes(bytes) | Throughput::BytesDecimal(bytes) => {
                for val in values {
                    *val = (bytes as f64) / (*val * 2f64 * 10f64.powi(-9))
                }

                "b/s/2"
            }
            Throughput::Elements(elems) => {
                for val in values {
                    *val = (elems as f64) / (*val * 2f64 * 10f64.powi(-9))
                }

                "elem/s/2"
            }
        }
    }

    fn scale_for_machines(&self, values: &mut [f64]) -> &'static str {
        for val in values {
            *val *= 2f64 * 10f64.powi(-9);
        }

        "s/2"
    }
}

const NANOS_PER_SEC: u64 = 1_000_000_000;

/// Silly "measurement" that is really just wall-clock time reported in half-seconds.
struct HalfSeconds;
impl Measurement for HalfSeconds {
    type Intermediate = Instant;
    type Value = Duration;

    fn start(&self) -> Self::Intermediate {
        Instant::now()
    }
    fn end(&self, i: Self::Intermediate) -> Self::Value {
        i.elapsed()
    }
    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        *v1 + *v2
    }
    fn zero(&self) -> Self::Value {
        Duration::from_secs(0)
    }
    fn to_f64(&self, val: &Self::Value) -> f64 {
        let nanos = val.as_secs() * NANOS_PER_SEC + u64::from(val.subsec_nanos());
        nanos as f64
    }
    fn formatter(&self) -> &dyn ValueFormatter {
        &HalfSecFormatter
    }
}

fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
    }
}

fn fibonacci_cycles(criterion: &mut Criterion<HalfSeconds>) {
    criterion.bench_function("fibonacci_custom_measurement", |bencher| {
        bencher.iter(|| fibonacci_slow(black_box(10)))
    });
}

fn alternate_measurement() -> Criterion<HalfSeconds> {
    Criterion::default().with_measurement(HalfSeconds)
}

criterion_group! {
    name = benches;
    config = alternate_measurement();
    targets = fibonacci_cycles
}
