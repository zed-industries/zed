[![Workflow Status](https://github.com/enarx/ciborium/workflows/test/badge.svg)](https://github.com/enarx/ciborium/actions?query=workflow%3A%22test%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/enarx/ciborium.svg)](https://isitmaintained.com/project/enarx/ciborium "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/enarx/ciborium.svg)](https://isitmaintained.com/project/enarx/ciborium "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# ciborium-io

Simple, Low-level I/O traits

This crate provides two simple traits: `Read` and `Write`. These traits
mimic their counterparts in `std::io`, but are trimmed for simplicity
and can be used in `no_std` and `no_alloc` environments. Since this
crate contains only traits, inline functions and unit structs, it should
be a zero-cost abstraction.

If the `std` feature is enabled, we provide blanket implementations for
all `std::io` types. If the `alloc` feature is enabled, we provide
implementations for `Vec<u8>`. In all cases, you get implementations
for byte slices. You can, of course, implement the traits for your own
types.

License: Apache-2.0
