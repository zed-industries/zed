use super::{BoolExt as _, Dispatcher, FontSystem, Window};
use crate::{executor, platform};
use anyhow::Result;
use cocoa::{appkit::NSApplication, base::nil};
use objc::{msg_send, sel, sel_impl};
use std::{rc::Rc, sync::Arc};

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
            let app = NSApplication::sharedApplication(nil);
            app.activateIgnoringOtherApps_(ignoring_other_apps.to_objc());
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

    fn quit(&self) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let _: () = msg_send![app, terminate: nil];
        }
    }
}
