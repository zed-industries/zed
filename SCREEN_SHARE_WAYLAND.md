# Wayland Screen Sharing — Implementation Plan

## Goal

Enable screen sharing on Wayland/Linux by adding a parallel code path that uses
libwebrtc's `DesktopCapturer` (which supports Wayland via PipeWire + XDG Desktop
Portal), while leaving the existing scap/macOS capture paths untouched.

## Background

Screen sharing currently works on macOS (native ScreenCaptureKit) and X11/Windows
(via `scap`). On Wayland the code explicitly returns an error:

```rust
// crates/gpui_linux/src/linux/wayland/client.rs, line ~710
"Wayland screen capture not yet implemented."
```

Wayland's security model forbids direct screen access. Capture must go through the
XDG Desktop Portal, which shows a system picker dialog and streams frames via
PipeWire. libwebrtc's `DesktopCapturer` already implements this pipeline.

## Prerequisite: DesktopCapturer availability

Zed's LiveKit fork (`zed-industries/livekit-rust-sdks`, rev `c1209aa...`) already
contains the full `desktop_capturer` module at `libwebrtc/src/desktop_capturer.rs`.
The `libwebrtc` crate is already a dependency of `livekit_client`. **No fork update
is needed.**

Available API:

- `DesktopCapturer::new(options) -> Option<Self>`
- `DesktopCapturer::start_capture(source, callback)`
- `DesktopCapturer::capture_frame()`
- `DesktopCapturer::get_source_list() -> Vec<CaptureSource>`
- `DesktopCapturerOptions::new(source_type)` / `.set_include_cursor(bool)`
- `DesktopCaptureSourceType::{Screen, Window, Generic}` (`Generic` on Linux/macOS)
- `CaptureSource` with `.id()`, `.title()`, `.display_id()`
- `DesktopFrame` with `.width()`, `.height()`, `.stride()`, `.data()` (ARGB)
- `CaptureError::{Temporary, Permanent}`
- Default feature `glib-main-loop` starts a GLib event loop for GDBus (portal
  communication)

## Why the Wayland path can't reuse the existing ScreenCaptureSource abstraction

The current architecture routes capture through gpui traits:

```
gpui Platform trait
  → screen_capture_sources() → Vec<Rc<dyn ScreenCaptureSource>>
  → ScreenCaptureSource::stream(callback) → Box<dyn ScreenCaptureStream>
  → callback(ScreenCaptureFrame(PlatformScreenCaptureFrame))

PlatformScreenCaptureFrame = scap::frame::Frame on Linux
playback.rs converts scap::frame::Frame → NV12Buffer for WebRTC
```

Three things prevent plugging DesktopCapturer into this path:

1. **Frame type mismatch** — `PlatformScreenCaptureFrame` is `scap::frame::Frame`
   on Linux. DesktopCapturer produces ARGB `DesktopFrame`s. We'd have to either
   change the type (breaks scap) or copy into a scap variant (wasteful).
2. **Source enumeration doesn't work on Wayland** — the XDG Desktop Portal does not
   expose available sources to the application. `get_source_list()` returns a single
   dummy entry with an empty title. The real picker is shown by the portal when
   capture starts.
3. **Resolution unknown upfront** — `ScreenCaptureSource::metadata()` expects
   resolution before capture starts, but on Wayland the resolution is only available
   in the first `DesktopFrame`.

The solution: bypass the gpui `ScreenCaptureSource` abstraction for Wayland and talk
to `DesktopCapturer` directly from `livekit_client`. The Wayland capture path does
its own ARGB → I420 conversion and publishes directly to a `NativeVideoSource`.

## Key simplification: `ScreenCaptureStreamHandle` implements `ScreenCaptureStream`

The existing `ScreenCaptureStream` trait is trivial:

```rust
// crates/gpui/src/platform.rs
pub trait ScreenCaptureStream {
    fn metadata(&self) -> Result<SourceMetadata>;
}
```

And the **only consumer** of `metadata()` on a live stream is
`Room::shared_screen_id()`, which only reads `meta.id`:

```rust
// crates/call/src/call_impl/room.rs
pub fn shared_screen_id(&self) -> Option<u64> {
    self.live_kit.as_ref().and_then(|lk| match lk.screen_track {
        LocalTrack::Published { ref _stream, .. } => {
            _stream.metadata().ok().map(|meta| meta.id)
        }
        _ => None,
    })
}
```

By implementing `ScreenCaptureStream` for our new `ScreenCaptureStreamHandle`, the
Wayland handle can be boxed as `Box<dyn ScreenCaptureStream>` and stored in the same
`LocalTrack<dyn ScreenCaptureStream>` field. This means:

- `LiveKitRoom` struct — **no change**
- `LocalTrack<dyn ScreenCaptureStream>` enum — **no change**
- `shared_screen_id()` — **no change**
- `is_sharing_screen()` — **no change**
- `unshare_screen()` — **no change**
- `stop_publishing()` — **no change**

---

## Changes by file

### 1. `crates/livekit_client/Cargo.toml`

Add `tokio` for the frame-loop timer (it's a workspace dep but not currently listed
here):

```toml
tokio.workspace = true
```

No other dependency changes. `libwebrtc` and `livekit` are already deps.

### 2. `crates/livekit_client/src/livekit_client.rs`

Add a new `ScreenCaptureStreamHandle` struct and implement `ScreenCaptureStream`:

```rust
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use gpui::{DevicePixels, ScreenCaptureStream, SourceMetadata, size};
use libwebrtc::desktop_capturer::{
    CaptureError, CaptureSource, DesktopCaptureSourceType,
    DesktopCapturer, DesktopCapturerOptions, DesktopFrame,
};
use libwebrtc::native::yuv_helper;
use livekit::options::{TrackPublishOptions, VideoCodec};
use livekit::track::TrackSource;
use livekit::webrtc::prelude::{I420Buffer, RtcVideoSource, VideoFrame, VideoResolution, VideoRotation};
use livekit::webrtc::video_source::native::NativeVideoSource;
use futures::{SinkExt, channel::mpsc};

pub struct ScreenCaptureStreamHandle {
    pub screen_id: u64,
    stop_capture: Arc<AtomicBool>,
    _task: gpui::Task<Result<(), gpui_tokio::JoinError>>,
}

impl Drop for ScreenCaptureStreamHandle {
    fn drop(&mut self) {
        self.stop_capture.store(true, Ordering::Release);
    }
}

impl ScreenCaptureStream for ScreenCaptureStreamHandle {
    fn metadata(&self) -> anyhow::Result<SourceMetadata> {
        Ok(SourceMetadata {
            id: self.screen_id,
            label: None,
            is_main: None,
            resolution: size(DevicePixels(0), DevicePixels(0)),
        })
    }
}
```

Add a helper to get Wayland-compatible capturer options:

```rust
fn wayland_capturer_options() -> DesktopCapturerOptions {
    let mut options = DesktopCapturerOptions::new(DesktopCaptureSourceType::Generic);
    options.set_include_cursor(true);
    options
}
```

Add a public function to list sources (returns a dummy entry on Wayland):

```rust
pub fn wayland_screen_capture_sources() -> Vec<CaptureSource> {
    let Some(capturer) = DesktopCapturer::new(wayland_capturer_options()) else {
        return Vec::new();
    };
    capturer.get_source_list()
}
```

Add the Wayland publish method on `LocalParticipant`:

```rust
impl LocalParticipant {
    pub async fn publish_screenshare_track_wayland(
        &self,
        source: Option<CaptureSource>,
        cx: &mut AsyncApp,
    ) -> Result<(LocalTrackPublication, ScreenCaptureStreamHandle)> {
        let stop_capture = Arc::new(AtomicBool::new(false));
        let (mut video_source_tx, mut video_source_rx) = mpsc::channel(0);

        let screen_id = source.as_ref().map(|s| s.id()).unwrap_or(0);

        let callback = {
            let mut stream_width: u32 = 1920;
            let mut stream_height: u32 = 1080;
            let mut video_frame = VideoFrame {
                rotation: VideoRotation::VideoRotation0,
                buffer: I420Buffer::new(stream_width, stream_height),
                timestamp_us: 0,
            };
            let mut video_source: Option<NativeVideoSource> = None;
            let stop_capture = stop_capture.clone();

            move |result: Result<DesktopFrame, CaptureError>| {
                let frame = match result {
                    Ok(frame) => frame,
                    Err(CaptureError::Temporary) => return,
                    Err(CaptureError::Permanent) => {
                        log::error!("Permanent error capturing screen");
                        stop_capture.store(true, Ordering::Release);
                        return;
                    }
                };

                let width = frame.width() as u32;
                let height = frame.height() as u32;

                if width != stream_width || height != stream_height {
                    stream_width = width;
                    stream_height = height;
                    video_frame.buffer = I420Buffer::new(width, height);
                }

                let (s_y, s_u, s_v) = video_frame.buffer.strides();
                let (y, u, v) = video_frame.buffer.data_mut();
                yuv_helper::argb_to_i420(
                    frame.data(), frame.stride(),
                    y, s_y, u, s_u, v, s_v,
                    frame.width(), frame.height(),
                );

                if let Some(ref vs) = video_source {
                    vs.capture_frame(&video_frame);
                } else {
                    let vs = NativeVideoSource::new(VideoResolution {
                        width: stream_width, height: stream_height,
                    });
                    video_source_tx.try_send(vs.clone()).ok();
                    vs.capture_frame(&video_frame);
                    video_source = Some(vs);
                }
            }
        };

        let mut capturer = DesktopCapturer::new(wayland_capturer_options())
            .ok_or(anyhow!("Failed to create DesktopCapturer"))?;
        capturer.start_capture(source, callback);

        let task = gpui_tokio::Tokio::spawn(cx, {
            let stop = stop_capture.clone();
            async move {
                loop {
                    if stop.load(Ordering::Acquire) { break; }
                    capturer.capture_frame();
                    tokio::time::sleep(Duration::from_secs_f32(1.0 / 60.0)).await;
                }
            }
        });

        use futures::StreamExt;
        let video_source = video_source_rx.next().await
            .ok_or(anyhow!("No video source received from DesktopCapturer"))?;

        let track = livekit::track::LocalVideoTrack::create_video_track(
            "screen_share",
            RtcVideoSource::Native(video_source),
        );

        let publication = self.publish_track(
            livekit::track::LocalTrack::Video(track),
            TrackPublishOptions {
                source: TrackSource::Screenshare,
                video_codec: VideoCodec::VP8,
                ..Default::default()
            },
            cx,
        ).await?;

        Ok((publication, ScreenCaptureStreamHandle {
            screen_id,
            stop_capture,
            _task: task,
        }))
    }
}
```

### 3. `crates/livekit_client/src/lib.rs`

Re-export the new types from the production module. Add near the existing
`pub use livekit_client::*`:

```rust
// The ScreenCaptureStreamHandle and wayland_screen_capture_sources are
// exported by livekit_client.rs via the existing `pub use livekit_client::*`.
```

No structural change needed — `pub use livekit_client::*` already re-exports
everything public from that module.

### 4. `crates/livekit_client/src/mock_client/participant.rs`

Add a matching mock method to `LocalParticipant`:

```rust
pub async fn publish_screenshare_track_wayland(
    &self,
    _source: Option<libwebrtc::desktop_capturer::CaptureSource>,
    _cx: &mut AsyncApp,
) -> Result<(LocalTrackPublication, ScreenCaptureStreamHandle)> {
    let this = self.clone();
    let server = this.room.test_server();
    let sid = server
        .publish_video_track(this.room.token(), LocalVideoTrack {})
        .await?;
    Ok((
        LocalTrackPublication {
            room: self.room.downgrade(),
            sid,
        },
        ScreenCaptureStreamHandle {
            screen_id: 0,
            stop_capture: Arc::new(AtomicBool::new(false)),
            _task: gpui::Task::ready(Ok(())),
        },
    ))
}
```

