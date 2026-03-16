# Audio Metrics Plan

## Goal

Add a Slack-like call diagnostics surface to Zed that helps users understand whether call issues are caused by:

- network conditions
- local system conditions
- selected audio/video devices
- current media pipeline behavior

The first version should focus on metrics we can already source from LiveKit/libwebrtc plus device information Zed already exposes in Settings. The diagnostics UI should avoid inventing a second device-management flow and instead reuse the existing audio device settings and test-audio workflow.

## Non-goals

This plan does not include:

- a full implementation
- a complete system diagnostics framework for all of Zed
- true editor/collaboration RTT measurement for CRDT/data sync
- a commitment to Slack’s exact thresholds or wording

Where Slack reports polished aggregate values, Zed will initially prefer correctness and traceability over aggressive summarization.

## Summary of What We Can Report

### Network

These metrics are realistically available from the patched LiveKit/libwebrtc SDK:

- overall connection quality
- latency
- jitter
- packet loss
- bitrate
- selected network path health
- audio receive/playout quality indicators
- video/screenshare quality indicators

### Devices

These can be reported by combining existing Zed device state with media stats:

- selected microphone
- selected speakers/output device
- selected camera if applicable
- microphone activity level
- speaker/output test flow
- camera/video health
- input/output device availability

### System

This is only partially available from LiveKit/libwebrtc:

- media-specific encoder/decoder hints
- CPU-vs-bandwidth limitation hints for video

But not directly:

- CPU percentage
- system load percentage
- power source / battery state
- generic hardware acceleration status
- memory pressure

Those require separate OS/process instrumentation and should not block the audio metrics work.

## Product Shape

The target UX is a diagnostics panel similar in spirit to Slack’s:

- `Network`
- `System`
- `Devices`

With the first pass emphasizing:

- high-confidence network metrics
- reused device selection/test UI
- light system messaging only where backed by real data

A good first rollout would support:

- a compact call health summary in the call UI
- an expanded diagnostics panel for troubleshooting
- links/actions to device selection and audio testing

## Recommended Phases

## Phase 1: Plumbing and Network Metrics MVP

### Scope

Expose the missing LiveKit metrics in Zed and build a simple diagnostics panel with:

- overall network quality
- latency
- jitter
- packet loss
- speaker/playout delay or a temporary placeholder if derivation is not yet validated
- current input/output device names
- launch point to existing audio test UI

### Why first

This gives the most user value with the least speculative work.

## Phase 2: Device Diagnostics Integration

### Scope

Reuse the current device inventory and audio test flows inside the call diagnostics UI:

- selected microphone
- selected output device
- direct access to test-audio workflow
- microphone level bar
- output test guidance
- missing-device/fallback warnings

## Phase 3: Richer Media Diagnostics

### Scope

Add:

- bitrate
- video FPS
- frame drops/freezes
- encoder/decoder implementation hints
- CPU-vs-bandwidth limitation messaging
- camera/screenshare health rows

## Phase 4: Optional System Diagnostics

### Scope

Add OS/process metrics if desired:

- CPU %
- power source
- process load
- device power-saving hints
- hardware acceleration summary

This should be treated as separate follow-up work, not part of the initial LiveKit-based effort.

## Metrics Mapping

## Network Section

### Overall network quality

#### Source
Use LiveKit’s participant connection quality:

- `connection_quality()`
- room event for connection-quality changes

#### Notes
This is the best source for a user-facing header like:

- `Good`
- `Poor`
- `Lost`

It is already a summarized quality signal and should be used for the section header rather than re-deriving a top-level status from raw stats in the first pass.

### Latency

#### Source
Prefer one of:

- selected ICE candidate pair RTT
- RTP remote-inbound / remote-outbound RTT if track-focused reporting is better

#### Suggested interpretation
Treat this as the local client’s media-path RTT. In LiveKit’s SFU model, this is effectively client-to-SFU RTT rather than literal participant-to-participant RTT.

#### UI wording
It is acceptable to label this `Latency`, but internally we should document that it is a media-path RTT proxy.

### Jitter

#### Source
Inbound RTP stats, especially on the active remote audio track.

#### Notes
This should be one of the most trustworthy rows in the panel. It maps well to user-perceived instability in received audio.

