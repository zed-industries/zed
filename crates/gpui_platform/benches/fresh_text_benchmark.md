# Fresh text baseline

The warm and first-populated-frame text groups have been validated and measured.
Run validation and measurement serially; do not overlap builds or tests with
measured runs.

- Branch: `gpui-fresh-text-benchmark`.
- Exact base: `9ab0715969e9854f2efe61a9d782e7698e5fe6d2` (`origin/main` at preparation).
- Preserved source: `gpui-retained-source-preserved` at
  `7c9daf13fb9baf8c8fb7b8ba39859378f5b74b75`, including the text fitting patch
  and installed-font fixture on `c9aa7c0481d9d79cfbd53dc6cbfe5690ca730cd4`.
- No production optimization belongs in this branch. Keep the benchmark file
  byte-identical on future baseline/candidate branches; record its SHA-256.
  Retention comparisons are separate follow-ups.

The existing randomized groups remain intact. The two text groups use
`BenchAppContext`, production entities, text shaping, layout, and paint:

- `TextPanels/two updates`: historical requested family `monospace`.
- `TextPanels/installed font two updates`: explicit `Noto Sans Mono`, Linux only.
- Each uses 8/32/96 panels, 64 labels per panel, and updates panels 0 and
  `panel_count / 2` on each measured iteration.
- The 120 FPS setting is an 8.33 ms reporting budget, not frame pacing.
- Setup draws before timing, warming layout/glyph state. Initial viewport,
  quads, glyphs, root renders, and panel renders are logged once per input/family.
- Each panel clips its 64 labels to 192 pixels; not every label produces visible
  glyphs, and larger inputs may extend beyond the viewport.
- Progress assertions and final glyph equality against a full refresh are
  outside timing. Glyph snapshots compare scene sprite debug data, not pixels.
- `GPUI_RETAINED_DISPATCH=1` is rejected when text inputs are registered.

The installed-font check requires an enumerated family and checks that GPUI's
resolved ID maps back to that family rather than its default fallback stack.
This mapping is not an authoritative physical font identity: it does not pin
font bytes, face index, version, or prove no per-character shaping fallback.
Record the actual font files and their hashes on the validation machine. Do not
compare the two font groups as if they were the same workload, or assume the
generic `monospace` request names an installed face.

## Warm benchmark reproduction

The warm groups have completed smoke, quick, and measured runs using this
feature configuration. Use one validation process at a time.
Linux needs `x11` or `wayland` for production text shaping; commands below select
`x11`. Do not enable `test-support`, directly or transitively.

```bash
git rev-parse HEAD
sha256sum crates/gpui_platform/benches/randomized_element_tree.rs \
  crates/gpui/src/window.rs Cargo.lock
fc-match -f '%{family}\n%{style}\n%{file}\n%{index}\n' 'Noto Sans Mono'
# Record sha256sum of the font file reported above as well.

feature_tree="$(cargo tree -p gpui_platform --no-default-features \
  --features bench-support,x11 -e normal,build,dev,features)" || exit 1
if grep -F 'feature "test-support"' <<<"$feature_tree"; then
  echo 'benchmark graph contains test-support' >&2
  exit 1
fi

cargo bench -p gpui_platform --bench randomized_element_tree \
  --no-default-features --features bench-support,x11 \
  --profile release-fast --no-run
```

Use the exact executable printed by Cargo as `$BENCH`. On Linux, run each
invocation below with its hard timeout; do not run concurrently:

```bash
timeout 300s env -u GPUI_RETAINED_DISPATCH "$BENCH" --bench 'TextPanels/' --test
timeout 300s env -u GPUI_RETAINED_DISPATCH "$BENCH" --bench 'TextPanels/' --quick
timeout 300s env -u GPUI_RETAINED_DISPATCH \
  BENCH_MEASUREMENT=foreground-instructions "$BENCH" --bench 'TextPanels/' \
  --save-baseline fresh-text-main-foreground-instructions
```

Confirm all six text inputs ran, assertions passed, and the actual feature graph
contains no `test-support` before calling the harness validated. Record the
toolchain, OS, features, font identity, renderer, and hashes alongside results.
Instruction counters may be unavailable under host permissions. Pair them with
wall time and frame reports; do not infer display latency from unpaced frames.
macOS headless rendering uses Metal; Linux presentation may discard the scene
without GPU submission. Confirm the selected platform rather than claiming GPU
coverage from CPU scene correctness.

## First populated frame and setup live heap

