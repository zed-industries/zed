# Layout cache churn diagnosis

No production behavior was changed. This continues the churn WIP on
`9ede260872db4e3b0e36ed72e10cf973c589c9e1`. The trees of the two supplied WIP
commits, `2a993fd965f2ab689f504f229b4b28a5e9201e8c` and `273a1f9c24`, were
identical. The continuation narrows the accounting helper to `cfg(test)` and
extends the heap probe from 1000 to 3000 frames.

## Build provenance and checks

The initial shared-target build reused a binary with no LayoutChurn group.
Its tests also reported 412 passing tests rather than the rebuilt 413.
Discard all initial shared-target results and the supplied historical numbers
as evidence for this source tree.

Following the architect's revised protocol, under
`flock /tmp/gpui-experiments.lock`, seed `$PWD/target/private` with reflinks of
the shared target's `debug` and `release-fast` directories, then touch all
tracked Rust files and Cargo manifests. All results below use
`CARGO_TARGET_DIR=$PWD/target/private`, with local workspace crates rebuilt.
Every build/test/Clippy command was locked and bounded by `timeout 900s`;
every benchmark/profile process was locked and bounded by `timeout 300s`.
The executable was copied to `target/layout-churn/randomized_element_tree`
in the same locked command as its build.

Completed checks:

- `GPUI_LAYOUT_CACHE=0 cargo test --locked -p gpui --lib`: 413 passed,
  2 ignored, no failures.
- `GPUI_LAYOUT_CACHE=1 cargo test --locked -p gpui --lib`: same result.
- `./script/clippy --locked -p gpui`: passed. The WIP's `cache_counts`
  helper previously failed dead-code lint in a non-test `test-support` build;
  it is now test-only.
- `cargo tree --locked -p gpui_platform --no-default-features
  --features bench-support,x11 -e normal,build,dev,features`: no
  `feature "test-support"` occurrence.
- `cargo bench --locked -p gpui_platform --bench randomized_element_tree
  --no-default-features --features bench-support,x11 --profile release-fast
  --no-run`: passed.
- All four churn inputs passed `--test`, `--quick`, and measured runs in both
  modes, including render-count and incremental/full-refresh glyph assertions.
- `git diff --check`: passed.

`--quick` reports **one observed renderer iteration** per workload in this
harness. In particular, remount sees the 48-panel phase only. It is smoke
evidence, not a representative steady-state measurement.

Environment: Linux 7.2.9-arch1-1, x86_64, rustc 1.98.1
(`48a229cea 2026-09-01`), CPU 0 affinity, `release-fast`, x11 production text
system. Installed font: Noto Sans Mono, Fontconfig index 0,
`/usr/share/fonts/noto/NotoSansMono-Regular.ttf`.
Linux headless presentation discards the scene; these are CPU frame-production
measurements, not GPU submission or display latency.

Benchmark source SHA-256:
`24ad3ab16b805758a153cbc88699ef0eb4f7683ed96f064d1b50d0e2614ea7a4`.
Cargo.lock SHA-256:
`a25b0e36e2cafe3d9b38ec79819f7ebc168fbd6f285b6f98dc05bad9e4e3dc41`.
Copied executable SHA-256:
`9fe7b59053ae56043c4c630db50fa0f48b436c421069f41e286587c50bcc8bb0`.

## Eviction rule

Confirmed in `crates/gpui/src/taffy.rs:121-188`: a cached request is matched
by the full GlobalElementId path within a hash bucket, then its request ordinal.
After a drawn frame, a node survives iff its ordinal is below the current
scope cursor **and** it is reachable from a compute_layout root. Removed
nodes have their contexts cleared before Taffy removal. Empty scopes are
removed; cursors reset. There is no age grace period.

The added tests exercise 1000 frames of replacements, ordinal shrinkage and
absent scopes, count live nodes/text contexts/scopes, and check weak text-state
handles are released. The removed-ID test compares fresh/cached geometry and
wrapped text after changed scale, text and style. This is bounded-state and
layout evidence, not a rasterized-pixel test.

## Heap plateau

Process-wide requested live bytes, read outside timing. Checkpoints are
multiples of ten so each workload has the same live shape at each checkpoint.

| Workload | Off bytes at 1000 / 3000 | On bytes at 1000 / 3000 | Off slope 200→1000 B/frame | On slope 200→1000 B/frame | Off / on slope 1000→3000 B/frame |
| --- | ---: | ---: | ---: | ---: | ---: |
| Replace 8/96 | 74,676,591 / 74,676,591 | 75,046,639 / 75,046,639 | 15.36 | 18.845 | 0 / 0 |
| Remount 48/96 | 72,788,988 / 72,789,052 | 73,330,816 / 73,330,816 | 15.36 | 15.36 | 0.032 / 0 |
| ID/count | 96,025,760 / 96,025,760 | 93,018,780 / 93,018,780 | 15.36 | 15.36 | 0 / 0 |
| Text 96 | 73,868,555 / 73,868,555 | 74,357,735 / 74,357,735 | 15.36 | 15.36 | 0 / 0 |

