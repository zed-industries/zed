# ZED-84X Stack Overflow Crash Investigation

## Summary

Sentry issue `ZED-84X` reports a Windows-only fatal crash:

- Exception: `EXCEPTION_STACK_OVERFLOW`
- Crash address: `0x7ff7380fcc57`
- Version: `1.3.3`
- Channel: `preview`
- OS: Windows `10.0.26200`
- Crashed thread: `RayonWorker6`
- Process uptime from minidump: about `1596` seconds (~26 minutes)
- Minidump path during this investigation: `/tmp/minidump.dmp`
- Full saved stack trace path during this investigation: `/tmp/stacktrace.txt`

The crash appears to occur while Zed is constructing a text buffer for a file load. The key path is:

```text
project::buffer_store::open_buffer
→ worktree.load_file
→ text::Buffer::new
→ Rope::from
→ Rope::push
→ Rope::push_large
→ SumTree::par_extend
→ SumTree::from_par_iter
→ rayon::iter::plumbing::bridge_producer_consumer::helper<T>
→ __chkstk
→ EXCEPTION_STACK_OVERFLOW
```

This points to stack exhaustion in Rayon parallel iterator machinery while building `Rope`/`SumTree` structures for loaded text. It does not look like a GPU/driver crash, despite many NVIDIA driver threads being present in the dump.

## Evidence

### Crashed thread

The Sentry stack shows the crashed thread as `RayonWorker6`:

```text
Thread 67224 Crashed:
0 Zed.exe __chkstk
1 Zed.exe rayon::iter::plumbing::bridge_producer_consumer::helper<T> (mod.rs:393)
```

Several other Rayon workers are in the same recursive Rayon bridge/join machinery. Some are actively creating rope chunks or summaries, for example:

```text
rope::Chunk::new
sum_tree::SumTree::from_par_iter
sum_tree::SumTree::par_extend
rope::Rope::push_large
rope::Rope::push
rope::impl$1::from
text::Buffer::new
project::buffer_store::open_buffer
```

### Minidump confirmation

Running `minidump-stackwalk /tmp/minidump.dmp` locally confirmed:

```text
Operating system: Windows NT 10.0.26200
CPU: amd64, 16 CPUs
Crash reason: EXCEPTION_STACK_OVERFLOW
Crash address: 0x00007ff7380fcc57
Crashing instruction: mov byte [r11], 0x0
Process uptime: 1596 seconds
Thread 8 RayonWorker6 (crashed) - tid: 67224
```

The minidump dump also reported:

```text
MinidumpThreadList
  thread_count = 1012
```

A thread count over 1000 is unusual and supports the hypothesis that the process was under very high concurrent workload or had accumulated many background/blocking threads.

### Code path details

Relevant files:

- `crates/project/src/buffer_store.rs`
- `crates/text/src/text.rs`
- `crates/rope/src/rope.rs`
- `crates/rope/src/chunk.rs`
- `crates/sum_tree/src/sum_tree.rs`
- `crates/zed/src/main.rs`

`BufferStore::open_buffer` loads file text and constructs a `text::Buffer` on the background executor:

```rust
let text_buffer = cx
    .background_spawn(async move {
        text::Buffer::new(ReplicaId::LOCAL, buffer_id, loaded.text)
    })
    .await;
```

`text::Buffer::new` normalizes line endings and constructs a rope:

```rust
let mut base_text = base_text.into();
let line_ending = LineEnding::detect(&base_text);
LineEnding::normalize(&mut base_text);
Self::new_normalized(replica_id, remote_id, line_ending, Rope::from(&*base_text))
```

`Rope::from` calls `Rope::push`, which delegates to `push_large` when the text is above a threshold:

```rust
if text.len() > NUM_CHUNKS * chunk::MAX_BASE - NUM_CHUNKS * 4 {
    return self.push_large(text);
}
```

In normal non-test builds:

- `NUM_CHUNKS = 4`
- `chunk::MAX_BASE = 128`

So `push_large` is used for text larger than approximately `496` bytes. However, `push_large` only enters Rayon if the generated chunk count exceeds `PARALLEL_THRESHOLD`.

In `Rope::push_large`:

```rust
#[cfg(not(all(test, not(rust_analyzer))))]
const PARALLEL_THRESHOLD: usize = 84 * (2 * sum_tree::TREE_BASE);

if new_chunks.len() >= PARALLEL_THRESHOLD {
    self.chunks
        .par_extend(new_chunks.into_par_iter().map(Chunk::new), ());
} else {
    self.chunks
        .extend(new_chunks.into_iter().map(Chunk::new), ());
}
```

