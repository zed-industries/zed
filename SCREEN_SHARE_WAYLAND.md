# Wayland Screen Sharing — Revised Implementation Plan

## Goal

Enable screen sharing on Wayland/Linux by adding a Wayland-specific publish path that uses libwebrtc’s portal-backed desktop capture, while leaving the existing `scap` and macOS capture paths unchanged.

The key constraint is that Wayland does not expose application-controlled screen enumeration. The system XDG Desktop Portal owns the picker UI and provides frames over PipeWire. The implementation should reflect that model instead of trying to preserve the current X11-style source-selection flow.

## Non-goals

This change does not attempt to:

- replace `scap` on X11 or Windows
- replace ScreenCaptureKit on macOS
- add an application-controlled screen/window picker on Wayland
- change the `gpui` platform capture traits for all platforms
- unify all capture paths behind a single abstraction
- solve system-audio sharing on Wayland

## Codebase constraints confirmed in this repo

The current codebase has three properties that drive this design:

- `crates/gpui_linux/src/linux/wayland/client.rs` reports screen capture as unsupported on Wayland.
- The main screen-share UI in `crates/title_bar/src/collab.rs` is gated on `cx.is_screen_capture_supported()`, so Wayland currently hides the control.
- The live shared-screen state stored in `Room` only relies on `ScreenCaptureStream::metadata().id` for `shared_screen_id()`. It does not depend on full source metadata after publishing.

Those constraints mean the safest design is:

- keep the existing `gpui` capture path unchanged for X11/macOS
- add a parallel Wayland publish path in `livekit_client`
- keep storing a boxed `dyn ScreenCaptureStream` in `Room`

## Phase 0: Validate the external API before changing behavior

Before writing the production code, verify the actual `libwebrtc` and `livekit` API exposed by the vendored dependency revision already pinned in `Cargo.lock`.

This validation step should confirm:

- the exact `DesktopCapturer` construction and start-capture API
- the correct source type for Wayland portal capture, which is expected to be a generic desktop-capture mode rather than app-enumerated screens
- the callback threading model, especially whether callback invocation is serialized
- the available frame-conversion helpers for ARGB to I420
- whether `NativeVideoSource` accepts frames whose resolution changes after initialization
- how portal cancellation or permission denial is surfaced:
  - explicit permanent error
  - repeated temporary errors
  - no frames arriving
- whether a direct `tokio` dependency is needed for any code in `livekit_client`

Exit criteria for this phase:

- the implementation team has a minimal compileable prototype or a short note documenting any API differences from this plan
- the final code uses the verified external signatures rather than the placeholders in this document

## Design summary

### High-level design

Add a Wayland-only screen-share path inside `livekit_client` that:

- constructs a portal-backed desktop capturer
- pumps capture frames on a scheduled interval
- converts captured ARGB frames to WebRTC I420
- initializes a `NativeVideoSource` when the first valid frame arrives
- publishes a `LocalVideoTrack` with the same LiveKit publish options used by the current screen-share path
- returns a `WaylandScreenCaptureStream` that implements `ScreenCaptureStream`, so the rest of the call stack can keep storing a boxed stream handle without changing the `Room` data model

### Important design choices

#### 1. Do not add a Wayland source-list API

Do not add `wayland_screen_capture_sources()` or a parallel source-enumeration API.

Reason:

- the product UI will not use it
- Wayland source enumeration is not app-controlled
- a fake or dummy source list would make the abstraction more confusing, not less

If future debugging or experiments need raw source enumeration from the dependency, that can be added later as an internal helper, not as part of the product-facing design.

#### 2. Do not expose `CaptureSource` above `livekit_client`

The `call` crate and the UI should not take a `libwebrtc::desktop_capturer::CaptureSource` parameter for the Wayland path.

Instead, use:

- `LocalParticipant::publish_screenshare_track_wayland(&self, cx: &mut AsyncApp)`
- `Room::share_screen_wayland(&mut self, cx: &mut Context<Self>)`

Reason:

- product UI always relies on the system portal picker on Wayland
- this avoids adding a new `libwebrtc` dependency to `crates/call/Cargo.toml`
- this keeps the portal-specific details localized to `livekit_client`

#### 3. Use a synthetic screen-share ID on Wayland

`WaylandScreenCaptureStream::metadata().id` should return a synthetic, stable-per-share `u64`, not a real platform screen ID.

Reason:

- Wayland does not provide an app-controlled source identity comparable to the X11 path
- `shared_screen_id()` only needs a stable token for the active share
- the Wayland UI will not compare active shares to a source list

