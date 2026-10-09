# Fresh text baseline

Static preparation only. Do not run Cargo, smoke, benchmarks, or profiles until
the parent authorizes validation and the mainline font worker releases its
exclusive validation slot.

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

## Deferred validation

These commands have **not been run**. Use one validation process at a time.
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

## First populated frame and setup live heap (static preparation)

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
- Current fresh stack has no actual-mode query for `GPUI_LAYOUT_CACHE`.
  The probe logs the requested value explicitly as **actual mode unverified**;
  setting it to `1` is not evidence that a cache exists or is active. Parent must
  verify the future prototype's real phase-1 mode seam before making on/off
  claims. Both `GPUI_RETAINED_DISPATCH` and `GPUI_VIEW_RETENTION` must remain
  unset in all comparison runs; the new input registration asserts this.
- The prior supplied warm Noto/96 comparison (25.84 ms original to 17.73 ms
  TT14 stack) is not first-frame or initial-heap evidence. No new measurements
  have been run or results inferred here.

### Parent-owned serial validation plan

No Cargo, build, test, benchmark, or profile was run for this preparation.
After obtaining the serial slot, run feature isolation and the optimized build
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
