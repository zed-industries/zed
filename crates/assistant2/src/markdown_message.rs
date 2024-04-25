use gpui::{prelude::*, InteractiveText, Task, View, ViewContext};
use language::{LanguageRegistry, Rope};
use rich_text::Highlight;
use std::{ops::Range, sync::Arc};
use util::ResultExt;

pub struct MarkdownMessage {
    message: Rope,
    parsed: ParsedMarkdown,
    should_reparse: bool,
    pending_parse: Option<Task<()>>,
    language_registry: Arc<LanguageRegistry>,
}

#[derive(Default)]
struct ParsedMarkdown {
    message: Rope,
    highlights: Vec<(Range<usize>, Highlight)>,
    link_ranges: Vec<Range<usize>>,
    link_urls: Vec<String>,
}

impl MarkdownMessage {
    pub fn new(text: &str, language_registry: Arc<LanguageRegistry>) -> Self {
        Self {
            message: Rope::new(),
            should_reparse: false,
            pending_parse: None,
            parsed: ParsedMarkdown::default(),
            language_registry,
        }
    }

    pub fn push(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        self.message.push(text);
        self.parsed.message = self.message.clone();
        self.reparse(cx);
        cx.notify();
    }

    fn reparse(&mut self, cx: &mut ViewContext<Self>) {
        self.should_reparse = true;
        if self.pending_parse.is_some() {
            return;
        }

        let message = self.message.clone();
        let language_registry = self.language_registry.clone();
        self.should_reparse = false;
        self.pending_parse = Some(cx.spawn(|this, cx| async move {
            let parsed = cx
                .background_executor()
                .spawn(async move {
                    let input = self.message.to_string();
                    let mut output = String::new();
                    let mut highlights = Vec::new();
                    let mut link_ranges = Vec::new();
                    let mut link_urls = Vec::new();
                    rich_text::render_markdown_mut(
                        &input,
                        &[],
                        &language_registry,
                        None,
                        &mut output,
                        &mut highlights,
                        &mut link_ranges,
                        &mut link_urls,
                    );

                    ParsedMarkdown {
                        message,
                        highlights,
                        link_ranges,
                        link_urls,
                    }
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.parsed = parsed;
                this.parsed.message = this.message.clone();
                this.pending_parse = None;
                if this.should_reparse {
                    this.reparse(cx);
                }
                cx.notify();
            })
            .log_err();
        }));
    }
}

impl Render for MarkdownMessage {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        MarkdownMessageElement(cx.view().clone())
    }
}

pub struct MarkdownMessageElement(View<MarkdownMessage>);

impl Element for MarkdownMessageElement {
    type RequestLayoutState = InteractiveText;
    type PrepaintState = ();

    fn request_layout(
        &mut self,
        cx: &mut ui::prelude::WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
    }

    fn prepaint(
        &mut self,
        bounds: gpui::Bounds<ui::prelude::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut ui::prelude::WindowContext,
    ) -> Self::PrepaintState {
        todo!()
    }

    fn paint(
        &mut self,
        bounds: gpui::Bounds<ui::prelude::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut ui::prelude::WindowContext,
    ) {
        todo!()
    }
}
