use anyhow::Result;
use futures::StreamExt as _;
use futures::channel::oneshot;
use gpui::{AsyncApp, ScreenCaptureStream};
use livekit::track;
use livekit::webrtc::{
    prelude::NV12Buffer,
    video_frame::{VideoFrame, VideoRotation},
    video_source::{RtcVideoSource, VideoResolution, native::NativeVideoSource},
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

static NEXT_WAYLAND_SHARE_ID: AtomicU64 = AtomicU64::new(1);
const PIPEWIRE_TIMEOUT_S: u64 = 30;

pub struct WaylandScreenCaptureStream {
    id: u64,
    stop_flag: Arc<AtomicBool>,
    _capture_task: gpui::Task<()>,
}

impl WaylandScreenCaptureStream {
    pub fn new(stop_flag: Arc<AtomicBool>, capture_task: gpui::Task<()>) -> Self {
        Self {
            id: NEXT_WAYLAND_SHARE_ID.fetch_add(1, Ordering::Relaxed),
            stop_flag,
            _capture_task: capture_task,
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
        self.stop_flag.store(true, Ordering::Release);
    }
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
        DesktopFrame,
    };
    use libwebrtc::native::yuv_helper::argb_to_nv12;
    use std::time::Duration;
    use webrtc_sys::webrtc::ffi as webrtc_ffi;

    fn webrtc_log_callback(message: String, severity: webrtc_ffi::LoggingSeverity) {
        match severity {
            webrtc_ffi::LoggingSeverity::Error => log::error!("[webrtc] {}", message.trim()),
            _ => log::debug!("[webrtc] {}", message.trim()),
        }
    }

    let _webrtc_log_sink = webrtc_ffi::new_log_sink(webrtc_log_callback);
    log::debug!("Wayland desktop capture: WebRTC internal logging enabled");

    let stop_flag = Arc::new(AtomicBool::new(false));
    let (mut video_source_tx, mut video_source_rx) = mpsc::channel::<NativeVideoSource>(1);
    let (failure_tx, failure_rx) = oneshot::channel::<()>();

    let mut options = DesktopCapturerOptions::new(DesktopCaptureSourceType::Generic);
    options.set_include_cursor(true);
    let mut capturer = DesktopCapturer::new(options).ok_or_else(|| {
        anyhow::anyhow!(
            "Failed to create desktop capturer. \
             Check that xdg-desktop-portal is installed and running."
        )
    })?;

    let permanent_error = Arc::new(AtomicBool::new(false));
    let stop_cb = stop_flag.clone();
    let permanent_error_cb = permanent_error.clone();
    capturer.start_capture(None, {
        let mut video_source: Option<NativeVideoSource> = None;
        let mut current_width: u32 = 0;
        let mut current_height: u32 = 0;
        let mut video_frame = VideoFrame {
            rotation: VideoRotation::VideoRotation0,
            buffer: NV12Buffer::new(1, 1),
            timestamp_us: 0,
        };

        move |result: Result<DesktopFrame, CaptureError>| {
            let frame = match result {
                Ok(frame) => frame,
                Err(CaptureError::Temporary) => return,
                Err(CaptureError::Permanent) => {
                    log::error!("Wayland desktop capture encountered a permanent error");
                    permanent_error_cb.store(true, Ordering::Release);
                    stop_cb.store(true, Ordering::Release);
                    return;
                }
            };

            let width = frame.width() as u32;
            let height = frame.height() as u32;
            if width != current_width || height != current_height {
                current_width = width;
                current_height = height;
                video_frame.buffer = NV12Buffer::new(width, height);
            }

            let (stride_y, stride_uv) = video_frame.buffer.strides();
            let (data_y, data_uv) = video_frame.buffer.data_mut();
            argb_to_nv12(
                frame.data(),
                frame.stride(),
                data_y,
                stride_y,
                data_uv,
                stride_uv,
                width as i32,
                height as i32,
            );

            if let Some(source) = &video_source {
                source.capture_frame(&video_frame);
            } else {
                let source = NativeVideoSource::new(VideoResolution { width, height }, true);
                source.capture_frame(&video_frame);
                video_source_tx.try_send(source.clone()).ok();
                video_source = Some(source);
            }
        }
    });

    log::info!("Wayland desktop capture: starting capture loop");

    let stop = stop_flag.clone();
    let tokio_task = gpui_tokio::Tokio::spawn(cx, async move {
        loop {
            if stop.load(Ordering::Acquire) {
                break;
            }
            capturer.capture_frame();
            tokio::time::sleep(Duration::from_millis(33)).await;
        }
        drop(capturer);

        if permanent_error.load(Ordering::Acquire) {
            log::error!("Wayland screen capture ended due to a permanent capture error");
            let _ = failure_tx.send(());
        }
    });

    let capture_task = cx.background_executor().spawn(async move {
        if let Err(error) = tokio_task.await {
            log::error!("Wayland capture task failed: {error}");
        }
    });

    let executor = cx.background_executor().clone();
    let video_source = video_source_rx
        .next()
        .with_timeout(Duration::from_secs(PIPEWIRE_TIMEOUT_S), &executor)
        .await
        .map_err(|_| {
            stop_flag.store(true, Ordering::Relaxed);
            log::error!("Wayland desktop capture timed out.");
            anyhow::anyhow!(
                "Screen sharing timed out waiting for the first frame. \
                 Check that xdg-desktop-portal and PipeWire are running, \
                 and that your portal backend matches your compositor."
            )
        })?
        .ok_or_else(|| {
            stop_flag.store(true, Ordering::Relaxed);
            anyhow::anyhow!(
                "Screen sharing was canceled or the portal denied permission. \
                 You can try again from the screen share button."
            )
        })?;

    let track = super::LocalVideoTrack(track::LocalVideoTrack::create_video_track(
        "screen share",
        RtcVideoSource::Native(video_source),
    ));

    Ok((track, stop_flag, capture_task, failure_rx))
}
