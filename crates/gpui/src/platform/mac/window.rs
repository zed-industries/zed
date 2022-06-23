use crate::{
    executor,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    keymap::Keystroke,
    platform::{self, Event, WindowBounds, WindowContext},
    Scene,
};
use block::ConcreteBlock;
use cocoa::{
    appkit::{
        CGPoint, NSApplication, NSBackingStoreBuffered, NSScreen, NSView, NSViewHeightSizable,
        NSViewWidthSizable, NSWindow, NSWindowButton, NSWindowStyleMask,
    },
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSInteger, NSSize, NSString},
    quartzcore::AutoresizingMask,
};
use core_graphics::display::CGRect;
use ctor::ctor;
use foreign_types::ForeignType as _;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Protocol, Sel, BOOL, NO, YES},
    sel, sel_impl,
};
use postage::oneshot;
use smol::Timer;
use std::{
    any::Any,
    cell::{Cell, RefCell},
    convert::TryInto,
    ffi::c_void,
    mem, ptr,
    rc::{Rc, Weak},
    sync::Arc,
    time::Duration,
};

use super::{geometry::RectFExt, renderer::Renderer};

const WINDOW_STATE_IVAR: &'static str = "windowState";

static mut WINDOW_CLASS: *const Class = ptr::null();
static mut VIEW_CLASS: *const Class = ptr::null();

#[allow(non_upper_case_globals)]
const NSViewLayerContentsRedrawDuringViewResize: NSInteger = 2;

#[ctor]
unsafe fn build_classes() {
    WINDOW_CLASS = {
        let mut decl = ClassDecl::new("GPUIWindow", class!(NSWindow)).unwrap();
        decl.add_ivar::<*mut c_void>(WINDOW_STATE_IVAR);
        decl.add_method(sel!(dealloc), dealloc_window as extern "C" fn(&Object, Sel));
        decl.add_method(
            sel!(canBecomeMainWindow),
            yes as extern "C" fn(&Object, Sel) -> BOOL,
        );
        decl.add_method(
            sel!(canBecomeKeyWindow),
            yes as extern "C" fn(&Object, Sel) -> BOOL,
        );
        decl.add_method(
            sel!(sendEvent:),
            send_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(windowDidResize:),
            window_did_resize as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(windowDidBecomeKey:),
            window_did_change_key_status as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(windowDidResignKey:),
            window_did_change_key_status as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(windowShouldClose:),
            window_should_close as extern "C" fn(&Object, Sel, id) -> BOOL,
        );
        decl.add_method(sel!(close), close_window as extern "C" fn(&Object, Sel));
        decl.register()
    };

    VIEW_CLASS = {
        let mut decl = ClassDecl::new("GPUIView", class!(NSView)).unwrap();
        decl.add_ivar::<*mut c_void>(WINDOW_STATE_IVAR);

        decl.add_method(sel!(dealloc), dealloc_view as extern "C" fn(&Object, Sel));

        decl.add_method(
            sel!(performKeyEquivalent:),
            handle_key_equivalent as extern "C" fn(&Object, Sel, id) -> BOOL,
        );
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
            sel!(cancelOperation:),
            cancel_operation as extern "C" fn(&Object, Sel, id),
        );

        decl.add_method(
            sel!(makeBackingLayer),
            make_backing_layer as extern "C" fn(&Object, Sel) -> id,
        );

        decl.add_protocol(Protocol::get("CALayerDelegate").unwrap());
        decl.add_method(
            sel!(viewDidChangeBackingProperties),
            view_did_change_backing_properties as extern "C" fn(&Object, Sel),
        );
        decl.add_method(
            sel!(setFrameSize:),
            set_frame_size as extern "C" fn(&Object, Sel, NSSize),
        );
        decl.add_method(
            sel!(displayLayer:),
            display_layer as extern "C" fn(&Object, Sel, id),
        );

        decl.register()
    };
}

pub struct Window(Rc<RefCell<WindowState>>);

