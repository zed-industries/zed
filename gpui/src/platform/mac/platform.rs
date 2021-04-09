use super::{BoolExt as _, Dispatcher, FontSystem, Window};
use crate::{executor, keymap::Keystroke, platform, Event, Menu, MenuItem};
use anyhow::Result;
use cocoa::{
    appkit::{
        NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        NSEventModifierFlags, NSMenu, NSMenuItem, NSModalResponse, NSOpenPanel, NSPasteboard,
        NSPasteboardTypeString, NSWindow,
    },
    base::{id, nil, selector},
    foundation::{NSArray, NSAutoreleasePool, NSData, NSInteger, NSString, NSURL},
};
use ctor::ctor;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use ptr::null_mut;
use std::{
    cell::RefCell,
    ffi::{c_void, CStr},
    os::raw::c_char,
    path::PathBuf,
    ptr,
    rc::Rc,
    sync::Arc,
};

const MAC_PLATFORM_IVAR: &'static str = "runner";
static mut APP_CLASS: *const Class = ptr::null();
static mut APP_DELEGATE_CLASS: *const Class = ptr::null();

#[ctor]
unsafe fn build_classes() {
    APP_CLASS = {
        let mut decl = ClassDecl::new("GPUIApplication", class!(NSApplication)).unwrap();
        decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
        decl.add_method(
            sel!(sendEvent:),
            send_event as extern "C" fn(&mut Object, Sel, id),
        );
        decl.register()
    };

    APP_DELEGATE_CLASS = {
        let mut decl = ClassDecl::new("GPUIApplicationDelegate", class!(NSResponder)).unwrap();
        decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            did_finish_launching as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(applicationDidBecomeActive:),
            did_become_active as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(applicationDidResignActive:),
            did_resign_active as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(handleGPUIMenuItem:),
            handle_menu_item as extern "C" fn(&mut Object, Sel, id),
        );
        decl.add_method(
            sel!(application:openFiles:),
            open_files as extern "C" fn(&mut Object, Sel, id, id),
        );
        decl.register()
    }
}

pub struct MacPlatform {
    dispatcher: Arc<Dispatcher>,
    fonts: Arc<FontSystem>,
    callbacks: RefCell<Callbacks>,
    menu_item_actions: RefCell<Vec<String>>,
}

#[derive(Default)]
struct Callbacks {
    become_active: Option<Box<dyn FnMut()>>,
    resign_active: Option<Box<dyn FnMut()>>,
    event: Option<Box<dyn FnMut(crate::Event) -> bool>>,
    menu_command: Option<Box<dyn FnMut(&str)>>,
    open_files: Option<Box<dyn FnMut(Vec<PathBuf>)>>,
    finish_launching: Option<Box<dyn FnOnce() -> ()>>,
}

impl MacPlatform {
    pub fn new() -> Arc<dyn platform::Platform> {
        let result = Arc::new(Self {
            dispatcher: Arc::new(Dispatcher),
            fonts: Arc::new(FontSystem::new()),
            callbacks: Default::default(),
            menu_item_actions: Default::default(),
        });

        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let app_delegate: id = msg_send![APP_DELEGATE_CLASS, new];
            let self_ptr = result.as_ref() as *const Self as *const c_void;
            app.setDelegate_(app_delegate);
            (*app).set_ivar(MAC_PLATFORM_IVAR, self_ptr);
            (*app_delegate).set_ivar(MAC_PLATFORM_IVAR, self_ptr);
        }

        result
    }

    pub fn run() {
        unsafe {
            let pool = NSAutoreleasePool::new(nil);
            let app: id = msg_send![APP_CLASS, sharedApplication];

            app.run();
            pool.drain();
            (*app).set_ivar(MAC_PLATFORM_IVAR, null_mut::<c_void>());
            (*app.delegate()).set_ivar(MAC_PLATFORM_IVAR, null_mut::<c_void>());
        }
    }

    unsafe fn create_menu_bar(&self, menus: &[Menu]) -> id {
        let menu_bar = NSMenu::new(nil).autorelease();
        let mut menu_item_actions = self.menu_item_actions.borrow_mut();
        menu_item_actions.clear();

        for menu_config in menus {
            let menu_bar_item = NSMenuItem::new(nil).autorelease();
            let menu = NSMenu::new(nil).autorelease();

            menu.setTitle_(ns_string(menu_config.name));

            for item_config in menu_config.items {
                let item;

                match item_config {
                    MenuItem::Separator => {
                        item = NSMenuItem::separatorItem(nil);
                    }
                    MenuItem::Action {
                        name,
                        keystroke,
                        action,
                    } => {
                        if let Some(keystroke) = keystroke {
                            let keystroke = Keystroke::parse(keystroke).unwrap_or_else(|err| {
                                panic!(
                                    "Invalid keystroke for menu item {}:{} - {:?}",
                                    menu_config.name, name, err
                                )
                            });

                            let mut mask = NSEventModifierFlags::empty();
                            for (modifier, flag) in &[
                                (keystroke.cmd, NSEventModifierFlags::NSCommandKeyMask),
                                (keystroke.ctrl, NSEventModifierFlags::NSControlKeyMask),
                                (keystroke.alt, NSEventModifierFlags::NSAlternateKeyMask),
                            ] {
                                if *modifier {
                                    mask |= *flag;
                                }
                            }

                            item = NSMenuItem::alloc(nil)
                                .initWithTitle_action_keyEquivalent_(
                                    ns_string(name),
                                    selector("handleGPUIMenuItem:"),
                                    ns_string(&keystroke.key),
                                )
                                .autorelease();
                            item.setKeyEquivalentModifierMask_(mask);
                        } else {
                            item = NSMenuItem::alloc(nil)
                                .initWithTitle_action_keyEquivalent_(
                                    ns_string(name),
                                    selector("handleGPUIMenuItem:"),
                                    ns_string(""),
                                )
                                .autorelease();
                        }

                        let tag = menu_item_actions.len() as NSInteger;
                        let _: () = msg_send![item, setTag: tag];
                        menu_item_actions.push(action.to_string());
                    }
                }

                menu.addItem_(item);
            }

            menu_bar_item.setSubmenu_(menu);
            menu_bar.addItem_(menu_bar_item);
        }

        menu_bar
    }
}

