# Plan: Reduce `edit_file_tool` foreground stalls

## Profile findings

Profile analyzed: `zed-profiles/performance_profile.miniprof.json`

- Size: 109.3 MB
- Shape: foreground-only `miniprof.json`
- Total timings: 973,128
- Timings >= 1 ms: 11,580
- Captured span: ~52.5 minutes

The largest `edit_file_tool` frame-risk interval is severe:

| Location | Duration | Approx. 60 Hz frame budgets |
| --- | ---: | ---: |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 711.56 ms | 43 |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 641.01 ms | 39 |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 221.99 ms | 14 |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 108.90 ms | 7 |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 94.46 ms | 6 |

In the worst 2 second window around `2850.3s..2852.3s`:

| Location | Total | Max | Hits |
| --- | ---: | ---: | ---: |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 1629.02 ms | 711.56 ms | 10 |
| `crates/editor/src/bracket_colorization.rs:98:42` | 151.66 ms | 51.14 ms | 10 |
| `crates/agent/src/agent.rs:1267:12` | 46.48 ms | 12.30 ms | 13 |
| `crates/agent/src/thread.rs:1918:23` | 14.47 ms | 4.14 ms | 13 |
| `crates/action_log/src/action_log.rs:190:40` | 4.37 ms | 1.54 ms | 3 |

Across the whole profile, the biggest aggregate foreground costs were:

| Location | Total | Max | Hits |
| --- | ---: | ---: | ---: |
| `crates/editor/src/bracket_colorization.rs:98:42` | 59.31 s | 83.91 ms | 4069 |
| `crates/agent/src/tools/edit_file_tool.rs:252:12` | 7.57 s | 711.56 ms | 1588 |
| `crates/agent/src/agent.rs:1267:12` | 3.44 s | 31.98 ms | 1739 |
| `crates/action_log/src/action_log.rs:190:40` | 3.06 s | 71.63 ms | 2059 |
| `crates/session/src/session.rs:76:16` | 2.62 s | 30.93 ms | 1216 |

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

1. **Instrument internal edit phases** to confirm where the 711 ms and 641 ms polls are spent.
2. **Batch `CharOperation` application** so each streamed/final edit event causes fewer buffer/action-log updates.
3. **Add cooperative yielding** around large parser-event and char-op processing to stop single-frame lockups quickly.
4. **Add exact/line-window matching fast paths** before full fuzzy matching.
5. **Move fuzzy matching / streaming diff computation to background tasks** if instrumentation shows CPU computation dominates.
6. **Coalesce bracket colorization and action-log follow-up work** if follow-up editor churn remains high after batching.

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