This addition targets `gpui-fresh-taffy-experiment` at
`4223238c9eb2ac01fc47090fc3bcfc4d21f7b800`. The preparation fork inherited an
uncommitted indexed-bounds change in `crates/gpui/src/taffy.rs`; it is not part of
this benchmark change and must not become the baseline. Transfer only this
document and `randomized_element_tree.rs` to the clean base and each candidate.
Keep their benchmark source identical and record its SHA-256 before building.
The earlier provenance above describes the original warm benchmark, not this
fresh-stack base.

The separate `TextPanels/first populated installed font/panels/{8,32,96}` group
uses the same `TextPanelRoot`/`TextPanel` rendering bodies as the six historical
warm inputs. Their mutations, layout, label bodies, and correctness assertions
are unchanged; only the inline root construction is extracted into a shared
constructor. No retained-rendering APIs or detached-entity workload are added.

Each `bench_batched_task` iteration has these boundaries:

1. **Prepare, untimed:** create and activate a new production benchmark window,
   its window-specific resources/context and sprite atlas, and empty counters.
   The harness settles empty-window creation here, not as the measured tree.
2. **Measure:** attach a newly constructed full Noto root, including all panels;
   deliver the normal production pending frame callback through layout,
   prepaint, paint, and present. The task output owns an async window context and
   root entity until after measurement. This measures construction/attachment
   plus first populated rendering, not rendering alone.
3. **Validate and teardown, untimed:** task-output destruction asserts exactly
   one root render, exactly `panel_count` panel renders (including offscreen
   panels), unchanged model values, and nonempty glyph output, then closes the
   window. It is not an `n`-view render-skip win. The harness settles destruction
   before the next setup.

The API contract was inspected statically: `bench_batched_task` dispatches
pending frames after calling the measured closure and before awaiting its task;
`Task::ready` does not bypass that dispatch. Its output and setup input are
dropped outside Criterion timing. `BenchWindowContext` alone does not close the
window on drop, hence the explicit untimed `remove_window`.

Before any timed iterations, one separate populated setup probe per input reads
`bench_metrics::allocation_stats()` at three points: before window creation,
after empty-window creation, and after the first populated frame settles. It
prints raw `live_bytes()` and signed deltas for both window-plus-setup and
population-only. Reads precede glyph snapshots and logging. The probe checks
first-frame counts and glyphs, then verifies glyph equality against a full
refresh and exactly two renders of the root and every panel. Its extra refresh
does not relax the timed iterations' exactly-one-frame assertion.

### Cache state and interpretation

- Every measured iteration has a new window, window context and atlas. The
  `BenchAppContext` and platform text system are shared: enumerating/resolving the
  explicit Noto face, the untimed probe, earlier inputs, and Criterion warmup may
  warm font, shaping, allocator, and platform caches. This is **fresh-window,
  first-populated-frame with potentially warm shared resources**, not a
  completely cold process or first-use font-load measurement.
- The once-per-input probe runs before that input's timed samples, but after
  font resolution and possibly earlier groups/inputs. Use identical filters and
  ordering on baseline/candidate. Run individual inputs in separate fresh
  processes to compare less-warmed setup probes; even then font resolution is
  intentionally before the heap baseline and probe before timing.
- Live heap is a process-wide requested-allocation estimate of window-held
  setup cost, **not** exclusive ownership, peak heap, RSS, or GPU/driver memory.
  Shared cache growth, retained allocator resources, background allocation/free
  noise, and setup order can affect it. Negative deltas are preserved, not
  clamped. Missing allocator statistics print unavailable, not zero.
- Explicit family remains `Noto Sans Mono`; retain the same Fontconfig file,
  face index and full font SHA-256 as the parent baseline (the supplied prefix
  `65b...` is not a substitute for the full hash). The supplied 1920x1080,
  48-quads/8138-glyph scene is a comparison checkpoint for the corresponding
  input/platform, not a portable expected count. The new probe logs actual
  viewport, quads, glyphs, root and panel counts.
- The original static preparation logged the requested `GPUI_LAYOUT_CACHE`
  value as unverified. The measured harness now checks the window's actual
  read-only mode against the requested value during untimed validation.
  The reference comparison branch supplies a benchmark-only getter returning
  false; the candidate getter reads its actual engine. Both
  `GPUI_RETAINED_DISPATCH` and `GPUI_VIEW_RETENTION` must remain unset in all
  comparison runs; the new input registration asserts this.
- The prior supplied warm Noto/96 comparison (25.84 ms original to 17.73 ms
  TT14 stack) is not first-frame or initial-heap evidence. No new measurements
  have been run or results inferred here.

