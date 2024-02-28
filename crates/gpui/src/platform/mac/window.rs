use super::{global_bounds_from_ns_rect, ns_string, renderer, MacDisplay, NSRange};
use crate::{
    global_bounds_to_ns_rect, platform::PlatformInputHandler, point, px, size, AnyWindowHandle,
    Bounds, DisplayLink, ExternalPaths, FileDropEvent, ForegroundExecutor, GlobalPixels,
    KeyDownEvent, Keystroke, Modifiers, ModifiersChangedEvent, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformWindow, Point, PromptLevel, Size, Timer, WindowAppearance, WindowBounds, WindowKind,
    WindowOptions,
};
use block::ConcreteBlock;
use cocoa::{
    appkit::{
        CGPoint, NSApplication, NSBackingStoreBuffered, NSEventModifierFlags,
        NSFilenamesPboardType, NSPasteboard, NSScreen, NSView, NSViewHeightSizable,
        NSViewWidthSizable, NSWindow, NSWindowButton, NSWindowCollectionBehavior,
        NSWindowOcclusionState, NSWindowStyleMask, NSWindowTitleVisibility,
    },
    base::{id, nil},
    foundation::{
        NSArray, NSAutoreleasePool, NSDictionary, NSFastEnumeration, NSInteger, NSPoint, NSRect,
        NSSize, NSString, NSUInteger,
    },
};
use core_graphics::display::{CGDirectDisplayID, CGRect};
use ctor::ctor;
use futures::channel::oneshot;
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Protocol, Sel, BOOL, NO, YES},
    sel, sel_impl,
};
use parking_lot::Mutex;
use raw_window_handle::{
    AppKitDisplayHandle, AppKitWindowHandle, DisplayHandle, HasDisplayHandle, HasWindowHandle,
    RawWindowHandle, WindowHandle,
};
use smallvec::SmallVec;
use std::{
    any::Any,
    cell::Cell,
    ffi::{c_void, CStr},
    mem,
    ops::Range,
    os::raw::c_char,
    path::PathBuf,
    ptr::{self, NonNull},
    rc::Rc,
    sync::{Arc, Weak},
    time::Duration,
};
use util::ResultExt;

const WINDOW_STATE_IVAR: &str = "windowState";

static mut WINDOW_CLASS: *const Class = ptr::null();
static mut PANEL_CLASS: *const Class = ptr::null();
static mut VIEW_CLASS: *const Class = ptr::null();

#[allow(non_upper_case_globals)]
const NSWindowStyleMaskNonactivatingPanel: NSWindowStyleMask =
    unsafe { NSWindowStyleMask::from_bits_unchecked(1 << 7) };
#[allow(non_upper_case_globals)]
const NSNormalWindowLevel: NSInteger = 0;
#[allow(non_upper_case_globals)]
const NSPopUpWindowLevel: NSInteger = 101;
#[allow(non_upper_case_globals)]
const NSTrackingMouseEnteredAndExited: NSUInteger = 0x01;
#[allow(non_upper_case_globals)]
const NSTrackingMouseMoved: NSUInteger = 0x02;
#[allow(non_upper_case_globals)]
const NSTrackingActiveAlways: NSUInteger = 0x80;
#[allow(non_upper_case_globals)]
const NSTrackingInVisibleRect: NSUInteger = 0x200;
#[allow(non_upper_case_globals)]
const NSWindowAnimationBehaviorUtilityWindow: NSInteger = 4;
#[allow(non_upper_case_globals)]
const NSViewLayerContentsRedrawDuringViewResize: NSInteger = 2;
// https://developer.apple.com/documentation/appkit/nsdragoperation
type NSDragOperation = NSUInteger;
#[allow(non_upper_case_globals)]
const NSDragOperationNone: NSDragOperation = 0;
#[allow(non_upper_case_globals)]
const NSDragOperationCopy: NSDragOperation = 1;