impl platform::Platform for MacPlatform {
    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().become_active = Some(callback);
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().resign_active = Some(callback);
    }

    fn on_event(&self, callback: Box<dyn FnMut(crate::Event) -> bool>) {
        self.callbacks.borrow_mut().event = Some(callback);
    }

    fn on_menu_command(&self, callback: Box<dyn FnMut(&str)>) {
        self.callbacks.borrow_mut().menu_command = Some(callback);
    }

    fn on_open_files(&self, callback: Box<dyn FnMut(Vec<PathBuf>)>) {
        self.callbacks.borrow_mut().open_files = Some(callback);
    }

    fn on_finish_launching(&self, callback: Box<dyn FnOnce() -> ()>) {
        self.callbacks.borrow_mut().finish_launching = Some(callback);
    }

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

    fn set_menus(&self, menus: &[Menu]) {
        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            app.setMainMenu_(self.create_menu_bar(menus));
        }
    }
}

unsafe fn get_platform(object: &mut Object) -> &MacPlatform {
    let platform_ptr: *mut c_void = *object.get_ivar(MAC_PLATFORM_IVAR);
    assert!(!platform_ptr.is_null());
    &*(platform_ptr as *const MacPlatform)
}

extern "C" fn send_event(this: &mut Object, _sel: Sel, native_event: id) {
    unsafe {
        if let Some(event) = Event::from_native(native_event, None) {
            let platform = get_platform(this);
            if let Some(callback) = platform.callbacks.borrow_mut().event.as_mut() {
                if callback(event) {
                    return;
                }
            }
        }

        msg_send![super(this, class!(NSApplication)), sendEvent: native_event]
    }
}

extern "C" fn did_finish_launching(this: &mut Object, _: Sel, _: id) {
    unsafe {
        let app: id = msg_send![APP_CLASS, sharedApplication];
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        let platform = get_platform(this);
        if let Some(callback) = platform.callbacks.borrow_mut().finish_launching.take() {
            callback();
        }
    }
}

extern "C" fn did_become_active(this: &mut Object, _: Sel, _: id) {
    let platform = unsafe { get_platform(this) };
    if let Some(callback) = platform.callbacks.borrow_mut().become_active.as_mut() {
        callback();
    }
}

extern "C" fn did_resign_active(this: &mut Object, _: Sel, _: id) {
    let platform = unsafe { get_platform(this) };
    if let Some(callback) = platform.callbacks.borrow_mut().resign_active.as_mut() {
        callback();
    }
}

extern "C" fn open_files(this: &mut Object, _: Sel, _: id, paths: id) {
    let paths = unsafe {
        (0..paths.count())
            .into_iter()
            .filter_map(|i| {
                let path = paths.objectAtIndex(i);
                match CStr::from_ptr(path.UTF8String() as *mut c_char).to_str() {
                    Ok(string) => Some(PathBuf::from(string)),
                    Err(err) => {
                        log::error!("error converting path to string: {}", err);
                        None
                    }
                }
            })
            .collect::<Vec<_>>()
    };
    let platform = unsafe { get_platform(this) };
    if let Some(callback) = platform.callbacks.borrow_mut().open_files.as_mut() {
        callback(paths);
    }
}

extern "C" fn handle_menu_item(this: &mut Object, _: Sel, item: id) {
    unsafe {
        let platform = get_platform(this);
        if let Some(callback) = platform.callbacks.borrow_mut().menu_command.as_mut() {
            let tag: NSInteger = msg_send![item, tag];
            let index = tag as usize;
            if let Some(action) = platform.menu_item_actions.borrow().get(index) {
                callback(&action);
            }
        }
    }
}

unsafe fn ns_string(string: &str) -> id {
    NSString::alloc(nil).init_str(string).autorelease()
}
