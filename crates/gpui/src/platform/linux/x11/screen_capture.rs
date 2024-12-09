use crate::{
    new_scap_capturer, DevicePixels, ScapFrame, ScapStream, ScreenCaptureFrame,
    ScreenCaptureSource, Size,
};
use anyhow::anyhow;
use futures::channel::oneshot;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

struct X11ScreenCaptureSource {
    target: scap::Target,
    size: Size<DevicePixels>,
}

impl ScreenCaptureSource for X11ScreenCaptureSource {
    fn resolution(&self) -> anyhow::Result<Size<DevicePixels>> {
        Ok(self.size)
    }

    fn stream(
        &self,
        frame_callback: Box<dyn Fn(crate::ScreenCaptureFrame) + Send>,
    ) -> oneshot::Receiver<anyhow::Result<Box<dyn crate::ScreenCaptureStream>>> {
        let (tx, rx) = oneshot::channel();

        // TODO: can clone be avoided here and elsewhere?
        let target = self.target.clone();
        // Due to use of blocking APIs a dedicated thread is used.
        std::thread::spawn(move || {
            let cancel_stream = Arc::new(AtomicBool::new(false));
            let mut capturer = match new_scap_capturer(Some(target)) {
                Ok(capturer) => {
                    tx.send(Ok(Box::new(ScapStream(cancel_stream.clone()))
                        as Box<dyn crate::ScreenCaptureStream>))
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
                    Ok(frame) => frame_callback(ScreenCaptureFrame(ScapFrame(frame))),
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

pub fn x11_screen_capture_sources() -> anyhow::Result<Vec<Box<dyn ScreenCaptureSource>>> {
    if !scap::has_permission() {
        if !scap::request_permission() {
            Err(anyhow!("No permissions to share screen"))?;
        }
    }

    // TODO(mgsloan): Handle window capture too? On Mac it's only displays.
    Ok(scap::get_all_targets()
        .iter()
        .filter_map(|target| match target {
            scap::Target::Display(display) => {
                let size = Size {
                    width: DevicePixels(display.width as i32),
                    height: DevicePixels(display.height as i32),
                };
                Some(Box::new(X11ScreenCaptureSource {
                    target: target.clone(),
                    size,
                }) as Box<dyn ScreenCaptureSource>)
            }
            scap::Target::Window(_) => None,
        })
        .collect::<Vec<_>>())
}