#[ctor]
unsafe fn build_classes() {
    WINDOW_CLASS = build_window_class("GPUIWindow", class!(NSWindow));
    PANEL_CLASS = build_window_class("GPUIPanel", class!(NSPanel));
    VIEW_CLASS = {
        let mut decl = ClassDecl::new("GPUIView", class!(NSView)).unwrap();
        decl.add_ivar::<*mut c_void>(WINDOW_STATE_IVAR);

        decl.add_method(sel!(dealloc), dealloc_view as extern "C" fn(&Object, Sel));

        decl.add_method(
            sel!(performKeyEquivalent:),
            handle_key_equivalent as extern "C" fn(&Object, Sel, id) -> BOOL,
        );
        decl.add_method(
            sel!(keyDown:),
            handle_key_down as extern "C" fn(&Object, Sel, id),
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
            sel!(mouseExited:),
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

        decl.add_protocol(Protocol::get("NSTextInputClient").unwrap());
        decl.add_method(
            sel!(validAttributesForMarkedText),
            valid_attributes_for_marked_text as extern "C" fn(&Object, Sel) -> id,
        );
        decl.add_method(
            sel!(hasMarkedText),
            has_marked_text as extern "C" fn(&Object, Sel) -> BOOL,
        );
        decl.add_method(
            sel!(markedRange),
            marked_range as extern "C" fn(&Object, Sel) -> NSRange,
        );
        decl.add_method(
            sel!(selectedRange),
            selected_range as extern "C" fn(&Object, Sel) -> NSRange,
        );
        decl.add_method(
            sel!(firstRectForCharacterRange:actualRange:),
            first_rect_for_character_range as extern "C" fn(&Object, Sel, NSRange, id) -> NSRect,
        );
        decl.add_method(
            sel!(insertText:replacementRange:),
            insert_text as extern "C" fn(&Object, Sel, id, NSRange),
        );
        decl.add_method(
            sel!(setMarkedText:selectedRange:replacementRange:),
            set_marked_text as extern "C" fn(&Object, Sel, id, NSRange, NSRange),
        );
        decl.add_method(sel!(unmarkText), unmark_text as extern "C" fn(&Object, Sel));
        decl.add_method(
            sel!(attributedSubstringForProposedRange:actualRange:),
            attributed_substring_for_proposed_range
                as extern "C" fn(&Object, Sel, NSRange, *mut c_void) -> id,
        );
        decl.add_method(
            sel!(viewDidChangeEffectiveAppearance),
            view_did_change_effective_appearance as extern "C" fn(&Object, Sel),
        );

        // Suppress beep on keystrokes with modifier keys.
        decl.add_method(
            sel!(doCommandBySelector:),
            do_command_by_selector as extern "C" fn(&Object, Sel, Sel),
        );

        decl.add_method(
            sel!(acceptsFirstMouse:),
            accepts_first_mouse as extern "C" fn(&Object, Sel, id) -> BOOL,
        );

        decl.register()
    };
}

pub(crate) fn convert_mouse_position(position: NSPoint, window_height: Pixels) -> Point<Pixels> {
    point(
        px(position.x as f32),
        // MacOS screen coordinates are relative to bottom left
        window_height - px(position.y as f32),
    )
}

unsafe fn build_window_class(name: &'static str, superclass: &Class) -> *const Class {
    let mut decl = ClassDecl::new(name, superclass).unwrap();
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
        sel!(windowDidResize:),
        window_did_resize as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(windowDidChangeOcclusionState:),
        window_did_change_occlusion_state as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(windowWillEnterFullScreen:),
        window_will_enter_fullscreen as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(windowWillExitFullScreen:),
        window_will_exit_fullscreen as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(windowDidMove:),
        window_did_move as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(windowDidChangeScreen:),
        window_did_change_screen as extern "C" fn(&Object, Sel, id),
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

    decl.add_method(
        sel!(draggingEntered:),
        dragging_entered as extern "C" fn(&Object, Sel, id) -> NSDragOperation,
    );
    decl.add_method(
        sel!(draggingUpdated:),
        dragging_updated as extern "C" fn(&Object, Sel, id) -> NSDragOperation,
    );
    decl.add_method(
        sel!(draggingExited:),
        dragging_exited as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(performDragOperation:),
        perform_drag_operation as extern "C" fn(&Object, Sel, id) -> BOOL,
    );
    decl.add_method(
        sel!(concludeDragOperation:),
        conclude_drag_operation as extern "C" fn(&Object, Sel, id),
    );

    decl.register()
}

#[allow(clippy::enum_variant_names)]
enum ImeInput {
    InsertText(String, Option<Range<usize>>),
    SetMarkedText(String, Option<Range<usize>>, Option<Range<usize>>),
    UnmarkText,
}

struct MacWindowState {
    handle: AnyWindowHandle,
    executor: ForegroundExecutor,
    native_window: id,
    native_window_was_closed: bool,
    native_view: NonNull<Object>,
    display_link: Option<DisplayLink>,
    renderer: renderer::Renderer,
    kind: WindowKind,
    request_frame_callback: Option<Box<dyn FnMut()>>,
    event_callback: Option<Box<dyn FnMut(PlatformInput) -> bool>>,
    activate_callback: Option<Box<dyn FnMut(bool)>>,
    resize_callback: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    fullscreen_callback: Option<Box<dyn FnMut(bool)>>,
    moved_callback: Option<Box<dyn FnMut()>>,
    should_close_callback: Option<Box<dyn FnMut() -> bool>>,
    close_callback: Option<Box<dyn FnOnce()>>,
    appearance_changed_callback: Option<Box<dyn FnMut()>>,
    input_handler: Option<PlatformInputHandler>,
    last_key_equivalent: Option<KeyDownEvent>,
    synthetic_drag_counter: usize,
    last_fresh_keydown: Option<Keystroke>,
    traffic_light_position: Option<Point<Pixels>>,
    previous_modifiers_changed_event: Option<PlatformInput>,
    // State tracking what the IME did after the last request
    input_during_keydown: Option<SmallVec<[ImeInput; 1]>>,
    previous_keydown_inserted_text: Option<String>,
    external_files_dragged: bool,
}

impl MacWindowState {
    fn move_traffic_light(&self) {
        if let Some(traffic_light_position) = self.traffic_light_position {
            if self.is_fullscreen() {
                // Moving traffic lights while fullscreen doesn't work,
                // see https://github.com/zed-industries/zed/issues/4712
                return;
            }

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
                let mut origin = point(
                    traffic_light_position.x,
                    titlebar_height
                        - traffic_light_position.y
                        - px(close_button_frame.size.height as f32),
                );
                let button_spacing =
                    px((min_button_frame.origin.x - close_button_frame.origin.x) as f32);

                close_button_frame.origin = CGPoint::new(origin.x.into(), origin.y.into());
                let _: () = msg_send![close_button, setFrame: close_button_frame];
                origin.x += button_spacing;

                min_button_frame.origin = CGPoint::new(origin.x.into(), origin.y.into());
                let _: () = msg_send![min_button, setFrame: min_button_frame];
                origin.x += button_spacing;

                zoom_button_frame.origin = CGPoint::new(origin.x.into(), origin.y.into());
                let _: () = msg_send![zoom_button, setFrame: zoom_button_frame];
                origin.x += button_spacing;
            }
        }
    }

    fn start_display_link(&mut self) {
        self.stop_display_link();
        let display_id = unsafe { display_id_for_screen(self.native_window.screen()) };
        if let Some(mut display_link) =
            DisplayLink::new(display_id, self.native_view.as_ptr() as *mut c_void, step).log_err()
        {
            display_link.start().log_err();
            self.display_link = Some(display_link);
        }
    }

    fn stop_display_link(&mut self) {
        self.display_link = None;
    }

    fn is_fullscreen(&self) -> bool {
        unsafe {
            let style_mask = self.native_window.styleMask();
            style_mask.contains(NSWindowStyleMask::NSFullScreenWindowMask)
        }
    }

    fn bounds(&self) -> WindowBounds {
        unsafe {
            if self.is_fullscreen() {
                return WindowBounds::Fullscreen;
            }

            let frame = self.frame();
            let screen_size = self.native_window.screen().visibleFrame().into();
            if frame.size == screen_size {
                WindowBounds::Maximized
            } else {
                WindowBounds::Fixed(frame)
            }
        }
    }

    fn frame(&self) -> Bounds<GlobalPixels> {
        let frame = unsafe { NSWindow::frame(self.native_window) };
        global_bounds_from_ns_rect(frame)
    }

    fn content_size(&self) -> Size<Pixels> {
        let NSSize { width, height, .. } =
            unsafe { NSView::frame(self.native_window.contentView()) }.size;
        size(px(width as f32), px(height as f32))
    }

    fn scale_factor(&self) -> f32 {
        get_scale_factor(self.native_window)
    }

    fn update_drawable_size(&mut self, drawable_size: NSSize) {
        self.renderer.update_drawable_size(Size {
            width: drawable_size.width,
            height: drawable_size.height,
        })
    }

    fn titlebar_height(&self) -> Pixels {
        unsafe {
            let frame = NSWindow::frame(self.native_window);
            let content_layout_rect: CGRect = msg_send![self.native_window, contentLayoutRect];
            px((frame.size.height - content_layout_rect.size.height) as f32)
        }
    }

    fn to_screen_ns_point(&self, point: Point<Pixels>) -> NSPoint {
        unsafe {
            let point = NSPoint::new(
                point.x.into(),
                (self.content_size().height - point.y).into(),
            );
            msg_send![self.native_window, convertPointToScreen: point]
        }
    }
}

unsafe impl Send for MacWindowState {}

pub(crate) struct MacWindow(Arc<Mutex<MacWindowState>>);

