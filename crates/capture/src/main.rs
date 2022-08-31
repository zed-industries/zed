mod bindings;

use crate::bindings::SCStreamOutputType;
use block::ConcreteBlock;
use cocoa::{
    base::{id, nil, YES},
    foundation::{NSArray, NSString, NSUInteger},
};
use core_foundation::{base::TCFType, number::CFNumberRef, string::CFStringRef};
use core_media::{CMSampleBuffer, CMSampleBufferRef};
use futures::StreamExt;
use gpui::{
    actions,
    elements::{Canvas, *},
    keymap::Binding,
    platform::current::Surface,
    Menu, MenuItem, ViewContext,
};
use log::LevelFilter;
use media::core_video::{self, CVImageBuffer};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Object, Sel},
    sel, sel_impl,
};
use parking_lot::Mutex;
use simplelog::SimpleLogger;
use std::{ffi::c_void, ptr, slice, str, sync::Arc};

#[allow(non_upper_case_globals)]
const NSUTF8StringEncoding: NSUInteger = 4;

actions!(capture, [Quit]);

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.platform().activate(true);
        cx.add_global_action(quit);

        cx.add_bindings([Binding::new("cmd-q", Quit, None)]);
        cx.set_menus(vec![Menu {
            name: "Zed",
            items: vec![MenuItem::Action {
                name: "Quit",
                action: Box::new(Quit),
            }],
        }]);

        cx.add_window(Default::default(), |cx| ScreenCaptureView::new(cx));
    });
}

struct ScreenCaptureView {
    image_buffer: Option<core_video::CVImageBuffer>,
}

impl gpui::Entity for ScreenCaptureView {
    type Event = ();
}

impl ScreenCaptureView {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let (image_buffer_tx, mut image_buffer_rx) =
            postage::watch::channel::<Option<CVImageBuffer>>();
        let image_buffer_tx = Arc::new(Mutex::new(image_buffer_tx));

        unsafe {
            let block = ConcreteBlock::new(move |content: id, error: id| {
                if !error.is_null() {
                    println!(
                        "ERROR {}",
                        string_from_objc(msg_send![error, localizedDescription])
                    );
                    return;
                }

                let applications: id = msg_send![content, applications];
                let displays: id = msg_send![content, displays];
                let display: id = displays.objectAtIndex(0);
                let display_width: usize = msg_send![display, width];
                let display_height: usize = msg_send![display, height];

                let mut decl = ClassDecl::new("CaptureOutput", class!(NSObject)).unwrap();
                decl.add_ivar::<*mut c_void>("callback");
                decl.add_method(
                    sel!(stream:didOutputSampleBuffer:ofType:),
                    sample_output as extern "C" fn(&Object, Sel, id, id, SCStreamOutputType),
                );
                let capture_output_class = decl.register();

                let output: id = msg_send![capture_output_class, alloc];
                let output: id = msg_send![output, init];
                let surface_tx = image_buffer_tx.clone();

                let callback = Box::new(move |buffer: CMSampleBufferRef| {
                    let buffer = CMSampleBuffer::wrap_under_get_rule(buffer);
                    let attachments = buffer.attachments();
                    let attachments = attachments.first().expect("no attachments for sample");
                    let string = bindings::SCStreamFrameInfoStatus.0 as CFStringRef;
                    let status = core_foundation::number::CFNumber::wrap_under_get_rule(
                        *attachments.get(string) as CFNumberRef,
                    )
                    .to_i64()
                    .expect("invalid frame info status");

                    if status != bindings::SCFrameStatus_SCFrameStatusComplete {
                        println!("received incomplete frame");
                        return;
                    }

                    let image_buffer = buffer.image_buffer();
                    *surface_tx.lock().borrow_mut() = Some(image_buffer);
                }) as Box<dyn FnMut(CMSampleBufferRef)>;
                let callback = Box::into_raw(Box::new(callback));
                (*output).set_ivar("callback", callback as *mut c_void);

                let filter: id = msg_send![class!(SCContentFilter), alloc];
                let filter: id = msg_send![filter, initWithDisplay: display includingApplications: applications exceptingWindows: nil];
                // let filter: id = msg_send![filter, initWithDesktopIndependentWindow: window];
                let config: id = msg_send![class!(SCStreamConfiguration), alloc];
                let config: id = msg_send![config, init];
                let _: () = msg_send![config, setWidth: display_width * 2];
                let _: () = msg_send![config, setHeight: display_height * 2];
                let _: () = msg_send![config, setMinimumFrameInterval: bindings::CMTimeMake(1, 60)];
                let _: () = msg_send![config, setQueueDepth: 6];
                let _: () = msg_send![config, setShowsCursor: YES];
                let _: () = msg_send![
                    config,
                    setPixelFormat: media::core_video::kCVPixelFormatType_32BGRA
                ];

                let stream: id = msg_send![class!(SCStream), alloc];
                let stream: id = msg_send![stream, initWithFilter: filter configuration: config delegate: output];
                let error: id = nil;
                let queue = bindings::dispatch_queue_create(
                    ptr::null(),
                    bindings::NSObject(ptr::null_mut()),
                );

                let _: () = msg_send![stream,
                    addStreamOutput: output type: bindings::SCStreamOutputType_SCStreamOutputTypeScreen
                    sampleHandlerQueue: queue
                    error: &error
                ];

                let start_capture_completion = ConcreteBlock::new(move |error: id| {
                    if !error.is_null() {
                        println!(
                            "error starting capture... error? {}",
                            string_from_objc(msg_send![error, localizedDescription])
                        );
                        return;
                    }

                    println!("starting capture");
                });

                assert!(!stream.is_null());
                let _: () = msg_send![
                    stream,
                    startCaptureWithCompletionHandler: start_capture_completion
                ];
            });

            let _: id = msg_send![
                class!(SCShareableContent),
                getShareableContentWithCompletionHandler: block
            ];
        }