The common 12,288-byte step from 200 to 1000 is the debug frame overlay, not
layout-cache growth. `window.rs:3592` records every draw even with the overlay
hidden; `debug_overlay.rs:32,87-92` keeps at most 1000 `Duration` samples.
A standalone Rust reproduction of that exact VecDeque push/pop rule confirmed:
capacity 64 at frame 50 (1024 B), 256 at frame 200 (4096 B), and 1024 at
frames 1000/2000/3000 (16,384 B). Duration is 16 bytes on this target.
Thus the measured common 3072-byte 50→200 and 12,288-byte 200→1000 steps match
the overlay exactly; storage plateaus once capacity reaches 1024.

The text-input-history suspicion is refuted for these workloads:
`window.rs:5430-5444` forwards configuration only on change. They have no input
handler, so the default is forwarded once. The push in
`platform/test/window.rs:389-390` does not run every draw.
The additional 2788 B in cache-on replacement and 64 B in cache-off remount
were not individually attributed; neither continues growing after its
checkpoint. These probes do not establish bounds for arbitrary workload sizes
or allocator/GPU memory.

## Miss-path measurements

Command, with both retention variables unset:

```sh
flock /tmp/gpui-experiments.lock timeout 300s taskset -c 0 \
  env -u GPUI_RETAINED_DISPATCH -u GPUI_VIEW_RETENTION \
  GPUI_LAYOUT_CACHE=<0|1> BENCH_MEASUREMENT=foreground-instructions \
  target/layout-churn/randomized_element_tree --bench LayoutChurn/ \
  --save-baseline private-churn-<off|on>
```

Criterion configuration: 20 samples, 1 s warmup, 2 s measurement.

| Workload | Foreground instructions M off / on | Change | Secondary wall median ms off / on | Foreground allocations K off / on |
| --- | ---: | ---: | ---: | ---: |
| Replace 8/96 | 223.41 / 142.83 | -36.1% | 17.690 / 11.413 | 65.331 / 30.998 |
| Remount 48/96 | 168.92 / 120.32 | -28.8% | 13.041 / 9.699 | 47.236 / 26.521 |
| ID/count | 201.20 / 220.49 | +9.59% | 15.780 / 18.516 | 57.329 / 58.383 |
| Text 96 | 223.34 / 227.82 | +2.01% | 17.449 / 16.885 | 65.299 / 52.912 |

ID instruction 95% intervals: 200.77–201.65 M off, 220.25–220.75 M on.
Text intervals: 223.12–223.64 M off, 227.58–228.09 M on.
Text regresses instructions, **not wall median in this capture**.
ID wall sample ranges: 15.415–17.956 ms off, 17.924–19.917 ms on.
Text wall ranges: 16.842–18.170 ms off, 16.510–17.790 ms on.
Variable-count and remount workloads cycle through different frame costs;
Criterion calibration and per-routine reset affect the distribution. These
are same-source off/on runs, not perfectly phase-matched per-frame attribution.

## Profile attribution

Four serial captures used the copied private-target executable:

```sh
flock /tmp/gpui-experiments.lock timeout 300s taskset -c 0 \
  samply record --save-only --unstable-presymbolicate \
  -o target/layout-churn/<id|text>-<off|on>.json.gz \
  env -u GPUI_RETAINED_DISPATCH -u GPUI_VIEW_RETENTION \
  GPUI_LAYOUT_CACHE=<0|1> BENCH_MEASUREMENT=foreground-instructions \
  target/layout-churn/randomized_element_tree \
  --bench LayoutChurn/workload/<id-and-anonymous-count|text-96> \
  --profile-time 10
```

Analysis selects only main-thread stacks containing `bench_renderer`,
excluding the untimed 3000-frame probe, font setup and snapshot validation.
Selected sample counts: ID off/on 10,665/11,322; text off/on 10,079/9794.
Shares are sampled CPU-time shares, **not retired-instruction shares**.
Inclusive rows overlap and must not be summed.