struct WindowState {
    id: usize,
    native_window: id,
    event_callback: Option<Box<dyn FnMut(Event) -> bool>>,
    activate_callback: Option<Box<dyn FnMut(bool)>>,
    resize_callback: Option<Box<dyn FnMut()>>,
    should_close_callback: Option<Box<dyn FnMut() -> bool>>,
    close_callback: Option<Box<dyn FnOnce()>>,
    synthetic_drag_counter: usize,
    executor: Rc<executor::Foreground>,
    scene_to_render: Option<Scene>,
    renderer: Renderer,
    command_queue: metal::CommandQueue,
    last_fresh_keydown: Option<(Keystroke, Option<String>)>,
    layer: id,
    traffic_light_position: Option<Vector2F>,
}

impl Window {
    pub fn open(
        id: usize,
        options: platform::WindowOptions,
        executor: Rc<executor::Foreground>,
        fonts: Arc<dyn platform::FontSystem>,
    ) -> Self {
        const PIXEL_FORMAT: metal::MTLPixelFormat = metal::MTLPixelFormat::BGRA8Unorm;

        unsafe {
            let pool = NSAutoreleasePool::new(nil);

            let frame = match options.bounds {
                WindowBounds::Maximized => RectF::new(Default::default(), vec2f(1024., 768.)),
                WindowBounds::Fixed(rect) => rect,
            }
            .to_ns_rect();
            let mut style_mask = NSWindowStyleMask::NSClosableWindowMask
                | NSWindowStyleMask::NSMiniaturizableWindowMask
                | NSWindowStyleMask::NSResizableWindowMask
                | NSWindowStyleMask::NSTitledWindowMask;

            if options.titlebar_appears_transparent {
                style_mask |= NSWindowStyleMask::NSFullSizeContentViewWindowMask;
            }

            let native_window: id = msg_send![WINDOW_CLASS, alloc];
            let native_window = native_window.initWithContentRect_styleMask_backing_defer_(
                frame,
                style_mask,
                NSBackingStoreBuffered,
                NO,
            );
            assert!(!native_window.is_null());

            if matches!(options.bounds, WindowBounds::Maximized) {
                let screen = native_window.screen();
                native_window.setFrame_display_(screen.visibleFrame(), YES);
            }

            let device =
                metal::Device::system_default().expect("could not find default metal device");

            let layer: id = msg_send![class!(CAMetalLayer), layer];
            let _: () = msg_send![layer, setDevice: device.as_ptr()];
            let _: () = msg_send![layer, setPixelFormat: PIXEL_FORMAT];
            let _: () = msg_send![layer, setAllowsNextDrawableTimeout: NO];
            let _: () = msg_send![layer, setNeedsDisplayOnBoundsChange: YES];
            let _: () = msg_send![layer, setPresentsWithTransaction: YES];
            let _: () = msg_send![
                layer,
                setAutoresizingMask: AutoresizingMask::WIDTH_SIZABLE
                    | AutoresizingMask::HEIGHT_SIZABLE
            ];

            let native_view: id = msg_send![VIEW_CLASS, alloc];
            let native_view = NSView::init(native_view);
            assert!(!native_view.is_null());

            let window = Self(Rc::new(RefCell::new(WindowState {
                id,
                native_window,
                event_callback: None,
                resize_callback: None,
                should_close_callback: None,
                close_callback: None,
                activate_callback: None,
                synthetic_drag_counter: 0,
                executor,
                scene_to_render: Default::default(),
                renderer: Renderer::new(
                    device.clone(),
                    PIXEL_FORMAT,
                    get_scale_factor(native_window),
                    fonts,
                ),
                command_queue: device.new_command_queue(),
                last_fresh_keydown: None,
                layer,
                traffic_light_position: options.traffic_light_position,
            })));

            (*native_window).set_ivar(
                WINDOW_STATE_IVAR,
                Rc::into_raw(window.0.clone()) as *const c_void,
            );
            native_window.setDelegate_(native_window);
            (*native_view).set_ivar(
                WINDOW_STATE_IVAR,
                Rc::into_raw(window.0.clone()) as *const c_void,
            );

            if let Some(title) = options.title.as_ref() {
                native_window.setTitle_(NSString::alloc(nil).init_str(title));
            }
            if options.titlebar_appears_transparent {
                native_window.setTitlebarAppearsTransparent_(YES);
            }
            native_window.setAcceptsMouseMovedEvents_(YES);

            native_view.setAutoresizingMask_(NSViewWidthSizable | NSViewHeightSizable);
            native_view.setWantsBestResolutionOpenGLSurface_(YES);

            // From winit crate: On Mojave, views automatically become layer-backed shortly after
            // being added to a native_window. Changing the layer-backedness of a view breaks the
            // association between the view and its associated OpenGL context. To work around this,
            // on we explicitly make the view layer-backed up front so that AppKit doesn't do it
            // itself and break the association with its context.
            native_view.setWantsLayer(YES);
            let _: () = msg_send![
                native_view,
                setLayerContentsRedrawPolicy: NSViewLayerContentsRedrawDuringViewResize
            ];

            native_window.setContentView_(native_view.autorelease());
            native_window.makeFirstResponder_(native_view);

            native_window.center();
            native_window.makeKeyAndOrderFront_(nil);

            window.0.borrow().move_traffic_light();
            pool.drain();

            window
        }
    }

