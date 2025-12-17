use super::ns_string;
use crate::{
    DevicePixels, ForegroundExecutor, SharedString, SourceMetadata,
    platform::{ScreenCaptureFrame, ScreenCaptureSource, ScreenCaptureStream},
    size,
};
use anyhow::{Result, anyhow};
use block::ConcreteBlock;
use cocoa::{
    base::{NO, YES, id, nil},
    foundation::NSArray,
};
use collections::HashMap;
use core_foundation::base::TCFType;
use core_graphics::{
    base::CGFloat,
    color_space::CGColorSpace,
    display::{
        CGDirectDisplayID, CGDisplayCopyDisplayMode, CGDisplayModeGetPixelHeight,
        CGDisplayModeGetPixelWidth, CGDisplayModeRelease,
    },
    image::CGImage,
};
use core_video::pixel_buffer::CVPixelBuffer;
use ctor::ctor;
use foreign_types::ForeignType;
use futures::channel::oneshot;
use image::{ImageBuffer, Rgba, RgbaImage};
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
    meta: Option<ScreenMeta>,
}

pub struct MacScreenCaptureStream {
    sc_stream: id,
    sc_stream_output: id,
    meta: SourceMetadata,
}

static mut DELEGATE_CLASS: *const Class = ptr::null();
static mut OUTPUT_CLASS: *const Class = ptr::null();
const FRAME_CALLBACK_IVAR: &str = "frame_callback";

#[allow(non_upper_case_globals)]
const SCStreamOutputTypeScreen: NSInteger = 0;