### Parent-owned serial validation plan

Static preparation ran no Cargo commands. Subsequent serial smoke, quick,
and measured runs passed. For reproduction, run feature isolation and the optimized build
above on the clean base, recording toolchain, lockfile, flags, platform/renderer,
font identity and both retention variable values (`None`). Set `$BENCH` to the
exact resulting executable. Each command below is separately bounded to 300 s;
use terminal `timeout_ms=300000` as well. Keep `GPUI_LAYOUT_CACHE` unset for the
baseline. Do not compare this baseline to the inherited indexed-bounds diff.

```bash
# FirstPopulatedSmoke
timeout 300s env -u GPUI_RETAINED_DISPATCH -u GPUI_VIEW_RETENTION -u GPUI_LAYOUT_CACHE \
  "$BENCH" --bench 'TextPanels/first populated installed font/' --test
# FirstPopulatedQuick
timeout 300s env -u GPUI_RETAINED_DISPATCH -u GPUI_VIEW_RETENTION -u GPUI_LAYOUT_CACHE \
  "$BENCH" --bench 'TextPanels/first populated installed font/' --quick
# FirstPopulatedBaseline (default wall time plus available secondary counters)
timeout 300s env -u GPUI_RETAINED_DISPATCH -u GPUI_VIEW_RETENTION -u GPUI_LAYOUT_CACHE \
  "$BENCH" --bench 'TextPanels/first populated installed font/' \
  --save-baseline fresh-taffy-first-populated
```

Validate the six old warm inputs independently with their unchanged filters.
Then apply identical benchmark source to each candidate and repeat smoke,
quick, and measured runs serially with separate baseline names. Only after the
actual-mode seam is verified may the cache candidate use `GPUI_LAYOUT_CACHE=1`.
Report first-populated wall-time confidence intervals, frame/foreground
reports, secondary metrics, all correctness counts and setup heap deltas side
by side. Linux present may discard the scene rather than submit GPU work;
these runs cannot establish end-to-end display latency.

## Layout-cache prototype comparison

The opt-in prototype reuses authoritative Taffy nodes by identified scope plus
local request ordinal. Fresh `Render`, dispatch construction, and scene
construction still run each frame. All opaque measurement callbacks invalidate
layout. Built-in text carries canonical final-layout lines only when the full
descriptor matches; preliminary size probes cannot overwrite them. Text,
runs, style, font generation, rem size and scale changes reject reuse.

Baseline: `gpui-fresh-first-frame-baseline` at `b9fe84440c`, Taffy-only stack
plus benchmark instrumentation and a reference-mode getter. Candidate:
`gpui-fresh-layout-comparison` at `783d5b59fd`, including the validated
`d7ee748cce` / `826a2e58cc` / `dbd983cf52` prototype. Both used benchmark SHA-256
`a95362d2222f3cfa63714b55988dd231ab75001a8963e60aadcb8006224a28aa`
and identical lockfile SHA-256
`a25b0e36e2cafe3d9b38ec79819f7ebc168fbd6f285b6f98dc05bad9e4e3dc41`.
The earlier unverified-mode first-frame captures are not used below.

| Installed Noto, 96 panels | Taffy-only baseline | Candidate cache off | Candidate cache on |
| --- | --- | --- | --- |
| Warm foreground instructions | 236.02 M | 238.10 M | 154.73 M |
| Warm wall median | 18.189 ms | 18.184 ms | 11.205 ms |
| Warm foreground allocations/frame | 77,556 | 77,556 | 46,941 |
| First populated foreground instructions | 252.36 M | 254.31 M | 286.83 M |
| First populated wall median | 18.517 ms | 18.850 ms | 21.236 ms |
| First populated foreground allocations/frame | 78,400 | 78,406 | 91,881 |
| Population setup live-heap delta | 59,576,733 B | 59,675,149 B | 60,499,005 B |

On the same candidate binary, cache-on saves 35.0% of warm instructions,
38.4% of warm wall time and 39.5% of warm foreground allocations. It adds
12.8% first-populated instructions, 12.7% first-populated wall time and
17.2% first-populated allocations. The prototype's disabled path itself adds
about 0.9% warm instructions versus the Taffy-only implementation. This is an
experiment, not a startup-parity or universal speedup claim.

Warm instruction confidence intervals were 237.95–238.23 M off and
154.69–154.78 M on; first populated intervals were 254.20–254.42 M off and
286.63–287.06 M on. Warm wall sample ranges were 17.805–19.103 ms off and
11.041–12.717 ms on; first populated ranges were 18.499–25.905 ms off and
20.855–22.462 ms on. Wall-time ranges are not confidence intervals.
Heap deltas are once-per-input process-wide estimates, not exclusive window
ownership or original-mainline startup measurements.