Add necessary imports (`Arc`, `AtomicBool`, `ScreenCaptureStreamHandle`).

### 5. `crates/livekit_client/src/mock_client.rs`

Add a mock `wayland_screen_capture_sources`:

```rust
pub fn wayland_screen_capture_sources() -> Vec<libwebrtc::desktop_capturer::CaptureSource> {
    Vec::new()
}
```

### 6. `crates/call/Cargo.toml`

Add `libwebrtc` for the `CaptureSource` type used in the new method signature:

```toml
[dependencies]
# ... existing ...
libwebrtc.workspace = true
```

Note: `libwebrtc` is a workspace dependency gated on
`cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))`.
The new `share_screen_wayland` method should also be gated to only compile on Linux.

### 7. `crates/call/src/call_impl/room.rs`

Add one new method. The existing `share_screen` is **not modified**:

```rust
/// Share screen on Wayland using libwebrtc's DesktopCapturer.
/// The XDG Desktop Portal will show a system picker dialog.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub fn share_screen_wayland(
    &mut self,
    source: Option<libwebrtc::desktop_capturer::CaptureSource>,
    cx: &mut Context<Self>,
) -> Task<Result<()>> {
    if self.status.is_offline() {
        return Task::ready(Err(anyhow!("room is offline")));
    }
    if self.is_sharing_screen() {
        return Task::ready(Err(anyhow!("screen was already shared")));
    }

    let (participant, publish_id) = if let Some(live_kit) = self.live_kit.as_mut() {
        let publish_id = post_inc(&mut live_kit.next_publish_id);
        live_kit.screen_track = LocalTrack::Pending { publish_id };
        cx.notify();
        (live_kit.room.local_participant(), publish_id)
    } else {
        return Task::ready(Err(anyhow!("live-kit was not initialized")));
    };

    cx.spawn(async move |this, cx| {
        let publication = participant
            .publish_screenshare_track_wayland(source, cx)
            .await;

        this.update(cx, |this, cx| {
            let live_kit = this
                .live_kit
                .as_mut()
                .context("live-kit was not initialized")?;

            let canceled = if let LocalTrack::Pending {
                publish_id: cur_publish_id,
            } = &live_kit.screen_track
            {
                *cur_publish_id != publish_id
            } else {
                true
            };

            match publication {
                Ok((publication, handle)) => {
                    if canceled {
                        cx.spawn(async move |_, cx| {
                            participant.unpublish_track(publication.sid(), cx).await
                        })
                        .detach()
                    } else {
                        live_kit.screen_track = LocalTrack::Published {
                            track_publication: publication,
                            _stream: Box::new(handle),
                        };
                        cx.notify();
                    }
                    Audio::play_sound(Sound::StartScreenshare, cx);
                    Ok(())
                }
                Err(error) => {
                    if canceled {
                        Ok(())
                    } else {
                        live_kit.screen_track = LocalTrack::None;
                        cx.notify();
                        Err(error)
                    }
                }
            }
        })?
    })
}
```

The only difference from the existing `share_screen` is the call to
`publish_screenshare_track_wayland` and boxing the handle as
`Box::new(handle)` (which works because `ScreenCaptureStreamHandle` implements
`ScreenCaptureStream`).

### 8. `crates/title_bar/src/collab.rs`

Add a Wayland-aware toggle function alongside the existing one:

```rust
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn toggle_screen_sharing_wayland(window: &mut Window, cx: &mut App) {
    let call = ActiveCall::global(cx).read(cx);
    let Some(room) = call.room().cloned() else { return };
    let task = room.update(cx, |room, cx| {
        if room.is_sharing_screen() {
            telemetry::event!(
                "Screen Share Disabled",
                room_id = room.id(),
                channel_id = room.channel_id(),
            );
            room.unshare_screen(true, cx).ok();
            Task::ready(Ok(()))
        } else {
            telemetry::event!(
                "Screen Share Enabled",
                room_id = room.id(),
                channel_id = room.channel_id(),
            );
            // source = None: the XDG Desktop Portal will show its own picker
            room.share_screen_wayland(None, cx)
        }
    });
    task.detach_and_prompt_err(
        "Sharing Screen Failed", window, cx,
        |e, _, _| Some(format!("{e:?}\n\nPlease check that screen sharing \
            permissions are granted.")),
    );
}
```