Use a `static AtomicU64` counter in `livekit_client` to generate nonzero synthetic IDs. The ID must be nonzero because `shared_screen_id()` returns `Option<u64>` and a zero value could be confused with "no active share" by future callers.

#### 4. Keep Wayland UX intentionally different

On Wayland, the title bar should show a plain screen-share toggle button, not the existing split-button with a dropdown list.

Reason:

- there is no meaningful app-level list to show
- the XDG Desktop Portal already provides the system picker
- trying to mimic the X11 picker on Wayland would be misleading

## Runtime dependencies

Wayland screen sharing via XDG Desktop Portal routes frames through PipeWire. The following are hard runtime requirements:

- `xdg-desktop-portal` (the D-Bus service that brokers the portal request)
- a portal backend for the compositor, such as `xdg-desktop-portal-gnome`, `xdg-desktop-portal-kde`, or `xdg-desktop-portal-wlr`
- `pipewire` (the multimedia framework that delivers frames)

If any of these are missing, the portal capture path will fail. The implementation should detect this as early as possible and surface a clear, actionable error message to the user rather than falling through to a generic timeout. See the "Error messages" subsection below.

## Runtime and timeout model

### Use GPUI for timeout handling

Do not add `tokio` just for timeouts.

`AsyncApp` already provides access to `cx.background_executor()`, and GPUI already supports:

- `cx.background_executor().timer(duration)`
- `FutureExt::with_timeout(duration, executor)`

Use GPUI executor timers for:

- the first-frame timeout
- any other setup-time timeout in this path
- simple delay-based coordination if no Tokio-specific scheduling behavior is required

### Use Tokio only where the dependency already requires it

Keep Tokio usage limited to places that already need it for the LiveKit SDK or related APIs, such as:

- publishing tracks
- unpublishing tracks
- any API that already runs through `gpui_tokio::Tokio::spawn(...)`

If the validated `DesktopCapturer` integration proves that the capture loop itself benefits from or requires Tokio context, that can be justified during implementation. But timeout handling alone is not a reason to add Tokio.

## Capture lifecycle

The Wayland capture flow should work like this:

1. The user clicks the screen-share action on Wayland.
2. `Room::share_screen_wayland()` marks the screen track as pending, exactly like the existing `share_screen()` flow.
3. `LocalParticipant::publish_screenshare_track_wayland()` starts a portal-backed desktop capturer.
4. A background task pumps `capture_frame()` on a fixed interval.
5. The capturer callback ignores temporary pre-selection conditions, converts successful frames, and waits for the first valid frame.
6. On the first valid frame:
   - create or initialize the `NativeVideoSource`
   - create the first WebRTC frame buffer
   - send a one-time “ready” signal back to the publishing task
7. The publishing task waits for that first-frame signal with a timeout using the GPUI executor.
8. Once the video source is ready, publish the LiveKit track using the same publish options as the current screen-share path.
9. On success, return the publication and the `WaylandScreenCaptureStream`.
10. On unshare or drop, the handle stops the capture loop.

### First-frame timeout

The initial publish must not wait forever.

Use a bounded timeout while waiting for the first successful frame. A timeout around 10 to 15 seconds is reasonable for the initial implementation.

If the timeout expires:

- stop the capture loop
- return a descriptive error (see "Error messages" below)
- restore `Room` state back to no active screen share

This timeout covers cases such as:

- the portal dialog being dismissed
- permissions being denied without a clean explicit error
- no frames ever arriving because the portal session never became active

### Error messages

Setup-time failures must reach the user through the existing error-prompt path with an informative message. The following cases should produce distinct, actionable messages:

- **Portal unavailable** (e.g., `xdg-desktop-portal` not running): explain that the XDG Desktop Portal service is required and name the likely missing package.
- **PipeWire unavailable**: explain that PipeWire is required for Wayland screen sharing.
- **Portal picker dismissed or permission denied**: explain that the user canceled or denied screen sharing, and that they can try again.
- **First-frame timeout expired**: explain that no frames were received from the portal within the timeout period, suggest checking that `xdg-desktop-portal` and PipeWire are running, and that the portal backend matches the compositor.

Avoid generic messages like "screen sharing failed." The user should be able to act on the error without searching online.

### Frame cadence

Use a scheduled interval for the frame pump rather than a manual busy loop.

Initial cadence should be conservative, for example around 30 fps. If the current desktop-capturer implementation clearly expects a different cadence, follow the dependency’s guidance instead.