impl ScreenCaptureSource for MacScreenCaptureSource {
    fn metadata(&self) -> Result<SourceMetadata> {
        let (display_id, size) = unsafe {
            let display_id: CGDirectDisplayID = msg_send![self.sc_display, displayID];
            let display_mode_ref = CGDisplayCopyDisplayMode(display_id);
            let width = CGDisplayModeGetPixelWidth(display_mode_ref);
            let height = CGDisplayModeGetPixelHeight(display_mode_ref);
            CGDisplayModeRelease(display_mode_ref);

            (
                display_id,
                size(DevicePixels(width as i32), DevicePixels(height as i32)),
            )
        };
        let (label, is_main) = self
            .meta
            .clone()
            .map(|meta| (meta.label, meta.is_main))
            .unzip();

        Ok(SourceMetadata {
            id: display_id as u64,
            label,
            is_main,
            resolution: size,
        })
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

            let meta = self.metadata().unwrap();
            let _: id = msg_send![configuration, setWidth: meta.resolution.width.0 as i64];
            let _: id = msg_send![configuration, setHeight: meta.resolution.height.0 as i64];
            let stream: id = msg_send![stream, initWithFilter:filter configuration:configuration delegate:delegate];

            // Stream contains filter, configuration, and delegate internally so we release them here
            // to prevent a memory leak when steam is dropped
            let _: () = msg_send![filter, release];
            let _: () = msg_send![configuration, release];
            let _: () = msg_send![delegate, release];

            let (mut tx, rx) = oneshot::channel();

            let mut error: id = nil;
            let _: () = msg_send![stream, addStreamOutput:output type:SCStreamOutputTypeScreen sampleHandlerQueue:0 error:&mut error as *mut id];
            if error != nil {
                let message: id = msg_send![error, localizedDescription];
                let _: () = msg_send![stream, release];
                let _: () = msg_send![output, release];
                tx.send(Err(anyhow!("failed to add stream output {message:?}")))
                    .ok();
                return rx;
            }

            let tx = Rc::new(RefCell::new(Some(tx)));
            let handler = ConcreteBlock::new({
                move |error: id| {
                    let result = if error == nil {
                        let stream = MacScreenCaptureStream {
                            meta: meta.clone(),
                            sc_stream: stream,
                            sc_stream_output: output,
                        };
                        Ok(Box::new(stream) as Box<dyn ScreenCaptureStream>)
                    } else {
                        let _: () = msg_send![stream, release];
                        let _: () = msg_send![output, release];
                        let message: id = msg_send![error, localizedDescription];
                        Err(anyhow!("failed to start screen capture stream {message:?}"))
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

impl ScreenCaptureStream for MacScreenCaptureStream {
    fn metadata(&self) -> Result<SourceMetadata> {
        Ok(self.meta.clone())
    }
}

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

#[derive(Clone)]
struct ScreenMeta {
    label: SharedString,
    // Is this the screen with menu bar?
    is_main: bool,
}

unsafe fn screen_id_to_human_label() -> HashMap<CGDirectDisplayID, ScreenMeta> {
    let screens: id = msg_send![class!(NSScreen), screens];
    let count: usize = msg_send![screens, count];
    let mut map = HashMap::default();
    let screen_number_key = unsafe { ns_string("NSScreenNumber") };
    for i in 0..count {
        let screen: id = msg_send![screens, objectAtIndex: i];
        let device_desc: id = msg_send![screen, deviceDescription];
        if device_desc == nil {
            continue;
        }

        let nsnumber: id = msg_send![device_desc, objectForKey: screen_number_key];
        if nsnumber == nil {
            continue;
        }

        let screen_id: u32 = msg_send![nsnumber, unsignedIntValue];

        let name: id = msg_send![screen, localizedName];
        if name != nil {
            let cstr: *const std::os::raw::c_char = msg_send![name, UTF8String];
            let rust_str = unsafe {
                std::ffi::CStr::from_ptr(cstr)
                    .to_string_lossy()
                    .into_owned()
            };
            map.insert(
                screen_id,
                ScreenMeta {
                    label: rust_str.into(),
                    is_main: i == 0,
                },
            );
        }
    }
    map
}

pub(crate) fn get_sources() -> oneshot::Receiver<Result<Vec<Rc<dyn ScreenCaptureSource>>>> {
    unsafe {
        let (mut tx, rx) = oneshot::channel();
        let tx = Rc::new(RefCell::new(Some(tx)));
        let screen_id_to_label = screen_id_to_human_label();
        let block = ConcreteBlock::new(move |shareable_content: id, error: id| {
            let Some(mut tx) = tx.borrow_mut().take() else {
                return;
            };

            let result = if error == nil {
                let displays: id = msg_send![shareable_content, displays];
                let mut result = Vec::new();
                for i in 0..displays.count() {
                    let display = displays.objectAtIndex(i);
                    let id: CGDirectDisplayID = msg_send![display, displayID];
                    let meta = screen_id_to_label.get(&id).cloned();
                    let source = MacScreenCaptureSource {
                        sc_display: msg_send![display, retain],
                        meta,
                    };
                    result.push(Rc::new(source) as Rc<dyn ScreenCaptureSource>);
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

/// Captures a single screenshot of a specific window by its CGWindowID.
///
/// This uses ScreenCaptureKit's `initWithDesktopIndependentWindow:` API which can
/// capture windows even when they are positioned off-screen (e.g., at -10000, -10000).
///
/// # Arguments
/// * `window_id` - The CGWindowID (NSWindow's windowNumber) of the window to capture
///
/// # Returns
/// An `RgbaImage` containing the captured window contents, or an error if capture failed.
pub fn capture_window_screenshot(window_id: u32) -> oneshot::Receiver<Result<RgbaImage>> {
    let (tx, rx) = oneshot::channel();
    let tx = Rc::new(RefCell::new(Some(tx)));

    unsafe {
        log::info!(
            "capture_window_screenshot: looking for window_id={}",
            window_id
        );
        let content_handler = ConcreteBlock::new(move |shareable_content: id, error: id| {
            log::info!("capture_window_screenshot: content handler called");
            if error != nil {
                if let Some(sender) = tx.borrow_mut().take() {
                    let msg: id = msg_send![error, localizedDescription];
                    sender
                        .send(Err(anyhow!(
                            "Failed to get shareable content: {:?}",
                            NSStringExt::to_str(&msg)
                        )))
                        .ok();
                }
                return;
            }

            let windows: id = msg_send![shareable_content, windows];
            let count: usize = msg_send![windows, count];

            let mut target_window: id = nil;
            log::info!(
                "capture_window_screenshot: searching {} windows for window_id={}",
                count,
                window_id
            );
            for i in 0..count {
                let window: id = msg_send![windows, objectAtIndex: i];
                let wid: u32 = msg_send![window, windowID];
                if wid == window_id {
                    log::info!(
                        "capture_window_screenshot: found matching window at index {}",
                        i
                    );
                    target_window = window;
                    break;
                }
            }

            if target_window == nil {
                if let Some(sender) = tx.borrow_mut().take() {
                    sender
                        .send(Err(anyhow!(
                            "Window with ID {} not found in shareable content",
                            window_id
                        )))
                        .ok();
                }
                return;
            }

            log::info!("capture_window_screenshot: calling capture_window_frame");
            capture_window_frame(target_window, &tx);
        });
        let content_handler = content_handler.copy();

        let _: () = msg_send![
            class!(SCShareableContent),
            getShareableContentExcludingDesktopWindows:NO
                                   onScreenWindowsOnly:NO
                                     completionHandler:content_handler
        ];
    }

    rx
}

unsafe fn capture_window_frame(
    sc_window: id,
    tx: &Rc<RefCell<Option<oneshot::Sender<Result<RgbaImage>>>>>,
) {
    log::info!("capture_window_frame: creating filter for window");
    let filter: id = msg_send![class!(SCContentFilter), alloc];
    let filter: id = msg_send![filter, initWithDesktopIndependentWindow: sc_window];
    log::info!("capture_window_frame: filter created: {:?}", filter);

    let configuration: id = msg_send![class!(SCStreamConfiguration), alloc];
    let configuration: id = msg_send![configuration, init];

    let frame: cocoa::foundation::NSRect = msg_send![sc_window, frame];
    let width = frame.size.width as i64;
    let height = frame.size.height as i64;
    log::info!("capture_window_frame: window frame {}x{}", width, height);

    if width <= 0 || height <= 0 {
        if let Some(tx) = tx.borrow_mut().take() {
            tx.send(Err(anyhow!(
                "Window has invalid dimensions: {}x{}",
                width,
                height
            )))
            .ok();
        }
        return;
    }

    let _: () = msg_send![configuration, setWidth: width];
    let _: () = msg_send![configuration, setHeight: height];
    let _: () = msg_send![configuration, setScalesToFit: true];
    let _: () = msg_send![configuration, setPixelFormat: 0x42475241u32]; // 'BGRA'
    let _: () = msg_send![configuration, setShowsCursor: false];
    let _: () = msg_send![configuration, setCapturesAudio: false];

    let tx_for_capture = tx.clone();
    // The completion handler receives (CGImageRef, NSError*), not CMSampleBuffer
    let capture_handler =
        ConcreteBlock::new(move |cg_image: core_graphics::sys::CGImageRef, error: id| {
            log::info!("Screenshot capture handler called");

            let Some(tx) = tx_for_capture.borrow_mut().take() else {
                log::warn!("Screenshot capture: tx already taken");
                return;
            };

            unsafe {
                if error != nil {
                    let msg: id = msg_send![error, localizedDescription];
                    let error_str = NSStringExt::to_str(&msg);
                    log::error!("Screenshot capture error from API: {:?}", error_str);
                    tx.send(Err(anyhow!("Screenshot capture failed: {:?}", error_str)))
                        .ok();
                    return;
                }

                if cg_image.is_null() {
                    log::error!("Screenshot capture: cg_image is null");
                    tx.send(Err(anyhow!(
                        "Screenshot capture returned null CGImage. \
                         This may mean Screen Recording permission is not granted."
                    )))
                    .ok();
                    return;
                }

                log::info!("Screenshot capture: got CGImage, converting...");
                let cg_image = CGImage::from_ptr(cg_image);
                match cg_image_to_rgba_image(&cg_image) {
                    Ok(image) => {
                        log::info!(
                            "Screenshot capture: success! {}x{}",
                            image.width(),
                            image.height()
                        );
                        tx.send(Ok(image)).ok();
                    }
                    Err(e) => {
                        log::error!("Screenshot capture: CGImage conversion failed: {}", e);
                        tx.send(Err(e)).ok();
                    }
                }
            }
        });
    let capture_handler = capture_handler.copy();

    log::info!("Calling SCScreenshotManager captureImageWithFilter...");
    let _: () = msg_send![
        class!(SCScreenshotManager),
        captureImageWithFilter: filter
                 configuration: configuration
             completionHandler: capture_handler
    ];
    log::info!("SCScreenshotManager captureImageWithFilter called");
}

/// Converts a CGImage to an RgbaImage.
fn cg_image_to_rgba_image(cg_image: &CGImage) -> Result<RgbaImage> {
    let width = cg_image.width();
    let height = cg_image.height();

    if width == 0 || height == 0 {
        return Err(anyhow!("CGImage has zero dimensions: {}x{}", width, height));
    }

    // Create a bitmap context to draw the CGImage into
    let color_space = CGColorSpace::create_device_rgb();
    let bytes_per_row = width * 4;
    let mut pixel_data: Vec<u8> = vec![0; height * bytes_per_row];

    let context = core_graphics::context::CGContext::create_bitmap_context(
        Some(pixel_data.as_mut_ptr() as *mut c_void),
        width,
        height,
        8,             // bits per component
        bytes_per_row, // bytes per row
        &color_space,
        core_graphics::base::kCGImageAlphaPremultipliedLast // RGBA
            | core_graphics::base::kCGBitmapByteOrder32Big,
    );

    // Draw the image into the context
    let rect = core_graphics::geometry::CGRect::new(
        &core_graphics::geometry::CGPoint::new(0.0, 0.0),
        &core_graphics::geometry::CGSize::new(width as CGFloat, height as CGFloat),
    );
    context.draw_image(rect, cg_image);

    // The pixel data is now in RGBA format
    ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, pixel_data)
        .ok_or_else(|| anyhow!("Failed to create RgbaImage from CGImage pixel data"))
}

/// Converts a CVPixelBuffer (in BGRA format) to an RgbaImage.
///
/// This function locks the pixel buffer, reads the raw pixel data,
/// converts from BGRA to RGBA format, and returns an image::RgbaImage.
pub fn cv_pixel_buffer_to_rgba_image(pixel_buffer: &CVPixelBuffer) -> Result<RgbaImage> {
    use core_video::r#return::kCVReturnSuccess;

    unsafe {
        if pixel_buffer.lock_base_address(0) != kCVReturnSuccess {
            return Err(anyhow!("Failed to lock pixel buffer base address"));
        }

        let width = pixel_buffer.get_width();
        let height = pixel_buffer.get_height();
        let bytes_per_row = pixel_buffer.get_bytes_per_row();
        let base_address = pixel_buffer.get_base_address();

        if base_address.is_null() {
            pixel_buffer.unlock_base_address(0);
            return Err(anyhow!("Pixel buffer base address is null"));
        }

        let mut rgba_data = Vec::with_capacity(width * height * 4);

        for y in 0..height {
            let row_start = base_address.add(y * bytes_per_row) as *const u8;
            for x in 0..width {
                let pixel = row_start.add(x * 4);
                let b = *pixel;
                let g = *pixel.add(1);
                let r = *pixel.add(2);
                let a = *pixel.add(3);
                rgba_data.push(r);
                rgba_data.push(g);
                rgba_data.push(b);
                rgba_data.push(a);
            }
        }

        pixel_buffer.unlock_base_address(0);

        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width as u32, height as u32, rgba_data)
            .ok_or_else(|| anyhow!("Failed to create RgbaImage from pixel data"))
    }
}

/// Converts a ScreenCaptureFrame to an RgbaImage.
///
/// This is useful for converting frames received from continuous screen capture streams.
pub fn screen_capture_frame_to_rgba_image(frame: &ScreenCaptureFrame) -> Result<RgbaImage> {
    unsafe {
        let pixel_buffer =
            CVPixelBuffer::wrap_under_get_rule(frame.0.as_concrete_TypeRef() as *mut _);
        cv_pixel_buffer_to_rgba_image(&pixel_buffer)
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
