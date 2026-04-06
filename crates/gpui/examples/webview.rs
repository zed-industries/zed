//! Example demonstrating the WebView element.
//!
//! Run with:
//!   cargo run -p gpui --example webview --features webview
//!
//! On Wayland, webviews open as separate GTK windows.
//! To test inline embedding (X11), force both GPUI and GTK to X11:
//!   WAYLAND_DISPLAY= GDK_BACKEND=x11 cargo run -p gpui --example webview --features webview

use gpui::{
    App, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb,
    size,
};

#[cfg(feature = "webview")]
use gpui::WebView;

const COUNTER_HTML: &str = r#"<!DOCTYPE html>
<html>
<head><style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { background: #1e1e2e; color: #cdd6f4; font-family: system-ui, sans-serif;
           display: flex; flex-direction: column; align-items: center; justify-content: center;
           height: 100vh; }
    h1 { color: #89b4fa; margin-bottom: 12px; }
    p { color: #a6adc8; margin-bottom: 16px; }
    button { background: #89b4fa; color: #1e1e2e; border: none; padding: 8px 16px;
             border-radius: 6px; cursor: pointer; font-size: 14px; margin: 0 4px; }
    button:hover { background: #b4befe; }
    #counter { font-size: 64px; color: #f38ba8; margin: 20px 0; }
    .info { background: #313244; padding: 12px; border-radius: 8px; margin-top: 16px;
            font-size: 13px; max-width: 400px; text-align: center; }
    .info code { color: #a6e3a1; }
</style></head>
<body>
    <h1>Zed WebView</h1>
    <p>Interactive HTML + JavaScript running in WebKitGTK</p>
    <div id="counter">0</div>
    <div>
        <button onclick="count(-1)">-</button>
        <button onclick="count(1)">+</button>
        <button onclick="document.getElementById('counter').textContent='0'; n=0;">Reset</button>
    </div>
    <div class="info">
        User agent: <code id="ua"></code>
    </div>
    <script>
        let n = 0;
        function count(d) { n += d; document.getElementById('counter').textContent = n; }
        document.getElementById('ua').textContent = navigator.userAgent;
    </script>
</body>
</html>"#;

const PLOTLY_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
<script src="https://cdn.plot.ly/plotly-2.35.2.min.js"></script>
<style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { background: #1e1e2e; font-family: system-ui, sans-serif; }
    .container { padding: 16px; }
    h2 { color: #89b4fa; text-align: center; margin-bottom: 8px; }
    p { color: #a6adc8; text-align: center; font-size: 13px; margin-bottom: 12px; }
    #scatter, #surface { width: 100%; height: 350px; }
    .divider { height: 1px; background: #313244; margin: 16px 0; }
</style>
</head>
<body>
<div class="container">
    <h2>Interactive Plotly Charts</h2>
    <div id="scatter"></div>
    <div class="divider"></div>
    <div id="surface"></div>
</div>
<script>
    var layout = {
        paper_bgcolor: '#1e1e2e', plot_bgcolor: '#313244',
        font: { color: '#cdd6f4', size: 11 },
        margin: { t: 40, b: 40, l: 50, r: 20 },
        xaxis: { gridcolor: '#45475a' }, yaxis: { gridcolor: '#45475a' },
    };

    var x = Array.from({length: 50}, (_, i) => i * 0.2);
    Plotly.newPlot('scatter', [
        { x: x, y: x.map(v => Math.sin(v)), type: 'scatter', name: 'sin(x)',
          line: { color: '#89b4fa', width: 2 } },
        { x: x, y: x.map(v => Math.cos(v)), type: 'scatter', name: 'cos(x)',
          line: { color: '#f38ba8', width: 2 } },
        { x: x, y: x.map(v => Math.sin(v) * Math.cos(v * 0.5)), type: 'scatter',
          name: 'sin(x)*cos(x/2)', line: { color: '#a6e3a1', width: 2 } },
    ], { ...layout, title: { text: 'Trigonometric Functions', font: { size: 14 } } },
    { responsive: true });

    var size = 30;
    var z = [];
    for (var i = 0; i < size; i++) {
        z[i] = [];
        for (var j = 0; j < size; j++) {
            var x = (i - size/2) / 5, y = (j - size/2) / 5;
            z[i][j] = Math.sin(Math.sqrt(x*x + y*y)) * 5;
        }
    }
    Plotly.newPlot('surface', [{ z: z, type: 'surface',
        colorscale: [[0,'#89b4fa'],[0.5,'#1e1e2e'],[1,'#f38ba8']] }],
    { ...layout, title: { text: '3D Surface Plot', font: { size: 14 } },
      scene: { xaxis: { gridcolor: '#45475a' }, yaxis: { gridcolor: '#45475a' },
               zaxis: { gridcolor: '#45475a' },
               bgcolor: '#1e1e2e' },
      margin: { t: 40, b: 10, l: 10, r: 10 } },
    { responsive: true });
</script>
</body>
</html>"#;

#[derive(Clone)]
enum DemoContent {
    Counter,
    Plotly,
    Website,
}

struct WebViewEntry {
    id: usize,
    demo: DemoContent,
}

struct WebViewExample {
    entries: Vec<WebViewEntry>,
}

impl WebViewExample {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    fn add_webview(&mut self, demo: DemoContent, cx: &mut Context<Self>) {
        let id = WebView::next_id();
        self.entries.push(WebViewEntry { id, demo });
        cx.notify();
    }

    fn remove_webview(&mut self, id: usize, cx: &mut Context<Self>) {
        WebView::remove(id);
        self.entries.retain(|entry| entry.id != id);
        cx.notify();
    }
}

impl Render for WebViewExample {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let closed = WebView::drain_closed();
        if !closed.is_empty() {
            for id in &closed {
                self.entries.retain(|entry| entry.id != *id);
            }
        }
        let active = WebView::active_count();

        let mut content_area = div().w_full().flex_1().flex().flex_col().gap_2().p_2();

        for entry in &self.entries {
            let webview = match &entry.demo {
                DemoContent::Counter => WebView::from_html(entry.id, COUNTER_HTML),
                DemoContent::Plotly => WebView::from_html(entry.id, PLOTLY_HTML),
                DemoContent::Website => WebView::from_url(entry.id, "https://zed.dev"),
            };

            let label = match &entry.demo {
                DemoContent::Counter => "Counter",
                DemoContent::Plotly => "Plotly",
                DemoContent::Website => "zed.dev",
            };

            let entry_id = entry.id;
            content_area = content_area.child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .border_1()
                    .border_color(rgb(0x313244))
                    .rounded_md()
                    .overflow_hidden()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_2()
                            .py_1()
                            .bg(rgb(0x313244))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xa6adc8))
                                    .child(label.to_string()),
                            )
                            .child(
                                div()
                                    .id(("close", entry_id))
                                    .px_2()
                                    .rounded_sm()
                                    .cursor_pointer()
                                    .text_color(rgb(0x585b70))
                                    .hover(|style| {
                                        style.text_color(rgb(0xf38ba8)).bg(rgb(0x45475a))
                                    })
                                    .child("x")
                                    .on_click(cx.listener(move |this, _event, _window, cx| {
                                        this.remove_webview(entry_id, cx);
                                    })),
                            ),
                    )
                    .child(webview.w_full().flex_1().bg(gpui::rgba(0x00000000))),
            );
        }

        if self.entries.is_empty() {
            content_area = content_area.child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0x585b70))
                    .child("Click a button above to add a webview"),
            );
        }

        div()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .flex()
            .flex_col()
            .child(
                div()
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_lg()
                                    .text_color(rgb(0x89b4fa))
                                    .child("WebView Example"),
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x585b70))
                                    .text_sm()
                                    .child(format!("{} active", active)),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .id("btn-counter")
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x89b4fa))
                                    .text_color(rgb(0x1e1e2e))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .child("Counter")
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.add_webview(DemoContent::Counter, cx);
                                    })),
                            )
                            .child(
                                div()
                                    .id("btn-plotly")
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0xf9e2af))
                                    .text_color(rgb(0x1e1e2e))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .child("Plotly")
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.add_webview(DemoContent::Plotly, cx);
                                    })),
                            )
                            .child(
                                div()
                                    .id("btn-website")
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0xcba6f7))
                                    .text_color(rgb(0x1e1e2e))
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_sm()
                                    .child("zed.dev")
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.add_webview(DemoContent::Website, cx);
                                    })),
                            ),
                    ),
            )
            .child(content_area)
    }
}

fn main() {
    env_logger::init();

    #[cfg(not(feature = "webview"))]
    {
        eprintln!("WebView feature is not compiled in.");
        eprintln!("Run with: cargo run -p gpui --example webview --features webview");
        return;
    }

    gpui_platform::application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.), px(700.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| WebViewExample::new()),
        )
        .unwrap();
        cx.activate(true);
    });
}
