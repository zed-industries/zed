mod bindings;

use std::{slice, str};

use block::ConcreteBlock;
use cocoa::{
    base::{id, nil},
    foundation::{NSArray, NSString, NSUInteger, NSInteger},
};
use gpui::{actions, elements::*, keymap::Binding, Menu, MenuItem};
use log::LevelFilter;
use objc::{class, msg_send, sel, sel_impl, declare::ClassDecl, runtime::{Protocol, Object, Sel}};
use simplelog::SimpleLogger;

use crate::bindings::dispatch_get_main_queue;

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

        unsafe {
            let block = ConcreteBlock::new(move |content: id, error: id| {
                if !error.is_null() {
                    println!("ERROR {}", string_from_objc(msg_send![error, localizedDescription]));
                    return;
                }
                
                let displays: id = msg_send![content, displays];
                
                if let Some(display) = (0..displays.count())
                    .map(|ix| displays.objectAtIndex(ix))
                    .next()
                {
                    
                    let display_id: u32 = msg_send![display, displayID];
                    println!("display id {:?}", display_id);
                    
                    let mut decl = ClassDecl::new("CaptureOutput", class!(NSObject)).unwrap();
                    decl.add_protocol(Protocol::get("SCStreamOutput").unwrap());
                    decl.add_method(sel!(stream:didOutputSampleBuffer:ofType:), sample_output as extern "C" fn(&Object, Sel, id, id, NSInteger));
                    let capture_output_class = decl.register();
                    
                    let output: id = msg_send![capture_output_class, alloc];
                    let output: id = msg_send![output, init];
                    
                    let conforms: bool = msg_send![output, conformsToProtocol: Protocol::get("SCStreamOutput").unwrap()];
                    dbg!(conforms);
                    assert!(conforms, "expect CaptureOutput instance to conform to SCStreamOutput protocol");
                    
                    let excluded_windows: id = msg_send![class!(NSArray), array];
                    let filter: id = msg_send![class!(SCContentFilter), alloc];
                    let filter: id = msg_send![filter, initWithDisplay: display excludingWindows: excluded_windows];
                    let config: id = msg_send![class!(SCStreamConfiguration), alloc];
                    let config: id = msg_send![config, init];
                    // Configure the display content width and height.
                    let _: () = msg_send![config, setWidth: 800];
                    let _: () = msg_send![config, setHeight: 600];
                    let _: () = msg_send![config, setMinimumFrameInterval: bindings::CMTimeMake(1, 60)];
                    let _: () = msg_send![config, setQueueDepth: 5];
                    
                    let stream: id = msg_send![class!(SCStream), alloc];
                    let stream: id = msg_send![stream, initWithFilter: filter configuration: config delegate: nil];
                    let error: id = nil;
                    // let queue = dispatch_queue_create(ptr::null(), ptr::null_mut());
                    
                    let _: () = msg_send![stream, addStreamOutput: output type: 0 sampleHandlerQueue: dispatch_get_main_queue() error: &error];
                    
                    let start_capture_completion = ConcreteBlock::new(move |error: id| {
                        if !error.is_null() {
                            println!("error starting capture... error? {}", string_from_objc(msg_send![error, localizedDescription]));
                            return;
                        }
                        
                        println!("starting capture");
                    });
                    
                    assert!(!stream.is_null());
                    let _: () = msg_send![stream, startCaptureWithCompletionHandler: start_capture_completion];
                }
            });

            let _: id = msg_send![
                class!(SCShareableContent),
                getShareableContentWithCompletionHandler: block
            ];
        }

        // cx.add_window(Default::default(), |_| ScreenCaptureView);
    });
}

struct ScreenCaptureView;

impl gpui::Entity for ScreenCaptureView {
    type Event = ();
}

impl gpui::View for ScreenCaptureView {
    fn ui_name() -> &'static str {
        "View"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<Self>) -> gpui::ElementBox {
        Empty::new().boxed()
    }
}

pub unsafe fn string_from_objc(string: id) -> String {
    let len = msg_send![string, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
    let bytes = string.UTF8String() as *const u8;
    str::from_utf8(slice::from_raw_parts(bytes, len))
        .unwrap()
        .to_string()
}

extern "C" fn sample_output(this: &Object, _: Sel, stream: id, buffer: id, kind: NSInteger) {
    println!("sample_output");
}

fn quit(_: &Quit, cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}
