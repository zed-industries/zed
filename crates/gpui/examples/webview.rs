use gpui::{
    App, AppContext as _, Application, Bounds, Context, DismissEvent, Element, ElementId, Entity,
    EventEmitter, FocusHandle, Focusable, GlobalElementId, InteractiveElement, IntoElement,
    LayoutId, ParentElement as _, Pixels, Render, Size, Style, Styled as _, Window, WindowBounds,
    WindowKind, WindowOptions, canvas, div, px, rgb, size,
};
use raw_window_handle::HasWindowHandle;
use std::{ops::Deref, rc::Rc};
use wry::{
    Rect,
    dpi::{self, LogicalSize},
};

struct WebViewWindow {
    webview: Entity<WebView>,
}

impl WebViewWindow {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let webview = cx.new(|cx| {
            let webview = wry::WebViewBuilder::new()
                .with_url("https://gpui.rs")
                .build_as_child(&window.window_handle().expect("no window handle"))
                .unwrap();
            WebView::new(webview, window, cx)
        });

        Self { webview }
    }
}

impl Render for WebViewWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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

pub struct WebView {
    focus_handle: FocusHandle,
    webview: Rc<wry::WebView>,
    visible: bool,
    bounds: Bounds<Pixels>,
}

impl Drop for WebView {
    fn drop(&mut self) {
        self.hide();
    }
}

impl WebView {
    pub fn new(webview: wry::WebView, _: &mut Window, cx: &mut App) -> Self {
        let _ = webview.set_bounds(Rect::default());

        Self {
            focus_handle: cx.focus_handle(),
            visible: true,
            bounds: Bounds::default(),
            webview: Rc::new(webview),
        }
    }

    pub fn show(&mut self) {
        let _ = self.webview.set_visible(true);
    }

    pub fn hide(&mut self) {
        _ = self.webview.focus_parent();
        _ = self.webview.set_visible(false);
    }

    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    pub fn load_url(&mut self, url: &str) {
        self.webview.load_url(url).unwrap();
    }
}

impl Deref for WebView {
    type Target = wry::WebView;

    fn deref(&self) -> &Self::Target {
        &self.webview
    }
}

impl Focusable for WebView {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for WebView {}

impl Render for WebView {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .child({
                let view = cx.entity().clone();
                canvas(
                    move |bounds, _, cx| view.update(cx, |r, _| r.bounds = bounds),
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full()
            })
            .child(WebViewElement::new(self.webview.clone(), window, cx))
    }
}

/// A webview element can display a wry webview.
pub struct WebViewElement {
    view: Rc<wry::WebView>,
}

impl WebViewElement {
    /// Create a new webview element from a wry WebView.
    pub fn new(view: Rc<wry::WebView>, _window: &mut Window, _cx: &mut App) -> Self {
        Self { view }
    }
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
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let style = Style {
            flex_grow: 0.0,
            size: Size::full(),
            ..Default::default()
        };
        // If the parent view is no longer visible, we don't need to layout the webview

        let id = window.request_layout(style, [], cx);
        (id, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Window,
        _: &mut App,
    ) -> Self::PrepaintState {
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

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        _: &mut Window,
        _: &mut App,
    ) {
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        let window = cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    kind: WindowKind::Normal,
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| WebViewWindow::new(window, cx)),
            )
            .unwrap();

        cx.spawn(async move |cx| {
            window
                .update(cx, |_, window, cx| {
                    window.activate_window();
                    window.set_window_title("WebView Example");
                    cx.on_release(|_, _app| {
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