All nine text smoke inputs and the complete seed-3 randomized smoke suite
passed in both modes. The resolved benchmark feature tree had zero
`test-support` occurrences. Prototype validation before parent integration
passed 397 GPUI tests in each mode, project Clippy, and `cargo check`.
Independent static review found no concrete measurement-boundary issue.
Mac/Metal and glyph-changing font installation remain untested.

The matched seed-3 randomized tree captures also retained correctness:

| Workload wall median | Wide off / on | Dense entities off / on |
| --- | --- | --- |
| Full refresh | 278.555 / 257.833 µs | 936.512 / 805.561 µs |
| Unchanged | 283.763 / 267.838 µs | 938.969 / 808.819 µs |
| Leaf style | 291.366 / 275.341 µs | 949.855 / 837.494 µs |
| Leaf bounds | 301.940 / 299.541 µs | 953.967 / 846.973 µs |
| Root layout | 287.587 / 286.792 µs | 947.542 / 850.157 µs |
| Insert/remove | 354.140 / 331.347 µs | 1006.000 / 905.380 µs |
| Reorder | 320.180 / 316.018 µs | 996.590 / 886.469 µs |

These are modest box-tree improvements, not view-render skipping. The quick
dense-reorder capture was much worse than its measured run; do not generalize
the measured median to every structural transition. Further seed/transition
coverage is required. Logs:
`target/fresh-performance/first-frame-confirmed-baseline-measured.log`,
`layout-cache-mode-{0,1}-measured.log`, and
`layout-cache-mode-{0,1}-tree-measured.log`. All measured runs were serial on
CPU 0, with the same 20 samples / 1 s warmup / 2 s measurement configuration.

### Follow-up text trials

The profile then identified ordinary style conversion and empty text-background
painting as bounded costs. Separate preserved trials are:

- `2d90309841`: construct the default text Taffy style directly, with a full
  equality test against GPUI default conversion across rem sizes and fractional
  scales. Adds no cache or duplicated per-node style.
- `3b3b555957`: skip background bounds/layers/glyph traversal when every
  decoration run has no background. Transparent `Some` backgrounds retain the
  original path. Scene-order tests use bounded synthetic glyph primitives
  because the no-op test font backend does not rasterize glyphs; they are not
  pixel-rendering evidence.
- `d1ed32c243`: retain the first preliminary text measurement only when no
  payload exists. Subsequent probes still cannot replace existing lines;
  final layout either reuses compatible lines or publishes its own result.
  This uses the existing payload, not a second provisional snapshot.

At 96 installed-font panels, before resolved-style caching:

| Stage, cache on | Warm instructions / wall median | First-populated instructions / allocations |
| --- | --- | --- |
| Original layout-cache checkpoint | 154.73 M / 11.205 ms | 286.83 M / 91,881 |
| Default text style | 149.22 M / 10.783 ms | 281.21 M / 91,881 |
| Skip absent backgrounds | 142.04 M / 10.546 ms | 274.11 M / 91,878 |
| First-probe reuse | 141.51 M / 10.638 ms | 246.55 M / 79,585 |

The first-probe stage's same-binary cache-off warm result was
224.98 M instructions / 17.200 ms / 77,556 allocations versus cache-on
141.51 M / 10.638 ms / 46,684 allocations. First-populated cache-off was
241.03 M / 78,397 allocations. The population live-heap estimate was
59,540,272 B off versus 60,342,733 B on.

The broad capture's first-populated wall medians were 17.403 ms off versus
18.859 ms on. Two subsequent alternating, first-input-only captures were
18.236/18.413 ms and 18.210/18.356 ms off/on. Their instructions stayed near
241.2/246.4 M, allocations near 78,397/79,579, and cycles near 95.0/95.8 M.
The smaller repeatable startup premium is encouraging but is **not** parity:
instructions remain about 2.2% higher, allocations 1.5% higher, and population
live heap about 1.3% higher. Do not select a wall-time run to claim a startup win.

First-probe instruction confidence intervals in the broad capture were
224.89–225.11 M off versus 141.47–141.57 M on for warm updates, and
241.00–241.07 M off versus 246.39–246.74 M on for startup. Independent static
review found no concrete first-probe/final-hit counterexample in Taffy 0.14.
Parent validation passed 404 GPUI tests in each mode, including the preserved
strict final-hit matrix, plus all nine text smoke inputs. Earlier background
validation and the final first-probe stack both passed project Clippy.

