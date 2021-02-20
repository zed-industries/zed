use super::Event;
pub use cocoa::foundation::NSSize;
use cocoa::{
    base::{id, nil},
    foundation::{NSArray, NSAutoreleasePool, NSString},
};
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
};

#[derive(Default)]
pub struct App {
    finish_launching_callback: Option<Box<dyn FnOnce()>>,
    become_active_callback: Option<Box<dyn FnMut()>>,
    resign_active_callback: Option<Box<dyn FnMut()>>,
    event_callback: Option<Box<dyn FnMut(Event) -> bool>>,
    open_files_callback: Option<Box<dyn FnMut(Vec<PathBuf>)>>,
}

const RUST_WRAPPER_IVAR_NAME: &'static str = "rustWrapper";

impl super::App for App {
    fn on_finish_launching<F: 'static + FnOnce()>(mut self, callback: F) -> Self {
        self.finish_launching_callback = Some(Box::new(callback));
        self
    }

    fn on_become_active<F: 'static + FnMut()>(mut self, callback: F) -> Self {
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
            let app: id = msg_send![build_app_class(), sharedApplication];
            (*app).set_ivar(RUST_WRAPPER_IVAR_NAME, self_ptr as *mut c_void);
            let app_delegate: id = msg_send![build_app_delegate_class(), new];
            (*app_delegate).set_ivar(RUST_WRAPPER_IVAR_NAME, self_ptr as *mut c_void);
            let _: () = msg_send![app, setDelegate: app_delegate];
            let _: () = msg_send![app, run];
            let _: () = msg_send![pool, drain];

            // App is done running when we get here, so we can reinstantiate the Box and drop it.
            Box::from_raw(self_ptr);
        }
    }
}

fn build_app_class() -> *const Class {
    unsafe {
        let mut decl = ClassDecl::new("GPUIApplication", class!(NSApplication)).unwrap();
        decl.add_ivar::<*mut c_void>(RUST_WRAPPER_IVAR_NAME);
        decl.add_method(
            sel!(sendEvent:),
            send_event as extern "C" fn(&Object, Sel, id),
        );
        decl.register()
    }
}

fn build_app_delegate_class() -> *const Class {
    unsafe {
        let superclass = class!(NSResponder);
        let mut decl = ClassDecl::new("GPUIApplicationDelegate", superclass).unwrap();
        decl.add_ivar::<*mut c_void>(RUST_WRAPPER_IVAR_NAME);
        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            did_finish_launching as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(applicationDidBecomeActive:),
            did_become_active as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(applicationDidResignActive:),
            did_resign_active as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(application:openFiles:),
            open_files as extern "C" fn(&Object, Sel, id, id),
        );
        decl.register()
    }
}

unsafe fn get_app(object: &Object) -> &mut App {
    let wrapper_ptr: *mut c_void = *object.get_ivar(RUST_WRAPPER_IVAR_NAME);
    &mut *(wrapper_ptr as *mut App)
}

extern "C" fn send_event(this: &Object, _sel: Sel, native_event: id) {
    let event = unsafe { Event::from_native(native_event, None) };

    if let Some(event) = event {
        let app = unsafe { get_app(this) };
        if let Some(callback) = app.event_callback.as_mut() {
            if callback(event) {
                return;
            }
        }
    }

    unsafe {
        let _: () = msg_send![super(this, class!(NSApplication)), sendEvent: native_event];
    }
}

extern "C" fn did_finish_launching(this: &Object, _: Sel, _: id) {
    let app = unsafe { get_app(this) };
    if let Some(callback) = app.finish_launching_callback.take() {
        callback();
    }
}

extern "C" fn did_become_active(this: &Object, _: Sel, _: id) {
    let app = unsafe { get_app(this) };
    if let Some(callback) = app.become_active_callback.as_mut() {
        callback();
    }
}

extern "C" fn did_resign_active(this: &Object, _: Sel, _: id) {
    let app = unsafe { get_app(this) };
    if let Some(callback) = app.resign_active_callback.as_mut() {
        callback();
    }
}

extern "C" fn open_files(this: &Object, _: Sel, _: id, paths: id) {
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
    let app = unsafe { get_app(this) };
    if let Some(callback) = app.open_files_callback.as_mut() {
        callback(paths);
    }
}