        cx.spawn_weak(|this, mut cx| async move {
            while let Some(image_buffer) = image_buffer_rx.next().await {
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        this.image_buffer = image_buffer;
                        cx.notify();
                    })
                } else {
                    break;
                }
            }
        })
        .detach();

        Self { image_buffer: None }
    }
}

impl gpui::View for ScreenCaptureView {
    fn ui_name() -> &'static str {
        "View"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<Self>) -> gpui::ElementBox {
        let image_buffer = self.image_buffer.clone();
        Canvas::new(move |bounds, _, cx| {
            if let Some(image_buffer) = image_buffer.clone() {
                cx.scene.push_surface(Surface {
                    bounds,
                    image_buffer,
                });
            }
        })
        .boxed()
    }
}

pub unsafe fn string_from_objc(string: id) -> String {
    if string.is_null() {
        Default::default()
    } else {
        let len = msg_send![string, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
        let bytes = string.UTF8String() as *const u8;
        str::from_utf8(slice::from_raw_parts(bytes, len))
            .unwrap()
            .to_string()
    }
}

extern "C" fn sample_output(
    this: &Object,
    _: Sel,
    _stream: id,
    buffer: id,
    _kind: SCStreamOutputType,
) {
    unsafe {
        let callback = *this.get_ivar::<*mut c_void>("callback");
        let callback = &mut *(callback as *mut Box<dyn FnMut(CMSampleBufferRef)>);
        (*callback)(buffer as CMSampleBufferRef);
    }
}

fn quit(_: &Quit, cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}

mod core_media {
    #![allow(non_snake_case)]

    use crate::core_video::{CVImageBuffer, CVImageBufferRef};
    use core_foundation::{
        array::{CFArray, CFArrayRef},
        base::{CFTypeID, TCFType},
        declare_TCFType,
        dictionary::CFDictionary,
        impl_CFTypeDescription, impl_TCFType,
        string::CFString,
    };
    use std::ffi::c_void;

    #[repr(C)]
    pub struct __CMSampleBuffer(c_void);
    // The ref type must be a pointer to the underlying struct.
    pub type CMSampleBufferRef = *const __CMSampleBuffer;

    declare_TCFType!(CMSampleBuffer, CMSampleBufferRef);
    impl_TCFType!(CMSampleBuffer, CMSampleBufferRef, CMSampleBufferGetTypeID);
    impl_CFTypeDescription!(CMSampleBuffer);

    impl CMSampleBuffer {
        pub fn attachments(&self) -> Vec<CFDictionary<CFString>> {
            unsafe {
                let attachments =
                    CMSampleBufferGetSampleAttachmentsArray(self.as_concrete_TypeRef(), true);
                CFArray::<CFDictionary>::wrap_under_get_rule(attachments)
                    .into_iter()
                    .map(|attachments| {
                        CFDictionary::wrap_under_get_rule(attachments.as_concrete_TypeRef())
                    })
                    .collect()
            }
        }

        pub fn image_buffer(&self) -> CVImageBuffer {
            unsafe {
                CVImageBuffer::wrap_under_get_rule(CMSampleBufferGetImageBuffer(
                    self.as_concrete_TypeRef(),
                ))
            }
        }
    }

    extern "C" {
        fn CMSampleBufferGetTypeID() -> CFTypeID;
        fn CMSampleBufferGetSampleAttachmentsArray(
            buffer: CMSampleBufferRef,
            create_if_necessary: bool,
        ) -> CFArrayRef;
        fn CMSampleBufferGetImageBuffer(buffer: CMSampleBufferRef) -> CVImageBufferRef;
    }
}
