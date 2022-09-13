use crate::{
    geometry::vector::{vec2f, Vector2F},
    platform::{self, mac::renderer::Renderer},
    Event, FontSystem, Scene,
};
use cocoa::{
    appkit::{
        NSApplication, NSButton, NSSquareStatusItemLength, NSStatusBar, NSStatusItem, NSView,
        NSViewHeightSizable, NSViewWidthSizable, NSWindow,
    },
    base::{id, nil, YES},
    foundation::{NSSize, NSUInteger},
};
use ctor::ctor;
use foreign_types::ForeignTypeRef;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    rc::StrongPtr,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use std::{
    cell::RefCell,
    ffi::c_void,
    ptr,
    rc::{Rc, Weak},
    sync::Arc,
};

static mut HANDLER_CLASS: *const Class = ptr::null();
const STATE_IVAR: &str = "state";

#[allow(non_upper_case_globals)]
const NSEventMaskAny: NSUInteger = NSUInteger::MAX;

#[ctor]
unsafe fn build_classes() {
    HANDLER_CLASS = {
        let mut decl = ClassDecl::new("GPUIStatusItemEventHandler", class!(NSObject)).unwrap();
        decl.add_ivar::<*mut c_void>(STATE_IVAR);
        decl.add_method(
            sel!(dealloc),
            dealloc_handler as extern "C" fn(&Object, Sel),
        );
        decl.add_method(
            sel!(handleEvent),
            handle_event as extern "C" fn(&Object, Sel),
        );

        decl.register()
    };
}

pub struct StatusItem(Rc<RefCell<StatusItemState>>);

struct StatusItemState {
    native_item: StrongPtr,
    renderer: Renderer,
    event_callback: Option<Box<dyn FnMut(Event) -> bool>>,
    _event_handler: StrongPtr,
}

impl StatusItem {
    pub fn add(fonts: Arc<dyn FontSystem>) -> Self {
        unsafe {
            let renderer = Renderer::new(fonts);
            let status_bar = NSStatusBar::systemStatusBar(nil);
            let native_item =
                StrongPtr::retain(status_bar.statusItemWithLength_(NSSquareStatusItemLength));

            let button = native_item.button();
            button.setAutoresizingMask_(NSViewWidthSizable | NSViewHeightSizable);
            button.setWantsBestResolutionOpenGLSurface_(YES);
            button.setLayer(renderer.layer().as_ptr() as id);

            Self(Rc::new_cyclic(|state| {
                let event_handler = StrongPtr::new(msg_send![HANDLER_CLASS, alloc]);
                let _: () = msg_send![*event_handler, init];
                (**event_handler)
                    .set_ivar(STATE_IVAR, Weak::into_raw(state.clone()) as *const c_void);
                button.setTarget_(*event_handler);
                button.setAction_(sel!(handleEvent));
                let _: () = msg_send![button, sendActionOn: NSEventMaskAny];

                RefCell::new(StatusItemState {
                    native_item,
                    renderer,
                    event_callback: None,
                    _event_handler: event_handler,
                })
            }))
        }
    }
}

impl platform::Window for StatusItem {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn on_event(&mut self, callback: Box<dyn FnMut(crate::Event) -> bool>) {
        self.0.borrow_mut().event_callback = Some(callback);
    }

    fn on_active_status_change(&mut self, _: Box<dyn FnMut(bool)>) {
        unimplemented!()
    }

    fn on_resize(&mut self, _: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_fullscreen(&mut self, _: Box<dyn FnMut(bool)>) {
        unimplemented!()
    }

    fn on_should_close(&mut self, _: Box<dyn FnMut() -> bool>) {
        unimplemented!()
    }

    fn on_close(&mut self, _: Box<dyn FnOnce()>) {
        unimplemented!()
    }

    fn set_input_handler(&mut self, _: Box<dyn crate::InputHandler>) {
        unimplemented!()
    }

    fn prompt(
        &self,
        _: crate::PromptLevel,
        _: &str,
        _: &[&str],
    ) -> postage::oneshot::Receiver<usize> {
        unimplemented!()
    }

    fn activate(&self) {
        unimplemented!()
    }

    fn set_title(&mut self, _: &str) {
        unimplemented!()
    }

    fn set_edited(&mut self, _: bool) {
        unimplemented!()
    }

    fn show_character_palette(&self) {
        unimplemented!()
    }

    fn minimize(&self) {
        unimplemented!()
    }

    fn zoom(&self) {
        unimplemented!()
    }

    fn toggle_full_screen(&self) {
        unimplemented!()
    }

    fn size(&self) -> Vector2F {
        self.0.borrow().size()
    }

    fn scale_factor(&self) -> f32 {
        self.0.borrow().scale_factor()
    }

    fn titlebar_height(&self) -> f32 {
        0.
    }

    fn present_scene(&mut self, scene: Scene) {
        self.0.borrow_mut().renderer.render(&scene);
    }
}

impl StatusItemState {
    fn size(&self) -> Vector2F {
        let NSSize { width, height, .. } = unsafe { NSView::frame(self.native_item.button()) }.size;
        vec2f(width as f32, height as f32)
    }

    fn scale_factor(&self) -> f32 {
        unsafe {
            let window: id = msg_send![self.native_item.button(), window];
            window.screen().backingScaleFactor() as f32
        }
    }
}

extern "C" fn dealloc_handler(this: &Object, _: Sel) {
    unsafe {
        drop_state(this);
        let _: () = msg_send![super(this, class!(NSObject)), dealloc];
    }
}

extern "C" fn handle_event(this: &Object, _: Sel) {
    unsafe {
        if let Some(state) = get_state(this).upgrade() {
            let mut state_borrow = state.as_ref().borrow_mut();
            let app = NSApplication::sharedApplication(nil);
            let native_event: id = msg_send![app, currentEvent];
            if let Some(event) = Event::from_native(native_event, Some(state_borrow.size().y())) {
                if let Some(mut callback) = state_borrow.event_callback.take() {
                    drop(state_borrow);
                    callback(event);
                    state.borrow_mut().event_callback = Some(callback);
                }
            }
        }
    }
}

unsafe fn get_state(object: &Object) -> Weak<RefCell<StatusItemState>> {
    let raw: *mut c_void = *object.get_ivar(STATE_IVAR);
    let weak1 = Weak::from_raw(raw as *mut RefCell<StatusItemState>);
    let weak2 = weak1.clone();
    let _ = Weak::into_raw(weak1);
    weak2
}

unsafe fn drop_state(object: &Object) {
    let raw: *const c_void = *object.get_ivar(STATE_IVAR);
    Weak::from_raw(raw as *const RefCell<StatusItemState>);
}
