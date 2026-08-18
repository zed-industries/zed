# PR #62779 review feedback

## Review verdict

The overall architecture is strong: the bounded journal, independent collectors, completion-order events, cause-anchored active window, and telemetry caps are all clean choices. Separating the new incident telemetry from the legacy logging and miniprof paths also feels appropriately scoped.

I would request changes before merging, primarily because the timeout contract has a detection hole.

## Findings

### 1. Blocker: a qualifying hang can remain unreported indefinitely

`IntervalSealer` checks timeouts only while processing an event:

- `crates/gpui/src/profiler/journal.rs:451-477`
- `crates/gpui/src/profiler/hang.rs:64-77`

Consider a release build, where the threshold is 100 ms:

1. The application is idle.
2. A foreground task blocks for 300 ms without invalidating a window.
3. Because the interval was empty, its start slides to the task's start.
4. The task ends before the one-second timeout, so nothing seals.
5. Repeated `HangDetector::poll()` calls drain no events and cannot advance the sealer.

That incident remains buffered until another draw or non-summary event, potentially forever. If the next draw is ten seconds later, `active_ms` also absorbs almost ten seconds of idle time. If the process quits first, the hang is lost.

`SmallPolls` does not rescue this case because it is categorically prohibited from causing a timeout at `journal.rs:461-475`.

The randomized test always performs a final draw at `hang.rs:476-477`, masking this case.

Potential direction: give the sealer an explicit wall-clock advancement operation, such as `seal_expired(now)`, called after each drain. Cadence summaries and forced pre-draw summaries may also need distinct treatment.

### 2. Blocker/data correctness: small polls cross non-draw timeout boundaries

For actions, inputs, presents, and slow task polls, the writer pushes the explicit event before flushing accumulated small polls:

- `crates/gpui/src/profiler/journal.rs:298-334`

If that explicit event crosses the timeout, `IntervalSealer` seals immediately before seeing the summary:

```text
Action(end >= timeout)
SmallPolls(summary of work before Action)
```

The timeout snapshot therefore omits those polls, and the summary is attached wholesale to the following interval. That can both understate one incident and contaminate a later incident's counts and occupancy.

The draw path already has the correct ordering—flush, then draw—at `journal.rs:323-327`. Non-draw explicit events should preserve the same boundary attribution.

### 3. Important: the implementation cannot reliably identify the first frame

The detector is constructed only after a 200 ms sleep:

- `crates/zed/src/reliability/hang_detection.rs:103-108`

A new detector intentionally starts its collector at the current journal tail:

- `crates/gpui/src/profiler/hang.rs:43-52`
- `crates/gpui/src/profiler/journal.rs:381-386`

Therefore, if the actual first frame occurred during those 200 ms, it is permanently missed. `first_frame_at` becomes the first later draw observed by the detector—or stays `None`, classifying every incident as startup.

Creating the detector before the grace sleep would retain the phase boundary. If the grace period is still desirable, filtering early incidents is safer than discarding the underlying events.

### 4. Important: presentation and some frame-callback work are blind spots

`Window::present` performs the potentially blocking platform renderer call before recording a timestamp-only `Present`:

- `crates/gpui/src/window.rs:2959-2966`
- `crates/gpui/src/profiler/journal.rs:84-96`

The resulting `Present` event always has zero duration, so it can never be a contributor. On macOS, frame requests invoke the callback directly rather than through a foreground task poll, so an enclosing task timing is not guaranteed.

The same request-frame callback executes next-frame callbacks before `Window::draw` at `crates/gpui/src/window.rs:1590-1619`; blocking there is also outside the draw timing.

Consequences:

- A renderer/GPU submission stall can be missed entirely.
- `dirty_to_draw_ms` is accurately named, but its documentation overstates it as time "before reaching the screen" in `hang.rs:103-105`.
- A first draw is not necessarily a first paint/presentation.

If comprehensive foreground-hang coverage is the contract, consider bracketing the platform request-frame callback or at least timing `platform_window.draw`.

### 5. Important: `busy_fraction` is not reliable for the active window

