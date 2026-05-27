# Plan: Reduce `edit_file_tool` foreground stalls

## Profile findings

Profile analyzed: `zed-profiles/performance_profile.miniprof.json`

- Size: 109.3 MB
- Shape: foreground-only `miniprof.json`
- Total timings: 973,128
- Timings >= 1 ms: 11,580
- Captured span: ~52.5 minutes

The largest `edit_file_tool` frame-risk interval is severe:

| Location                                          |  Duration | Approx. 60 Hz frame budgets |
| ------------------------------------------------- | --------: | --------------------------: |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 711.56 ms |                          43 |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 641.01 ms |                          39 |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 221.99 ms |                          14 |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 108.90 ms |                           7 |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` |  94.46 ms |                           6 |

In the worst 2 second window around `2850.3s..2852.3s`:

| Location                                          |      Total |       Max | Hits |
| ------------------------------------------------- | ---------: | --------: | ---: |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 1629.02 ms | 711.56 ms |   10 |
| `crates/editor/src/bracket_colorization.rs:98:42` |  151.66 ms |  51.14 ms |   10 |
| `crates/agent/src/agent.rs:1267:12`               |   46.48 ms |  12.30 ms |   13 |
| `crates/agent/src/thread.rs:1918:23`              |   14.47 ms |   4.14 ms |   13 |
| `crates/action_log/src/action_log.rs:190:40`      |    4.37 ms |   1.54 ms |    3 |

Across the whole profile, the biggest aggregate foreground costs were:

| Location                                          |   Total |       Max | Hits |
| ------------------------------------------------- | ------: | --------: | ---: |
| `crates/editor/src/bracket_colorization.rs:98:42` | 59.31 s |  83.91 ms | 4069 |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` |  7.57 s | 711.56 ms | 1588 |
| `crates/agent/src/agent.rs:1267:12`               |  3.44 s |  31.98 ms | 1739 |
| `crates/action_log/src/action_log.rs:190:40`      |  3.06 s |  71.63 ms | 2059 |
| `crates/session/src/session.rs:76:16`             |  2.62 s |  30.93 ms | 1216 |

Note: the profile’s line number for `edit_file_tool.rs:252` appears to correspond to the `EditFileTool::run` foreground task wrapper in the current tree (`crates/agent/src/tools/edit_file_tool.rs` around line 264). Miniprof records the task spawn site, so the expensive work is inside the async task poll, not necessarily on that exact line.

## Working hypothesis

The actual foreground stalls are probably inside `edit_session` work invoked by `EditFileTool::run`:

- `EditSession::process_edit` / `EditSession::finalize_edit` process parser events synchronously on the foreground thread.
- `EditPipeline::process_event` can do expensive matching/diff/apply work before yielding.
- `StreamingFuzzyMatcher::resolve_location_fuzzy` scans the whole buffer for each new query line and calls fuzzy string matching.
- `StreamingDiff::push_new` does dynamic-programming work proportional to `old_text` x streamed `new_text` size.
- `apply_char_operations` applies each `CharOperation` separately through `agent_edit_buffer`, which also updates `ActionLog` per operation.
- Applying edits triggers follow-on UI/editor work, especially bracket colorization, which shows repeated 50–83 ms foreground spans.

The profile suggests the biggest win is to prevent a single `edit_file_tool` poll from doing hundreds of milliseconds of synchronous foreground work.

## Current benchmark evidence

A broader Criterion benchmark was added for the streamed `edit_file` path:

- `crates/agent/benches/edit_file_tool.rs`

It creates a fake project and file, runs the real `EditFileTool` through `AgentTool::run`, and sends realistic streamed partial JSON snapshots. This exercises the production path through `StreamingParser`, `StreamingFuzzyMatcher`, reindentation, `StreamingDiff::push_new`, buffer edit application, action-log plumbing, and final output generation.

The bench now tears down the per-iteration `TestAppContext` without leaking it. The previous `std::mem::forget(cx)` workaround was hiding a GPUI test leak-detector false positive, not a production memory leak:

- `ActionLog::track_buffer_internal` stores strong `Entity<Buffer>` handles and spawns a diff-maintenance task that also captures the buffer.
- Dropping the last `ActionLog` handle only marks the entity for deferred release; its value is dropped on the next `flush_effects` / `release_dropped_entities` pass.
- Dropping the `Task` then cancels the async task, but the cancelled future releases captured handles only after the executor is pumped.
- The benchmark `Drop` now explicitly runs an empty GPUI update and `run_until_parked()` before `cx.quit()`.
- The harness is returned as part of Criterion's routine output so this cleanup happens after the measured timer stops.

