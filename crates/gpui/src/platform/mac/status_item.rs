use crate::{
    geometry::vector::{vec2f, Vector2F},
    platform::{
        self,
        mac::{
            platform::{NSKeyValueObservingOptionNew, NSViewLayerContentsRedrawDuringViewResize},
            renderer::Renderer,
        },
    },
    Event, FontSystem, Scene,
};
use cocoa::{
    appkit::{NSSquareStatusItemLength, NSStatusBar, NSStatusItem, NSView, NSWindow},
    base::{id, nil, YES},
    foundation::{NSPoint, NSRect, NSSize, NSString},
};
use ctor::ctor;
use foreign_types::ForeignTypeRef;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    rc::StrongPtr,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use std::{
    cell::RefCell,
    ffi::c_void,
    ptr,
    rc::{Rc, Weak},
    sync::Arc,
};

static mut VIEW_CLASS: *const Class = ptr::null();
const STATE_IVAR: &str = "state";

#[ctor]
unsafe fn build_classes() {
    VIEW_CLASS = {
        let mut decl = ClassDecl::new("GPUIStatusItemView", class!(NSView)).unwrap();
        decl.add_ivar::<*mut c_void>(STATE_IVAR);

        decl.add_method(sel!(dealloc), dealloc_view as extern "C" fn(&Object, Sel));

        decl.add_method(
            sel!(mouseDown:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(mouseUp:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(rightMouseDown:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(rightMouseUp:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(otherMouseDown:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(otherMouseUp:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(mouseMoved:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(mouseDragged:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(scrollWheel:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(flagsChanged:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(makeBackingLayer),
            make_backing_layer as extern "C" fn(&Object, Sel) -> id,
        );
        decl.add_method(
            sel!(observeValueForKeyPath:ofObject:change:context:),
            appearance_changed as extern "C" fn(&Object, Sel, id, id, id, id),
        );

        decl.add_protocol(Protocol::get("CALayerDelegate").unwrap());
        decl.add_method(
            sel!(displayLayer:),
            display_layer as extern "C" fn(&Object, Sel, id),
        );

        decl.register()
    };
}

pub struct StatusItem(Rc<RefCell<StatusItemState>>);

struct StatusItemState {
    native_item: StrongPtr,
    native_view: StrongPtr,
    renderer: Renderer,
    scene: Option<Scene>,
    event_callback: Option<Box<dyn FnMut(Event) -> bool>>,
    appearance_changed_callback: Option<Box<dyn FnMut()>>,
}

impl StatusItem {
    pub fn add(fonts: Arc<dyn FontSystem>) -> Self {
        unsafe {
            let renderer = Renderer::new(false, fonts);
            let status_bar = NSStatusBar::systemStatusBar(nil);
            let native_item =
                StrongPtr::retain(status_bar.statusItemWithLength_(NSSquareStatusItemLength));

            let button = native_item.button();
            let _: () = msg_send![button, setHidden: YES];

            let native_view = msg_send![VIEW_CLASS, alloc];
            let state = Rc::new(RefCell::new(StatusItemState {
                native_item,
                native_view: StrongPtr::new(native_view),
                renderer,
                scene: None,
                event_callback: None,
                appearance_changed_callback: None,
            }));

            let parent_view = button.superview().superview();
            NSView::initWithFrame_(
                native_view,
                NSRect::new(NSPoint::new(0., 0.), NSView::frame(parent_view).size),
            );
            (*native_view).set_ivar(
                STATE_IVAR,
                Weak::into_raw(Rc::downgrade(&state)) as *const c_void,
            );
            native_view.setWantsBestResolutionOpenGLSurface_(YES);
            native_view.setWantsLayer(true);
            let _: () = msg_send![
                native_view,
                setLayerContentsRedrawPolicy: NSViewLayerContentsRedrawDuringViewResize
            ];
            let _: () = msg_send![
                button,
                addObserver: native_view
                forKeyPath: NSString::alloc(nil).init_str("effectiveAppearance")
                options: NSKeyValueObservingOptionNew
                context: nil
            ];

            parent_view.addSubview_(native_view);

            {
                let state = state.borrow();
                let layer = state.renderer.layer();
                let scale_factor = state.scale_factor();
                let size = state.size() * scale_factor;
                layer.set_contents_scale(scale_factor.into());
                layer.set_drawable_size(metal::CGSize::new(size.x().into(), size.y().into()));
            }

            Self(state)
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

    fn on_appearance_changed(&mut self, callback: Box<dyn FnMut()>) {
        self.0.borrow_mut().appearance_changed_callback = Some(callback);
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
        self.0.borrow_mut().scene = Some(scene);
        unsafe {
            let _: () = msg_send![*self.0.borrow().native_view, setNeedsDisplay: YES];
        }
    }

    fn appearance(&self) -> crate::Appearance {
        unsafe {
            let appearance: id =
                msg_send![self.0.borrow().native_item.button(), effectiveAppearance];
            crate::Appearance::from_native(appearance)
        }
    }
}

impl StatusItemState {
    fn size(&self) -> Vector2F {
        unsafe {
            let NSSize { width, height, .. } =
                NSWindow::frame(self.native_item.button().superview().superview()).size;
            vec2f(width as f32, height as f32)
        }
    }

    fn scale_factor(&self) -> f32 {
        unsafe {
            let window: id = msg_send![self.native_item.button(), window];
            window.screen().backingScaleFactor() as f32
        }
    }
}

extern "C" fn dealloc_view(this: &Object, _: Sel) {
    unsafe {
        drop_state(this);

        let _: () = msg_send![super(this, class!(NSView)), dealloc];
    }
}

extern "C" fn handle_view_event(this: &Object, _: Sel, native_event: id) {
    unsafe {
        if let Some(state) = get_state(this).upgrade() {
            let mut state_borrow = state.as_ref().borrow_mut();
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

extern "C" fn make_backing_layer(this: &Object, _: Sel) -> id {
    if let Some(state) = unsafe { get_state(this).upgrade() } {
        let state = state.borrow();
        state.renderer.layer().as_ptr() as id
    } else {
        nil
    }
}

extern "C" fn display_layer(this: &Object, _: Sel, _: id) {
    unsafe {
        if let Some(state) = get_state(this).upgrade() {
            let mut state = state.borrow_mut();
            if let Some(scene) = state.scene.take() {
                state.renderer.render(&scene);
            }
        }
    }
}

extern "C" fn appearance_changed(this: &Object, _: Sel, _: id, _: id, _: id, _: id) {
    unsafe {
        if let Some(state) = get_state(this).upgrade() {
            let mut state_borrow = state.as_ref().borrow_mut();
            if let Some(mut callback) = state_borrow.appearance_changed_callback.take() {
                drop(state_borrow);
                callback();
                state.borrow_mut().appearance_changed_callback = Some(callback);
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
