# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1] - 2023-05-26

### Fixed
 - Quick mode (--quick) no longer crashes with measured times over 5 seconds when --noplot is not active

## [0.5.0] - 2023-05-23

### Changed
- Replaced lazy_static dependency with once_cell
- Improved documentation of the `html_reports` feature
- Replaced atty dependency with is-terminal
- MSRV bumped to 1.64
- Upgraded clap dependency to v4
- Upgraded tempfile dependency to v3.5.0

### Fixed
- Quick mode (`--quick`) no longer outputs 1ms for measured times over 5 seconds
- Documentation updates

## [0.4.0] - 2022-09-10

### Removed

- The `Criterion::can_plot` function has been removed.
- The `Criterion::bench_function_over_inputs` function has been removed.
- The `Criterion::bench_functions` function has been removed.
- The `Criterion::bench` function has been removed.

### Changed

- HTML report hidden behind non-default feature flag: 'html_reports'
- Standalone support (ie without cargo-criterion) feature flag: 'cargo_bench_support'
- MSRV bumped to 1.57
- `rayon` and `plotters` are optional (and default) dependencies.
- Status messages ('warming up', 'analyzing', etc) are printed to stderr, benchmark results are printed to stdout.
- Accept subsecond durations for `--warm-up-time`, `--measurement-time` and `--profile-time`.
- Replaced serde_cbor with ciborium because the former is no longer maintained.
- Upgrade clap to v3 and regex to v1.5.

### Added

- A `--discard-baseline` flag for discarding rather than saving benchmark results.
- Formal support for benchmarking code compiled to web-assembly.
- A `--quiet` flag for printing just a single line per benchmark.
- A `Throughput::BytesDecimal` option for measuring throughput in bytes but printing them using
  decimal units like kilobytes instead of binary units like kibibytes.

### Fixed
- When using `bench_with_input`, the input parameter will now be passed through `black_box` before
  passing it to the benchmark.

## [0.3.6] - 2022-07-06
### Changed
- MSRV bumped to 1.49
- Symbol for microseconds changed from ASCII 'us' to unicode 'µs'
- Documentation fixes
- Clippy fixes

## [0.3.5] - 2021-07-26

### Fixed

- Corrected `Criterion.toml` in the book.
- Corrected configuration typo in the book.

### Changed

- Bump plotters dependency to always include a bug-fix.
- MSRV bumped to 1.46.

## [0.3.4] - 2021-01-24

### Added

- Added support for benchmarking async functions
- Added `with_output_color` for enabling or disabling CLI output coloring programmatically.

### Fixed

- Criterion.rs will now give a clear error message in case of benchmarks that take zero time.
- Added some extra code to ensure that every sample has at least one iteration.
- Added a notice to the `--help` output regarding "unrecognized option" errors.
- Increased opacity on violin charts.
- Fixed violin chart X axis not starting at zero in the plotters backend.
- Criterion.rs will now automatically detect the right output directory.

### Deprecated

- `Criterion::can_plot` is no longer useful and is deprecated pending deletion in 0.4.0.
- `Benchmark` and `ParameterizedBenchmark` were already hidden from documentation, but are now
  formally deprecated pending deletion in 0.4.0. Callers should use `BenchmarkGroup` instead.
- `Criterion::bench_function_over_inputs`, `Criterion::bench_functions`, and `Criterion::bench` were
  already hidden from documentation, but are now formally deprecated pending deletion in 0.4.0.
  Callers should use `BenchmarkGroup` instead.
- Three new optional features have been added; "html_reports", "csv_output" and
  "cargo_bench_support". These features currently do nothing except disable a warning message at
  runtime, but in version 0.4.0 they will be used to enable HTML report generation, CSV file
  generation, and the ability to run in cargo-bench (as opposed to [cargo-criterion]).
  "cargo_bench_support" is enabled by default, but "html_reports" and "csv_output"
  are not. If you use Criterion.rs' HTML reports, it is recommended to switch to [cargo-criterion].
  If you use CSV output, it is recommended to switch to [cargo-criterion] and use the
  `--message-format=json` option for machine-readable output instead. A warning message will be
  printed at the start of benchmark runs which do not have "html_reports" or "cargo_bench_support"
  enabled, but because CSV output is not widely used it has no warning.

[cargo-criterion]: https://github.com/bheisler/cargo-criterion

## [0.3.3] - 2020-06-29

### Added

- Added `CRITERION_HOME` environment variable to set the directory for Criterion to store
  its results and charts in.
- Added support for [cargo-criterion]. The long-term goal here is to remove code from Criterion-rs
  itself to improve compile times, as well as to add features to `cargo-criterion` that are
  difficult to implement in Criterion-rs.
