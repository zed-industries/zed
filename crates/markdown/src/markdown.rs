mod parser;

use futures::FutureExt;
use gpui::{
    AnyElement, Bounds, FontStyle, FontWeight, GlobalElementId, Hsla, Render, StrikethroughStyle,
    Style, StyledText, Task, TextRun, TextStyle, TextStyleRefinement, WrappedLine,
};
use language::{Language, LanguageRegistry};
use parser::{parse_markdown, MarkdownEvent, MarkdownTag, MarkdownTagEnd};
use std::{cell::Cell, iter, mem, rc::Rc, sync::Arc};
use ui::prelude::*;
use util::TryFutureExt;

use crate::parser::CodeBlockKind;

#[derive(Clone)]
pub struct MarkdownStyle {
    pub code: TextStyleRefinement,
    pub code_background_color: Hsla,
    pub block_quote: TextStyleRefinement,
    pub link: TextStyleRefinement,
    pub rule_color: Hsla,
    pub block_quote_border_color: Hsla,
}

pub struct Markdown {
    text: String,
    style: MarkdownStyle,
    parsed_markdown: ParsedMarkdown,
    should_reparse: bool,
    pending_parse: Option<Task<Option<()>>>,
    language_registry: Arc<LanguageRegistry>,
}

impl Markdown {
    pub fn new(
        text: String,
        style: MarkdownStyle,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut this = Self {
            text: text.into(),
            style,
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
        MarkdownElement::new(
            self.parsed_markdown.clone(),
            self.style.clone(),
            self.language_registry.clone(),
        )
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
    style: MarkdownStyle,
    language_registry: Arc<LanguageRegistry>,
    div_stack: Vec<Div>,
    pending_text: String,
}

impl MarkdownElement {
    pub fn new(
        markdown: ParsedMarkdown,
        style: MarkdownStyle,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        Self {
            markdown,
            style,
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
        let mut builder = MarkdownElementBuilder::new(cx.text_style());
        for event in self.markdown.events.iter() {
            match event {
                MarkdownEvent::Start(tag) => match tag {
                    MarkdownTag::Paragraph => {
                        builder.push_div(div().line_height(rems(1.3)));
                    }
                    MarkdownTag::Heading { level, .. } => {
                        let mut heading = div().mt_2();
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
                        builder.push_text_style(self.style.block_quote.clone());
                        builder.push_div(
                            div()
                                .pl_4()
                                .my_2()
                                .border_l_4()
                                .border_color(self.style.block_quote_border_color),
                        );
                    }
                    MarkdownTag::CodeBlock(kind) => {
                        let language = if let CodeBlockKind::Fenced(language) = kind {
                            // todo!("notify when language is finally loaded")
                            self.language_registry
                                .language_for_name(language.as_ref())
                                .now_or_never()
                                .and_then(Result::ok)
                        } else {
                            None
                        };

                        builder.push_code_block(language);
                        builder.push_text_style(self.style.code.clone());
                        builder.push_div(
                            div()
                                .p_4()
                                .my_2()
                                .w_full()
                                .bg(self.style.code_background_color),
                        );
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
                            builder.push_text("• ");
                        };
                    }
                    MarkdownTag::Emphasis => builder.push_text_style(TextStyleRefinement {
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    }),
                    MarkdownTag::Strong => builder.push_text_style(TextStyleRefinement {
                        font_weight: Some(FontWeight::BOLD),
                        ..Default::default()
                    }),
                    MarkdownTag::Strikethrough => builder.push_text_style(TextStyleRefinement {
                        strikethrough: Some(StrikethroughStyle {
                            thickness: px(1.),
                            color: None,
                        }),
                        ..Default::default()
                    }),
                    MarkdownTag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    } => builder.push_text_style(self.style.link.clone()),
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
                        builder.pop_div();
                    }
                    MarkdownTagEnd::Heading(_) => builder.pop_div(),
                    MarkdownTagEnd::BlockQuote => {
                        builder.pop_text_style();
                        builder.pop_div()
                    }
                    MarkdownTagEnd::CodeBlock => {
                        builder.trim_trailing_newline();
                        builder.pop_div();
                        builder.pop_text_style();
                        builder.pop_code_block();
                    }
                    MarkdownTagEnd::HtmlBlock => builder.pop_div(),
                    MarkdownTagEnd::List(_) => {
                        builder.pop_list();
                        builder.pop_div();
                    }
                    MarkdownTagEnd::Item => builder.pop_div(),
                    MarkdownTagEnd::Emphasis => builder.pop_text_style(),
                    MarkdownTagEnd::Strong => builder.pop_text_style(),
                    MarkdownTagEnd::Strikethrough => builder.pop_text_style(),
                    MarkdownTagEnd::Link => builder.pop_text_style(),
                    MarkdownTagEnd::Image => todo!(),
                    _ => log::info!("unsupported markdown tag end: {:?}", tag),
                },
                MarkdownEvent::Text(range) => {
                    builder.push_text(&self.markdown.text[range.clone()]);
                }
                MarkdownEvent::Code(range) => {
                    builder.push_text_style(TextStyleRefinement {
                        background_color: Some(self.style.code_background_color),
                        ..self.style.code.clone()
                    });
                    builder.push_text(&self.markdown.text[range.clone()]);
                    builder.pop_text_style();
                }
                MarkdownEvent::Html(range) => {
                    builder.push_text(&self.markdown.text[range.clone()]);
                }
                MarkdownEvent::InlineHtml(range) => {
                    builder.push_text(&self.markdown.text[range.clone()]);
                }

                MarkdownEvent::Rule => {
                    builder.push_div(
                        div()
                            .border_b_1()
                            .my_2()
                            .border_color(self.style.rule_color),
                    );
                    builder.pop_div()
                }
                MarkdownEvent::FootnoteReference(_) => todo!(),
                MarkdownEvent::SoftBreak => todo!(),
                MarkdownEvent::HardBreak => todo!(),
                MarkdownEvent::TaskListMarker(_) => todo!(),
            }
        }

        let mut element = builder.finish().into_any();
        let child_layout_id = element.request_layout(cx);
        let layout_id = cx.request_layout(&Style::default(), [child_layout_id]);

        (layout_id, element)
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
    pending_runs: Vec<TextRun>,
    base_text_style: TextStyle,
    text_style_stack: Vec<TextStyleRefinement>,
    code_block_stack: Vec<Option<Arc<Language>>>,
    list_stack: Vec<ListStackEntry>,
}

struct ListStackEntry {
    item_index: Option<u64>,
}

impl MarkdownElementBuilder {
    fn new(base_text_style: TextStyle) -> Self {
        Self {
            div_stack: vec![div().debug_selector(|| "inner".into())],
            pending_text: String::new(),
            pending_runs: Vec::new(),
            base_text_style,
            text_style_stack: Vec::new(),
            code_block_stack: Vec::new(),
            list_stack: Vec::new(),
        }
    }