In `sum_tree` normal builds:

```rust
#[cfg(not(test))]
pub const TREE_BASE: usize = 6;
```

Therefore:

- `PARALLEL_THRESHOLD = 84 * (2 * 6) = 1008` chunks
- Each chunk is up to 128 bytes
- Rayon is used for files roughly >= 129 KB of text, depending on UTF-8 chunk boundaries

This threshold is low enough that many ordinary generated/source files can trigger parallel rope construction.

`SumTree::from_par_iter` uses Rayon recursively with small chunk groups:

```rust
let mut nodes = iter
    .into_par_iter()
    .chunks(2 * TREE_BASE)
    .map(|items| { ... })
    .collect::<Vec<_>>();

let mut height = 0;
while nodes.len() > 1 {
    height += 1;
    nodes = nodes
        .into_par_iter()
        .chunks(2 * TREE_BASE)
        .map(|child_nodes| { ... })
        .collect::<Vec<_>>();
}
```

Since `2 * TREE_BASE` is only `12`, the parallel units are small. Under many concurrent buffer loads, many nested Rayon jobs can be created and recursively joined/stolen, matching the repeated `rayon_core::join::join_context` frames in the crash report.

### Rayon stack size configuration

Zed configures the global Rayon pool in `crates/zed/src/main.rs`:

```rust
rayon::ThreadPoolBuilder::new()
    .num_threads(std::thread::available_parallelism().map_or(1, |n| n.get().div_ceil(2)))
    .stack_size(10 * 1024 * 1024)
    .thread_name(|ix| format!("RayonWorker{}", ix))
    .build_global()
    .unwrap();
```

The CLI and remote server configure similar Rayon pools. The configured Rayon worker stack is already 10 MB, so simply increasing stack size may hide the symptom but is not ideal. With 1000+ total process threads, increasing thread stack reservations could worsen memory pressure.

## Probable cause

The most likely root cause is a concurrency/parallelism interaction:

1. Zed opened or loaded many text buffers concurrently.
2. Many of those files were large enough to enter `Rope::push_large` and then `SumTree::par_extend`.
3. Each rope construction submitted recursive Rayon parallel iterator work.
4. Rayon workers recursively split and joined nested jobs while many background tasks waited on results.
5. A Rayon worker exhausted its stack and Windows raised `EXCEPTION_STACK_OVERFLOW`.

This is likely Windows-specific in how it surfaces because of Windows stack guard behavior, Windows threadpool scheduling, and the exact stack/exception semantics. The underlying stress pattern is cross-platform, but Linux may not reproduce the same fatal signature.

## What probably happened from the user's perspective

The exact user action cannot be proven from the stack alone. The dump does not include a direct UI action breadcrumb in the provided report.

However, based on the stack and process state, plausible scenarios are:

1. **Bulk buffer opening after the app had already been running**
   - The process uptime was ~26 minutes, so this was probably not an immediate startup crash.
   - The user may have opened many files/tabs, clicked many search/diagnostic/symbol results, or triggered a feature that opened many buffers.

2. **Session or project restore with many previously open tabs**
   - Still possible if restore/loading was lazy or delayed until some later action.
   - Less likely as an immediate launch restore because uptime was ~26 minutes.

3. **Agent/tooling reading or editing many files**
   - Many agent tools call `project.open_buffer` for reads/edits/mentions/diagnostics.
   - This is plausible but not proven by the report.

4. **Workspace containing many moderately large generated files**
   - Files only need to be roughly >= 129 KB to trigger parallel rope construction.
   - A set of generated JS/JSON/lock/log files could be enough if opened concurrently.

A single huge file could contribute, but the 1012-thread state makes "many concurrent operations" more likely than "one file alone".

## Why this is not likely GPU-related

The stack trace includes many `nvwgf2umx.dll` NVIDIA driver threads, but they are waiting in driver routines. The crashing thread and the active Zed frames are in text buffer loading and Rayon. No GPU frames are on the crashing stack.

## Reproduction ideas

### Best reproduction environment

Use Windows if possible. This issue is likely easiest to reproduce on Windows because the observed crash is a Windows `EXCEPTION_STACK_OVERFLOW`.

### Stress pattern to reproduce