| Inclusive renderer sample share | ID off / on | Text off / on | GPUI source |
| --- | ---: | ---: | --- |
| Cleanup | 5.11% / 15.61% | 5.56% / 1.45% | `taffy.rs:142` |
| Scope lookup/identity | 0.04% / 0.58% | 0.01% / 0.62% | `taffy.rs:121` |
| Node claim | absent / 7.39% | absent / 2.46% | `taffy.rs:190` |
| Text-node request | 9.78% / 4.90% | 7.48% / 6.77% | `taffy.rs:305` |
| Box-node request | 5.36% / 5.83% | 5.85% / 4.06% | `taffy.rs:252` |
| Text descriptor carry | not observed / not observed | absent / 0.54% | `elements/text.rs:646` |
| Layout compute | 33.81% / 31.81% | 36.21% / 37.72% | `taffy.rs:418` |
| Element-ID push | 0.05% / 0.06% | 0.01% / 0.04% | `window.rs:985` |

ID-on cleanup source-line attribution via llvm-addr2line:

- Context destruction at `taffy.rs:161`: 5.70% inclusive.
- Its setter's recursive dirty propagation: 1.78% inclusive.
- Taffy node removal at `taffy.rs:163`: 4.44% inclusive.
- Reachability insertion at `taffy.rs:149`: 0.51% leaf.
- Cleanup scope-loop code: 2.26% leaf; `clear` itself: 1.03% leaf.
- Text-on node claim: 1.91% leaf.

Root cause: ID churn never reuses the panel scope's nodes but still pays
cache registration plus individual eviction. Eviction calls the context
setter (which marks nodes/ancestors dirty) even for context-free boxes, then
Taffy remove. Requests create children before parents, while eviction iterates
ordinals forward: removing children repeatedly scans/compacts each old parent's
sibling vector. Taffy 0.14 remove also marks the surviving parent dirty.
Cache-off clears the node storage instead. ID-on has about 1054 extra
allocations/frame, not a dominant new per-text Box allocation problem; both
modes allocate TextMeasure boxes on ID misses. Existing nodes already bypass
descriptor comparison on an identity miss (`taffy.rs:312`).

Text churn is a different miss: identities and nodes hit, but the text
descriptor rejects carry at its first text comparison. It saves about 12,387
allocations/frame; scope lookup, node style/child checks and dirty propagation
remain, while essentially all text layout recomputes. The 0.54% carry share
does not support blaming expensive full-descriptor comparisons alone for the
4.48 M extra instructions.

## Proposed fixes, not implemented

1. **First trial: parent-first eviction within a dead scope.** Reverse request
   ordinal order where the whole scope is dead. Parents were requested after
   children; removing them first detaches the child parent pointers, avoiding
   repeated sibling-vector compaction and ancestor dirties. This is the smallest
   profile-supported change. Only apply this to evicted nodes, not live nodes.
   Cross-scope parents and nodes reparented into surviving scopes need explicit
   tests; request order is not a universal substitute for topological order.
   Expected effect: reduce the 4.44% removal bucket and some dirty cost, but
   no claim of complete parity until measured.
2. **Batch/dead-subtree removal that drops contexts without dirtying doomed
   nodes.** This requires a correct Taffy removal primitive, not omitting the
   context clear (Taffy remove leaves contexts behind). Preserve surviving
   parent/child links and invalidate any surviving affected ancestors once.
   Combine with skipping per-node reachability probes for an unvisited scope.
   Profile strongly supports architect option (b); reduce the roughly
   10-percentage-point cleanup premium, though descriptor destruction itself
   remains necessary. Do not defer without a strict memory bound.
3. **New-scope request fast path**, architect option (a), is secondary:
   resolve the scope once and append fresh nodes through a direct scope handle
   rather than probing the scope hash map for every ordinal. Use fresh
   `new_leaf_with_context` semantics on genuinely new text nodes and retain a
   cheap ordinal list for next frame. Entire new descendant scopes cannot hit
   the prior frame, but must still register for future hits. Handle interleaved
   scope execution and hash collisions; preserve full-path equality and all
   ordinal/descriptor rules on hits. Lookup alone is only ~0.6% of this profile;
   do not expect it to erase the 9.6% regression.
4. **Do not implement option (c):** path hashing is already incremental on
   ElementIdStack push (`window.rs:975-1006`); set_request_scope reads the
   cached hash. Cheaper, collision-unsafe identity keys are unjustified.

For text churn, measure a direct active-scope handle and reducing repeated
style checks separately before changing descriptor rules. There is no
profile-supported one-line change that promises cache-off cost for every
miss. Keep arbitrary callback invalidation and text descriptor guards intact.

## Artifacts

Local, ignored `target/layout-churn/` contains:
`private-{tests-off,tests-on,clippy,features,build}.log`,
`private-{smoke,quick,measured}-{0,1}.log`,
`profile-{id,text}-{off,on}.log`, the four profiles and symbol sidecars,
`profile-summary.txt`, and the copied executable. Profiles include local
metadata and should remain local. No GPU or macOS checks were run.