`occupancy_within` clamps individual events to the requested window, but then adds all folded small-poll time wholesale:

- `crates/gpui/src/profiler/journal.rs:195-224`

For an incident whose active window starts well after the snapshot began, small polls from before the cause are still counted. This can turn a low-occupancy interval into `busy_fraction = 1.0`, undermining the intended "work versus scheduling delay" distinction.

The top-level journal documentation also calls small-poll occupancy "exact," while this loses both boundary placement and possible nesting information. Retaining each `SmallPollFlush` span in the snapshot—or explicitly apportioning it by overlap—would produce a more defensible approximation.

### 6. Important: on-quit flush does not drain the detector

The quit handler only sends incidents already transferred into `telemetry::Reporter`:

- `crates/zed/src/reliability/hang_detection.rs:90-94`

The detector is owned by the monitor thread and polled once per second:

- `crates/zed/src/reliability/hang_detection.rs:106-126`

A draw-sealed incident recorded after the final poll is therefore absent from the on-quit telemetry send. This conflicts with the claim that the quit flush covers short sessions in `telemetry.rs:10-12`.

## Phase recommendation

One field should not mean both "before anything was visible" and "application startup." They are useful, orthogonal signals:

- `visibility_phase`: `pre_first_present` / `visible`
- `app_phase`: `startup` / `steady`, ending at an explicit "initial workspace is interactive" milestone

If only one dimension fits this PR, keep the visibility distinction because it directly affects whether a freeze was user-visible, but anchor it to the first present and name it accordingly. If the field remains named `startup`, reviewers and dashboard consumers will reasonably assume it covers the whole initialization episode.

## Positive notes

- Cause-anchored `active_window` is much more useful than reporting the entire frame interval.
- Separating `sealed_by` from contributors avoids conflating boundary with cause.
- Per-window action stacks are better than relying on the legacy global running-action slot.
- Keeping total incident counts while retaining only the worst payloads is a sensible telemetry bound.
- Idle sliding itself looks right; the problem is advancement after pending work, not the sliding rule.

## Suggested deterministic tests

Add direct `IntervalSealer` tests for at least:

- A sub-timeout qualifying hang followed by no more events.
- A pending hang followed only by cadence `SmallPolls` summaries.
- Small-poll attribution when a non-draw event crosses the timeout.
- Idle sliding before the first event.
- A draw preceded by a forced small-poll flush.
- Phase classification when the first frame occurs during the 200 ms grace period.

The randomized integration test is useful for production hook wiring, but its mandatory final draw prevents it from exercising the no-further-event timeout case.

## Validation performed

Passed:

- `cargo test -p gpui --features profiler detects_randomly_placed_foreground_hangs -- --nocapture`
- `cargo test -p gpui --features profiler profiler:: -- --nocapture` — 12 tests
- `git diff --check origin/main...HEAD`

## Walkthrough architecture questions

### Do no-draw intervals need a timeout boundary?

`SealReason` currently supports `Draw` and `Timeout`. Consider whether timeout sealing can be removed in favor of a more semantic boundary, given that most foreground UI work should eventually produce a draw.

Points to cover:

- Foreground work that intentionally produces no draw, including headless apps, hidden or occluded windows, non-invalidating actions, and internal task processing.
- The distinction between closing an interval after completed work and detecting a hang while that work is still running.
- Whether no-draw incidents should seal at a qualifying contributor's completion, at executor quiescence, after an inactivity deadline, or remain timeout-based.
- Whether `FrameSnapshot` is the right name for intervals that can exist without a frame.
- What additional begin/end instrumentation would be required to report a foreground hang before the blocked work yields.

### Can a completed foreground turn be the no-draw boundary?

Investigate treating the end of an outermost foreground turn as a semantic seal when that turn caused no invalidation and no frame was already pending.

Potential model:

- Bracket top-level executor polls, platform input dispatches, and frame callbacks as foreground turns.
- Track nesting so action handlers and other work inside a turn do not seal independently.
- Maintain an app-wide invalidation generation and pending-frame state shared by all window invalidators.
- Capture the generation when the outermost turn begins and inspect it after effects have flushed and the turn ends.
- If the turn caused no invalidation and no frame is pending, seal exactly at the turn's end with a reason such as `Quiescent` or `NoDraw`.
- If it dirtied a window, keep the interval open until the corresponding presentation boundary.
- Retain a deadline only as a backstop for dirty windows that never receive a frame callback, such as hidden or occluded windows.

Questions and edge cases:

- Initial windows begin dirty, so pending-frame state must include window creation rather than only invalidation generation changes.
- A task or input can contain nested actions; only the outermost turn should decide the boundary.
- Multiple windows can be dirty simultaneously, so a boolean may be insufficient if a draw should clear only one window's pending state.
- Task profiler hooks do not receive `&mut App`, so invalidation state would need a shared foreground-thread tracker or additional executor integration.
- Decide whether every no-draw turn should produce an interval or whether only hang detection should consume these boundaries, to avoid excessive snapshots for ordinary short work.

Preferred direction from the walkthrough: use the more semantic dispatcher-level boundary. Seal when the foreground returns to the platform run loop, has no immediately ready work left in the current dispatch batch, and has no frame pending. Do not seal after each individual nested action, input sub-operation, or task poll. This groups related polls into one activity episode and avoids producing large numbers of tiny snapshots. Implementing it will require integration with platform/executor queue-draining points in addition to the existing per-event profiler hooks.

### Prefer presentation over draw as the visual seal

Use the completion of `Window::present` as the normal visual interval boundary rather than the completion of `Window::draw`.

Reasons:

- A synchronous draw can be nested inside an input dispatch or foreground task and does not necessarily submit anything to the platform.
- Presentation is the point at which the rendered scene is handed to the platform, so dirty-to-present latency better approximates how long the user waited for an updated frame.
- Keeping the interval open through presentation includes stalls in `platform_window.draw`, which the current timestamp-only `PresentTiming` does not measure.
- Frame-delay and dropped-frame analysis should compare invalidation and presentation times rather than treating CPU draw completion as visibility.

Required follow-up:

- Give `PresentTiming` a start and end so platform submission work can be measured.
- Associate a presentation with the draw and dirty timestamp it satisfies, including the relevant window ID.
- Decide how repeated presentations of an unchanged frame affect sealing; the existing profiler already suppresses these journal events through `drew_since_last_present`.
- Preserve a semantic quiescent/no-frame boundary for foreground activity that does not request presentation.
- Retain a frame deadline for dirty or drawn windows that never present because they are hidden, occluded, or blocked by the platform.

### Flush summarized polls at explicit timeline boundaries

Flush accumulated small-poll summaries before recording every individually retained event or semantic boundary that can contribute to or seal an interval, not only before draws. This should keep the ring ordered as prior summarized work followed by the explicit event and prevent summaries from crossing presentation, quiescence, action, input, or long-task boundaries.

The flush timestamp should reflect when the summarized polls could actually have occurred. Because recording happens at event completion, simply flushing with the explicit event's end time makes the summary's coverage span overlap that event. Consider closing the summary at the explicit event's start and then advancing the summary cursor past the explicit event's end so later summaries do not claim its span.

### Evaluate ring allocation and foreground lock contention

`VecDeque` is an appropriate ring shape and stops reallocating after it reaches capacity, but `VecDeque::new()` grows incrementally on early hot-path writes. Consider preallocating `MAX_JOURNAL_EVENTS` if the fixed ~4 MiB profiler allocation is acceptable, or using a fixed-capacity backing store if measurements justify the added implementation complexity.

Do not replace the mutex with an `RwLock`: collectors would hold shared locks while copying and block the foreground writer for at least as long, while concurrent collectors provide little value compared with writer latency. A lock-free overwrite ring with independent cursors is possible but significantly more complex because readers must not race an overwrite of a slot they are copying.

