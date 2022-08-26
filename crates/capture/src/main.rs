use std::{slice, str};

use block::ConcreteBlock;
use cocoa::{
    base::id,
    foundation::{NSString, NSUInteger},
};
use gpui::{actions, elements::*, keymap::Binding, Menu, MenuItem};
use log::LevelFilter;
use objc::{class, msg_send, sel, sel_impl};
use simplelog::SimpleLogger;

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
                println!(
                    "got response with shareable content {:?} {:?} {:?}",
                    content,
                    error,
                    string_from_objc(msg_send![error, localizedDescription]),
                )
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

fn quit(_: &Quit, cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}