    fn push_text_style(&mut self, style: TextStyleRefinement) {
        self.text_style_stack.push(style);
    }

    fn text_style(&self) -> TextStyle {
        let mut style = self.base_text_style.clone();
        for refinement in &self.text_style_stack {
            style.refine(refinement);
        }
        style
    }

    fn pop_text_style(&mut self) {
        self.text_style_stack.pop();
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

    fn push_text(&mut self, text: &str) {
        let run = self.text_style().to_run(text.len());
        self.pending_text.push_str(text);
        self.pending_runs.push(run);
    }

    fn trim_trailing_newline(&mut self) {
        if self.pending_text.ends_with('\n') {
            self.pending_text.truncate(self.pending_text.len() - 1);
            self.pending_runs.last_mut().unwrap().len -= 1;
        }
    }

    fn flush_text(&mut self) {
        let text = mem::take(&mut self.pending_text);
        let runs = mem::take(&mut self.pending_runs);
        if text.is_empty() {
            return;
        }

        let text = StyledText::new(text).with_runs(runs);
        self.div_stack
            .last_mut()
            .unwrap()
            .extend(iter::once(text.into_any()));
    }

    fn finish(mut self) -> Div {
        debug_assert_eq!(self.div_stack.len(), 1);
        self.flush_text();
        self.div_stack.pop().unwrap()
    }
}
