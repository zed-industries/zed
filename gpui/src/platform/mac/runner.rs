use crate::{keymap::Keystroke, platform::Event, Menu, MenuItem};
use cocoa::{
    appkit::{
        NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        NSEventModifierFlags, NSMenu, NSMenuItem, NSWindow,
    },
    base::{id, nil, selector},
    foundation::{NSArray, NSAutoreleasePool, NSInteger, NSString},
};
use ctor::ctor;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use std::{
    ffi::CStr,
    os::raw::{c_char, c_void},
    path::PathBuf,
    ptr,
};

const RUNNER_IVAR: &'static str = "runner";
static mut APP_CLASS: *const Class = ptr::null();
static mut APP_DELEGATE_CLASS: *const Class = ptr::null();

#[ctor]
unsafe fn build_classes() {
    APP_CLASS = {
        let mut decl = ClassDecl::new("GPUIApplication", class!(NSApplication)).unwrap();
        decl.add_ivar::<*mut c_void>(RUNNER_IVAR);
        decl.add_method(
            sel!(sendEvent:),
            send_event as extern "C" fn(&mut Object, Sel, id),
        );
        decl.register()
    };

    APP_DELEGATE_CLASS = {
        let mut decl = ClassDecl::new("GPUIApplicationDelegate", class!(NSResponder)).unwrap();
        decl.add_ivar::<*mut c_void>(RUNNER_IVAR);
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

#[derive(Default)]
pub struct Runner {
    finish_launching_callback: Option<Box<dyn FnOnce()>>,
    become_active_callback: Option<Box<dyn FnMut()>>,
    resign_active_callback: Option<Box<dyn FnMut()>>,
    event_callback: Option<Box<dyn FnMut(Event) -> bool>>,
    open_files_callback: Option<Box<dyn FnMut(Vec<PathBuf>)>>,
    menu_command_callback: Option<Box<dyn FnMut(&str)>>,
    menu_item_actions: Vec<String>,
}

impl Runner {
    pub fn new() -> Self {
        Default::default()
    }

    unsafe fn create_menu_bar(&mut self, menus: &[Menu]) -> id {
        let menu_bar = NSMenu::new(nil).autorelease();
        self.menu_item_actions.clear();

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

                        let tag = self.menu_item_actions.len() as NSInteger;
                        let _: () = msg_send![item, setTag: tag];
                        self.menu_item_actions.push(action.to_string());
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

impl crate::platform::Runner for Runner {
    fn on_finish_launching<F: 'static + FnOnce()>(mut self, callback: F) -> Self {
        self.finish_launching_callback = Some(Box::new(callback));
        self
    }

    fn on_menu_command<F: 'static + FnMut(&str)>(mut self, callback: F) -> Self {
        self.menu_command_callback = Some(Box::new(callback));
        self
    }

    fn on_become_active<F: 'static + FnMut()>(mut self, callback: F) -> Self {
        log::info!("become active");
        self.become_active_callback = Some(Box::new(callback));
        self
    }

    fn on_resign_active<F: 'static + FnMut()>(mut self, callback: F) -> Self {
        self.resign_active_callback = Some(Box::new(callback));
        self
    }

    fn on_event<F: 'static + FnMut(Event) -> bool>(mut self, callback: F) -> Self {
        self.event_callback = Some(Box::new(callback));
        self
    }

    fn on_open_files<F: 'static + FnMut(Vec<PathBuf>)>(mut self, callback: F) -> Self {
        self.open_files_callback = Some(Box::new(callback));
        self
    }

    fn set_menus(mut self, menus: &[Menu]) -> Self {
        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            app.setMainMenu_(self.create_menu_bar(menus));
        }
        self
    }

    fn run(self) {
        unsafe {
            let self_ptr = Box::into_raw(Box::new(self));

            let pool = NSAutoreleasePool::new(nil);
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let app_delegate: id = msg_send![APP_DELEGATE_CLASS, new];

            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
            (*app).set_ivar(RUNNER_IVAR, self_ptr as *mut c_void);
            (*app_delegate).set_ivar(RUNNER_IVAR, self_ptr as *mut c_void);
            app.setDelegate_(app_delegate);
            app.run();
            pool.drain();

            // The Runner is done running when we get here, so we can reinstantiate the Box and drop it.
            Box::from_raw(self_ptr);
        }
    }
}

unsafe fn get_runner(object: &mut Object) -> &mut Runner {
    let runner_ptr: *mut c_void = *object.get_ivar(RUNNER_IVAR);
    &mut *(runner_ptr as *mut Runner)
}

extern "C" fn send_event(this: &mut Object, _sel: Sel, native_event: id) {
    let event = unsafe { Event::from_native(native_event, None) };

    if let Some(event) = event {
        let runner = unsafe { get_runner(this) };
        if let Some(callback) = runner.event_callback.as_mut() {
            if callback(event) {
                return;
            }
        }
    }

    unsafe {
        let _: () = msg_send![super(this, class!(NSApplication)), sendEvent: native_event];
    }
}

extern "C" fn did_finish_launching(this: &mut Object, _: Sel, _: id) {
    let runner = unsafe { get_runner(this) };
    if let Some(callback) = runner.finish_launching_callback.take() {
        callback();
    }
}

extern "C" fn did_become_active(this: &mut Object, _: Sel, _: id) {
    let runner = unsafe { get_runner(this) };
    if let Some(callback) = runner.become_active_callback.as_mut() {
        callback();
    }
}

extern "C" fn did_resign_active(this: &mut Object, _: Sel, _: id) {
    let runner = unsafe { get_runner(this) };
    if let Some(callback) = runner.resign_active_callback.as_mut() {
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
    let runner = unsafe { get_runner(this) };
    if let Some(callback) = runner.open_files_callback.as_mut() {
        callback(paths);
    }
}

extern "C" fn handle_menu_item(this: &mut Object, _: Sel, item: id) {
    unsafe {
        let runner = get_runner(this);
        if let Some(callback) = runner.menu_command_callback.as_mut() {
            let tag: NSInteger = msg_send![item, tag];
            let index = tag as usize;
            if let Some(action) = runner.menu_item_actions.get(index) {
                callback(&action);
            }
        }
    }
}

unsafe fn ns_string(string: &str) -> id {
    NSString::alloc(nil).init_str(string).autorelease()
}
