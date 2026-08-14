#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("The native_webview example is only available on macOS.");
}

#[cfg(target_os = "macos")]
mod macos {
    use std::rc::Rc;

    use cocoa::{
        appkit::NSView,
        base::{YES, id, nil},
        foundation::{NSPoint, NSRect, NSSize, NSString},
    };
    use gpui::{
        App, Bounds, Context, Div, Element, ElementId, GlobalElementId, IntoElement, LayoutId,
        MouseButton, Pixels, Stateful, Style, Window, WindowBounds, WindowOptions, deferred, div,
        prelude::*, px, relative, rgb, size,
    };
    use gpui_platform::application;
    use objc::{class, msg_send, sel, sel_impl};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    #[link(name = "WebKit", kind = "framework")]
    unsafe extern "C" {}

    const PAGE: &str = include_str!("native_webview.html");

    struct NativeWebView {
        parent: id,
        view: id,
    }

    impl NativeWebView {
        fn new(window: &Window) -> anyhow::Result<Self> {
            let window_handle = HasWindowHandle::window_handle(window).map_err(|error| {
                anyhow::anyhow!("failed to get AppKit window handle: {error:?}")
            })?;
            let parent = match window_handle.as_raw() {
                RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr() as id,
                _ => anyhow::bail!("native_webview requires an AppKit window"),
            };

            unsafe {
                let configuration: id = msg_send![class!(WKWebViewConfiguration), new];
                let view: id = msg_send![class!(WKWebView), alloc];
                let view: id = msg_send![
                    view,
                    initWithFrame: NSRect::new(
                        NSPoint::new(0., 0.),
                        NSSize::new(0., 0.),
                    )
                    configuration: configuration
                ];
                if view.is_null() {
                    let _: () = msg_send![configuration, release];
                    anyhow::bail!("failed to create WKWebView");
                }

                let _: () = msg_send![view, setWantsLayer: YES];
                let layer: id = msg_send![view, layer];
                let border_color: id = msg_send![
                    class!(NSColor),
                    colorWithSRGBRed: 63. / 255.
                    green: 64. / 255.
                    blue: 67. / 255.
                    alpha: 1.
                ];
                let border_color: id = msg_send![border_color, CGColor];
                let _: () = msg_send![layer, setMasksToBounds: YES];
                let _: () = msg_send![layer, setCornerRadius: 8.];
                let _: () = msg_send![layer, setBorderWidth: 1.];
                let _: () = msg_send![layer, setBorderColor: border_color];

                let html = NSString::alloc(nil).init_str(PAGE);
                let _: id = msg_send![view, loadHTMLString: html baseURL: nil];
                parent.addSubview_(view);

                let _: () = msg_send![html, release];
                let _: () = msg_send![configuration, release];

                Ok(Self { parent, view })
            }
        }

        fn set_bounds(&self, bounds: Bounds<Pixels>) {
            unsafe {
                let parent_bounds = NSView::bounds(self.parent);
                let frame = NSRect::new(
                    NSPoint::new(
                        f64::from(bounds.origin.x),
                        parent_bounds.size.height
                            - f64::from(bounds.origin.y)
                            - f64::from(bounds.size.height),
                    ),
                    NSSize::new(f64::from(bounds.size.width), f64::from(bounds.size.height)),
                );
                let _: () = msg_send![self.view, setFrame: frame];
            }
        }

        fn focus_parent(&self) {
            unsafe {
                let window: id = msg_send![self.view, window];
                if !window.is_null() {
                    let _: bool = msg_send![window, makeFirstResponder: self.parent];
                }
            }
        }
    }

    impl Drop for NativeWebView {
        fn drop(&mut self) {
            unsafe {
                NSView::removeFromSuperview(self.view);
                let _: () = msg_send![self.view, release];
            }
        }
    }

    struct NativeWebViewElement {
        webview: Rc<NativeWebView>,
    }

    impl IntoElement for NativeWebViewElement {
        type Element = Self;

        fn into_element(self) -> Self::Element {
            self
        }
    }

