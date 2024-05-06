mod parser;

use futures::FutureExt;
use gpui::{AnyElement, Bounds, FontWeight, GlobalElementId, Render, Task, WrappedLine};
use language::{Language, LanguageRegistry};
use parser::{parse_markdown, MarkdownEvent, MarkdownTag, MarkdownTagEnd};
use std::{cell::Cell, iter, mem, rc::Rc, sync::Arc};
use ui::prelude::*;
use util::TryFutureExt;

use crate::parser::CodeBlockKind;

pub struct Markdown {
    text: String,
    parsed_markdown: ParsedMarkdown,
    should_reparse: bool,
    pending_parse: Option<Task<Option<()>>>,
    language_registry: Arc<LanguageRegistry>,
}

impl Markdown {
    pub fn new(
        text: String,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut this = Self {
            text: text.into(),
            should_reparse: false,
            parsed_markdown: ParsedMarkdown::default(),
            pending_parse: None,
            language_registry,
        };
        this.parse(cx);
        this
    }

    pub fn append(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        self.text.push_str(text);
        self.parse(cx);
    }

    fn parse(&mut self, cx: &mut ViewContext<Self>) {
        if self.text.is_empty() {
            return;
        }

        if self.pending_parse.is_some() {
            self.should_reparse = true;
            return;
        }

        let text = self.text.clone();
        let parsed = cx.background_executor().spawn(async move {
            let text = SharedString::from(text);
            let events = Arc::from(parse_markdown(text.as_ref()));
            anyhow::Ok(ParsedMarkdown { text, events })
        });

        self.should_reparse = false;
        self.pending_parse = Some(cx.spawn(|this, mut cx| {
            async move {
                let parsed = parsed.await?;
                this.update(&mut cx, |this, cx| {
                    this.parsed_markdown = parsed;
                    this.pending_parse.take();
                    if this.should_reparse {
                        this.parse(cx);
                    }
                    cx.notify();
                })
                .ok();
                anyhow::Ok(())
            }
            .log_err()
        }));
    }
}

impl Render for Markdown {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(gpui::white())
            .child(MarkdownElement::new(
                self.parsed_markdown.clone(),
                self.language_registry.clone(),
            ))
    }
}

#[derive(Clone)]
struct ParsedMarkdown {
    text: SharedString,
    events: Arc<[MarkdownEvent]>,
}

impl Default for ParsedMarkdown {
    fn default() -> Self {
        Self {
            text: SharedString::default(),
            events: Arc::from([]),
        }
    }
}

pub struct MarkdownElement {
    markdown: ParsedMarkdown,
    language_registry: Arc<LanguageRegistry>,
    div_stack: Vec<Div>,
    pending_text: String,
}

impl MarkdownElement {
    pub fn new(markdown: ParsedMarkdown, language_registry: Arc<LanguageRegistry>) -> Self {
        Self {
            markdown,
            language_registry,
            div_stack: Vec::new(),
            pending_text: String::new(),
        }
    }
}

