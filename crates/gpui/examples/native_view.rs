//! Shows a live `WKWebView` composited into a GPUI window on macOS.
//!
//! Things to try:
//! - Press `cmd-shift-p` while typing in the page. The palette opens across the page's
//!   edge, takes keyboard input, and `escape` returns the caret to the same page field.
//! - Click the palette's button where it overlaps the page, then scroll the exposed page.
//! - Click the page while the palette is open, which dismisses the palette.
//! - Click the GPUI text box, type, then click back into the page.
//! - Click an empty part of the page and press some keys. The page sees every key first,
//!   and the sidebar lists the ones it left for GPUI. It keeps `cmd-k` for itself, while
//!   `cmd-shift-p` is reserved, so it opens the palette whatever the page does.
//! - Drag the divider, then load the other page or toggle the page's colors, and drag it
//!   again. The strip that shows for a frame where the page is catching up takes the
//!   page's own background color.

#[cfg(target_os = "macos")]
#[path = "example_support/fonts.rs"]
mod example_support;

#[cfg(target_os = "macos")]
mod example {
    use futures::{StreamExt as _, channel::mpsc};
    use gpui::{
        App, Context, FocusHandle, KeyBinding, KeyDownEvent, MouseButton, MouseMoveEvent,
        NativeView, Pixels, Rgba, Task, Window, WindowOptions, actions, anchored, deferred, div,
        point, prelude::*, px, rgb,
    };
    use objc2::{
        class, msg_send,
        rc::{Allocated, Retained},
        runtime::{AnyClass, AnyObject, ClassBuilder, Sel},
        sel,
    };
    use std::{
        ffi::{CStr, c_char, c_void},
        ptr::{self, NonNull},
        sync::OnceLock,
    };

    #[link(name = "WebKit", kind = "framework")]
    unsafe extern "C" {}

    actions!(native_view_example, [TogglePalette, DismissPalette]);

    /// Runs in every page and reports its background color, so that GPUI can show
    /// that color wherever the web view is still catching up with its element.
    ///
    /// It reports again when the color changes, which a page does by restyling
    /// its root elements, changing its stylesheets, or following the system's
    /// appearance. Checks are limited to one per frame, and only run after one
    /// of those happens.
    const REPORT_BACKGROUND: &CStr = cr#"(function () {
  let reported = null;
  let scheduled = false;
  function report() {
    scheduled = false;
    const transparent = "rgba(0, 0, 0, 0)";
    let color = getComputedStyle(document.body).backgroundColor;
    if (color === transparent) {
      color = getComputedStyle(document.documentElement).backgroundColor;
    }
    if (color === transparent) {
      color = "rgb(255, 255, 255)";
    }
    if (color !== reported) {
      reported = color;
      window.webkit.messageHandlers.background.postMessage(color);
    }
  }
  function schedule() {
    if (!scheduled) {
      scheduled = true;
      requestAnimationFrame(report);
    }
  }
  report();
  window.addEventListener("load", schedule);
  const observer = new MutationObserver(schedule);
  observer.observe(document.documentElement, { attributes: true });
  observer.observe(document.body, { attributes: true });
  observer.observe(document.head, { attributes: true, childList: true, subtree: true });
  matchMedia("(prefers-color-scheme: dark)").addEventListener("change", schedule);
  for (const element of [document.documentElement, document.body]) {
    element.addEventListener("transitionend", schedule);
  }
})();"#;

    const OTHER_PAGE: &CStr = cr#"<!doctype html>
<html>
  <body style="font: 15px -apple-system; margin: 24px; color: #eee; background: #204060">
    <h2>Another page</h2>
    <p><input placeholder="Type here" size="36" /></p>
  </body>
</html>"#;

    const PAGE: &CStr = cr#"<!doctype html>
<html>
  <body style="font: 15px -apple-system; margin: 24px; color: #222; background: #fdf6e3">
    <h2>A live WKWebView</h2>
    <p>
      <input placeholder="Type here, then press cmd-shift-p" size="36" autofocus />
    </p>
    <p><textarea rows="3" cols="38">A textarea with some text to select.</textarea></p>
    <p contenteditable style="border: 1px solid #999; padding: 6px">An editable paragraph.</p>
    <p><a href="https://example.com">A link, for the pointing-hand cursor</a></p>
    <p id="kept">This page keeps cmd-k for itself.</p>
    <button onclick="this.textContent = 'Clicked ' + (++window.clicks || (window.clicks = 1))">
      A page button
    </button>
    <button onclick="document.body.style.background = window.dark ? '#fdf6e3' : '#30343a'; document.body.style.color = window.dark ? '#222' : '#ddd'; window.dark = !window.dark">
      Toggle the page's colors
    </button>
    <script>
      document.addEventListener("keydown", (event) => {
        if (event.metaKey && event.key === "k") {
          event.preventDefault();
          window.keptCount = (window.keptCount || 0) + 1;
          document.getElementById("kept").textContent =
            "This page kept cmd-k " + window.keptCount + " times.";
        }
      });
      for (let line = 1; line <= 60; line++) {
        const paragraph = document.createElement("p");
        paragraph.textContent = "Scrollable line " + line;
        document.body.appendChild(paragraph);
      }
    </script>
  </body>
