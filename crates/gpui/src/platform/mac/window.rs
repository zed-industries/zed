use crate::{
    executor,
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    keymap_matcher::Keystroke,
    platform::{
        self,
        mac::{
            platform::NSViewLayerContentsRedrawDuringViewResize, renderer::Renderer, screen::Screen,
        },
        Event, InputHandler, KeyDownEvent, ModifiersChangedEvent, MouseButton, MouseButtonEvent,
        MouseMovedEvent, Scene, WindowBounds, WindowKind,
    },
};
use block::ConcreteBlock;
use cocoa::{
    appkit::{
        CGPoint, NSApplication, NSBackingStoreBuffered, NSScreen, NSView, NSViewHeightSizable,
        NSViewWidthSizable, NSWindow, NSWindowButton, NSWindowCollectionBehavior,
        NSWindowStyleMask, NSWindowTitleVisibility,
    },
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSInteger, NSPoint, NSRect, NSSize, NSString, NSUInteger},
};
use core_graphics::display::CGRect;
use ctor::ctor;
use foreign_types::ForeignTypeRef;
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
    ffi::{c_void, CStr},
    mem,
    ops::Range,
    os::raw::c_char,
    ptr,
    rc::{Rc, Weak},
    sync::Arc,
    time::Duration,
};

use super::{
    geometry::{NSRectExt, Vector2FExt},
    ns_string, NSRange,
};

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
        sel!(sendEvent:),
        send_event as extern "C" fn(&Object, Sel, id),
    );
    decl.add_method(
        sel!(windowDidResize:),
        window_did_resize as extern "C" fn(&Object, Sel, id),
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
}

///Used to track what the IME does when we send it a keystroke.
///This is only used to handle the case where the IME mysteriously
///swallows certain keys.
///
///Basically a direct copy of the approach that WezTerm uses in:
///github.com/wez/wezterm : d5755f3e : window/src/os/macos/window.rs
enum ImeState {
    Continue,
    Acted,
    None,
}

struct InsertText {
    replacement_range: Option<Range<usize>>,
    text: String,
}

struct WindowState {
    id: usize,
    native_window: id,
    kind: WindowKind,
    event_callback: Option<Box<dyn FnMut(Event) -> bool>>,
    activate_callback: Option<Box<dyn FnMut(bool)>>,
    resize_callback: Option<Box<dyn FnMut()>>,
    fullscreen_callback: Option<Box<dyn FnMut(bool)>>,
    moved_callback: Option<Box<dyn FnMut()>>,
    should_close_callback: Option<Box<dyn FnMut() -> bool>>,
    close_callback: Option<Box<dyn FnOnce()>>,
    appearance_changed_callback: Option<Box<dyn FnMut()>>,
    input_handler: Option<Box<dyn InputHandler>>,
    pending_key_down: Option<(KeyDownEvent, Option<InsertText>)>,
    performed_key_equivalent: bool,
    synthetic_drag_counter: usize,
    executor: Rc<executor::Foreground>,
    scene_to_render: Option<Scene>,
    renderer: Renderer,
    last_fresh_keydown: Option<Keystroke>,
    traffic_light_position: Option<Vector2F>,
    previous_modifiers_changed_event: Option<Event>,
    //State tracking what the IME did after the last request
    ime_state: ImeState,
    //Retains the last IME Text
    ime_text: Option<String>,
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

