# HdrHistogram_rust

[![Crates.io](https://img.shields.io/crates/v/hdrhistogram.svg)](https://crates.io/crates/hdrhistogram)
[![Documentation](https://docs.rs/hdrhistogram/badge.svg)](https://docs.rs/hdrhistogram/)
[![Codecov](https://codecov.io/github/HdrHistogram/HdrHistogram_rust/coverage.svg?branch=master)](https://codecov.io/gh/HdrHistogram/HdrHistogram_rust)
[![Dependency status](https://deps.rs/repo/github/HdrHistogram/HdrHistogram_rust/status.svg)](https://deps.rs/repo/github/HdrHistogram/HdrHistogram_rust)

HdrSample is a port of Gil Tene's HdrHistogram to native Rust. It provides recording and
analyzing of sampled data value counts across a large, configurable value range with
configurable precision within the range. The resulting "HDR" histogram allows for fast and
accurate analysis of the extreme ranges of data with non-normal distributions, like latency.

## HdrHistogram

What follows is a description from [the HdrHistogram
website](https://hdrhistogram.github.io/HdrHistogram/). Users are encouraged to read the
documentation from the original [Java
implementation](https://github.com/HdrHistogram/HdrHistogram), as most of the concepts
translate directly to the Rust port.

HdrHistogram supports the recording and analyzing of sampled data value counts across a
configurable integer value range with configurable value precision within the range. Value
precision is expressed as the number of significant digits in the value recording, and provides
control over value quantization behavior across the value range and the subsequent value
resolution at any given level.

For example, a Histogram could be configured to track the counts of observed integer values
between 0 and 3,600,000,000 while maintaining a value precision of 3 significant digits across
that range. Value quantization within the range will thus be no larger than 1/1,000th (or 0.1%)
of any value. This example Histogram could be used to track and analyze the counts of observed
response times ranging between 1 microsecond and 1 hour in magnitude, while maintaining a value
resolution of 1 microsecond up to 1 millisecond, a resolution of 1 millisecond (or better) up
to one second, and a resolution of 1 second (or better) up to 1,000 seconds. At it's maximum
tracked value (1 hour), it would still maintain a resolution of 3.6 seconds (or better).

HDR Histogram is designed for recording histograms of value measurements in latency and
performance sensitive applications. Measurements show value recording times as low as 3-6
nanoseconds on modern (circa 2014) Intel CPUs. The HDR Histogram maintains a fixed cost in both
space and time. A Histogram's memory footprint is constant, with no allocation operations
involved in recording data values or in iterating through them. The memory footprint is fixed
regardless of the number of data value samples recorded, and depends solely on the dynamic
range and precision chosen. The amount of work involved in recording a sample is constant, and
directly computes storage index locations such that no iteration or searching is ever involved
in recording data values.

If you are looking for FFI bindings to
[`HdrHistogram_c`](https://github.com/HdrHistogram/HdrHistogram_c), you want the
[`hdrhistogram_c`](https://crates.io/crates/hdrhistogram_c) crate instead.

## Interacting with the library

HdrSample's API follows that of the original HdrHistogram Java implementation, with some
modifications to make its use more idiomatic in Rust. The description in this section has been
adapted from that given by the [Python port](https://github.com/HdrHistogram/HdrHistogram_py),
as it gives a nicer first-time introduction to the use of HdrHistogram than the Java docs do.

HdrSample is generally used in one of two modes: recording samples, or querying for analytics.
In distributed deployments, the recording may be performed remotely (and possibly in multiple
locations), to then be aggregated later in a central location for analysis.

### Recording samples

A histogram instance is created using the `::new` methods on the `Histogram` struct. These come
in three variants: `new`, `new_with_max`, and `new_with_bounds`. The first of these only sets
the required precision of the sampled data, but leaves the value range open such that any value
may be recorded. A `Histogram` created this way (or one where auto-resize has been explicitly
enabled) will automatically resize itself if a value that is too large to fit in the current
dataset is encountered. `new_with_max` sets an upper bound on the values to be recorded, and
disables auto-resizing, thus preventing any re-allocation during recording. If the application
attempts to record a larger value than this maximum bound, the `record` call will return an
error. Finally, `new_with_bounds` restricts the lowest representable value of the dataset,
such that a smaller range needs to be covered (thus reducing the overall allocation size).

For example the example below shows how to create a `Histogram` that can count values in the
`[1..3600000]` range with 1% precision, which could be used to track latencies in the range `[1
msec..1 hour]`).

```rust
use hdrhistogram::Histogram;
let mut hist = Histogram::<u64>::new_with_bounds(1, 60 * 60 * 1000, 2).unwrap();

// samples can be recorded using .record, which will error if the value is too small or large
hist.record(54321).expect("value 54321 should be in range");

// for ergonomics, samples can also be recorded with +=
// this call will panic if the value is out of range!
hist += 54321;

// if the code that generates the values is subject to Coordinated Omission,
// the self-correcting record method should be used instead.
// for example, if the expected sampling interval is 10 msec:
hist.record_correct(54321, 10).expect("value 54321 should be in range");
```

Note the `u64` type. This type can be changed to reduce the storage overhead for all the
histogram bins, at the cost of a risk of saturating if a large number of samples end up in the
same bin.

### Querying samples

At any time, the histogram can be queried to return interesting statistical measurements, such
as the total number of recorded samples, or the value at a given quantile:

```rust
use hdrhistogram::Histogram;
let hist = Histogram::<u64>::new(2).unwrap();
// ...
println!("# of samples: {}", hist.len());
println!("99.9'th percentile: {}", hist.value_at_quantile(0.999));
```

Several useful iterators are also provided for quickly getting an overview of the dataset. The
simplest one is `iter_recorded()`, which yields one item for every non-empty sample bin. All
the HdrHistogram iterators are supported in HdrSample, so look for the `*Iterator` classes in
the [Java documentation](https://hdrhistogram.github.io/HdrHistogram/JavaDoc/).

```rust
use hdrhistogram::Histogram;
let hist = Histogram::<u64>::new(2).unwrap();
// ...
for v in hist.iter_recorded() {
    println!("{}'th percentile of data is {} with {} samples",
        v.percentile(), v.value_iterated_to(), v.count_at_value());
}
```

### Panics and error handling

As long as you're using safe, non-panicking functions (see below), this library should never
panic. Any panics you encounter are a bug; please file them in the issue tracker.

A few functions have their functionality exposed via `AddAssign` and `SubAssign`
implementations. These alternate forms are equivalent to simply calling `unwrap()` on the
normal functions, so the normal rules of `unwrap()` apply: view with suspicion when used in
production code, etc.

| Returns Result                 | Panics on error    | Functionality                   |
| ------------------------------ | ------------------ | ------------------------------- |
| `h.record(v)`                  | `h += v`           | Increment count for value `v`   |
| `h.add(h2)`                    | `h += h2`          | Add `h2`'s counts to `h`        |
| `h.subtract(h2)`               | `h -= h2`          | Subtract `h2`'s counts from `h` |

Other than the panicking forms of the above functions, everything will return `Result` or
`Option` if it can fail.

### `usize` limitations

Depending on the configured number of significant digits and maximum value, a histogram's
internal storage may have hundreds of thousands of cells. Systems with a 16-bit `usize` cannot
represent pointer offsets that large, so relevant operations (creation, deserialization, etc)
will fail with a suitable error (e.g. `CreationError::UsizeTypeTooSmall`). If you are using such
a system and hitting these errors, reducing the number of significant digits will greatly reduce
memory consumption (and therefore the need for large `usize` values). Lowering the max value may
also help as long as resizing is disabled.

32- and above systems will not have any such issues, as all possible histograms fit within a
32-bit index.

### Floating point accuracy

Some calculations inherently involve floating point values, like `value_at_quantile`, and are
therefore subject to the precision limits of IEEE754 floating point calculations. The user-
visible consequence of this is that in certain corner cases, you might end up with a bucket (and
therefore value) that is higher or lower than it would be if the calculation had been done
with arbitrary-precision arithmetic. However, double-precision IEEE754 (i.e. `f64`) is very
good at its job, so these cases should be rare. Also, we haven't seen a case that was off by
more than one bucket.

To minimize FP precision losses, we favor working with quantiles rather than percentiles. A
quantile represents a portion of a set with a number in `[0, 1]`. A percentile is the same
concept, except it uses the range `[0, 100]`. Working just with quantiles means we can skip an
FP operation in a few places, and therefore avoid opportunities for precision loss to creep in.

## Limitations and Caveats

As with all the other HdrHistogram ports, the latest features and bug fixes from the upstream
HdrHistogram implementations may not be available in this port. A number of features have also
not (yet) been implemented:

 - Concurrency support (`AtomicHistogram`, `ConcurrentHistogram`, …).
 - `DoubleHistogram`.
 - The `Recorder` feature of HdrHistogram.
 - Value shifting ("normalization").
 - Textual output methods. These seem almost orthogonal to HdrSample, though it might be
   convenient if we implemented some relevant traits (CSV, JSON, and possibly simple
   `fmt::Display`).

Most of these should be fairly straightforward to add, as the code aligns pretty well with the
original Java/C# code. If you do decide to implement one and send a PR, please make sure you
also port the [test
cases](https://github.com/HdrHistogram/HdrHistogram/tree/master/src/test/java/org/HdrHistogram),
and try to make sure you implement appropriate traits to make the use of the feature as
ergonomic as possible.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hdrhistogram = "7"
```

and this to your crate root:

```rust
extern crate hdrhistogram;
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your option. This file may not be
copied, modified, or distributed except according to those terms.