First measure the current uncontended push cost and worst-case collector hold time. The main risk is `collect_unseen` allocating and copying up to the retained ring while holding a spinlock; if that is material, prefer shortening the critical section or simplifying to a single-consumer swap/double-buffer/chunked design before implementing a fully lock-free multi-reader ring.

### Keep boundary metadata on the interval

Consider storing the sealing boundary and its metadata directly on the completed interval rather than inferring it from `events.last()`. Context events are capped and may be dropped, but the presentation, semantic-quiescence, or frame-deadline boundary is part of the interval's identity and should not be lost when `MAX_INTERVAL_EVENTS` is reached.

A presentation boundary should retain at least its window ID, submission start/end, and the dirty/draw information for the frame it satisfied. A semantic no-frame boundary should retain its exact end and reason. The ordinary context-event vector can then remain lossy without making fields such as dirty-to-present latency or active-window start disappear.

### Consider channels only with an explicit multi-reader broker

A per-event MPSC channel naturally has one receiver, while hang detection and a journal-backed profiler UI need independent read access and may attach at different times. Separate channels would clone every event and require subscriber/backpressure management; a broadcast channel would still need bounded lag semantics and retained history for late readers, effectively recreating a ring.

Potential hybrids to evaluate if polling or ring contention becomes a problem:

- Keep the ring as authoritative storage and use a channel only to notify consumers that new events or boundaries are available.
- Publish immutable, reference-counted event batches or completed intervals at presentation/quiescence boundaries through a broker, rather than sending every raw event.
- Have one aggregator consume a producer channel, retain bounded history, and fan out to subscribers with independent cursors and loss accounting.

Any channel path must be non-blocking for the foreground producer and explicitly count dropped messages. Until measurements show a problem, the append-only ring remains the simpler fit for multiple independent readers.

### Retain compact input metadata and correlate nested actions

Do not retain or clone the raw `PlatformInput` in each journal event. Although the enum is currently about 72 bytes, it is `Clone` rather than `Copy`, some variants contain heap-backed or sensitive data such as key text and file paths, and embedding it would enlarge every `ForegroundEvent`, reduce the ring's effective history, and create telemetry privacy/cardinality concerns.

Instead, map `&PlatformInput` at `begin_input` into a compact, `Copy`, non-sensitive descriptor containing only fields needed for diagnosis, such as input kind, window ID, mouse button or file-drop phase, and possibly modifiers. Assign each outer input dispatch a lightweight monotonically increasing `InputId`; journaled actions begun while that input is active can retain `parent_input_id: Option<InputId>`. This cheaply reconstructs nested input-to-action causality without copying raw payloads.

`end_input` can retain outcomes such as whether the dispatch invalidated a window, stopped propagation, or prevented default behavior. Element attribution needs a more explicit design: mouse dispatch may invoke several capture and bubble listeners, and the current listener vector does not retain stable element metadata. A raw `DispatchNodeId` is only meaningful within one dispatch tree. Define whether a “handler” means every invoked listener, the listener that stopped propagation, or the final listener before adding attribution. Stable action names plus input IDs are likely suitable for telemetry; ephemeral view/node IDs may be useful only in the local profiler UI.

### Instrument outer platform dispatch turns for complete coverage

Native window systems convert callbacks into `PlatformInput` and invoke GPUI's registered `on_input` callback, which then calls `Window::dispatch_event`; those paths reach the current input instrumentation. However, platform callbacks such as resize, move, activation, appearance, close, and request-frame are not necessarily represented as `PlatformInput`. Truly headless apps also have no platform-window input callback: executor-driven work is visible as task polls, while direct callbacks outside the executor remain blind spots.

This reinforces instrumenting outer foreground/platform dispatch turns. Such instrumentation can cover UI input, non-input native callbacks, frame requests, and headless dispatch integrations while also providing the preferred semantic quiescence boundary.

### Migrate legacy action logging and remove `ACTION_STATISTICS`

This is broader than the current telemetry-only cutover because legacy logging and miniprof were intentionally left on their old paths, but completing the migration would remove overhead and eliminate a known source of incorrect action timing rather than maintaining two competing representations.
