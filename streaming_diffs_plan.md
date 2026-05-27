# Plan: Speed up `StreamingDiff`

## Context

`performance_profile.miniprof.json` showed large foreground stalls attributed to `edit_file_tool`:

- 711.56 ms
- 641.01 ms
- 221.99 ms

Because miniprof records foreground task poll duration, these are active foreground-thread CPU stalls, not async wait time.

A new Criterion benchmark was added in:

- `crates/streaming_diff/benches/streaming_diff.rs`

It uses deterministic randomized Rust-like source text and benchmarks:

- `StreamingDiff::push_new`
- `StreamingDiff::finish`

Run it with:

```sh
cargo bench -p streaming_diff --bench streaming_diff --profile release-fast
```

For shorter smoke runs:

```sh
cargo bench -p streaming_diff --bench streaming_diff --profile release-fast -- --warm-up-time 1 --measurement-time 2
```

## Current benchmark smoke results

Using `release-fast` profile:

| Benchmark                                                |      Approx. time |
| -------------------------------------------------------- | ----------------: |
| `streaming_diff_push_new/tiny_function_rewrite/1510`     |             11 ms |
| `streaming_diff_push_new/small_function_rewrite/3353`    |             51 ms |
| `streaming_diff_push_new/medium_many_small_changes/5074` |        126–134 ms |
| `streaming_diff_push_new/medium_insertions/5077`         |        116–119 ms |
| `streaming_diff_finish/*`                                | sub-µs to ~1.1 µs |

Conclusion: `finish` is not the issue in these scenarios. The expensive work is in `push_new`.

## Suspected hotspot

`StreamingDiff::push_new` currently stores equal-run lengths in a hash map:

```rust
equal_runs: HashMap<(usize, usize), u32>,
```

The hot loop does hash-table work for equal characters:

```rust
let mut equal_run = self.equal_runs.get(&(i - 1, j - 1)).copied().unwrap_or(0);
equal_run += 1;
self.equal_runs.insert((i, j), equal_run);
```

It also prunes the map each `push_new`:

```rust
self.equal_runs.retain(|(_i, j), _| *j == self.new_text_ix);
```

This is likely bad for cache locality and constant factors because it puts hash lookups and inserts inside the dynamic-programming inner loop.

## Proposed optimization: replace `HashMap` with two vectors

Use two contiguous rows instead of `HashMap<(usize, usize), u32>`:

```rust
previous_equal_runs: Vec<u32>,
current_equal_runs: Vec<u32>,
```

Conceptual algorithm:

```rust
for j in self.new_text_ix + 1..=self.new.len() {
    self.current_equal_runs.fill(0);

    for i in 1..=self.old.len() {
        let equality_score = if self.old[i - 1] == self.new[j - 1] {
            let equal_run = self.previous_equal_runs[i - 1] + 1;
            self.current_equal_runs[i] = equal_run;

            let exponent = cmp::min(equal_run as i32 / 4, Self::MAX_EQUALITY_EXPONENT);
            self.scores.get(i - 1, relative_j - 1) + Self::EQUALITY_BASE.powi(exponent)
        } else {
            f64::NEG_INFINITY
        };

        // unchanged insertion/deletion/equality max logic
    }

    std::mem::swap(&mut self.previous_equal_runs, &mut self.current_equal_runs);
}
```

Why this should help:

- Removes hash lookup/insert from the hot path.
- Uses contiguous memory, likely improving cache locality.
- Avoids tuple key hashing/allocation behavior.
- Avoids `HashMap::retain` per `push_new`.
- Better matches the existing column-oriented DP traversal.

Caveat: this is likely a strong constant-factor improvement, but not a full solution if `push_new` remains O(old × new). Large edit blocks may still need higher-level avoidance, chunking, or direct replacement paths.

## Measuring cache behavior

Criterion does not measure cache hits/misses by itself. It measures elapsed time and can report throughput, but hardware counter data needs an external profiler.

### Linux: `perf stat`

On Linux, use hardware counters around the bench binary:

```sh
cargo bench -p streaming_diff --bench streaming_diff --profile release-fast --no-run
```

