use anyhow::Result;
use gpui::{App, ClipboardItem, Context, Entity, Window, div, prelude::*, px};
use language::Buffer;

use crate::components::webview::WebView;
use crate::outputs::OutputContent;

pub struct HtmlView {
    webview: Entity<WebView>,
    html: String,
}

impl HtmlView {
    pub fn new(html: String, window: &mut Window, cx: &mut App) -> Result<Self> {
        let wry_webview = wry::WebViewBuilder::new()
            .with_html(&html)
            .with_transparent(true)
            .with_visible(true)
            .build_as_child(window)?;

        let webview = cx.new(|cx| WebView::new(wry_webview, window, cx));

        Ok(Self { webview, html })
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().w(px(600.0)).h(px(300.0)).child(self.webview.clone())
    }
}
