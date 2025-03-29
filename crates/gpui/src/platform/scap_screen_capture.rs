//! Screen capture for Linux and Windows
use crate::{DevicePixels, ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream, Size};
use futures::channel::oneshot;
use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;

pub(crate) fn scap_screen_sources(
) -> oneshot::Receiver<anyhow::Result<Vec<Box<dyn ScreenCaptureSource>>>> {
    let (tx, rx) = oneshot::channel();
    // Due to use of blocking APIs, a new thread is used.
    std::thread::spawn(|| {
        let sources = scap::get_all_targets()
            .iter()
            .filter_map(|target| match target {
                scap::Target::Display(display) => {
                    let size = Size {
                        width: DevicePixels(display.width as i32),
                        height: DevicePixels(display.height as i32),
                    };
                    Some(Box::new(ScapScreenCaptureSource {
                        target: target.clone(),
                        size,
                    }) as Box<dyn ScreenCaptureSource>)
                }
                scap::Target::Window(_) => None,
            })
            .collect::<Vec<_>>();
        tx.send(anyhow::Ok(sources));
    });
    // Ignore window targets for now as there is no selection UI.
    rx
}

struct ScapScreenCaptureSource {
    target: scap::Target,
    size: Size<DevicePixels>,
}

impl ScreenCaptureSource for ScapScreenCaptureSource {
    fn resolution(&self) -> anyhow::Result<Size<DevicePixels>> {
        Ok(self.size)
    }

    fn stream(
        &self,
        frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
    ) -> oneshot::Receiver<anyhow::Result<Box<dyn ScreenCaptureStream>>> {
        let (tx, rx) = oneshot::channel();

        let target = self.target.clone();
        // Due to use of blocking APIs, a dedicated thread is used.
        std::thread::spawn(move || {
            let cancel_stream = Arc::new(AtomicBool::new(false));
            let mut capturer = match new_scap_capturer(Some(target)) {
                Ok(capturer) => {
                    let cancel_stream = cancel_stream.clone();
                    tx.send(Ok(
                        Box::new(ScapStream { cancel_stream }) as Box<dyn ScreenCaptureStream>
                    ))
                    .ok();
                    capturer
                }
                Err(e) => {
                    tx.send(Err(e)).ok();
                    return;
                }
            };
            while cancel_stream.load(std::sync::atomic::Ordering::SeqCst) {
                match capturer.get_next_frame() {
                    Ok(frame) => frame_callback(ScreenCaptureFrame(frame)),
                    Err(std::sync::mpsc::RecvError) => {
                        break;
                    }
                }
            }
            capturer.stop_capture();
        });

        rx
    }
}

fn new_scap_capturer(target: Option<scap::Target>) -> anyhow::Result<scap::capturer::Capturer> {
    Ok(scap::capturer::Capturer::build(scap::capturer::Options {
        fps: 60,
        show_cursor: true,
        show_highlight: true,
        // Note that the actual frame output type may differ.
        output_type: scap::frame::FrameType::YUVFrame,
        output_resolution: scap::capturer::Resolution::Captured,
        crop_area: None,
        target,
        excluded_targets: None,
    })?)
}

struct ScapStream {
    cancel_stream: Arc<AtomicBool>,
}

impl ScreenCaptureStream for ScapStream {}

impl Drop for ScapStream {
    fn drop(&mut self) {
        self.cancel_stream.store(true, atomic::Ordering::SeqCst);
    }
}

/*

struct ScapScreenCaptureSource {
    stream_tx: std::sync::mpsc::Sender<(
        oneshot::Sender<anyhow::Result<Box<dyn ScreenCaptureStream>>>,
        Box<dyn Fn(ScreenCaptureFrame) + Send>,
    )>,
    size: Size<DevicePixels>,
};

struct Wrapper(Task<()>);
impl ScreenCaptureStream for Wrapper {}

impl ScreenCaptureSource for ScapCapturer {
    fn resolution(&self) -> anyhow::Result<Size<DevicePixels>> {
        Ok(self.size)
    }

    fn stream(
        &self,
        frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
    ) -> oneshot::Receiver<anyhow::Result<Box<dyn ScreenCaptureStream>>> {
        let (tx, rx) = oneshot::channel();
        self.stream_tx.send((tx, frame_callback)).ok();
        rx
    }
}

pub(crate) async fn capture_local_video_track(
    cx: &mut AsyncApp,
) -> Result<(crate::LocalVideoTrack, Box<dyn ScreenCaptureStream>)> {
    let running = Arc::new(atomic::AtomicBool::new(true));
    // todo! bound
    let (mut frame_tx, mut frame_rx) = futures::channel::mpsc::channel(1);
    std::thread::spawn(move || {
        // TODO: needed?
        if !scap::has_permission() {
            if !scap::request_permission() {
                // todo! no panic
                panic!("no permission");
            }
        }

        let mut capturer = scap::capturer::Capturer::build(scap::capturer::Options {
            fps: 60,
            show_cursor: true,
            show_highlight: true,
            output_type: scap::frame::FrameType::YUVFrame,
            output_resolution: scap::capturer::Resolution::Captured,
            crop_area: None,
            target: None,
            excluded_targets: None,
        })
        .unwrap();

        capturer.start_capture();
        while running.load(atomic::Ordering::Relaxed) {
            // todo! how to handle errors?
            if let Ok(frame) = capturer.get_next_frame() {
                // todo! remove log_err
                frame_tx.try_send(frame).log_err();
            }
        }
    });

    let first_frame = frame_rx.next().await.unwrap();

    let (width, height) = match first_frame {
        scap::frame::Frame::YUVFrame(frame) => (frame.width, frame.height),
        scap::frame::Frame::RGB(frame) => (frame.width, frame.height),
        scap::frame::Frame::RGBx(frame) => (frame.width, frame.height),
        scap::frame::Frame::XBGR(frame) => (frame.width, frame.height),
        scap::frame::Frame::BGRx(frame) => (frame.width, frame.height),
        scap::frame::Frame::BGR0(frame) => (frame.width, frame.height),
        scap::frame::Frame::BGRA(frame) => (frame.width, frame.height),
    };

    let track_source = gpui_tokio::Tokio::spawn(cx, async move {
        NativeVideoSource::new(VideoResolution {
            width: width as u32,
            height: height as u32,
        })
    })?
    .await?;
    let track_source_2 = track_source.clone();

    let task = cx.background_spawn(async move {
        while let Some(frame) = frame_rx.next().await {
            track_source_2.capture_frame(&VideoFrame {
                rotation: VideoRotation::VideoRotation0,
                timestamp_us: 0,
                buffer: video_frame_buffer_to_webrtc(frame).unwrap(),
            });
        }
    });

    Ok((
        LocalVideoTrack(track::LocalVideoTrack::create_video_track(
            "screen share",
            RtcVideoSource::Native(track_source),
        )),
        Box::new(Wrapper(task)),
    ))
}
*/
