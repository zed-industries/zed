use crate::{
    DevicePixels, ForegroundExecutor, Size,
    platform::{ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream},
    size,
};
use anyhow::{Result, anyhow};
use block::ConcreteBlock;
use cocoa::{
    base::{YES, id, nil},
    foundation::NSArray,
};
use core_foundation::base::TCFType;
use core_graphics::display::{
    CGDirectDisplayID, CGDisplayCopyDisplayMode, CGDisplayModeGetPixelHeight,
    CGDisplayModeGetPixelWidth, CGDisplayModeRelease,
};
use ctor::ctor;
use futures::channel::oneshot;
use media::core_media::{CMSampleBuffer, CMSampleBufferRef};
use metal::NSInteger;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use std::{cell::RefCell, ffi::c_void, mem, ptr, rc::Rc};

use super::NSStringExt;

#[derive(Clone)]
pub struct MacScreenCaptureSource {
    sc_display: id,
}

pub struct MacScreenCaptureStream {
    sc_stream: id,
    sc_stream_output: id,
}

static mut DELEGATE_CLASS: *const Class = ptr::null();
static mut OUTPUT_CLASS: *const Class = ptr::null();
const FRAME_CALLBACK_IVAR: &str = "frame_callback";

#[allow(non_upper_case_globals)]
const SCStreamOutputTypeScreen: NSInteger = 0;

impl ScreenCaptureSource for MacScreenCaptureSource {
    fn resolution(&self) -> Result<Size<DevicePixels>> {
        unsafe {
            let display_id: CGDirectDisplayID = msg_send![self.sc_display, displayID];
            let display_mode_ref = CGDisplayCopyDisplayMode(display_id);
            let width = CGDisplayModeGetPixelWidth(display_mode_ref);
            let height = CGDisplayModeGetPixelHeight(display_mode_ref);
            CGDisplayModeRelease(display_mode_ref);

            Ok(size(
                DevicePixels(width as i32),
                DevicePixels(height as i32),
            ))
        }
    }

    fn stream(
        &self,
        _foreground_executor: &ForegroundExecutor,
        frame_callback: Box<dyn Fn(ScreenCaptureFrame) + Send>,
    ) -> oneshot::Receiver<Result<Box<dyn ScreenCaptureStream>>> {
        unsafe {
            let stream: id = msg_send![class!(SCStream), alloc];
            let filter: id = msg_send![class!(SCContentFilter), alloc];
            let configuration: id = msg_send![class!(SCStreamConfiguration), alloc];
            let delegate: id = msg_send![DELEGATE_CLASS, alloc];
            let output: id = msg_send![OUTPUT_CLASS, alloc];

            let excluded_windows = NSArray::array(nil);
            let filter: id = msg_send![filter, initWithDisplay:self.sc_display excludingWindows:excluded_windows];
            let configuration: id = msg_send![configuration, init];
            let _: id = msg_send![configuration, setScalesToFit: true];
            let _: id = msg_send![configuration, setPixelFormat: 0x42475241];
            // let _: id = msg_send![configuration, setShowsCursor: false];
            // let _: id = msg_send![configuration, setCaptureResolution: 3];
            let delegate: id = msg_send![delegate, init];
            let output: id = msg_send![output, init];

            output.as_mut().unwrap().set_ivar(
                FRAME_CALLBACK_IVAR,
                Box::into_raw(Box::new(frame_callback)) as *mut c_void,
            );

            let resolution = self.resolution().unwrap();
            let _: id = msg_send![configuration, setWidth: resolution.width.0 as i64];
            let _: id = msg_send![configuration, setHeight: resolution.height.0 as i64];
            let stream: id = msg_send![stream, initWithFilter:filter configuration:configuration delegate:delegate];

            let (mut tx, rx) = oneshot::channel();

            let mut error: id = nil;
            let _: () = msg_send![stream, addStreamOutput:output type:SCStreamOutputTypeScreen sampleHandlerQueue:0 error:&mut error as *mut id];
            if error != nil {
                let message: id = msg_send![error, localizedDescription];
                tx.send(Err(anyhow!("failed to add stream  output {message:?}")))
                    .ok();
                return rx;
            }

            let tx = Rc::new(RefCell::new(Some(tx)));
            let handler = ConcreteBlock::new({
                move |error: id| {
                    let result = if error == nil {
                        let stream = MacScreenCaptureStream {
                            sc_stream: stream,
                            sc_stream_output: output,
                        };
                        Ok(Box::new(stream) as Box<dyn ScreenCaptureStream>)
                    } else {
                        let message: id = msg_send![error, localizedDescription];
                        Err(anyhow!("failed to stop screen capture stream {message:?}"))
                    };
                    if let Some(tx) = tx.borrow_mut().take() {
                        tx.send(result).ok();
                    }
                }
            });
            let handler = handler.copy();
            let _: () = msg_send![stream, startCaptureWithCompletionHandler:handler];
            rx
        }
    }
}