Then run the produced binary under `perf stat`, for example:

```sh
perf stat -d target/release-fast/deps/streaming_diff-<hash> --bench streaming_diff_push_new --warm-up-time 1 --measurement-time 2
```

Useful events:

```sh
perf stat \
  -e cycles,instructions,branches,branch-misses,cache-references,cache-misses,L1-dcache-loads,L1-dcache-load-misses,LLC-loads,LLC-load-misses \
  target/release-fast/deps/streaming_diff-<hash> --bench streaming_diff_push_new --warm-up-time 1 --measurement-time 2
```

Compare before/after:

- wall time from Criterion
- instructions
- cycles
- branch misses
- cache references/misses
- L1D miss rate
- LLC miss rate

### macOS: Instruments / `xctrace`

On macOS, use Instruments. Options depend on hardware and OS permissions.

Recommended approaches:

1. Use Instruments.app with the **Time Profiler** template to confirm where CPU time is spent.
2. If available, use the **CPU Counters** / hardware counter instruments to inspect cache-related counters.
3. Run the Criterion bench binary directly under Instruments rather than wrapping `cargo bench`, so the profiler attaches to the benchmark process.

Build first:

```sh
cargo bench -p streaming_diff --bench streaming_diff --profile release-fast --no-run
```

Then profile the produced binary under Instruments. If using `xctrace`, inspect available templates first:

```sh
xcrun xctrace list templates
```

Then record the benchmark binary with the relevant template. Template names vary by Xcode/macOS version, so verify locally with the command above.

### Scriptable macOS workflow

`xctrace` is scriptable enough for PR data, but the XML table schemas vary by Xcode/macOS/template. The robust workflow is:

1. Build the benchmark binary.
2. Record a `.trace` with `xctrace record`.
3. Export the trace table of contents with `xctrace export --toc`.
4. Inspect the table schemas in the TOC.
5. Export specific tables with `xctrace export --xpath`.
6. Parse the exported XML with Python.

A useful helper script can start generic: record and export TOC plus all obvious CPU tables, then let the developer refine XPath after seeing the schemas.

Example script sketch:

```python
#!/usr/bin/env python3
import argparse
import json
import os
import plistlib
import re
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path


def run(command, **kwargs):
    print("+", " ".join(command), file=sys.stderr)
    return subprocess.run(command, check=True, text=True, **kwargs)


def cargo_bench_binary(package, bench, profile):
    messages = subprocess.check_output([
        "cargo",
        "bench",
        "-p",
        package,
        "--bench",
        bench,
        "--profile",
        profile,
        "--no-run",
        "--message-format=json",
    ], text=True)
    executable = None
    for line in messages.splitlines():
        message = json.loads(line)
        if message.get("reason") == "compiler-artifact":
            target = message.get("target", {})
            if "bench" in target.get("kind", []) and target.get("name") == bench:
                executable = message.get("executable")
    if executable is None:
        raise RuntimeError("bench executable not found in cargo JSON output")
    return Path(executable)


def export_xml(trace_path, output_path, *query):
    run(["xcrun", "xctrace", "export", "--input", str(trace_path), *query, "--output", str(output_path)])


def summarize_toc(toc_path):
    tree = ET.parse(toc_path)
    root = tree.getroot()
    print("\nExportable tables:")
    for table in root.findall(".//table"):
        schema = table.attrib.get("schema")
        name = table.attrib.get("name")
        if schema or name:
            print(f"- schema={schema!r} name={name!r}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--package", default="streaming_diff")
    parser.add_argument("--bench", default="streaming_diff")
    parser.add_argument("--profile", default="release-fast")
    parser.add_argument("--template", default="CPU Counters")
    parser.add_argument("--time-limit", default="20s")
    parser.add_argument("--out", default="target/xctrace-streaming-diff")
    parser.add_argument("--bench-arg", action="append", default=[])
    args = parser.parse_args()

    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)

    executable = cargo_bench_binary(args.package, args.bench, args.profile)
    trace_path = out / f"{args.bench}-{args.template.replace(' ', '-')}.trace"
    toc_path = out / "toc.xml"

    bench_args = args.bench_arg or [
        "--bench", "streaming_diff_push_new", "--warm-up-time", "1", "--measurement-time", "2"
    ]

    run([
        "xcrun", "xctrace", "record",
        "--template", args.template,
        "--time-limit", args.time_limit,
        "--output", str(trace_path),
        "--launch", "--", str(executable), *bench_args,
    ])

    export_xml(trace_path, toc_path, "--toc")
    summarize_toc(toc_path)

    print(f"\nTrace: {trace_path}")
    print(f"TOC:   {toc_path}")
    print("\nNext step: choose schemas from the TOC and export them with:")
    print(f"xcrun xctrace export --input {trace_path} --xpath '<xpath from toc>' --output table.xml")


if __name__ == "__main__":
    main()
```

