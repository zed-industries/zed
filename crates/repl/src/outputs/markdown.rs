use gpui::{App, AppContext, ClipboardItem, Context, Entity, Window, div, prelude::*};
use language::Buffer;
use markdown::{Markdown, MarkdownElement, MarkdownFont, MarkdownStyle};

use crate::outputs::OutputContent;

pub struct MarkdownView {
    markdown: Entity<Markdown>,
    links_as_text: bool,
}

impl MarkdownView {
    pub fn from(text: String, cx: &mut Context<Self>) -> Self {
        let markdown = cx.new(|cx| Markdown::new(text.clone().into(), None, None, cx));

        Self {
            markdown,
            links_as_text: false,
        }
    }

    pub fn without_links(text: String, cx: &mut Context<Self>) -> Self {
        Self {
            links_as_text: true,
            ..Self::from(text, cx)
        }
    }
}

impl OutputContent for MarkdownView {
    fn clipboard_content(&self, _window: &Window, cx: &App) -> Option<ClipboardItem> {
        let source = self.markdown.read(cx).source().to_string();
        Some(ClipboardItem::new_string(source))
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn has_buffer_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn buffer_content(&mut self, _: &mut Window, cx: &mut App) -> Option<Entity<Buffer>> {
        let source = self.markdown.read(cx).source().to_string();
        let buffer = cx.new(|cx| {
            let mut buffer =
                Buffer::local(source.clone(), cx).with_language(language::PLAIN_TEXT.clone(), cx);
            buffer.set_capability(language::Capability::ReadOnly, cx);
            buffer
        });
        Some(buffer)
    }
}

impl Render for MarkdownView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let style = MarkdownStyle {
            links_as_text: self.links_as_text,
            ..markdown_style(window, cx)
        };
        div()
            .w_full()
            .child(MarkdownElement::new(self.markdown.clone(), style))
    }
}

fn markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    MarkdownStyle::themed(MarkdownFont::Editor, window, cx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext, point, px};

    fn click_first_link(
        view: fn(String, &mut Context<MarkdownView>) -> MarkdownView,
        cx: &mut TestAppContext,
    ) -> Option<String> {
        cx.update(|cx| {
            if !cx.has_global::<settings::SettingsStore>() {
                settings::init(cx);
            }
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        let (_, cx) =
            cx.add_window_view(|_, cx| view("[site](https://example.com/site)".into(), cx));
        cx.run_until_parked();
        cx.simulate_click(point(px(8.), px(8.)), Modifiers::default());
        cx.opened_url()
    }

    #[gpui::test]
    fn notebook_output_links_are_text(cx: &mut TestAppContext) {
        assert_eq!(click_first_link(MarkdownView::without_links, cx), None);
    }

    #[gpui::test]
    fn repl_output_links_open(cx: &mut TestAppContext) {
        assert_eq!(
            click_first_link(MarkdownView::from, cx),
            Some("https://example.com/site".into())
        );
    }
}