    pub fn key_window_id() -> Option<usize> {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let key_window: id = msg_send![app, keyWindow];
            if msg_send![key_window, isKindOfClass: WINDOW_CLASS] {
                let id = get_window_state(&*key_window).borrow().id;
                Some(id)
            } else {
                None
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            self.0.as_ref().borrow().native_window.close();
        }
    }
}

impl platform::Window for Window {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn on_event(&mut self, callback: Box<dyn FnMut(Event) -> bool>) {
        self.0.as_ref().borrow_mut().event_callback = Some(callback);
    }

    fn on_resize(&mut self, callback: Box<dyn FnMut()>) {
        self.0.as_ref().borrow_mut().resize_callback = Some(callback);
    }

    fn on_should_close(&mut self, callback: Box<dyn FnMut() -> bool>) {
        self.0.as_ref().borrow_mut().should_close_callback = Some(callback);
    }

    fn on_close(&mut self, callback: Box<dyn FnOnce()>) {
        self.0.as_ref().borrow_mut().close_callback = Some(callback);
    }

    fn on_active_status_change(&mut self, callback: Box<dyn FnMut(bool)>) {
        self.0.as_ref().borrow_mut().activate_callback = Some(callback);
    }

    fn prompt(
        &self,
        level: platform::PromptLevel,
        msg: &str,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        unsafe {
            let alert: id = msg_send![class!(NSAlert), alloc];
            let alert: id = msg_send![alert, init];
            let alert_style = match level {
                platform::PromptLevel::Info => 1,
                platform::PromptLevel::Warning => 0,
                platform::PromptLevel::Critical => 2,
            };
            let _: () = msg_send![alert, setAlertStyle: alert_style];
            let _: () = msg_send![alert, setMessageText: ns_string(msg)];
            for (ix, answer) in answers.into_iter().enumerate() {
                let button: id = msg_send![alert, addButtonWithTitle: ns_string(answer)];
                let _: () = msg_send![button, setTag: ix as NSInteger];
            }
            let (done_tx, done_rx) = oneshot::channel();
            let done_tx = Cell::new(Some(done_tx));
            let block = ConcreteBlock::new(move |answer: NSInteger| {
                if let Some(mut done_tx) = done_tx.take() {
                    let _ = postage::sink::Sink::try_send(&mut done_tx, answer.try_into().unwrap());
                }
            });
            let block = block.copy();
            let native_window = self.0.borrow().native_window;
            let _: () = msg_send![
                alert,
                beginSheetModalForWindow: native_window
                completionHandler: block
            ];
            done_rx
        }
    }

    fn activate(&self) {
        unsafe { msg_send![self.0.borrow().native_window, makeKeyAndOrderFront: nil] }
    }

    fn set_title(&mut self, title: &str) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let window = self.0.borrow().native_window;
            let title = ns_string(title);
            msg_send![app, changeWindowsItem:window title:title filename:false]
        }
    }

    fn set_edited(&mut self, edited: bool) {
        unsafe {
            let window = self.0.borrow().native_window;
            msg_send![window, setDocumentEdited: edited as BOOL]
        }

        // Changing the document edited state resets the traffic light position,
        // so we have to move it again.
        self.0.borrow().move_traffic_light();
    }
}

impl platform::WindowContext for Window {
    fn size(&self) -> Vector2F {
        self.0.as_ref().borrow().size()
    }

