use anyhow::Result;
use futures::StreamExt as _;
use futures::channel::oneshot;
use gpui::{AsyncApp, ScreenCaptureStream};
use livekit::track;
use livekit::webrtc::{
    video_frame::{VideoFrame, VideoRotation},
    video_source::{RtcVideoSource, VideoResolution, native::NativeVideoSource},
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};

static NEXT_WAYLAND_SHARE_ID: AtomicU64 = AtomicU64::new(1);

pub struct WaylandScreenCaptureStream {
    id: u64,
    stop_flag: Arc<AtomicBool>,
    _feed_task: gpui::Task<()>,
}

impl WaylandScreenCaptureStream {
    pub fn new(stop_flag: Arc<AtomicBool>, feed_task: gpui::Task<()>) -> Self {
        Self {
            id: NEXT_WAYLAND_SHARE_ID.fetch_add(1, Ordering::Relaxed),
            stop_flag,
            _feed_task: feed_task,
        }
    }
}

impl ScreenCaptureStream for WaylandScreenCaptureStream {
    fn metadata(&self) -> Result<gpui::SourceMetadata> {
        Ok(gpui::SourceMetadata {
            id: self.id,
            label: None,
            is_main: None,
            resolution: gpui::size(gpui::DevicePixels(1), gpui::DevicePixels(1)),
        })
    }
}

impl Drop for WaylandScreenCaptureStream {
    fn drop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}

struct CapturedFrame {
    width: u32,
    height: u32,
    stride: u32,
    data: Vec<u8>,
}

fn desktop_frame_to_nv12(frame: &CapturedFrame) -> livekit::webrtc::prelude::NV12Buffer {
    use libwebrtc::native::yuv_helper::argb_to_nv12;
    use livekit::webrtc::prelude::NV12Buffer;

    let mut buffer = NV12Buffer::new(frame.width, frame.height);
    let (stride_y, stride_uv) = buffer.strides();
    let (data_y, data_uv) = buffer.data_mut();
    argb_to_nv12(
        &frame.data,
        frame.stride,
        data_y,
        stride_y,
        data_uv,
        stride_uv,
        frame.width as i32,
        frame.height as i32,
    );
    buffer
}

pub(crate) async fn start_wayland_desktop_capture(
    cx: &mut AsyncApp,
) -> Result<(
    crate::LocalVideoTrack,
    Arc<AtomicBool>,
    gpui::Task<()>,
    oneshot::Receiver<()>,
)> {
    use futures::channel::mpsc;
    use gpui::FutureExt as _;
    use libwebrtc::desktop_capturer::{
        CaptureError, DesktopCaptureSourceType, DesktopCapturer, DesktopCapturerOptions,
    };

    let (frame_tx, mut frame_rx) = mpsc::channel::<CapturedFrame>(2);
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop = stop_flag.clone();

    let permanent_error = Arc::new(AtomicBool::new(false));
    let permanent_error_cb = permanent_error.clone();

    let executor = cx.background_executor().clone();

    let capture_executor = executor.clone();
    executor
        .spawn(async move {
            let mut options = DesktopCapturerOptions::new(DesktopCaptureSourceType::Generic);
            options.set_include_cursor(true);

            let Some(mut capturer) = DesktopCapturer::new(options) else {
                log::error!(
                    "Failed to create Wayland desktop capturer. Is xdg-desktop-portal running?"
                );
                return;
            };

            let frame_tx_cb = parking_lot::Mutex::new(frame_tx.clone());
            capturer.start_capture(None, move |result| match result {
                Ok(frame) => {
                    let captured = CapturedFrame {
                        width: frame.width() as u32,
                        height: frame.height() as u32,
                        stride: frame.stride(),
                        data: frame.data().to_vec(),
                    };
                    frame_tx_cb.lock().try_send(captured).ok();
                }
                Err(CaptureError::Temporary) => {
                    // Expected before the portal picker completes
                }
                Err(CaptureError::Permanent) => {
                    permanent_error_cb.store(true, Ordering::Relaxed);
                    log::error!("Wayland desktop capture encountered a permanent error");
                }
            });

            while !stop.load(Ordering::Relaxed) {
                capturer.capture_frame();
                if permanent_error.load(Ordering::Relaxed) {
                    break;
                }
                capture_executor.timer(Duration::from_millis(33)).await;
            }

            drop(frame_tx);
        })
        .detach();
    let first_frame = frame_rx
        .next()
        .with_timeout(Duration::from_secs(15), &executor)
        .await
        .map_err(|_| {
            stop_flag.store(true, Ordering::Relaxed);
            anyhow::anyhow!(
                "Screen sharing timed out waiting for the first frame. \
                 Check that xdg-desktop-portal and PipeWire are running, \
                 and that your portal backend matches your compositor."
            )
        })?
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Screen sharing was canceled or the portal denied permission. \
                 You can try again from the screen share button."
            )
        })?;

    let width = first_frame.width;
    let height = first_frame.height;
    let video_source = gpui_tokio::Tokio::spawn(cx, async move {
        NativeVideoSource::new(VideoResolution { width, height }, true)
    })
    .await?;

    let nv12 = desktop_frame_to_nv12(&first_frame);
    video_source.capture_frame(&VideoFrame {
        rotation: VideoRotation::VideoRotation0,
        timestamp_us: 0,
        buffer: nv12,
    });

    let track = super::LocalVideoTrack(track::LocalVideoTrack::create_video_track(
        "screen share",
        RtcVideoSource::Native(video_source.clone()),
    ));

    let (failure_tx, failure_rx) = oneshot::channel::<()>();
    let feed_stop = stop_flag.clone();
    let feed_task = cx.background_executor().spawn(async move {
        while let Some(frame) = frame_rx.next().await {
            if feed_stop.load(Ordering::Relaxed) {
                break;
            }
            let nv12 = desktop_frame_to_nv12(&frame);
            video_source.capture_frame(&VideoFrame {
                rotation: VideoRotation::VideoRotation0,
                timestamp_us: 0,
                buffer: nv12,
            });
        }
        if !feed_stop.load(Ordering::Relaxed) {
            log::error!("Wayland screen capture ended unexpectedly");
            let _ = failure_tx.send(());
        }
    });

    Ok((track, stop_flag, feed_task, failure_rx))
}
