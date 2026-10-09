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
