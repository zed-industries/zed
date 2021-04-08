use crate::platform::Event;
use cocoa::{
    appkit::{
        NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular, NSMenu,
        NSMenuItem, NSWindow,
    },
    base::{id, nil, selector},
    foundation::{NSArray, NSAutoreleasePool, NSString},
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
}

impl Runner {
    pub fn new() -> Self {
        Default::default()
    }
}

impl crate::platform::Runner for Runner {
    fn on_finish_launching<F: 'static + FnOnce()>(mut self, callback: F) -> Self {
        self.finish_launching_callback = Some(Box::new(callback));
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

    fn run(self) {
        unsafe {
            let self_ptr = Box::into_raw(Box::new(self));

            let pool = NSAutoreleasePool::new(nil);
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let app_delegate: id = msg_send![APP_DELEGATE_CLASS, new];

            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
            (*app).set_ivar(RUNNER_IVAR, self_ptr as *mut c_void);
            (*app_delegate).set_ivar(RUNNER_IVAR, self_ptr as *mut c_void);
            app.setMainMenu_(create_menu_bar());
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

unsafe fn create_menu_bar() -> id {
    let menu_bar = NSMenu::new(nil).autorelease();

    // App menu
    let app_menu_item = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(
            ns_string("Application"),
            Sel::from_ptr(ptr::null()),
            ns_string(""),
        )
        .autorelease();
    let quit_item = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(
            ns_string("Quit"),
            selector("terminate:"),
            ns_string("q\0"),
        )
        .autorelease();
    let app_menu = NSMenu::new(nil).autorelease();
    app_menu.addItem_(quit_item);
    app_menu_item.setSubmenu_(app_menu);
    menu_bar.addItem_(app_menu_item);

    // File menu
    let file_menu_item = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(
            ns_string("File"),
            Sel::from_ptr(ptr::null()),
            ns_string(""),
        )
        .autorelease();
    let open_item = NSMenuItem::alloc(nil)
        .initWithTitle_action_keyEquivalent_(
            ns_string("Open"),
            selector("openDocument:"),
            ns_string("o\0"),
        )
        .autorelease();
    let file_menu = NSMenu::new(nil).autorelease();
    file_menu.setTitle_(ns_string("File"));
    file_menu.addItem_(open_item);
    file_menu_item.setSubmenu_(file_menu);
    menu_bar.addItem_(file_menu_item);

    menu_bar
}

unsafe fn ns_string(string: &str) -> id {
    NSString::alloc(nil).init_str(string).autorelease()
}
