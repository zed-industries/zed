# GPUI software renderer: 23-idea verification ledger

Review baseline: `320bbe3180edae32c0f2c27f6fd8896d8cd185eb`.
Implementation result: **20 retained, 3 tried and discarded**. No new first-party
`unsafe`, intrinsics, assembly, or machine-code generation was authored. The safe
`softbuffer` API is the only retained new direct dependency. X11 uses opaque
software frames and explicit selection; native macOS presentation and broad
presenter consolidation were not added.

## Dispositions

Times below are representative measurements of the specific experiment, not
additive speedups. Core timings are Criterion central estimates in microseconds;
platform timings are medians in milliseconds. The accompanying
[measurement data](review_measurements.json) preserves confidence bounds and
platform p95 values.

| # | Idea | Result and evidence |
|---|---|---|
| 1 | Validate atlas bounds before SIMD | **Retained.** Reject wrong texture kind, negative origins, nonpositive sizes, overflow, and out-of-texture rectangles before dispatch. Public malformed-tile tests and native AVX2 parity pass. `bb4d08cfec` |
| 2 | Include atlas generations in damage | **Retained.** Texture deletion/reuse changes sprite identity; adding another glyph does not invalidate existing tiles. A 260-replacement regression verifies changed pixels and subsequent clean frames. `e447fd2990` |
| 3 | Clamp oversized borders | **Retained.** Border bands stay within the quad and partition its interior without overlap. Oversized and asymmetric border tests pass. `493a46f625` |
| 4 | Preserve path antialiasing and thin paths | **Retained.** Conservative fractional bounds and Vello coverage replace the binary cutoff. Subpixel-width paths remain visible. `493a46f625` |
| 5 | Render path gradients and opacity | **Retained.** Coverage modulates the gradient, including descending transparent stops. Ascending/descending opacity tests pass. Final review also reproduced and fixed stale gradient pixels when fractional path bounds changed inside the same snapped rectangle. `493a46f625`, `ac5abe2` |
| 6 | Handle singular sprite transforms | **Retained.** Noninvertible/nonfinite transforms yield empty sprites. Regression tests verify that they do not paint unrelated atlas pixels. `493a46f625` |
| 7 | Adapt parallelism to dirty work | **Retained.** Up to 32 dirty cells use the calling thread; larger work retains Rayon bands. Single-line updates: 355→218 µs at 1080p, 556→421 µs at 4K, before subsequent allocation improvements. Captures identical. `045c332009` |
| 8 | Separate Windows wakeup from exposure | **Retained.** Software vsync uses internal paint requests; real exposure, resize, and failed presentation still force repairs. Caret rendering/presentation: 0.939→0.323 ms; scroll: 2.906→1.327 ms. Three externally corrupted pixels were repaired exactly without changing the scene. `7632679e58` |
| 9 | Prepare paths once and reuse Vello scratch | **Retained.** Lazy per-path preparation and bounded worker-local scratch avoid rebuilding a path for every cell. Large path: 483→345 µs. Captures identical. `16f8d6943e` |
| 10 | Composite premultiplied path pixels directly | **Retained.** Large path: 345→313 µs. Exhaustive valid 8-bit channel combinations match a floating-point source-over reference. Removing the previous double rounding changes captured channels by at most 1/255; the exhaustive worst case versus the old formula is 2/255. `326b3f96b5` |
| 11 | Integer sampling for one-to-one sprites | **Retained.** Avoid floating-point coordinate conversion for untransformed, unscaled sprites. Full 1080p: 530→485 µs; 4K scroll: 836→760 µs. Coordinate parity covers clipping, origins, and varied widths; captures identical. `b6e323f5f0` |
| 12 | Retain bounded font/gradient lookup tables | **Retained.** Cache content and correction settings; trim beyond 256 entries. Gradient workload: 455→268 µs. Capacity and font-setting invalidation tests pass; captures identical. `1441c81ec8` |
| 13 | Reuse frame allocations and borrow paths | **Retained.** Reuse lowered operations and cell vectors, and borrow scene paths. Unchanged 1080p: 177→121 µs; unchanged 4K: 371→244 µs. Captures identical. `88ae1dae3e` |
| 14 | Reset hidden cell history at an opaque cover | **Retained.** Both preceding operations and their hashes are discarded when fully covered. Occluded workload: 205→46 µs, with zero damage after invisible changes. Regression failed before the fix. `c44f5c40ce` |
| 15 | Remove transient indices from damage hashes | **Retained.** Hash path/gradient content rather than frame-local array positions. Inserting an unrelated path preserves clean visible cells. Regression failed before the fix. `c44f5c40ce` |
| 16 | Preserve sparse damage and coalesce rectangles | **Retained.** Merge contiguous runs vertically; use row spans when fragmentation exceeds 64 rectangles. Full 4K presentation rectangles: 68→1. Sparse-gap, edge-clipping, and fragmented-grid coverage tests pass; core timing does not regress. `f3599513ae` |
| 17 | Reuse the freshest released Wayland buffer | **Retained.** Prefer recent released contents and allocate extra buffers only while needed. Native trace: two rather than three 5.76 MB mappings, saving 5.76 MB in this workload. Rendering/presentation medians remained effectively unchanged; busy-buffer exclusion and damage tests pass. `64403aa436` |
| 18 | Replace temporary files with `memfd` | **Discarded after implementation and native testing.** The safe rustix prototype passed sizing tests and used real anonymous mappings. Fedora already uses tmpfs for `/tmp`; two comparison runs showed no consistent speed benefit. Removed the implementation and dependency change. An independently measured need on a disk-backed temporary directory would justify revisiting it. |
| 19 | Add safe portable SIMD | **Discarded after implementation, parity testing, and benchmarking.** ARM64 `wide::u32x4` monochrome blending matched scalar output for all coverage values and varied row tails; frame captures were identical. 1080p was essentially unchanged, while 4K was slightly slower (full: 692→702 µs; scroll: 615→619 µs). Removed all prototype code and dependencies. |
| 20 | Evaluate softbuffer and X11 support | **Retained as X11 support.** Explicit `GPUI_RENDERER=software` now works on X11 through safe softbuffer calls. Fixed missing `gpui/x11` feature propagation and an idle exposure flush bug found by native probes. Three exposure repairs and a resize round trip reproduce the full client image exactly. Software scenarios complete at 0.51–1.18 ms rendering/presentation; existing WGPU smoke passes. Kept the specialized Windows/Wayland presenters. `60c65d3957` |
| 21 | Skip eye candy before scene construction | **Discarded after implementation and measurement.** A platform capability skipped drop/inset shadow construction, tested with two shadowed editor panels. Draw-time changes were mixed (caret 4.025→4.170 ms, single line 4.046→3.955 ms, scroll 3.149→3.060 ms), without a reliable benefit supporting a new cross-platform API. Removed the capability and fixture changes. Existing software simplifications remain. |
| 22 | Remove redundant gradient affine branch | **Retained.** Replace identical branches with the shared constant. Covered by gradient and pixel parity tests. `493a46f625` |
| 23 | Make benchmarks detect real rendering work | **Retained.** Sixteen distinct glyph masks, thirteen row colors, viewport-derived row counts, 14 px/3 px scrolling, unchanged frames, gradients, large paths, and occlusion. Assertions require interior scroll changes, nonempty damage, and equality with forced redraws in both directions. Thirteen final smoke scenarios pass. `cc24dd285a` plus later fixtures |

