---
name: gpui-bench
description: >-
  Design, write, review, run, and interpret production-shaped GPUI Criterion
  benchmarks, including gpui::bench, BenchAppContext, renderer and task
  benchmarks, headless Metal frame data, responsiveness and hang regressions,
  feature isolation from test-support, and before/after performance evidence.
---

# GPUI Benchmarks

Use this skill when a user asks to benchmark GPUI code, reproduce a UI hang or frame drop, evaluate a performance fix, use `#[gpui::bench]`, interpret `BenchReport`, or review whether a GPUI benchmark represents production.

The primary goal is UI responsiveness. Throughput matters, but a UI that finishes work quickly while blocking input and frames is still regressed.

## Start with the performance question

Before editing, establish or derive:

1. What user-visible problem is being reproduced: slow computation, long foreground poll, delayed input, frame drops, scrolling hitch, GPU draw cost, or a true hang?
2. What production event starts the work, and what path performs it?
3. What competing UI work must remain responsive? Prefer a rendered progress indicator, cursor, spinner, or scrolling frame when practical.
4. What target frame rate applies? GPUI defaults to 120 FPS, an 8.33 ms frame budget. At 60 FPS the budget is 16.67 ms.
5. What fixed workload sizes expose a progression from normal to degraded to severe?
6. What state proves the workload completed without dropping, duplicating, or reordering work?
7. Which commits are the baseline and candidate, and can the exact same benchmark code run on both?

Ask only for inputs that cannot be derived from the repository, issue, trace, or existing benchmark.

## Non-negotiable rules

- A benchmark must not enable any crate's `test-support` feature, directly or transitively.
- Use production constructors, storage, executors, rendering, synchronization, and data sizes whenever practical.
- Do not use `TestAppContext`, deterministic test executors, reduced test-only CRDT settings, fake clocks, fake filesystems, or fake services merely because setup is easier.
- A narrow benchmark seam may simulate an external boundary such as a PTY, server, or filesystem event, but everything after that boundary should follow the production path.
- A performance fix should normally include or extend a benchmark that reproduces its problem. If it does not, explain why before implementing the fix.
- Every benchmark or profile invocation run by an agent must have a hard timeout of at most five minutes. Use `timeout_ms <= 300000` with the terminal tool.
- Run smoke and quick modes before measured runs. Never start an unbounded profile or Criterion run.
- Do not run measured benchmarks concurrently. Parallel benchmark processes contaminate each other's results.
- Preserve correctness assertions in benchmark fixtures. Faster output that loses work is not an improvement.
- Never invent timing, frame, percentile, or throughput results.

## Verify feature isolation first

Use GPUI's canonical `bench-support` feature. The published `bench` feature is a temporary compatibility alias; new code should not use it. Neither feature may enable `test-support`.

Inspect the complete feature graph for the actual benchmark package:

```sh
feature_tree="$(cargo tree -p <benchmark-package> -e normal,build,dev,features)"
if grep -F 'feature "test-support"' <<<"${feature_tree}"; then
  echo 'benchmark graph contains test-support' >&2
  exit 1
fi
```

Also use inverse feature trees when diagnosing a leak:

```sh
cargo tree -p <benchmark-package> -e features -i gpui
cargo tree -p <benchmark-package> -e features -i <crate-under-benchmark>
```

Cargo unifies features across every target in one package. If an unrelated benchmark needs `test-support`, move the production benchmark to a separate package when necessary; selecting one `--bench` target does not undo package-wide feature unification.

Add a repository script or CI check for important isolation rules. Do not rely only on a one-time manual `cargo tree` inspection.

## Benchmark-only APIs

Use these feature names:

- `bench-support`: benchmark infrastructure and narrow, production-faithful constructors or external-boundary seams.
- `test-support`: test doubles, mutation hooks, failure injection, and test harnesses. It is forbidden in benchmark graphs.

The `#[gpui::bench]` attribute keeps its existing name. The Cargo feature describes support consumed by benchmark targets, not the benchmark declaration itself.

When a useful function is currently test-gated:

1. Determine whether its behavior is production-safe or whether it is a fake.
2. Extract shared production logic into a private primitive when possible.
3. Expose the smallest wrapper under `#[cfg(any(test, feature = "bench-support"))]`.
4. Keep mutations, failure injection, and fake global state test-only unless the benchmark specifically models that production boundary.
5. Add a parity test when the benchmark wrapper must match an existing test helper.
6. Verify that enabling `bench-support` alone brings in no `test-support` feature.

Prefer a seam that supplies bytes or events to a production pipeline over widening an entire fake test harness.

## Model the production path

Map the workload before constructing the fixture:

```text
production trigger
    -> queue or event boundary
    -> background preparation
    -> foreground application
    -> invalidation
    -> layout
    -> prepaint
    -> paint
    -> present
    -> observable completion
```

Keep each relevant boundary in the benchmark. For a queue or stream, drive enough work to exceed capacity and let a concurrent producer refill it while the consumer drains. A burst that fits entirely in the queue cannot reproduce sustained backpressure.

Use realistic expensive shapes, not only easy append-only inputs. For rendering, include representative text lengths, styles, images, tools, comments, viewport sizes, and invalidation patterns. Change one workload dimension at a time so scaling remains interpretable.

Separate each iteration into:

1. **Prepare:** construct or reset state outside timing.
2. **Measure:** perform the production operation.
3. **Validate:** assert output, work counts, ordering, and completion outside timing when possible.

State whether caches, glyph atlases, storage, and other fixtures are cold or warm. Do not accidentally compare a cold baseline with a warm candidate.

## Choose the right GPUI measurement API

`#[gpui::bench]` creates a Criterion benchmark using `BenchAppContext` and a production-style threaded dispatcher. The benchmark function itself is synchronous; use the context's task APIs for asynchronous work.

The attribute supports options such as:

```rust
#[gpui::bench(
    fps = 120,
    inputs = workload_sizes(),
    input_name = "items",
    group = "Streaming update",
    sample_size = 10
)]
```

Inspect the current macro before depending on an option because the API is evolving.

### `bench_iter`: synchronous functions and compute work

Use `bench_iter` for synchronous application or compute code:

```rust
#[gpui::bench(inputs = sizes(), input_name = "bytes", group = "Parse")]
fn parse_document(byte_count: &usize, cx: &mut gpui::BenchAppContext) {
    let input = build_input(*byte_count);
    cx.bench_iter(|_| {
        std::hint::black_box(parse(std::hint::black_box(&input)));
    });
}
```

Build reusable fixtures outside the measured closure. Use `black_box` for pure computation whose result could otherwise be optimized away. If the code does not interact with GPUI, ordinary Criterion may be simpler.

### `bench_task`: asynchronous work to completion

Use `bench_task` when the measured operation returns a GPUI `Task` and setup can be reused:

```rust
cx.bench_task(|cx| fixture.run_one_workload(cx));
```

This captures foreground task polls, action handlers, input dispatches, and draws produced while the task runs. It can surface a long foreground poll even when no window manages to draw during the stall.

### `bench_batched_task`: per-iteration setup outside timing

Use `bench_batched_task` when each iteration needs fresh input or state:

```rust
cx.bench_batched_task(
    |cx| fixture.prepare_iteration(cx),
    |iteration, cx| iteration.run(cx),
);
```

The setup result and task output are dropped outside the timed operation. This is the preferred shape for queues, streams, synchronization, and workloads whose setup must not contaminate measurement.

### `bench_renderer`: frame production

Use `bench_renderer` when each iteration updates an entity in a window's render tree and should draw and present a frame:

```rust
let mut window = cx.add_empty_window();
let view = window.update(|window, cx| {
    window.replace_root(cx, |_window, cx| cx.new(|_| BenchmarkView::new()))
});

cx.bench_renderer(view, |view, _window, cx| {
    view.advance_one_step();
    cx.notify();
});
```

Do not benchmark a detached entity and call the result rendering performance. Do not call `run_until_idle` inside a measured loop merely to make the fixture pass: production does not drain all asynchronous work before every frame, and doing so erases the scheduling behavior being measured.

## Headless rendering and frame data

`#[gpui::bench]` constructs its platform through `gpui::bench_platform` and requests `gpui_platform::current_headless_renderer`.

On macOS, the headless renderer uses Metal without showing a window. `bench_renderer` shapes text with the platform text system, builds the scene, exercises the sprite atlas, and submits the frame to the GPU on present. This catches CPU rendering, glyph, atlas, scene, and GPU-submission regressions that a no-op renderer misses.

On platforms without a headless renderer, GPUI still performs CPU-side window work, but presenting discards the scene and does not measure real GPU submission. State this limitation in results.

Depending on the pinned revision, `BenchReport` can include:

- dirty-to-draw duration;
- draw and present intervals;
- invalidations per frame;
- foreground task, action, and input-dispatch durations;
- p50, p90, p95, p99, and maximum durations;
- total and maximum frame-budget overruns.

Frame-budget overruns are a synthetic missed-deadline proxy because the headless harness has no display vsync. Do not call them literal display-dropped frames. Supplement important results with app automation and an Instruments, xctrace, miniprof, or equivalent profile.

