use super::{BoolExt as _, Dispatcher, FontSystem, Window};
use crate::{executor, platform};
use anyhow::Result;
use cocoa::{
    appkit::{NSPasteboard, NSPasteboardTypeString},
    base::{id, nil},
    foundation::NSData,
};
use objc::{class, msg_send, sel, sel_impl};
use std::{ffi::c_void, rc::Rc, sync::Arc};

pub struct App {
    dispatcher: Arc<Dispatcher>,
    fonts: Arc<FontSystem>,
}

impl App {
    pub fn new() -> Self {
        Self {
            dispatcher: Arc::new(Dispatcher),
            fonts: Arc::new(FontSystem::new()),
        }
    }
}

impl platform::App for App {
    fn dispatcher(&self) -> Arc<dyn platform::Dispatcher> {
        self.dispatcher.clone()
    }

    fn activate(&self, ignoring_other_apps: bool) {
        unsafe {
            let app: id = msg_send![class!(NSApplication), sharedApplication];
            let _: () = msg_send![app, activateIgnoringOtherApps: ignoring_other_apps.to_objc()];
        }
    }

    fn open_window(
        &self,
        options: platform::WindowOptions,
        executor: Rc<executor::Foreground>,
    ) -> Result<Box<dyn platform::Window>> {
        Ok(Box::new(Window::open(options, executor, self.fonts())?))
    }

    fn fonts(&self) -> Arc<dyn platform::FontSystem> {
        self.fonts.clone()
    }

    fn copy(&self, text: &str) {
        unsafe {
            let data = NSData::dataWithBytes_length_(
                nil,
                text.as_ptr() as *const c_void,
                text.len() as u64,
            );
            let pasteboard = NSPasteboard::generalPasteboard(nil);
            pasteboard.clearContents();
            pasteboard.setData_forType(data, NSPasteboardTypeString);
        }
    }
}
