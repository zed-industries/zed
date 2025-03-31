//! Screen capture for Linux and Windows
use crate::{
    size, DevicePixels, ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream, Size,
};
use anyhow::{anyhow, Result};
use futures::channel::oneshot;
use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;
use util::ResultExt;

/// Populates the receiver with the screens that can be captured.
///
/// `scap_default_target_source` should be used instead on Wayland, since `scap_screen_sources`
/// won't return any results.
pub(crate) fn scap_screen_sources() -> oneshot::Receiver<Result<Vec<Box<dyn ScreenCaptureSource>>>>
{
    let (tx, rx) = oneshot::channel();
    get_screen_targets(tx);
    rx
}

/// Starts screen capture for the default target, and populates the receiver with a single source
/// for it. The first frame of the screen capture is used to determine the size of the stream.
///
/// On Wayland (Linux), prompts the user to select a target, and populates the receiver with a
/// single screen capture source for their selection.
///
/// todo! What happens if a wayland window is resized?
pub(crate) fn start_scap_default_target_source(
) -> oneshot::Receiver<Result<Vec<Box<dyn ScreenCaptureSource>>>> {
    let (sources_tx, sources_rx) = oneshot::channel();
    start_default_target_screen_capture(sources_tx);
    sources_rx
}

struct ScapCaptureSource {
    target: scap::Target,
    size: Size<DevicePixels>,
}

/// Populates the sender with the screens available for capture.
fn get_screen_targets(sources_tx: oneshot::Sender<Result<Vec<Box<dyn ScreenCaptureSource>>>>) {
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
                    Some(Box::new(ScapCaptureSource {
                        target: target.clone(),
                        size,
                    }) as Box<dyn ScreenCaptureSource>)
                }
                scap::Target::Window(_) => None,
            })
            .collect::<Vec<_>>();
        sources_tx.send(Ok(sources)).ok();
    });
}

impl ScreenCaptureSource for ScapCaptureSource {
    fn resolution(&self) -> Result<Size<DevicePixels>> {
        Ok(self.size)
    }

    fn stream(
        &self,
        frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
    ) -> oneshot::Receiver<Result<Box<dyn ScreenCaptureStream>>> {
        let (stream_tx, stream_rx) = oneshot::channel();
        let target = self.target.clone();

        // Due to use of blocking APIs, a dedicated thread is used.
        std::thread::spawn(move || match new_scap_capturer(Some(target)) {
            Ok(mut capturer) => {
                capturer.start_capture();
                run_capture(capturer, frame_callback, stream_tx);
            }
            Err(e) => {
                stream_tx.send(Err(e)).ok();
            }
        });

        stream_rx
    }
}

struct ScapDefaultTargetCaptureSource {
    // Sender populated by single call to `ScreenCaptureSource::stream`.
    stream_call_tx: std::sync::mpsc::SyncSender<(
        // Provides the result of `ScreenCaptureSource::stream`.
        oneshot::Sender<Result<Box<dyn ScreenCaptureStream>>>,
        // Callback for frames.
        Box<dyn Fn(ScreenCaptureFrame) + Send>,
    )>,
    size: Size<DevicePixels>,
}

/// Starts screen capture on the default capture target, and populates the sender with the source.
fn start_default_target_screen_capture(
    sources_tx: oneshot::Sender<Result<Vec<Box<dyn ScreenCaptureSource>>>>,
) {
    // Due to use of blocking APIs, a dedicated thread is used.
    std::thread::spawn(|| {
        let start_result = util::maybe!({
            let mut capturer = new_scap_capturer(None)?;
            capturer.start_capture();
            let size = match capturer.get_next_frame() {
                Ok(frame) => frame_size(&frame),
                Err(std::sync::mpsc::RecvError) => Err(anyhow!(
                    "Failed to get first frame of screenshare to get the size."
                ))?,
            };
            Ok((capturer, size))
        });

        match start_result {
            Err(e) => {
                sources_tx.send(Err(e)).ok();
            }
            Ok((capturer, size)) => {
                let (stream_call_tx, stream_rx) = std::sync::mpsc::sync_channel(1);
                sources_tx
                    .send(Ok(vec![Box::new(ScapDefaultTargetCaptureSource {
                        stream_call_tx,
                        size,
                    })]))
                    .ok();
                let Ok((stream_tx, frame_callback)) = stream_rx.recv() else {
                    return;
                };
                run_capture(capturer, frame_callback, stream_tx);
            }
        }
    });
}

impl ScreenCaptureSource for ScapDefaultTargetCaptureSource {
    fn resolution(&self) -> Result<Size<DevicePixels>> {
        Ok(self.size)
    }

    fn stream(
        &self,
        frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
    ) -> oneshot::Receiver<Result<Box<dyn ScreenCaptureStream>>> {
        let (tx, rx) = oneshot::channel();
        match self.stream_call_tx.try_send((tx, frame_callback)) {
            Ok(()) => {}
            Err(std::sync::mpsc::TrySendError::Full((tx, _)))
            | Err(std::sync::mpsc::TrySendError::Disconnected((tx, _))) => {
                // Note: support could be added for being called again after end of prior stream.
                tx.send(Err(anyhow!(
                    "Can't call ScapDefaultTargetCaptureSource::stream multiple times."
                )))
                .ok();
            }
        }
        rx
    }
}

fn new_scap_capturer(target: Option<scap::Target>) -> Result<scap::capturer::Capturer> {
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

fn run_capture(
    mut capturer: scap::capturer::Capturer,
    frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
    stream_tx: oneshot::Sender<Result<Box<dyn ScreenCaptureStream>>>,
) {
    let cancel_stream = Arc::new(AtomicBool::new(false));
    let stream_send_result = stream_tx.send(Ok(Box::new(ScapStream {
        cancel_stream: cancel_stream.clone(),
    }) as Box<dyn ScreenCaptureStream>));
    if let Err(_) = stream_send_result {
        return;
    }
    while !cancel_stream.load(std::sync::atomic::Ordering::SeqCst) {
        match capturer.get_next_frame() {
            Ok(frame) => frame_callback(ScreenCaptureFrame(frame)),
            Err(std::sync::mpsc::RecvError) => {
                log::error!("Screen capture stream unexpectedly ended.");
                break;
            }
        }
    }
    capturer.stop_capture();
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

fn frame_size(frame: &scap::frame::Frame) -> Size<DevicePixels> {
    let (width, height) = match frame {
        scap::frame::Frame::YUVFrame(frame) => (frame.width, frame.height),
        scap::frame::Frame::RGB(frame) => (frame.width, frame.height),
        scap::frame::Frame::RGBx(frame) => (frame.width, frame.height),
        scap::frame::Frame::XBGR(frame) => (frame.width, frame.height),
        scap::frame::Frame::BGRx(frame) => (frame.width, frame.height),
        scap::frame::Frame::BGR0(frame) => (frame.width, frame.height),
        scap::frame::Frame::BGRA(frame) => (frame.width, frame.height),
    };
    size(DevicePixels(width), DevicePixels(height))
}