## Final before/after core comparison

Same strengthened fixture, optimized binaries, 25 samples per case. These
end-to-end core measurements include the retained correctness fixes.

| Workload | Before (µs) | After (µs) | Reduction |
|---|---:|---:|---:|
| full-1080p | 525.02 | 421.45 | 19.7% |
| single-line-1080p | 335.73 | 148.74 | 55.7% |
| scroll-1080p | 489.20 | 391.55 | 20.0% |
| smooth-scroll-1080p | 487.93 | 385.89 | 20.9% |
| unchanged-1080p | 167.76 | 119.41 | 28.8% |
| full-4k | 885.70 | 693.65 | 21.7% |
| single-line-4k | 536.56 | 280.00 | 47.8% |
| scroll-4k | 813.45 | 607.78 | 25.3% |
| smooth-scroll-4k | 803.04 | 609.70 | 24.1% |
| unchanged-4k | 358.50 | 241.85 | 32.5% |
| large-path | 525.27 | 292.79 | 44.3% |

## Verification scope

Final gates passed: 25 ARM64 core tests, 26 native x86-64 core tests (including
AVX2), one Windows presenter test, two Wayland presenter tests, thirteen benchmark
smoke scenarios, Clippy for the core and both changed native platform crates plus
the native Windows benchmark package, formatting, and benchmark feature isolation
on macOS, Fedora, and Windows.