impl MacWindow {
    pub fn open(
        handle: AnyWindowHandle,
        options: WindowOptions,
        executor: ForegroundExecutor,
        renderer_context: renderer::Context,
    ) -> Self {
        unsafe {
            let pool = NSAutoreleasePool::new(nil);

            let mut style_mask;
            if let Some(titlebar) = options.titlebar.as_ref() {
                style_mask = NSWindowStyleMask::NSClosableWindowMask
                    | NSWindowStyleMask::NSMiniaturizableWindowMask
                    | NSWindowStyleMask::NSResizableWindowMask
                    | NSWindowStyleMask::NSTitledWindowMask;

                if titlebar.appears_transparent {
                    style_mask |= NSWindowStyleMask::NSFullSizeContentViewWindowMask;
                }
            } else {
                style_mask = NSWindowStyleMask::NSTitledWindowMask
                    | NSWindowStyleMask::NSFullSizeContentViewWindowMask;
            }

            let native_window: id = match options.kind {
                WindowKind::Normal => msg_send![WINDOW_CLASS, alloc],
                WindowKind::PopUp => {
                    style_mask |= NSWindowStyleMaskNonactivatingPanel;
                    msg_send![PANEL_CLASS, alloc]
                }
            };

            let display = options
                .display_id
                .and_then(MacDisplay::find_by_id)
                .unwrap_or_else(MacDisplay::primary);

            let mut target_screen = nil;
            let screens = NSScreen::screens(nil);
            let count: u64 = cocoa::foundation::NSArray::count(screens);
            for i in 0..count {
                let screen = cocoa::foundation::NSArray::objectAtIndex(screens, i);
                let display_id = display_id_for_screen(screen);
                if display_id == display.id().0 {
                    target_screen = screen;
                    break;
                }
            }

            let native_window = native_window.initWithContentRect_styleMask_backing_defer_screen_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(1024., 768.)),
                style_mask,
                NSBackingStoreBuffered,
                NO,
                target_screen,
            );
            assert!(!native_window.is_null());
            let () = msg_send![
                native_window,
                registerForDraggedTypes:
                    NSArray::arrayWithObject(nil, NSFilenamesPboardType)
            ];

            let native_view: id = msg_send![VIEW_CLASS, alloc];
            let native_view = NSView::init(native_view);
            assert!(!native_view.is_null());

            let window_size = {
                let bounds = match options.bounds {
                    WindowBounds::Fullscreen | WindowBounds::Maximized => {
                        native_window.screen().visibleFrame()
                    }
                    WindowBounds::Fixed(bounds) => global_bounds_to_ns_rect(bounds),
                };
                let scale = get_scale_factor(native_window);
                size(
                    bounds.size.width as f32 * scale,
                    bounds.size.height as f32 * scale,
                )
            };

            let window = Self(Arc::new(Mutex::new(MacWindowState {
                handle,
                executor,
                native_window,
                native_window_was_closed: false,
                native_view: NonNull::new_unchecked(native_view),
                display_link: None,
                renderer: renderer::new_renderer(
                    renderer_context,
                    native_window as *mut _,
                    native_view as *mut _,
                    window_size,
                ),
                kind: options.kind,
                request_frame_callback: None,
                event_callback: None,
                activate_callback: None,
                resize_callback: None,
                fullscreen_callback: None,
                moved_callback: None,
                should_close_callback: None,
                close_callback: None,
                appearance_changed_callback: None,
                input_handler: None,
                last_key_equivalent: None,
                synthetic_drag_counter: 0,
                last_fresh_keydown: None,
                traffic_light_position: options
                    .titlebar
                    .as_ref()
                    .and_then(|titlebar| titlebar.traffic_light_position),
                previous_modifiers_changed_event: None,
                input_during_keydown: None,
                previous_keydown_inserted_text: None,
                external_files_dragged: false,
            })));

            (*native_window).set_ivar(
                WINDOW_STATE_IVAR,
                Arc::into_raw(window.0.clone()) as *const c_void,
            );
            native_window.setDelegate_(native_window);
            (*native_view).set_ivar(
                WINDOW_STATE_IVAR,
                Arc::into_raw(window.0.clone()) as *const c_void,
            );

            if let Some(title) = options
                .titlebar
                .as_ref()
                .and_then(|t| t.title.as_ref().map(AsRef::as_ref))
            {
                native_window.setTitle_(NSString::alloc(nil).init_str(title));
            }

            native_window.setMovable_(options.is_movable as BOOL);

            if options
                .titlebar
                .map_or(true, |titlebar| titlebar.appears_transparent)
            {
                native_window.setTitlebarAppearsTransparent_(YES);
                native_window.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleHidden);
            }

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

            if options.center {
                native_window.center();
            }

            match options.kind {
                WindowKind::Normal => {
                    native_window.setLevel_(NSNormalWindowLevel);
                    native_window.setAcceptsMouseMovedEvents_(YES);
                }
                WindowKind::PopUp => {
                    // Use a tracking area to allow receiving MouseMoved events even when
                    // the window or application aren't active, which is often the case
                    // e.g. for notification windows.
                    let tracking_area: id = msg_send![class!(NSTrackingArea), alloc];
                    let _: () = msg_send![
                        tracking_area,
                        initWithRect: NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.))
                        options: NSTrackingMouseEnteredAndExited | NSTrackingMouseMoved | NSTrackingActiveAlways | NSTrackingInVisibleRect
                        owner: native_view
                        userInfo: nil
                    ];
                    let _: () =
                        msg_send![native_view, addTrackingArea: tracking_area.autorelease()];

                    native_window.setLevel_(NSPopUpWindowLevel);
                    let _: () = msg_send![
                        native_window,
                        setAnimationBehavior: NSWindowAnimationBehaviorUtilityWindow
                    ];
                    native_window.setCollectionBehavior_(
                        NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces |
                        NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
                    );
                }
            }
            if options.focus {
                native_window.makeKeyAndOrderFront_(nil);
            } else if options.show {
                native_window.orderFront_(nil);
            }

            let screen = native_window.screen();
            match options.bounds {
                WindowBounds::Fullscreen => {
                    // We need to toggle full screen asynchronously as doing so may
                    // call back into the platform handlers.
                    window.toggle_full_screen()
                }
                WindowBounds::Maximized => {
                    native_window.setFrame_display_(screen.visibleFrame(), YES);
                }
                WindowBounds::Fixed(bounds) => {
                    let display_bounds = display.bounds();
                    let frame = if bounds.intersects(&display_bounds) {
                        global_bounds_to_ns_rect(bounds)
                    } else {
                        global_bounds_to_ns_rect(display_bounds)
                    };
                    native_window.setFrame_display_(frame, YES);
                }
            }

            window.0.lock().move_traffic_light();

            pool.drain();

            window
        }
    }

    pub fn active_window() -> Option<AnyWindowHandle> {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let main_window: id = msg_send![app, mainWindow];
            if msg_send![main_window, isKindOfClass: WINDOW_CLASS] {
                let handle = get_window_state(&*main_window).lock().handle;
                Some(handle)
            } else {
                None
            }
        }
    }
}

impl Drop for MacWindow {
    fn drop(&mut self) {
        let mut this = self.0.lock();
        this.renderer.destroy();
        let window = this.native_window;
        this.display_link.take();
        unsafe {
            this.native_window.setDelegate_(nil);
        }
        if !this.native_window_was_closed {
            this.executor
                .spawn(async move {
                    unsafe {
                        window.close();
                    }
                })
                .detach();
        }
    }
}

impl PlatformWindow for MacWindow {
    fn bounds(&self) -> WindowBounds {
        self.0.as_ref().lock().bounds()
    }

