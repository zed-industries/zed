use anyhow::Result;
use gpui::{App, ClipboardItem, Context, Entity, Window, div, prelude::*, px};
use language::Buffer;
use std::sync::{Arc, Mutex};

use crate::components::webview::WebView;
use crate::outputs::OutputContent;

use super::template::wrap_html_with_theme;

pub struct HtmlView {
    webview: Entity<WebView>,
    html: String,
    content_size: Arc<Mutex<Option<(f64, f64)>>>,
}

impl HtmlView {
    pub fn new(html: String, window: &mut Window, cx: &mut App) -> Result<Self> {
        let content_size = Arc::new(Mutex::new(None));
        let content_size_clone = content_size.clone();

        // Get theme background color to prevent white flash
        let theme = theme::GlobalTheme::theme(cx);
        let bg_color = theme.colors().editor_background;
        let bg_rgba = bg_color.to_rgb();

        // Wrap HTML content with themed template
        let html_with_theme = wrap_html_with_theme(&html, cx);

        let wry_webview = wry::WebViewBuilder::new()
            .with_html(&html_with_theme)
            .with_background_color((
                (bg_rgba.r * 255.0) as u8,
                (bg_rgba.g * 255.0) as u8,
                (bg_rgba.b * 255.0) as u8,
                255,
            ))
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
