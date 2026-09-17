use anyhow::{Result, anyhow};
use block2::RcBlock;
use collections::HashMap;
use futures::channel::oneshot;
use gpui::{
    DevicePixels, ForegroundExecutor, ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream,
    SharedString, SourceMetadata, size,
};
use objc2::{
    AnyThread, DefinedClass, MainThreadMarker, define_class, msg_send,
    rc::Retained,
    runtime::{NSObject, ProtocolObject},
};
use objc2_app_kit::NSScreen;
use objc2_core_graphics::{CGDirectDisplayID, CGDisplayCopyDisplayMode, CGDisplayMode};
use objc2_core_media::CMSampleBuffer;
use objc2_foundation::{NSArray, NSError, NSNumber, NSObjectProtocol, NSString};
use objc2_screen_capture_kit::{
    SCContentFilter, SCDisplay, SCShareableContent, SCStream, SCStreamConfiguration,
    SCStreamOutput, SCStreamOutputType, SCWindow,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct MacScreenCaptureSource {
    sc_display: Retained<SCDisplay>,
    meta: Option<ScreenMeta>,
}

pub struct MacScreenCaptureStream {
    sc_stream: Retained<SCStream>,
    sc_stream_output: Retained<StreamOutput>,
    meta: SourceMetadata,
}

impl MacScreenCaptureSource {
    fn metadata(&self) -> SourceMetadata {
        let display_id = unsafe { self.sc_display.displayID() };
        let display_mode = CGDisplayCopyDisplayMode(display_id);
        let width = CGDisplayMode::pixel_width(display_mode.as_deref());
        let height = CGDisplayMode::pixel_height(display_mode.as_deref());
        let size = size(DevicePixels(width as i32), DevicePixels(height as i32));
        let (label, is_main) = self
            .meta
            .clone()
            .map(|meta| (meta.label, meta.is_main))
            .unzip();

        SourceMetadata {
            id: display_id as u64,
            label,
            is_main,
            resolution: size,
        }
    }
}

impl ScreenCaptureSource for MacScreenCaptureSource {
    fn metadata(&self) -> Result<SourceMetadata> {
        Ok(self.metadata())
    }

    fn stream(
        &self,
        _foreground_executor: &ForegroundExecutor,
        frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
    ) -> oneshot::Receiver<Result<Box<dyn ScreenCaptureStream>>> {
        let (sender, receiver) = oneshot::channel();
        let metadata = self.metadata();

        unsafe {
            let excluded_windows = NSArray::<SCWindow>::new();
            let filter = SCContentFilter::initWithDisplay_excludingWindows(
                SCContentFilter::alloc(),
                &self.sc_display,
                &excluded_windows,
            );
            let configuration = SCStreamConfiguration::new();
            configuration.setScalesToFit(true);
            configuration.setPixelFormat(0x42475241);
            configuration.setWidth(metadata.resolution.width.0 as usize);
            configuration.setHeight(metadata.resolution.height.0 as usize);

            let stream = SCStream::initWithFilter_configuration_delegate(
                SCStream::alloc(),
                &filter,
                &configuration,
                None,
            );
            let output = StreamOutput::new(frame_callback);
            let output_protocol = ProtocolObject::from_ref(&*output);

            if let Err(error) = stream.addStreamOutput_type_sampleHandlerQueue_error(
                output_protocol,
                SCStreamOutputType::Screen,
                None,
            ) {
                _ = sender.send(Err(anyhow!(
                    "failed to add stream output: {}",
                    error.localizedDescription()
                )));

                return receiver;
            }

            let state = Rc::new(RefCell::new(Some((sender, stream.clone(), output))));
            let handler = RcBlock::new(move |error: *mut NSError| {
                let Some((sender, stream, output)) = state.borrow_mut().take() else {
                    return;
                };
                let result = if let Some(error) = error.as_ref() {
                    Err(anyhow!(
                        "failed to start screen capture stream: {}",
                        error.localizedDescription()
                    ))
                } else {
                    Ok(Box::new(MacScreenCaptureStream {
                        meta: metadata.clone(),
                        sc_stream: stream,
                        sc_stream_output: output,
                    }) as Box<dyn ScreenCaptureStream>)
                };

                _ = sender.send(result);
            });
            stream.startCaptureWithCompletionHandler(Some(&handler));
        }

        receiver
    }
}

impl ScreenCaptureStream for MacScreenCaptureStream {
    fn metadata(&self) -> Result<SourceMetadata> {
        Ok(self.meta.clone())
    }
}

impl Drop for MacScreenCaptureStream {
    fn drop(&mut self) {
        unsafe {
            let output = ProtocolObject::from_ref(&*self.sc_stream_output);
            if let Err(error) = self
                .sc_stream
                .removeStreamOutput_type_error(output, SCStreamOutputType::Screen)
            {
                log::error!(
                    "failed to remove screen capture stream output: {}",
                    error.localizedDescription()
                );
            }

            let handler = RcBlock::new(move |error: *mut NSError| {
                if let Some(error) = error.as_ref() {
                    log::error!(
                        "failed to stop screen capture stream: {}",
                        error.localizedDescription()
                    );
                }
            });
            self.sc_stream
                .stopCaptureWithCompletionHandler(Some(&handler));
        }
    }
}

#[derive(Clone)]
struct ScreenMeta {
    label: SharedString,
    // Is this the screen with menu bar?
    is_main: bool,
}

fn screen_id_to_human_label(marker: MainThreadMarker) -> HashMap<CGDirectDisplayID, ScreenMeta> {
    let screens = NSScreen::screens(marker);
    let mut map = HashMap::default();

    let screen_number_key = NSString::from_str("NSScreenNumber");

    for (i, screen) in screens.iter().enumerate() {
        let description = screen.deviceDescription();
        let Some(obj) = description.objectForKey(&screen_number_key) else {
            continue;
        };
        let Some(screen_id) = obj.downcast_ref::<NSNumber>() else {
            continue;
        };
        let name = screen.localizedName().to_string();

        map.insert(
            screen_id.as_u32(),
            ScreenMeta {
                label: name.into(),
                is_main: i == 0,
            },
        );
    }

    map
}

pub(crate) fn get_sources(
    marker: MainThreadMarker,
) -> oneshot::Receiver<Result<Vec<Rc<dyn ScreenCaptureSource>>>> {
    let (tx, rx) = oneshot::channel();
    let tx = Rc::new(RefCell::new(Some(tx)));
    let screen_id_to_label = screen_id_to_human_label(marker);

    let handler = RcBlock::new(
        move |content: *mut SCShareableContent, error: *mut NSError| {
            let Some(tx) = tx.borrow_mut().take() else {
                return;
            };

            let result = if let Some(error) = unsafe { error.as_ref() } {
                Err(anyhow!(
                    "Screen share failed: {}",
                    error.localizedDescription()
                ))
            } else if let Some(content) = unsafe { content.as_ref() } {
                // SAFETY: Marked unsafe conservatively by objc2
                let result = unsafe { content.displays() }
                    .into_iter()
                    .map(|display| {
                        // SAFETY: Marked unsafe conservatively by objc2
                        let id = unsafe { display.displayID() };
                        let metadata = screen_id_to_label.get(&id).cloned();
                        let source = MacScreenCaptureSource {
                            sc_display: display,
                            meta: metadata,
                        };
                        Rc::new(source) as Rc<dyn ScreenCaptureSource>
                    })
                    .collect::<Vec<_>>();

                Ok(result)
            } else {
                // The two pointers are mutually exclusive, this should never happen
                Err(anyhow!("Screen share failed"))
            };

            _ = tx.send(result);
        },
    );

    unsafe {
        SCShareableContent::getShareableContentExcludingDesktopWindows_onScreenWindowsOnly_completionHandler(true, true, &handler);
    }

    rx
}

struct StreamOutputIvars {
    frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
}

define_class!(
    // SAFETY: `NSObject` has no subclassing requirements, and `StreamOutput` does not
    // implement `Drop`; `define_class!` manages destruction of its ivars.
    #[unsafe(super(NSObject))]
    #[ivars = StreamOutputIvars]
    struct StreamOutput;

    unsafe impl NSObjectProtocol for StreamOutput {}

    unsafe impl SCStreamOutput for StreamOutput {
        #[unsafe(method(stream:didOutputSampleBuffer:ofType:))]
        fn stream_did_output_sample_buffer_of_type(
            &self,
            _stream: &SCStream,
            sample_buffer: &CMSampleBuffer,
            buffer_type: SCStreamOutputType,
        ) {
            if buffer_type != SCStreamOutputType::Screen {
                return;
            }

            let Some(image_buffer) = (unsafe { sample_buffer.image_buffer() }) else {
                return;
            };
            (self.ivars().frame_callback)(ScreenCaptureFrame(image_buffer));
        }
    }
);

impl StreamOutput {
    fn new(frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(StreamOutputIvars { frame_callback });
        unsafe { msg_send![super(this), init] }
    }
}