    fn content_size(&self) -> Size<Pixels> {
        self.0.as_ref().lock().content_size()
    }

    fn scale_factor(&self) -> f32 {
        self.0.as_ref().lock().scale_factor()
    }

    fn titlebar_height(&self) -> Pixels {
        self.0.as_ref().lock().titlebar_height()
    }

    fn appearance(&self) -> WindowAppearance {
        unsafe {
            let appearance: id = msg_send![self.0.lock().native_window, effectiveAppearance];
            WindowAppearance::from_native(appearance)
        }
    }

    fn display(&self) -> Rc<dyn PlatformDisplay> {
        unsafe {
            let screen = self.0.lock().native_window.screen();
            let device_description: id = msg_send![screen, deviceDescription];
            let screen_number: id = NSDictionary::valueForKey_(
                device_description,
                NSString::alloc(nil).init_str("NSScreenNumber"),
            );

            let screen_number: u32 = msg_send![screen_number, unsignedIntValue];

            Rc::new(MacDisplay(screen_number))
        }
    }

    fn mouse_position(&self) -> Point<Pixels> {
        let position = unsafe {
            self.0
                .lock()
                .native_window
                .mouseLocationOutsideOfEventStream()
        };
        convert_mouse_position(position, self.content_size().height)
    }

    fn modifiers(&self) -> Modifiers {
        unsafe {
            let modifiers: NSEventModifierFlags = msg_send![class!(NSEvent), modifierFlags];

            let control = modifiers.contains(NSEventModifierFlags::NSControlKeyMask);
            let alt = modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
            let shift = modifiers.contains(NSEventModifierFlags::NSShiftKeyMask);
            let command = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);
            let function = modifiers.contains(NSEventModifierFlags::NSFunctionKeyMask);

            Modifiers {
                control,
                alt,
                shift,
                command,
                function,
            }
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.as_ref().lock().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.as_ref().lock().input_handler.take()
    }

    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        // macOs applies overrides to modal window buttons after they are added.
        // Two most important for this logic are:
        // * Buttons with "Cancel" title will be displayed as the last buttons in the modal
        // * Last button added to the modal via `addButtonWithTitle` stays focused
        // * Focused buttons react on "space"/" " keypresses
        // * Usage of `keyEquivalent`, `makeFirstResponder` or `setInitialFirstResponder` does not change the focus
        //
        // See also https://developer.apple.com/documentation/appkit/nsalert/1524532-addbuttonwithtitle#discussion
        // ```
        // By default, the first button has a key equivalent of Return,
        // any button with a title of “Cancel” has a key equivalent of Escape,
        // and any button with the title “Don’t Save” has a key equivalent of Command-D (but only if it’s not the first button).
        // ```
        //
        // To avoid situations when the last element added is "Cancel" and it gets the focus
        // (hence stealing both ESC and Space shortcuts), we find and add one non-Cancel button
        // last, so it gets focus and a Space shortcut.
        // This way, "Save this file? Yes/No/Cancel"-ish modals will get all three buttons mapped with a key.
        let latest_non_cancel_label = answers
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &label)| label != "Cancel")
            .filter(|&(label_index, _)| label_index > 0);

        unsafe {
            let alert: id = msg_send![class!(NSAlert), alloc];
            let alert: id = msg_send![alert, init];
            let alert_style = match level {
                PromptLevel::Info => 1,
                PromptLevel::Warning => 0,
                PromptLevel::Critical => 2,
            };
            let _: () = msg_send![alert, setAlertStyle: alert_style];
            let _: () = msg_send![alert, setMessageText: ns_string(msg)];
            if let Some(detail) = detail {
                let _: () = msg_send![alert, setInformativeText: ns_string(detail)];
            }

            for (ix, answer) in answers
                .iter()
                .enumerate()
                .filter(|&(ix, _)| Some(ix) != latest_non_cancel_label.map(|(ix, _)| ix))
            {
                let button: id = msg_send![alert, addButtonWithTitle: ns_string(answer)];
                let _: () = msg_send![button, setTag: ix as NSInteger];
            }
            if let Some((ix, answer)) = latest_non_cancel_label {
                let button: id = msg_send![alert, addButtonWithTitle: ns_string(answer)];
                let _: () = msg_send![button, setTag: ix as NSInteger];
            }

            let (done_tx, done_rx) = oneshot::channel();
            let done_tx = Cell::new(Some(done_tx));
            let block = ConcreteBlock::new(move |answer: NSInteger| {
                if let Some(done_tx) = done_tx.take() {
                    let _ = done_tx.send(answer.try_into().unwrap());
                }
            });
            let block = block.copy();
            let native_window = self.0.lock().native_window;
            let executor = self.0.lock().executor.clone();
            executor
                .spawn(async move {
                    let _: () = msg_send![
                        alert,
                        beginSheetModalForWindow: native_window
                        completionHandler: block
                    ];
                })
                .detach();

            done_rx
        }
    }

    fn activate(&self) {
        let window = self.0.lock().native_window;
        let executor = self.0.lock().executor.clone();
        executor
            .spawn(async move {
                unsafe {
                    let _: () = msg_send![window, makeKeyAndOrderFront: nil];
                }
            })
            .detach();
    }

    fn set_title(&mut self, title: &str) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let window = self.0.lock().native_window;
            let title = ns_string(title);
            let _: () = msg_send![app, changeWindowsItem:window title:title filename:false];
            let _: () = msg_send![window, setTitle: title];
            self.0.lock().move_traffic_light();
        }
    }

    fn set_edited(&mut self, edited: bool) {
        unsafe {
            let window = self.0.lock().native_window;
            msg_send![window, setDocumentEdited: edited as BOOL]
        }

        // Changing the document edited state resets the traffic light position,
        // so we have to move it again.
        self.0.lock().move_traffic_light();
    }

    fn show_character_palette(&self) {
        let this = self.0.lock();
        let window = this.native_window;
        this.executor
            .spawn(async move {
                unsafe {
                    let app = NSApplication::sharedApplication(nil);
                    let _: () = msg_send![app, orderFrontCharacterPalette: window];
                }
            })
            .detach();
    }

    fn minimize(&self) {
        let window = self.0.lock().native_window;
        unsafe {
            window.miniaturize_(nil);
        }
    }

    fn zoom(&self) {
        let this = self.0.lock();
        let window = this.native_window;
        this.executor
            .spawn(async move {
                unsafe {
                    window.zoom_(nil);
                }
            })
            .detach();
    }

    fn toggle_full_screen(&self) {
        let this = self.0.lock();
        let window = this.native_window;
        this.executor
            .spawn(async move {
                unsafe {
                    window.toggleFullScreen_(nil);
                }
            })
            .detach();
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.0.as_ref().lock().request_frame_callback = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        self.0.as_ref().lock().event_callback = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.as_ref().lock().activate_callback = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.as_ref().lock().resize_callback = Some(callback);
    }

    fn on_fullscreen(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.as_ref().lock().fullscreen_callback = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.as_ref().lock().moved_callback = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.as_ref().lock().should_close_callback = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.0.as_ref().lock().close_callback = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().appearance_changed_callback = Some(callback);
    }

    fn is_topmost_for_position(&self, position: Point<Pixels>) -> bool {
        let self_borrow = self.0.lock();
        let self_handle = self_borrow.handle;

        unsafe {
            let app = NSApplication::sharedApplication(nil);

            // Convert back to screen coordinates
            let screen_point = self_borrow.to_screen_ns_point(position);

            let window_number: NSInteger = msg_send![class!(NSWindow), windowNumberAtPoint:screen_point belowWindowWithWindowNumber:0];
            let top_most_window: id = msg_send![app, windowWithWindowNumber: window_number];

            let is_panel: BOOL = msg_send![top_most_window, isKindOfClass: PANEL_CLASS];
            let is_window: BOOL = msg_send![top_most_window, isKindOfClass: WINDOW_CLASS];
            if is_panel == YES || is_window == YES {
                let topmost_window = get_window_state(&*top_most_window).lock().handle;
                topmost_window == self_handle
            } else {
                // Someone else's window is on top
                false
            }
        }
    }

    fn draw(&self, scene: &crate::Scene) {
        let mut this = self.0.lock();
        this.renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.0.lock().renderer.sprite_atlas().clone()
    }
}

