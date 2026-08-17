// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

//! This examples shows how to use multiple and custom registries,
//! and how to perform registration across function boundaries.

use std::collections::HashMap;

use prometheus::{Encoder, IntCounter, Registry};

use lazy_static::lazy_static;

lazy_static! {
    static ref DEFAULT_COUNTER: IntCounter = IntCounter::new("default", "generic counter").unwrap();
    static ref CUSTOM_COUNTER: IntCounter = IntCounter::new("custom", "dedicated counter").unwrap();
}

fn main() {
    // Register default metrics.
    default_metrics(prometheus::default_registry());

    // Register custom metrics to a custom registry.
    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();
    custom_metrics(&custom_registry);

    // Print metrics for the default registry.
    let mut buffer = Vec::<u8>::new();
    let encoder = prometheus::TextEncoder::new();
    encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
    println!("## Default registry");
    println!("{}", String::from_utf8(buffer.clone()).unwrap());

    // Print metrics for the custom registry.
    let mut buffer = Vec::<u8>::new();
    let encoder = prometheus::TextEncoder::new();
    encoder
        .encode(&custom_registry.gather(), &mut buffer)
        .unwrap();
    println!("## Custom registry");
    println!("{}", String::from_utf8(buffer.clone()).unwrap());
}

/// Default metrics, to be collected by the default registry.
fn default_metrics(registry: &Registry) {
    registry
        .register(Box::new(DEFAULT_COUNTER.clone()))
        .unwrap();

    DEFAULT_COUNTER.inc();
    assert_eq!(DEFAULT_COUNTER.get(), 1);
}

/// Custom metrics, to be collected by a dedicated registry.
fn custom_metrics(registry: &Registry) {
    registry.register(Box::new(CUSTOM_COUNTER.clone())).unwrap();

    CUSTOM_COUNTER.inc_by(42);
    assert_eq!(CUSTOM_COUNTER.get(), 42);
}
