use crate::{
    executor,
    geometry::vector::Vector2F,
    platform::{self, Event},
    Scene,
};
use anyhow::{anyhow, Result};
use cocoa::{
    appkit::{
        NSBackingStoreBuffered, NSScreen, NSView, NSViewHeightSizable, NSViewWidthSizable,
        NSWindow, NSWindowStyleMask,
    },
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSSize, NSString},
    quartzcore::AutoresizingMask,
};
use ctor::ctor;
use foreign_types::ForeignType as _;
use metal::{MTLClearColor, MTLLoadAction, MTLStoreAction};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Protocol, Sel, BOOL, NO, YES},
    sel, sel_impl,
};
use pathfinder_geometry::vector::vec2f;
use smol::Timer;
use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    mem, ptr,
    rc::Rc,
    time::{Duration, Instant},
};

use super::{geometry::RectFExt, renderer::Renderer};

const WINDOW_STATE_IVAR: &'static str = "windowState";

static mut WINDOW_CLASS: *const Class = ptr::null();
static mut VIEW_CLASS: *const Class = ptr::null();

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
        decl.register()
    };

    VIEW_CLASS = {
        let mut decl = ClassDecl::new("GPUIView", class!(NSView)).unwrap();
        decl.add_ivar::<*mut c_void>(WINDOW_STATE_IVAR);

        decl.add_method(sel!(dealloc), dealloc_view as extern "C" fn(&Object, Sel));

        decl.add_method(
            sel!(keyDown:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
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
            sel!(mouseDragged:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(scrollWheel:),
            handle_view_event as extern "C" fn(&Object, Sel, id),
        );

        decl.add_protocol(Protocol::get("CALayerDelegate").unwrap());
        decl.add_method(
            sel!(makeBackingLayer),
            make_backing_layer as extern "C" fn(&Object, Sel) -> id,
        );
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

pub struct Window(Rc<WindowState>);

struct WindowState {
    native_window: id,
    event_callback: RefCell<Option<Box<dyn FnMut(Event) -> bool>>>,
    resize_callback: RefCell<Option<Box<dyn FnMut(Vector2F, f32)>>>,
    synthetic_drag_counter: Cell<usize>,
    executor: Rc<executor::Foreground>,
    scene_to_render: RefCell<Option<Scene>>,
    renderer: RefCell<Renderer>,
    command_queue: metal::CommandQueue,
    device: metal::Device,
    layer: id,
}

pub struct RenderContext<'a> {
    pub drawable_size: Vector2F,
    pub device: &'a metal::Device,
    pub command_encoder: &'a metal::RenderCommandEncoderRef,
}

impl Window {
    pub fn open(
        options: platform::WindowOptions,
        executor: Rc<executor::Foreground>,
    ) -> Result<Self> {
        const PIXEL_FORMAT: metal::MTLPixelFormat = metal::MTLPixelFormat::BGRA8Unorm;

        unsafe {
            let pool = NSAutoreleasePool::new(nil);

            let frame = options.bounds.to_ns_rect();
            let style_mask = NSWindowStyleMask::NSClosableWindowMask
                | NSWindowStyleMask::NSMiniaturizableWindowMask
                | NSWindowStyleMask::NSResizableWindowMask
                | NSWindowStyleMask::NSTitledWindowMask;

            let native_window: id = msg_send![WINDOW_CLASS, alloc];
            let native_window = native_window.initWithContentRect_styleMask_backing_defer_(
                frame,
                style_mask,
                NSBackingStoreBuffered,
                NO,
            );

            if native_window == nil {
                return Err(anyhow!("window returned nil from initializer"));
            }

            let device = metal::Device::system_default()
                .ok_or_else(|| anyhow!("could not find default metal device"))?;

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
            if native_view == nil {
                return Err(anyhow!("view return nil from initializer"));
            }

            let window = Self(Rc::new(WindowState {
                native_window,
                event_callback: RefCell::new(None),
                resize_callback: RefCell::new(None),
                synthetic_drag_counter: Cell::new(0),
                executor,
                scene_to_render: Default::default(),
                renderer: RefCell::new(Renderer::new(&device, PIXEL_FORMAT)?),
                command_queue: device.new_command_queue(),
                device,
                layer,
            }));

            (*native_window).set_ivar(
                WINDOW_STATE_IVAR,
                Rc::into_raw(window.0.clone()) as *const c_void,
            );
            (*native_view).set_ivar(
                WINDOW_STATE_IVAR,
                Rc::into_raw(window.0.clone()) as *const c_void,
            );

            if let Some(title) = options.title.as_ref() {
                native_window.setTitle_(NSString::alloc(nil).init_str(title));
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

            native_view.layer().setBackgroundColor_(
                msg_send![class!(NSColor), colorWithRed:1.0 green:0.0 blue:0.0 alpha:1.0],
            );

            native_window.setContentView_(native_view.autorelease());
            native_window.makeFirstResponder_(native_view);

            native_window.center();
            native_window.makeKeyAndOrderFront_(nil);

            pool.drain();

            Ok(window)
        }
    }

    pub fn zoom(&self) {
        unsafe {
            self.0.native_window.performZoom_(nil);
        }
    }

    pub fn on_event<F: 'static + FnMut(Event) -> bool>(&mut self, callback: F) {
        *self.0.event_callback.borrow_mut() = Some(Box::new(callback));
    }

    pub fn on_resize<F: 'static + FnMut(Vector2F, f32)>(&mut self, callback: F) {
        *self.0.resize_callback.borrow_mut() = Some(Box::new(callback));
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            self.0.native_window.close();
            let _: () = msg_send![self.0.native_window.delegate(), release];
        }
    }
}

