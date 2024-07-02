use std::rc::Rc;

use gpui::*;
use wry::{
    dpi::{self, LogicalSize},
    Rect,
};

struct WebViewWindow {
    webview: View<WebView>,
}

impl WebViewWindow {
    fn new(cx: &mut WindowContext) -> Self {
        Self {
            webview: cx.new_view(|cx| WebView::new(cx)),
        }
    }
}

impl Render for WebViewWindow {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().flex().bg(rgb(0xF0F0F0)).size_full().p_10().child(
            div()
                .flex()
                .flex_col()
                .size_full()
                .justify_center()
                .items_center()
                .gap_4()
                .child("Wry WebView Demo")
                .child(self.webview.clone()),
        )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    kind: WindowKind::Normal,
                    ..Default::default()
                },
                |cx| cx.new_view(|cx| WebViewWindow::new(cx)),
            )
            .unwrap();

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, cx| {
                    cx.activate_window();
                    cx.set_window_title("WebView Example");
                    cx.on_release(|_, _, _cx| {
                        // exit app
                        std::process::exit(0);
                    })
                    .detach();
                })
                .unwrap();
        })
        .detach();
    });
}

/// A webview element that can display a URL or HTML content.
pub struct WebView {
    view: Rc<wry::WebView>,
}

impl WebView {
    /// Create a new webview element.
    pub fn new(cx: &mut WindowContext) -> Self {
        let view = Rc::new(
            wry::WebViewBuilder::new_as_child(&cx.raw_window_handle())
                .with_url("https://zed.dev")
                .build()
                .expect("Failed to create webview."),
        );

        Self { view }
    }
}

impl Render for WebView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("WebView")
            .block()
            .overflow_y_scroll()
            .size_full()
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .rounded_md()
            .bg(gpui::white())
            .border_color(rgb(0xD0D0D0))
            .child(WebViewElement {
                view: self.view.clone(),
            })
    }
}

struct WebViewElement {
    view: Rc<wry::WebView>,
}
impl IntoElement for WebViewElement {
    type Element = WebViewElement;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for WebViewElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.flex_grow = 1.0;
        style.size = Size::full();
        let id = cx.request_layout(style, []);
        (id, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        if bounds.top() > cx.viewport_size().height || bounds.bottom() < Pixels::ZERO {
            self.view.set_visible(false).unwrap();
        } else {
            self.view.set_visible(true).unwrap();

            self.view
                .set_bounds(Rect {
                    size: dpi::Size::Logical(LogicalSize {
                        width: (bounds.size.width.0).into(),
                        height: (bounds.size.height.0).into(),
                    }),
                    position: dpi::Position::Logical(dpi::LogicalPosition::new(
                        bounds.origin.x.into(),
                        bounds.origin.y.into(),
                    )),
                })
                .unwrap();
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        _: &mut WindowContext,
    ) {
        // Nothing to do here
    }
}
