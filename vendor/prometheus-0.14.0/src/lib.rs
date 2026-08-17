// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/*!
The Rust client library for [Prometheus](https://prometheus.io/).

Use of this library involves a few core concepts:

* [`Metric`s](core/trait.Metric.html) like [`Counter`s](type.Counter.html) that
  represent information about your system.

* A [`Registry`](struct.Registry.html) that [`Metric`s](core/trait.Metric.html)
  are registered with.

* An endpoint that calls [`gather`](fn.gather.html) which returns
  [`MetricFamily`s](proto/struct.MetricFamily.html) through an
  [`Encoder`](trait.Encoder.html).


# Basic Example

```rust
use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};

// Create a Counter.
let counter_opts = Opts::new("test_counter", "test counter help");
let counter = Counter::with_opts(counter_opts).unwrap();

// Create a Registry and register Counter.
let r = Registry::new();
r.register(Box::new(counter.clone())).unwrap();

// Inc.
counter.inc();

// Gather the metrics.
let mut buffer = vec![];
let encoder = TextEncoder::new();
let metric_families = r.gather();
encoder.encode(&metric_families, &mut buffer).unwrap();

// Output to the standard output.
println!("{}", String::from_utf8(buffer).unwrap());
```

You can find more examples within
[`/examples`](https://github.com/tikv/rust-prometheus/tree/master/examples).


# Static Metrics

This crate supports staticly built metrics. You can use it with
[`lazy_static`](https://docs.rs/lazy_static/) to quickly build up and collect
some metrics.

```rust
use prometheus::{self, IntCounter, TextEncoder, Encoder};

use lazy_static::lazy_static;
use prometheus::register_int_counter;

lazy_static! {
    static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!("highfives", "Number of high fives received").unwrap();
}

HIGH_FIVE_COUNTER.inc();
assert_eq!(HIGH_FIVE_COUNTER.get(), 1);
```

By default, this registers with a default registry. To make a report, you can call
[`gather`](fn.gather.html). This will return a family of metrics you can then feed through an
[`Encoder`](trait.Encoder.html) and report to Promethus.

```
# use prometheus::IntCounter;
use prometheus::{self, TextEncoder, Encoder};

use lazy_static::lazy_static;
use prometheus::register_int_counter;

// Register & measure some metrics.
# lazy_static! {
#     static ref HIGH_FIVE_COUNTER: IntCounter =
#        register_int_counter!("highfives", "Number of high fives received").unwrap();
# }
# HIGH_FIVE_COUNTER.inc();

let mut buffer = Vec::new();
let encoder = TextEncoder::new();

// Gather the metrics.
let metric_families = prometheus::gather();
// Encode them to send.
encoder.encode(&metric_families, &mut buffer).unwrap();

let output = String::from_utf8(buffer.clone()).unwrap();
const EXPECTED_OUTPUT: &'static str = "# HELP highfives Number of high fives received\n# TYPE highfives counter\nhighfives 1\n";
assert!(output.starts_with(EXPECTED_OUTPUT));
```

See [prometheus_static_metric](https://docs.rs/prometheus-static-metric) for
additional functionality.


# Features

This library supports four features:

* `gen`: To generate protobuf client with the latest protobuf version instead of
  using the pre-generated client.
* `nightly`: Enable nightly only features.
* `process`: For collecting process info.
* `push`: Enable push support.

*/

#![allow(
    clippy::needless_pass_by_value,
    clippy::new_without_default,
    clippy::new_ret_no_self
)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

/// Protocol buffers format of metrics.
#[cfg(feature = "protobuf")]
#[allow(warnings)]
#[rustfmt::skip]
#[path = "../proto/proto_model.rs"]
pub mod proto;

#[cfg(not(feature = "protobuf"))]
#[path = "plain_model.rs"]
pub mod proto;

#[cfg(feature = "protobuf")]
mod proto_ext;

#[macro_use]
mod macros;
mod atomic64;
mod auto_flush;
mod counter;
mod desc;
mod encoder;
mod errors;
mod gauge;
mod histogram;
mod metrics;
mod nohash;
mod pulling_gauge;
#[cfg(feature = "push")]
mod push;
mod registry;
mod value;
mod vec;

// Public for generated code.
#[doc(hidden)]
pub mod timer;

#[cfg(all(feature = "process", target_os = "linux"))]
pub mod process_collector;

pub mod local {
    /*!

    Unsync local metrics, provides better performance.

    */
    pub use super::counter::{
        CounterWithValueType, LocalCounter, LocalCounterVec, LocalIntCounter, LocalIntCounterVec,
    };
    pub use super::histogram::{LocalHistogram, LocalHistogramTimer, LocalHistogramVec};
    pub use super::metrics::{LocalMetric, MayFlush};

    pub use super::auto_flush::{
        AFLocalCounter, AFLocalHistogram, CounterDelegator, HistogramDelegator,
    };
}

pub mod core {
    /*!

    Core traits and types.

    */

    pub use super::atomic64::*;
    pub use super::counter::{
        GenericCounter, GenericCounterVec, GenericLocalCounter, GenericLocalCounterVec,
    };
    pub use super::desc::{Desc, Describer};
    pub use super::gauge::{GenericGauge, GenericGaugeVec};
    pub use super::metrics::{Collector, Metric, Opts};
    pub use super::vec::{MetricVec, MetricVecBuilder};
}

pub use self::counter::{Counter, CounterVec, IntCounter, IntCounterVec};
pub use self::encoder::Encoder;
#[cfg(feature = "protobuf")]
pub use self::encoder::ProtobufEncoder;
pub use self::encoder::TextEncoder;
#[cfg(feature = "protobuf")]
pub use self::encoder::PROTOBUF_FORMAT;
pub use self::encoder::TEXT_FORMAT;
pub use self::errors::{Error, Result};
pub use self::gauge::{Gauge, GaugeVec, IntGauge, IntGaugeVec};
pub use self::histogram::DEFAULT_BUCKETS;
pub use self::histogram::{exponential_buckets, linear_buckets};
pub use self::histogram::{Histogram, HistogramOpts, HistogramTimer, HistogramVec};
pub use self::metrics::Opts;
pub use self::pulling_gauge::PullingGauge;
#[cfg(feature = "push")]
pub use self::push::{
    hostname_grouping_key, push_add_collector, push_add_metrics, push_collector, push_metrics,
    BasicAuthentication,
};
pub use self::registry::Registry;
pub use self::registry::{default_registry, gather, register, unregister};
