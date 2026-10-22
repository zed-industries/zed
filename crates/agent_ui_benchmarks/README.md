# Agent model search benchmark

This benchmark measures the search pipeline shared by the agent model and ACP
configuration-option pickers, including candidate construction, matching, ranking,
and result cloning. It records both completion latency and foreground task-poll
durations, so work moved to a background task is not mistaken for eliminated work.

The benchmark is a separate package because `agent_ui`'s dev dependencies enable
`test-support`. It compiles the production `model_search` module directly rather
than maintaining a benchmark-only copy of the search algorithm. The runner checks
the complete package feature graph and refuses to run if `test-support` leaks in.

From the repository root:

```sh
# Smoke: exercise every workload once and validate the results.
./script/bench-agent-model-search --test

# Quick timing check before a measured run.
./script/bench-agent-model-search --quick '1000-options-128-byte-descriptions-selective'

# Record a baseline before changing the production search implementation.
./script/bench-agent-model-search --save-baseline before

# Run the identical benchmark after the change.
./script/bench-agent-model-search --baseline before
```

Run measurements sequentially on an otherwise idle machine. Each measured case
uses 15 samples, 200 ms warm-up, and 500 ms measurement time; filter to a single
workload for longer local investigation. Use `BENCH_MEASUREMENT=wall-time` to
disable counters, or leave it unset to report supported hardware and allocation
counters alongside wall time.

Fixtures contain 100, 1,000, or 10,000 synthetic ACP-like options, with 128-byte or
1,024-byte descriptions. They cover broad, selective, combined name/metadata,
missing, and exact-name ranking queries. Workers and matcher pools are warm.
Each iteration includes one foreground clone of the input snapshot and the full
production search; fixture generation and result validation are outside timing.
The synchronous reference checks all returned value identifiers and their order.

The benchmark measures the search pipeline, not the full picker: it does not
measure provider requests, layout, painting, presentation, or live keystrokes.
No window is rendered. Foreground frame-budget overruns use GPUI's synthetic
120 FPS budget (8.33 ms); they are not display-dropped frames. The foreground
report includes warm-up and calibration, whereas Criterion's latency estimate
uses its measurement phase.