    fn scale_factor(&self) -> f32 {
        self.0.as_ref().borrow().scale_factor()
    }

    fn present_scene(&mut self, scene: Scene) {
        self.0.as_ref().borrow_mut().present_scene(scene);
    }

    fn titlebar_height(&self) -> f32 {
        self.0.as_ref().borrow().titlebar_height()
    }
}

impl WindowState {
    fn move_traffic_light(&self) {
        if let Some(traffic_light_position) = self.traffic_light_position {
            let titlebar_height = self.titlebar_height();

            unsafe {
                let close_button: id = msg_send![
                    self.native_window,
                    standardWindowButton: NSWindowButton::NSWindowCloseButton
                ];
                let min_button: id = msg_send![
                    self.native_window,
                    standardWindowButton: NSWindowButton::NSWindowMiniaturizeButton
                ];
                let zoom_button: id = msg_send![
                    self.native_window,
                    standardWindowButton: NSWindowButton::NSWindowZoomButton
                ];

                let mut close_button_frame: CGRect = msg_send![close_button, frame];
                let mut min_button_frame: CGRect = msg_send![min_button, frame];
                let mut zoom_button_frame: CGRect = msg_send![zoom_button, frame];
                let mut origin = vec2f(
                    traffic_light_position.x(),
                    titlebar_height
                        - traffic_light_position.y()
                        - close_button_frame.size.height as f32,
                );
                let button_spacing =
                    (min_button_frame.origin.x - close_button_frame.origin.x) as f32;

                close_button_frame.origin = CGPoint::new(origin.x() as f64, origin.y() as f64);
                let _: () = msg_send![close_button, setFrame: close_button_frame];
                origin.set_x(origin.x() + button_spacing);

                min_button_frame.origin = CGPoint::new(origin.x() as f64, origin.y() as f64);
                let _: () = msg_send![min_button, setFrame: min_button_frame];
                origin.set_x(origin.x() + button_spacing);

                zoom_button_frame.origin = CGPoint::new(origin.x() as f64, origin.y() as f64);
                let _: () = msg_send![zoom_button, setFrame: zoom_button_frame];
            }
        }
    }
}

impl platform::WindowContext for WindowState {
    fn size(&self) -> Vector2F {
        let NSSize { width, height, .. } =
            unsafe { NSView::frame(self.native_window.contentView()) }.size;
        vec2f(width as f32, height as f32)
    }

    fn scale_factor(&self) -> f32 {
        get_scale_factor(self.native_window)
    }

    fn titlebar_height(&self) -> f32 {
        unsafe {
            let frame = NSWindow::frame(self.native_window);
            let content_layout_rect: CGRect = msg_send![self.native_window, contentLayoutRect];
            (frame.size.height - content_layout_rect.size.height) as f32
        }
    }

    fn present_scene(&mut self, scene: Scene) {
        self.scene_to_render = Some(scene);
        unsafe {
            let _: () = msg_send![self.native_window.contentView(), setNeedsDisplay: YES];
        }
    }
}

fn get_scale_factor(native_window: id) -> f32 {
    unsafe {
        let screen: id = msg_send![native_window, screen];
        NSScreen::backingScaleFactor(screen) as f32
    }
}

unsafe fn get_window_state(object: &Object) -> Rc<RefCell<WindowState>> {
    let raw: *mut c_void = *object.get_ivar(WINDOW_STATE_IVAR);
    let rc1 = Rc::from_raw(raw as *mut RefCell<WindowState>);
    let rc2 = rc1.clone();
    mem::forget(rc1);
    rc2
}

unsafe fn drop_window_state(object: &Object) {
    let raw: *mut c_void = *object.get_ivar(WINDOW_STATE_IVAR);
    Rc::from_raw(raw as *mut RefCell<WindowState>);
}

extern "C" fn yes(_: &Object, _: Sel) -> BOOL {
    YES
}

extern "C" fn dealloc_window(this: &Object, _: Sel) {
    unsafe {
        drop_window_state(this);
        let () = msg_send![super(this, class!(NSWindow)), dealloc];
    }
}

extern "C" fn dealloc_view(this: &Object, _: Sel) {
    unsafe {
        drop_window_state(this);
        let () = msg_send![super(this, class!(NSView)), dealloc];
    }
}

