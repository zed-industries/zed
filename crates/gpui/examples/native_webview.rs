#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("The native_webview example is only available on macOS.");
}

#[cfg(target_os = "macos")]
mod macos {
    use std::rc::Rc;

    use cocoa::{
        appkit::NSView,
        base::{id, nil},
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
        dialog_open: bool,
        popover_open: bool,
    }

    fn button(id: &'static str, label: &'static str) -> Stateful<Div> {
        div()
            .id(id)
            .px_4()
            .py_2()
            .rounded_md()
            .border_1()
            .border_color(rgb(0xcfd1d2))
            .bg(rgb(0xececed))
            .text_color(rgb(0x5c6166))
            .text_sm()
            .cursor_pointer()
            .hover(|style| style.bg(rgb(0xdfe0e1)).border_color(rgb(0xcfd0d2)))
            .child(label)
    }

    fn layer_row(index: &'static str, label: &'static str, color: gpui::Rgba) -> Div {
        div()
            .flex()
            .items_center()
            .gap_2()
            .py_2()
            .border_b_1()
            .border_color(rgb(0xcfd1d2))
            .child(div().w_1().h_5().rounded_sm().bg(color))
            .child(div().text_color(rgb(0xa9acae)).w(px(18.)).child(index))
            .child(div().text_color(rgb(0x5c6166)).child(label))
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
                .bg(rgb(0xdcddde))
                .text_color(rgb(0x5c6166))
                // While a deferred overlay is visible, the transparent GPUI
                // NSView captures the whole window. This handler therefore
                // also dismisses clicks geometrically over the WKWebView.
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.webview.focus_parent();
                        this.popover_open = false;
                        this.dialog_open = false;
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .justify_between()
                        .w(px(150.))
                        .py_1()
                        .child(
                            div()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0xf1ad49))
                                        .child("GPUI / LAB 04"),
                                )
                                .child(
                                    div()
                                        .mt_3()
                                        .text_2xl()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .child("Native\ncomposition"),
                                )
                                .child(
                                    div()
                                        .mt_3()
                                        .text_sm()
                                        .text_color(rgb(0x8b8e92))
                                        .child("Three surfaces.\nOne visual stack."),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .text_xs()
                                .child(layer_row("03", "GPUI overlay", rgb(0xf1ad49)))
                                .child(layer_row("02", "WKWebView", rgb(0x3b9ee5)))
                                .child(layer_row("01", "GPUI base", rgb(0x8b8e92))),
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
                                                .text_color(rgb(0x8b8e92))
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
                                                cx.notify();
                                            }),
                                        )),
                                ),
                        )
                        .child(
                            div()
                                .flex_1()
                                .min_h_0()
                                .p(px(6.))
                                .rounded_xl()
                                .border_1()
                                .border_color(rgb(0xcfd1d2))
                                .bg(rgb(0xececed))
                                .shadow_xl()
                                .overflow_hidden()
                                .child(NativeWebViewElement {
                                    webview: self.webview.clone(),
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
                                .border_color(rgb(0xcfd1d2))
                                .bg(rgb(0xececed))
                                .text_color(rgb(0x5c6166))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0xf1ad49))
                                        .child("SURFACE 03"),
                                )
                                .child(
                                    div()
                                        .mt_2()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .child("Deferred GPUI popover"),
                                )
                                .child(div().mt_2().text_sm().text_color(rgb(0x8b8e92)).child(
                                    "Painted after the native WebView without changing its \
                                         AppKit z-order.",
                                )),
                        )
                        .priority(1),
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
                                .bg(rgb(0x5c6166).opacity(0.38))
                                .child(
                                    div()
                                        .w(px(500.))
                                        .p_7()
                                        .rounded_xl()
                                        .shadow_xl()
                                        .border_1()
                                        .border_color(rgb(0xcfd1d2))
                                        .bg(rgb(0xececed))
                                        .text_color(rgb(0x5c6166))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0xf1ad49))
                                                        .child("SURFACE 03 / GPUI OVERLAY"),
                                                )
                                                .child(
                                                    div()
                                                        .px_2()
                                                        .py_1()
                                                        .rounded_md()
                                                        .bg(rgb(0xdfe0e1))
                                                        .text_xs()
                                                        .text_color(rgb(0x8b8e92))
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
                                        .child(div().mt_3().text_color(rgb(0x8b8e92)).child(
                                            "GPUI splits its scene before deferred draws. \
                                                     AppKit places WKWebView between the base and \
                                                     this transparent overlay surface.",
                                        ))
                                        .child(div().mt_5().h(px(1.)).w_full().bg(rgb(0xcfd1d2)))
                                        .child(
                                            button("close-dialog", "Close").mt_5().on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    cx.stop_propagation();
                                                    this.webview.focus_parent();
                                                    this.dialog_open = false;
                                                    cx.notify();
                                                }),
                                            ),
                                        ),
                                ),
                        )
                        .priority(2),
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
                        dialog_open: false,
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