    impl Element for NativeWebViewElement {
        type RequestLayoutState = ();
        type PrepaintState = ();

        fn id(&self) -> Option<ElementId> {
            None
        }

        fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
            None
        }

        fn request_layout(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            window: &mut Window,
            cx: &mut App,
        ) -> (LayoutId, Self::RequestLayoutState) {
            let mut style = Style::default();
            style.size.width = relative(1.).into();
            style.size.height = relative(1.).into();
            (window.request_layout(style, [], cx), ())
        }

        fn prepaint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            _window: &mut Window,
            _cx: &mut App,
        ) -> Self::PrepaintState {
            self.webview.set_bounds(bounds);
        }

        fn paint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            _bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            _prepaint: &mut Self::PrepaintState,
            _window: &mut Window,
            _cx: &mut App,
        ) {
        }
    }

    struct NativeWebViewExample {
        webview: Rc<NativeWebView>,
        about_active: bool,
        dialog_open: bool,
        menu_open: bool,
        popover_open: bool,
    }

    fn button(id: &'static str, label: &'static str) -> Stateful<Div> {
        div()
            .id(id)
            .px_4()
            .py_2()
            .rounded_md()
            .border_1()
            .border_color(rgb(0x3f4043))
            .bg(rgb(0x1f2127))
            .text_color(rgb(0xbfbdb6))
            .text_sm()
            .cursor_pointer()
            .hover(|style| style.bg(rgb(0x2d2f34)).border_color(rgb(0x3e4043)))
            .child(label)
    }

    fn tab(id: &'static str, label: &'static str, active: bool) -> Stateful<Div> {
        div()
            .id(id)
            .px_3()
            .py_2()
            .border_b_2()
            .border_color(if active { rgb(0x5ac1fe) } else { rgb(0x313337) })
            .text_color(if active { rgb(0xbfbdb6) } else { rgb(0x8a8986) })
            .text_sm()
            .cursor_pointer()
            .hover(|style| style.text_color(rgb(0xbfbdb6)))
            .child(label)
    }

    fn menu_item(id: &'static str, label: &'static str) -> Stateful<Div> {
        div()
            .id(id)
            .px_2()
            .py_1()
            .rounded_md()
            .text_xs()
            .text_color(rgb(0xbfbdb6))
            .cursor_pointer()
            .hover(|style| style.bg(rgb(0x2d2f34)))
            .child(label)
    }

    fn layer_row(index: &'static str, label: &'static str, color: gpui::Rgba) -> Div {
        div()
            .flex()
            .items_center()
            .gap_2()
            .py_2()
            .border_b_1()
            .border_color(rgb(0x3f4043))
            .child(div().w_1().h_5().rounded_sm().bg(color))
            .child(div().text_color(rgb(0x696a6a)).w(px(18.)).child(index))
            .child(div().text_color(rgb(0xbfbdb6)).child(label))
    }

    impl Render for NativeWebViewExample {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("native-webview-example")
                .relative()
                .flex()
                .gap_6()
                .size_full()
                .p_7()
                .bg(rgb(0x313337))
                .text_color(rgb(0xbfbdb6))
                // While a deferred overlay is visible, the transparent GPUI
                // NSView captures the whole window. This handler therefore
                // also dismisses clicks geometrically over the WKWebView.
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.webview.focus_parent();
                        this.popover_open = false;
                        this.dialog_open = false;
                        this.menu_open = false;
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .justify_between()
                        .w(px(180.))
                        .child(
                            div()
                                .child(
                                    div()
                                        .mt(px(-2.))
                                        .text_lg()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .text_color(rgb(0xfeb454))
                                        .child("GPUI NATIVE WEBVIEW"),
                                )
                                .child(div().mt_2().text_sm().text_color(rgb(0x8a8986)).child(
                                    "Native composition.\nThree surfaces, one visual stack.",
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .text_xs()
                                .child(layer_row("03", "GPUI overlay", rgb(0xfeb454)))
                                .child(layer_row("02", "WKWebView", rgb(0x5ac1fe)))
                                .child(layer_row("01", "GPUI base", rgb(0x8a8986))),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_w_0()
                        .h_full()
                        .gap_4()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0x8a8986))
                                                .child("COMPOSITION TARGET"),
                                        )
                                        .child(
                                            div().mt_1().text_lg().child("WebView overlay proof"),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap_2()
                                        .child(
                                            button("toggle-popover", "Show popover").on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    cx.stop_propagation();
                                                    this.webview.focus_parent();
                                                    this.popover_open = !this.popover_open;
                                                    this.menu_open = false;
                                                    cx.notify();
                                                }),
                                            ),
                                        )
                                        .child(button("open-dialog", "Open dialog").on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                cx.stop_propagation();
                                                this.webview.focus_parent();
                                                this.dialog_open = true;
                                                this.menu_open = false;
                                                cx.notify();
                                            }),
                                        )),
                                ),
                        )
                        .child(
                            div()
                                .relative()
                                .flex()
                                .items_center()
                                .justify_between()
                                .border_b_1()
                                .border_color(rgb(0x3f4043))
                                .child(
                                    div()
                                        .flex()
                                        .child(
                                            tab("webview-tab", "WebView", !self.about_active)
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _, _, cx| {
                                                        cx.stop_propagation();
                                                        this.about_active = false;
                                                        this.popover_open = false;
                                                        this.dialog_open = false;
                                                        this.menu_open = false;
                                                        cx.notify();
                                                    }),
                                                ),
                                        )
                                        .child(
                                            tab("about-tab", "About", self.about_active)
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _, _, cx| {
                                                        cx.stop_propagation();
                                                        this.webview.focus_parent();
                                                        this.about_active = true;
                                                        this.popover_open = false;
                                                        this.dialog_open = false;
                                                        this.menu_open = false;
                                                        cx.notify();
                                                    }),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .id("popup-menu-trigger")
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .w(px(28.))
                                        .h(px(28.))
                                        .rounded_md()
                                        .text_base()
                                        .text_color(rgb(0x8a8986))
                                        .cursor_pointer()
                                        .hover(|style| {
                                            style.bg(rgb(0x2d2f34)).text_color(rgb(0xbfbdb6))
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                cx.stop_propagation();
                                                this.webview.focus_parent();
                                                this.menu_open = !this.menu_open;
                                                cx.notify();
                                            }),
                                        )
                                        .child("…"),
                                )
                                .when(self.menu_open, |tab_bar| {
                                    tab_bar.child(
                                        deferred(
                                            div()
                                                .absolute()
                                                .top(px(34.))
                                                .right_0()
                                                .w(px(180.))
                                                .p_1()
                                                .rounded_lg()
                                                .border_1()
                                                .border_color(rgb(0x3f4043))
                                                .bg(rgb(0x1f2127))
                                                .shadow_xl()
                                                .child(menu_item(
                                                    "popup-menu-reload",
                                                    "Reload WebView",
                                                ))
                                                .child(menu_item(
                                                    "popup-menu-inspect",
                                                    "Inspect native surface",
                                                ))
                                                .child(div().my_1().h(px(1.)).bg(rgb(0x3f4043)))
                                                .child(menu_item(
                                                    "popup-menu-about",
                                                    "About this example",
                                                ))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _, _, cx| {
                                                        cx.stop_propagation();
                                                        this.menu_open = false;
                                                        cx.notify();
                                                    }),
                                                ),
                                        )
                                        .priority(2),
                                    )
                                }),
                        )
                        .child(
                            div()
                                .relative()
                                .flex_1()
                                .min_h_0()
                                .child(NativeWebViewElement {
                                    webview: self.webview.clone(),
                                })
                                .when(self.about_active, |content| {
                                    content.child(
                                        deferred(
                                            div()
                                                .absolute()
                                                .inset_0()
                                                .flex()
                                                .flex_col()
                                                .justify_center()
                                                .p_10()
                                                .rounded_lg()
                                                .bg(rgb(0x0d1016))
                                                .text_color(rgb(0xbfbdb6))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x5ac1fe))
                                                        .child("GPUI OVERLAY CONTENT"),
                                                )
                                                .child(
                                                    div()
                                                        .mt_3()
                                                        .text_3xl()
                                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                                        .child("A regular rendered view"),
                                                )
                                                .child(
                                                    div()
                                                        .mt_4()
                                                        .max_w(px(520.))
                                                        .text_color(rgb(0x8a8986))
                                                        .line_height(relative(1.6))
                                                        .child(
                                                            "This tab is rendered by GPUI above \
                                                             the native WebView. It verifies that \
                                                             non-popup content can replace and \
                                                             fully occlude a native surface.",
                                                        ),
                                                ),
                                        )
                                        .priority(1),
                                    )
                                }),
                        ),
                )
                .when(self.popover_open, |root| {
                    root.child(
                        deferred(
                            div()
                                .absolute()
                                .top(px(92.))
                                .right(px(42.))
                                .w(px(300.))
                                .p_4()
                                .rounded_lg()
                                .shadow_xl()
                                .border_1()
                                .border_color(rgb(0x3f4043))
                                .bg(rgb(0x1f2127))
                                .text_color(rgb(0xbfbdb6))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0xfeb454))
                                        .child("SURFACE 03"),
                                )
                                .child(
                                    div()
                                        .mt_2()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .child("Deferred GPUI popover"),
                                )
                                .child(div().mt_2().text_sm().text_color(rgb(0x8a8986)).child(
                                    "Painted after the native WebView without changing its \
                                         AppKit z-order.",
                                )),
                        )
                        .priority(3),
                    )
                })
                .when(self.dialog_open, |root| {
                    root.child(
                        deferred(
                            div()
                                .absolute()
                                .inset_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .bg(rgb(0x0d1016).opacity(0.72))
                                .child(
                                    div()
                                        .w(px(500.))
                                        .p_7()
                                        .rounded_xl()
                                        .shadow_xl()
                                        .border_1()
                                        .border_color(rgb(0x3f4043))
                                        .bg(rgb(0x1f2127))
                                        .text_color(rgb(0xbfbdb6))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0xfeb454))
                                                        .child("SURFACE 03 / GPUI OVERLAY"),
                                                )
                                                .child(
                                                    div()
                                                        .px_2()
                                                        .py_1()
                                                        .rounded_md()
                                                        .bg(rgb(0x2d2f34))
                                                        .text_xs()
                                                        .text_color(rgb(0x8a8986))
                                                        .child("LIVE"),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .mt_5()
                                                .text_2xl()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .child(
                                                    "The native layer stays exactly where it is.",
                                                ),
                                        )
                                        .child(div().mt_3().text_color(rgb(0x8a8986)).child(
                                            "GPUI splits its scene before deferred draws. \
                                                     AppKit places WKWebView between the base and \
                                                     this transparent overlay surface.",
                                        ))
                                        .child(div().mt_5().h(px(1.)).w_full().bg(rgb(0x3f4043)))
                                        .child(div().mt_5().flex().child(
                                            button("close-dialog", "Close").on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    cx.stop_propagation();
                                                    this.webview.focus_parent();
                                                    this.dialog_open = false;
                                                    cx.notify();
                                                }),
                                            ),
                                        )),
                                ),
                        )
                        .priority(4),
                    )
                })
        }
    }

    pub fn run() {
        application().run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(900.), px(640.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let webview = Rc::new(NativeWebView::new(window).unwrap());

                    // Insert the transparent GPUI overlay after the native
                    // WebView so AppKit places it above the browser view.
                    window.enable_scene_overlay().unwrap();

                    cx.new(|_| NativeWebViewExample {
                        webview,
                        about_active: false,
                        dialog_open: false,
                        menu_open: false,
                        popover_open: false,
                    })
                },
            )
            .unwrap();
            cx.activate(true);
        });
    }
}

#[cfg(target_os = "macos")]
fn main() {
    macos::run();
}