impl HasWindowHandle for MacWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        // SAFETY: The AppKitWindowHandle is a wrapper around a pointer to an NSView
        unsafe {
            Ok(WindowHandle::borrow_raw(RawWindowHandle::AppKit(
                AppKitWindowHandle::new(self.0.lock().native_view.cast()),
            )))
        }
    }
}

impl HasDisplayHandle for MacWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        // SAFETY: This is a no-op on macOS
        unsafe { Ok(DisplayHandle::borrow_raw(AppKitDisplayHandle::new().into())) }
    }
}

fn get_scale_factor(native_window: id) -> f32 {
    let factor = unsafe {
        let screen: id = msg_send![native_window, screen];
        NSScreen::backingScaleFactor(screen) as f32
    };

    // We are not certain what triggers this, but it seems that sometimes
    // this method would return 0 (https://github.com/zed-industries/zed/issues/6412)
    // It seems most likely that this would happen if the window has no screen
    // (if it is off-screen), though we'd expect to see viewDidChangeBackingProperties before
    // it was rendered for real.
    // Regardless, attempt to avoid the issue here.
    if factor == 0.0 {
        2.
    } else {
        factor
    }
}

unsafe fn get_window_state(object: &Object) -> Arc<Mutex<MacWindowState>> {
    let raw: *mut c_void = *object.get_ivar(WINDOW_STATE_IVAR);
    let rc1 = Arc::from_raw(raw as *mut Mutex<MacWindowState>);
    let rc2 = rc1.clone();
    mem::forget(rc1);
    rc2
}

unsafe fn drop_window_state(object: &Object) {
    let raw: *mut c_void = *object.get_ivar(WINDOW_STATE_IVAR);
    Arc::from_raw(raw as *mut Mutex<MacWindowState>);
}

extern "C" fn yes(_: &Object, _: Sel) -> BOOL {
    YES
}

extern "C" fn dealloc_window(this: &Object, _: Sel) {
    unsafe {
        drop_window_state(this);
        let _: () = msg_send![super(this, class!(NSWindow)), dealloc];
    }
}

extern "C" fn dealloc_view(this: &Object, _: Sel) {
    unsafe {
        drop_window_state(this);
        let _: () = msg_send![super(this, class!(NSView)), dealloc];
    }
}

extern "C" fn handle_key_equivalent(this: &Object, _: Sel, native_event: id) -> BOOL {
    handle_key_event(this, native_event, true)
}

extern "C" fn handle_key_down(this: &Object, _: Sel, native_event: id) {
    handle_key_event(this, native_event, false);
}

// Things to test if you're modifying this method:
//  Brazilian layout:
//   - `" space` should type a quote
//   - `" backspace` should delete the marked quote
//   - `" up` should type the quote, unmark it, and move up one line
//   - `" cmd-down` should not leave a marked quote behind (it maybe should dispatch the key though?)
//   - `cmd-ctrl-space` and clicking on an emoji should type it
//  Czech (QWERTY) layout:
//   - in vim mode `option-4`  should go to end of line (same as $)
extern "C" fn handle_key_event(this: &Object, native_event: id, key_equivalent: bool) -> BOOL {
    let window_state = unsafe { get_window_state(this) };
    let mut lock = window_state.as_ref().lock();

    let window_height = lock.content_size().height;
    let event = unsafe { PlatformInput::from_native(native_event, Some(window_height)) };

    if let Some(PlatformInput::KeyDown(mut event)) = event {
        // For certain keystrokes, macOS will first dispatch a "key equivalent" event.
        // If that event isn't handled, it will then dispatch a "key down" event. GPUI
        // makes no distinction between these two types of events, so we need to ignore
        // the "key down" event if we've already just processed its "key equivalent" version.
        if key_equivalent {
            lock.last_key_equivalent = Some(event.clone());
        } else if lock.last_key_equivalent.take().as_ref() == Some(&event) {
            return NO;
        }

        let keydown = event.keystroke.clone();
        let fn_modifier = keydown.modifiers.function;
        // Ignore events from held-down keys after some of the initially-pressed keys
        // were released.
        if event.is_held {
            if lock.last_fresh_keydown.as_ref() != Some(&keydown) {
                return YES;
            }
        } else {
            lock.last_fresh_keydown = Some(keydown.clone());
        }
        lock.input_during_keydown = Some(SmallVec::new());
        drop(lock);

        // Send the event to the input context for IME handling, unless the `fn` modifier is
        // being pressed.
        // this will call back into `insert_text`, etc.
        if !fn_modifier {
            unsafe {
                let input_context: id = msg_send![this, inputContext];
                let _: BOOL = msg_send![input_context, handleEvent: native_event];
            }
        }

        let mut handled = false;
        let mut lock = window_state.lock();
        let previous_keydown_inserted_text = lock.previous_keydown_inserted_text.take();
        let mut input_during_keydown = lock.input_during_keydown.take().unwrap();
        let mut callback = lock.event_callback.take();
        drop(lock);

        let last_ime = input_during_keydown.pop();
        // on a brazilian keyboard typing `"` and then hitting `up` will cause two IME
        // events, one to unmark the quote, and one to send the up arrow.
        for ime in input_during_keydown {
            send_to_input_handler(this, ime);
        }

        let is_composing =
            with_input_handler(this, |input_handler| input_handler.marked_text_range())
                .flatten()
                .is_some();

        if let Some(ime) = last_ime {
            if let ImeInput::InsertText(text, _) = &ime {
                if !is_composing {
                    window_state.lock().previous_keydown_inserted_text = Some(text.clone());
                    if let Some(callback) = callback.as_mut() {
                        event.keystroke.ime_key = Some(text.clone());
                        handled = callback(PlatformInput::KeyDown(event));
                    }
                }
            }

            if !handled {
                handled = true;
                send_to_input_handler(this, ime);
            }
        } else if !is_composing {
            let is_held = event.is_held;

            if let Some(callback) = callback.as_mut() {
                handled = callback(PlatformInput::KeyDown(event));
            }

            if !handled && is_held {
                if let Some(text) = previous_keydown_inserted_text {
                    // MacOS IME is a bit funky, and even when you've told it there's nothing to
                    // enter it will still swallow certain keys (e.g. 'f', 'j') and not others
                    // (e.g. 'n'). This is a problem for certain kinds of views, like the terminal.
                    with_input_handler(this, |input_handler| {
                        if input_handler.selected_text_range().is_none() {
                            handled = true;
                            input_handler.replace_text_in_range(None, &text)
                        }
                    });
                    window_state.lock().previous_keydown_inserted_text = Some(text);
                }
            }
        }

        window_state.lock().event_callback = callback;

        handled as BOOL
    } else {
        NO
    }
}

