# Prometheus Rust client library

[![Build Status](https://github.com/tikv/rust-prometheus/actions/workflows/rust.yml/badge.svg)](https://github.com/tikv/rust-prometheus/actions/workflows/rust.yml)
[![docs.rs](https://docs.rs/prometheus/badge.svg)](https://docs.rs/prometheus)
[![crates.io](https://img.shields.io/crates/v/prometheus.svg)](https://crates.io/crates/prometheus)

This is the [Rust](https://www.rust-lang.org) client library for
[Prometheus](http://prometheus.io). The main data structures and APIs are ported
from [Go client](https://github.com/prometheus/client_golang).

## Documentation

Find the latest documentation at <https://docs.rs/prometheus>.

## Advanced

### Crate features

This crate provides several optional components which can be enabled via [Cargo `[features]`](https://doc.rust-lang.org/cargo/reference/features.html):

- `protobuf`: Protobuf support, enabled by default.

- `gen`: To generate protobuf client with the latest protobuf version instead of
  using the pre-generated client.

- `nightly`: Enable nightly only features.

- `process`: Enable [process metrics](https://prometheus.io/docs/instrumenting/writing_clientlibs/#process-metrics) support.

- `push`: Enable [push metrics](https://prometheus.io/docs/instrumenting/pushing/) support.

### Static Metric

When using a `MetricVec` with label values known at compile time
prometheus-static-metric reduces the overhead of retrieving the concrete
`Metric` from a `MetricVec`.

See [static-metric](./static-metric) directory for details.

## Thanks

- [brian-brazil](https://github.com/brian-brazil)
- [ccmtaylor](https://github.com/ccmtaylor)
- [kamalmarhubi](https://github.com/kamalmarhubi)
- [lucab](https://github.com/lucab)
- [koushiro](https://github.com/koushiro)