This is a better starting point than assuming 60 fps:

- it reduces CPU overhead
- it is adequate for typical screen sharing
- it is easier to tune later than an overly aggressive default

### Resolution changes

Resolution changes must be treated as an explicit acceptance criterion, not an afterthought.

The implementation should verify whether `NativeVideoSource` supports variable-size frames after initialization.

If it does:

- recreate the reusable I420 buffer when width or height changes
- continue publishing

If it does not:

- do not silently keep sending mismatched frames
- treat size changes as a terminal condition for the first patch, or restart the capture/publish path in a controlled way
- document the chosen behavior in the PR

Manual testing for resizing and monitor changes is required.

### Runtime terminal failures after publish

Once the track is published, terminal capture failures should not be silently ignored.

Minimum acceptable behavior:

- log a descriptive error
- stop the capture loop

Preferred behavior, if the plumbing is straightforward:

- have the capture path expose a one-shot terminal-failure signal
- have `Room::share_screen_wayland()` spawn a small observer task that, if the same share is still active, resets the screen track state and unpublishes the track

Post-publish runtime failures should at least clean up local state, even if the first patch only logs the reason.

## File-by-file plan

### 1. `crates/livekit_client/src/livekit_client/playback.rs`

Add the Wayland-specific capture helper here, near the existing local video capture logic.

This helper should:

- build desktop-capturer options appropriate for Wayland portal capture
- start the capturer
- run the frame-pump interval
- convert ARGB frames to I420
- initialize the `NativeVideoSource` on the first valid frame
- handle resolution changes
- enforce the first-frame timeout with GPUI executor timers
- return the local video track plus the new `WaylandScreenCaptureStream`
- optionally expose a one-shot runtime-failure signal for the caller to observe

Keeping this in `playback.rs` is preferable to putting all capture logic in `livekit_client.rs`, because capture setup and frame conversion already live here today.

### 2. `crates/livekit_client/src/livekit_client.rs`

Add the public `WaylandScreenCaptureStream` type and the `LocalParticipant::publish_screenshare_track_wayland()` method.

`WaylandScreenCaptureStream` should:

- implement `ScreenCaptureStream`
- hold the synthetic share ID (from the `static AtomicU64` counter)
- hold the stop flag
- keep the background capture task alive for the lifetime of the share
- stop capture in `Drop`

Its `metadata()` implementation should return:

- synthetic `id`
- `label: None`
- `is_main: None`
- a placeholder resolution

Use a small placeholder resolution such as `1x1` unless later code is updated to surface real runtime resolution. The current caller only reads the ID.

`publish_screenshare_track_wayland()` should:

- call the new playback helper
- publish the returned track with the existing screen-share `TrackPublishOptions`
- return the publication and handle

### 3. `crates/livekit_client/Cargo.toml`

Do not add `tokio` solely for timeout handling.

Only add `tokio.workspace = true` if the validated implementation needs direct Tokio APIs for the capture loop or other dependency-facing runtime requirements. If GPUI executors cover the scheduling needs, no dependency change is required.

### 4. `crates/livekit_client/src/mock_client/participant.rs`

Add a matching mock `publish_screenshare_track_wayland()` method.

The mock implementation only needs to:

- publish a fake local video track through the test server
- return a stub `WaylandScreenCaptureStream`

This should be the bare minimum needed to smoke-test `Room::share_screen_wayland()` through the existing test infrastructure. No mock source-list function is needed.

### 5. `crates/call/src/call_impl/room.rs`

Add `Room::share_screen_wayland()` with the same pending/publish/cancel state machine used by `share_screen()`.

It should:

- reject offline rooms
- reject duplicate share attempts
- mark `screen_track` as pending with a new `publish_id`
- call `participant.publish_screenshare_track_wayland(cx)`
- if the request was canceled while publishing, unpublish the new track and log any error
- on success, box the returned handle as `Box<dyn ScreenCaptureStream>` and store it in `LocalTrack::Published`
- on failure, reset the room state to `LocalTrack::None`

If the Wayland handle exposes a terminal-failure receiver, this is the place to spawn the observer task before boxing the handle.

Follow the existing `share_screen()` conventions: use `.detach()` for the canceled-unpublish task, matching what the X11/macOS path already does.

### 6. `crates/title_bar/src/collab.rs`

Add a small helper used by the call controls, such as a local `use_portal_screen_share()` predicate. `gpui::guess_compositor()` returns `&'static str` (`"Wayland"`, `"X11"`, or `"Headless"`), so the predicate would look like:

```rust
fn use_portal_screen_share() -> bool {
    gpui::guess_compositor() == "Wayland"
}
```

Then update the render logic so that the screen-share control is shown when either:

- the legacy screen-capture path is supported, or
- the app is running on Wayland and should use the portal path

Behavior by compositor:

- on Wayland: render a plain toggle button that calls `share_screen_wayland()` or `unshare_screen()`
- on non-Wayland platforms: keep the existing split-button and source-list dropdown unchanged

Do not reuse the existing `pick_default_screen()` helper on Wayland.

### 7. `crates/collab_ui/src/collab_panel.rs`

Update the `ScreenShare` action handler to branch the same way as the title bar:

- on Wayland, call `share_screen_wayland()` directly
- on other platforms, keep the current source-enumeration path

Note: the existing handler does not gate on `is_screen_capture_supported()` — it goes straight to `cx.screen_capture_sources()`. The Wayland branch must be inserted *before* that call, not alongside it, otherwise the handler will attempt source enumeration and fail silently on Wayland.

This keeps keyboard and command-palette behavior aligned with the title bar.

### 8. `script/linux`

Treat distro package updates as a validation-driven follow-up, not a mandatory part of the initial patch.

After the feature compiles on at least one supported Wayland distro, update `script/linux` only for packages that are confirmed to be required and missing from the install flow.

## What does not change

The revised plan intentionally leaves these areas alone:

- `gpui` `ScreenCaptureSource` and `ScreenCaptureStream` traits
- `gpui` `Platform::screen_capture_sources()` and `Platform::is_screen_capture_supported()`
- existing X11 and Windows `scap` capture
- macOS screen capture
- `Room::share_screen()` for the legacy source-selection flow
- `pick_default_screen()` for non-Wayland platforms
- any product-facing source list on Wayland

## Testing plan

Wayland screen sharing is inherently difficult to test in automated CI because it requires a real portal session, a running PipeWire daemon, and a Wayland compositor. Manual testing on a real Wayland session is the primary validation method.

The initial implementation should include the bare minimum of automated tests needed to exercise the new code paths through the mock client. Additional coverage can be added progressively as the feature matures.

### Automated tests (bare minimum)

Using the existing mock client infrastructure:

- successful `share_screen_wayland()` publish through the mock client, confirming the room transitions to the sharing state and `shared_screen_id()` returns a nonzero synthetic ID
- unshare stops the active Wayland stream handle and resets room state

These two tests cover the basic happy path and cleanup. Further tests (cancellation while pending, timeout behavior, UI branching) can be added incrementally once the core path is stable.

For tests that need timeouts or scheduler-driven waiting, prefer GPUI executor timers over unrelated timer implementations.

### Manual validation

Manual testing is required on a real Wayland compositor and is the primary acceptance gate.

Primary validation:

- start sharing from the title bar on Wayland
- start sharing from the `ScreenShare` action (keyboard or command palette)
- accept the portal picker and confirm the remote participant receives video
- cancel the picker and confirm the UI returns to the non-sharing state with a useful, actionable error message
- deny permissions and confirm the UI returns to the non-sharing state with a useful, actionable error message
- confirm the error message is informative when `xdg-desktop-portal` or PipeWire is not running

Resolution and lifecycle validation:

- resize the shared window if window capture is used
- share a monitor, then change resolution or scaling if practical
- stop sharing and confirm the capture loop exits cleanly
- if possible, close the portal session externally and observe cleanup behavior

Regression validation:

- X11 screen sharing still works through the existing source-list path
- macOS behavior is unchanged

## Open questions to resolve during implementation

These questions should be answered in Phase 0 and in the implementation PR:

- Does the vendored `DesktopCapturer` callback run serially?
- What exact source type should be used for Wayland portal capture?
- How does the dependency surface portal cancellation and permission denial?
- Does `NativeVideoSource` support runtime resolution changes?
- Does the capture loop need Tokio context for dependency-facing reasons, or can GPUI executors own the full loop?
- Is there an existing room-level notification path suitable for post-publish capture failure, or should the initial patch log and auto-clean up only?

## Acceptance criteria

This change is complete when all of the following are true:

- the screen-share button appears on Wayland
- starting a share opens the system portal picker
- accepting the picker publishes a visible screen-share track to remote participants
- canceling or denying the picker returns a clear error and leaves the room in a non-sharing state
- unsharing stops the capture loop cleanly
- X11/macOS screen-share behavior is unchanged
- the implementation does not add a fake Wayland source list or broaden platform traits unnecessarily