impl Drop for MacScreenCaptureSource {
    fn drop(&mut self) {
        unsafe {
            let _: () = msg_send![self.sc_display, release];
        }
    }
}

impl ScreenCaptureStream for MacScreenCaptureStream {}

impl Drop for MacScreenCaptureStream {
    fn drop(&mut self) {
        unsafe {
            let mut error: id = nil;
            let _: () = msg_send![self.sc_stream, removeStreamOutput:self.sc_stream_output type:SCStreamOutputTypeScreen error:&mut error as *mut _];
            if error != nil {
                let message: id = msg_send![error, localizedDescription];
                log::error!("failed to add stream  output {message:?}");
            }

            let handler = ConcreteBlock::new(move |error: id| {
                if error != nil {
                    let message: id = msg_send![error, localizedDescription];
                    log::error!("failed to stop screen capture stream {message:?}");
                }
            });
            let block = handler.copy();
            let _: () = msg_send![self.sc_stream, stopCaptureWithCompletionHandler:block];
            let _: () = msg_send![self.sc_stream, release];
            let _: () = msg_send![self.sc_stream_output, release];
        }
    }
}

pub(crate) fn get_sources() -> oneshot::Receiver<Result<Vec<Box<dyn ScreenCaptureSource>>>> {
    unsafe {
        let (mut tx, rx) = oneshot::channel();
        let tx = Rc::new(RefCell::new(Some(tx)));

        let block = ConcreteBlock::new(move |shareable_content: id, error: id| {
            let Some(mut tx) = tx.borrow_mut().take() else {
                return;
            };
            let result = if error == nil {
                let displays: id = msg_send![shareable_content, displays];
                let mut result = Vec::new();
                for i in 0..displays.count() {
                    let display = displays.objectAtIndex(i);
                    let source = MacScreenCaptureSource {
                        sc_display: msg_send![display, retain],
                    };
                    result.push(Box::new(source) as Box<dyn ScreenCaptureSource>);
                }
                Ok(result)
            } else {
                let msg: id = msg_send![error, localizedDescription];
                Err(anyhow!(
                    "Screen share failed: {:?}",
                    NSStringExt::to_str(&msg)
                ))
            };
            tx.send(result).ok();
        });
        let block = block.copy();

        let _: () = msg_send![
            class!(SCShareableContent),
            getShareableContentExcludingDesktopWindows:YES
                                   onScreenWindowsOnly:YES
                                     completionHandler:block];
        rx
    }
}

#[ctor]
unsafe fn build_classes() {
    let mut decl = ClassDecl::new("GPUIStreamDelegate", class!(NSObject)).unwrap();
    unsafe {
        decl.add_method(
            sel!(outputVideoEffectDidStartForStream:),
            output_video_effect_did_start_for_stream as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(outputVideoEffectDidStopForStream:),
            output_video_effect_did_stop_for_stream as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(stream:didStopWithError:),
            stream_did_stop_with_error as extern "C" fn(&Object, Sel, id, id),
        );
        DELEGATE_CLASS = decl.register();

        let mut decl = ClassDecl::new("GPUIStreamOutput", class!(NSObject)).unwrap();
        decl.add_method(
            sel!(stream:didOutputSampleBuffer:ofType:),
            stream_did_output_sample_buffer_of_type
                as extern "C" fn(&Object, Sel, id, id, NSInteger),
        );
        decl.add_ivar::<*mut c_void>(FRAME_CALLBACK_IVAR);

        OUTPUT_CLASS = decl.register();
    }
}

extern "C" fn output_video_effect_did_start_for_stream(_this: &Object, _: Sel, _stream: id) {}

extern "C" fn output_video_effect_did_stop_for_stream(_this: &Object, _: Sel, _stream: id) {}

extern "C" fn stream_did_stop_with_error(_this: &Object, _: Sel, _stream: id, _error: id) {}

extern "C" fn stream_did_output_sample_buffer_of_type(
    this: &Object,
    _: Sel,
    _stream: id,
    sample_buffer: id,
    buffer_type: NSInteger,
) {
    if buffer_type != SCStreamOutputTypeScreen {
        return;
    }

    unsafe {
        let sample_buffer = sample_buffer as CMSampleBufferRef;
        let sample_buffer = CMSampleBuffer::wrap_under_get_rule(sample_buffer);
        if let Some(buffer) = sample_buffer.image_buffer() {
            let callback: Box<Box<dyn Fn(ScreenCaptureFrame)>> =
                Box::from_raw(*this.get_ivar::<*mut c_void>(FRAME_CALLBACK_IVAR) as *mut _);
            callback(ScreenCaptureFrame(buffer));
            mem::forget(callback);
        }
    }
}