impl Element for MarkdownElement {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let mut builder = MarkdownElementBuilder::new();
        for event in self.markdown.events.iter() {
            match event {
                MarkdownEvent::Start(tag) => match tag {
                    MarkdownTag::Paragraph => {
                        builder.push_div(div());
                    }
                    MarkdownTag::Heading { level, .. } => {
                        let mut heading = div().font_weight(FontWeight::BOLD);
                        heading = match level {
                            pulldown_cmark::HeadingLevel::H1 => heading.text_3xl(),
                            pulldown_cmark::HeadingLevel::H2 => heading.text_2xl(),
                            pulldown_cmark::HeadingLevel::H3 => heading.text_xl(),
                            pulldown_cmark::HeadingLevel::H4 => heading.text_lg(),
                            _ => heading,
                        };
                        builder.push_div(heading);
                    }
                    MarkdownTag::BlockQuote => {
                        // todo!("use the right color")
                        builder.push_div(div().pl_2().text_color(gpui::red()));
                    }
                    MarkdownTag::CodeBlock(kind) => {
                        let language = if let CodeBlockKind::Fenced(language) = kind {
                            self.language_registry
                                .language_for_name(language.as_ref())
                                .now_or_never()
                                .and_then(Result::ok)
                        } else {
                            None
                        };

                        builder.push_code_block(language);
                        // todo!("use the right color")
                        builder.push_div(div().p_4().w_full().bg(gpui::green()));
                    }
                    MarkdownTag::HtmlBlock => builder.push_div(div()),
                    MarkdownTag::List(number) => {
                        builder.push_list(*number);
                        builder.push_div(div().pl_4());
                    }
                    MarkdownTag::Item => {
                        builder.push_div(div());
                        if let Some(item_index) = builder.next_list_item_index() {
                            builder.push_text(&format!("{}. ", item_index));
                        } else {
                            builder.push_text("- ");
                        };
                    }
                    MarkdownTag::Emphasis => builder.push_emphasis(),
                    MarkdownTag::Strong => builder.push_strong(),
                    MarkdownTag::Strikethrough => builder.push_strikethrough(),
                    MarkdownTag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    } => todo!(),
                    MarkdownTag::Image {
                        link_type,
                        dest_url,
                        title,
                        id,
                    } => todo!(),
                    _ => log::info!("unsupported markdown tag {:?}", tag),
                },
                MarkdownEvent::End(tag) => match tag {
                    MarkdownTagEnd::Paragraph => builder.pop_div(),
                    MarkdownTagEnd::Heading(_) => builder.pop_div(),
                    MarkdownTagEnd::BlockQuote => builder.pop_div(),
                    MarkdownTagEnd::CodeBlock => {
                        builder.pop_code_block();
                        builder.pop_div();
                    }
                    MarkdownTagEnd::HtmlBlock => builder.pop_div(),
                    MarkdownTagEnd::List(_) => {
                        builder.pop_list();
                        builder.pop_div();
                    }
                    MarkdownTagEnd::Item => builder.pop_div(),
                    MarkdownTagEnd::Emphasis => builder.pop_emphasis(),
                    MarkdownTagEnd::Strong => builder.pop_strong(),
                    MarkdownTagEnd::Strikethrough => builder.pop_strikethrough(),
                    MarkdownTagEnd::Link => todo!(),
                    MarkdownTagEnd::Image => todo!(),
                    _ => log::info!("unsupported markdown tag end: {:?}", tag),
                },
                MarkdownEvent::Text(range) => {
                    builder.push_text(&self.markdown.text[range.clone()]);
                }
                MarkdownEvent::Code(range) => {
                    builder.push_text(&self.markdown.text[range.clone()]);
                }
                MarkdownEvent::Html(range) => {
                    builder.push_text(&self.markdown.text[range.clone()]);
                }
                MarkdownEvent::InlineHtml(range) => {
                    builder.push_text(&self.markdown.text[range.clone()]);
                }
                _ => todo!(),
            }
        }

        let mut element = builder.finish().into_any();
        (element.request_layout(cx), element)
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        request_layout.prepaint(cx);
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        request_layout.paint(cx);
    }
}

impl IntoElement for MarkdownElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

#[derive(Clone)]
struct TextBlock {
    bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    line: WrappedLine,
}

struct MarkdownElementBuilder {
    div_stack: Vec<Div>,
    pending_text: String,
    bold_depth: usize,
    italic_depth: usize,
    strikethrough_depth: usize,
    code_block_stack: Vec<Option<Arc<Language>>>,
    list_stack: Vec<ListStackEntry>,
}

struct ListStackEntry {
    item_index: Option<u64>,
}

impl MarkdownElementBuilder {
    fn new() -> Self {
        Self {
            div_stack: vec![div()],
            pending_text: String::new(),
            bold_depth: 0,
            italic_depth: 0,
            strikethrough_depth: 0,
            code_block_stack: Vec::new(),
            list_stack: Vec::new(),
        }
    }

    fn push_div(&mut self, div: Div) {
        self.flush_text();
        self.div_stack.push(div);
    }

    fn pop_div(&mut self) {
        self.flush_text();
        let div = self.div_stack.pop().unwrap().into_any();
        self.div_stack.last_mut().unwrap().extend(iter::once(div));
    }

    fn push_list(&mut self, item_index: Option<u64>) {
        self.list_stack.push(ListStackEntry { item_index });
    }

    fn next_list_item_index(&mut self) -> Option<u64> {
        self.list_stack.last_mut().and_then(|entry| {
            let item_index = entry.item_index.as_mut()?;
            *item_index += 1;
            Some(*item_index - 1)
        })
    }

    fn pop_list(&mut self) {
        self.list_stack.pop();
    }

    fn push_code_block(&mut self, language: Option<Arc<Language>>) {
        self.code_block_stack.push(language);
    }

    fn pop_code_block(&mut self) {
        self.code_block_stack.pop();
    }

    fn push_emphasis(&mut self) {
        self.italic_depth += 1;
    }

    fn pop_emphasis(&mut self) {
        self.italic_depth -= 1;
    }

    fn push_strong(&mut self) {
        self.bold_depth += 1;
    }

    fn pop_strong(&mut self) {
        self.bold_depth -= 1;
    }

    fn push_strikethrough(&mut self) {
        self.strikethrough_depth += 1;
    }

    fn pop_strikethrough(&mut self) {
        self.strikethrough_depth -= 1;
    }

    fn push_text(&mut self, text: &str) {
        self.pending_text.push_str(text);
    }

    fn flush_text(&mut self) {
        let pending_text = mem::take(&mut self.pending_text);
        if pending_text.is_empty() {
            return;
        }

        self.div_stack
            .last_mut()
            .unwrap()
            .extend(iter::once(SharedString::from(pending_text).into_any()));
    }

    fn finish(mut self) -> Div {
        debug_assert_eq!(self.div_stack.len(), 1);
        self.flush_text();
        self.div_stack.pop().unwrap()
    }
}
