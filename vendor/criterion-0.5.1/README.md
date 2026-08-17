<h1 align="center">Criterion.<span></span>rs</h1>

<div align="center">Statistics-driven Microbenchmarking in Rust</div>

<div align="center">
	<a href="https://bheisler.github.io/criterion.rs/book/getting_started.html">Getting Started</a>
    |
    <a href="https://bheisler.github.io/criterion.rs/book/index.html">User Guide</a>
    |
    <a href="https://bheisler.github.io/criterion.rs/criterion/">Master API Docs</a>
    |
    <a href="https://docs.rs/crate/criterion/">Released API Docs</a>
    |
    <a href="https://github.com/bheisler/criterion.rs/blob/master/CHANGELOG.md">Changelog</a>
</div>

<div align="center">
	<a href="https://github.com/bheisler/criterion.rs/actions/workflows/ci.yaml">
        <img src="https://img.shields.io/github/checks-status/rgeometry/rgeometry/main?label=tests&logo=github" alt="GitHub branch checks state">
    </a>
    |
    <a href="https://ci.appveyor.com/project/bheisler/criterion-rs-vt9fl">
        <img src="https://ci.appveyor.com/api/projects/status/4255ads9ctpupcl2?svg=true" alt="Appveyor">
    </a>
    |
    <a href="https://crates.io/crates/criterion">
        <img src="https://img.shields.io/crates/v/criterion.svg" alt="Crates.io">
    </a>
</div>

Criterion.<span></span>rs helps you write fast code by detecting and measuring performance improvements or regressions, even small ones, quickly and accurately. You can optimize with confidence, knowing how each change affects the performance of your code.

## Table of Contents
- [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Quickstart](#quickstart)
  - [Goals](#goals)
  - [Contributing](#contributing)
  - [Compatibility Policy](#compatibility-policy)
  - [Maintenance](#maintenance)
  - [License](#license)
  - [Related Projects](#related-projects)
  - [Criterion.rs Extensions](#criterionrs-extensions)

### Features

- __Statistics__: Statistical analysis detects if, and by how much, performance has changed since the last benchmark run
- __Charts__: Uses [gnuplot](http://www.gnuplot.info/) to generate detailed graphs of benchmark results
- __Stable-compatible__: Benchmark your code without installing nightly Rust

### Quickstart

In order to generate plots, you must have [gnuplot](http://www.gnuplot.info/) installed. See the gnuplot website for installation instructions. See [Compatibility Policy](#compatibility-policy) for details on the minimum supported Rust version.

To start with Criterion.<span></span>rs, add the following to your `Cargo.toml` file:

```toml
[dev-dependencies]
criterion = { version = "0.4", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

Next, define a benchmark by creating a file at `$PROJECT/benches/my_benchmark.rs` with the following contents:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

Finally, run this benchmark with `cargo bench`. You should see output similar to the following:

```
     Running target/release/deps/example-423eedc43b2b3a93
fib 20                  time:   [26.029 us 26.251 us 26.505 us]
Found 11 outliers among 99 measurements (11.11%)
  6 (6.06%) high mild
  5 (5.05%) high severe
```

See the [Getting Started](https://bheisler.github.io/criterion.rs/book/getting_started.html) guide for more details.

### Goals

The primary goal of Criterion.<span></span>rs is to provide a powerful and statistically rigorous tool for measuring the performance of code, preventing performance regressions and accurately measuring optimizations. Additionally, it should be as programmer-friendly as possible and make it easy to create reliable, useful benchmarks, even for programmers without an advanced background in statistics.

### Contributing

First, thank you for contributing.

One great way to contribute to Criterion.<span></span>rs is to use it for your own benchmarking needs and report your experiences, file and comment on issues, etc.

Code or documentation improvements in the form of pull requests are also welcome. If you're not
sure what to work on, try checking the
[Beginner label](https://github.com/bheisler/criterion.rs/issues?q=is%3Aissue+is%3Aopen+label%3ABeginner).

If your issues or pull requests have no response after a few days, feel free to ping me (@bheisler).

For more details, see the [CONTRIBUTING.md file](https://github.com/bheisler/criterion.rs/blob/master/CONTRIBUTING.md).

### Compatibility Policy

Criterion.<span></span>rs supports the last three stable minor releases of Rust. At time of
writing, this means Rust 1.59 or later. Older versions may work, but are not guaranteed.

Currently, the oldest version of Rust believed to work is 1.57. Future versions of Criterion.<span></span>rs may
break support for such old versions, and this will not be considered a breaking change. If you
require Criterion.<span></span>rs to work on old versions of Rust, you will need to stick to a
specific patch version of Criterion.<span></span>rs.

### Maintenance

Criterion.<span></span>rs was originally created by Jorge Aparicio (@japaric) and is currently being maintained by Brook Heisler (@bheisler).

### License

Criterion.<span></span>rs is dual licensed under the Apache 2.0 license and the MIT license.

### Related Projects

- [bencher](https://github.com/bluss/bencher) - A port of the libtest benchmark runner to stable Rust
- [criterion](http://www.serpentine.com/criterion/) - The Haskell microbenchmarking library that inspired Criterion.<span></span>rs
- [cargo-benchcmp](https://github.com/BurntSushi/cargo-benchcmp) - Cargo subcommand to compare the output of two libtest or bencher benchmark runs
- [cargo-flamegraph](https://github.com/ferrous-systems/flamegraph) - Cargo subcommand to profile an executable and produce a flamegraph

### Criterion.rs Extensions

- [criterion-cycles-per-byte](https://crates.io/crates/criterion-cycles-per-byte) - A custom-measurement plugin that counts the number of CPU cycles used by the benchmark
- [criterion-perf-events](https://crates.io/crates/criterion-perf-events) - A custom-measurement plugin that counts perf events created by the benchmark