The report may include Criterion warmup and calibration events even though Criterion's timing estimate uses its measurement phase. Read the current `BenchReport` implementation before interpreting its sample count or percentiles.

## Responsiveness benchmarks

A throughput-only benchmark is insufficient for foreground UI work. Measure both:

1. **Responsiveness:** longest uninterrupted foreground poll, high-percentile foreground duration, frame-budget overruns, dirty-to-draw delay, and frame/present cadence.
2. **Completion:** total time to consume the workload and, where useful, items or bytes per second.

A strong workload keeps an independent UI signal alive while expensive work runs. Suitable signals include a loading icon, cursor blink, scrolling viewport, progress counter, or small animation. The heavy workload and signal must share the production foreground executor so starvation is observable.

For each fixed input size, report:

- workload units: bytes, messages, edits, rows, frames, or tool calls;
- p50/p95/p99/max foreground work;
- total and maximum frame-budget overruns;
- draw and dirty-to-draw percentiles when a window is involved;
- total completion time;
- correctness and progress counts.

A fix that improves maximum foreground latency while making completion slightly slower may be a valid responsiveness tradeoff. State the tradeoff rather than hiding either metric.

## Reproducing hangs safely

A benchmark should reproduce a hang as a long or starved operation, not leave the process stuck forever.

For queue or streaming hangs:

- Drive more work than the queue capacity.
- Ensure a concurrent producer can refill the queue while the consumer drains it.
- Use the production blocking, wake, gather, synchronization, and rendering path.
- Include expensive update shapes, not only small append-only inputs.
- Add fixed small, medium, and severe workloads to show the scaling curve and point where frame budgets are missed.
- Record work counts alongside durations so a faster run cannot hide dropped work.
- Assert final state, ordering, and completion.
- Use a short internal watchdog for a true deadlock and the five-minute outer command timeout.

When possible, share the fixture between:

- a production-shaped benchmark that measures the performance progression; and
- a deterministic regression test that asserts the underlying invariant, work bound, or eventual progress.

Avoid fixed wall-clock performance assertions in portable unit tests. Prefer deterministic visit counts, batch bounds, no-loss assertions, or scaling properties. Use benchmark history or controlled performance CI for timing regressions.

A smoke mode should execute the complete benchmark path once and verify correctness without doing a long statistical run.

## Measure before optimizing

Profile before choosing an optimization. First reduce total work or improve the algorithm and data structures. Then consider allocations, cloning, cache behavior, and only finally specialized techniques such as SIMD.

For a hot operation, ask:

1. Can it be called fewer times by coalescing or incremental indexing?
2. Can it process less data by staying viewport- or change-bounded?
3. Can it avoid allocation, cloning, format conversion, or repeated lookup?
4. Can its data layout improve locality?
5. Does a profile still show a compute-heavy inner loop suitable for vectorization?

Do not infer a hot leaf from one stack or optimize a nearby function because it looks expensive. Profiles show where time is spent; the benchmark proves whether changing that work improves the user-visible result.

Parameterize workload sizes and report throughput units when they clarify scaling. Criterion estimates include confidence intervals and outlier analysis; report raw ranges and percentiles rather than only saying a result improved. Measurement time can exceed its configured value when one iteration is long, so retain the outer five-minute timeout.

## Before-and-after method

1. Add the benchmark before changing the production rule when practical.
2. Prove it reproduces the symptom on the baseline.
3. Keep benchmark code identical between baseline and candidate. Use a benchmark-only commit that can be applied to both revisions or a separate comparison worktree.
4. Build both with the same optimized profile, features, Rust flags, and lockfile.
5. Run on the same machine under similar thermal and power conditions.
6. Run a short smoke first, then a bounded measured run with the same warmup, measurement time, sample size, and input filter.
7. If results are noisy, alternate baseline and candidate runs instead of running every baseline first.
8. Report raw before/after numbers and percentage changes. Keep regressions and tradeoffs visible.
9. Profile when the benchmark shows a change but does not explain the hot path.

Do not treat a benchmark that only passes on the fixed branch as valid before understanding why it cannot run on the baseline.

## Profile with Instruments and xctrace

Use the normal optimized `release-fast` build for benchmark timing. For attribution-focused profiling on macOS, build the exact target with linker deduplication disabled:

```sh
RUSTFLAGS="-C link-arg=-Wl,-no_deduplicate -C codegen-units=1" \
  cargo bench -p <package> --bench <target> \
  --features bench-support --profile release-fast --no-run
```

These flags improve Rust symbol attribution but change code generation. Do not use that build for baseline timing numbers.

Run `dsymutil` on the exact executable printed by Cargo:

```sh
dsymutil target/release-fast/deps/<target>-<hash>
```

Prefer the `zed-bench` Instruments template, which combines Time Profiler data, CPU counters, and hang or microhang tables. Fall back to `Time Profiler` if it is unavailable:

```sh
rm -rf /tmp/zed-profiles/<name>.trace
xctrace record \
  --template "zed-bench" \
  --output /tmp/zed-profiles/<name>.trace \
  --launch -- target/release-fast/deps/<target>-<hash> \
  --bench <workload-filter> --profile-time 10
```

Running a Criterion executable directly requires `--bench`. `--profile-time` repeats the measured routine without Criterion's statistical analysis, making profiler samples easier to interpret.

Inspect the trace table of contents and export only focused tables such as `time-sample`; never load a complete `.trace` or large XML export into agent context. Filter by target PID, main thread when investigating foreground stalls, running state, and timer-fired samples. Symbolicate with the exact binary, dSYM, architecture, and load address using `atos`, then use `rustfilt` for Rust names.

If the profile contains a large `<deduplicated_symbol>` bucket or implausible functions, stop treating its symbol-level attribution as actionable and recapture with `-Wl,-no_deduplicate`. Report both flat leaf and inclusive bottom-up profiles. Load the `xctrace-rust-profile` skill for the complete export and symbolication workflow.

Trace bundles can contain process environments, paths, source, and credentials. Keep them local, inspect focused exports, and never paste raw trace metadata into a shared thread.

## Using agents well

Delegate independent analysis, not competing benchmark measurements.

Useful parallel roles:

- **Production-path auditor:** maps the real entry point, queues, executors, storage, rendering, and completion signal; read-only.
- **Feature-isolation auditor:** proves the graph contains no `test-support` and identifies the smallest `bench-support` seams; read-only.
- **Workload designer:** derives realistic fixed inputs from the incident, trace, or production data shapes and defines correctness checks; read-only.
- **Profile analyst:** analyzes an existing xctrace or miniprof without loading the raw profile into model context.

Keep one owner for benchmark implementation and measured runs. Do not let two agents modify the same fixture or run CPU or GPU measurements concurrently.

Every delegated task must state:

- exact branch/base and files in scope;
- the production path and user-visible symptom;
- the prohibition on `test-support`;
- the five-minute timeout for every benchmark invocation;
- required correctness checks and metrics;
- whether the task is read-only or may edit;
- that no PR should be opened until the benchmark reproduces the baseline and validates the candidate.

Ask agents to retain large trace exports and benchmark logs on disk and report only focused aggregates.

## Review checklist

Before accepting a GPUI benchmark, verify:

- [ ] The benchmark states the user-visible performance hypothesis.
- [ ] The exact benchmark feature graph has zero `test-support` occurrences.
- [ ] Any `bench-support` seam is narrow and production-faithful.
- [ ] The benchmark uses production storage, executors, and render paths where they affect the result.
- [ ] Setup is excluded from the timed interval unless setup is the operation being measured.
- [ ] Inputs are deterministic, fixed, realistic, and include a severe case.
- [ ] Cache state is controlled and documented.
- [ ] A responsiveness signal competes with the heavy work when foreground starvation matters.
- [ ] Frame metrics and completion throughput are both reported.
- [ ] Final correctness, ordering, and work counts are asserted.
- [ ] The workload completes under bounded smoke and measured runs.
- [ ] The same benchmark code runs on baseline and candidate.
- [ ] Results include raw percentiles, maxima, and confidence information, and disclose regressions.
- [ ] macOS headless Metal coverage or non-macOS limitations are stated.
- [ ] A deterministic regression test covers the non-timing invariant where practical.
- [ ] Benchmark and profile commands are bounded to five minutes and not run concurrently.

## Reporting results

Lead with whether the user-visible responsiveness problem was reproduced and whether the candidate fixed it. Then provide:

1. Exact commits, command, profile, platform, FPS, inputs, and sample configuration.
2. Foreground and frame metrics plus total completion before and after.
3. Correctness and work-count evidence.
4. Feature-isolation evidence.
5. Headless-renderer limitations and sources of noise.
6. Remaining expensive work and the next profile or optimization to pursue.

## References

- [Criterion measurement analysis](https://bheisler.github.io/criterion.rs/book/analysis.html)
- [Criterion advanced configuration](https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html)
- [Criterion external profiling](https://bheisler.github.io/criterion.rs/book/user_guide/profiling.html)
- [The Rust Performance Book: Profiling](https://nnethercote.github.io/perf-book/profiling.html)
- [The Rust Performance Book: General Tips](https://nnethercote.github.io/perf-book/general-tips.html)
