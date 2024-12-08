use crate::{
    new_scap_capturer, size, DevicePixels, ScapFrame, ScapStream, ScreenCaptureFrame,
    ScreenCaptureSource, ScreenCaptureStream, Size,
};
use anyhow::anyhow;
use futures::channel::oneshot;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

struct ScapCapturer {
    stream_tx: std::sync::mpsc::Sender<(
        oneshot::Sender<anyhow::Result<Box<dyn ScreenCaptureStream>>>,
        Box<dyn Fn(ScreenCaptureFrame) + Send>,
    )>,
    size: Size<DevicePixels>,
}

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

/// Requests that wayland prompts the user about which screen or window to capture. The receiver
/// will be filled with a capture source.
pub fn wayland_screen_capture_sources(
) -> oneshot::Receiver<anyhow::Result<Vec<Box<dyn ScreenCaptureSource>>>> {
    let (mut tx, result_rx) = oneshot::channel();

    // Due to use of blocking APIs a dedicated thread is used.
    std::thread::spawn(|| {
        let (stream_tx, stream_rx) = std::sync::mpsc::channel();

        let screen_capturer = util::maybe!({
            // TODO: needed?
            if !scap::has_permission() {
                if !scap::request_permission() {
                    Err(anyhow!("No permissions to share screen"))?;
                }
            }

            let mut capturer = new_scap_capturer(None)?;

            // Screen capture needs to start immediately so that the size can be determined.
            // In Zed the size is needed in order to initialize the LiveKit video channel.
            //
            // FIXME: can this be done way simpler in capture_local_video_track?
            capturer.start_capture();
            let size = match capturer.get_next_frame() {
                Ok(frame) => get_frame_size(&frame),
                Err(std::sync::mpsc::RecvError) => Err(anyhow!(
                    "Failed to get first frame of screenshare to get the size."
                ))?,
            };

            Ok((
                capturer,
                vec![Box::new(ScapCapturer { stream_tx, size }) as Box<dyn ScreenCaptureSource>],
            ))
        });

        match screen_capturer {
            Err(e) => {
                tx.send(Err(e)).ok();
            }
            Ok((mut capturer, sources)) => {
                tx.send(Ok(sources)).ok();

                while let Ok((tx, callback)) = stream_rx.recv() {
                    let cancel_stream = Arc::new(AtomicBool::new(false));
                    tx.send(Ok(Box::new(ScapStream(cancel_stream.clone()))))
                        .ok();
                    while cancel_stream.load(std::sync::atomic::Ordering::SeqCst) {
                        match capturer.get_next_frame() {
                            Ok(frame) => callback(ScreenCaptureFrame(ScapFrame(frame))),
                            Err(std::sync::mpsc::RecvError) => {
                                break;
                            }
                        }
                    }
                }

                capturer.stop_capture();
            }
        }
    });

    result_rx
}

fn get_frame_size(frame: &scap::frame::Frame) -> Size<DevicePixels> {
    match frame {
        scap::frame::Frame::YUVFrame(frame) => {
            size(DevicePixels(frame.width), DevicePixels(frame.height))
        }
        scap::frame::Frame::RGB(frame) => {
            size(DevicePixels(frame.width), DevicePixels(frame.height))
        }
        scap::frame::Frame::RGBx(frame) => {
            size(DevicePixels(frame.width), DevicePixels(frame.height))
        }
        scap::frame::Frame::XBGR(frame) => {
            size(DevicePixels(frame.width), DevicePixels(frame.height))
        }
        scap::frame::Frame::BGRx(frame) => {
            size(DevicePixels(frame.width), DevicePixels(frame.height))
        }
        scap::frame::Frame::BGR0(frame) => {
            size(DevicePixels(frame.width), DevicePixels(frame.height))
        }
        scap::frame::Frame::BGRA(frame) => {
            size(DevicePixels(frame.width), DevicePixels(frame.height))
        }
    }
}
