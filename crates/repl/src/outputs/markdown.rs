use anyhow::Result;
use gpui::{div, prelude::*, ClipboardItem, Model, Task, ViewContext, WindowContext};
use language::Buffer;
use markdown_preview::{
    markdown_elements::ParsedMarkdown, markdown_parser::parse_markdown,
    markdown_renderer::render_markdown_block,
};
use ui::v_flex;

use crate::outputs::OutputContent;

pub struct MarkdownView {
    raw_text: String,
    contents: Option<ParsedMarkdown>,
    parsing_markdown_task: Option<Task<Result<()>>>,
}

impl MarkdownView {
    pub fn from(text: String, cx: &mut ViewContext<Self>) -> Self {
        let task = cx.spawn(|markdown_view, mut cx| {
            let text = text.clone();
            let parsed = cx
                .background_executor()
                .spawn(async move { parse_markdown(&text, None, None).await });

            async move {
                let content = parsed.await;

                markdown_view.update(&mut cx, |markdown, cx| {
                    markdown.parsing_markdown_task.take();
                    markdown.contents = Some(content);
                    cx.notify();
                })
            }
        });

        Self {
            raw_text: text.clone(),
            contents: None,
            parsing_markdown_task: Some(task),
        }
    }
}

impl OutputContent for MarkdownView {
    fn clipboard_content(&self, _cx: &WindowContext) -> Option<ClipboardItem> {
        Some(ClipboardItem::new_string(self.raw_text.clone()))
    }

    fn has_clipboard_content(&self, _cx: &WindowContext) -> bool {
        true
    }

    fn has_buffer_content(&self, _cx: &WindowContext) -> bool {
        true
    }

    fn buffer_content(&mut self, cx: &mut WindowContext) -> Option<Model<Buffer>> {
        let buffer = cx.new_model(|cx| {
            // todo!(): Bring in the language registry so we can set the language to markdown
            let mut buffer = Buffer::local(self.raw_text.clone(), cx)
                .with_language(language::PLAIN_TEXT.clone(), cx);
            buffer.set_capability(language::Capability::ReadOnly, cx);
            buffer
        });
        Some(buffer)
    }
}

impl Render for MarkdownView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(parsed) = self.contents.as_ref() else {
            return div().into_any_element();
        };

        let mut markdown_render_context =
            markdown_preview::markdown_renderer::RenderContext::new(None, cx);

        v_flex()
            .gap_3()
            .py_4()
            .children(parsed.children.iter().map(|child| {
                div().relative().child(
                    div()
                        .relative()
                        .child(render_markdown_block(child, &mut markdown_render_context)),
                )
            }))
            .into_any_element()
    }
}