extern "C" fn handle_view_event(this: &Object, _: Sel, native_event: id) {
    let window_state = unsafe { get_window_state(this) };
    let weak_window_state = Arc::downgrade(&window_state);
    let mut lock = window_state.as_ref().lock();
    let is_active = unsafe { lock.native_window.isKeyWindow() == YES };

    let window_height = lock.content_size().height;
    let event = unsafe { PlatformInput::from_native(native_event, Some(window_height)) };

    if let Some(mut event) = event {
        match &mut event {
            PlatformInput::MouseDown(
                event @ MouseDownEvent {
                    button: MouseButton::Left,
                    modifiers: Modifiers { control: true, .. },
                    ..
                },
            ) => {
                // On mac, a ctrl-left click should be handled as a right click.
                *event = MouseDownEvent {
                    button: MouseButton::Right,
                    modifiers: Modifiers {
                        control: false,
                        ..event.modifiers
                    },
                    click_count: 1,
                    ..*event
                };
            }

            // Because we map a ctrl-left_down to a right_down -> right_up let's ignore
            // the ctrl-left_up to avoid having a mismatch in button down/up events if the
            // user is still holding ctrl when releasing the left mouse button
            PlatformInput::MouseUp(
                event @ MouseUpEvent {
                    button: MouseButton::Left,
                    modifiers: Modifiers { control: true, .. },
                    ..
                },
            ) => {
                *event = MouseUpEvent {
                    button: MouseButton::Right,
                    modifiers: Modifiers {
                        control: false,
                        ..event.modifiers
                    },
                    click_count: 1,
                    ..*event
                };
            }

            _ => {}
        };

        match &event {
            PlatformInput::MouseMove(
                event @ MouseMoveEvent {
                    pressed_button: Some(_),
                    ..
                },
            ) => {
                // Synthetic drag is used for selecting long buffer contents while buffer is being scrolled.
                // External file drag and drop is able to emit its own synthetic mouse events which will conflict
                // with these ones.
                if !lock.external_files_dragged {
                    lock.synthetic_drag_counter += 1;
                    let executor = lock.executor.clone();
                    executor
                        .spawn(synthetic_drag(
                            weak_window_state,
                            lock.synthetic_drag_counter,
                            event.clone(),
                        ))
                        .detach();
                }
            }

            PlatformInput::MouseMove(_) if !(is_active || lock.kind == WindowKind::PopUp) => return,

            PlatformInput::MouseUp(MouseUpEvent { .. }) => {
                lock.synthetic_drag_counter += 1;
            }

            PlatformInput::ModifiersChanged(ModifiersChangedEvent { modifiers }) => {
                // Only raise modifiers changed event when they have actually changed
                if let Some(PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                    modifiers: prev_modifiers,
                })) = &lock.previous_modifiers_changed_event
                {
                    if prev_modifiers == modifiers {
                        return;
                    }
                }

                lock.previous_modifiers_changed_event = Some(event.clone());
            }

            _ => {}
        }

        if let Some(mut callback) = lock.event_callback.take() {
            drop(lock);
            callback(event);
            window_state.lock().event_callback = Some(callback);
        }
    }
}

// Allows us to receive `cmd-.` (the shortcut for closing a dialog)
// https://bugs.eclipse.org/bugs/show_bug.cgi?id=300620#c6
extern "C" fn cancel_operation(this: &Object, _sel: Sel, _sender: id) {
    let window_state = unsafe { get_window_state(this) };
    let mut lock = window_state.as_ref().lock();

    let keystroke = Keystroke {
        modifiers: Default::default(),
        key: ".".into(),
        ime_key: None,
    };
    let event = PlatformInput::KeyDown(KeyDownEvent {
        keystroke: keystroke.clone(),
        is_held: false,
    });

    lock.last_fresh_keydown = Some(keystroke);
    if let Some(mut callback) = lock.event_callback.take() {
        drop(lock);
        callback(event);
        window_state.lock().event_callback = Some(callback);
    }
}

extern "C" fn window_did_change_occlusion_state(this: &Object, _: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    let lock = &mut *window_state.lock();
    unsafe {
        if lock
            .native_window
            .occlusionState()
            .contains(NSWindowOcclusionState::NSWindowOcclusionStateVisible)
        {
            lock.start_display_link();
        } else {
            lock.stop_display_link();
        }
    }
}

extern "C" fn window_did_resize(this: &Object, _: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    window_state.as_ref().lock().move_traffic_light();
}

extern "C" fn window_will_enter_fullscreen(this: &Object, _: Sel, _: id) {
    window_fullscreen_changed(this, true);
}

extern "C" fn window_will_exit_fullscreen(this: &Object, _: Sel, _: id) {
    window_fullscreen_changed(this, false);
}

fn window_fullscreen_changed(this: &Object, is_fullscreen: bool) {
    let window_state = unsafe { get_window_state(this) };
    let mut lock = window_state.as_ref().lock();
    if let Some(mut callback) = lock.fullscreen_callback.take() {
        drop(lock);
        callback(is_fullscreen);
        window_state.lock().fullscreen_callback = Some(callback);
    }
}

extern "C" fn window_did_move(this: &Object, _: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    let mut lock = window_state.as_ref().lock();
    if let Some(mut callback) = lock.moved_callback.take() {
        drop(lock);
        callback();
        window_state.lock().moved_callback = Some(callback);
    }
}

extern "C" fn window_did_change_screen(this: &Object, _: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    let mut lock = window_state.as_ref().lock();
    lock.start_display_link();
}

extern "C" fn window_did_change_key_status(this: &Object, selector: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    let lock = window_state.lock();
    let is_active = unsafe { lock.native_window.isKeyWindow() == YES };

    // When opening a pop-up while the application isn't active, Cocoa sends a spurious
    // `windowDidBecomeKey` message to the previous key window even though that window
    // isn't actually key. This causes a bug if the application is later activated while
    // the pop-up is still open, making it impossible to activate the previous key window
    // even if the pop-up gets closed. The only way to activate it again is to de-activate
    // the app and re-activate it, which is a pretty bad UX.
    // The following code detects the spurious event and invokes `resignKeyWindow`:
    // in theory, we're not supposed to invoke this method manually but it balances out
    // the spurious `becomeKeyWindow` event and helps us work around that bug.
    if selector == sel!(windowDidBecomeKey:) && !is_active {
        unsafe {
            let _: () = msg_send![lock.native_window, resignKeyWindow];
            return;
        }
    }

    let executor = lock.executor.clone();
    drop(lock);
    executor
        .spawn(async move {
            let mut lock = window_state.as_ref().lock();
            if let Some(mut callback) = lock.activate_callback.take() {
                drop(lock);
                callback(is_active);
                window_state.lock().activate_callback = Some(callback);
            };
        })
        .detach();
}