extern "C" fn handle_key_equivalent(this: &Object, _: Sel, native_event: id) -> BOOL {
    let window_state = unsafe { get_window_state(this) };
    let mut window_state_borrow = window_state.as_ref().borrow_mut();

    let event = unsafe { Event::from_native(native_event, Some(window_state_borrow.size().y())) };
    if let Some(event) = event {
        match &event {
            Event::KeyDown {
                keystroke,
                input,
                is_held,
            } => {
                let keydown = (keystroke.clone(), input.clone());
                // Ignore events from held-down keys after some of the initially-pressed keys
                // were released.
                if *is_held {
                    if window_state_borrow.last_fresh_keydown.as_ref() != Some(&keydown) {
                        return YES;
                    }
                } else {
                    window_state_borrow.last_fresh_keydown = Some(keydown);
                }
            }
            _ => return NO,
        }

        if let Some(mut callback) = window_state_borrow.event_callback.take() {
            drop(window_state_borrow);
            let handled = callback(event);
            window_state.borrow_mut().event_callback = Some(callback);
            handled as BOOL
        } else {
            NO
        }
    } else {
        NO
    }
}

extern "C" fn handle_view_event(this: &Object, _: Sel, native_event: id) {
    let window_state = unsafe { get_window_state(this) };
    let weak_window_state = Rc::downgrade(&window_state);
    let mut window_state_borrow = window_state.as_ref().borrow_mut();

    let event = unsafe { Event::from_native(native_event, Some(window_state_borrow.size().y())) };

    if let Some(event) = event {
        match &event {
            Event::LeftMouseDragged { position } => {
                window_state_borrow.synthetic_drag_counter += 1;
                window_state_borrow
                    .executor
                    .spawn(synthetic_drag(
                        weak_window_state,
                        window_state_borrow.synthetic_drag_counter,
                        *position,
                    ))
                    .detach();
            }
            Event::LeftMouseUp { .. } => {
                window_state_borrow.synthetic_drag_counter += 1;
            }
            _ => {}
        }

        if let Some(mut callback) = window_state_borrow.event_callback.take() {
            drop(window_state_borrow);
            callback(event);
            window_state.borrow_mut().event_callback = Some(callback);
        }
    }
}

// Allows us to receive `cmd-.` (the shortcut for closing a dialog)
// https://bugs.eclipse.org/bugs/show_bug.cgi?id=300620#c6
extern "C" fn cancel_operation(this: &Object, _sel: Sel, _sender: id) {
    let window_state = unsafe { get_window_state(this) };
    let mut window_state_borrow = window_state.as_ref().borrow_mut();

    let chars = ".".to_string();
    let keystroke = Keystroke {
        cmd: true,
        ctrl: false,
        alt: false,
        shift: false,
        key: chars.clone(),
    };
    let event = Event::KeyDown {
        keystroke: keystroke.clone(),
        input: Some(chars.clone()),
        is_held: false,
    };

    window_state_borrow.last_fresh_keydown = Some((keystroke, Some(chars)));
    if let Some(mut callback) = window_state_borrow.event_callback.take() {
        drop(window_state_borrow);
        callback(event);
        window_state.borrow_mut().event_callback = Some(callback);
    }
}

extern "C" fn send_event(this: &Object, _: Sel, native_event: id) {
    unsafe {
        let () = msg_send![super(this, class!(NSWindow)), sendEvent: native_event];
    }
}

extern "C" fn window_did_resize(this: &Object, _: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    window_state.as_ref().borrow().move_traffic_light();
}

extern "C" fn window_did_change_key_status(this: &Object, selector: Sel, _: id) {
    let is_active = if selector == sel!(windowDidBecomeKey:) {
        true
    } else if selector == sel!(windowDidResignKey:) {
        false
    } else {
        unreachable!();
    };

    let window_state = unsafe { get_window_state(this) };
    let executor = window_state.as_ref().borrow().executor.clone();
    executor
        .spawn(async move {
            let mut window_state_borrow = window_state.as_ref().borrow_mut();
            if let Some(mut callback) = window_state_borrow.activate_callback.take() {
                drop(window_state_borrow);
                callback(is_active);
                window_state.borrow_mut().activate_callback = Some(callback);
            };
        })
        .detach();
}

