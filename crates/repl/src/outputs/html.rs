use anyhow::Result;
use gpui::{App, ClipboardItem, Context, Entity, Window, div, prelude::*, px};
use language::Buffer;
use std::sync::{Arc, Mutex};
use ui::Pixels;

use crate::components::webview::WebView;
use crate::outputs::OutputContent;

pub struct HtmlView {
    webview: Entity<WebView>,
    html: String,
    content_size: Arc<Mutex<Option<(f64, f64)>>>,
}

impl HtmlView {
    pub fn new(html: String, window: &mut Window, cx: &mut App) -> Result<Self> {
        let content_size = Arc::new(Mutex::new(None));
        let content_size_clone = content_size.clone();

        // Wrap HTML in a document with size measurement script
        let html_with_script = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{ margin: 0; padding: 8px; overflow: auto; }}
    </style>
</head>
<body>
    {}
    <script>
        function measureContent() {{
            const width = Math.max(
                document.documentElement.scrollWidth,
                document.body.scrollWidth
            );
            const height = Math.max(
                document.documentElement.scrollHeight,
                document.body.scrollHeight
            );
            window.ipc.postMessage(JSON.stringify({{ width: width, height: height }}));
        }}

        if (document.readyState === 'loading') {{
            document.addEventListener('DOMContentLoaded', measureContent);
        }} else {{
            measureContent();
        }}

        window.addEventListener('resize', measureContent);
    </script>
</body>
</html>"#,
            html
        );

        let wry_webview = wry::WebViewBuilder::new()
            .with_html(&html_with_script)
            .with_transparent(true)
            .with_visible(true)
            .with_ipc_handler(move |message: wry::http::Request<String>| {
                if let Ok(size_data) = serde_json::from_str::<serde_json::Value>(message.body()) {
                    if let (Some(width), Some(height)) =
                        (size_data["width"].as_f64(), size_data["height"].as_f64())
                    {
                        let mut size = content_size_clone.lock().unwrap();
                        *size = Some((width, height));
                    }
                }
            })
            .build_as_child(window)?;

        let webview = cx.new(|cx| WebView::new(wry_webview, window, cx));

        Ok(Self {
            webview,
            html,
            content_size,
        })
    }
}

impl OutputContent for HtmlView {
    fn clipboard_content(&self, _window: &Window, _cx: &App) -> Option<ClipboardItem> {
        Some(ClipboardItem::new_string(self.html.clone()))
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn has_buffer_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn buffer_content(&mut self, _: &mut Window, cx: &mut App) -> Option<Entity<Buffer>> {
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(self.html.clone(), cx)
                .with_language(language::PLAIN_TEXT.clone(), cx);
            buffer.set_capability(language::Capability::ReadOnly, cx);
            buffer
        });
        Some(buffer)
    }
}

impl Render for HtmlView {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let content_size = self.content_size.lock().unwrap();

        // Ideally we'd get the width from the parent element
        // Just putting this in for now to have it working in a basic way
        let viewport_size = window.viewport_size();

        let height: f64 = content_size
            .as_ref()
            .map_or(50.0, |(_content_width, content_height)| *content_height);

        div()
            .w(viewport_size.width * 0.80)
            .h(px(height as f32))
            .child(self.webview.clone())
    }
}
