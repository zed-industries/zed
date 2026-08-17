# kqueue

[![Gitlab
Pipelines](https://gitlab.com/rust-kqueue/rust-kqueue/badges/main/pipeline.svg)](https://gitlab.com/rust-kqueue/rust-kqueue/-/commits/main)

`kqueue(2)` library for rust

`kqueue(2)` is a powerful API in BSDs that allows you to get events based on
fs events, buffer readiness, timers, process events and signals.

This is useful for code that's either BSD-specific, or as a component in an
abstraction over similar APIs in cross-platform code.

## Docs

Docs are mirrored here: https://docs.worrbase.com/rust/kqueue/ .

## Examples

There are some basic usage examples in `examples/`.