- Add sampling mode option for benchmarks. This allows the user to change how Criterion.rs chooses
  the iteration counts in each sample. By default, nothing will change for most benchmarks, but
  very slow benchmarks will now run fewer iterations to fit in the desired number of samples.
  This affects the statistics and plots generated.

### Changed

- The serialization format for some of the files has changed. This may cause your first benchmark
  run after updating to produce errors, but they're harmless and will go away after running the
  benchmarks once.

### Fixed

- Fixed a bug where the current measurement was not shown on the relative regression plot.
- Fixed rare panic in the plotters backend.
- Panic with a clear error message (rather than panicking messily later on) when the user sets the
  group or function name to the empty string.
- Escape single quotes in benchmark names when generating Gnuplot scripts.

## [0.3.2] - 2020-04-26

### Added

- Added `?Sized` bound to benchmark parameter types, which allows dynamically sized types like
  `&str` and `&[T]` to be used as benchmark parameters.
- Added the `--output-format <format>` command-line option. If `--output-format bencher` is passed,
  Criterion.rs will print its measurements in a format similar to that used by the `bencher` crate
  or unstable `libtest` benchmarks, and using similar statistical measurements as well. Though this
  provides less information than the default format, it may be useful for supporting tools which
  parse this output format.
- Added `--nocapture` argument. This argument does nothing, but prevents Criterion.rs from exiting
  when running tests or benchmarks and allowing stdout output from other tests.

### Fixed

- Fixed panic when environment variables contains non-UTF8 characters.
- Fixed panic when `CRITERION_DEBUG` or `CRITERION_TARGET_DIR` environment variables contain
  non-UTF8 characters.

## [0.3.1] - 2020-01-25

### Added

- Added new plotting backend using the `plotters` crate. Implementation generously provided by Hao
  Hou, author of the `plotters` crate.
- Added `--plotting-backend` command-line option to select the plotting backend. The existing
  gnuplot backend will be used by default when available, and the plotters backend will be used when
  gnuplot is not available or when requested.
- Added `Criterion::plotting_backend()` function to configure the plotting backend in code.
- Added `--load-baseline` command-line option to load a baseline for comparison
  rather than measuring the current code
- Benchmark filters can now be regular expressions.

### Fixed

- Fixed `fibonacci` functions.
- Fixed `#[criterion]` benchmarks ignoring the command-line options.
- Fixed incorrect scaling of the violin plots.
- Don't print the recommended sample count if it's the same as the configured
  sample count.
- Fix potential panic when `nresamples` is set too low. Also added a warning
  against setting `nresamples` too low.
- Fixed issue where a slow outer closure would cause Criterion.rs to calculate
  the wrong estimated time and number of iterations in the warm-up phase.

## [0.3.0] - 2019-08-25

### Added

- Added support for plugging in custom measurements (eg. processor counters)
  into Criterion.rs' measurement and analysis.
- Added support for plugging in instrumentation for internal profilers such as
  `cpuprofiler` which must be explicitly started and stopped within the profiled
  process.
- Added the `BenchmarkGroup` type, which supersedes `ParameterizedBenchmark`, `Benchmark`,
  `Criterion::bench_functions`, `Criterion::bench_function_over_inputs`, and `Criterion::bench`.
  `BenchmarkGroup` performs the same function as all of the above, but is cleaner to use and more
  powerful and flexible. All of these types/functions are now soft-deprecated (meaning they're
  hidden from the documentation and should not be used in new code). They will be fully deprecated
  at some point in the 0.3.\* series and removed in 0.4.0.
- `iter_custom` - a "timing loop" that allows the caller to perform their own measurements. This is
  useful for complex measurements that don't fit into the usual mode of calling a lambda in a loop.
- If the benchmark cannot be completed in approximately the requested measurement time,
  Criterion.rs will now print a suggested measurement time and sample size that would work.
- Two new fields, `throughput_num` and `throughput_type` have been added to the `raw.csv` file.
- Added command-line options to set the defaults for warm-up time, measurement-time, etc.

### Changed

- The `raw.csv` file format has been changed slightly. The `sample_time_nanos` field has been split
  into `sample_measured_value` and `unit` fields to accommodate custom measurements.
- Throughput has been expanded from u32 to u64 to accommodate very large input sizes.

### Fixed

- Fixed possible invalid file name error on Windows
- Fixed potential case where data for two different benchmarks would be stored in the same directory.

### Removed

- Removed the `--measure-only` command-line argument; it was deprecated in favor of `--profile-time`
  in 0.2.6.