In `render_call_controls`, where the screen share button is built, branch on
compositor:

```rust
if can_use_microphone && screen_sharing_supported {
    // ... existing trigger building ...

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    let is_wayland = gpui::guess_compositor() == "Wayland";
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    let is_wayland = false;

    if is_wayland {
        // On Wayland: simple toggle button, no picker dropdown.
        // The XDG Desktop Portal shows the system picker dialog.
        let trigger = trigger.on_click(move |_, window, cx| {
            toggle_screen_sharing_wayland(window, cx);
        });
        children.push(trigger.into_any_element());
    } else {
        // Existing code: SplitButton with screen list dropdown
        // ... unchanged ...
    }
}
```

On Wayland, `is_screen_capture_supported()` currently returns `false`, so we also
need to remove that guard, or make the Wayland path bypass it:

```rust
// Change this:
if can_use_microphone && screen_sharing_supported {
// To this:
let wayland_can_share = cfg!(any(target_os = "linux", target_os = "freebsd"))
    && gpui::guess_compositor() == "Wayland";
if can_use_microphone && (screen_sharing_supported || wayland_can_share) {
```

### 9. `crates/collab_ui/src/collab_panel.rs`

In the `ScreenShare` action handler, add a Wayland branch:

```rust
workspace.register_action(|_, _: &ScreenShare, window, cx| {
    let room = ActiveCall::global(cx).read(cx).room().cloned();
    if let Some(room) = room {
        window.defer(cx, move |_window, cx| {
            room.update(cx, |room, cx| {
                if room.is_sharing_screen() {
                    room.unshare_screen(true, cx).ok();
                } else {
                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                    if gpui::guess_compositor() == "Wayland" {
                        room.share_screen_wayland(None, cx)
                            .detach_and_log_err(cx);
                        return;
                    }

                    // Existing scap path (unchanged)
                    let sources = cx.screen_capture_sources();
                    cx.spawn(async move |room, cx| {
                        let sources = sources.await??;
                        let first = sources.into_iter().next();
                        if let Some(first) = first {
                            room.update(cx, |room, cx| room.share_screen(first, cx))?
                                .await
                        } else {
                            Ok(())
                        }
                    })
                    .detach_and_log_err(cx);
                };
            });
        });
    }
});
```

### 10. `script/linux`

Add system libraries needed by libwebrtc's desktop capture on Linux. These are
already present in `nix/build.nix` but missing from the distro install scripts.

**apt** (Debian/Ubuntu/Mint/etc.) — add to `deps` array:

```bash
libxfixes-dev
libxdamage-dev
libxrandr-dev
libxcomposite-dev
libxext-dev
libdrm-dev
libgbm-dev
```

**dnf** (Fedora/RHEL/etc.) — add to `deps` array:

```bash
libXfixes-devel
libXdamage-devel
libXrandr-devel
libXcomposite-devel
libXext-devel
libdrm-devel
mesa-libgbm-devel
```

**zypper** (openSUSE) — add to `deps` array:

```bash
libXfixes-devel
libXdamage-devel
libXrandr-devel
libXcomposite-devel
libXext-devel
libdrm-devel
Mesa-libgbm-devel
```

**pacman** (Arch) — these are typically pulled in by existing deps but can be
listed explicitly:

```bash
libxfixes
libxdamage
libxrandr
libxcomposite
libxext
libdrm
mesa
```

**xbps** (Void) — add to `deps` array:

```bash
libXfixes-devel
libXdamage-devel
libXrandr-devel
libXcomposite-devel
libXext-devel
libdrm-devel
MesaLib-devel
```

**emerge** (Gentoo) — add to `deps` array:

```bash
x11-libs/libXfixes
x11-libs/libXdamage
x11-libs/libXrandr
x11-libs/libXcomposite
x11-libs/libXext
x11-libs/libdrm
media-libs/mesa
```

