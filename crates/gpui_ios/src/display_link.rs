use objc::{
    class, declare::ClassDecl, msg_send, runtime::{Class, Object, Sel}, sel, sel_impl,
};
use std::{ffi::c_void, sync::OnceLock};

/// iOS replacement for CVDisplayLink — uses CADisplayLink which fires on the main
/// run loop at the display refresh rate (up to 120 Hz on ProMotion iPads).
///
/// Unlike CVDisplayLink, CADisplayLink fires on the thread it was created on (the
/// main thread), so no dispatch-source indirection is needed.
pub struct DisplayLink {
    /// Retained CADisplayLink ObjC object.
    display_link: *mut Object,
    /// Retained ZedDisplayLinkTarget ObjC object (holds the callback pointer).
    target: *mut Object,
}

// DisplayLink is only accessed from the main thread.
unsafe impl Send for DisplayLink {}
unsafe impl Sync for DisplayLink {}

impl DisplayLink {
    /// Creates a CADisplayLink that calls `callback(data)` on every vsync.
    ///
    /// The link starts paused; call `start()` to begin receiving callbacks.
    pub fn new(data: *mut c_void, callback: extern "C" fn(*mut c_void)) -> Self {
        let target_class = register_target_class();

        let target: *mut Object = unsafe {
            let object: *mut Object = msg_send![target_class, alloc];
            msg_send![object, init]
        };
        unsafe {
            (*target).set_ivar("callback_data", data);
            (*target).set_ivar("callback_fn", callback as usize);
        }

        let display_link: *mut Object = unsafe {
            msg_send![
                class!(CADisplayLink),
                displayLinkWithTarget: target
                selector: sel!(displayLinkFired:)
            ]
        };
        // Retain — CADisplayLink is autoreleased by +displayLinkWithTarget:selector:.
        unsafe {
            let _: *mut Object = msg_send![display_link, retain];
        }

        // Add to the main run loop in NSRunLoopCommonModes so it fires during
        // scroll tracking and other non-default run-loop modes.
        unsafe {
            let main_run_loop: *mut Object = msg_send![class!(NSRunLoop), mainRunLoop];
            let common_modes: *mut Object = get_run_loop_common_modes();
            let _: () = msg_send![display_link, addToRunLoop: main_run_loop forMode: common_modes];
            // Start paused; caller decides when to begin frames.
            let _: () = msg_send![display_link, setPaused: 1i8];
        }

        Self {
            display_link,
            target,
        }
    }

    /// Unpauses the CADisplayLink so it starts delivering vsync callbacks.
    pub fn start(&self) {
        unsafe {
            let _: () = msg_send![self.display_link, setPaused: 0i8];
        }
    }

    /// Pauses the CADisplayLink without invalidating it.
    #[allow(dead_code)]
    pub fn stop(&self) {
        unsafe {
            let _: () = msg_send![self.display_link, setPaused: 1i8];
        }
    }
}

impl Drop for DisplayLink {
    fn drop(&mut self) {
        // invalidate() removes the link from its run loop and prevents further callbacks.
        unsafe {
            let _: () = msg_send![self.display_link, invalidate];
            let _: () = msg_send![self.display_link, release];
            let _: () = msg_send![self.target, release];
        }
    }
}

/// Registers the `ZedDisplayLinkTarget` ObjC class (once per process).
///
/// The class stores two ivars:
///   - `callback_data: *mut c_void`  — forwarded to the Rust callback
///   - `callback_fn: usize`          — function pointer cast from `extern "C" fn(*mut c_void)`
fn register_target_class() -> &'static Class {
    static CLASS: OnceLock<&'static Class> = OnceLock::new();
    CLASS.get_or_init(|| {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("ZedDisplayLinkTarget", superclass)
            .expect("ZedDisplayLinkTarget already registered");

        unsafe {
            decl.add_ivar::<*mut c_void>("callback_data");
            decl.add_ivar::<usize>("callback_fn");

            extern "C" fn display_link_fired(this: &Object, _sel: Sel, _link: *mut Object) {
                unsafe {
                    let data: *mut c_void = *this.get_ivar("callback_data");
                    let fn_ptr: usize = *this.get_ivar("callback_fn");
                    let callback: extern "C" fn(*mut c_void) = std::mem::transmute(fn_ptr);
                    callback(data);
                }
            }

            decl.add_method(
                sel!(displayLinkFired:),
                display_link_fired as extern "C" fn(&Object, Sel, *mut Object),
            );
        }

        decl.register()
    })
}

/// Returns the `NSRunLoopCommonModes` string constant via a linked symbol.
fn get_run_loop_common_modes() -> *mut Object {
    unsafe {
        #[link(name = "Foundation", kind = "framework")]
        unsafe extern "C" {
            static NSRunLoopCommonModes: *mut Object;
        }
        NSRunLoopCommonModes
    }
}