- External program benchmarks have been removed; they were deprecated in 0.2.6. The new
  `iter_custom` timing loop can be used as a substitute; see `benches/external_process.rs` for an
  example of this.

### Deprecated

- The `--test` argument is now deprecated. To test benchmarks, use `cargo test --benches`.

## [0.2.11] - 2019-04-08

### Added

- Enabled automatic text-coloring on Windows.

### Fixed

- Fixed panic caused by outdated files after benchmark names or types were changed.
- Reduced timing overhead of `Criterion::iter_batched/iter_batched_ref`.

## [0.2.10] - 2019-02-09

### Added

- Added `iter_batched/iter_batched_ref` timing loops, which allow for setup (like
  `iter_with_setup/iter_with_large_setup`) and exclude drop (like `iter_with_large_drop`) but
  measure the runtime more accurately, use less memory and are more flexible.

### Deprecated

- `iter_with_setup/iter_with_large_setup` are now deprecated in favor of `iter_batched`.

## [0.2.9] - 2019-01-24

### Changed

- Criterion.rs no longer depends on the default features of the `rand-core` crate. This fixes some
  downstream crates which use `rand` in a `no_std` context.

## [0.2.8] - 2019-01-20

### Changed

- Criterion.rs now uses `rayon` internally instead of manual `unsafe` code built with thread-scoped.
- Replaced handlebars templates with [TinyTemplate](https://github.com/bheisler/TinyTemplate)
- Merged `criterion-stats` crate into `criterion` crate. `criterion-stats` will no longer receive
  updates.
- Replaced or removed various other dependencies to reduce the size of Criterion.rs' dependency
  tree.

## [0.2.7] - 2018-12-29

### Fixed

- Fixed version numbers to prevent incompatibilities between `criterion` and `criterion-stats`
  crates.

## [0.2.6] - 2018-12-27 - Yanked

### Added

- Added `--list` command line option, which lists the benchmarks but does not run them, to match
  `cargo test -- --list`.
- Added README/CONTRIBUTING/LICENSE files to sub-crates.
- Displays change in throughput in the command-line and HTML output as well as change in iteration
  time.
- Benchmarks with multiple functions and multiple values will now generate a per-value summary
  report file in addition to the existing per-function one.
- Added a `--profile-time` command-line argument which disables reporting and analysis and instead
  simply iterates each benchmark for approximately the given number of seconds. This supersedes the
  (now-deprecated) `--measure-only` argument.

### Fixed

- Functions passed to `Bencher::iter_with_large_setup` can now return output. This is necessary to
  prevent the compiler from optimizing away the benchmark. This is technically a breaking change -
  that function requires a new type parameter. It's so unlikely to break existing code that I
  decided not to delay this for a breaking-change release.
- Reduced measurement overhead for the `iter_with_large_setup` and `iter_with_drop` methods.
- `criterion_group` and `criterion_main` macros no longer require the `Criterion` struct to be
  explicitly imported.
- Don't panic when `gnuplot --version` fails.
- Criterion.rs macros no longer require user to `use criterion::Criterion;`
- Criterion.rs no longer initializes a logger, meaning that it will no longer conflict with user
  code which does.
- Criterion.rs no longer fails to parse gnuplot version numbers like
  `gnuplot 5.2 patchlevel 5a (Gentoo revision r0)`
- Criterion.rs no longer prints an error message that gnuplot couldn't be found when chart
  generation is disabled (either by `Criterion::without_plots`, `--noplot` or disabling the
  HTML reports feature)
- Benchmark names are now automatically truncated to 100 characters and a number may be added to
  make them unique. This fixes a problem where gnuplot would crash if the title was extremely long,
  and also improves the general usability of Criterion.rs.

### Changed

- Changed timing model of `iter_with_large_setup` to exclude time spent dropping values returned
  by the routine. Time measurements taken with 0.2.6 using these methods may differ from those taken
  with 0.2.5.
- Benchmarks with multiple functions and multiple values will now appear as a table rather than a
  tree in the benchmark index. This is to accommodate the new per-value summary reports.

### Deprecated

- Deprecated the `--measure-only` command-line-argument in favor of `--profile-time`. This will be
  removed in 0.3.0.
- External-program benchmarks are now deprecated. They will be removed in 0.3.0.
- The `html_reports` cargo feature is now deprecated. This feature will become non-optional in 0.3.0.
- Sample sizes less than 10 are deprecated and will be disallowed in 0.3.0.
- This is not an exhaustive list - the full scope of changes in 0.3.0 is not yet determined. There
  may be breaking changes that are not listed here.

## [0.2.5] - 2018-08-27

### Fixed

- Fixed links from generated report files to documentation.
- Fixed formatting for very large percentage changes (>1000%)
- Sorted the benchmarks in the index report by name
- Fixed case where benchmark ID with special characters would cause Criterion.rs to open the wrong
  file and log an error message.
- Fixed case where running `cargo clean; cargo bench -- <filter>` would cause Criterion.rs to log
  an error message.
- Fixed a GNUplot error message when sample size is very small.
- Fixed several cases where Criterion.rs would generate invalid path names.
- Fixed a bug where Criterion.rs would print an error if run with a filter that allowed no benchmarks and a clean target directory.
- Fixed bug where some benchmarks didn't appear in the benchmark index report.
- Criterion.rs now honors the `CARGO_TARGET_DIR` environment variable.

### Added

- Criterion.rs will generate a chart showing the effects of changes in input (or input size) for all
  benchmarks with numeric inputs or throughput, not just for those which compare multiple functions.

## [0.2.4] 2018-07-08

### Added

- Added a pair of flags, `--save-baseline` and `--baseline`, which change
  how benchmark results are stored and compared. This is useful for
  working against a fixed baseline(eg. comparing progress on an
  optimization feature branch to the commit it forked from).
  Default behavior of Criterion.rs is now `--save-baseline base`
  which emulates the previous, user facing behavior.
  - `--save-baseline` saves the benchmark results under the provided name.
  - `--baseline` compares the results to a saved baseline.
    If the baseline does not exist for a benchmark, an error is given.
- Added user-guide documentation for baselines, throughput measurements and
  plot configuration.
- Added a flag, `--test`, which causes Criterion to execute the benchmarks once
  without measuring or reporting the results. This is useful for checking that the
  benchmarks run successfully in a CI setting.
- Added a `raw.csv` file to the output which contains a stable, machine-readable
  representation of the measurements taken by benchmarks. This enables users to
  perform their own analysis or keep historical information without depending on
  private implementation details.

### Fixed

- The `sample_size` method on the `Criterion`, `Benchmark` and
  `ParameterizedBenchmark` structs has been changed to panic if the sample size
  is less than 2. Other parts of the code require this and will panic if the
  sample size is 1, so this is not considered to be a breaking change.
- API documentation has been updated to show more-complete examples.
- Certain characters will now be replaced with underscores when creating benchmark
  directory paths, to avoid generating invalid or unexpected paths.

## [0.2.3] - 2018-04-14

### Fixed

- Criterion.rs will now panic with a clear error message if the user attempts to run
  a benchmark which doesn't call the `Bencher::iter` function or a related function,
  rather than failing in an uncontrolled manner later.
- Fixed broken links in some more summary reports.

### Added

- Added a `--measure-only` argument which causes the benchmark executable to run the
  warmup and measurement and then move on to the next benchmark without analyzing or
  saving data. This is useful to prevent Criterion.rs' analysis code from appearing
  in profile data when profiling benchmarks.
- Added an index report file at "target/criterion/report/index.html" which links to
  the other reports for easy navigation.

## [0.2.2] - 2018-03-25

### Fixed

- Fixed broken links in some summary reports.
- Work around apparent rustc bug in >= 1.24.0.

## [0.2.1] - 2018-02-24

### Added

- HTML reports are now a default Cargo feature. If you wish to disable HTML reports,
  disable Criterion.rs' default features. Doing so will allow compatibility with
  older Rust versions such as 1.20. If you wish to continue using HTML reports, you
  don't need to do anything.
- Added a summary report for benchmarks that compare multiple functions or different
  inputs.

### Changed

- The plots and HTML reports are now generated in a `report` folder.

### Fixed

- Underscores in benchmark names will no longer cause subscripted characters to
  appear in generated plots.

## [0.2.0] - 2018-02-05

### Added

- Added `Criterion.bench` function, which accepts either a `Benchmark` or
  `ParameterizedBenchmark`. These new structures allow for custom per-benchmark
  configuration as well as more complex benchmark grouping (eg. comparing a Rust
  function against an external program over a range of inputs) which was not
  possible previously.
- Criterion.rs can now report the throughput of the benchmarked code in units of
  bytes or elements per second. See the `Benchmark.throughput` and
  `ParameterizedBenchmark.throughput` functions for further details.
- Criterion.rs now generates a basic HTML report for each benchmark.
- Added `--noplot` command line option to disable plot generation.

### Changed

- The builder methods on the Criterion struct now take and return self by value
  for easier chaining. Functions which configure a Criterion structure will need
  to be updated accordingly, or will need to be changed to work with the
  `Benchmark` or `ParameterizedBenchmark` types to do per-benchmark configuration
  instead.
- The closures taken by `Criterion.bench_*` must now have a `'static` lifetime.
  This means that you may need to change your closures from `|bencher| {...}`
  to `move |bencher| {...}`.
- `Criterion.bench_functions` now takes `I` as an input parameter, not `&I`.
- Input values must now implement `Debug` rather than `Display`.
- The generated plots are stored in `target/criterion` rather than `.criterion`.

### Removed

- The hidden `criterion::ConfidenceInterval` and`criterion::Estimate` types are
  no longer publicly accessible.
- The `Criterion.summarize` function has been removed.

### Fixed

- Fixed the relative mean and median reports.
- Fixed panic while summarizing benchmarks.

## [0.1.2] - 2018-01-12

### Changed

- Criterion.rs is now stable-compatible!
- Criterion.rs now includes its own stable-compatible `black_box` function.
  Some benchmarks may now be affected by dead-code-elimination where they
  previously weren't and may have to be updated.
- Criterion.rs now uses `serde` to save results. Existing results files will
  be automatically removed when benchmarks are run.
- Redesigned the command-line output to highlight the important information
  and reduce noise.

### Added

- Running benchmarks with the variable "CRITERION_DEBUG" in the environment will
  cause Criterion.rs to generate extra debug output and save the gnuplot scripts
  alongside the generated plots.

### Fixed

- Don't panic on IO errors or gnuplot failures
- Fix generation of invalid gnuplot scripts when benchmarking over inputs and inputs include values <= 0.
- Bug where benchmarks would run one sample fewer than was configured.

### Removed

- Generated plots will no longer use log-scale.

## [0.1.1] - 2017-12-12

### Added

- A changelog file.
- Added a chapter to the book on how Criterion.rs collects and analyzes data.
- Added macro rules to generate a test harness for use with `cargo bench`.
  Benchmarks defined without these macros should continue to work.
- New contribution guidelines
- Criterion.rs can selectively run benchmarks. See the Command-line page for
  more details

## 0.1.0 - 2017-12-02

### Added

- Initial release on Crates.io.

[Unreleased]: https://github.com/bheisler/criterion.rs/compare/0.4.0...HEAD
[0.1.1]: https://github.com/bheisler/criterion.rs/compare/0.1.0...0.1.1
[0.1.2]: https://github.com/bheisler/criterion.rs/compare/0.1.1...0.1.2
[0.2.0]: https://github.com/bheisler/criterion.rs/compare/0.1.2...0.2.0
[0.2.1]: https://github.com/bheisler/criterion.rs/compare/0.2.0...0.2.1
[0.2.2]: https://github.com/bheisler/criterion.rs/compare/0.2.1...0.2.2
[0.2.3]: https://github.com/bheisler/criterion.rs/compare/0.2.2...0.2.3
[0.2.4]: https://github.com/bheisler/criterion.rs/compare/0.2.3...0.2.4
[0.2.5]: https://github.com/bheisler/criterion.rs/compare/0.2.4...0.2.5
[0.2.6]: https://github.com/bheisler/criterion.rs/compare/0.2.5...0.2.6
[0.2.7]: https://github.com/bheisler/criterion.rs/compare/0.2.6...0.2.7
[0.2.8]: https://github.com/bheisler/criterion.rs/compare/0.2.7...0.2.8
[0.2.9]: https://github.com/bheisler/criterion.rs/compare/0.2.8...0.2.9
[0.2.10]: https://github.com/bheisler/criterion.rs/compare/0.2.9...0.2.10
[0.2.11]: https://github.com/bheisler/criterion.rs/compare/0.2.10...0.2.11
[0.3.0]: https://github.com/bheisler/criterion.rs/compare/0.2.11...0.3.0
[0.3.1]: https://github.com/bheisler/criterion.rs/compare/0.3.0...0.3.1
[0.3.2]: https://github.com/bheisler/criterion.rs/compare/0.3.1...0.3.2
[0.3.3]: https://github.com/bheisler/criterion.rs/compare/0.3.2...0.3.3
[0.3.4]: https://github.com/bheisler/criterion.rs/compare/0.3.3...0.3.4
[0.3.5]: https://github.com/bheisler/criterion.rs/compare/0.3.4...0.3.5
[0.3.6]: https://github.com/bheisler/criterion.rs/compare/0.3.5...0.3.6
[0.4.0]: https://github.com/bheisler/criterion.rs/compare/0.3.6...0.4.0
[0.5.0]: https://github.com/bheisler/criterion.rs/compare/0.4.0...0.5.0
[0.5.1]: https://github.com/bheisler/criterion.rs/compare/0.5.0...0.5.1
