//! Shows that an AppKit control outside GPUI keeps keyboard input beside a native view.
//!
//! The window holds a `WKWebView` and, in its top right corner, an `NSTextView`
//! that GPUI knows nothing about. Click that field and type: the text goes into
//! it, and the line at the top says which of the three has keyboard input.
//!
//! A second after it opens, the example also gives the field keyboard input
//! itself and prints whether the field still had it half a second later.

#[cfg(target_os = "macos")]
#[path = "example_support/fonts.rs"]
mod example_support;

#[cfg(target_os = "macos")]
mod example {
    use gpui::{
        App, Context, Focusable as _, NativeView, Task, Window, WindowOptions, div, prelude::*, px,
        rgb,
    };
    use objc2::{
        class, msg_send,
        rc::{Allocated, Retained},
        runtime::{AnyObject, Bool},
    };
    use objc2_core_foundation::{CGPoint, CGRect, CGSize};
    use std::{
        ffi::{CStr, c_void},
        ptr::{self, NonNull},
        time::Duration,
    };

    #[link(name = "WebKit", kind = "framework")]
    unsafe extern "C" {}

    const PAGE: &CStr = cr#"<!doctype html>
<html>
  <body style="font: 15px -apple-system; margin: 24px">
    <p><input placeholder="A field in the page" size="36" autofocus /></p>
  </body>
</html>"#;

    #[derive(Clone, Copy, PartialEq)]
    enum KeyboardInput {
        AppKitField,
        Page,
        Gpui,
    }

    struct Example {
        web_view: NativeView,
        web_view_object: Retained<AnyObject>,
        appkit_field: Retained<AnyObject>,
        keyboard_input: KeyboardInput,
        _watch: Task<()>,
    }

    impl Example {
        fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
            // SAFETY: These are AppKit and WebKit classes used as documented, on
            // the main thread.
            let (web_view_object, appkit_field) = unsafe {
                let web_view: Allocated<AnyObject> = msg_send![class!(WKWebView), alloc];
                let web_view: Retained<AnyObject> = msg_send![web_view, init];
                let _: *mut AnyObject = msg_send![
                    &*web_view,
                    loadHTMLString: &*ns_string(PAGE),
                    baseURL: ptr::null::<AnyObject>()
                ];
                // A text view takes keyboard input itself, where a text field
                // would pass it to an editor that AppKit manages.
                let frame = CGRect::new(CGPoint::new(0., 0.), CGSize::new(220., 22.));
                let field: Allocated<AnyObject> = msg_send![class!(NSTextView), alloc];
                let field: Retained<AnyObject> = msg_send![field, initWithFrame: frame];
                let _: () = msg_send![&*field, setString: &*ns_string(c"An AppKit field")];
                (web_view, field)
            };
            // SAFETY: The pointer is the live web view, which is in no view hierarchy.
            let web_view = unsafe {
                NativeView::new(
                    NonNull::from(&*web_view_object).cast::<c_void>(),
                    window,
                    cx,
                )
            }
            .expect("this example only runs where native views are composited");
            // SAFETY: Attaching the web view put it in the GPUI window, whose
            // content view is live, on the main thread.
            unsafe {
                let ns_window: *mut AnyObject = msg_send![&*web_view_object, window];
                let content_view: *mut AnyObject = msg_send![ns_window, contentView];
                let bounds: CGRect = msg_send![content_view, bounds];
                let is_flipped: bool = msg_send![content_view, isFlipped];
                let size = CGSize::new(220., 22.);
                let top = if is_flipped {
                    8.
                } else {
                    bounds.size.height - size.height - 8.
                };
                let origin = CGPoint::new(bounds.size.width - size.width - 8., top);
                let _: () = msg_send![&*appkit_field, setFrame: CGRect::new(origin, size)];
                // Keeps the field in the corner as the window resizes:
                // `NSViewMinXMargin`, with `NSViewMaxYMargin` or `NSViewMinYMargin`.
                let mask: usize = if is_flipped { 1 | 32 } else { 1 | 8 };
                let _: () = msg_send![&*appkit_field, setAutoresizingMask: mask];
                let _: () = msg_send![content_view, addSubview: &*appkit_field];
            }
            window.focus(&web_view.focus_handle(cx), cx);