### Packet loss

#### Source
Use inbound RTP loss and optionally remote-reported loss of our outbound stream.

#### Suggested aggregation
For the first pass, prefer a single local receive-side packet-loss percentage from the main audio stream, with optional later extension to show both send and receive.

### Speaker delay

#### Source candidates
Use a derived value from one or more of:

- audio playout delay
- jitter-buffer delay
- total processing delay
- estimated playout timing

#### Notes
This is the least turnkey Slack-like metric. We should treat it as an experimental derived metric until validated with real calls and test cases.

If we cannot confidently produce a stable number in Phase 1, we should:

- omit it temporarily, or
- label it conservatively, or
- ship it only in the expanded panel

rather than show misleading numbers.

### Bitrate

#### Source
Candidate-pair and RTP stream stats.

#### Use
Useful for advanced diagnostics and for explaining poor video/screenshare quality. This is not required for the first Slack-like network section, but should be collected if the cost is low.

## Devices Section

This section should explicitly reuse existing Zed audio device infrastructure rather than create a new source of truth.

### Existing device infrastructure to reuse

#### Audio settings model
Current audio settings already store:

- selected input device
- selected output device

These are the right canonical values for device display in diagnostics.

#### Device inventory
Current audio code already provides a global list of available audio devices.

This should remain the source for:

- device labels
- device presence checks
- fallback behavior if a selected device disappears

#### Shared dropdown rendering
The settings UI already has shared rendering for input/output audio device dropdowns.

This should be reused directly or factored into a component used by both:

- Settings
- call diagnostics panel

to avoid drift in behavior and labeling.

#### Existing audio test flow
Zed already has a standalone `Audio Test` window with:

- input device dropdown
- output device dropdown
- start/stop test playback button
- persisted setting updates

This should be treated as the existing troubleshooting tool, not replaced.

### Devices rows to include

#### Microphone
Show:

- selected input device name
- fallback to `System Default` when unset
- microphone activity meter
- warning if selected device is missing and a fallback is active

#### Speakers
Show:

- selected output device name
- fallback to `System Default` when unset
- button or link to launch audio test
- warning if selected device is missing and a fallback is active

#### Camera
If available in the call UI, show:

- selected camera
- muted/unmuted state
- active resolution/FPS when video is on

This part may require a separate device source outside the current audio settings and is not required for the first audio-focused plan.

### Reuse strategy

Do not build a separate `Devices` panel with its own bespoke input/output selectors.

Instead:

- use the same device inventory already shown in Settings
- use the same `System Default` semantics
- use the same persistence path to settings
- use the same `Audio Test` window for loopback testing
- add lightweight entry points from diagnostics into the existing test flow

### Recommended UX for device troubleshooting

The diagnostics panel should include one or both of:

- a button like `Test Audio`
- a button like `Open Audio Setup`

Both should route into existing device setup/test flows rather than opening a new diagnostics-specific device page.

## System Section

## What we can report now

From media stats, we can likely report hints such as:

- video is bandwidth-limited
- video is CPU-limited
- encoder implementation name
- decoder implementation name
- whether power-efficient encoder/decoder paths are active

## What we should not fake

Do not present media hints as if they were general machine health.

For example:

- `quality limitation: CPU` is not the same as `CPU use: 85%`
- `power efficient encoder: true` is not the same as a universal `hardware acceleration enabled`

## Recommendation

For the first version:

- either omit `System`, or
- show a minimal section with carefully worded media-specific hints

Example safe wording:

- `Video is currently limited by bandwidth`
- `Hardware video encoding appears active`
- `No media-side CPU limitation detected`

Avoid Slack-like rows such as `CPU use 5%` unless backed by real OS/process telemetry.

## Aggregation Strategy

The raw stats surface is broad and needs an opinionated aggregation layer.

## Principles

### Prefer audio-first health metrics
For a call diagnostics panel, user-perceived quality is usually dominated by audio. When picking a single number:

- prefer audio-track metrics over video
- prefer active/primary subscribed tracks over all tracks
- prefer receive-side values for what the user hears

### Prefer stable values over noisy spikes
The UI should not update directly from every raw sample. Aggregate using:

- periodic polling
- short rolling windows
- smoothing for display values
- instant updates only for coarse state changes like disconnected or quality changed

### Keep raw values available for debugging
Even if the panel shows a summarized label like `Normal`, keep the raw value in the underlying diagnostics model so it can later support:

- tooltips
- verbose diagnostics
- logs
- issue reports

## Proposed Data Model

Add an internal diagnostics model that sits above raw LiveKit stats and below the UI.

Example categories:

- `NetworkDiagnostics`
- `DeviceDiagnostics`
- `SystemDiagnostics`
- `CallDiagnosticsSnapshot`

Each snapshot should include:

- a timestamp
- coarse health states
- normalized numeric values ready for display
- optional raw backing values for debugging

This should be an app-side model, not a new SDK-facing API type unless it proves reusable.

## SDK and Wrapper Audit Results

A detailed audit of both the upstream `livekit-rust-sdks` and Zed's `livekit_client` wrapper reveals that
the upstream SDK already exposes nearly everything we need. The work splits into:

- one small fix in `livekit-rust-sdks` (re-export a type trapped in a private module)
- wrapper changes in Zed's `livekit_client` crate
- corresponding mock-client additions for test support

## What livekit-rust-sdks already exposes (no changes needed)

### libwebrtc crate

The entire stats surface is fully public and ready to use:

- `pub mod stats` with `pub enum RtcStats` (15 variants)
- all variant wrapper structs (`InboundRtpStats`, `OutboundRtpStats`, `CandidatePairStats`, etc.) are `pub` with `pub` fields
- all `dictionaries::*` structs are `pub` with `pub` fields
- all supporting enums (`QualityLimitationReason`, `IceCandidatePairState`, etc.) are `pub`
- `PeerConnection::get_stats()`, `RtpSender::get_stats()`, `RtpReceiver::get_stats()` are all `pub`

No changes needed in `libwebrtc`.

### livekit crate — already public

The following are already public and usable as-is:

| Item | Access path | Notes |
|---|---|---|
| `ConnectionQuality` enum | `livekit::prelude::ConnectionQuality` | `Excellent`, `Good`, `Poor`, `Lost` |
| `participant.audio_level()` | method on `Participant`, `LocalParticipant`, `RemoteParticipant` | returns `f32` |
| `participant.connection_quality()` | same | returns `ConnectionQuality` |
| `participant.is_speaking()` | same | returns `bool` |
| `RoomEvent::ActiveSpeakersChanged` | `livekit::RoomEvent` | `{ speakers: Vec<Participant> }` |
| `RoomEvent::ConnectionQualityChanged` | `livekit::RoomEvent` | `{ participant, quality }` |
| Per-track `get_stats()` | on `Track`, `LocalTrack`, `RemoteTrack`, all concrete track types | returns `RoomResult<Vec<RtcStats>>` |
| `RtcStats` | `livekit::webrtc::stats::RtcStats` | via `pub mod webrtc { pub use libwebrtc::*; }` |
| `Room::get_stats()` | method on `Room` | returns `RoomResult<SessionStats>` |
| `SessionStats` | `livekit::SessionStats` | re-exported from `lib.rs`; contains `publisher_stats: Vec<RtcStats>` and `subscriber_stats: Vec<RtcStats>` |

## What livekit-rust-sdks needs to change

**Nothing.** All required SDK-side work is already done:

- `SessionStats` is re-exported from `livekit/src/lib.rs` (`pub use rtc_engine::SessionStats`)
- `Room::get_stats()` already returns `RoomResult<SessionStats>` with proper error conversion
- `RoomError` already has `#[from] EngineError` so the `.map_err(Into::into)` at the call site works
- All participant accessors (`audio_level`, `connection_quality`, `is_speaking`) are public
- All room events (`ConnectionQualityChanged`, `ActiveSpeakersChanged`) are public
- All per-track `get_stats()` methods are public
- The full `libwebrtc::stats` surface is public

The remaining work is entirely on Zed's side: the `livekit_client` wrapper and mock client.

## What Zed's livekit_client wrapper needs to change

The wrapper currently exposes a minimal surface. A significant number of upstream APIs exist
but are not forwarded.