Artifacts: `default-text-style-mode-{0,1}-measured.log`,
`empty-background-mode-{0,1}-measured.log`,
`first-probe-mode-{0,1}-measured.log`, and
`first-probe-startup-{0,1}-{0,1}.log` under `target/fresh-performance`.
All use the same benchmark-source hash above. These trials do not establish
that the implementation beats gpui-fast, and the latest text changes have not
yet been remeasured on the box-tree groups.

Resolved-style trial `e222a9e06f` subsequently passed 406 GPUI tests in each
mode, project Clippy and both complete seed-3 benchmark smoke suites. Noto/96
warm medians were 222.79 M instructions / 16.739 ms / 65,268 allocations off,
and 138.33 M / 10.172 ms / 34,396 allocations on. First-populated instructions
were 238.47 M off and 243.64 M on, with 66,111 versus 67,293 allocations.
Population live-heap estimates were 59,297,941 B off and 60,110,136 B on.
First-populated wall medians reversed relative to previous captures
(18.669 ms off, 17.588 ms on); this is not a consistent startup-latency win.
The extended seed sweep and final WGPU rerun were interrupted and have no
completed result. Logs are `resolved-style-mode-{0,1}-measured.log`.

The absent-background and resolved-style commits have also been extracted onto
the Taffy-0.13 mainline-based stack as `00fd54fc4a` and `68b81bf917`.
Those separate extractions have passed static checks only; their mainline
runtime and performance validation remains pending. The combined prototype
measurements above are not standalone mainline attribution.

## Measured fresh-only checkpoints

These are historical warm-frame results, not measurements of the new
first-populated-frame group. All stages still render the complete tree and
construct a fresh dispatch tree and scene. No view-output retention is enabled.
The workload has 96 panels with 64 labels each, updating two panels per frame.

| Stage | Generic `monospace`: instructions / wall median | Installed Noto: instructions / wall median | Noto foreground allocations/frame |
| --- | --- | --- | --- |
| Original mainline `9ab0715969`, benchmark-only port | 550.69 M / 41.085 ms | 323.73 M / 25.843 ms | 102,981 |
| Cached font-failure fix `70b9afd455` | 374.02 M / 29.040 ms | 323.54 M / 25.193 ms | 102,981 |
| Fitting measurement and lazy wrapper `f5602320cd` | 289.18 M / 21.512 ms | 276.81 M / 20.637 ms | 90,431 |
| Taffy 0.14 `4223238c9e` | 247.88 M / 18.626 ms | 236.10 M / 17.733 ms | 77,556 |

Instructions are Criterion's foreground-instruction estimates; wall times and
allocations are secondary medians from the same runs. For installed Noto/96,
instruction confidence intervals were 323.38–324.12 M on mainline,
276.64–277.02 M after fitting measurements, and 236.03–236.17 M after Taffy.
Wall-time sample ranges were respectively 24.867–26.641 ms,
20.234–25.066 ms, and 17.517–18.840 ms. These ranges are not wall-time
confidence intervals. The font fix does not meaningfully reduce instructions
when the preferred installed font resolves successfully.

Local artifacts are in `target/fresh-performance/{baseline,font,fitting,taffy}-measured.log`.
Runs used Linux x86_64, rustc 1.98.1, `release-fast`,
`--no-default-features --features bench-support,x11`, CPU 0 affinity,
`BENCH_MEASUREMENT=foreground-instructions`, 20 samples, one-second warmup
and two-second measurement, with smoke and quick checks first. The original
warm benchmark source was identical across these stages; the new
first-populated group has a different source hash and requires new baseline
and candidate captures. Taffy's dependency/lockfile change is intentional.

The installed face was `/usr/share/fonts/noto/NotoSansMono-Regular.ttf`,
Fontconfig face index 0, SHA-256
`65b5e2b2c4a1fba9ae8be1f026cb35b03dcb8886d9b2a4147054fde12f7e767d`.
Family verification is not a proof against per-character fallback.
Glyph/full-refresh assertions passed. These measurements cover CPU-side frame
production, not end-to-end display latency or GPU execution.

The separate indexed-bounds experiment `624c05b94f` is not included in the
table or the layout-cache comparison base. Alternating Noto/96 captures found
about 1.1% fewer instructions but no consistent wall-time improvement. It
remains an experiment, not a demonstrated speed win. Decoration-buffer trials
are also excluded from this stack.
