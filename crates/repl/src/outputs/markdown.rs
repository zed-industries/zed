use gpui::{App, ClipboardItem, Context, Entity, Window, prelude::*};
use language::Buffer;
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use ui::v_flex;

use crate::outputs::OutputContent;

pub struct MarkdownView {
    raw_text: String,
    markdown: Entity<Markdown>,
}

impl MarkdownView {
    pub fn from(text: String, cx: &mut Context<Self>) -> Self {
        let markdown = cx.new(|cx| Markdown::new(text.clone().into(), None, None, cx));

        Self {
            raw_text: text,
            markdown,
        }
    }
}

impl OutputContent for MarkdownView {
    fn clipboard_content(&self, _window: &Window, _cx: &App) -> Option<ClipboardItem> {
        Some(ClipboardItem::new_string(self.raw_text.clone()))
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn has_buffer_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn buffer_content(&mut self, _: &mut Window, cx: &mut App) -> Option<Entity<Buffer>> {
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(self.raw_text.clone(), cx)
                .with_language(language::PLAIN_TEXT.clone(), cx);
            buffer.set_capability(language::Capability::ReadOnly, cx);
            buffer
        });
        Some(buffer)
    }
}

impl Render for MarkdownView {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let style = MarkdownStyle {
            base_text_style: window.text_style(),
            ..Default::default()
        };

        v_flex()
            .w_full()
            .gap_3()
            .py_4()
            .child(MarkdownElement::new(self.markdown.clone(), style))
            .into_any_element()
    }
}