- M5 Max macOS ARM64: headless software-core tests, Criterion, and captured framebuffer comparisons. This does not claim a native macOS software window presenter.
- Windows x86-64, 24 logical CPUs: native AVX2 core tests, Windows presenter unit test, interactive scheduled-task UI runs, and external Win32 exposure repairs. Waking a desktop app through an ordinary SSH session was unsuitable; measured runs used the active desktop.
- Fedora Cloud 44 ARM64, 12 vCPUs/32 GiB, native Apple VZ through Lima: Weston headless/pixman Wayland and Xvfb/Openbox X11. X11's measured client was 1598×873 after window decorations; Wayland and Windows clients were 1600×900. Do not compare their timings as if the environments were identical.
- Core randomized differential tests compare incremental and forced redraws over 100 seeded scene changes. Atlas reuse, borders, transforms, thin paths, gradient direction, cache invalidation, occlusion, and fragmented damage have targeted regressions.
- Final captures match the corrected premultiplied reference byte for byte. The intentional AA/gradient fixes cannot match the original buggy output; their targeted correctness tests establish expected behavior instead.
- Native GUI probes establish presentation and exposure behavior in the tested environments. They are not exhaustive validation of every compositor, GPU driver, desktop interaction, or FreeBSD installation.

## Reproduction

Build once before timing. Every benchmark run used a watchdog of at most 300 seconds,
with smoke and quick checks before measurement; measurements ran sequentially.
The two stalled X11 startup attempts produced no reports and are excluded.

```sh
cargo test --locked -p gpui_software --lib
cargo bench --locked -p gpui_software --bench frame --no-run
# Use the executable path printed by Cargo:
timeout 60 FRAME_EXECUTABLE --test
timeout 100 FRAME_EXECUTABLE --bench --quick --noplot
timeout 120 FRAME_EXECUTABLE --bench --noplot \
  --sample-size 25 --warm-up-time 0.3 --measurement-time 1
```

Set `GPUI_SOFTWARE_CAPTURE_DIR` on the smoke run to write raw ARGB captures.
The baseline executable uses the original renderer with the strengthened T30
fixture. Each isolated experiment also preserved its immediately preceding binary.

```sh
cargo build --locked -p gpui_renderer_bench --profile release-fast
GPUI_RENDERER=software timeout 100 target/release-fast/gpui-renderer-bench run \
  --output result.json --width 1600 --height 900 \
  --warmup-frames 30 --measure-frames 180
```

Use the platform's display/session environment. The Windows executable suffix is
`.exe`. The `run` command permits software-to-software comparisons; `compare`
intentionally forces a different renderer for its baseline.

```sh
GITHUB_ACTIONS=1 ./script/clippy -p gpui_software
GITHUB_ACTIONS=1 ./script/clippy -p gpui_linux       # Fedora
GITHUB_ACTIONS=1 ./script/clippy -p gpui_windows -p gpui_renderer_bench # Windows
cargo fmt --all -- --check
./script/check-gpui-renderer-bench-features
cargo tree --locked -p gpui_software -e normal,build,dev,features
```

Windows runs require the MSVC developer environment. When invoking the repository
Clippy script from Git Bash, set Cargo's target linker to the MSVC `link.exe`;
otherwise Git's unrelated `/usr/bin/link.exe` can be selected. Production benchmark
feature graphs were checked separately from platform tests using `test-support`.

Ignored working evidence and discarded prototype patches are in
`target/astra-review/`. The retained measurement summary is intended to make
this review assessable without those build artifacts.