extern "C" fn window_should_close(this: &Object, _: Sel, _: id) -> BOOL {
    let window_state = unsafe { get_window_state(this) };
    let mut lock = window_state.as_ref().lock();
    if let Some(mut callback) = lock.should_close_callback.take() {
        drop(lock);
        let should_close = callback();
        window_state.lock().should_close_callback = Some(callback);
        should_close as BOOL
    } else {
        YES
    }
}

extern "C" fn close_window(this: &Object, _: Sel) {
    unsafe {
        let close_callback = {
            let window_state = get_window_state(this);
            let mut lock = window_state.as_ref().lock();
            lock.native_window_was_closed = true;
            lock.close_callback.take()
        };

        if let Some(callback) = close_callback {
            callback();
        }

        let _: () = msg_send![super(this, class!(NSWindow)), close];
    }
}

extern "C" fn make_backing_layer(this: &Object, _: Sel) -> id {
    let window_state = unsafe { get_window_state(this) };
    let window_state = window_state.as_ref().lock();
    window_state.renderer.layer_ptr() as id
}

extern "C" fn view_did_change_backing_properties(this: &Object, _: Sel) {
    let window_state = unsafe { get_window_state(this) };
    let mut lock = window_state.as_ref().lock();

    let scale_factor = lock.scale_factor() as f64;
    let size = lock.content_size();
    let drawable_size: NSSize = NSSize {
        width: f64::from(size.width) * scale_factor,
        height: f64::from(size.height) * scale_factor,
    };
    unsafe {
        let _: () = msg_send![
            lock.renderer.layer(),
            setContentsScale: scale_factor
        ];
    }

    lock.update_drawable_size(drawable_size);

    if let Some(mut callback) = lock.resize_callback.take() {
        let content_size = lock.content_size();
        let scale_factor = lock.scale_factor();
        drop(lock);
        callback(content_size, scale_factor);
        window_state.as_ref().lock().resize_callback = Some(callback);
    };
}

extern "C" fn set_frame_size(this: &Object, _: Sel, size: NSSize) {
    let window_state = unsafe { get_window_state(this) };
    let mut lock = window_state.as_ref().lock();

    if lock.content_size() == size.into() {
        return;
    }

    unsafe {
        let _: () = msg_send![super(this, class!(NSView)), setFrameSize: size];
    }

    let scale_factor = lock.scale_factor() as f64;
    let drawable_size: NSSize = NSSize {
        width: size.width * scale_factor,
        height: size.height * scale_factor,
    };

    lock.update_drawable_size(drawable_size);

    drop(lock);
    let mut lock = window_state.lock();
    if let Some(mut callback) = lock.resize_callback.take() {
        let content_size = lock.content_size();
        let scale_factor = lock.scale_factor();
        drop(lock);
        callback(content_size, scale_factor);
        window_state.lock().resize_callback = Some(callback);
    };
}

extern "C" fn display_layer(this: &Object, _: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    let mut lock = window_state.lock();
    if let Some(mut callback) = lock.request_frame_callback.take() {
        #[cfg(not(feature = "macos-blade"))]
        lock.renderer.set_presents_with_transaction(true);
        lock.stop_display_link();
        drop(lock);
        callback();

        let mut lock = window_state.lock();
        lock.request_frame_callback = Some(callback);
        #[cfg(not(feature = "macos-blade"))]
        lock.renderer.set_presents_with_transaction(false);
        lock.start_display_link();
    }
}

unsafe extern "C" fn step(view: *mut c_void) {
    let view = view as id;
    let window_state = unsafe { get_window_state(&*view) };
    let mut lock = window_state.lock();

    if let Some(mut callback) = lock.request_frame_callback.take() {
        drop(lock);
        callback();
        window_state.lock().request_frame_callback = Some(callback);
    }
}

extern "C" fn valid_attributes_for_marked_text(_: &Object, _: Sel) -> id {
    unsafe { msg_send![class!(NSArray), array] }
}

extern "C" fn has_marked_text(this: &Object, _: Sel) -> BOOL {
    with_input_handler(this, |input_handler| input_handler.marked_text_range())
        .flatten()
        .is_some() as BOOL
}

extern "C" fn marked_range(this: &Object, _: Sel) -> NSRange {
    with_input_handler(this, |input_handler| input_handler.marked_text_range())
        .flatten()
        .map_or(NSRange::invalid(), |range| range.into())
}

extern "C" fn selected_range(this: &Object, _: Sel) -> NSRange {
    with_input_handler(this, |input_handler| input_handler.selected_text_range())
        .flatten()
        .map_or(NSRange::invalid(), |range| range.into())
}

extern "C" fn first_rect_for_character_range(
    this: &Object,
    _: Sel,
    range: NSRange,
    _: id,
) -> NSRect {
    let frame = unsafe {
        let window = get_window_state(this).lock().native_window;
        NSView::frame(window)
    };
    with_input_handler(this, |input_handler| {
        input_handler.bounds_for_range(range.to_range()?)
    })
    .flatten()
    .map_or(
        NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.)),
        |bounds| {
            NSRect::new(
                NSPoint::new(
                    frame.origin.x + bounds.origin.x.0 as f64,
                    frame.origin.y + frame.size.height
                        - bounds.origin.y.0 as f64
                        - bounds.size.height.0 as f64,
                ),
                NSSize::new(bounds.size.width.0 as f64, bounds.size.height.0 as f64),
            )
        },
    )
}

extern "C" fn insert_text(this: &Object, _: Sel, text: id, replacement_range: NSRange) {
    unsafe {
        let is_attributed_string: BOOL =
            msg_send![text, isKindOfClass: [class!(NSAttributedString)]];
        let text: id = if is_attributed_string == YES {
            msg_send![text, string]
        } else {
            text
        };
        let text = CStr::from_ptr(text.UTF8String() as *mut c_char)
            .to_str()
            .unwrap();
        let replacement_range = replacement_range.to_range();
        send_to_input_handler(
            this,
            ImeInput::InsertText(text.to_string(), replacement_range),
        );
    }
}

extern "C" fn set_marked_text(
    this: &Object,
    _: Sel,
    text: id,
    selected_range: NSRange,
    replacement_range: NSRange,
) {
    unsafe {
        let is_attributed_string: BOOL =
            msg_send![text, isKindOfClass: [class!(NSAttributedString)]];
        let text: id = if is_attributed_string == YES {
            msg_send![text, string]
        } else {
            text
        };
        let selected_range = selected_range.to_range();
        let replacement_range = replacement_range.to_range();
        let text = CStr::from_ptr(text.UTF8String() as *mut c_char)
            .to_str()
            .unwrap();

        send_to_input_handler(
            this,
            ImeInput::SetMarkedText(text.to_string(), replacement_range, selected_range),
        );
    }
}
extern "C" fn unmark_text(this: &Object, _: Sel) {
    send_to_input_handler(this, ImeInput::UnmarkText);
}