            let watch = cx.spawn(async move |this, cx| {
                let mut elapsed = Duration::ZERO;
                let step = Duration::from_millis(250);
                loop {
                    cx.background_executor().timer(step).await;
                    elapsed += step;
                    let updated = this.update(cx, |this, cx| {
                        if elapsed == Duration::from_millis(1000) {
                            this.focus_appkit_field();
                        }
                        let keyboard_input = this.keyboard_input();
                        if elapsed == Duration::from_millis(1500) {
                            println!(
                                "AppKit field kept keyboard input: {}",
                                keyboard_input == KeyboardInput::AppKitField
                            );
                        }
                        if this.keyboard_input != keyboard_input {
                            this.keyboard_input = keyboard_input;
                            cx.notify();
                        }
                    });
                    if updated.is_err() {
                        break;
                    }
                }
            });

            Self {
                web_view,
                web_view_object,
                appkit_field,
                keyboard_input: KeyboardInput::Gpui,
                _watch: watch,
            }
        }

        fn focus_appkit_field(&self) {
            // SAFETY: The field is live and in the window, on the main thread.
            unsafe {
                let ns_window: *mut AnyObject = msg_send![&*self.appkit_field, window];
                let accepted: Bool = msg_send![ns_window, makeFirstResponder: &*self.appkit_field];
                println!("AppKit field took keyboard input: {}", accepted.as_bool());
            }
        }

        /// Which view the window's first responder is, or is inside.
        fn keyboard_input(&self) -> KeyboardInput {
            // SAFETY: The views are live and in the window, on the main thread.
            unsafe {
                let ns_window: *mut AnyObject = msg_send![&*self.appkit_field, window];
                let responder: *mut AnyObject = msg_send![ns_window, firstResponder];
                let is_view: bool =
                    !responder.is_null() && msg_send![responder, isKindOfClass: class!(NSView)];
                let inside = |view: &AnyObject| -> bool {
                    is_view && msg_send![responder, isDescendantOf: view]
                };
                if inside(&self.appkit_field) {
                    KeyboardInput::AppKitField
                } else if inside(&self.web_view_object) {
                    KeyboardInput::Page
                } else {
                    KeyboardInput::Gpui
                }
            }
        }
    }

    impl Render for Example {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let keyboard_input = match self.keyboard_input {
                KeyboardInput::AppKitField => "the AppKit field",
                KeyboardInput::Page => "the page",
                KeyboardInput::Gpui => "GPUI",
            };
            div()
                .size_full()
                .flex()
                .flex_col()
                .bg(rgb(0x202428))
                .text_color(rgb(0xdddddd))
                .child(
                    div()
                        .p(px(8.))
                        .child(format!("Keyboard input is with {keyboard_input}.")),
                )
                .child(div().flex_1().child(self.web_view.clone()))
        }
    }

    fn ns_string(text: &CStr) -> Retained<AnyObject> {
        // SAFETY: `text` is nul-terminated, and `NSString` copies it.
        unsafe { msg_send![class!(NSString), stringWithUTF8String: text.as_ptr()] }
    }

    pub fn run() {
        gpui_platform::application().run(|cx: &mut App| {
            if !crate::example_support::load_fonts(cx) {
                return;
            }
            cx.open_window(WindowOptions::default(), |window, cx| {
                cx.new(|cx| Example::new(window, cx))
            })
            .expect("opening the example window");
            cx.activate(true);
        });
    }
}

#[cfg(target_os = "macos")]
fn main() {
    example::run();
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("Native views are only composited on macOS.");
}