### Currently forwarded room events (21 of ~40 upstream variants)

- `ParticipantConnected`
- `ParticipantDisconnected`
- `LocalTrackPublished`
- `LocalTrackUnpublished`
- `LocalTrackSubscribed`
- `TrackSubscribed`
- `TrackUnsubscribed`
- `TrackSubscriptionFailed`
- `TrackPublished`
- `TrackUnpublished`
- `TrackMuted`
- `TrackUnmuted`
- `RoomMetadataChanged`
- `ParticipantMetadataChanged`
- `ParticipantNameChanged`
- `ParticipantAttributesChanged`
- `ActiveSpeakersChanged`
- `ConnectionStateChanged`
- `Connected`
- `Disconnected`
- `Reconnecting`
- `Reconnected`

### Currently dropped (diagnostics-relevant)

- `ConnectionQualityChanged` — **high priority**, per-participant quality updates

All other dropped events are not relevant to diagnostics.

### Wrapper additions needed

#### Types to add

| Type | Source |
|---|---|
| `ConnectionQuality` | re-export or mirror from `livekit::prelude::ConnectionQuality` |
| `SessionStats` | re-export or newtype from `livekit::SessionStats` (after SDK fix) |
| `RtcStats` | re-export from `livekit::webrtc::stats::RtcStats` |

#### Room methods to add

| Method | Upstream source |
|---|---|
| `get_stats()` | `livekit::Room::get_stats()` |

#### Participant methods to add

| Method | On which types | Upstream source |
|---|---|---|
| `audio_level()` | `LocalParticipant`, `RemoteParticipant`, `Participant` | `livekit::participant::*.audio_level()` |
| `connection_quality()` | `LocalParticipant`, `RemoteParticipant`, `Participant` | `livekit::participant::*.connection_quality()` |
| `is_speaking()` | `LocalParticipant`, `RemoteParticipant`, `Participant` | `livekit::participant::*.is_speaking()` |

#### Room events to forward

| Event | Payload |
|---|---|
| `ConnectionQualityChanged` | `{ participant: Participant, quality: ConnectionQuality }` |

This requires adding a `ConnectionQualityChanged` variant to Zed's `RoomEvent` enum
in `livekit_client/src/lib.rs` and handling it in `room_event_from_livekit()`.

### Mock client additions needed

Every new wrapper API also needs a mock implementation in `mock_client.rs` / `test.rs`
for the `test-support` feature. Specifically:

| New API | Mock approach |
|---|---|
| `ConnectionQuality` type | Define matching enum or re-export |
| `Room::get_stats()` | Return a struct with dummy or configurable values |
| `LocalParticipant::audio_level()` | Return `0.0` or store configurable `f32` in `RoomState` |
| `LocalParticipant::connection_quality()` | Return `ConnectionQuality::Excellent` or configurable |
| `LocalParticipant::is_speaking()` | Return `false` or configurable |
| `RemoteParticipant::audio_level()` | Same pattern, per-participant in test state |
| `RemoteParticipant::connection_quality()` | Same pattern |
| `RemoteParticipant::is_speaking()` | Same pattern |
| `RoomEvent::ConnectionQualityChanged` | Add variant; test server needs method to simulate it |

## Polling Strategy

## Initial recommendation

Use periodic polling for raw stats plus event-driven updates for coarse state changes.

### Poll cadence
Start with a modest cadence such as once per second.

This is likely frequent enough for diagnostics without adding unnecessary overhead or causing a jittery UI.

### Event-driven updates
Use room events for:

- connected/disconnected
- reconnecting/reconnected
- active speakers changes
- connection quality changed

### Why both
The events give responsiveness, while polling provides the numeric metrics that events do not summarize.

## UI Integration Plan

## Entry points

### Compact entry point
Add a compact call-health control in the existing call UI, such as:

- a small network-quality icon
- a `Call Health` button
- a dropdown entry in the call controls menu

### Expanded panel
Open a panel/popover/modal containing:

- network section
- devices section
- optional system section
- troubleshooting actions

## Reuse of existing UI

### Devices controls
The diagnostics panel should not fork device-selection behavior.

Instead, either:

