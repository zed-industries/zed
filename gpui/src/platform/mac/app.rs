use super::{BoolExt as _, Dispatcher, Window};
use crate::{executor, platform};
use anyhow::Result;
use cocoa::base::id;
use objc::{class, msg_send, sel, sel_impl};
use std::{rc::Rc, sync::Arc};

pub struct App {
    dispatcher: Arc<Dispatcher>,
}

impl App {
    pub fn new() -> Self {
        Self {
            dispatcher: Arc::new(Dispatcher),
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
        Ok(Box::new(Window::open(options, executor)?))
    }
}
