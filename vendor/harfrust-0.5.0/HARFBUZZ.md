# Comparing to HarfBuzz for correctness & performance

We aspire to match HarfBuzz, in correctness and performance.

We have our own subset of the HarfBuzz shaping tests in `tests`,
which all pass (`cargo test`).

We also have our own benchmark suite against HarfBuzz with
`cargo bench` that runs the same shaping benchmarks using the
[`harfbuzz_rs`](https://github.com/harfbuzz/harfbuzz_rs)
Rust HarfBuzz bindings crate.

This document tells you how to run HarfBuzz's own shaping tests
and benchmark suite against HarfRust, and how to compare the
results.

## Building HarfBuzz with HarfRust support

First, HarfBuzz requires the Rust nightly toolchain to build
with HarfRust support. It also requires bindgen. You can prepare
those all using:

```sh
$ rustup default nightly
$ rustup component add rust-src
$ cargo install bindgen-cli
```

Next, we need to build HarfBuzz with HarfRust shaper support
and the benchmark suite enabled. We also build in `release` mode.
By default HarfBuzz builds with a `debugoptimized` mode, that
is slightly slower, but better for development.

```sh
$ meson build -Dbenchmark=enabled -Dharfrust=enabled -Dbuildtype=release
$ ninja -C build
```


## Running HarfBuzz's Shaping Tests

This one is easy: we override the default shaper via an environment
variable, and then run the tests.  Note that you have to use `meson`
to run the tests; it won't work with `ninja` directly, since `ninja`
locks the environment variables.

```sh
$ HB_SHAPER_LIST=harfrust meson test -C build
```

If all goes well, you should see lots of output, ending in:

```
Ok:                 217
Expected Fail:      0
Fail:               2
Unexpected Pass:    0
Skipped:            0
Timeout:            0
```

If you scroll up, you'd see that the two failing tests are:
```
214/219 harfbuzz:shape+text-rendering-tests / text-rendering-tests            FAIL             0.12s   432/435 subtests passed
```
and
```
218/219 harfbuzz:shape+in-house / in-house                                    FAIL             0.36s   4779/4800 subtests passed
```

This is pretty good. In total, there are 24 shaping tests failing.
Those are mostly due to HarfRust not supporting some esoteric
shaping features of HarfBuzz.

To see specific failures, you can run inspect the test log:
```sh
$ less build/meson-logs/testlog.txt
```

Currently the following tests fail:
- `SHBALI-3.tests`: Rounding differences with unusual UPEM.
- `arabic-fallback-positioning.tests`: Not implemented.
- `collections.tests`: `DFONT` format is not supported.
- `vertical.tests`: Fallback based on glyph extents not supported.


## Running HarfBuzz's Benchmark Tests

The tool we are interested in is `build/perf/benchmark-shape`.
You can try running it, and it will run a few benchmarks against
all compiled shaping backends. There is a `--help` option, but
the following are the most useful:

`--benchmark_filter=REGEX` to select which benchmarks to run.
  Note the underscore in the option name. The regex is matched
  against the benchmark name.  To only run against HarfRust,
  you can use `--benchmark_filter=harfrust`. Eg.:
```
$ build/perf/benchmark-shape --benchmark_filter=harfrust
2025-08-05T14:49:40-06:00
Running build/perf/benchmark-shape
Run on (24 X 2208 MHz CPU s)
CPU Caches:
  L1 Data 32 KiB (x12)
  L1 Instruction 32 KiB (x12)
  L2 Unified 512 KiB (x12)
  L3 Unified 32768 KiB (x2)
Load Average: 0.54, 0.60, 0.61
----------------------------------------------------------------------------------------------------------------
Benchmark                                                                      Time             CPU   Iterations
----------------------------------------------------------------------------------------------------------------
BM_Shape/NotoNastaliqUrdu-Regular.ttf/fa-thelittleprince.txt/harfrust        193 ms          192 ms            4
BM_Shape/NotoNastaliqUrdu-Regular.ttf/fa-words.txt/harfrust                  213 ms          212 ms            3
BM_Shape/Amiri-Regular.ttf/fa-thelittleprince.txt/harfrust                  69.6 ms         69.2 ms           10
BM_Shape/NotoSansDevanagari-Regular.ttf/hi-words.txt/harfrust               41.8 ms         41.6 ms           17
BM_Shape/Roboto-Regular.ttf/en-thelittleprince.txt/harfrust                 21.3 ms         21.2 ms           33
BM_Shape/Roboto-Regular.ttf/en-words.txt/harfrust                           28.7 ms         28.6 ms           24
BM_Shape/SourceSerifVariable-Roman.ttf/react-dom.txt/harfrust                233 ms          232 ms            3
```

We are interested in the CPU column.  To get the output from HarfBuzz's
own shaper (called `ot` for historical reasons) as well as HarfRust, you can use:
```sh
$ build/perf/benchmark-shape --benchmark_filter='/(ot|harfrust)'
...
----------------------------------------------------------------------------------------------------------------
Benchmark                                                                      Time             CPU   Iterations
----------------------------------------------------------------------------------------------------------------
BM_Shape/NotoNastaliqUrdu-Regular.ttf/fa-thelittleprince.txt/ot             83.1 ms         82.7 ms            9
BM_Shape/NotoNastaliqUrdu-Regular.ttf/fa-thelittleprince.txt/harfrust        189 ms          188 ms            4
BM_Shape/NotoNastaliqUrdu-Regular.ttf/fa-words.txt/ot                       96.6 ms         96.0 ms            7
BM_Shape/NotoNastaliqUrdu-Regular.ttf/fa-words.txt/harfrust                  213 ms          212 ms            3
BM_Shape/Amiri-Regular.ttf/fa-thelittleprince.txt/ot                        39.6 ms         39.4 ms           18
BM_Shape/Amiri-Regular.ttf/fa-thelittleprince.txt/harfrust                  67.9 ms         67.6 ms           10
BM_Shape/NotoSansDevanagari-Regular.ttf/hi-words.txt/ot                     25.1 ms         25.0 ms           28
BM_Shape/NotoSansDevanagari-Regular.ttf/hi-words.txt/harfrust               42.6 ms         42.5 ms           17
BM_Shape/Roboto-Regular.ttf/en-thelittleprince.txt/ot                       8.83 ms         8.79 ms           80
BM_Shape/Roboto-Regular.ttf/en-thelittleprince.txt/harfrust                 21.4 ms         21.3 ms           33
BM_Shape/Roboto-Regular.ttf/en-words.txt/ot                                 11.9 ms         11.9 ms           59
BM_Shape/Roboto-Regular.ttf/en-words.txt/harfrust                           29.5 ms         29.3 ms           24
BM_Shape/SourceSerifVariable-Roman.ttf/react-dom.txt/ot                     97.5 ms         97.0 ms            7
BM_Shape/SourceSerifVariable-Roman.ttf/react-dom.txt/harfrust                234 ms          233 ms            3
```

You might get a warning that your CPU frequency scaling is not set to
performance and that might affect the results.  You can try to fix that
depending on what OS you are using.  On Linux, you can try:
```sh
$ sudo cpupower frequency-set -g performance
```

`--benchmark_out=FILE` to write the results to a file, in JSON format.
This can be used to compare results across runs, using a comparison tool
provided by the benchmark framework:
```sh
$ subprojects/benchmark-1.8.4/tools/compare.py benchmarks BEFORE.json AFTER.json
```
Substitute 1.8.4 with the actual version you have.

`--benchmark_repetitions=NUM` to run the benchmarks multiple times, to
get more stable results.  Note that in each run, each benchmark is
already run to a minimum of 0.5 seconds.

To run the benchmark tool against a custom font, you need to also pick
a test file to shape, and you just pass the two to the benchmark tool.
There are a few useful text files in `perf/texts` directory.  For example,
to run the lookup-heavy Gulzar font against the `fa-words.txt` file:

```sh
$ build/perf/benchmark-shape Gulzar-Regular.ttf perf/texts/fa-words.txt --benchmark_filter='(ot|harfrust)'
...
--------------------------------------------------------------------------------------------
Benchmark                                                  Time             CPU   Iterations
--------------------------------------------------------------------------------------------
BM_Shape/Gulzar-Regular.ttf/fa-words.txt/ot              607 ms          604 ms            1
BM_Shape/Gulzar-Regular.ttf/fa-words.txt/harfrust       2085 ms         2074 ms            1
```

We maintain a running performance comparison of HarfRust against HarfBuzz
over time [here](https://docs.google.com/spreadsheets/d/1lyPPZHXIF8gE0Tpx7_IscwhwaZa4KOpdt7vnV0jQT9o/preview).


## Running against local HarfRust and Fontations

By default, HarfBuzz will use HarfRust from github `main` branch,
as can bee seen in `src/rust/Cargo.toml` file:
```toml
[dependencies]
skrifa = { version = "0.*", optional = true }
harfrust = { git = "https://github.com/harfbuzz/harfrust", optional = true }
```

To run against a different branch, just add the branch name as an
extra configuration to `src/rust/Cargo.toml`, eg.:
```toml
[dependencies]
skrifa = { version = "0.*", optional = true }
harfrust = { branch = "NAME", git = "https://github.com/harfbuzz/harfrust", optional = true }
```

To run against a local HarfRust, you can use a local path, eg.:
```toml
[dependencies]
skrifa = { version = "0.*", optional = true }
harfrust = { path = "../../../harfrust", optional = true }
```

Note that to run HarfRust against a local Fontations, you need to
also modify the HarfRust `Cargo.toml` file to point to the local
`fontations` path. Look for the `read-fonts` under `[dependencies]` section:
```toml
read-fonts = { path = "../fontations/read-fonts", default-features = false, features = ["libm"] }
```

Finally, HarfBuzz wouldn't see any changes you make to the HarfRust
or Fontations code, so it would not rebuild them. You can force
HarfBuzz to rebuild by, eg., touching the HarfBuzz rust shaper file:
```sh
$ touch src/rust/shaper.rs && ninja -Cbuild
```
or faster, if you just want to run the benchmark:
```sh
$ touch src/rust/shaper.rs && ninja -Cbuild perf/benchmark-shape
```