- embed the existing shared audio dropdown component directly, or
- provide a small summary row plus buttons that open the existing Settings or Audio Test window

### Audio test
The existing `Audio Test` window should remain the primary loopback tool. The diagnostics panel should launch it rather than duplicate its implementation.

## Suggested First-Pass UI

### Network
- Overall: `Good`
- Latency: `20 ms`
- Jitter: `10 ms`
- Packet loss: `0%`
- Speaker delay: hidden until validated or marked experimental

### Devices
- Microphone: selected input device
- Speakers: selected output device
- `Test Audio` button
- mic activity bar

### System
- omitted in MVP, or
- minimal media-limitation hints only

## File / Module Plan

## Likely existing code areas to touch

### LiveKit wrapper
Use the existing call/media wrapper layer to expose missing upstream metrics.

### Call crate
Add a diagnostics polling/aggregation layer near the active room or room state so it can observe:

- connection lifecycle
- participants
- tracks
- current device settings

### Audio crate
Reuse existing audio device inventory and selected device settings as the source of truth for the devices section.

### Settings UI
Reuse the existing:

- audio device dropdown rendering
- audio test window
- collaboration settings actions

## Likely new pieces

### Diagnostics model
A new internal model for summarized call diagnostics snapshots.

### Diagnostics controller/task
A periodic task that polls stats, updates the model, and triggers UI refreshes.

### Diagnostics panel UI
A new panel/popover/modal in the call UI.

## Validation Plan

## Functional validation

Test the panel in:

- healthy network conditions
- constrained bandwidth
- induced packet loss
- induced jitter
- reconnect scenarios
- device removal/fallback scenarios
- no microphone / no speakers available scenarios

## Device validation

Verify that diagnostics and Settings stay in sync when:

- changing input device from Settings
- changing output device from Settings
- changing devices in the Audio Test window
- unplugging the selected device
- returning to system default

## Metric validation

Cross-check displayed values against:

- raw logged stats
- controlled test scenarios
- user-reported symptoms
- external call tools if needed

The `speaker delay` row specifically should not ship broadly until we are confident its derivation matches user-perceived behavior.

## Risks

## Risk: misleading derived metrics
Some Slack-style values are polished aggregates. If we display raw or poorly derived values with overconfident labels, we risk misleading users.

### Mitigation
Prefer:
- fewer rows
- more accurate wording
- experimental status for less certain metrics

## Risk: duplicate device flows
A new diagnostics UI could accidentally drift from Settings.

### Mitigation
Reuse existing device settings, dropdown rendering, and the Audio Test window.

## Risk: overloading the UI with noisy data
Raw WebRTC stats are volatile.

### Mitigation
Add smoothing and summarize only high-value rows.

## Risk: system metrics creep
Trying to fully match Slack’s `System` section could turn this into a separate platform instrumentation project.

### Mitigation
Scope the initial work to network and devices.

## Open Questions

- What should the first entry point be in the call UI: button, icon, or menu item?
- Do we want a compact always-visible quality indicator in addition to the expanded panel?
- Should the first version include `speaker delay`, or wait until validated?
- Should packet loss show one number or separate send/receive values in an advanced view?
- Should the diagnostics panel embed device selectors directly, or only link to existing audio setup/test flows?
- Do we want to expose raw values in a verbose/debug mode for support and issue reports?

## Concrete MVP Recommendation

Ship an MVP with:

### Network
- overall connection quality
- latency
- jitter
- packet loss

### Devices
- selected microphone
- selected speakers
- microphone activity bar
- `Test Audio` action that opens the existing Audio Test window

### System
- omitted or limited to a short note when media stats clearly indicate bandwidth or CPU limitation

This gives Zed a strong Slack-like troubleshooting surface while staying grounded in metrics we can actually source and explain.

## Implementation Order

1. Expose connection quality, audio level, room stats, and track stats through the wrapper
2. Build a diagnostics snapshot model and polling task
3. Add a compact call-health entry point in the call UI
4. Build the expanded diagnostics panel with `Network` and `Devices`
5. Reuse current audio device settings and `Audio Test` flow inside the troubleshooting UX
6. Validate speaker-delay derivation before showing it by default
7. Add richer video/media/system hints only after the MVP is solid