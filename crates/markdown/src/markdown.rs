mod parser;

use futures::FutureExt;
use gpui::{
    AnyElement, Bounds, FontWeight, GlobalElementId, Render, ShapedLine, Task, WrappedLine,
};
use language::LanguageRegistry;
use parser::{parse_markdown, MarkdownEvent, MarkdownTag, MarkdownTagEnd};
use std::{cell::Cell, iter, rc::Rc, sync::Arc};
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
        MarkdownElement {
            markdown: self.parsed_markdown.clone(),
            language_registry: self.language_registry.clone(),
        }
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
        let mut bold_depth = 0;
        let mut italic_depth = 0;
        let mut strikethrough_depth = 0;

        let mut current_language = None;
        let mut div_stack = vec![div()];
        let mut list_stack = Vec::new();

        for event in self.markdown.events.iter() {
            match event {
                MarkdownEvent::Start(tag) => match tag {
                    MarkdownTag::Paragraph => {
                        div_stack.push(div());
                    }
                    MarkdownTag::Heading {
                        level,
                        id,
                        classes,
                        attrs,
                    } => {
                        let mut heading = div().font_weight(FontWeight::BOLD);
                        heading = match level {
                            pulldown_cmark::HeadingLevel::H1 => heading.text_3xl(),
                            pulldown_cmark::HeadingLevel::H2 => heading.text_2xl(),
                            pulldown_cmark::HeadingLevel::H3 => heading.text_xl(),
                            pulldown_cmark::HeadingLevel::H4 => heading.text_lg(),
                            _ => heading,
                        };
                        div_stack.push(heading);
                    }
                    MarkdownTag::BlockQuote => {
                        // todo!("use the right color")
                        div_stack.push(div().pl_2().text_color(gpui::red()));
                    }
                    MarkdownTag::CodeBlock(kind) => {
                        if let CodeBlockKind::Fenced(language) = kind {
                            current_language = self
                                .language_registry
                                .language_for_name(language.as_ref())
                                .now_or_never()
                                .and_then(Result::ok);
                        }

                        // todo!("use the right color")
                        div_stack.push(div().p_4().w_full().bg(gpui::green()));
                    }
                    MarkdownTag::HtmlBlock => div_stack.push(div()),
                    MarkdownTag::List(number) => {
                        list_stack.push(*number);
                        div_stack.push(div().pl_2());
                    }
                    MarkdownTag::Item => {
                        let item_prefix = if let Some(Some(number)) = list_stack.last_mut() {
                            let prefix = SharedString::from(format!("{}. ", *number));
                            *number += 1;
                            prefix
                        } else {
                            SharedString::from("- ")
                        };

                        div_stack.push(div().flex_col().child(item_prefix));
                    }
                    MarkdownTag::Emphasis => italic_depth += 1,
                    MarkdownTag::Strong => bold_depth += 1,
                    MarkdownTag::Strikethrough => strikethrough_depth += 1,
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
                    MarkdownTagEnd::Paragraph => {
                        let div = div_stack.pop().unwrap().into_any();
                        div_stack.last_mut().unwrap().extend(iter::once(div));
                    }
                    MarkdownTagEnd::Heading(_) => {
                        let div = div_stack.pop().unwrap().into_any();
                        div_stack.last_mut().unwrap().extend(iter::once(div));
                    }
                    MarkdownTagEnd::BlockQuote => {
                        let div = div_stack.pop().unwrap().into_any();
                        div_stack.last_mut().unwrap().extend(iter::once(div));
                    }
                    MarkdownTagEnd::CodeBlock => {
                        let div = div_stack.pop().unwrap().into_any();
                        div_stack.last_mut().unwrap().extend(iter::once(div));
                    }
                    MarkdownTagEnd::HtmlBlock => {
                        let div = div_stack.pop().unwrap().into_any();
                        div_stack.last_mut().unwrap().extend(iter::once(div));
                    }
                    MarkdownTagEnd::List(_) => {
                        let div = div_stack.pop().unwrap().into_any();
                        div_stack.last_mut().unwrap().extend(iter::once(div));
                    }
                    MarkdownTagEnd::Item => {
                        let div = div_stack.pop().unwrap().into_any();
                        div_stack.last_mut().unwrap().extend(iter::once(div));
                    }
                    MarkdownTagEnd::Emphasis => italic_depth -= 1,
                    MarkdownTagEnd::Strong => bold_depth -= 1,
                    MarkdownTagEnd::Strikethrough => strikethrough_depth -= 1,
                    MarkdownTagEnd::Link => todo!(),
                    MarkdownTagEnd::Image => todo!(),
                    _ => log::info!("unsupported markdown tag end: {:?}", tag),
                },
                MarkdownEvent::Text(range) => {
                    let text = self.markdown.text[range.clone()].to_string();
                    div_stack
                        .last_mut()
                        .unwrap()
                        .extend(iter::once(SharedString::from(text).into_any()));
                }
                MarkdownEvent::Code(range) => {
                    let text = self.markdown.text[range.clone()].to_string();
                    div_stack
                        .last_mut()
                        .unwrap()
                        .extend(iter::once(SharedString::from(text).into_any()));
                }
                MarkdownEvent::Html(range) => {
                    let text = self.markdown.text[range.clone()].to_string();
                    div_stack
                        .last_mut()
                        .unwrap()
                        .extend(iter::once(SharedString::from(text).into_any()));
                }
                MarkdownEvent::InlineHtml(range) => {
                    let text = self.markdown.text[range.clone()].to_string();
                    div_stack
                        .last_mut()
                        .unwrap()
                        .extend(iter::once(SharedString::from(text).into_any()));
                }
                _ => todo!(),
            }
        }

        let mut element = div_stack.pop().unwrap().into_any();
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