Create a workspace with many moderately large text files and trigger many concurrent `open_buffer` calls. File sizes should exceed the Rayon threshold for rope construction; use files above ~129 KB, preferably several hundred KB to a few MB.

Possible stress tests/actions:

- Restore a workspace with many open tabs pointing to 129 KB+ text files.
- Open many search results or diagnostics that materialize buffers.
- Use agent/tooling to read many files concurrently.
- Add a test that constructs a fake project/worktree with many files and calls `project.open_buffer`/`buffer_store.open_buffer` for all of them concurrently.

### Linux reproduction caveat

A Linux machine can validate the workload pattern, but may not reproduce the Windows crash. On Linux, the same workload may complete, slow down, hit memory/thread limits, or fail differently.

To make Linux reproduction more likely, one could temporarily lower Rayon worker stack size in the global pool setup, but this requires changing test/runtime setup. Zed currently sets Rayon worker stack size to 10 MB in `crates/zed/src/main.rs`; tests may need their own Rayon pool or a separate process because Rayon global pools can only be initialized once.

## Potential fixes to investigate

### 1. Raise or platform-gate the rope parallelization threshold

In `crates/rope/src/rope.rs`, `PARALLEL_THRESHOLD` is currently about 1008 chunks (~129 KB). Consider raising it substantially, especially on Windows, so ordinary moderately large files use sequential construction.

Pros:

- Simple and low-risk.
- Avoids Rayon recursion for common file sizes.
- Likely directly mitigates this crash.

Cons:

- Could slow opening of some large files.

### 2. Coarsen `SumTree::from_par_iter` parallel granularity

`SumTree::from_par_iter` chunks work in groups of `2 * TREE_BASE`, which is only 12 nodes/items in normal builds. Consider a larger Rayon granularity or a sequential fallback for moderate input sizes.

Pros:

- Addresses the underlying over-parallelization.
- May reduce job overhead even outside this crash.

Cons:

- Requires more careful benchmarking and testing.

### 3. Throttle concurrent buffer construction

Add a limiter/semaphore around expensive buffer construction or file opening so many `Rope::push_large` calls cannot enter Rayon simultaneously.

Pros:

- Addresses the 1000+ thread/concurrency symptom.
- Could improve responsiveness under bulk open/restore scenarios.

Cons:

- More architectural.
- Needs careful UI/error propagation.

### 4. Improve telemetry/crash context

Add non-sensitive context around buffer construction failures/performance:

- file size
- chunk count
- whether rope construction used Rayon
- number of concurrently loading buffers
- maybe open-buffer caller category if available

Avoid recording paths or file contents unless privacy expectations allow it.

## Files inspected

- `crates/sum_tree/src/sum_tree.rs`
  - `TREE_BASE`
  - `SumTree::from_par_iter`
  - `SumTree::par_extend`
- `crates/rope/src/rope.rs`
  - `Rope::push`
  - `Rope::push_large`
  - `From<&str> for Rope`
- `crates/rope/src/chunk.rs`
  - `Bitmap`
  - `MAX_BASE`
  - `Chunk::new`
- `crates/text/src/text.rs`
  - `Buffer::new`
- `crates/project/src/buffer_store.rs`
  - `LocalBufferStore::open_buffer`
  - `BufferStore::open_buffer`
- `crates/zed/src/main.rs`
  - global Rayon pool setup

## Useful commands run

```sh
ls -lh /tmp/minidump.dmp /tmp/stacktrace.txt
file /tmp/minidump.dmp
minidump-stackwalk /tmp/minidump.dmp | head -n 180
minidump-stackwalk --dump --brief /tmp/minidump.dmp | head -n 140
git --no-pager log --oneline -n 20 -- crates/rope/src/rope.rs crates/sum_tree/src/sum_tree.rs
git --no-pager log --oneline -S "stack_size(10 * 1024 * 1024)" -- crates/zed/src/main.rs crates/cli/src/main.rs crates/remote_server/src/server.rs
```

## Suggested next step

Write a repro/stress test that opens many ~256 KB to multi-MB files concurrently through the project/buffer-store API. Then experiment with one of:

- raising `PARALLEL_THRESHOLD`,
- making the threshold platform-specific for Windows,
- coarsening `SumTree::from_par_iter`,
- limiting concurrent buffer construction.

Validate on Windows if possible. Linux-only validation should be treated as weak evidence because the crash manifestation is Windows-specific.