impl platform::Window for Window {
    fn size(&self) -> Vector2F {
        self.0.size()
    }

    fn scale_factor(&self) -> f32 {
        self.0.scale_factor()
    }

    fn render_scene(&self, scene: Scene) {
        *self.0.scene_to_render.borrow_mut() = Some(scene);
        unsafe {
            let _: () = msg_send![self.0.native_window.contentView(), setNeedsDisplay: YES];
        }
    }
}

impl WindowState {
    fn size(&self) -> Vector2F {
        let NSSize { width, height, .. } =
            unsafe { NSView::frame(self.native_window.contentView()) }.size;
        vec2f(width as f32, height as f32)
    }

    fn scale_factor(&self) -> f32 {
        unsafe {
            let screen: id = msg_send![self.native_window, screen];
            NSScreen::backingScaleFactor(screen) as f32
        }
    }

    fn next_synthetic_drag_id(&self) -> usize {
        let next_id = self.synthetic_drag_counter.get() + 1;
        self.synthetic_drag_counter.set(next_id);
        next_id
    }
}

unsafe fn window_state(object: &Object) -> Rc<WindowState> {
    let raw: *mut c_void = *object.get_ivar(WINDOW_STATE_IVAR);
    let rc1 = Rc::from_raw(raw as *mut WindowState);
    let rc2 = rc1.clone();
    mem::forget(rc1);
    rc2
}

unsafe fn drop_window_state(object: &Object) {
    let raw: *mut c_void = *object.get_ivar(WINDOW_STATE_IVAR);
    Rc::from_raw(raw as *mut WindowState);
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

extern "C" fn handle_view_event(this: &Object, _: Sel, native_event: id) {
    let window = unsafe { window_state(this) };

    let event = unsafe { Event::from_native(native_event, Some(window.size().y())) };

    if let Some(event) = event {
        match event {
            Event::LeftMouseDragged { position } => schedule_synthetic_drag(&window, position),
            Event::LeftMouseUp { .. } => {
                window.next_synthetic_drag_id();
            }
            _ => {}
        }

        if let Some(callback) = window.event_callback.borrow_mut().as_mut() {
            if callback(event) {
                return;
            }
        }
    }
}

extern "C" fn send_event(this: &Object, _: Sel, native_event: id) {
    unsafe {
        let () = msg_send![super(this, class!(NSWindow)), sendEvent: native_event];
    }
}

extern "C" fn make_backing_layer(this: &Object, _: Sel) -> id {
    let window = unsafe { window_state(this) };
    window.layer
}

extern "C" fn view_did_change_backing_properties(this: &Object, _: Sel) {
    let window;
    unsafe {
        window = window_state(this);
        let _: () = msg_send![window.layer, setContentsScale: window.scale_factor() as f64];
    }

    if let Some(callback) = window.resize_callback.borrow_mut().as_mut() {
        let size = window.size();
        let scale_factor = window.scale_factor();
        callback(size, scale_factor);
    };
}

extern "C" fn set_frame_size(this: &Object, _: Sel, size: NSSize) {
    let window;
    unsafe {
        window = window_state(this);
        if window.size() == vec2f(size.width as f32, size.height as f32) {
            return;
        }

        let _: () = msg_send![super(this, class!(NSView)), setFrameSize: size];

        let scale_factor = window.scale_factor() as f64;
        let drawable_size: NSSize = NSSize {
            width: size.width * scale_factor,
            height: size.height * scale_factor,
        };
        let _: () = msg_send![window.layer, setDrawableSize: drawable_size];
    }

    if let Some(callback) = window.resize_callback.borrow_mut().as_mut() {
        let size = window.size();
        let scale_factor = window.scale_factor();
        callback(size, scale_factor);
    };
}

extern "C" fn display_layer(this: &Object, _: Sel, _: id) {
    unsafe {
        let window = window_state(this);

        if let Some(scene) = window.scene_to_render.borrow_mut().take() {
            let drawable: &metal::MetalDrawableRef = msg_send![window.layer, nextDrawable];

            let render_pass_descriptor = metal::RenderPassDescriptor::new();
            let color_attachment = render_pass_descriptor
                .color_attachments()
                .object_at(0)
                .unwrap();
            color_attachment.set_texture(Some(drawable.texture()));
            color_attachment.set_load_action(MTLLoadAction::Clear);
            color_attachment.set_store_action(MTLStoreAction::Store);
            color_attachment.set_clear_color(MTLClearColor::new(0., 0., 0., 1.));

            let command_buffer = window.command_queue.new_command_buffer();
            let command_encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);

            window.renderer.borrow_mut().render(
                &scene,
                RenderContext {
                    drawable_size: window.size() * window.scale_factor(),
                    device: &window.device,
                    command_encoder,
                },
            );

            command_encoder.end_encoding();
            command_buffer.commit();
            command_buffer.wait_until_completed();
            drawable.present();
        };
    }
}

fn schedule_synthetic_drag(window_state: &Rc<WindowState>, position: Vector2F) {
    let drag_id = window_state.next_synthetic_drag_id();
    let weak_window_state = Rc::downgrade(window_state);
    let instant = Instant::now() + Duration::from_millis(16);
    window_state
        .executor
        .spawn(async move {
            Timer::at(instant).await;
            if let Some(window_state) = weak_window_state.upgrade() {
                if window_state.synthetic_drag_counter.get() == drag_id {
                    if let Some(callback) = window_state.event_callback.borrow_mut().as_mut() {
                        schedule_synthetic_drag(&window_state, position);
                        callback(Event::LeftMouseDragged { position });
                    }
                }
            }
        })
        .detach();
}