</html>"#;

    struct Example {
        web_view: NativeView,
        web_view_object: Retained<AnyObject>,
        showing_other_page: bool,
        _background_updates: Task<()>,
        web_view_visible: bool,
        sidebar_width: Pixels,
        resizing_sidebar: bool,
        text_box_focus: FocusHandle,
        text_box_text: String,
        palette_focus: FocusHandle,
        palette_text: String,
        palette_clicks: usize,
        focus_before_palette: Option<FocusHandle>,
        keys_left_by_page: Vec<String>,
    }

    impl Example {
        fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
            let (background_sender, mut backgrounds) = mpsc::unbounded();
            // SAFETY: `WKWebView` is a view class in the linked WebKit framework, and
            // windows are opened on the main thread.
            let (web_view, web_view_object) = unsafe {
                let web_view: Allocated<AnyObject> = msg_send![class!(WKWebView), alloc];
                let web_view: Retained<AnyObject> = msg_send![web_view, init];

                let configuration: Retained<AnyObject> = msg_send![&*web_view, configuration];
                let content_controller: Retained<AnyObject> =
                    msg_send![&*configuration, userContentController];
                const AT_DOCUMENT_END: isize = 1;
                let script: Allocated<AnyObject> = msg_send![class!(WKUserScript), alloc];
                let script: Retained<AnyObject> = msg_send![
                    script,
                    initWithSource: &*ns_string(REPORT_BACKGROUND),
                    injectionTime: AT_DOCUMENT_END,
                    forMainFrameOnly: true
                ];
                let _: () = msg_send![&*content_controller, addUserScript: &*script];
                let _: () = msg_send![
                    &*content_controller,
                    addScriptMessageHandler: &*background_reporter(background_sender),
                    name: &*ns_string(c"background")
                ];

                // A web view fills the area its page has not painted yet with the
                // system's text background, which shows as a sliver while it grows.
                // Without it, the window's native view background shows there instead.
                let draws_background: Retained<AnyObject> =
                    msg_send![class!(NSNumber), numberWithBool: false];
                let _: () = msg_send![
                    &*web_view,
                    setValue: &*draws_background,
                    forKey: &*ns_string(c"drawsBackground")
                ];
                load_page(&web_view, PAGE);
                let native_view = NativeView::new(NonNull::from(&*web_view).cast(), window, cx)
                    .expect("macOS windows composite native views");
                // The page sees keys before GPUI does, so without this a page that
                // handled this key itself would keep the palette from opening.
                native_view.reserve_action::<TogglePalette>();
                (native_view, web_view)
            };
            let background_updates = cx.spawn(async move |this, cx| {
                while let Some(color) = backgrounds.next().await {
                    let Some(color) = parse_css_color(&color) else {
                        continue;
                    };
                    let updated = this.update(cx, |this, cx| {
                        this.web_view.set_background(color);
                        cx.notify();
                    });
                    if updated.is_err() {
                        break;
                    }
                }
            });
            Self {
                web_view,
                web_view_object,
                showing_other_page: false,
                _background_updates: background_updates,
                web_view_visible: true,
                sidebar_width: px(240.),
                resizing_sidebar: false,
                text_box_focus: cx.focus_handle(),
                text_box_text: String::new(),
                palette_focus: cx.focus_handle(),
                palette_text: String::new(),
                palette_clicks: 0,
                focus_before_palette: None,
                keys_left_by_page: Vec::new(),
            }
        }

        fn toggle_palette(
            &mut self,
            _: &TogglePalette,
            window: &mut Window,
            cx: &mut Context<Self>,
        ) {
            if self.focus_before_palette.is_some() {
                self.dismiss_palette(&DismissPalette, window, cx);
                return;
            }
            self.focus_before_palette = Some(
                window
                    .focused(cx)
                    .unwrap_or_else(|| self.text_box_focus.clone()),
            );
            window.focus(&self.palette_focus, cx);
            cx.notify();
        }

        fn dismiss_palette(
            &mut self,
            _: &DismissPalette,
            window: &mut Window,
            cx: &mut Context<Self>,
        ) {
            if let Some(previous_focus) = self.focus_before_palette.take() {
                window.focus(&previous_focus, cx);
                cx.notify();
            }
        }

        fn render_palette(&self, cx: &mut Context<Self>) -> impl IntoElement {
            deferred(
                anchored().position(point(px(170.), px(120.))).child(
                    div()
                        .id("palette")
                        .track_focus(&self.palette_focus)
                        .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                            this.dismiss_palette(&DismissPalette, window, cx);
                        }))
                        .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                            if let Some(text) = &event.keystroke.key_char {
                                this.palette_text.push_str(text);
                                cx.notify();
                            }
                        }))
                        .occlude()
                        .w(px(320.))
                        .p_4()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .rounded_lg()
                        .shadow_xl()
                        .bg(rgb(0x1f2933))
                        .text_color(gpui::white())
                        .child("A GPUI palette over the page")
                        .child(
                            div()
                                .p_2()
                                .rounded_md()
                                .bg(rgb(0x323f4b))
                                .child(format!("Typed: {}", self.palette_text)),
                        )
                        .child(
                            div()
                                .id("palette-button")
                                .p_2()
                                .rounded_md()
                                .bg(rgb(0x3e7bfa))
                                .cursor_pointer()
                                .child(format!("Clicked {} times", self.palette_clicks))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.palette_clicks += 1;
                                    cx.notify();
                                })),
                        ),
                ),
            )
            .with_priority(1)
        }
    }

    impl Render for Example {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let text_box_focused = self.text_box_focus.is_focused(window);
            div()
                .size_full()
                .flex()
                .bg(rgb(0xf0f4f8))
                .text_color(rgb(0x1f2933))
                .on_action(cx.listener(Self::toggle_palette))
                .on_action(cx.listener(Self::dismiss_palette))
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                    if gpui::Focusable::focus_handle(&this.web_view, cx).is_focused(window) {
                        this.keys_left_by_page
                            .push(event.keystroke.unparse(gpui::OperatingSystem::Mac));
                        let excess = this.keys_left_by_page.len().saturating_sub(8);
                        this.keys_left_by_page.drain(..excess);
                        cx.notify();
                    }
                }))
                .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                    if this.resizing_sidebar {
                        this.sidebar_width = event.position.x.clamp(px(120.), px(600.));
                        cx.notify();
                    }
                }))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _, _, _| this.resizing_sidebar = false),
                )
                .child(
                    div()
                        .flex_none()
                        .w(self.sidebar_width)
                        .p_4()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child("GPUI content")
                        .child(
                            div()
                                .id("text-box")
                                .track_focus(&self.text_box_focus)
                                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                                    if let Some(text) = &event.keystroke.key_char {
                                        this.text_box_text.push_str(text);
                                        cx.notify();
                                    }
                                }))
                                .on_click(cx.listener(|this, _, window, cx| {
                                    window.focus(&this.text_box_focus, cx);
                                }))
                                .min_h(px(60.))
                                .p_2()
                                .rounded_md()
                                .border_1()
                                .border_color(if text_box_focused {
                                    rgb(0x3e7bfa)
                                } else {
                                    rgb(0x9aa5b1)
                                })
                                .bg(gpui::white())
                                .child(format!("Click, then type: {}", self.text_box_text)),
                        )
                        .child(
                            div()
                                .id("toggle-web-view")
                                .p_2()
                                .rounded_md()
                                .bg(rgb(0xd9e2ec))
                                .cursor_pointer()
                                .child(if self.web_view_visible {
                                    "Hide the web view"
                                } else {
                                    "Show the web view"
                                })
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.web_view_visible = !this.web_view_visible;
                                    cx.notify();
                                })),
                        )
                        .child(
                            div()
                                .id("load-other-page")
                                .p_2()
                                .rounded_md()
                                .bg(rgb(0xd9e2ec))
                                .cursor_pointer()
                                .child("Load the other page")
                                .on_click(cx.listener(|this, _, _, _| {
                                    this.showing_other_page = !this.showing_other_page;
                                    let page = if this.showing_other_page {
                                        OTHER_PAGE
                                    } else {
                                        PAGE
                                    };
                                    // SAFETY: The web view is live, and listeners run on
                                    // the main thread.
                                    unsafe { load_page(&this.web_view_object, page) };
                                })),
                        )
                        .child("cmd-shift-p toggles the palette, escape dismisses it.")
                        .child(format!(
                            "Keys the page left for GPUI: {}",
                            self.keys_left_by_page.join(" ")
                        )),
                )
                .child(
                    div()
                        .id("divider")
                        .flex_none()
                        .w(px(6.))
                        .h_full()
                        .bg(rgb(0x9aa5b1))
                        .cursor_col_resize()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, _, _| this.resizing_sidebar = true),
                        ),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .h_full()
                        .when(self.web_view_visible, |this| {
                            this.child(self.web_view.clone())
                        }),
                )
                .when(self.focus_before_palette.is_some(), |this| {
                    this.child(self.render_palette(cx))
                })
        }
    }

    fn ns_string(text: &CStr) -> Retained<AnyObject> {
        // SAFETY: `text` is a valid, nul-terminated UTF-8 string.
        unsafe { msg_send![class!(NSString), stringWithUTF8String: text.as_ptr()] }
    }

    /// # Safety
    ///
    /// `web_view` must be a `WKWebView`, and this must be called on the main thread.
    unsafe fn load_page(web_view: &AnyObject, page: &CStr) {
        unsafe {
            let _: *mut AnyObject = msg_send![
                web_view,
                loadHTMLString: &*ns_string(page),
                baseURL: ptr::null::<AnyObject>()
            ];
        }
    }

    /// Creates the object the page's script posts its background color to.
    fn background_reporter(sender: mpsc::UnboundedSender<String>) -> Retained<AnyObject> {
        const SENDER: &CStr = c"sender";

        extern "C" fn received(
            this: &AnyObject,
            _: Sel,
            _: *mut AnyObject,
            message: *mut AnyObject,
        ) {
            let Some(sender) = this.class().instance_variable(SENDER) else {
                return;
            };
            // SAFETY: The ivar holds the sender leaked below, WebKit passes a live
            // `WKScriptMessage`, and the script only posts strings.
            unsafe {
                let sender = *sender.load::<*mut c_void>(this);
                let sender = &*sender.cast::<mpsc::UnboundedSender<String>>();
                let body: *mut AnyObject = msg_send![message, body];
                let text: *const c_char = msg_send![body, UTF8String];
                if !text.is_null() {
                    let color = CStr::from_ptr(text).to_string_lossy().into_owned();
                    // The example's window is gone if nothing is receiving.
                    sender.unbounded_send(color).ok();
                }
            }
        }

        static CLASS: OnceLock<&'static AnyClass> = OnceLock::new();
        let class = CLASS.get_or_init(|| {
            let mut builder =
                ClassBuilder::new(c"NativeViewExampleBackgroundReporter", class!(NSObject))
                    .expect("the class is only registered here");
            builder.add_ivar::<*mut c_void>(SENDER);
            // SAFETY: The function matches the signature of
            // `-userContentController:didReceiveScriptMessage:`.
            unsafe {
                builder.add_method(
                    sel!(userContentController:didReceiveScriptMessage:),
                    received as extern "C" fn(_, _, _, _),
                );
            }
            builder.register()
        });
        // SAFETY: The class was built from `NSObject` above. The web view keeps the
        // reporter for as long as the example runs, so the sender is never freed.
        unsafe {
            let reporter: Allocated<AnyObject> = msg_send![*class, alloc];
            let reporter: Retained<AnyObject> = msg_send![reporter, init];
            if let Some(ivar) = class.instance_variable(SENDER) {
                ivar.load_ptr::<*mut c_void>(&reporter)
                    .write(Box::into_raw(Box::new(sender)).cast());
            }
            reporter
        }
    }

    /// Parses the `rgb(r, g, b)` and `rgba(r, g, b, a)` forms that computed styles use.
    fn parse_css_color(text: &str) -> Option<Rgba> {
        let components = text.split_once('(')?.1.strip_suffix(')')?;
        let mut components = components
            .split(',')
            .map(|component| component.trim().parse::<f32>().ok());
        Some(Rgba {
            r: components.next()?? / 255.,
            g: components.next()?? / 255.,
            b: components.next()?? / 255.,
            a: 1.,
        })
    }

    pub fn run() {
        gpui_platform::application().run(|cx: &mut App| {
            if !crate::example_support::load_fonts(cx) {
                return;
            }
            cx.bind_keys([
                KeyBinding::new("cmd-shift-p", TogglePalette, None),
                KeyBinding::new("escape", DismissPalette, None),
            ]);
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