---

## What does NOT change

| Area | Why |
|---|---|
| `gpui` `ScreenCaptureSource` / `ScreenCaptureStream` traits | Kept as-is; Wayland path bypasses them |
| `gpui` `Platform` trait (`screen_capture_sources`, `is_screen_capture_supported`) | Kept; Wayland client still returns "not supported" for legacy path |
| `scap` dependency and all scap-based capture code | Untouched; used on X11 and Windows |
| macOS `ScreenCaptureKit` code (`gpui_macos/src/screen_capture.rs`) | Untouched |
| `scap_screen_capture.rs` in gpui | Untouched |
| `video_frame_buffer_to_webrtc()` in `playback.rs` | Untouched; Wayland path does its own ARGB→I420 |
| `Room::share_screen()` (existing scap method) | Untouched |
| `Room::unshare_screen()` | Untouched; works on `dyn ScreenCaptureStream` |
| `Room::shared_screen_id()` | Untouched; works via `ScreenCaptureStream::metadata()` |
| `LiveKitRoom` struct / `LocalTrack` enum | Untouched; `Box<dyn ScreenCaptureStream>` fits both |
| `guess_compositor()` return type | Stays `&'static str` |
| LiveKit fork / revision | No update needed |
| CI workflows | Already have `CC=clang` / `CXX=clang++` |
| Nix build (`nix/build.nix`, `nix/livekit-libwebrtc/`) | Already has all needed deps |

## File change summary

| File | Change |
|---|---|
| `crates/livekit_client/Cargo.toml` | Add `tokio.workspace = true` |
| `crates/livekit_client/src/livekit_client.rs` | Add `ScreenCaptureStreamHandle`, `wayland_screen_capture_sources`, `publish_screenshare_track_wayland` |
| `crates/livekit_client/src/mock_client/participant.rs` | Add mock `publish_screenshare_track_wayland` |
| `crates/livekit_client/src/mock_client.rs` | Add mock `wayland_screen_capture_sources` |
| `crates/call/Cargo.toml` | Add `libwebrtc.workspace = true` |
| `crates/call/src/call_impl/room.rs` | Add `share_screen_wayland` method |
| `crates/title_bar/src/collab.rs` | Add `toggle_screen_sharing_wayland`, branch in `render_call_controls` |
| `crates/collab_ui/src/collab_panel.rs` | Branch on Wayland in `ScreenShare` handler |
| `script/linux` | Add system library packages for all distros |

**9 files touched. Zero modifications to existing capture paths.**

## Testing

1. **Wayland (GNOME/KDE/Sway):** Start a call, click "Share Screen". The XDG
   Desktop Portal picker should appear. After selecting a screen/window, the remote
   participant should see the shared content.
2. **X11:** Verify existing screen sharing still works (scap path unchanged).
3. **macOS:** Verify existing screen sharing still works (native path unchanged).
4. **Integration tests:** Existing tests use `share_screen()` with mock sources and
   should pass without modification. The `share_screen_wayland` path uses the same
   `LocalTrack` machinery and can be tested with the mock.

## Wayland UX notes

- On Wayland, there is **no application-controlled screen picker**. The system's XDG
  Desktop Portal presents its own dialog. This is by design — Wayland forbids apps
  from enumerating screens/windows for privacy.
- The title bar shows a **simple toggle button** (no dropdown picker) on Wayland.
- `DesktopCapturer::get_source_list()` returns a dummy entry on Wayland. The
  `wayland_screen_capture_sources()` function is provided for completeness but the
  primary UX is the portal dialog.
- `CaptureError::Temporary` is expected while the portal picker is open and should be
  silently ignored. `CaptureError::Permanent` stops capture.

## Future work (out of scope)

- Replace scap entirely with `DesktopCapturer` on all platforms (eliminates the dual
  code path).
- Support window-specific capture on Wayland (already partially supported by
  `DesktopCaptureSourceType::Generic`).
- System audio sharing on Wayland.
- Persistent portal permissions / screen share tokens.