            let window_frame = self.frame();
            let screen_frame = self.native_window.screen().visibleFrame().to_rectf();
            if window_frame.size() == screen_frame.size() {
                WindowBounds::Maximized
            } else {
                WindowBounds::Fixed(window_frame)
            }
        }
    }

    // Returns the window bounds in window coordinates
    fn frame(&self) -> RectF {
        unsafe {
            let screen_frame = self.native_window.screen().visibleFrame();
            let window_frame = NSWindow::frame(self.native_window);
            RectF::new(
                vec2f(
                    window_frame.origin.x as f32,
                    (screen_frame.size.height - window_frame.origin.y - window_frame.size.height)
                        as f32,
                ),
                vec2f(
                    window_frame.size.width as f32,
                    window_frame.size.height as f32,
                ),
            )
        }
    }

    fn content_size(&self) -> Vector2F {
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

pub struct Window(Rc<RefCell<WindowState>>);

impl Window {
    pub fn open(
        id: usize,
        options: platform::WindowOptions,
        executor: Rc<executor::Foreground>,
        fonts: Arc<dyn platform::FontSystem>,
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
            let native_window = native_window.initWithContentRect_styleMask_backing_defer_screen_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(1024., 768.)),
                style_mask,
                NSBackingStoreBuffered,
                NO,
                options
                    .screen
                    .and_then(|screen| {
                        Some(screen.as_any().downcast_ref::<Screen>()?.native_screen)
                    })
                    .unwrap_or(nil),
            );
            assert!(!native_window.is_null());

            let screen = native_window.screen();
            match options.bounds {
                WindowBounds::Fullscreen => {
                    native_window.toggleFullScreen_(nil);
                }
                WindowBounds::Maximized => {
                    native_window.setFrame_display_(screen.visibleFrame(), YES);
                }
                WindowBounds::Fixed(rect) => {
                    let screen_frame = screen.visibleFrame();
                    let ns_rect = NSRect::new(
                        NSPoint::new(
                            rect.origin_x() as f64,
                            screen_frame.size.height
                                - rect.origin_y() as f64
                                - rect.height() as f64,
                        ),
                        NSSize::new(rect.width() as f64, rect.height() as f64),
                    );

                    if ns_rect.intersects(screen_frame) {
                        native_window.setFrame_display_(ns_rect, YES);
                    } else {
                        native_window.setFrame_display_(screen_frame, YES);
                    }
                }
            }

            let native_view: id = msg_send![VIEW_CLASS, alloc];
            let native_view = NSView::init(native_view);

            assert!(!native_view.is_null());

            let window = Self(Rc::new(RefCell::new(WindowState {
                id,
                native_window,
                kind: options.kind,
                event_callback: None,
                resize_callback: None,
                should_close_callback: None,
                close_callback: None,
                activate_callback: None,
                fullscreen_callback: None,
                moved_callback: None,
                appearance_changed_callback: None,
                input_handler: None,
                pending_key_down: None,
                performed_key_equivalent: false,
                synthetic_drag_counter: 0,
                executor,
                scene_to_render: Default::default(),
                renderer: Renderer::new(true, fonts),
                last_fresh_keydown: None,
                traffic_light_position: options
                    .titlebar
                    .as_ref()
                    .and_then(|titlebar| titlebar.traffic_light_position),
                previous_modifiers_changed_event: None,
                ime_state: ImeState::None,
                ime_text: None,
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

            if let Some(title) = options.titlebar.as_ref().and_then(|t| t.title) {
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
            } else {
                native_window.orderFront_(nil);
            }

            window.0.borrow().move_traffic_light();
            pool.drain();

            window
        }
    }

    pub fn main_window_id() -> Option<usize> {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let main_window: id = msg_send![app, mainWindow];
            if msg_send![main_window, isKindOfClass: WINDOW_CLASS] {
                let id = get_window_state(&*main_window).borrow().id;
                Some(id)
            } else {
                None
            }
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let this = self.0.borrow();
        let window = this.native_window;
        this.executor
            .spawn(async move {
                unsafe {
                    window.close();
                }
            })
            .detach();
    }
}

impl platform::Window for Window {
    fn bounds(&self) -> WindowBounds {
        self.0.as_ref().borrow().bounds()
    }

    fn content_size(&self) -> Vector2F {
        self.0.as_ref().borrow().content_size()
    }

    fn scale_factor(&self) -> f32 {
        self.0.as_ref().borrow().scale_factor()
    }

    fn titlebar_height(&self) -> f32 {
        self.0.as_ref().borrow().titlebar_height()
    }

    fn appearance(&self) -> platform::Appearance {
        unsafe {
            let appearance: id = msg_send![self.0.borrow().native_window, effectiveAppearance];
            platform::Appearance::from_native(appearance)
        }
    }

    fn screen(&self) -> Rc<dyn platform::Screen> {
        unsafe {
            Rc::new(Screen {
                native_screen: self.0.as_ref().borrow().native_window.screen(),
            })
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_input_handler(&mut self, input_handler: Box<dyn InputHandler>) {
        self.0.as_ref().borrow_mut().input_handler = Some(input_handler);
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
            for (ix, answer) in answers.iter().enumerate() {
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
            self.0
                .borrow()
                .executor
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
        let window = self.0.borrow().native_window;
        self.0
            .borrow()
            .executor
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
            let window = self.0.borrow().native_window;
            let title = ns_string(title);
            let _: () = msg_send![app, changeWindowsItem:window title:title filename:false];
            let _: () = msg_send![window, setTitle: title];
            self.0.borrow().move_traffic_light();
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

    fn show_character_palette(&self) {
        unsafe {
            let app = NSApplication::sharedApplication(nil);
            let window = self.0.borrow().native_window;
            let _: () = msg_send![app, orderFrontCharacterPalette: window];
        }
    }

    fn minimize(&self) {
        let window = self.0.borrow().native_window;
        unsafe {
            window.miniaturize_(nil);
        }
    }

    fn zoom(&self) {
        let this = self.0.borrow();
        let window = this.native_window;
        this.executor
            .spawn(async move {
                unsafe {
                    window.zoom_(nil);
                }
            })
            .detach();
    }

    fn present_scene(&mut self, scene: Scene) {
        self.0.as_ref().borrow_mut().present_scene(scene);
    }

    fn toggle_full_screen(&self) {
        let this = self.0.borrow();
        let window = this.native_window;
        this.executor
            .spawn(async move {
                unsafe {
                    window.toggleFullScreen_(nil);
                }
            })
            .detach();
    }

    fn on_event(&mut self, callback: Box<dyn FnMut(Event) -> bool>) {
        self.0.as_ref().borrow_mut().event_callback = Some(callback);
    }

    fn on_active_status_change(&mut self, callback: Box<dyn FnMut(bool)>) {
        self.0.as_ref().borrow_mut().activate_callback = Some(callback);
    }

    fn on_resize(&mut self, callback: Box<dyn FnMut()>) {
        self.0.as_ref().borrow_mut().resize_callback = Some(callback);
    }

    fn on_fullscreen(&mut self, callback: Box<dyn FnMut(bool)>) {
        self.0.as_ref().borrow_mut().fullscreen_callback = Some(callback);
    }

    fn on_moved(&mut self, callback: Box<dyn FnMut()>) {
        self.0.as_ref().borrow_mut().moved_callback = Some(callback);
    }

    fn on_should_close(&mut self, callback: Box<dyn FnMut() -> bool>) {
        self.0.as_ref().borrow_mut().should_close_callback = Some(callback);
    }

    fn on_close(&mut self, callback: Box<dyn FnOnce()>) {
        self.0.as_ref().borrow_mut().close_callback = Some(callback);
    }

    fn on_appearance_changed(&mut self, callback: Box<dyn FnMut()>) {
        self.0.borrow_mut().appearance_changed_callback = Some(callback);
    }

    fn is_topmost_for_position(&self, position: Vector2F) -> bool {
        let self_borrow = self.0.borrow();
        let self_id = self_borrow.id;

        unsafe {
            let app = NSApplication::sharedApplication(nil);

            // Convert back to screen coordinates
            let screen_point = position.to_screen_ns_point(
                self_borrow.native_window,
                self_borrow.content_size().y() as f64,
            );

            let window_number: NSInteger = msg_send![class!(NSWindow), windowNumberAtPoint:screen_point belowWindowWithWindowNumber:0];
            let top_most_window: id = msg_send![app, windowWithWindowNumber: window_number];

            let is_panel: BOOL = msg_send![top_most_window, isKindOfClass: PANEL_CLASS];
            let is_window: BOOL = msg_send![top_most_window, isKindOfClass: WINDOW_CLASS];
            if is_panel == YES || is_window == YES {
                let topmost_window_id = get_window_state(&*top_most_window).borrow().id;
                topmost_window_id == self_id
            } else {
                // Someone else's window is on top
                false
            }
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

extern "C" fn handle_key_event(this: &Object, native_event: id, key_equivalent: bool) -> BOOL {
    let window_state = unsafe { get_window_state(this) };
    let mut window_state_borrow = window_state.as_ref().borrow_mut();

    let window_height = window_state_borrow.content_size().y();
    let event = unsafe { Event::from_native(native_event, Some(window_height)) };

    if let Some(event) = event {
        if key_equivalent {
            window_state_borrow.performed_key_equivalent = true;
        } else if window_state_borrow.performed_key_equivalent {
            return NO;
        }

        let function_is_held;
        window_state_borrow.pending_key_down = match event {
            Event::KeyDown(event) => {
                let keydown = event.keystroke.clone();
                // Ignore events from held-down keys after some of the initially-pressed keys
                // were released.
                if event.is_held {
                    if window_state_borrow.last_fresh_keydown.as_ref() != Some(&keydown) {
                        return YES;
                    }
                } else {
                    window_state_borrow.last_fresh_keydown = Some(keydown);
                }
                function_is_held = event.keystroke.function;
                Some((event, None))
            }

            _ => return NO,
        };

        drop(window_state_borrow);

        if !function_is_held {
            unsafe {
                let input_context: id = msg_send![this, inputContext];
                let _: BOOL = msg_send![input_context, handleEvent: native_event];
            }
        }

        let mut handled = false;
        let mut window_state_borrow = window_state.borrow_mut();
        let ime_text = window_state_borrow.ime_text.clone();
        if let Some((event, insert_text)) = window_state_borrow.pending_key_down.take() {
            let is_held = event.is_held;
            if let Some(mut callback) = window_state_borrow.event_callback.take() {
                drop(window_state_borrow);

                let is_composing =
                    with_input_handler(this, |input_handler| input_handler.marked_text_range())
                        .flatten()
                        .is_some();
                if !is_composing {
                    handled = callback(Event::KeyDown(event));
                }

                if !handled {
                    if let Some(insert) = insert_text {
                        handled = true;
                        with_input_handler(this, |input_handler| {
                            input_handler
                                .replace_text_in_range(insert.replacement_range, &insert.text)
                        });
                    } else if !is_composing && is_held {
                        if let Some(last_insert_text) = ime_text {
                            //MacOS IME is a bit funky, and even when you've told it there's nothing to
                            //inter it will still swallow certain keys (e.g. 'f', 'j') and not others
                            //(e.g. 'n'). This is a problem for certain kinds of views, like the terminal
                            with_input_handler(this, |input_handler| {
                                if input_handler.selected_text_range().is_none() {
                                    handled = true;
                                    input_handler.replace_text_in_range(None, &last_insert_text)
                                }
                            });
                        }
                    }
                }

                window_state.borrow_mut().event_callback = Some(callback);
            }
        } else {
            handled = true;
        }

        handled as BOOL
    } else {
        NO
    }
}

extern "C" fn handle_view_event(this: &Object, _: Sel, native_event: id) {
    let window_state = unsafe { get_window_state(this) };
    let weak_window_state = Rc::downgrade(&window_state);
    let mut window_state_borrow = window_state.as_ref().borrow_mut();
    let is_active = unsafe { window_state_borrow.native_window.isKeyWindow() == YES };

    let window_height = window_state_borrow.content_size().y();
    let event = unsafe { Event::from_native(native_event, Some(window_height)) };
    if let Some(event) = event {
        match &event {
            Event::MouseMoved(
                event @ MouseMovedEvent {
                    pressed_button: Some(_),
                    ..
                },
            ) => {
                window_state_borrow.synthetic_drag_counter += 1;
                window_state_borrow
                    .executor
                    .spawn(synthetic_drag(
                        weak_window_state,
                        window_state_borrow.synthetic_drag_counter,
                        *event,
                    ))
                    .detach();
            }

            Event::MouseMoved(_)
                if !(is_active || window_state_borrow.kind == WindowKind::PopUp) =>
            {
                return
            }

            Event::MouseUp(MouseButtonEvent {
                button: MouseButton::Left,
                ..
            }) => {
                window_state_borrow.synthetic_drag_counter += 1;
            }

            Event::ModifiersChanged(ModifiersChangedEvent { modifiers }) => {
                // Only raise modifiers changed event when they have actually changed
                if let Some(Event::ModifiersChanged(ModifiersChangedEvent {
                    modifiers: prev_modifiers,
                })) = &window_state_borrow.previous_modifiers_changed_event
                {
                    if prev_modifiers == modifiers {
                        return;
                    }
                }

                window_state_borrow.previous_modifiers_changed_event = Some(event.clone());
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

    let keystroke = Keystroke {
        cmd: true,
        ctrl: false,
        alt: false,
        shift: false,
        function: false,
        key: ".".into(),
    };
    let event = Event::KeyDown(KeyDownEvent {
        keystroke: keystroke.clone(),
        is_held: false,
    });

    window_state_borrow.last_fresh_keydown = Some(keystroke);
    if let Some(mut callback) = window_state_borrow.event_callback.take() {
        drop(window_state_borrow);
        callback(event);
        window_state.borrow_mut().event_callback = Some(callback);
    }
}

extern "C" fn send_event(this: &Object, _: Sel, native_event: id) {
    unsafe {
        let _: () = msg_send![super(this, class!(NSWindow)), sendEvent: native_event];
        get_window_state(this).borrow_mut().performed_key_equivalent = false;
    }
}

extern "C" fn window_did_resize(this: &Object, _: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    window_state.as_ref().borrow().move_traffic_light();
}

extern "C" fn window_will_enter_fullscreen(this: &Object, _: Sel, _: id) {
    window_fullscreen_changed(this, true);
}

extern "C" fn window_will_exit_fullscreen(this: &Object, _: Sel, _: id) {
    window_fullscreen_changed(this, false);
}

fn window_fullscreen_changed(this: &Object, is_fullscreen: bool) {
    let window_state = unsafe { get_window_state(this) };
    let mut window_state_borrow = window_state.as_ref().borrow_mut();
    if let Some(mut callback) = window_state_borrow.fullscreen_callback.take() {
        drop(window_state_borrow);
        callback(is_fullscreen);
        window_state.borrow_mut().fullscreen_callback = Some(callback);
    }
}

extern "C" fn window_did_move(this: &Object, _: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    let mut window_state_borrow = window_state.as_ref().borrow_mut();
    if let Some(mut callback) = window_state_borrow.moved_callback.take() {
        drop(window_state_borrow);
        callback();
        window_state.borrow_mut().moved_callback = Some(callback);
    }
}

extern "C" fn window_did_change_key_status(this: &Object, selector: Sel, _: id) {
    let window_state = unsafe { get_window_state(this) };
    let window_state_borrow = window_state.borrow();
    let is_active = unsafe { window_state_borrow.native_window.isKeyWindow() == YES };

    // When opening a pop-up while the application isn't active, Cocoa sends a spurious
    // `windowDidBecomeKey` message to the previous key window even though that window
    // isn't actually key. This causes a bug if the application is later activated while
    // the pop-up is still open, making it impossible to activate the previous key window
    // even if the pop-up gets closed. The only way to activate it again is to de-activate
    // the app and re-activate it, which is a pretty bad UX.
    // The following code detects the spurious event and invokes `resignKeyWindow`:
    // in theory, we're not supposed to invoke this method manually but it balances out
    // the spurious `becomeKeyWindow` event and helps us work around that bug.
    if selector == sel!(windowDidBecomeKey:) {
        if !is_active {
            unsafe {
                let _: () = msg_send![window_state_borrow.native_window, resignKeyWindow];
                return;
            }
        }
    }

    let executor = window_state_borrow.executor.clone();
    drop(window_state_borrow);
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

        let _: () = msg_send![super(this, class!(NSWindow)), close];
    }
}

extern "C" fn make_backing_layer(this: &Object, _: Sel) -> id {
    let window_state = unsafe { get_window_state(this) };
    let window_state = window_state.as_ref().borrow();
    window_state.renderer.layer().as_ptr() as id
}

extern "C" fn view_did_change_backing_properties(this: &Object, _: Sel) {
    let window_state = unsafe { get_window_state(this) };
    let mut window_state_borrow = window_state.as_ref().borrow_mut();

    unsafe {
        let scale_factor = window_state_borrow.scale_factor() as f64;
        let size = window_state_borrow.content_size();
        let drawable_size: NSSize = NSSize {
            width: size.x() as f64 * scale_factor,
            height: size.y() as f64 * scale_factor,
        };

        let _: () = msg_send![
            window_state_borrow.renderer.layer(),
            setContentsScale: scale_factor
        ];
        let _: () = msg_send![
            window_state_borrow.renderer.layer(),
            setDrawableSize: drawable_size
        ];
    }

    if let Some(mut callback) = window_state_borrow.resize_callback.take() {
        drop(window_state_borrow);
        callback();
        window_state.as_ref().borrow_mut().resize_callback = Some(callback);
    };
}

extern "C" fn set_frame_size(this: &Object, _: Sel, size: NSSize) {
    let window_state = unsafe { get_window_state(this) };
    let window_state_borrow = window_state.as_ref().borrow();

    if window_state_borrow.content_size() == vec2f(size.width as f32, size.height as f32) {
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
        let _: () = msg_send![
            window_state_borrow.renderer.layer(),
            setDrawableSize: drawable_size
        ];
    }

    drop(window_state_borrow);
    let mut window_state_borrow = window_state.borrow_mut();
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
            window_state.renderer.render(&scene);
        };
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
        let window = get_window_state(this).borrow().native_window;
        NSView::frame(window)
    };
    with_input_handler(this, |input_handler| {
        input_handler.rect_for_range(range.to_range()?)
    })
    .flatten()
    .map_or(
        NSRect::new(NSPoint::new(0., 0.), NSSize::new(0., 0.)),
        |rect| {
            NSRect::new(
                NSPoint::new(
                    frame.origin.x + rect.origin_x() as f64,
                    frame.origin.y + frame.size.height - rect.origin_y() as f64,
                ),
                NSSize::new(rect.width() as f64, rect.height() as f64),
            )
        },
    )
}

extern "C" fn insert_text(this: &Object, _: Sel, text: id, replacement_range: NSRange) {
    unsafe {
        let window_state = get_window_state(this);
        let mut window_state_borrow = window_state.borrow_mut();
        let pending_key_down = window_state_borrow.pending_key_down.take();
        drop(window_state_borrow);

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

        window_state.borrow_mut().ime_text = Some(text.to_string());
        window_state.borrow_mut().ime_state = ImeState::Acted;

        let is_composing =
            with_input_handler(this, |input_handler| input_handler.marked_text_range())
                .flatten()
                .is_some();

        if is_composing || text.chars().count() > 1 || pending_key_down.is_none() {
            with_input_handler(this, |input_handler| {
                input_handler.replace_text_in_range(replacement_range, text)
            });
        } else {
            let mut pending_key_down = pending_key_down.unwrap();
            pending_key_down.1 = Some(InsertText {
                replacement_range,
                text: text.to_string(),
            });
            window_state.borrow_mut().pending_key_down = Some(pending_key_down);
        }
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
        let window_state = get_window_state(this);
        window_state.borrow_mut().pending_key_down.take();

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

        window_state.borrow_mut().ime_state = ImeState::Acted;
        window_state.borrow_mut().ime_text = Some(text.to_string());

        with_input_handler(this, |input_handler| {
            input_handler.replace_and_mark_text_in_range(replacement_range, text, selected_range);
        });
    }
}

extern "C" fn unmark_text(this: &Object, _: Sel) {
    unsafe {
        let state = get_window_state(this);
        let mut borrow = state.borrow_mut();
        borrow.ime_state = ImeState::Acted;
        borrow.ime_text.take();
    }

    with_input_handler(this, |input_handler| input_handler.unmark_text());
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

extern "C" fn do_command_by_selector(this: &Object, _: Sel, _: Sel) {
    unsafe {
        let state = get_window_state(this);
        let mut borrow = state.borrow_mut();
        borrow.ime_state = ImeState::Continue;
        borrow.ime_text.take();
    }
}

extern "C" fn view_did_change_effective_appearance(this: &Object, _: Sel) {
    unsafe {
        let state = get_window_state(this);
        let mut state_borrow = state.as_ref().borrow_mut();
        if let Some(mut callback) = state_borrow.appearance_changed_callback.take() {
            drop(state_borrow);
            callback();
            state.borrow_mut().appearance_changed_callback = Some(callback);
        }
    }
}

extern "C" fn accepts_first_mouse(this: &Object, _: Sel, _: id) -> BOOL {
    unsafe {
        let state = get_window_state(this);
        let state_borrow = state.as_ref().borrow();
        return if state_borrow.kind == WindowKind::PopUp {
            YES
        } else {
            NO
        };
    }
}

async fn synthetic_drag(
    window_state: Weak<RefCell<WindowState>>,
    drag_id: usize,
    event: MouseMovedEvent,
) {
    loop {
        Timer::after(Duration::from_millis(16)).await;
        if let Some(window_state) = window_state.upgrade() {
            let mut window_state_borrow = window_state.borrow_mut();
            if window_state_borrow.synthetic_drag_counter == drag_id {
                if let Some(mut callback) = window_state_borrow.event_callback.take() {
                    drop(window_state_borrow);
                    callback(Event::MouseMoved(event));
                    window_state.borrow_mut().event_callback = Some(callback);
                }
            } else {
                break;
            }
        }
    }
}

fn with_input_handler<F, R>(window: &Object, f: F) -> Option<R>
where
    F: FnOnce(&mut dyn InputHandler) -> R,
{
    let window_state = unsafe { get_window_state(window) };
    let mut window_state_borrow = window_state.as_ref().borrow_mut();
    if let Some(mut input_handler) = window_state_borrow.input_handler.take() {
        drop(window_state_borrow);
        let result = f(input_handler.as_mut());
        window_state.borrow_mut().input_handler = Some(input_handler);
        Some(result)
    } else {
        None
    }
}