Example usage:

```sh
python3 script/xctrace-bench.py --template "CPU Counters" --time-limit 20s
python3 script/xctrace-bench.py --template "Time Profiler" --time-limit 20s
python3 script/xctrace-bench.py --template "Allocations" --time-limit 20s
```

Notes:

- `CPU Counters` is present in the local `xcrun xctrace list templates` output.
- Hardware-counter availability can still depend on machine, OS, permissions, and SIP/security settings.
- The script should not assume stable XPath schemas. Always export `--toc` first and inspect available tables.
- For PR evidence, it is acceptable to include Criterion timing plus Time Profiler/Allocations data if CPU counter export is not available.

### macOS fallback if cache counters are unavailable

If hardware cache counters are not available, use these before/after signals instead:

- Criterion timing.
- Instruments Time Profiler self time in `StreamingDiff::push_new`.
- Allocations instrument to see whether removing `HashMap` reduces allocation/churn.
- `cargo instruments` if available in the local workflow.

For the PR, it is still useful to report elapsed time plus a clear explanation that replacing hash-map inner-loop work with contiguous row vectors should improve cache locality.

## Existing correctness tests

`crates/streaming_diff/src/streaming_diff.rs` already has useful tests, including:

- deterministic line operation tests
- newline insertion/deletion tests
- `test_cleaning_up_common_suffix`
- randomized streaming diff test: `test_random_diffs`

Important randomized test behavior:

```rust
let char_operations = random_streaming_diff(&mut rng, &old, &new);
let patched = apply_char_operations(&old, &char_operations);
assert_eq!(patched, new);
```

The randomized test supports environment variables:

- `ITERATIONS`
- `SEED`
- `OLD_TEXT_LEN`

Suggested post-change validation:

```sh
cargo test -p streaming_diff
ITERATIONS=1000 OLD_TEXT_LEN=100 cargo test -p streaming_diff test_random_diffs
ITERATIONS=200 OLD_TEXT_LEN=1000 cargo test -p streaming_diff test_random_diffs
```

## Suggested additional tests

Add deterministic tests around realistic source-like edits:

1. Mostly-identical source block with one localized function rewrite.
2. Many small replacements across a source block.
3. Helper block insertion in the middle of a source block.
4. Unicode-containing source text, because `StreamingDiff` internally uses chars but `CharOperation::Delete` uses byte lengths.

Each test should assert:

```rust
apply_char_operations(old, operations) == new
```

## Suggested implementation order

1. Run current benchmark and save baseline numbers.
2. Optionally gather Time Profiler / cache-counter baseline.
3. Replace `equal_runs: HashMap<(usize, usize), u32>` with two `Vec<u32>` rows.
4. Run full `streaming_diff` tests plus randomized tests with larger `OLD_TEXT_LEN`.
5. Run benchmarks again with `release-fast`.
6. If possible, rerun hardware-counter profiling and compare cache misses/instructions/cycles.
7. Include before/after benchmark results in the PR description.

## Notes for PR write-up

Useful claims to support with data:

- `push_new` is the expensive phase; `finish` is negligible in the current benchmark.
- The current implementation performs hash-map lookup/insert in the DP inner loop.
- The new implementation uses contiguous row buffers and removes hash-map work from the hot path.
- Benchmark results should include before/after times for each fixture.
- If hardware counters are available, include cache miss and instruction/cycle deltas.
