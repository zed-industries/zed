# GPUI renderer benchmark

This package drives a real GPUI window through deterministic editor-shaped workloads and exits without input. It records raw per-frame timings, process CPU time, renderer identity, and a Markdown comparison.

The primary comparison is valid on Windows when the legacy DirectX path reports a software-emulated DXGI adapter, or on Linux under Wayland when WGPU reports a software-emulated adapter such as Mesa lavapipe. The runner rejects a baseline that accidentally uses hardware acceleration and rejects Linux runs that are not Wayland-native.

Build the optimized runner before collecting measurements:

```powershell
cargo build -p gpui_renderer_bench --profile release-fast
```

On a GPU-less Windows system, compare the two renderers in the same executable:

```powershell
& .\target\release-fast\gpui-renderer-bench.exe compare `
  --output-dir .\target\renderer-benchmark-results `
  --rounds 3 `
  --width 1600 `
  --height 900 `
  --warmup-frames 30 `
  --measure-frames 180
```

Rounds alternate baseline-first and candidate-first to reduce thermal and ordering bias. The baseline is forced with `GPUI_RENDERER=directx` and `GPUI_D3D_ADAPTER=warp`; the candidate is forced with `GPUI_RENDERER=software`. Selecting WARP explicitly makes the legacy no-GPU path reproducible even when the benchmark host also has a physical GPU.

On a GPU-less Linux Wayland system, the same command syntax is used from a shell:

```sh
./target/release-fast/gpui-renderer-bench compare \
  --output-dir ./target/renderer-benchmark-results \
  --rounds 3 \
  --width 1600 \
  --height 900 \
  --warmup-frames 30 \
  --measure-frames 180
```

The Linux baseline is forced with `GPUI_RENDERER=wgpu` and must identify itself as a CPU adapter. The candidate is forced with `GPUI_RENDERER=software`. Both open the same native Wayland workload and use the same executable, unless `--baseline-exe` is supplied.

For the strictest upstream comparison, apply the benchmark-only commit to an unmodified upstream checkout and build one runner from each checkout. Then invoke the candidate runner with both paths:

```powershell
& .\candidate\gpui-renderer-bench.exe compare `
  --output-dir .\target\renderer-benchmark-results `
  --baseline-exe .\upstream\gpui-renderer-bench.exe `
  --candidate-exe .\candidate\gpui-renderer-bench.exe `
  --baseline-revision UPSTREAM_GIT_SHA `
  --candidate-revision CANDIDATE_GIT_SHA `
  --rounds 3
```

When `--baseline-exe` is supplied, the controller removes `GPUI_RENDERER` from that process so upstream chooses its normal no-GPU path. The candidate remains forced to `gpui_software`.

Each child process has a four-minute watchdog. The raw reports preserve every timing sample and identify the display server. `summary.json` and `summary.md` aggregate the rounds and calculate candidate changes without discarding regressions.

The scenarios are:

- `caret`: toggles a narrow caret in one editor row;
- `single_line`: changes one visible line while keeping the rest stable;
- `scroll`: advances the complete visible text viewport by one line;
- `full_frame`: changes the window and panel backgrounds.

`draw_ns` measures GPUI render-tree, layout, text, and scene construction. `renderer_present_ns` measures the platform renderer and submission/presentation call; DirectX can return before asynchronous WARP work completes, so this is labeled render/submit rather than completed-render time in the summary. `dirty_to_present_ns` measures from invalidation through that submission. Process CPU includes worker threads used by WARP or `gpui_software`, and frame interval measures the observable presentation cadence.

On macOS, `run` can verify that the workload and reporting are portable, but the result uses Metal and is not a valid no-GPU baseline. macOS has no upstream software Metal fallback corresponding to the Windows software DXGI adapter.

Verify that benchmark builds do not enable GPUI test support:

```sh
feature_tree="$(cargo tree -p gpui_renderer_bench -e normal,build,dev,features)"
if grep -F 'feature "test-support"' <<<"${feature_tree}"; then
  exit 1
fi
```

For an X11 software run, build the same runner and set `GPUI_RENDERER=software`
with `DISPLAY` set and `WAYLAND_DISPLAY` unset. Use `run --output PATH`; the
`compare` controller still enforces its Wayland baseline contract. X11 software
selection is explicit; automatic selection retains the existing WGPU behavior.

`GPUI_BENCHMARK_HOLD_AFTER_REPORT=1` keeps the final window open after writing the
report. This supports external exposure and resize probes against an unchanged
scene. Run this mode under a watchdog and terminate the process after probing.