extern "C" fn window_should_close(this: &Object, _: Sel, _: id) -> BOOL {
    let window_state = unsafe { get_window_state(this) };
    let mut window_state_borrow = window_state.as_ref().borrow_mut();
    if let Some(mut callback) = window_state_borrow.should_close_callback.take() {
        drop(window_state_borrow);
        let should_close = callback();
        window_state.borrow_mut().should_close_callback = Some(callback);
        should_close as BOOL
    } else {
        YES
    }
}

extern "C" fn close_window(this: &Object, _: Sel) {
    unsafe {
        let close_callback = {
            let window_state = get_window_state(this);
            window_state
                .as_ref()
                .try_borrow_mut()
                .ok()
                .and_then(|mut window_state| window_state.close_callback.take())
        };

        if let Some(callback) = close_callback {
            callback();
        }

        let () = msg_send![super(this, class!(NSWindow)), close];
    }
}

extern "C" fn make_backing_layer(this: &Object, _: Sel) -> id {
    let window_state = unsafe { get_window_state(this) };
    let window_state = window_state.as_ref().borrow();
    window_state.layer
}

extern "C" fn view_did_change_backing_properties(this: &Object, _: Sel) {
    let window_state = unsafe { get_window_state(this) };
    let mut window_state_borrow = window_state.as_ref().borrow_mut();

    unsafe {
        let _: () = msg_send![window_state_borrow.layer, setContentsScale: window_state_borrow.scale_factor() as f64];
    }

    if let Some(mut callback) = window_state_borrow.resize_callback.take() {
        drop(window_state_borrow);
        callback();
        window_state.as_ref().borrow_mut().resize_callback = Some(callback);
    };
}

extern "C" fn set_frame_size(this: &Object, _: Sel, size: NSSize) {
    let window_state = unsafe { get_window_state(this) };
    let mut window_state_borrow = window_state.as_ref().borrow_mut();

    if window_state_borrow.size() == vec2f(size.width as f32, size.height as f32) {
        return;
    }

    unsafe {
        let _: () = msg_send![super(this, class!(NSView)), setFrameSize: size];
    }

    let scale_factor = window_state_borrow.scale_factor() as f64;
    let drawable_size: NSSize = NSSize {
        width: size.width * scale_factor,
        height: size.height * scale_factor,
    };

    unsafe {
        let _: () = msg_send![window_state_borrow.layer, setDrawableSize: drawable_size];
    }

    if let Some(mut callback) = window_state_borrow.resize_callback.take() {
        drop(window_state_borrow);
        callback();
        window_state.borrow_mut().resize_callback = Some(callback);
    };
}

extern "C" fn display_layer(this: &Object, _: Sel, _: id) {
    unsafe {
        let window_state = get_window_state(this);
        let mut window_state = window_state.as_ref().borrow_mut();

        if let Some(scene) = window_state.scene_to_render.take() {
            let drawable: &metal::MetalDrawableRef = msg_send![window_state.layer, nextDrawable];
            let command_queue = window_state.command_queue.clone();
            let command_buffer = command_queue.new_command_buffer();

            let size = window_state.size();
            let scale_factor = window_state.scale_factor();

            window_state.renderer.render(
                &scene,
                size * scale_factor,
                command_buffer,
                drawable.texture(),
            );

            command_buffer.commit();
            command_buffer.wait_until_completed();
            drawable.present();
        };
    }
}

async fn synthetic_drag(
    window_state: Weak<RefCell<WindowState>>,
    drag_id: usize,
    position: Vector2F,
) {
    loop {
        Timer::after(Duration::from_millis(16)).await;
        if let Some(window_state) = window_state.upgrade() {
            let mut window_state_borrow = window_state.borrow_mut();
            if window_state_borrow.synthetic_drag_counter == drag_id {
                if let Some(mut callback) = window_state_borrow.event_callback.take() {
                    drop(window_state_borrow);
                    callback(Event::LeftMouseDragged { position });
                    window_state.borrow_mut().event_callback = Some(callback);
                }
            } else {
                break;
            }
        }
    }
}

unsafe fn ns_string(string: &str) -> id {
    NSString::alloc(nil).init_str(string).autorelease()
}
