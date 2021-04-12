use super::{BoolExt as _, Dispatcher, FontSystem, Window};
use crate::{executor, platform};
use anyhow::Result;
use cocoa::{
    appkit::{NSApplication, NSModalResponse, NSOpenPanel, NSPasteboard, NSPasteboardTypeString},
    base::nil,
    foundation::{NSArray, NSData, NSString, NSURL},
};
use objc::{msg_send, sel, sel_impl};
use std::{ffi::c_void, path::PathBuf, rc::Rc, sync::Arc};

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

    fn prompt_for_paths(
        &self,
        options: platform::PathPromptOptions,
    ) -> Option<Vec<std::path::PathBuf>> {
        unsafe {
            let panel = NSOpenPanel::openPanel(nil);
            panel.setCanChooseDirectories_(options.directories.to_objc());
            panel.setCanChooseFiles_(options.files.to_objc());
            panel.setAllowsMultipleSelection_(options.multiple.to_objc());
            panel.setResolvesAliases_(false.to_objc());
            let response = panel.runModal();
            if response == NSModalResponse::NSModalResponseOk {
                let mut result = Vec::new();
                let urls = panel.URLs();
                for i in 0..urls.count() {
                    let url = urls.objectAtIndex(i);
                    let string = url.absoluteString();
                    let string = std::ffi::CStr::from_ptr(string.UTF8String())
                        .to_string_lossy()
                        .to_string();
                    if let Some(path) = string.strip_prefix("file://") {
                        result.push(PathBuf::from(path));
                    }
                }
                Some(result)
            } else {
                None
            }
        }
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