extern "C" fn attributed_substring_for_proposed_range(
    this: &Object,
    _: Sel,
    range: NSRange,
    _actual_range: *mut c_void,
) -> id {
    with_input_handler(this, |input_handler| {
        let range = range.to_range()?;
        if range.is_empty() {
            return None;
        }

        let selected_text = input_handler.text_for_range(range)?;
        unsafe {
            let string: id = msg_send![class!(NSAttributedString), alloc];
            let string: id = msg_send![string, initWithString: ns_string(&selected_text)];
            Some(string)
        }
    })
    .flatten()
    .unwrap_or(nil)
}

extern "C" fn do_command_by_selector(_: &Object, _: Sel, _: Sel) {}

extern "C" fn view_did_change_effective_appearance(this: &Object, _: Sel) {
    unsafe {
        let state = get_window_state(this);
        let mut lock = state.as_ref().lock();
        if let Some(mut callback) = lock.appearance_changed_callback.take() {
            drop(lock);
            callback();
            state.lock().appearance_changed_callback = Some(callback);
        }
    }
}

extern "C" fn accepts_first_mouse(this: &Object, _: Sel, _: id) -> BOOL {
    unsafe {
        let state = get_window_state(this);
        let lock = state.as_ref().lock();
        if lock.kind == WindowKind::PopUp {
            YES
        } else {
            NO
        }
    }
}

extern "C" fn dragging_entered(this: &Object, _: Sel, dragging_info: id) -> NSDragOperation {
    let window_state = unsafe { get_window_state(this) };
    if send_new_event(&window_state, {
        let position = drag_event_position(&window_state, dragging_info);
        let paths = external_paths_from_event(dragging_info);
        PlatformInput::FileDrop(FileDropEvent::Entered { position, paths })
    }) {
        window_state.lock().external_files_dragged = true;
        NSDragOperationCopy
    } else {
        NSDragOperationNone
    }
}

extern "C" fn dragging_updated(this: &Object, _: Sel, dragging_info: id) -> NSDragOperation {
    let window_state = unsafe { get_window_state(this) };
    let position = drag_event_position(&window_state, dragging_info);
    if send_new_event(
        &window_state,
        PlatformInput::FileDrop(FileDropEvent::Pending { position }),
    ) {
        NSDragOperationCopy
    } else {
        NSDragOperationNone
    }
}

extern "C" fn dragging_exited(this: &Object, _: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    send_new_event(
        &window_state,
        PlatformInput::FileDrop(FileDropEvent::Exited),
    );
    window_state.lock().external_files_dragged = false;
}

extern "C" fn perform_drag_operation(this: &Object, _: Sel, dragging_info: id) -> BOOL {
    let window_state = unsafe { get_window_state(this) };
    let position = drag_event_position(&window_state, dragging_info);
    if send_new_event(
        &window_state,
        PlatformInput::FileDrop(FileDropEvent::Submit { position }),
    ) {
        YES
    } else {
        NO
    }
}

fn external_paths_from_event(dragging_info: *mut Object) -> ExternalPaths {
    let mut paths = SmallVec::new();
    let pasteboard: id = unsafe { msg_send![dragging_info, draggingPasteboard] };
    let filenames = unsafe { NSPasteboard::propertyListForType(pasteboard, NSFilenamesPboardType) };
    for file in unsafe { filenames.iter() } {
        let path = unsafe {
            let f = NSString::UTF8String(file);
            CStr::from_ptr(f).to_string_lossy().into_owned()
        };
        paths.push(PathBuf::from(path))
    }
    ExternalPaths(paths)
}

extern "C" fn conclude_drag_operation(this: &Object, _: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    send_new_event(
        &window_state,
        PlatformInput::FileDrop(FileDropEvent::Exited),
    );
}

async fn synthetic_drag(
    window_state: Weak<Mutex<MacWindowState>>,
    drag_id: usize,
    event: MouseMoveEvent,
) {
    loop {
        Timer::after(Duration::from_millis(16)).await;
        if let Some(window_state) = window_state.upgrade() {
            let mut lock = window_state.lock();
            if lock.synthetic_drag_counter == drag_id {
                if let Some(mut callback) = lock.event_callback.take() {
                    drop(lock);
                    callback(PlatformInput::MouseMove(event.clone()));
                    window_state.lock().event_callback = Some(callback);
                }
            } else {
                break;
            }
        }
    }
}

fn send_new_event(window_state_lock: &Mutex<MacWindowState>, e: PlatformInput) -> bool {
    let window_state = window_state_lock.lock().event_callback.take();
    if let Some(mut callback) = window_state {
        callback(e);
        window_state_lock.lock().event_callback = Some(callback);
        true
    } else {
        false
    }
}

fn drag_event_position(window_state: &Mutex<MacWindowState>, dragging_info: id) -> Point<Pixels> {
    let drag_location: NSPoint = unsafe { msg_send![dragging_info, draggingLocation] };
    convert_mouse_position(drag_location, window_state.lock().content_size().height)
}

fn with_input_handler<F, R>(window: &Object, f: F) -> Option<R>
where
    F: FnOnce(&mut PlatformInputHandler) -> R,
{
    let window_state = unsafe { get_window_state(window) };
    let mut lock = window_state.as_ref().lock();
    if let Some(mut input_handler) = lock.input_handler.take() {
        drop(lock);
        let result = f(&mut input_handler);
        window_state.lock().input_handler = Some(input_handler);
        Some(result)
    } else {
        None
    }
}

fn send_to_input_handler(window: &Object, ime: ImeInput) {
    unsafe {
        let window_state = get_window_state(window);
        let mut lock = window_state.lock();
        if let Some(ime_input) = lock.input_during_keydown.as_mut() {
            ime_input.push(ime);
            return;
        }
        if let Some(mut input_handler) = lock.input_handler.take() {
            drop(lock);
            match ime {
                ImeInput::InsertText(text, range) => {
                    input_handler.replace_text_in_range(range, &text)
                }
                ImeInput::SetMarkedText(text, range, marked_range) => {
                    input_handler.replace_and_mark_text_in_range(range, &text, marked_range)
                }
                ImeInput::UnmarkText => input_handler.unmark_text(),
            }
            window_state.lock().input_handler = Some(input_handler);
        }
    }
}

unsafe fn display_id_for_screen(screen: id) -> CGDirectDisplayID {
    let device_description = NSScreen::deviceDescription(screen);
    let screen_number_key: id = NSString::alloc(nil).init_str("NSScreenNumber");
    let screen_number = device_description.objectForKey_(screen_number_key);
    let screen_number: NSUInteger = msg_send![screen_number, unsignedIntegerValue];
    screen_number as CGDirectDisplayID
}