Run it with:

```sh
cargo bench -p agent --bench edit_file_tool --features test-support --profile release-fast
```

Short smoke run:

```sh
cargo bench -p agent --bench edit_file_tool --features test-support --profile release-fast -- --warm-up-time 2 --measurement-time 3
```

Current smoke results on the current branch:

| Benchmark                                                 | Approx. end-to-end time |
| --------------------------------------------------------- | ----------------------: |
| `edit_file_tool_streaming/tiny_function_rewrite/295`      |            0.97–1.06 ms |
| `edit_file_tool_streaming/small_function_rewrite/531`     |            2.23–2.24 ms |
| `edit_file_tool_streaming/medium_many_small_changes/4341` |                64–67 ms |
| `edit_file_tool_streaming/medium_insertions/4355`         |                66–69 ms |

The medium cases are still well above a 16.67 ms frame budget. The benchmark reports end-to-end tool-task CPU time, not miniprof poll duration, but because `EditFileTool::run` uses `cx.spawn` and the heavy work is on the main thread, these timings are a good proxy for foreground work that can block frames.

### Benchmark accuracy notes

The benchmark is useful, but it has important boundaries:

- It measures the real edit-session path after setup: parser, fuzzy matching, reindentation, streaming diff, buffer edits, action log, diff output, and final text generation.
- Criterion excludes per-iteration fake project / settings / thread setup and the new teardown from the measured time.
- It queues all partial payloads before starting the tool, so it represents a worst-case burst where streamed input is already available and the foreground task can process it as fast as possible. Real model streaming may insert network/model gaps between chunks, which can hide total CPU time but does not remove per-poll frame risk.
- It does not include a visible editor view, so it does not capture downstream editor work such as bracket colorization invalidations. The original miniprof data is still needed to evaluate those follow-up costs.
- It uses `FakeFs` / `Project::test`, so real filesystem, LSP, and language-server costs are intentionally minimized.
- Full-process xctrace captures Criterion warmup/setup too; setup samples such as `Templates::new`, settings initialization, glob/regex compilation, and Handlebars template compilation should be treated as profiling artifacts unless they appear inside the measured routine.

### xctrace findings

A normal `release-fast` Time Profiler trace had poor symbol attribution because macOS linker identical-code folding merged many Rust monomorphizations. A cleaner profile was captured with linker deduplication disabled:

```sh
RUSTFLAGS="-C link-arg=-Wl,-no_deduplicate -C codegen-units=1" cargo bench -p agent --bench edit_file_tool --features test-support --profile release-fast --no-run
xctrace record --template "Time Profiler" --output /tmp/zed-profiles/edit_file_tool_no_dedup.trace --launch -- target/release-fast/deps/edit_file_tool-<hash> --bench --warm-up-time 2 --measurement-time 6 --sample-size 10 medium_insertions
```

Filtered to main-thread `Running` timer samples for `medium_insertions`:

- Qualifying samples: 14,164
- `EditPipeline::process_event`: 12,045 leaf samples (85.04%), 13,009 inclusive samples (91.85%)
- `EditSession::process_edit`: 13,008 inclusive samples (91.84%)
- `StreamingFuzzyMatcher` total: ~1,698 inclusive samples (11.99%)
  - `StreamingFuzzyMatcher::push`: 857 inclusive samples (6.05%)
  - `StreamingFuzzyMatcher::resolve_location_fuzzy`: 841 inclusive samples (5.94%)
  - `strsim::normalized_levenshtein`: 784 leaf samples (5.54%)
- `agent_edit_buffer` / `apply_char_operations`: 81 inclusive samples each (0.57%)
- `text::Buffer::edit`: 57 inclusive samples (0.40%)
- `ActionLog`: ~79 inclusive samples (0.56%)
- `sum_tree`: ~104 inclusive samples (0.73%)
- `rope::Cursor`: ~21 inclusive samples (0.15%)
- `tree_sitter`: ~4 inclusive samples (0.03%)

Interpretation:

- The first no-dedup profile showed the dominant bucket as `EditPipeline::process_event`, but still did not split the 85% leaf bucket into source-level subphases.
- Splitting the four `process_event` arms into `#[cfg_attr(feature = "test-support", inline(never))]` helpers made the hotspot clear for `medium_insertions`:
  - `process_new_text_chunk`: 12,172 inclusive samples (85.71%)
  - `StreamingDiff::push_new`: 12,036 inclusive samples (84.75%), 11,532 leaf samples (81.21%)
  - `process_old_text_chunk`: ~530 inclusive samples (5.40%)
  - `StreamingFuzzyMatcher::push` / `resolve_location_fuzzy`: ~5.3% / ~5.2% inclusive
  - `strsim::normalized_levenshtein`: ~5.3% leaf
- `Reindenter` is not a material bottleneck in this fixture. With `Reindenter::{push,finish,drain}` also marked `inline(never)` under `test-support`, only `Reindenter::drain` appeared, with 3 inclusive samples (0.02%). Visible allocation/free was ~0.89%, `memmove` ~0.52%, and `memset`/`bzero` ~1.87%.
- Buffer mutation and action-log work are not dominant in this fixture: `apply_char_operations` ~0.80%, `agent_edit_buffer` ~0.79%, and `text::Buffer::edit` much smaller. They may still amplify downstream editor invalidation in real sessions.
- The next measurement/optimization step is now `StreamingDiff::push_new`, followed by the fuzzy old-text matching path.

## Plan of attack

### 1. Add targeted instrumentation first

Goal: split the current monolithic `edit_file_tool` span into actionable sub-spans.

Potential instrumentation points:

- `EditFileTool::process_streaming_edits`
- `EditSession::new`
- `EditSession::process_edit`
- `EditSession::finalize_edit`
- `StreamingParser::push_edits` / `finalize_edits`
- `StreamingFuzzyMatcher::finish` / `resolve_location_fuzzy`
- `StreamingDiff::push_new` / `finish`
- `apply_char_operations`
- `agent_edit_buffer`
- `EditSessionContext::ensure_buffer_saved`
- `EditSession::compute_new_text_and_diff`

What to collect:

- edit count
- old/new text byte lengths
- matched range size
- number of parser events
- number of `CharOperation`s
- number of buffer edits applied
- elapsed time per phase

This can be temporary logging or profiler-friendly scoped timing if available. The current miniprof data is enough to show the wrapper is slow, but not enough to rank the internal phases with confidence.

### 2. Batch `CharOperation`s into fewer buffer edits

Current path:

- `apply_char_operations` loops over every `CharOperation`.
- Each insert/delete calls `agent_edit_buffer` separately.
- `agent_edit_buffer` updates the buffer and action log every time.

Potential improvement:

- Convert adjacent or nearby `CharOperation`s into a single set of buffer edits.
- Call `agent_edit_buffer` once per chunk/event instead of once per operation.
- Update the agent location once after the batch.

Why this is promising:

- It reduces foreground entity updates.
- It reduces action-log notifications.
- It reduces downstream editor invalidation pressure.
- It should be a relatively contained change.

Risks / things to verify:

- Anchors must still be computed from the original snapshot correctly.
- Multiple operations in one batch need to preserve ordering and offset semantics.
- Tests should cover adjacent inserts/deletes and replacements.

### 3. Yield or chunk during large edit processing

Current path:

- `process_edit` and `finalize_edit` iterate over parser events synchronously.
- Large final payloads can process all remaining old/new text in one foreground poll.

Potential improvement:

- Make edit event processing async.
- After a time budget or operation count, yield back to the foreground executor.
- Continue processing in subsequent polls.

Possible budget:

- Yield after ~4–8 ms of work.
- Always yield after processing a large event or a large batch of char ops.

Why this is promising:

- It directly attacks the worst symptom: 200–700 ms foreground polls.
- Even if total work remains the same, the UI can render between chunks.

Risks / things to verify:

- Partial edit application must remain coherent if cancelled.
- Tool-call UI should not show confusing intermediate states.
- Buffer/action-log invariants must still hold if a later chunk fails.

### 4. Move expensive matching/diff computation off the foreground thread

Current likely expensive pieces:

- `StreamingFuzzyMatcher::resolve_location_fuzzy`
- `StreamingDiff::push_new` and `finish`

Potential improvement:

- Keep buffer mutations on the foreground thread.
- Move pure computation to background tasks using snapshots and owned strings.
- Return match ranges / char operations to the foreground thread for application.

Candidate split:

1. Foreground: capture `TextBufferSnapshot` / `BufferSnapshot` and input strings.
2. Background: fuzzy match or compute char operations.
3. Foreground: apply batched buffer edits.

Why this is promising:

- Matching and diffing are CPU-heavy and mostly pure.
- It reduces main-thread blocking rather than just slicing it.

Risks / things to verify:

- Snapshots may become stale if the buffer changes during computation.
- Need a clear strategy for retrying or rejecting stale edits.
- Streaming UX might become less immediate for very small edits unless fast paths remain synchronous.

### 5. Add fast paths for exact or line-hinted matching

`StreamingFuzzyMatcher::resolve_location_fuzzy` currently scans every buffer line for query lines. For large files and repeated partials, this is expensive.

Potential fast paths:

- Try exact substring match for `old_text` before fuzzy matching.
- If the model/user provided a line hint, search a bounded window first.
- For multi-line `old_text`, use first/last line anchors to narrow candidates.
- Cache candidate ranges across partial chunks instead of rescanning the entire buffer.
- Avoid `strsim::normalized_levenshtein` for obviously dissimilar lines before doing full fuzzy matching.

Why this is promising:

- Most edit tool inputs should match exact text from a recent `read_file` call.
- Fuzzy matching should be the fallback, not the default cost paid for every chunk.

Risks / things to verify:

- Must preserve robustness when whitespace or formatting differs.
- Ambiguous exact matches still need careful handling.

### 6. Defer or coalesce expensive editor follow-up work during agent edits

The profile shows bracket colorization as the largest aggregate foreground cost, including many 50–83 ms spans. These appear after edit-file activity in the worst window.

Potential improvement:

- Coalesce bracket colorization invalidations during a streaming agent edit.
- Delay bracket colorization until the edit session completes or pauses.
- Limit bracket colorization work per frame.
- Avoid clearing/reapplying highlights repeatedly while the same edit session is actively mutating the buffer.

Why this matters:

- Even after `edit_file_tool` itself is improved, bracket colorization can still drop frames.
- It may amplify the cost of applying many small edit operations.

Risks / things to verify:

- Highlight state may briefly lag behind edits.
- Need to preserve correctness when the user interacts during an active edit.

### 7. Reduce action-log churn from streaming edits

The action log is not the largest single span in the worst 2 second window, but it is a meaningful aggregate cost across the full profile.

Potential improvement:

- Batch action-log updates for agent edit batches.
- Avoid recomputing diffs after every tiny streamed operation.
- Mark an edit session as active and update tracked diff after a debounce or completion.

Why this is promising:

- It complements batching `CharOperation`s.
- It may reduce `BufferDiff` and ACP diff update work as well.

Risks / things to verify:

- The action log must still distinguish user edits from agent edits correctly.
- If the user edits during an active agent edit, attribution must remain correct.

## Suggested implementation order

1. **Optimize `StreamingDiff::push_new` or move it off the foreground thread.** The split no-dedup xctrace profile shows it dominating `medium_insertions` (~84.75% inclusive, ~81.21% leaf).
2. **Add cooperative yielding/chunking around new-text processing** so `process_new_text_chunk` / `StreamingDiff::push_new` cannot monopolize a foreground poll for tens or hundreds of milliseconds.
3. **Add exact/line-window matching fast paths** before full fuzzy matching. The no-dedup xctrace profile directly shows `StreamingFuzzyMatcher` / `strsim::normalized_levenshtein` as a visible secondary cost, and most model edits should match exact text from a recent `read_file` result.
4. **Keep the split `process_event` helpers or equivalent miniprof spans** while iterating so future profiles keep attributing time to old-text matching vs new-text diffing vs apply.
5. **Batch `CharOperation` application** only if real miniprof captures or fixtures with many emitted operations show buffer/action-log work growing; it is not dominant in the current `medium_insertions` bench.
6. **Coalesce bracket colorization and action-log follow-up work** if real miniprof captures still show repeated 50+ ms foreground spans after the edit-session CPU work is chunked.

## Validation strategy

Use a short, focused foreground-only profile:

1. Start profiler.
2. Trigger one or two representative `edit_file` tool edits.
3. Wait for visible slowdown or edit completion.
4. Stop profiler before exporting.
5. Compare:
   - max `edit_file_tool` span
   - count of frame-risk intervals over 16.67 ms
   - aggregate bracket colorization time
   - aggregate action-log time

Success target:

- No single `edit_file_tool` foreground poll above 16.67 ms for typical edits.
- Large edits may still exceed one frame in total work, but should be chunked so no single foreground poll is hundreds of milliseconds.
- Follow-up bracket colorization/action-log work should not produce repeated 50+ ms foreground spans.
